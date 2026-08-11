//! Offchain driver for the optimized FmspcTcbDaoV2 async upsert protocol.
//!
//! The worker parses Intel's minified inner `tcbInfo` JSON offchain, uploads typed
//! values plus source-order metadata, and lets the contract rebuild the signed raw
//! string while storing the legacy packed representation. Finalization verifies the
//! rebuilt raw against Intel's signature, so the chain no longer needs a JSON parser
//! or a second raw-vs-parsed consistency pass.

use crate::helper::{estimate_gas_at_latest, fetch_intel_collateral_with_eval_fallback};
use crate::pccs_types::TcbInfo;
use automata_dcap_qpl_contracts::{
    fmspc_tcb_dao::TcbInfoJsonObj, parse_address_from_env_var::parse_address_from_str,
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
const DEFAULT_ASYNC_PARSE_BATCH_SIZE: u64 = 3;
const FALLBACK_GAS_LIMIT_ENV: &str = "QPL_FALLBACK_GAS_LIMIT";
const ASYNC_PARSE_BATCH_SIZE_ENV: &str = "QPL_ASYNC_PARSE_BATCH_SIZE";

const TOP_FIELDS: [&str; 11] = [
    "id",
    "version",
    "issueDate",
    "nextUpdate",
    "fmspc",
    "pceId",
    "tcbType",
    "tcbEvaluationDataNumber",
    "tdxModule",
    "tdxModuleIdentities",
    "tcbLevels",
];
const TOP_ID: usize = 0;
const TOP_VERSION: usize = 1;
const TOP_ISSUE_DATE: usize = 2;
const TOP_NEXT_UPDATE: usize = 3;
const TOP_FMSPC: usize = 4;
const TOP_PCE_ID: usize = 5;
const TOP_TCB_TYPE: usize = 6;
const TOP_EVAL_NUMBER: usize = 7;
const TOP_TDX_MODULE: usize = 8;
const TOP_TDX_IDENTITIES: usize = 9;
const TOP_TCB_LEVELS: usize = 10;

const LEVEL_FIELDS: [&str; 4] = ["tcb", "tcbDate", "tcbStatus", "advisoryIDs"];
const TCB_FIELDS: [&str; 3] = ["sgxtcbcomponents", "pcesvn", "tdxtcbcomponents"];
const COMPONENT_FIELDS: [&str; 3] = ["svn", "category", "type"];
const IDENTITY_FIELDS: [&str; 5] = [
    "id",
    "mrsigner",
    "attributes",
    "attributesMask",
    "tcbLevels",
];
const TDX_MODULE_FIELDS: [&str; 3] = ["mrsigner", "attributes", "attributesMask"];

const LEVEL_FLAG_HAS_ADVISORY_FIELD: u8 = 1;
const LEVEL_FLAG_SGX_LAYOUT_OVERRIDE: u8 = 2;
const LEVEL_FLAG_TDX_LAYOUT_OVERRIDE: u8 = 4;

const TCB_LEVEL_PACKED_BASE_LEN: usize = 64;
const TDX_IDENTITY_PACKED_BASE_LEN: usize = 128;

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
        function asyncUpsertProtocolVersion() pure returns (uint8)
        function startAsyncUpsert(bytes32 refId, bytes signature, uint32 rawLength)
        function uploadBasicInfo(bytes32 refId, bytes basicPayload, bytes topLevelOrder)
        function uploadTcbLevelsBatch(bytes32 refId, uint256 start, uint256 itemCount, bytes payload) returns (uint256 parsed, uint256 total, bool complete)
        function uploadTdxModuleIdentitiesBatch(bytes32 refId, uint256 start, uint256 itemCount, bytes payload) returns (uint256 parsed, uint256 total, bool complete)
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
        match fetch_payload(
            &log_prefix,
            fmspc,
            platform,
            version,
            collateral_update_type,
            tcb_evaluation_data_number,
        )
        .await
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
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let dao = FmspcTcbDaoV2::new(
        parse_address_from_str(fmspc_tcb_dao_contract_addr),
        signer.clone(),
    );

    run_upsert(
        &dao,
        signer.as_ref(),
        &log_prefix,
        gas_price,
        &raw_inner,
        &tcb_info_obj,
        locator,
    )
    .await
}

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
    let plan = match build_async_plan(raw, locator) {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} async V2 payload planning failed: {}", log_prefix, err);
            return false;
        }
    };
    let ref_id = generate_ref_id(locator, tcb_info_obj);

    match dao.async_upsert_protocol_version().call().await {
        Ok(2) => {}
        Ok(other) => {
            log::error!(
                "{} unsupported async protocol version returned by target: {}",
                log_prefix,
                other
            );
            return false;
        }
        Err(err) => {
            log::error!(
                "{} asyncUpsertProtocolVersion call failed: {:?}",
                log_prefix,
                err
            );
            return false;
        }
    }

    if !send_transaction(
        signer,
        dao.start_async_upsert(ref_id, tcb_info_obj.signature.clone(), raw.len() as u32)
            .gas_price(gas_price),
        log_prefix,
        "start_async_upsert",
    )
    .await
    {
        return false;
    }

    if !send_transaction(
        signer,
        dao.upload_basic_info(
            ref_id,
            plan.basic_payload.into(),
            plan.top_level_order.to_vec().into(),
        )
        .gas_price(gas_price),
        log_prefix,
        "upload_basic_info",
    )
    .await
    {
        return false;
    }

    let batch_size = async_parse_batch_size() as usize;
    let mut sent = 0usize;
    while sent < plan.levels.len() {
        let end = std::cmp::min(sent + batch_size, plan.levels.len());
        let payload = build_level_batch_payload(
            &plan.levels[sent..end],
            plan.has_tdx_components,
            locator.version,
        );
        let label = format!("upload_tcb_levels_batch_{}_{}", sent, end);
        if !send_transaction(
            signer,
            dao.upload_tcb_levels_batch(
                ref_id,
                U256::from(sent),
                U256::from(end - sent),
                payload.into(),
            )
            .gas_price(gas_price),
            log_prefix,
            &label,
        )
        .await
        {
            return false;
        }
        sent = end;
    }

    let mut sent = 0usize;
    while sent < plan.identities.len() {
        let end = std::cmp::min(sent + batch_size, plan.identities.len());
        let payload = build_identity_batch_payload(&plan.identities[sent..end]);
        let label = format!("upload_tdx_module_identities_batch_{}_{}", sent, end);
        if !send_transaction(
            signer,
            dao.upload_tdx_module_identities_batch(
                ref_id,
                U256::from(sent),
                U256::from(end - sent),
                payload.into(),
            )
            .gas_price(gas_price),
            log_prefix,
            &label,
        )
        .await
        {
            return false;
        }
        sent = end;
    }

    let tcb_key = match dao
        .fmspc_tcb_key(locator.tcb_type, locator.fmspc, locator.version)
        .call()
        .await
    {
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
        dao.finalize_async_upsert(attestation_id, ref_id)
            .gas_price(gas_price),
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
    let raw =
        fs::read_to_string(path).map_err(|err| format!("unable to read {}: {:?}", path, err))?;
    let trimmed = raw.trim();
    let payload = if trimmed.starts_with(r#"{"tcbInfo":{"#) && trimmed.contains(r#","signature":"#)
    {
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
    } else if sibling_path.extension().and_then(|value| value.to_str()) == Some("json") {
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
    let base_url = format!(
        "https://api.trustedservices.intel.com/{}/certification/{}/tcb?fmspc={}",
        platform, version, fmspc
    );
    let body = fetch_intel_collateral_with_eval_fallback(
        &base_url,
        collateral_update_type,
        tcb_evaluation_data_number,
        "tcbInfo",
        log_prefix,
    )
    .await?;
    parse_payload(&body, platform, version)
}

fn parse_payload(
    raw_json: &str,
    platform: &str,
    collateral_version: &str,
) -> Result<(String, TcbInfoJsonObj, TcbLocator), String> {
    let tcb_info: TcbInfo =
        serde_json::from_str(raw_json).map_err(|err| format!("invalid tcb payload: {:?}", err))?;
    let inner = extract_tcb_info_str(raw_json)?;
    let signature = Bytes::from_hex(tcb_info.signature)
        .map_err(|err| format!("invalid signature: {:?}", err))?;

    let fmspc_hex = tcb_info
        .tcb_info
        .get("fmspc")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing fmspc".to_string())?;
    let fmspc_vec = hex::decode(fmspc_hex)
        .map_err(|err| format!("invalid fmspc hex {}: {:?}", fmspc_hex, err))?;
    let fmspc: [u8; 6] = fmspc_vec
        .try_into()
        .map_err(|_| "unexpected fmspc length".to_string())?;
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

    let tcb_info_obj = TcbInfoJsonObj {
        tcb_info_str: inner.clone(),
        signature,
    };
    Ok((
        inner,
        tcb_info_obj,
        TcbLocator {
            tcb_type,
            fmspc,
            version,
        },
    ))
}

/* ----- async payload planning ----- */

#[derive(Clone, Debug)]
struct JsonField {
    key: String,
    key_start: usize,
    value_start: usize,
    value_end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ComponentDescriptor {
    svn_order: u8,
    category_order: u8,
    type_order: u8,
    category: Vec<u8>,
    component_type: Vec<u8>,
}

#[derive(Clone, Debug)]
struct LevelEntry {
    byte_start: u32,
    byte_end: u32,
    level_order: [u8; 4],
    tcb_order: [u8; 3],
    base_flags: u8,
    sgx_svns: [u8; 16],
    tdx_svns: [u8; 16],
    pcesvn: u16,
    raw_tcb_date: [u8; 20],
    status: u8,
    advisories: Vec<Vec<u8>>,
    sgx_layout: Vec<ComponentDescriptor>,
    tdx_layout: Vec<ComponentDescriptor>,
}

#[derive(Clone, Debug)]
struct NestedLevelEntry {
    level_order: [u8; 4],
    flags: u8,
    isvsvn: u8,
    raw_tcb_date: [u8; 20],
    status: u8,
    advisories: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
struct IdentityEntry {
    byte_start: u32,
    byte_end: u32,
    identity_order: [u8; 5],
    id_raw: Vec<u8>,
    mrsigner_hex: Vec<u8>,
    attributes_hex: Vec<u8>,
    attributes_mask_hex: Vec<u8>,
    nested_levels: Vec<NestedLevelEntry>,
}

struct AsyncPlan {
    basic_payload: Vec<u8>,
    top_level_order: [u8; 11],
    levels: Vec<LevelEntry>,
    identities: Vec<IdentityEntry>,
    has_tdx_components: bool,
}

fn build_async_plan(raw: &[u8], locator: TcbLocator) -> Result<AsyncPlan, String> {
    let top_fields = parse_object_fields(raw, 0, raw.len())?;
    let top_order = order_for_fields::<11>(&top_fields, &TOP_FIELDS)?;
    validate_top_order(&top_order, locator.version, locator.tcb_type)?;

    let tcb_levels_field = field(&top_fields, "tcbLevels")?;
    let level_bounds = object_items_in_array(
        raw,
        tcb_levels_field.value_start,
        tcb_levels_field.value_end,
    )?;
    let levels = extract_levels(raw, &level_bounds, locator.version)?;
    if levels.is_empty() {
        return Err("tcbLevels must not be empty".to_string());
    }

    let has_tdx_components = levels.iter().any(|level| !level.tdx_layout.is_empty());
    if has_tdx_components && levels.iter().any(|level| level.tdx_layout.len() != 16) {
        return Err(
            "tdxtcbcomponents must be present in every level when any level has it".to_string(),
        );
    }

    let identities = if let Some(ids_field) = maybe_field(&top_fields, "tdxModuleIdentities") {
        let bounds = object_items_in_array(raw, ids_field.value_start, ids_field.value_end)?;
        extract_identities(raw, &bounds)?
    } else {
        Vec::new()
    };

    let levels_stream_len = levels.iter().map(level_stream_item_len).sum::<usize>();
    let identities_stream_len = identities
        .iter()
        .map(identity_stream_item_len)
        .sum::<usize>();
    let basic_payload = build_basic_payload(
        raw,
        &top_fields,
        &top_order,
        &levels,
        &identities,
        levels_stream_len,
        identities_stream_len,
        locator,
    )?;

    Ok(AsyncPlan {
        basic_payload,
        top_level_order: top_order,
        levels,
        identities,
        has_tdx_components,
    })
}

fn validate_top_order(order: &[u8; 11], version: u32, tcb_type: u8) -> Result<(), String> {
    for i in [
        TOP_VERSION,
        TOP_ISSUE_DATE,
        TOP_NEXT_UPDATE,
        TOP_FMSPC,
        TOP_PCE_ID,
        TOP_TCB_TYPE,
        TOP_EVAL_NUMBER,
        TOP_TCB_LEVELS,
    ] {
        if order[i] == 0 {
            return Err(format!(
                "missing required top-level field {}",
                TOP_FIELDS[i]
            ));
        }
    }
    if version >= 3 && order[TOP_ID] == 0 {
        return Err("missing id in TCBInfo v3 payload".to_string());
    }
    if version < 3 && order[TOP_ID] != 0 {
        return Err("TCBInfo v2 payload unexpectedly contains id".to_string());
    }
    if tcb_type == 1 {
        if order[TOP_TDX_MODULE] == 0 || order[TOP_TDX_IDENTITIES] == 0 {
            return Err("TDX TCBInfo must include tdxModule and tdxModuleIdentities".to_string());
        }
    } else if order[TOP_TDX_MODULE] != 0 || order[TOP_TDX_IDENTITIES] != 0 {
        return Err("SGX TCBInfo must not include TDX module fields".to_string());
    }
    Ok(())
}

fn build_basic_payload(
    raw: &[u8],
    top_fields: &[JsonField],
    top_order: &[u8; 11],
    levels: &[LevelEntry],
    identities: &[IdentityEntry],
    levels_stream_len: usize,
    identities_stream_len: usize,
    locator: TcbLocator,
) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    for key in TOP_FIELDS {
        push_u32(
            &mut out,
            maybe_field(top_fields, key)
                .map(|f| f.key_start)
                .unwrap_or(0),
            "top field offset",
        )?;
    }

    let levels_field = field(top_fields, "tcbLevels")?;
    push_u32(&mut out, levels_field.value_start, "tcbLevels start")?;
    push_u32(&mut out, levels_field.value_end, "tcbLevels end")?;
    push_u32(&mut out, levels.len(), "tcbLevels count")?;

    if let Some(ids_field) = maybe_field(top_fields, "tdxModuleIdentities") {
        push_u32(&mut out, ids_field.value_start, "tdxModuleIdentities start")?;
        push_u32(&mut out, ids_field.value_end, "tdxModuleIdentities end")?;
        push_u32(&mut out, identities.len(), "tdxModuleIdentities count")?;
    } else {
        out.extend_from_slice(&[0u8; 12]);
    }

    if let Some(module_field) = maybe_field(top_fields, "tdxModule") {
        push_u32(&mut out, module_field.value_start, "tdxModule start")?;
        push_u32(&mut out, module_field.value_end, "tdxModule end")?;
    } else {
        out.extend_from_slice(&[0u8; 8]);
    }

    push_u32(&mut out, levels_stream_len, "levels stream length")?;
    push_u32(&mut out, identities_stream_len, "identities stream length")?;

    let id_byte = if top_order[TOP_ID] != 0 {
        let id_raw = string_contents(raw, field(top_fields, "id")?)?;
        let parsed =
            parse_tcb_type(std::str::from_utf8(&id_raw).map_err(|_| "id is not utf8".to_string())?);
        if parsed != locator.tcb_type {
            return Err("top-level id does not match selected platform".to_string());
        }
        parsed
    } else {
        locator.tcb_type
    };
    out.push(id_byte);

    push_u32_value(&mut out, u32_field(raw, top_fields, "version")?);
    push_fixed_string(&mut out, raw, top_fields, "issueDate", 20)?;
    push_fixed_string(&mut out, raw, top_fields, "nextUpdate", 20)?;
    push_fixed_string(&mut out, raw, top_fields, "fmspc", 12)?;
    push_fixed_string(&mut out, raw, top_fields, "pceId", 4)?;
    out.push(u8_field(raw, top_fields, "tcbType")?);
    push_u32_value(
        &mut out,
        u32_field(raw, top_fields, "tcbEvaluationDataNumber")?,
    );

    if let Some(module_field) = maybe_field(top_fields, "tdxModule") {
        let module_fields =
            parse_object_fields(raw, module_field.value_start, module_field.value_end)?;
        let module_order = order_for_fields::<3>(&module_fields, &TDX_MODULE_FIELDS)?;
        out.push(1);
        out.extend_from_slice(&module_order);
        push_fixed_string_from_fields(&mut out, raw, &module_fields, "mrsigner", 96)?;
        push_fixed_string_from_fields(&mut out, raw, &module_fields, "attributes", 16)?;
        push_fixed_string_from_fields(&mut out, raw, &module_fields, "attributesMask", 16)?;
    } else {
        out.push(0);
    }

    Ok(out)
}

fn extract_levels(
    raw: &[u8],
    bounds: &[(usize, usize)],
    version: u32,
) -> Result<Vec<LevelEntry>, String> {
    let mut out = Vec::with_capacity(bounds.len());
    for &(start, end) in bounds {
        let level_fields = parse_object_fields(raw, start, end)?;
        let level_order = order_for_fields::<4>(&level_fields, &LEVEL_FIELDS)?;
        let tcb_field = field(&level_fields, "tcb")?;
        let tcb_fields = parse_object_fields(raw, tcb_field.value_start, tcb_field.value_end)?;

        let mut base_flags = 0u8;
        if maybe_field(&level_fields, "advisoryIDs").is_some() {
            base_flags |= LEVEL_FLAG_HAS_ADVISORY_FIELD;
        }

        let (tcb_order, sgx_svns, tdx_svns, sgx_layout, tdx_layout, pcesvn) =
            extract_tcb_object(raw, &tcb_fields, version)?;
        let raw_tcb_date = fixed_string_array(raw, field(&level_fields, "tcbDate")?, 20)?;
        let status_raw = string_contents(raw, field(&level_fields, "tcbStatus")?)?;
        let status_str =
            std::str::from_utf8(&status_raw).map_err(|_| "tcbStatus is not utf8".to_string())?;
        let status = status_string_to_enum(status_str)?;
        let advisories = if let Some(advisory_field) = maybe_field(&level_fields, "advisoryIDs") {
            string_array(raw, advisory_field.value_start, advisory_field.value_end)?
        } else {
            Vec::new()
        };

        out.push(LevelEntry {
            byte_start: start as u32,
            byte_end: end as u32,
            level_order,
            tcb_order,
            base_flags,
            sgx_svns,
            tdx_svns,
            pcesvn,
            raw_tcb_date,
            status,
            advisories,
            sgx_layout,
            tdx_layout,
        });
    }
    Ok(out)
}

fn extract_tcb_object(
    raw: &[u8],
    tcb_fields: &[JsonField],
    version: u32,
) -> Result<
    (
        [u8; 3],
        [u8; 16],
        [u8; 16],
        Vec<ComponentDescriptor>,
        Vec<ComponentDescriptor>,
        u16,
    ),
    String,
> {
    let pcesvn = u32_field_from_fields(raw, tcb_fields, "pcesvn")? as u16;
    let mut sgx_svns = [0u8; 16];
    let mut tdx_svns = [0u8; 16];
    let mut sgx_layout = Vec::new();
    let mut tdx_layout = Vec::new();

    if version >= 3 {
        let tcb_order = order_for_fields::<3>(tcb_fields, &TCB_FIELDS)?;
        let sgx_field = field(tcb_fields, "sgxtcbcomponents")?;
        let (svns, layout) =
            extract_component_array(raw, sgx_field.value_start, sgx_field.value_end)?;
        sgx_svns = svns;
        sgx_layout = layout;
        if let Some(tdx_field) = maybe_field(tcb_fields, "tdxtcbcomponents") {
            let (svns, layout) =
                extract_component_array(raw, tdx_field.value_start, tdx_field.value_end)?;
            tdx_svns = svns;
            tdx_layout = layout;
        }
        Ok((
            tcb_order, sgx_svns, tdx_svns, sgx_layout, tdx_layout, pcesvn,
        ))
    } else {
        for i in 0..16 {
            let key = format!("sgxtcbcomp{:02}svn", i + 1);
            sgx_svns[i] = u32_field_from_fields(raw, tcb_fields, &key)? as u8;
        }
        Ok((
            [0, 0, 0],
            sgx_svns,
            tdx_svns,
            sgx_layout,
            tdx_layout,
            pcesvn,
        ))
    }
}

fn extract_component_array(
    raw: &[u8],
    start: usize,
    end: usize,
) -> Result<([u8; 16], Vec<ComponentDescriptor>), String> {
    let bounds = object_items_in_array(raw, start, end)?;
    if bounds.len() != 16 {
        return Err(format!(
            "component array has {} items, expected 16",
            bounds.len()
        ));
    }

    let mut svns = [0u8; 16];
    let mut layout = Vec::with_capacity(16);
    for (idx, (item_start, item_end)) in bounds.into_iter().enumerate() {
        let fields = parse_object_fields(raw, item_start, item_end)?;
        let order = order_for_fields::<3>(&fields, &COMPONENT_FIELDS)?;
        svns[idx] = u32_field_from_fields(raw, &fields, "svn")? as u8;
        let category = if let Some(field) = maybe_field(&fields, "category") {
            string_contents(raw, field)?
        } else {
            Vec::new()
        };
        let component_type = if let Some(field) = maybe_field(&fields, "type") {
            string_contents(raw, field)?
        } else {
            Vec::new()
        };
        layout.push(ComponentDescriptor {
            svn_order: order[0],
            category_order: order[1],
            type_order: order[2],
            category,
            component_type,
        });
    }

    Ok((svns, layout))
}

fn extract_identities(raw: &[u8], bounds: &[(usize, usize)]) -> Result<Vec<IdentityEntry>, String> {
    let mut out = Vec::with_capacity(bounds.len());
    for &(start, end) in bounds {
        let fields = parse_object_fields(raw, start, end)?;
        let identity_order = order_for_fields::<5>(&fields, &IDENTITY_FIELDS)?;
        let levels_field = field(&fields, "tcbLevels")?;
        let nested_bounds =
            object_items_in_array(raw, levels_field.value_start, levels_field.value_end)?;
        let nested_levels = extract_nested_levels(raw, &nested_bounds)?;

        out.push(IdentityEntry {
            byte_start: start as u32,
            byte_end: end as u32,
            identity_order,
            id_raw: string_contents(raw, field(&fields, "id")?)?,
            mrsigner_hex: fixed_string_vec(raw, field(&fields, "mrsigner")?, 96)?,
            attributes_hex: fixed_string_vec(raw, field(&fields, "attributes")?, 16)?,
            attributes_mask_hex: fixed_string_vec(raw, field(&fields, "attributesMask")?, 16)?,
            nested_levels,
        });
    }
    Ok(out)
}

fn extract_nested_levels(
    raw: &[u8],
    bounds: &[(usize, usize)],
) -> Result<Vec<NestedLevelEntry>, String> {
    let mut out = Vec::with_capacity(bounds.len());
    for &(start, end) in bounds {
        let level_fields = parse_object_fields(raw, start, end)?;
        let level_order = order_for_fields::<4>(&level_fields, &LEVEL_FIELDS)?;
        let mut flags = 0u8;
        if maybe_field(&level_fields, "advisoryIDs").is_some() {
            flags |= LEVEL_FLAG_HAS_ADVISORY_FIELD;
        }

        let tcb_field = field(&level_fields, "tcb")?;
        let tcb_fields = parse_object_fields(raw, tcb_field.value_start, tcb_field.value_end)?;
        if tcb_fields.len() != 1 || tcb_fields[0].key != "isvsvn" {
            return Err("TDX module identity nested tcb must only contain isvsvn".to_string());
        }

        let raw_tcb_date = fixed_string_array(raw, field(&level_fields, "tcbDate")?, 20)?;
        let status_raw = string_contents(raw, field(&level_fields, "tcbStatus")?)?;
        let status_str = std::str::from_utf8(&status_raw)
            .map_err(|_| "nested tcbStatus is not utf8".to_string())?;
        let advisories = if let Some(advisory_field) = maybe_field(&level_fields, "advisoryIDs") {
            string_array(raw, advisory_field.value_start, advisory_field.value_end)?
        } else {
            Vec::new()
        };

        out.push(NestedLevelEntry {
            level_order,
            flags,
            isvsvn: u32_field_from_fields(raw, &tcb_fields, "isvsvn")? as u8,
            raw_tcb_date,
            status: status_string_to_enum(status_str)?,
            advisories,
        });
    }
    Ok(out)
}

/* ----- binary payload builders ----- */

fn build_level_batch_payload(
    levels: &[LevelEntry],
    has_tdx_components: bool,
    version: u32,
) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(if has_tdx_components { 1 } else { 0 });

    let sgx_header = if version >= 3 {
        Some(&levels[0].sgx_layout)
    } else {
        None
    };
    let tdx_header = if version >= 3 && has_tdx_components {
        Some(&levels[0].tdx_layout)
    } else {
        None
    };

    if let Some(layout) = sgx_header {
        append_component_layout(&mut out, layout);
    }
    if let Some(layout) = tdx_header {
        append_component_layout(&mut out, layout);
    }

    for level in levels {
        push_u32_value(&mut out, level.byte_start);
        push_u32_value(&mut out, level.byte_end);
        out.extend_from_slice(&level.level_order);
        out.extend_from_slice(&level.tcb_order);
        let mut flags = level.base_flags;
        // Keep batch headers as the common case, but attach a per-level descriptor
        // when Intel returns a valid level whose component metadata order differs.
        if let Some(layout) = sgx_header {
            if level.sgx_layout != *layout {
                flags |= LEVEL_FLAG_SGX_LAYOUT_OVERRIDE;
            }
        }
        if let Some(layout) = tdx_header {
            if level.tdx_layout != *layout {
                flags |= LEVEL_FLAG_TDX_LAYOUT_OVERRIDE;
            }
        }
        out.push(flags);
        out.extend_from_slice(&level.sgx_svns);
        if has_tdx_components {
            out.extend_from_slice(&level.tdx_svns);
        }
        push_u32_value(&mut out, level.pcesvn as u32);
        out.extend_from_slice(&level.raw_tcb_date);
        out.push(level.status);
        append_bytes_array(&mut out, &level.advisories);

        if flags & LEVEL_FLAG_SGX_LAYOUT_OVERRIDE != 0 {
            append_component_layout(&mut out, &level.sgx_layout);
        }
        if flags & LEVEL_FLAG_TDX_LAYOUT_OVERRIDE != 0 {
            append_component_layout(&mut out, &level.tdx_layout);
        }
    }

    out
}

fn build_identity_batch_payload(identities: &[IdentityEntry]) -> Vec<u8> {
    let mut out = Vec::new();
    for identity in identities {
        push_u32_value(&mut out, identity.byte_start);
        push_u32_value(&mut out, identity.byte_end);
        out.extend_from_slice(&identity.identity_order);
        out.push(identity.id_raw.len() as u8);
        out.extend_from_slice(&identity.id_raw);
        out.extend_from_slice(&identity.mrsigner_hex);
        out.extend_from_slice(&identity.attributes_hex);
        out.extend_from_slice(&identity.attributes_mask_hex);
        push_u32_value(&mut out, identity.nested_levels.len() as u32);

        for level in &identity.nested_levels {
            out.extend_from_slice(&level.level_order);
            out.push(level.flags);
            out.push(level.isvsvn);
            out.extend_from_slice(&level.raw_tcb_date);
            out.push(level.status);
            append_bytes_array(&mut out, &level.advisories);
        }
    }
    out
}

fn append_component_layout(out: &mut Vec<u8>, layout: &[ComponentDescriptor]) {
    assert_eq!(
        layout.len(),
        16,
        "component layout must have 16 descriptors"
    );
    for descriptor in layout {
        out.push(descriptor.svn_order);
        out.push(descriptor.category_order);
        out.push(descriptor.type_order);
        append_u16_bytes(out, &descriptor.category);
        append_u16_bytes(out, &descriptor.component_type);
    }
}

fn append_bytes_array(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    push_u32_value(out, values.len() as u32);
    for value in values {
        append_u16_bytes(out, value);
    }
}

fn append_u16_bytes(out: &mut Vec<u8>, value: &[u8]) {
    assert!(value.len() <= u16::MAX as usize, "string item too long");
    out.extend_from_slice(&(value.len() as u16).to_be_bytes());
    out.extend_from_slice(value);
}

fn level_stream_item_len(level: &LevelEntry) -> usize {
    4 + TCB_LEVEL_PACKED_BASE_LEN + joined_advisory_len(&level.advisories)
}

fn identity_stream_item_len(identity: &IdentityEntry) -> usize {
    4 + TDX_IDENTITY_PACKED_BASE_LEN + 32 * identity.nested_levels.len()
}

fn joined_advisory_len(values: &[Vec<u8>]) -> usize {
    let bytes = values.iter().map(Vec::len).sum::<usize>();
    if values.is_empty() {
        bytes
    } else {
        bytes + values.len() - 1
    }
}

fn push_fixed_string(
    out: &mut Vec<u8>,
    raw: &[u8],
    fields: &[JsonField],
    key: &str,
    len: usize,
) -> Result<(), String> {
    let value = fixed_string_vec(raw, field(fields, key)?, len)?;
    out.extend_from_slice(&value);
    Ok(())
}

fn push_fixed_string_from_fields(
    out: &mut Vec<u8>,
    raw: &[u8],
    fields: &[JsonField],
    key: &str,
    len: usize,
) -> Result<(), String> {
    let value = fixed_string_vec(raw, field(fields, key)?, len)?;
    out.extend_from_slice(&value);
    Ok(())
}

fn push_u32(out: &mut Vec<u8>, value: usize, label: &str) -> Result<(), String> {
    let value = u32::try_from(value).map_err(|_| format!("{} does not fit into u32", label))?;
    push_u32_value(out, value);
    Ok(())
}

fn push_u32_value(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

/* ----- JSON byte scanner ----- */

fn parse_object_fields(raw: &[u8], start: usize, end: usize) -> Result<Vec<JsonField>, String> {
    if raw.get(start) != Some(&b'{')
        || raw.get(
            end.checked_sub(1)
                .ok_or_else(|| "empty object range".to_string())?,
        ) != Some(&b'}')
    {
        return Err(format!("expected object at {}..{}", start, end));
    }

    let mut fields = Vec::new();
    let mut p = skip_ws(raw, start + 1);
    while p < end - 1 {
        let key_start = p;
        let (key, key_end) = parse_string_key(raw, p)?;
        p = skip_ws(raw, key_end);
        if raw.get(p) != Some(&b':') {
            return Err(format!("expected ':' after key {}", key));
        }
        p = skip_ws(raw, p + 1);
        let value_start = p;
        let value_end = find_value_end(raw, value_start)?;
        fields.push(JsonField {
            key,
            key_start,
            value_start,
            value_end,
        });
        p = skip_ws(raw, value_end);
        if p < end - 1 {
            if raw[p] != b',' {
                return Err(format!("expected ',' in object at byte {}", p));
            }
            p = skip_ws(raw, p + 1);
        }
    }
    Ok(fields)
}

fn parse_string_key(raw: &[u8], start: usize) -> Result<(String, usize), String> {
    if raw.get(start) != Some(&b'"') {
        return Err(format!("expected string key at byte {}", start));
    }
    let mut p = start + 1;
    while p < raw.len() {
        match raw[p] {
            b'\\' => return Err("escaped object keys are not supported".to_string()),
            b'"' => {
                let key = std::str::from_utf8(&raw[start + 1..p])
                    .map_err(|_| "object key is not utf8".to_string())?
                    .to_string();
                return Ok((key, p + 1));
            }
            _ => p += 1,
        }
    }
    Err("unterminated string key".to_string())
}

fn find_value_end(raw: &[u8], start: usize) -> Result<usize, String> {
    match raw.get(start).copied() {
        Some(b'"') => {
            let mut p = start + 1;
            while p < raw.len() {
                if raw[p] == b'\\' {
                    p += 2;
                    continue;
                }
                if raw[p] == b'"' {
                    return Ok(p + 1);
                }
                p += 1;
            }
            Err("unterminated string value".to_string())
        }
        Some(b'{') => find_matching(raw, start, b'{', b'}').map(|p| p + 1),
        Some(b'[') => find_matching(raw, start, b'[', b']').map(|p| p + 1),
        Some(_) => {
            let mut p = start;
            while p < raw.len() && !matches!(raw[p], b',' | b'}' | b']') {
                p += 1;
            }
            Ok(p)
        }
        None => Err("value starts past end of raw".to_string()),
    }
}

fn find_matching(raw: &[u8], start: usize, open: u8, close: u8) -> Result<usize, String> {
    let mut depth = 0i32;
    let mut p = start;
    while p < raw.len() {
        match raw[p] {
            b'"' => {
                p += 1;
                while p < raw.len() {
                    if raw[p] == b'\\' {
                        p += 2;
                        continue;
                    }
                    if raw[p] == b'"' {
                        break;
                    }
                    p += 1;
                }
            }
            c if c == open => depth += 1,
            c if c == close => {
                depth -= 1;
                if depth == 0 {
                    return Ok(p);
                }
            }
            _ => {}
        }
        p += 1;
    }
    Err(format!("unable to find matching {}", close as char))
}

fn object_items_in_array(
    raw: &[u8],
    start: usize,
    end: usize,
) -> Result<Vec<(usize, usize)>, String> {
    if raw.get(start) != Some(&b'[')
        || raw.get(
            end.checked_sub(1)
                .ok_or_else(|| "empty array range".to_string())?,
        ) != Some(&b']')
    {
        return Err(format!("expected array at {}..{}", start, end));
    }
    let mut items = Vec::new();
    let mut p = skip_ws(raw, start + 1);
    while p < end - 1 {
        if raw.get(p) != Some(&b'{') {
            return Err(format!("expected object array item at byte {}", p));
        }
        let item_end = find_matching(raw, p, b'{', b'}')? + 1;
        items.push((p, item_end));
        p = skip_ws(raw, item_end);
        if p < end - 1 {
            if raw[p] != b',' {
                return Err(format!("expected ',' after array item at byte {}", p));
            }
            p = skip_ws(raw, p + 1);
        }
    }
    Ok(items)
}

fn string_array(raw: &[u8], start: usize, end: usize) -> Result<Vec<Vec<u8>>, String> {
    if raw.get(start) != Some(&b'[')
        || raw.get(
            end.checked_sub(1)
                .ok_or_else(|| "empty string array".to_string())?,
        ) != Some(&b']')
    {
        return Err(format!("expected string array at {}..{}", start, end));
    }
    let mut values = Vec::new();
    let mut p = skip_ws(raw, start + 1);
    while p < end - 1 {
        if raw.get(p) != Some(&b'"') {
            return Err(format!("expected string array item at byte {}", p));
        }
        let value_end = find_value_end(raw, p)?;
        values.push(unquoted_string(raw, p, value_end)?);
        p = skip_ws(raw, value_end);
        if p < end - 1 {
            if raw[p] != b',' {
                return Err(format!("expected ',' in string array at byte {}", p));
            }
            p = skip_ws(raw, p + 1);
        }
    }
    Ok(values)
}

fn skip_ws(raw: &[u8], mut p: usize) -> usize {
    while p < raw.len() && matches!(raw[p], b' ' | b'\n' | b'\r' | b'\t') {
        p += 1;
    }
    p
}

fn order_for_fields<const N: usize>(
    fields: &[JsonField],
    keys: &[&str; N],
) -> Result<[u8; N], String> {
    let mut order = [0u8; N];
    for (pos, field) in fields.iter().enumerate() {
        let idx = keys
            .iter()
            .position(|key| *key == field.key)
            .ok_or_else(|| format!("unsupported JSON field {}", field.key))?;
        order[idx] = u8::try_from(pos + 1).map_err(|_| "field order exceeds u8".to_string())?;
    }
    Ok(order)
}

fn field<'a>(fields: &'a [JsonField], key: &str) -> Result<&'a JsonField, String> {
    maybe_field(fields, key).ok_or_else(|| format!("missing field {}", key))
}

fn maybe_field<'a>(fields: &'a [JsonField], key: &str) -> Option<&'a JsonField> {
    fields.iter().find(|field| field.key == key)
}

