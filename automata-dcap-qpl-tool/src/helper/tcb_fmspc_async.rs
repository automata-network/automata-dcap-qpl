//! Offchain driver for FmspcTcbDaoV2 — closes the V2 step-2/step-3 consistency hole by
//! submitting per-level / per-identity byte ranges into the raw plus the components template,
//! so finalize can reverse-serialize on-chain and assert byte-exact match with the signed raw.
//!
//! The on-chain layout this module mirrors is documented in
//! `automata-on-chain-pccs/src/bases/FmspcTcbDaoV2.sol` and
//! `automata-on-chain-pccs/src/helpers/FmspcTcbHelperV2.sol`.

use crate::helper::estimate_gas_at_latest;
use crate::pccs_types::TcbInfo;
use automata_dcap_qpl_contracts::{
    fmspc_tcb_dao::TcbInfoJsonObj,
    parse_address_from_env_var::parse_address_from_str,
};
use ethers::abi::Detokenize;
use ethers::contract::{abigen, builders::ContractCall};
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::utils::keccak256;
use hex::FromHex;
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, path::Path, sync::Arc};
use tokio::time::timeout;

const TX_CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(120);
const ASYNC_UPLOAD_CHUNK_SIZE: usize = 8 * 1024;
const DEFAULT_ASYNC_PARSE_BATCH_SIZE: u64 = 3;
const FALLBACK_GAS_LIMIT_ENV: &str = "QPL_FALLBACK_GAS_LIMIT";
const ASYNC_PARSE_BATCH_SIZE_ENV: &str = "QPL_ASYNC_PARSE_BATCH_SIZE";

fn async_parse_batch_size() -> u64 {
    std::env::var(ASYNC_PARSE_BATCH_SIZE_ENV)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_ASYNC_PARSE_BATCH_SIZE)
}

abigen!(
    FmspcTcbDaoV2,
    r#"[
        function FMSPC_TCB_KEY(uint8 tcbType, bytes6 fmspc, uint32 version) view returns (bytes32 key)
        function resolver() view returns (address)
        function startAsyncUpsert(bytes32 refId, bytes signature)
        function uploadChunkData(bytes32 refId, bytes chunkData)
        function uploadComponentsTemplate(bytes32 refId, bytes sgxTemplate, bytes tdxTemplate)
        function commitBasicsV2(bytes32 refId)
        function commitBasicsExtract(bytes32 refId)
        function commitTcbLevelsRange(bytes32 refId)
        function commitTdxIdentitiesRange(bytes32 refId)
        function uploadParsedTcbLevelsBatch(bytes32 refId, uint256 start, uint256 itemCount, bytes batchStream) returns (uint256 parsed, uint256 total, bool complete)
        function uploadParsedTdxModuleIdentitiesBatch(bytes32 refId, uint256 start, uint256 itemCount, bytes batchStream) returns (uint256 parsed, uint256 total, bool complete)
        function finalizeAsyncUpsert(bytes32 attestationId, bytes32 refId) returns (bytes32)
    ]"#
);

abigen!(
    CollateralResolver,
    r#"[
        function collateralPointer(bytes32 key) view returns (bytes32)
    ]"#
);

#[derive(Clone, Copy)]
pub(crate) struct TcbLocator {
    pub(crate) tcb_type: u8,
    pub(crate) fmspc: [u8; 6],
    pub(crate) version: u32,
}

pub async fn upsert_tcb_fmspc_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    fmspc: &str,
    platform: &str,
    version: &str,
    collateral_update_type: Option<&str>,
    tcb_evaluation_data_number: Option<u32>,
    fmspc_tcb_dao_contract_addr: &str,
    tcb_info_json_file: Option<&str>,
    tcb_info_signature_hex: Option<&str>,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, fmspc_tcb_dao_contract_addr);

    let (raw_inner, tcb_info_obj, locator) = if let Some(path) = tcb_info_json_file {
        match load_payload_from_file(path, tcb_info_signature_hex, platform, version) {
            Ok(v) => v,
            Err(err) => {
                log::error!("{} failed to load local TCB payload: {}", log_prefix, err);
                return false;
            }
        }
    } else {
        match fetch_payload(&log_prefix, fmspc, platform, version, collateral_update_type, tcb_evaluation_data_number).await
        {
            Ok(v) => v,
            Err(err) => {
                log::error!("{} failed to fetch TCB payload: {}", log_prefix, err);
                return false;
            }
        }
    };

    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain_id)));
    let dao = FmspcTcbDaoV2::new(parse_address_from_str(fmspc_tcb_dao_contract_addr), signer.clone());

    run_upsert(&dao, signer.as_ref(), &log_prefix, gas_price, &raw_inner, &tcb_info_obj, locator).await
}

