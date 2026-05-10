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
use std::sync::Arc;
use std::{fs, path::Path};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::timeout;

const TX_CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(120);
const ASYNC_UPLOAD_CHUNK_SIZE: usize = 8 * 1024;
const DEFAULT_ASYNC_PARSE_BATCH_SIZE: u64 = 1;
const FALLBACK_GAS_LIMIT_ENV: &str = "QPL_FALLBACK_GAS_LIMIT";
const ASYNC_PARSE_BATCH_SIZE_ENV: &str = "QPL_ASYNC_PARSE_BATCH_SIZE";

abigen!(
    FmspcTcbDaoV2,
    r#"[
        function FMSPC_TCB_KEY(uint8 tcbType, bytes6 fmspc, uint32 version) view returns (bytes32 key)
        function resolver() view returns (address)
        function startAsyncUpsert(bytes32 refId, bytes signature)
        function uploadChunkData(bytes32 refId, bytes chunkData)
        function parseTCBInfo(bytes32 refId, uint256 start, uint256 offset) returns (uint256 parsed, uint256 total, bool complete)
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
struct TcbLocator {
    tcb_type: u8,
    fmspc: [u8; 6],
    version: u32,
}

#[derive(Clone, Copy)]
struct AsyncParsePlan {
    total_levels: u64,
    total_module_identities: u64,
}

pub async fn upsert_tcb_fmspc_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    fmspc: &str,
    platform: &str, // "tdx" or "sgx"
    version: &str, // "v3" or "v4" or "v5"
    collateral_update_type: Option<&str>, // "standard" or "early"
    tcb_evaluation_data_number: Option<u32>,
    fmspc_tcb_dao_contract_addr: &str,
    tcb_info_json_file: Option<&str>,
    tcb_info_signature_hex: Option<&str>,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, fmspc_tcb_dao_contract_addr);
    let (tcb_info_str, tcb_info_obj, locator, parse_plan) = if let Some(path) = tcb_info_json_file {
        match load_tcb_info_payload_from_file(&log_prefix, path, platform, version, tcb_info_signature_hex) {
            Ok(v) => v,
            Err(err) => {
                log::error!("{} failed to load local TCB payload: {}", log_prefix, err);
                return false;
            }
        }
    } else {
        match fetch_tcb_info_payload(
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
    log::debug!("fetched TCB payload size: {}", tcb_info_str.len());

    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let fmspc_tcb_dao = FmspcTcbDaoV2::new(
        parse_address_from_str(fmspc_tcb_dao_contract_addr),
        signer.clone(),
    );

    upsert_tcb_fmspc_async(&fmspc_tcb_dao, signer.as_ref(), &log_prefix, gas_price, locator, parse_plan, tcb_info_obj)
    .await
}

fn load_tcb_info_payload_from_file(
    log_prefix: &str,
    path: &str,
    platform: &str,
    collateral_version: &str,
    signature_hex: Option<&str>,
) -> Result<(String, TcbInfoJsonObj, TcbLocator, AsyncParsePlan), String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("unable to read {}: {:?}", path, err))?;
    let trimmed = raw.trim();
    let payload = if trimmed.starts_with(r#"{"tcbInfo":{"#) && trimmed.contains(r#","signature":"#) {
        trimmed.to_string()
    } else {
        let sig = resolve_signature_hex(path, signature_hex)?;
        format!(r#"{{"tcbInfo":{},"signature":"{}"}}"#, trimmed, sig)
    };

    log::info!("{} using local TCB payload file: {}", log_prefix, path);
    let (tcb_info_obj, locator, parse_plan) = parse_tcb_info_payload(&payload, platform, collateral_version)?;
    Ok((payload, tcb_info_obj, locator, parse_plan))
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

async fn fetch_tcb_info_payload(
    log_prefix: &str,
    fmspc: &str,
    platform: &str,
    version: &str,
    collateral_update_type: Option<&str>,
    tcb_evaluation_data_number: Option<u32>,
) -> Result<(String, TcbInfoJsonObj, TcbLocator, AsyncParsePlan), String> {
    let mut req_url = format!(
        "https://api.trustedservices.intel.com/{}/certification/{}/tcb?fmspc={}",
        platform, version, fmspc
    );
    if let Some(data_number) = tcb_evaluation_data_number.filter(|value| *value > 0) {
        req_url.push_str(&format!("&tcbEvaluationDataNumber={}", data_number));
    } else if let Some(update_type) = collateral_update_type {
        req_url.push_str(&format!("&update={}", update_type));
    }

    log::debug!("{} req_url: {:?}", log_prefix, req_url);
    let response = reqwest::get(req_url.clone())
        .await
        .map_err(|err| format!("unable to get {}: {:?}", req_url, err))?;

    let tcb_info_str = if response.status().is_success() {
        let headers = response.headers();
        if let Some(cert) = headers.get("SGX-TCB-Info-Issuer-Chain") {
            log::info!("SGX-TCB-Info-Issuer-Chain: {:?}", cert);
        }
        if let Some(cert) = headers.get("TCB-Info-Issuer-Chain") {
            log::info!("TCB-Info-Issuer-Chain: {:?}", cert);
        }
        response
            .text()
            .await
            .map_err(|err| format!("unable to get the content of {}: {:?}", req_url, err))?
    } else {
        return Err(format!("{} returned {}", req_url, response.status()));
    };

    log::info!("TCB-Info: {}", tcb_info_str);
    let (tcb_info_obj, locator, parse_plan) = parse_tcb_info_payload(&tcb_info_str, platform, version)?;
    Ok((tcb_info_str, tcb_info_obj, locator, parse_plan))
}

async fn upsert_tcb_fmspc_async<M: Middleware + 'static>(
    dao: &FmspcTcbDaoV2<M>,
    signer: &M,
    log_prefix: &str,
    gas_price: U256,
    locator: TcbLocator,
    parse_plan: AsyncParsePlan,
    tcb_info_obj: TcbInfoJsonObj,
) -> bool
where
    M::Error: 'static,
{
    let ref_id = generate_ref_id(locator, &tcb_info_obj);

    if !send_transaction(
        signer,
        dao.start_async_upsert(ref_id, tcb_info_obj.signature.clone())
            .gas_price(gas_price),
        log_prefix,
        "start_async_upsert",
    )
    .await
    {
        return false;
    }

    let raw = tcb_info_obj.tcb_info_str.clone().into_bytes();
    for (index, chunk) in raw.chunks(ASYNC_UPLOAD_CHUNK_SIZE).enumerate() {
        let label = format!("upload_chunk_data_{}", index);
        if !send_transaction(
            signer,
            dao.upload_chunk_data(ref_id, chunk.to_vec().into())
                .gas_price(gas_price),
            log_prefix,
            &label,
        )
        .await
        {
            return false;
        }
    }

    let batch_size = async_parse_batch_size();
    for start in (0..parse_plan.total_levels).step_by(batch_size as usize) {
        let label = format!("parse_tcb_info_{}", start);
        if !send_transaction(
            signer,
            dao.parse_tcb_info(ref_id, U256::from(start), U256::from(batch_size))
                .gas_price(gas_price),
            log_prefix,
            &label,
        )
        .await
        {
            return false;
        }
    }

    for start in (0..parse_plan.total_module_identities).step_by(batch_size as usize) {
        let label = format!("parse_tcb_tdx_module_identities_{}", start);
        if !send_transaction(
            signer,
            dao.parse_tcb_info(ref_id, U256::from(start), U256::from(batch_size))
                .gas_price(gas_price),
            log_prefix,
            &label,
        )
        .await
        {
            return false;
        }
    }

    let tcb_key = match dao
        .fmspc_tcb_key(locator.tcb_type, locator.fmspc, locator.version)
        .call()
        .await
    {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} fmspc_tcb_key failed: {:?}", log_prefix, err);
            return false;
        }
    };

    let resolver_addr = match dao.resolver().call().await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} resolver lookup failed: {:?}", log_prefix, err);
            return false;
        }
    };
    let resolver = CollateralResolver::new(resolver_addr, signer.into());
    let attestation_id = match resolver.collateral_pointer(tcb_key).call().await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} collateralPointer failed: {:?}", log_prefix, err);
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