fn string_contents(raw: &[u8], field: &JsonField) -> Result<Vec<u8>, String> {
    unquoted_string(raw, field.value_start, field.value_end)
}

fn unquoted_string(raw: &[u8], start: usize, end: usize) -> Result<Vec<u8>, String> {
    if raw.get(start) != Some(&b'"')
        || raw.get(
            end.checked_sub(1)
                .ok_or_else(|| "empty string range".to_string())?,
        ) != Some(&b'"')
    {
        return Err(format!("expected quoted string at {}..{}", start, end));
    }
    let inner = &raw[start + 1..end - 1];
    if inner.iter().any(|b| *b == b'\\') {
        return Err(
            "escaped JSON strings are not supported by async V2 raw reconstruction".to_string(),
        );
    }
    Ok(inner.to_vec())
}

fn fixed_string_vec(raw: &[u8], field: &JsonField, len: usize) -> Result<Vec<u8>, String> {
    let bytes = string_contents(raw, field)?;
    if bytes.len() != len {
        return Err(format!(
            "field {} has len {}, expected {}",
            field.key,
            bytes.len(),
            len
        ));
    }
    Ok(bytes)
}

fn fixed_string_array<const N: usize>(
    raw: &[u8],
    field: &JsonField,
    len: usize,
) -> Result<[u8; N], String> {
    let bytes = fixed_string_vec(raw, field, len)?;
    bytes
        .try_into()
        .map_err(|_| format!("field {} wrong fixed length", field.key))
}