/// Run the staged V2 upsert end-to-end.
async fn run_upsert<M: Middleware + 'static>(
    dao: &FmspcTcbDaoV2<M>,
    signer: &M,
    log_prefix: &str,
    gas_price: U256,
    raw_inner: &str,
    tcb_info_obj: &TcbInfoJsonObj,
    locator: TcbLocator,
) -> bool
where
    M::Error: 'static,
{
    let raw = raw_inner.as_bytes();
    let ranges = match find_array_ranges(raw) {
        Ok(r) => r,
        Err(err) => {
            log::error!("{} byte-range scan failed: {}", log_prefix, err);
            return false;
        }
    };
    let levels = match extract_levels(raw, ranges.tcb_levels_start, ranges.tcb_levels_end, locator.version) {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} extract levels failed: {}", log_prefix, err);
            return false;
        }
    };
    let identities = if let Some((s, e)) = ranges.identities {
        match extract_identities(raw, s, e) {
            Ok(v) => v,
            Err(err) => {
                log::error!("{} extract identities failed: {}", log_prefix, err);
                return false;
            }
        }
    } else {
        Vec::new()
    };
    let sgx_template = extract_components_template(raw, levels[0].byte_start, levels[0].byte_end, b"\"sgxtcbcomponents\":")
        .unwrap_or_default();
    let tdx_template = if locator.tcb_type == 1 {
        extract_components_template(raw, levels[0].byte_start, levels[0].byte_end, b"\"tdxtcbcomponents\":").unwrap_or_default()
    } else {
        Vec::new()
    };

    let ref_id = generate_ref_id(locator, tcb_info_obj);

    if !send_transaction(
        signer,
        dao.start_async_upsert(ref_id, tcb_info_obj.signature.clone()).gas_price(gas_price),
        log_prefix,
        "start_async_upsert",
    )
    .await
    {
        return false;
    }

    for (i, chunk) in raw.chunks(ASYNC_UPLOAD_CHUNK_SIZE).enumerate() {
        let label = format!("upload_chunk_data_{}", i);
        if !send_transaction(
            signer,
            dao.upload_chunk_data(ref_id, chunk.to_vec().into()).gas_price(gas_price),
            log_prefix,
            &label,
        )
        .await
        {
            return false;
        }
    }

    if !send_transaction(
        signer,
        dao.upload_components_template(ref_id, sgx_template.into(), tdx_template.into()).gas_price(gas_price),
        log_prefix,
        "upload_components_template",
    )
    .await
    {
        return false;
    }

    // Three staged commits (vs the bundled `commitBasicsV2`) keep each tx under the per-block
    // gas cap as raw grows. Each stage walks only the slice of raw it needs:
    //   1) commitBasicsExtract       — top-level field scan via extractBasics (~300K-1M)
    //   2) commitTcbLevelsRange      — depth-1 scan + brace-count over the tcbLevels array
    //   3) commitTdxIdentitiesRange  — same but for tdxModuleIdentities (TDX only)
    if !send_transaction(
        signer,
        dao.commit_basics_extract(ref_id).gas_price(gas_price),
        log_prefix,
        "commit_basics_extract",
    )
    .await
    {
        return false;
    }
    if !send_transaction(
        signer,
        dao.commit_tcb_levels_range(ref_id).gas_price(gas_price),
        log_prefix,
        "commit_tcb_levels_range",
    )
    .await
    {
        return false;
    }
    if locator.tcb_type == 1 {
        if !send_transaction(
            signer,
            dao.commit_tdx_identities_range(ref_id).gas_price(gas_price),
            log_prefix,
            "commit_tdx_identities_range",
        )
        .await
        {
            return false;
        }
    }

    // Each async upload-batch tx does per-level reverse-serialization + keccak round-trip against
    // the signed raw — about 1.3M gas per level. Submitting all 15+ levels in one tx blows the
    // 2^24 (~16.7M) per-tx gas cap, so we chunk by `QPL_ASYNC_PARSE_BATCH_SIZE` (default 3).
    let batch_size = async_parse_batch_size() as usize;
    let mut sent = 0usize;
    while sent < levels.len() {
        let end = std::cmp::min(sent + batch_size, levels.len());
        let chunk_stream = build_level_stream(&levels[sent..end], locator.tcb_type == 1);
        if !send_transaction(
            signer,
            dao.upload_parsed_tcb_levels_batch(
                ref_id,
                U256::from(sent),
                U256::from(end - sent),
                chunk_stream.into(),
            )
            .gas_price(gas_price),
            log_prefix,
            "upload_parsed_tcb_levels_batch",
        )
        .await
        {
            return false;
        }
        sent = end;
    }

    if !identities.is_empty() {
        let mut sent = 0usize;
        while sent < identities.len() {
            let end = std::cmp::min(sent + batch_size, identities.len());
            let chunk_stream = build_identity_stream(&identities[sent..end]);
            if !send_transaction(
                signer,
                dao.upload_parsed_tdx_module_identities_batch(
                    ref_id,
                    U256::from(sent),
                    U256::from(end - sent),
                    chunk_stream.into(),
                )
                .gas_price(gas_price),
                log_prefix,
                "upload_parsed_tdx_module_identities_batch",
            )
            .await
            {
                return false;
            }
            sent = end;
        }
    }

    let tcb_key = match dao.fmspc_tcb_key(locator.tcb_type, locator.fmspc, locator.version).call().await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} fmspc_tcb_key call failed: {:?}", log_prefix, err);
            return false;
        }
    };
    let resolver_addr = match dao.resolver().call().await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} resolver call failed: {:?}", log_prefix, err);
            return false;
        }
    };
    let resolver = CollateralResolver::new(resolver_addr, signer.into());
    let attestation_id = match resolver.collateral_pointer(tcb_key).call().await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} collateralPointer call failed: {:?}", log_prefix, err);
            return false;
        }
    };

    send_transaction(
        signer,
        dao.finalize_async_upsert(attestation_id, ref_id).gas_price(gas_price),
        log_prefix,
        "finalize_async_upsert",
    )
    .await
}

/* ----- payload loading (file or HTTP) ----- */