fn async_parse_batch_size() -> u64 {
    std::env::var(ASYNC_PARSE_BATCH_SIZE_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_ASYNC_PARSE_BATCH_SIZE)
}

async fn send_transaction<M, D>(
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

async fn send_transaction_with_response<M, D>(
    signer: &M,
    call: ContractCall<M, D>,
    log_prefix: &str,
    label: &str,
) -> Option<D>
where
    M: Middleware,
    M::Error: 'static,
    D: Detokenize + Clone,
{
    let preview_call = if let Some(sender) = signer.default_sender() {
        call.clone().from(sender)
    } else {
        call.clone()
    };

    let response = match preview_call.call().await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{} txn[{}] call failed: {:?}", log_prefix, label, err);
            return None;
        }
    };

    let gas_with_buf = match gas_limit_with_fallback(signer, &call.tx, log_prefix, label).await {
        Some(g) => g,
        None => return None,
    };

    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!("{} txn[{}] hash: {:?}", log_prefix, label, pending_tx.tx_hash());
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!("{} txn[{}] receipt: {:?}", log_prefix, label, receipt);
                    Some(response)
                }
                Ok(Err(err)) => {
                    log::error!("{} txn[{}] receipt meet error: {:?}", log_prefix, label, err);
                    None
                }
                Err(_) => {
                    log::error!(
                        "{} txn[{}] timeout waiting for confirmation after {:?}",
                        log_prefix,
                        label,
                        TX_CONFIRMATION_TIMEOUT
                    );
                    None
                }
            }
        }
        Err(err) => {
            log::error!("{} txn[{}] meet error: {:?}", log_prefix, label, err);
            None
        }
    }
}