fn u32_field(raw: &[u8], fields: &[JsonField], key: &str) -> Result<u32, String> {
    u32_field_from_fields(raw, fields, key)
}

fn u32_field_from_fields(raw: &[u8], fields: &[JsonField], key: &str) -> Result<u32, String> {
    let f = field(fields, key)?;
    let value: u64 = serde_json::from_slice(&raw[f.value_start..f.value_end])
        .map_err(|err| format!("{} is not an unsigned integer: {:?}", key, err))?;
    u32::try_from(value).map_err(|_| format!("{} does not fit into u32", key))
}

fn u8_field(raw: &[u8], fields: &[JsonField], key: &str) -> Result<u8, String> {
    let value = u32_field_from_fields(raw, fields, key)?;
    u8::try_from(value).map_err(|_| format!("{} does not fit into u8", key))
}

/* ----- status enum, ISO date, pack helpers ----- */

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
            log::info!(
                "{} txn[{}] hash: {:?}",
                log_prefix,
                label,
                pending_tx.tx_hash()
            );
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
                    log::error!(
                        "{} txn[{}] receipt meet error: {:?}",
                        log_prefix,
                        label,
                        err
                    );
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
    let trimmed = raw_json.trim();
    let raw = trimmed.as_bytes();
    let fields = parse_object_fields(raw, 0, raw.len())?;
    let tcb_info = field(&fields, "tcbInfo")?;
    std::str::from_utf8(&raw[tcb_info.value_start..tcb_info.value_end])
        .map(|s| s.to_string())
        .map_err(|_| "inner tcbInfo is not utf8".to_string())
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

    const COMPONENTS_A: &str = r#"[{"svn":1,"category":"BIOS","type":"Early Microcode Update"},{"svn":1,"category":"OS/VMM","type":"SGX Late Microcode Update"},{"svn":0,"category":"OS/VMM","type":"TXT SINIT"},{"svn":0,"category":"BIOS"},{"svn":1,"category":"BIOS"},{"svn":255,"category":"BIOS"},{"svn":0},{"svn":1,"category":"OS/VMM","type":"SEAMLDR ACM"},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}]"#;
    const COMPONENTS_B: &str = r#"[{"category":"BIOS","svn":1,"type":"Early Microcode Update"},{"svn":1,"category":"OS/VMM","type":"SGX Late Microcode Update"},{"svn":0,"category":"OS/VMM","type":"TXT SINIT"},{"svn":0,"category":"BIOS"},{"svn":1,"category":"BIOS"},{"svn":255,"category":"BIOS"},{"svn":0},{"svn":1,"category":"OS/VMM","type":"SEAMLDR ACM"},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}]"#;

    fn sgx_inner_with_two_levels() -> String {
        format!(
            r#"{{"id":"SGX","version":3,"issueDate":"2024-07-03T13:09:33Z","nextUpdate":"2024-08-02T13:09:33Z","fmspc":"10A06D070000","pceId":"0000","tcbType":0,"tcbEvaluationDataNumber":16,"tcbLevels":[{{"tcb":{{"sgxtcbcomponents":{},"pcesvn":13}},"tcbDate":"2023-08-09T00:00:00Z","tcbStatus":"UpToDate"}},{{"tcb":{{"sgxtcbcomponents":{},"pcesvn":12}},"tcbDate":"2023-08-10T00:00:00Z","tcbStatus":"ConfigurationNeeded","advisoryIDs":["INTEL-SA-00001"]}}]}}"#,
            COMPONENTS_A, COMPONENTS_B
        )
    }

    const TDX_INNER: &str = r#"{"id":"TDX","version":3,"issueDate":"2024-07-03T13:09:33Z","nextUpdate":"2024-08-02T13:09:33Z","fmspc":"10A06D070000","pceId":"0000","tcbType":0,"tcbEvaluationDataNumber":16,"tdxModule":{"mrsigner":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","attributes":"0000000000000000","attributesMask":"ffffffffffffffff"},"tdxModuleIdentities":[{"id":"TDX_01","mrsigner":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef","attributes":"0000000000000000","attributesMask":"ffffffffffffffff","tcbLevels":[{"tcb":{"isvsvn":1},"tcbDate":"2023-08-09T00:00:00Z","tcbStatus":"UpToDate","advisoryIDs":[]}]}],"tcbLevels":[{"tcb":{"sgxtcbcomponents":[{"svn":1},{"svn":1},{"svn":0},{"svn":0},{"svn":1},{"svn":255},{"svn":0},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}],"pcesvn":13,"tdxtcbcomponents":[{"svn":1},{"svn":1},{"svn":0},{"svn":0},{"svn":1},{"svn":255},{"svn":0},{"svn":1},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0},{"svn":0}]},"tcbDate":"2023-08-09T00:00:00Z","tcbStatus":"UpToDate"}]}"#;

    #[test]
    fn extracts_inner_tcb_info_from_wrapper() {
        let wrapped = r#"{"tcbInfo":{"id":"SGX","version":3,"issueDate":"2024-01-01T00:00:00Z","nextUpdate":"2024-02-01T00:00:00Z","fmspc":"10A06D070000","pceId":"0000","tcbType":0,"tcbEvaluationDataNumber":1,"tcbLevels":[]},"signature":"aa"}"#;
        let inner = extract_tcb_info_str(wrapped).unwrap();
        assert!(inner.starts_with(r#"{"id":"SGX""#));
        assert!(inner.ends_with(r#""tcbLevels":[]}"#));
    }

    #[test]
    fn builds_sgx_plan_and_component_override_payload() {
        let raw = sgx_inner_with_two_levels();
        let plan = build_async_plan(
            raw.as_bytes(),
            TcbLocator {
                tcb_type: 0,
                fmspc: hex::decode("10A06D070000").unwrap().try_into().unwrap(),
                version: 3,
            },
        )
        .unwrap();
        assert_eq!(plan.levels.len(), 2);
        assert_ne!(plan.levels[0].sgx_layout, plan.levels[1].sgx_layout);

        let payload = build_level_batch_payload(&plan.levels, false, 3);
        let header_len = 1 + component_layout_len(&plan.levels[0].sgx_layout);
        let first_len = 4 + 4 + 4 + 3 + 1 + 16 + 4 + 20 + 1 + 4;
        let second_flags_offset = header_len + first_len + 4 + 4 + 4 + 3;
        assert_eq!(
            payload[second_flags_offset] & LEVEL_FLAG_SGX_LAYOUT_OVERRIDE,
            LEVEL_FLAG_SGX_LAYOUT_OVERRIDE
        );
    }

    #[test]
    fn builds_tdx_plan_with_nested_advisory_presence() {
        let plan = build_async_plan(
            TDX_INNER.as_bytes(),
            TcbLocator {
                tcb_type: 1,
                fmspc: hex::decode("10A06D070000").unwrap().try_into().unwrap(),
                version: 3,
            },
        )
        .unwrap();
        assert!(plan.has_tdx_components);
        assert_eq!(plan.identities.len(), 1);
        assert_eq!(
            plan.identities[0].nested_levels[0].flags & LEVEL_FLAG_HAS_ADVISORY_FIELD,
            1
        );
        let payload = build_identity_batch_payload(&plan.identities);
        assert!(!payload.is_empty());
    }

    fn component_layout_len(layout: &[ComponentDescriptor]) -> usize {
        layout
            .iter()
            .map(|descriptor| {
                3 + 2 + descriptor.category.len() + 2 + descriptor.component_type.len()
            })
            .sum()
    }
}