fn load_payload_from_file(
    path: &str,
    signature_hex: Option<&str>,
    platform: &str,
    collateral_version: &str,
) -> Result<(String, TcbInfoJsonObj, TcbLocator), String> {
    let raw = fs::read_to_string(path).map_err(|err| format!("unable to read {}: {:?}", path, err))?;
    let trimmed = raw.trim();
    let payload = if trimmed.starts_with(r#"{"tcbInfo":{"#) && trimmed.contains(r#","signature":"#) {
        trimmed.to_string()
    } else {
        let sig = resolve_signature_hex(path, signature_hex)?;
        format!(r#"{{"tcbInfo":{},"signature":"{}"}}"#, trimmed, sig)
    };
    parse_payload(&payload, platform, collateral_version)
}

fn resolve_signature_hex(path: &str, signature_hex: Option<&str>) -> Result<String, String> {
    if let Some(signature_hex) = signature_hex {
        return Ok(signature_hex.trim().trim_start_matches("0x").to_string());
    }

    let path_obj = Path::new(path);
    let file_name = path_obj
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("unable to derive signature path from {}", path))?;
    let sibling_name = if file_name.contains("_tcbinfo.") {
        file_name.replace("_tcbinfo.", "_signature.")
    } else {
        return Err(format!(
            "missing signature hex and cannot infer sibling signature file for {}",
            path
        ));
    };
    let sibling_path = path_obj.with_file_name(sibling_name);
    let signature_path = if sibling_path.exists() {
        sibling_path
    } else if sibling_path
        .extension()
        .and_then(|value| value.to_str())
        == Some("json")
    {
        sibling_path.with_extension("txt")
    } else {
        sibling_path
    };
    let sibling_str = signature_path
        .to_str()
        .ok_or_else(|| format!("invalid inferred signature path for {}", path))?;
    let signature = fs::read_to_string(sibling_str).map_err(|err| {
        format!(
            "unable to read inferred signature file {}: {:?}",
            sibling_str, err
        )
    })?;
    Ok(signature.trim().trim_start_matches("0x").to_string())
}

async fn fetch_payload(
    log_prefix: &str,
    fmspc: &str,
    platform: &str,
    version: &str,
    collateral_update_type: Option<&str>,
    tcb_evaluation_data_number: Option<u32>,
) -> Result<(String, TcbInfoJsonObj, TcbLocator), String> {
    let mut req_url = format!(
        "https://api.trustedservices.intel.com/{}/certification/{}/tcb?fmspc={}",
        platform, version, fmspc
    );
    if let Some(n) = tcb_evaluation_data_number.filter(|v| *v > 0) {
        req_url.push_str(&format!("&tcbEvaluationDataNumber={}", n));
    } else if let Some(u) = collateral_update_type {
        req_url.push_str(&format!("&update={}", u));
    }
    log::debug!("{} req_url: {}", log_prefix, req_url);
    let response = reqwest::get(req_url.clone())
        .await
        .map_err(|err| format!("unable to get {}: {:?}", req_url, err))?;
    if !response.status().is_success() {
        return Err(format!("{} returned {}", req_url, response.status()));
    }
    let body = response.text().await.map_err(|err| format!("unable to read body of {}: {:?}", req_url, err))?;
    parse_payload(&body, platform, version)
}

/// Extract the inner tcbInfo substring + signature from the wrapped `{"tcbInfo":{...},"signature":"..."}` body.
fn parse_payload(raw_json: &str, platform: &str, collateral_version: &str) -> Result<(String, TcbInfoJsonObj, TcbLocator), String> {
    let tcb_info: TcbInfo = serde_json::from_str(raw_json).map_err(|err| format!("invalid tcb payload: {:?}", err))?;
    let inner = extract_tcb_info_str(raw_json)?;
    let signature = Bytes::from_hex(tcb_info.signature).map_err(|err| format!("invalid signature: {:?}", err))?;

    let fmspc_hex = tcb_info.tcb_info.get("fmspc").and_then(Value::as_str).ok_or_else(|| "missing fmspc".to_string())?;
    let fmspc_vec = hex::decode(fmspc_hex).map_err(|err| format!("invalid fmspc hex {}: {:?}", fmspc_hex, err))?;
    let fmspc: [u8; 6] = fmspc_vec.try_into().map_err(|_| "unexpected fmspc length".to_string())?;
    let tcb_type = tcb_info
        .tcb_info
        .get("id")
        .and_then(Value::as_str)
        .map(parse_tcb_type)
        .unwrap_or_else(|| parse_tcb_type(platform));
    let version = tcb_info
        .tcb_info
        .get("version")
        .and_then(Value::as_u64)
        .map(|v| v as u32)
        .or_else(|| fallback_tcb_version(collateral_version))
        .ok_or_else(|| "missing version".to_string())?;

    let tcb_info_obj = TcbInfoJsonObj { tcb_info_str: inner.clone(), signature };
    Ok((inner, tcb_info_obj, TcbLocator { tcb_type, fmspc, version }))
}

/* ----- byte-range scanning (mirrors the on-chain depth-1 brace scanner) ----- */

struct ArrayRanges {
    tcb_levels_start: usize,
    tcb_levels_end: usize,
    /// Some((start, end)) when the JSON has tdxModuleIdentities (TDX+v3 schema).
    identities: Option<(usize, usize)>,
}

fn find_array_ranges(raw: &[u8]) -> Result<ArrayRanges, String> {
    let levels = find_top_level_array(raw, b"tcbLevels").ok_or_else(|| "tcbLevels not found".to_string())?;
    let ids = find_top_level_array(raw, b"tdxModuleIdentities");
    Ok(ArrayRanges {
        tcb_levels_start: levels.0,
        tcb_levels_end: levels.1,
        identities: ids,
    })
}