async fn gas_limit_with_fallback<M: Middleware>(
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

fn parse_tcb_info_payload(
    raw_json: &str,
    platform: &str,
    collateral_version: &str,
) -> Result<(TcbInfoJsonObj, TcbLocator, AsyncParsePlan), String> {
    let tcb_info: TcbInfo =
        serde_json::from_str(raw_json).map_err(|err| format!("invalid tcb payload: {:?}", err))?;

    let tcb_info_str = extract_tcb_info_str(raw_json)?;
    let signature =
        Bytes::from_hex(tcb_info.signature).map_err(|err| format!("invalid signature: {:?}", err))?;

    let fmspc_hex = tcb_info
        .tcb_info
        .get("fmspc")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing fmspc".to_string())?;
    let fmspc_vec =
        hex::decode(fmspc_hex).map_err(|err| format!("invalid fmspc hex {}: {:?}", fmspc_hex, err))?;
    let fmspc_len = fmspc_vec.len();
    let fmspc: [u8; 6] = fmspc_vec
        .try_into()
        .map_err(|_| format!("unexpected fmspc length: {}", fmspc_len))?;

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
        .map(|value| value as u32)
        .or_else(|| fallback_tcb_version(collateral_version))
        .ok_or_else(|| "missing version".to_string())?;
    let total_levels = tcb_info
        .tcb_info
        .get("tcbLevels")
        .and_then(Value::as_array)
        .map(|value| value.len() as u64)
        .ok_or_else(|| "missing tcbLevels".to_string())?;
    let total_module_identities = tcb_info
        .tcb_info
        .get("tdxModuleIdentities")
        .and_then(Value::as_array)
        .map(|value| value.len() as u64)
        .unwrap_or(0);

    Ok((
        TcbInfoJsonObj {
            tcb_info_str,
            signature,
        },
        TcbLocator {
            tcb_type,
            fmspc,
            version,
        },
        AsyncParsePlan {
            total_levels,
            total_module_identities,
        },
    ))
}

fn extract_tcb_info_str(raw_json: &str) -> Result<String, String> {
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

fn parse_tcb_type(value: &str) -> u8 {
    if value.eq_ignore_ascii_case("tdx") {
        1
    } else {
        0
    }
}

fn fallback_tcb_version(value: &str) -> Option<u32> {
    match value {
        "v3" => Some(2),
        "v4" | "v5" => Some(3),
        _ => None,
    }
}

fn generate_ref_id(locator: TcbLocator, tcb_info_obj: &TcbInfoJsonObj) -> [u8; 32] {
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
    use automata_dcap_qpl_contracts::fmspc_tcb_dao::FmspcTcbDao;
    use openssl::sha::sha256;
    use std::env;

    const ENV_PRIVATE_KEY: &str = "LIVE_TCB_TEST_PRIVATE_KEY";
    const ENV_RPC_URL: &str = "LIVE_TCB_TEST_RPC_URL";
    const ENV_CHAIN_ID: &str = "LIVE_TCB_TEST_CHAIN_ID";
    const ENV_GAS_PRICE: &str = "LIVE_TCB_TEST_GAS_PRICE";
    const ENV_DAO_ADDR: &str = "LIVE_TCB_TEST_FMSPC_TCB_DAO";
    const ENV_FMSPC: &str = "LIVE_TCB_TEST_FMSPC";
    const ENV_PLATFORM: &str = "LIVE_TCB_TEST_PLATFORM";
    const ENV_VERSION: &str = "LIVE_TCB_TEST_VERSION";
    const ENV_UPDATE_TYPE: &str = "LIVE_TCB_TEST_UPDATE_TYPE";
    const ENV_TCB_EVAL_NUMBER: &str = "LIVE_TCB_TEST_TCB_EVALUATION_DATA_NUMBER";

    #[tokio::test]
    #[ignore = "requires live RPC, funded key, and deployed FMSPC_TCB_DAO"]
    async fn live_async_upsert_tcb_fmspc_and_verify_onchain() {
        let _ = env_logger::builder().is_test(true).try_init();

        let private_key = required_env(ENV_PRIVATE_KEY);
        let rpc_url = required_env(ENV_RPC_URL);
        let chain_id = parse_u64_env(ENV_CHAIN_ID);
        let gas_price = U256::from_dec_str(&required_env(ENV_GAS_PRICE)).unwrap();
        let dao_addr = required_env(ENV_DAO_ADDR);
        let fmspc = env::var(ENV_FMSPC).unwrap_or_else(|_| "00606a000000".to_string());
        let platform = env::var(ENV_PLATFORM).unwrap_or_else(|_| "sgx".to_string());
        let version = env::var(ENV_VERSION).unwrap_or_else(|_| "v4".to_string());
        let update_type = env::var(ENV_UPDATE_TYPE).ok();
        let tcb_eval_number = env::var(ENV_TCB_EVAL_NUMBER)
            .ok()
            .map(|value| value.parse::<u32>().unwrap());
        let log_prefix = format!("[{}][{}][live-test]", chain_id, dao_addr);

        let (_, expected_obj, locator, parse_plan) = fetch_tcb_info_payload(
            &log_prefix,
            &fmspc,
            &platform,
            &version,
            update_type.as_deref(),
            tcb_eval_number,
        )
        .await
        .expect("failed to fetch latest TCB payload");

        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let wallet = private_key.parse::<LocalWallet>().unwrap();
        let signer = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(chain_id),
        ));

        let live_dao = FmspcTcbDaoV2::new(parse_address_from_str(&dao_addr), signer.clone());
        let readonly_dao = FmspcTcbDao::new(parse_address_from_str(&dao_addr), signer.clone());

        if !matches_onchain_tcb(&readonly_dao, locator, &expected_obj).await {
            let upsert_ok = upsert_tcb_fmspc_async(
                &live_dao,
                signer.as_ref(),
                &log_prefix,
                gas_price,
                locator,
                parse_plan,
                expected_obj.clone(),
            )
            .await;
            assert!(
                upsert_ok || matches_onchain_tcb(&readonly_dao, locator, &expected_obj).await,
                "async upsert failed and on-chain content still does not match expected payload"
            );
        }

        let onchain = readonly_dao
            .get_tcb_info(
                U256::from(locator.tcb_type),
                hex::encode(locator.fmspc),
                U256::from(locator.version),
            )
            .call()
            .await
            .expect("failed to read TCB info from contract");
        assert_eq!(onchain.tcb_info_str, expected_obj.tcb_info_str);
        assert_eq!(onchain.signature, expected_obj.signature);

        let tcb_key = readonly_dao
            .fmspc_tcb_key(locator.tcb_type, locator.fmspc, locator.version)
            .call()
            .await
            .expect("failed to derive TCB key");
        let onchain_hash = readonly_dao
            .get_collateral_hash(tcb_key)
            .call()
            .await
            .expect("failed to read collateral hash");
        assert_eq!(onchain_hash, sha256(expected_obj.tcb_info_str.as_bytes()));
    }

    async fn matches_onchain_tcb<M: Middleware>(
        dao: &FmspcTcbDao<M>,
        locator: TcbLocator,
        expected: &TcbInfoJsonObj,
    ) -> bool {
        let fetched = dao
            .get_tcb_info(
                U256::from(locator.tcb_type),
                hex::encode(locator.fmspc),
                U256::from(locator.version),
            )
            .call()
            .await;
        match fetched {
            Ok(value) => value.tcb_info_str == expected.tcb_info_str && value.signature == expected.signature,
            Err(_) => false,
        }
    }

    fn required_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("missing required env var {}", key))
    }

    fn parse_u64_env(key: &str) -> u64 {
        required_env(key)
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid u64 env var {}", key))
    }
}