/// Find a top-level array value `"key":[...]` at object depth==1. Returns (`[` offset, position after `]`).
fn find_top_level_array(raw: &[u8], key: &[u8]) -> Option<(usize, usize)> {
    let mut needle = Vec::with_capacity(key.len() + 3);
    needle.push(b'"');
    needle.extend_from_slice(key);
    needle.push(b'"');
    needle.push(b':');

    let mut depth: i32 = 0;
    let mut p = 0usize;
    while p < raw.len() {
        match raw[p] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            b'"' => {
                if depth == 1 && p + needle.len() <= raw.len() && &raw[p..p + needle.len()] == needle.as_slice() {
                    let val_start = p + needle.len();
                    if raw.get(val_start) != Some(&b'[') {
                        return None;
                    }
                    let mut adepth: i32 = 0;
                    let mut q = val_start;
                    while q < raw.len() {
                        match raw[q] {
                            b'[' => adepth += 1,
                            b']' => {
                                adepth -= 1;
                                if adepth == 0 {
                                    return Some((val_start, q + 1));
                                }
                            }
                            b'"' => {
                                q += 1;
                                while q < raw.len() && raw[q] != b'"' {
                                    if raw[q] == b'\\' { q += 1; }
                                    q += 1;
                                }
                            }
                            _ => {}
                        }
                        q += 1;
                    }
                    return None;
                }
                // Skip past string at any depth.
                p += 1;
                while p < raw.len() && raw[p] != b'"' {
                    if raw[p] == b'\\' { p += 1; }
                    p += 1;
                }
            }
            _ => {}
        }
        p += 1;
    }
    None
}

/// Depth-1 brace scan over an array `[ {...}, {...}, ... ]`. Returns per-item (start, end_exclusive).
fn brace_scan_items(raw: &[u8], arr_start: usize, arr_end: usize) -> Vec<(usize, usize)> {
    let mut items = Vec::new();
    let mut depth: i32 = 0;
    let mut cur_start = 0usize;
    let mut p = arr_start + 1;
    while p + 1 < arr_end {
        match raw[p] {
            b'{' => {
                if depth == 0 { cur_start = p; }
                depth += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    items.push((cur_start, p + 1));
                }
            }
            b'"' => {
                p += 1;
                while p + 1 < arr_end && raw[p] != b'"' {
                    if raw[p] == b'\\' { p += 1; }
                    p += 1;
                }
            }
            _ => {}
        }
        p += 1;
    }
    items
}

/* ----- level extraction + packing ----- */

struct LevelEntry {
    byte_start: u32,
    byte_end: u32,
    raw_tcb_date: [u8; 20],
    packed: Vec<u8>,
}

fn extract_levels(raw: &[u8], arr_start: usize, arr_end: usize, version: u32) -> Result<Vec<LevelEntry>, String> {
    let bounds = brace_scan_items(raw, arr_start, arr_end);
    let mut out = Vec::with_capacity(bounds.len());
    for (s, e) in bounds {
        let level_bytes = &raw[s..e];
        let level_json: Value =
            serde_json::from_slice(level_bytes).map_err(|err| format!("level parse failed: {:?}", err))?;
        let tcb = level_json
            .get("tcb")
            .ok_or_else(|| "level missing tcb".to_string())?;
        let pcesvn = tcb.get("pcesvn").and_then(Value::as_u64).ok_or_else(|| "missing pcesvn".to_string())? as u16;
        let (sgx_svns, tdx_svns) = extract_svns(tcb, version)?;

        let tcb_date_str = level_json
            .get("tcbDate")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing tcbDate".to_string())?;
        if tcb_date_str.len() != 20 {
            return Err(format!("unexpected tcbDate format: {}", tcb_date_str));
        }
        let mut raw_tcb_date = [0u8; 20];
        raw_tcb_date.copy_from_slice(tcb_date_str.as_bytes());
        let ts = iso_to_unix(tcb_date_str).ok_or_else(|| format!("bad tcbDate: {}", tcb_date_str))?;

        let status_str = level_json
            .get("tcbStatus")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing tcbStatus".to_string())?;
        let status = status_string_to_enum(status_str)?;
        let advisories = level_json
            .get("advisoryIDs")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
            .unwrap_or_default();

        out.push(LevelEntry {
            byte_start: s as u32,
            byte_end: e as u32,
            raw_tcb_date,
            packed: pack_tcb_level(pcesvn, ts, status, &sgx_svns, &tdx_svns, &advisories),
        });
    }
    Ok(out)
}

fn extract_svns(tcb: &Value, version: u32) -> Result<([u8; 16], [u8; 16]), String> {
    let mut sgx = [0u8; 16];
    let mut tdx = [0u8; 16];
    if version >= 3 {
        let sgx_arr = tcb
            .get("sgxtcbcomponents")
            .and_then(Value::as_array)
            .ok_or_else(|| "missing sgxtcbcomponents".to_string())?;
        if sgx_arr.len() != 16 { return Err("sgxtcbcomponents wrong size".to_string()); }
        for (i, item) in sgx_arr.iter().enumerate() {
            sgx[i] = item.get("svn").and_then(Value::as_u64).ok_or_else(|| "missing svn".to_string())? as u8;
        }
        if let Some(tdx_arr) = tcb.get("tdxtcbcomponents").and_then(Value::as_array) {
            if tdx_arr.len() != 16 { return Err("tdxtcbcomponents wrong size".to_string()); }
            for (i, item) in tdx_arr.iter().enumerate() {
                tdx[i] = item.get("svn").and_then(Value::as_u64).ok_or_else(|| "missing svn".to_string())? as u8;
            }
        }
    } else {
        // v2 schema: flat sgxtcbcompXXsvn keys
        for i in 0..16 {
            let key = format!("sgxtcbcomp{:02}svn", i + 1);
            sgx[i] = tcb.get(&key).and_then(Value::as_u64).ok_or_else(|| format!("missing {}", key))? as u8;
        }
    }
    Ok((sgx, tdx))
}

/// Replicates `FmspcTcbHelper.tcbLevelsObjToBytes`: slot1(32) | slot2(32) | advisoryIDs joined by '\n'.
fn pack_tcb_level(pcesvn: u16, tcb_date_ts: u64, status: u8, sgx: &[u8; 16], tdx: &[u8; 16], advisories: &[String]) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    // slot 1
    let mut s1 = [0u8; 32];
    s1[14..16].copy_from_slice(&pcesvn.to_be_bytes());
    s1[16..24].copy_from_slice(&tcb_date_ts.to_be_bytes());
    s1[31] = status;
    out.extend_from_slice(&s1);
    // slot 2
    let mut s2 = [0u8; 32];
    s2[0..16].copy_from_slice(sgx);
    if tdx.iter().any(|b| *b != 0) {
        s2[16..32].copy_from_slice(tdx);
    } else {
        // All-zero tdx svns: on-chain pack still writes them only if `tdxComponentCpuSvns.length > 0`.
        // The off-chain parser only fills tdx when sgxtcbcomponents is present and tdxtcbcomponents
        // had content. We mirror this by always-zero second half here; the on-chain serializer reads
        // tdx slot regardless, but it will be all-zero in both places for SGX-only.
        s2[16..32].copy_from_slice(tdx);
    }
    out.extend_from_slice(&s2);
    // advisoryIDs
    if !advisories.is_empty() {
        out.extend_from_slice(advisories.join("\n").as_bytes());
    }
    out
}

fn build_level_stream(levels: &[LevelEntry], _has_tdx: bool) -> Vec<u8> {
    let mut stream = Vec::new();
    for level in levels {
        stream.extend_from_slice(&(level.packed.len() as u32).to_be_bytes());
        stream.extend_from_slice(&level.packed);
        stream.extend_from_slice(&level.byte_start.to_be_bytes());
        stream.extend_from_slice(&level.byte_end.to_be_bytes());
        stream.extend_from_slice(&level.raw_tcb_date);
    }
    stream
}

/* ----- identity extraction + packing ----- */

struct IdentityEntry {
    byte_start: u32,
    byte_end: u32,
    packed: Vec<u8>,
    raw_mrsigner_hex: [u8; 96],
    raw_attr_hex: [u8; 16],
    raw_attr_mask_hex: [u8; 16],
    nested_dates: Vec<[u8; 20]>,
    nested_advisories: Vec<Vec<String>>,
}

fn extract_identities(raw: &[u8], arr_start: usize, arr_end: usize) -> Result<Vec<IdentityEntry>, String> {
    let bounds = brace_scan_items(raw, arr_start, arr_end);
    let mut out = Vec::with_capacity(bounds.len());
    for (s, e) in bounds {
        let bytes = &raw[s..e];
        let id_json: Value = serde_json::from_slice(bytes).map_err(|err| format!("identity parse failed: {:?}", err))?;

        let id_str = id_json.get("id").and_then(Value::as_str).ok_or_else(|| "missing id".to_string())?;
        let mrsigner_hex_str =
            id_json.get("mrsigner").and_then(Value::as_str).ok_or_else(|| "missing mrsigner".to_string())?;
        let attr_hex_str =
            id_json.get("attributes").and_then(Value::as_str).ok_or_else(|| "missing attributes".to_string())?;
        let attr_mask_hex_str = id_json
            .get("attributesMask")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing attributesMask".to_string())?;
        let mrsigner_bytes = hex::decode(mrsigner_hex_str).map_err(|err| format!("bad mrsigner hex: {:?}", err))?;
        if mrsigner_bytes.len() != 48 { return Err("mrsigner wrong length".to_string()); }
        let attr_bytes = hex::decode(attr_hex_str).map_err(|err| format!("bad attributes hex: {:?}", err))?;
        if attr_bytes.len() != 8 { return Err("attributes wrong length".to_string()); }
        let attr_mask_bytes =
            hex::decode(attr_mask_hex_str).map_err(|err| format!("bad attributesMask hex: {:?}", err))?;
        if attr_mask_bytes.len() != 8 { return Err("attributesMask wrong length".to_string()); }

        let nested = id_json
            .get("tcbLevels")
            .and_then(Value::as_array)
            .ok_or_else(|| "missing nested tcbLevels".to_string())?;
        let mut nested_packed = Vec::new();
        let mut nested_dates = Vec::new();
        let mut nested_advisories: Vec<Vec<String>> = Vec::new();
        for lvl in nested {
            let tcb = lvl.get("tcb").ok_or_else(|| "nested missing tcb".to_string())?;
            let isvsvn = tcb.get("isvsvn").and_then(Value::as_u64).ok_or_else(|| "missing isvsvn".to_string())? as u8;
            let date_str = lvl.get("tcbDate").and_then(Value::as_str).ok_or_else(|| "missing nested tcbDate".to_string())?;
            if date_str.len() != 20 { return Err("nested tcbDate wrong length".to_string()); }
            let mut date_arr = [0u8; 20];
            date_arr.copy_from_slice(date_str.as_bytes());
            let ts = iso_to_unix(date_str).ok_or_else(|| "nested tcbDate parse failed".to_string())?;
            let status_str = lvl.get("tcbStatus").and_then(Value::as_str).ok_or_else(|| "missing nested tcbStatus".to_string())?;
            let status = status_string_to_enum(status_str)?;

            // Pack nested slot: (isvsvn << 128) | (ts << 64) | status
            let mut slot = [0u8; 32];
            slot[15] = isvsvn;
            slot[16..24].copy_from_slice(&ts.to_be_bytes());
            slot[31] = status;
            nested_packed.extend_from_slice(&slot);
            nested_dates.push(date_arr);

            let advisories = lvl
                .get("advisoryIDs")
                .and_then(Value::as_array)
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
                .unwrap_or_default();
            nested_advisories.push(advisories);
        }

        // Pack identity per FmspcTcbHelper.tdxModuleIdentityToBytes: slot1 packOne(id), slot2 mrsigner[0..32],
        // slot3 mrsigner[32..48] + 16 zero bytes, slot4 attributes(high8)|attributesMask(at[16..24]), then nested slots.
        let mut packed = Vec::with_capacity(128 + nested_packed.len());
        packed.extend_from_slice(&pack_one(id_str));
        packed.extend_from_slice(&mrsigner_bytes[0..32]);
        let mut s3 = [0u8; 32];
        s3[0..16].copy_from_slice(&mrsigner_bytes[32..48]);
        packed.extend_from_slice(&s3);
        let mut s4 = [0u8; 32];
        s4[0..8].copy_from_slice(&attr_bytes);
        s4[16..24].copy_from_slice(&attr_mask_bytes);
        packed.extend_from_slice(&s4);
        packed.extend_from_slice(&nested_packed);

        let mut mrsigner_hex_arr = [0u8; 96];
        mrsigner_hex_arr.copy_from_slice(mrsigner_hex_str.as_bytes());
        let mut attr_hex_arr = [0u8; 16];
        attr_hex_arr.copy_from_slice(attr_hex_str.as_bytes());
        let mut attr_mask_hex_arr = [0u8; 16];
        attr_mask_hex_arr.copy_from_slice(attr_mask_hex_str.as_bytes());

        out.push(IdentityEntry {
            byte_start: s as u32,
            byte_end: e as u32,
            packed,
            raw_mrsigner_hex: mrsigner_hex_arr,
            raw_attr_hex: attr_hex_arr,
            raw_attr_mask_hex: attr_mask_hex_arr,
            nested_dates,
            nested_advisories,
        });
    }
    Ok(out)
}

fn build_identity_stream(identities: &[IdentityEntry]) -> Vec<u8> {
    let mut stream = Vec::new();
    for id in identities {
        stream.extend_from_slice(&(id.packed.len() as u32).to_be_bytes());
        stream.extend_from_slice(&id.packed);
        stream.extend_from_slice(&id.byte_start.to_be_bytes());
        stream.extend_from_slice(&id.byte_end.to_be_bytes());
        stream.extend_from_slice(&id.raw_mrsigner_hex);
        stream.extend_from_slice(&id.raw_attr_hex);
        stream.extend_from_slice(&id.raw_attr_mask_hex);
        stream.extend_from_slice(&(id.nested_dates.len() as u32).to_be_bytes());
        for (j, date) in id.nested_dates.iter().enumerate() {
            stream.extend_from_slice(date);
            let advisories = &id.nested_advisories[j];
            stream.extend_from_slice(&(advisories.len() as u16).to_be_bytes());
            for adv in advisories {
                stream.extend_from_slice(&(adv.len() as u16).to_be_bytes());
                stream.extend_from_slice(adv.as_bytes());
            }
        }
    }
    stream
}

/* ----- components template extraction ----- */

fn extract_components_template(raw: &[u8], start: u32, end: u32, key: &[u8]) -> Option<Vec<u8>> {
    let start = start as usize;
    let end = end as usize;
    let mut p = start;
    while p + key.len() <= end {
        if &raw[p..p + key.len()] == key {
            let val_start = p + key.len();
            if raw.get(val_start) != Some(&b'[') { return None; }
            let mut depth: i32 = 0;
            let mut q = val_start;
            while q < end {
                match raw[q] {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            return Some(raw[val_start..q + 1].to_vec());
                        }
                    }
                    b'"' => {
                        q += 1;
                        while q < end && raw[q] != b'"' {
                            if raw[q] == b'\\' { q += 1; }
                            q += 1;
                        }
                    }
                    _ => {}
                }
                q += 1;
            }
            return None;
        }
        p += 1;
    }
    None
}

/* ----- helpers: status enum, ISO date, packOne ----- */

fn status_string_to_enum(s: &str) -> Result<u8, String> {
    Ok(match s {
        "UpToDate" => 0,
        "SWHardeningNeeded" => 1,
        "ConfigurationAndSWHardeningNeeded" => 2,
        "ConfigurationNeeded" => 3,
        "OutOfDate" => 4,
        "OutOfDateConfigurationNeeded" => 5,
        "Revoked" => 6,
        other => return Err(format!("unknown tcbStatus: {}", other)),
    })
}

/// Parse `"YYYY-MM-DDTHH:MM:SSZ"` to a unix timestamp (seconds since 1970-01-01 UTC).
fn iso_to_unix(s: &str) -> Option<u64> {
    let b = s.as_bytes();
    if b.len() != 20 || b[4] != b'-' || b[7] != b'-' || b[10] != b'T' || b[13] != b':' || b[16] != b':' || b[19] != b'Z' {
        return None;
    }
    let n = |o: usize, l: usize| -> Option<u64> {
        let s = std::str::from_utf8(&b[o..o + l]).ok()?;
        s.parse::<u64>().ok()
    };
    let y = n(0, 4)?;
    let m = n(5, 2)?;
    let d = n(8, 2)?;
    let h = n(11, 2)?;
    let min = n(14, 2)?;
    let sec = n(17, 2)?;
    Some(days_from_civil(y, m, d) * 86400 + h * 3600 + min * 60 + sec)
}

/// Howard Hinnant's date algorithm: days since 1970-01-01 for civil (y, m, d).
fn days_from_civil(y: u64, m: u64, d: u64) -> u64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Solady LibString.packOne layout: byte0 = length, byte1..length+1 = string data, rest zero.
fn pack_one(s: &str) -> [u8; 32] {
    let b = s.as_bytes();
    assert!(b.len() < 32, "id too long for packOne");
    let mut out = [0u8; 32];
    out[0] = b.len() as u8;
    out[1..1 + b.len()].copy_from_slice(b);
    out
}

pub(crate) async fn send_transaction<M, D>(
    signer: &M,
    call: ContractCall<M, D>,
    log_prefix: &str,
    label: &str,
) -> bool
where
    M: Middleware,
    M::Error: 'static,
    D: Detokenize,
{
    let gas_with_buf = match gas_limit_with_fallback(signer, &call.tx, log_prefix, label).await {
        Some(g) => g,
        None => return false,
    };

    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!("{} txn[{}] hash: {:?}", log_prefix, label, pending_tx.tx_hash());
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!("{} txn[{}] receipt: {:?}", log_prefix, label, receipt);
                    match receipt.and_then(|r| r.status) {
                        Some(status) if status == U64::from(1_u64) => true,
                        Some(status) => {
                            log::error!(
                                "{} txn[{}] reverted on-chain with status {:?}",
                                log_prefix,
                                label,
                                status
                            );
                            false
                        }
                        None => {
                            log::error!(
                                "{} txn[{}] missing transaction status in receipt",
                                log_prefix,
                                label
                            );
                            false
                        }
                    }
                }
                Ok(Err(err)) => {
                    log::error!("{} txn[{}] receipt meet error: {:?}", log_prefix, label, err);
                    false
                }
                Err(_) => {
                    log::error!(
                        "{} txn[{}] timeout waiting for confirmation after {:?}",
                        log_prefix,
                        label,
                        TX_CONFIRMATION_TIMEOUT
                    );
                    false
                }
            }
        }
        Err(err) => {
            log::error!("{} txn[{}] meet error: {:?}", log_prefix, label, err);
            false
        }
    }
}

pub(crate) async fn gas_limit_with_fallback<M: Middleware>(
    signer: &M,
    tx: &TypedTransaction,
    log_prefix: &str,
    label: &str,
) -> Option<U256>
where
    M::Error: 'static,
{
    if let Some(gas) = estimate_gas_at_latest(signer, tx, log_prefix, label).await {
        return Some(gas);
    }

    let fallback = std::env::var(FALLBACK_GAS_LIMIT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(U256::from);
    if let Some(gas) = fallback {
        log::warn!(
            "{} txn[{}] using fallback gas limit from {}: {}",
            log_prefix,
            label,
            FALLBACK_GAS_LIMIT_ENV,
            gas
        );
        return Some(gas);
    }

    None
}

pub(crate) fn extract_tcb_info_str(raw_json: &str) -> Result<String, String> {
    let start = r#""tcbInfo":"#;
    let end = r#","signature""#;
    let trimmed = raw_json.trim();
    let start_idx = trimmed
        .find(start)
        .ok_or_else(|| "unexpected tcb payload prefix".to_string())?;
    let tail = &trimmed[start_idx + start.len()..];
    let end_idx = tail
        .find(end)
        .ok_or_else(|| "unable to locate signature delimiter".to_string())?;
    Ok(tail[..end_idx].to_string())
}

pub(crate) fn parse_tcb_type(value: &str) -> u8 {
    if value.eq_ignore_ascii_case("tdx") {
        1
    } else {
        0
    }
}

pub(crate) fn fallback_tcb_version(value: &str) -> Option<u32> {
    match value {
        "v3" => Some(2),
        "v4" | "v5" => Some(3),
        _ => None,
    }
}

pub(crate) fn generate_ref_id(locator: TcbLocator, tcb_info_obj: &TcbInfoJsonObj) -> [u8; 32] {
    let now_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    keccak256(format!(
        "fmspc-tcb-async:{}:{}:{}:{}:{}",
        hex::encode(locator.fmspc),
        locator.tcb_type,
        locator.version,
        now_nanos,
        hex::encode(tcb_info_obj.signature.as_ref())
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal SGX v3 inner tcbInfo with 1 level — sanity-check that brace scanner finds the
    /// outer tcbLevels array bounds and the per-level boundaries inside it.
    const SGX_V3_INNER: &str = r#"{"id":"SGX","version":3,"issueDate":"2024-07-03T13:09:33Z","nextUpdate":"2024-08-02T13:09:33Z","fmspc":"10A06D070000","pceId":"0000","tcbType":0,"tcbEvaluationDataNumber":16,"tcbLevels":[{"tcb":{"sgxtcbcomponents":[{"svn":1,"category":"BIOS","type":"Early Microcode Update"},{"svn":1,"category":"OS/VMM","type":"SGX Late Microcode Update"},{"svn":0,"category":"OS/VMM","type":"TXT SINIT"},{"svn":0,"category":"BIOS"},{"svn":1,"category":"BIOS"},{"svn":255,"category":"BIOS"},{"svn":0},{"svn":1,"category":"OS/VMM","type":"SEAMLDR ACM"},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":13},"tcbDate":"2023-08-09T00:00:00Z","tcbStatus":"UpToDate"}]}"#;

    #[test]
    fn finds_outer_tcb_levels_array() {
        let raw = SGX_V3_INNER.as_bytes();
        let (s, e) = find_top_level_array(raw, b"tcbLevels").expect("must locate outer array");
        assert_eq!(raw[s], b'[');
        assert_eq!(raw[e - 1], b']');
        // No tdxModuleIdentities in SGX-only fixture.
        assert!(find_top_level_array(raw, b"tdxModuleIdentities").is_none());
    }

    #[test]
    fn brace_scans_single_level() {
        let raw = SGX_V3_INNER.as_bytes();
        let (s, e) = find_top_level_array(raw, b"tcbLevels").unwrap();
        let items = brace_scan_items(raw, s, e);
        assert_eq!(items.len(), 1);
        // The item's content must start with `{` and end with `}`.
        let (start, end) = items[0];
        assert_eq!(raw[start], b'{');
        assert_eq!(raw[end - 1], b'}');
    }

    #[test]
    fn iso_to_unix_known_values() {
        // 1970-01-01T00:00:00Z → 0
        assert_eq!(iso_to_unix("1970-01-01T00:00:00Z"), Some(0));
        // 2024-03-13T00:00:00Z → 1710288000 (verified against `date -u -d '2024-03-13T00:00:00Z' +%s`)
        assert_eq!(iso_to_unix("2024-03-13T00:00:00Z"), Some(1710288000));
        // Leap year boundary: 2020-02-29T12:34:56Z
        assert_eq!(iso_to_unix("2020-02-29T12:34:56Z"), Some(1582979696));
    }

    #[test]
    fn extracts_sgx_components_template() {
        let raw = SGX_V3_INNER.as_bytes();
        let (s, e) = find_top_level_array(raw, b"tcbLevels").unwrap();
        let items = brace_scan_items(raw, s, e);
        let tpl = extract_components_template(raw, items[0].0 as u32, items[0].1 as u32, b"\"sgxtcbcomponents\":")
            .expect("template should be present");
        assert_eq!(tpl[0], b'[');
        assert_eq!(*tpl.last().unwrap(), b']');
        // Template must contain 16 `"svn":` occurrences.
        let svn_count = tpl.windows(6).filter(|w| *w == b"\"svn\":").count();
        assert_eq!(svn_count, 16);
    }

    #[test]
    fn pack_one_layout() {
        let p = pack_one("TDX_01");
        assert_eq!(p[0], 6);
        assert_eq!(&p[1..7], b"TDX_01");
        for b in &p[7..] {
            assert_eq!(*b, 0);
        }
    }

    #[test]
    fn dump_live_eval19_stream() {
        // Reads the live Intel response captured under /tmp and dumps the constructed level
        // stream + key offsets for comparison against the failing contract call.
        let path = "/tmp/intel-tcb-eval19.json";
        let raw_json = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return, // skip when the fixture isn't present locally
        };
        let inner = super::extract_tcb_info_str(&raw_json).unwrap();
        let raw = inner.as_bytes();
        let ranges = find_array_ranges(raw).unwrap();
        eprintln!("inner len: {}", raw.len());
        eprintln!(
            "tcb_levels_start={}  tcb_levels_end={}",
            ranges.tcb_levels_start, ranges.tcb_levels_end
        );
        eprintln!("byte at tcb_levels_start: {:?}", raw[ranges.tcb_levels_start] as char);
        eprintln!("byte at tcb_levels_end-1: {:?}", raw[ranges.tcb_levels_end - 1] as char);
        let levels = extract_levels(raw, ranges.tcb_levels_start, ranges.tcb_levels_end, 3).unwrap();
        eprintln!("levels: {}", levels.len());
        for (i, l) in levels.iter().enumerate() {
            eprintln!(
                "  [{}] byteStart={} byteEnd={} packedLen={} firstBytePacked=0x{:02x} rawDate={}",
                i,
                l.byte_start,
                l.byte_end,
                l.packed.len(),
                l.packed[0],
                std::str::from_utf8(&l.raw_tcb_date).unwrap()
            );
        }
        let template = extract_components_template(raw, levels[0].byte_start, levels[0].byte_end, b"\"sgxtcbcomponents\":");
        eprintln!(
            "sgx template found: {}",
            template.as_ref().map(|t| t.len()).unwrap_or(0)
        );
        let stream = build_level_stream(&levels, false);
        eprintln!("level_stream len: {}", stream.len());
        // Dump the first level item entirely (4-byte length prefix + packed + 4+4+20 metadata)
        let item0_size = 4 + 78 + 4 + 4 + 20;
        eprintln!("first item ({}B) hex: {}", item0_size, hex::encode(&stream[..item0_size]));
        // Show just the packed bytes (skip the 4-byte length prefix)
        eprintln!("level0 packed: {}", hex::encode(&stream[4..4 + 78]));
    }
}
