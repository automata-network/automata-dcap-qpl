#[path = "helper/tcb_fmspc_async.rs"]
pub(crate) mod tcb_fmspc_async;

use crate::cloud_providers::*;
use crate::contracts::*;
use crate::pccs_types::*;
use automata_dcap_qpl_common::*;
use automata_dcap_qpl_contracts::parse_address_from_env_var::parse_address_from_str;
use automata_dcap_qpl_contracts::{
    enclave_identity_dao::{EnclaveIdentityDao, EnclaveIdentityJsonObj},
    fmspc_tcb_dao::{FmspcTcbDao, TcbInfoJsonObj},
    parse_address_from_env_var::parse_address_from_env_var,
    pcs_dao::PcsDao,
    pcs_dao_v2::X509CrlHelperV2,
};
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use hex::FromHex;
use pccs_reader_rs::*;

use openssl::asn1::Asn1Time;
use openssl::x509::{X509Crl, X509};
use reqwest;
use std::cmp::Ordering;
use std::ffi::{c_char, CStr, CString};
use std::{str::FromStr, sync::Arc};
use tokio::time::{timeout, Duration};

#[allow(unused_imports)]
pub use tcb_fmspc_async::upsert_tcb_fmspc_func;

/// Timeout for waiting for transaction confirmation (2 minutes)
const TX_CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(120);

fn append_query_parameter(url: &str, key: &str, value: &str) -> Result<String, String> {
    let mut url = reqwest::Url::parse(url)
        .map_err(|err| format!("invalid Intel PCS URL {}: {:?}", url, err))?;
    url.query_pairs_mut().append_pair(key, value);
    Ok(url.into())
}

async fn fetch_intel_pcs_body(url: &str) -> Result<String, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|err| format!("unable to get {}: {:?}", url, err))?;
    if !response.status().is_success() {
        return Err(format!("{} returned {}", url, response.status()));
    }
    response
        .text()
        .await
        .map_err(|err| format!("unable to read body of {}: {:?}", url, err))
}

fn validate_collateral_tcb_eval_number(
    body: &str,
    collateral_field: &str,
    expected: u32,
) -> Result<(), String> {
    let payload: serde_json::Value = serde_json::from_str(body)
        .map_err(|err| format!("invalid Intel PCS collateral payload: {:?}", err))?;
    let actual = payload
        .get(collateral_field)
        .and_then(|collateral| collateral.get("tcbEvaluationDataNumber"))
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| {
            format!(
                "{}.tcbEvaluationDataNumber is missing or invalid",
                collateral_field
            )
        })?;
    if actual != expected {
        return Err(format!(
            "{}.tcbEvaluationDataNumber mismatch: expected {}, got {}",
            collateral_field, expected, actual
        ));
    }
    Ok(())
}

/// Fetches Intel PCS collateral for an exact TCB evaluation data number.
///
/// Intel PCS has known FMSPCs for which the exact-version URL returns 404 while
/// the unqualified standard URL returns valid collateral. In that case the
/// standard response is accepted only when its embedded evaluation number
/// exactly matches the requested value.
pub(crate) async fn fetch_intel_collateral_with_eval_fallback(
    base_url: &str,
    collateral_update_type: Option<&str>,
    tcb_evaluation_data_number: Option<u32>,
    collateral_field: &str,
    log_prefix: &str,
) -> Result<String, String> {
    if let Some(expected) = tcb_evaluation_data_number.filter(|value| *value > 0) {
        let exact_url =
            append_query_parameter(base_url, "tcbEvaluationDataNumber", &expected.to_string())?;
        match fetch_intel_pcs_body(&exact_url).await.and_then(|body| {
            validate_collateral_tcb_eval_number(&body, collateral_field, expected)?;
            Ok(body)
        }) {
            Ok(body) => return Ok(body),
            Err(exact_error) => {
                log::warn!(
                    "{} exact Intel PCS request failed; trying unqualified standard collateral: {}",
                    log_prefix,
                    exact_error
                );
            }
        }

        let standard_body = fetch_intel_pcs_body(base_url)
            .await
            .map_err(|standard_error| {
                format!(
                    "exact-version request failed and standard fallback failed: {}",
                    standard_error
                )
            })?;
        validate_collateral_tcb_eval_number(&standard_body, collateral_field, expected).map_err(
            |mismatch| {
                format!(
                    "standard fallback cannot satisfy requested TCB evaluation data number {}: {}",
                    expected, mismatch
                )
            },
        )?;
        log::warn!(
            "{} using verified Intel PCS standard fallback for TCB evaluation data number {}",
            log_prefix,
            expected
        );
        return Ok(standard_body);
    }

    let request_url = match collateral_update_type {
        Some(update) => append_query_parameter(base_url, "update", update)?,
        None => base_url.to_string(),
    };
    fetch_intel_pcs_body(&request_url).await
}

fn crl_is_current(crl: &X509Crl) -> bool {
    let now = match Asn1Time::days_from_now(0) {
        Ok(now) => now,
        Err(_) => return false,
    };
    let next_update = match crl.next_update() {
        Some(next_update) => next_update,
        None => return false,
    };

    matches!(crl.last_update().compare(&now), Ok(Ordering::Less))
        && matches!(next_update.compare(&now), Ok(Ordering::Greater))
}

/// Detect whether the configured PCS DAO uses exact-index V2 before any
/// state-changing transaction is submitted.
///
/// The `crlRevokedSetHashes` getter is the final V2 marker. A revert identifies
/// the deployed V1 helper; transport and ABI errors are never silently
/// downgraded because they could hide a broken deployment or RPC endpoint.
async fn detect_eager_crl_v2<M: Middleware + 'static>(
    pcs_dao: &PcsDao<M>,
    log_prefix: &str,
) -> Option<bool> {
    let crl_helper_address = match pcs_dao.crl_lib().call().await {
        Ok(address) => address,
        Err(err) => {
            log::error!("{} unable to resolve CRL helper: {:?}", log_prefix, err);
            return None;
        }
    };
    let crl_helper = X509CrlHelperV2::new(crl_helper_address, pcs_dao.client());

    match crl_helper.crl_revoked_set_hashes([0u8; 32]).call().await {
        Ok(_) => {
            log::info!(
                "{} detected exact-index PCS DAO V2 at {:?}",
                log_prefix,
                crl_helper_address
            );
            Some(true)
        }
        Err(err) if err.is_revert() => {
            log::info!(
                "{} detected PCS DAO V1 helper at {:?}; using the V1 upsert path",
                log_prefix,
                crl_helper_address
            );
            Some(false)
        }
        Err(err) => {
            log::error!(
                "{} unable to probe PCS DAO V2 CRL indexing support at {:?}: {:?}",
                log_prefix,
                crl_helper_address,
                err
            );
            None
        }
    }
}

/// V2 makes the exact index available in the same transaction as the upsert.
/// A successful receipt is therefore not sufficient: read the helper state and
/// fail closed if the DER written by that transaction is not immediately ready.
async fn eager_crl_index_is_ready<M: Middleware + 'static>(
    pcs_dao: &PcsDao<M>,
    crl: &Bytes,
    log_prefix: &str,
) -> bool {
    let crl_helper_address = match pcs_dao.crl_lib().call().await {
        Ok(address) => address,
        Err(err) => {
            log::error!("{} unable to resolve V2 CRL helper: {:?}", log_prefix, err);
            return false;
        }
    };
    let crl_helper = X509CrlHelperV2::new(crl_helper_address, pcs_dao.client());
    let der_hash = ethers::utils::keccak256(crl.as_ref());
    match crl_helper.indexed_crls(der_hash).call().await {
        Ok(true) => {
            log::info!(
                "{} exact CRL index is ready in the upsert transaction: der_hash=0x{}",
                log_prefix,
                hex::encode(der_hash)
            );
            true
        }
        Ok(false) => {
            log::error!(
                "{} V2 upsert completed without a ready exact CRL index: der_hash=0x{}",
                log_prefix,
                hex::encode(der_hash)
            );
            false
        }
        Err(err) => {
            log::error!(
                "{} unable to verify exact CRL index state: {:?}",
                log_prefix,
                err
            );
            false
        }
    }
}

/// Estimate gas against the `latest` block and return the value with a 30%
/// buffer. The default ethers-rs `fill_transaction` flow estimates against
/// the `pending` block; on Cosmos-EVM chains (e.g. Story 1315 / 1514) a
/// previously stuck pending tx pollutes that state and idempotency-checked
/// upserts revert with empty bytes. Estimating against `latest` avoids that.
/// Returning `None` means the call would also revert against `latest`, so
/// callers should skip the actual `send` to avoid wasted attempts.
pub async fn estimate_gas_at_latest<M: Middleware>(
    signer: &M,
    tx: &TypedTransaction,
    log_prefix: &str,
    label: &str,
) -> Option<U256>
where
    M::Error: 'static,
{
    let block = Some(BlockId::Number(BlockNumber::Latest));
    match signer.estimate_gas(tx, block).await {
        Ok(v) => Some(v.saturating_mul(U256::from(130)) / U256::from(100)),
        Err(err) => {
            log::error!(
                "{} txn[{}] estimate_gas at latest failed: {:?}",
                log_prefix,
                label,
                err
            );
            None
        }
    }
}

/// Returns the stored DER represented by the fetched CRL when no upsert is needed.
/// represented by the stored collateral. Exact DER equality covers normal
/// retries. TBS equality additionally covers a signature-only reissue, which
/// the DAO correctly treats as a duplicate because the signed content did not
/// change; in that case any V2 readiness check must use the DER actually stored on-chain.
async fn reusable_stored_crl<M>(
    pcs_dao: &PcsDao<M>,
    ca: u8,
    fetched_crl: &Bytes,
    log_prefix: &str,
) -> Option<Bytes>
where
    M: Middleware + 'static,
{
    let stored_crl = match pcs_dao
        .get_certificate_by_id(ca)
        .from(Address::zero())
        .call()
        .await
    {
        Ok((_, stored_crl)) if !stored_crl.is_empty() => stored_crl,
        Ok(_) => return None,
        Err(err) => {
            log::debug!(
                "{} unable to compare stored CRL before upsert: {:?}",
                log_prefix,
                err
            );
            return None;
        }
    };
    if stored_crl == *fetched_crl {
        return Some(stored_crl);
    }

    let crl_helper_address = pcs_dao.crl_lib().call().await.ok()?;
    let crl_helper = X509CrlHelperV2::new(crl_helper_address, pcs_dao.client());
    let (fetched_tbs, _) = crl_helper
        .get_tbs_and_sig(fetched_crl.clone())
        .call()
        .await
        .ok()?;
    let key = pcs_dao.pcs_key(ca, true).call().await.ok()?;
    let stored_tbs_hash = pcs_dao
        .get_collateral_hash(key)
        .from(Address::zero())
        .call()
        .await
        .ok()?;
    if ethers::utils::keccak256(fetched_tbs.as_ref()) == stored_tbs_hash {
        log::info!(
            "{} fetched CRL only changes the outer signature; retaining stored DER",
            log_prefix
        );
        return Some(stored_crl);
    }

    None
}

const INTEL_PCS_SUBSCRIPTION_KEY_ENV: &str = "INTEL_PCS_SUBSCRIPTION_KEY";

fn get_intel_pcs_subscription_key() -> String {
    match std::env::var(INTEL_PCS_SUBSCRIPTION_KEY_ENV) {
        Ok(v) => v,
        Err(_) => {
            println!("[ERROR] pleause configure the intel pcs subscription key");
            format!("")
        }
    }
}

pub fn check_missing_collateral(
    quote: &[u8],
    prv_key: &str,
    pccs_url: String,
    rpc_url: String,
    chain_id: u64,
) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    loop {
        let missing_collateral: MissingCollateral =
            rt.block_on(pccs_reader_rs::find_missing_collaterals_from_quote(quote));
        println!("missing_collateral: {:?}", missing_collateral);
        match missing_collateral {
            MissingCollateral::None => break,
            MissingCollateral::QEIdentity(enclave_id_type, quote_version) => {
                let (enclave_id, enclave_id_type) = match enclave_id_type {
                    pccs_reader_rs::pccs::enclave_id::EnclaveIdType::TDQE => {
                        (EnclaveID::TD_QE, "tdx".to_string())
                    }
                    _ => (EnclaveID::QE, "sgx".to_string()),
                };
                let collateral_version = if quote_version == 3 {
                    "v3".to_string()
                } else {
                    "v4".to_string()
                };
                let req_url = format!(
                    "{}/{}/certification/{}/qe/identity",
                    pccs_url, enclave_id_type, collateral_version
                );
                let response = match rt.block_on(reqwest::get(req_url.clone())) {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Unable to get {}", req_url);
                        return;
                    }
                };
                if response.status().is_success() {
                    let headers = response.headers();
                    let enclave_identity_issuer_chains_str =
                        if let Some(cert) = headers.get("SGX-Enclave-Identity-Issuer-Chain") {
                            cert.to_str().unwrap().to_string()
                        } else {
                            println!(
                                "Cannot find SGX-Enclave-Identity-Issuer-Chain in {:?}, exit",
                                req_url
                            );
                            return;
                        };
                    let enclave_identity_issuer_chains_str =
                        urlencoding::decode(&enclave_identity_issuer_chains_str)
                            .expect("Invalid UTF-8");
                    let enclave_identity_issuer_chains_str =
                        enclave_identity_issuer_chains_str.to_string();
                    let qe_identity_str = match rt.block_on(response.text()) {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Unable to get the content of {}", req_url);
                            return;
                        }
                    };
                    println!(
                        "Enclave-Identity-Issuer-Chain: {}",
                        enclave_identity_issuer_chains_str
                    );
                    println!("QE-Identity: {}", qe_identity_str);

                    upsert_enclave_identity(
                        prv_key,
                        rpc_url.clone(),
                        chain_id,
                        enclave_id,
                        collateral_version,
                        qe_identity_str.as_str(),
                        enclave_identity_issuer_chains_str.as_str(),
                    )
                }
            }
            MissingCollateral::FMSPCTCB(tcb_type, fmspc, tcb_version) => {
                let tcb_type = if tcb_type == 1 {
                    "tdx".to_string()
                } else {
                    "sgx".to_string()
                };
                let collateral_version = if tcb_version == 2 {
                    "v3".to_string()
                } else {
                    "v4".to_string()
                };
                let req_url = format!(
                    "{}/{}/certification/{}/tcb?fmspc={}",
                    pccs_url, tcb_type, collateral_version, fmspc
                );
                println!("req_url: {:?}", req_url);
                let response = match rt.block_on(reqwest::get(req_url.clone())) {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Unable to get {}", req_url);
                        return;
                    }
                };
                let tcb_info_str = if response.status().is_success() {
                    let headers = response.headers();
                    // v3
                    if let Some(cert) = headers.get("SGX-TCB-Info-Issuer-Chain") {
                        println!("SGX-TCB-Info-Issuer-Chain: {:?}", cert);
                    }
                    // v4
                    if let Some(cert) = headers.get("TCB-Info-Issuer-Chain") {
                        println!("TCB-Info-Issuer-Chain: {:?}", cert);
                    }
                    let content = match rt.block_on(response.text()) {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Unable to get the content of {}", req_url);
                            return;
                        }
                    };
                    println!("TCB-Info: {}", content);
                    content
                } else {
                    println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
                    return;
                };
                // TCB Info
                let tcb_info: TcbInfo = serde_json::from_str(&tcb_info_str).unwrap();
                let tcb_info_str = &tcb_info_str[r#""tcbInfo":{"#.len()..];
                let end_idx = tcb_info_str.find(r#","signature""#).unwrap();
                let tcb_info_str = &tcb_info_str[..end_idx];
                let tcb_info_obj = TcbInfoJsonObj {
                    tcb_info_str: tcb_info_str.to_string(),
                    signature: Bytes::from_hex(tcb_info.signature).unwrap(),
                };
                println!("tcb_info_obj.tcb_info_str: {}", tcb_info_obj.tcb_info_str);
                println!("tcb_info_obj.signature: {:?}", tcb_info_obj.signature);
                let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
                let wallet = prv_key.parse::<LocalWallet>().unwrap();
                let signer = Arc::new(SignerMiddleware::new(
                    provider,
                    wallet.with_chain_id(chain_id),
                ));
                let fmspc_tcb_dao =
                    FmspcTcbDao::new(parse_address_from_env_var("FMSPC_TCB_DAO"), signer.clone());
                match rt.block_on(
                    fmspc_tcb_dao
                        .upsert_fmspc_tcb(tcb_info_obj)
                        .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap())
                        .send(),
                ) {
                    Ok(pending_tx) => {
                        println!("txn[upsert_fmspc_tcb] hash: {:?}", pending_tx.tx_hash());
                        match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                            Ok(Ok(receipt)) => {
                                println!("txn[upsert_fmspc_tcb] receipt: {:?}", receipt);
                            }
                            Ok(Err(err)) => {
                                println!("txn[upsert_fmspc_tcb] receipt meet error: {:?}", err);
                            }
                            Err(_) => {
                                println!("txn[upsert_fmspc_tcb] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                            }
                        }
                    }
                    Err(err) => {
                        println!("txn[upsert_fmspc_tcb] meet error: {:?}", err);
                    }
                }
            }
            MissingCollateral::PCS(pck_ca, _, _) => {
                let (pck_ca, pck) = match pck_ca {
                    pccs_reader_rs::pccs::pcs::IPCSDao::CA::PROCESSOR => {
                        ("processor".to_string(), CAID::Processor)
                    }
                    pccs_reader_rs::pccs::pcs::IPCSDao::CA::PLATFORM => {
                        ("platform".to_string(), CAID::Platform)
                    }
                    _ => {
                        // should not happen because it's already included in the tcb/qe cert chain
                        continue;
                    }
                };
                let req_url = format!(
                    "{}/sgx/certification/v3/pckcrl?ca={}",
                    pccs_url,
                    pck_ca.clone()
                );
                let response = match rt.block_on(reqwest::get(req_url.clone())) {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Unable to get {}", req_url);
                        return;
                    }
                };
                let pck_crl = if response.status().is_success() {
                    let headers = response.headers();
                    if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
                        println!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
                    }
                    let content = match rt.block_on(response.text()) {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Unable to get the content of {}", req_url);
                            return;
                        }
                    };
                    println!("SGX-PCK-CRL: {}", content);
                    content
                } else {
                    println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
                    return;
                };
                let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
                let wallet = prv_key.parse::<LocalWallet>().unwrap();
                let signer = Arc::new(SignerMiddleware::new(
                    provider,
                    wallet.with_chain_id(chain_id),
                ));
                let pcs_dao = PcsDao::new(parse_address_from_env_var("PCS_DAO"), signer.clone());
                // PCK CRL
                let pck_crl = match X509Crl::from_pem(pck_crl.as_bytes()) {
                    Ok(c) => hex::encode(c.to_der().unwrap()),
                    Err(err) => {
                        println!("Error parsing certificate: {:?}", err);
                        return;
                    }
                };
                match rt.block_on(
                    pcs_dao
                        .upsert_pck_crl(pck as u8, Bytes::from_str(&pck_crl).unwrap())
                        .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap())
                        .send(),
                ) {
                    Ok(pending_tx) => {
                        println!("txn[upsert_pck_crl] hash: {:?}", pending_tx.tx_hash());
                        match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                            Ok(Ok(receipt)) => {
                                println!("txn[upsert_pck_crl] receipt: {:?}", receipt);
                            }
                            Ok(Err(err)) => {
                                println!("txn[upsert_pck_crl] receipt meet error: {:?}", err);
                            }
                            Err(_) => {
                                println!("txn[upsert_pck_crl] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                            }
                        }
                    }
                    Err(err) => {
                        println!("txn[upsert_pck_crl] meet error: {:?}", err);
                    }
                }
            }
        }
    }
}

pub fn sgx_ql_get_quote_config(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    pck_cert_id: SgxQlPckCertId,
    data_source: DataSource,
    collateral_version: String,
    pccs_url: String,
) {
    println!("pck_cert_id: {:?}", pck_cert_id);

    println!("cpu_svn: {:?}", unsafe { *pck_cert_id.p_platform_cpu_svn });
    println!("pce_svn: {:?}", unsafe {
        *pck_cert_id.p_platform_pce_isv_svn
    });

    let cpu_svn = unsafe { *pck_cert_id.p_platform_cpu_svn };
    let pce_svn = unsafe { *pck_cert_id.p_platform_pce_isv_svn };

    println!("cpu_svn: {:?}", hex::encode(cpu_svn.cpu_svn));
    println!("pce_svn: {:?}", hex::encode(pce_svn.isv_svn.to_le_bytes()));
    println!(
        "pce_id: {:?}",
        hex::encode(pck_cert_id.pce_id.to_le_bytes())
    );
    let qe_id = unsafe {
        std::slice::from_raw_parts(pck_cert_id.p_qe3_id, pck_cert_id.qe3_id_size as usize)
    };
    println!("qe_id: {:?}", hex::encode(qe_id));
    let encrypted_ppid = unsafe {
        std::slice::from_raw_parts(
            pck_cert_id.p_encrypted_ppid,
            pck_cert_id.encrypted_ppid_size as usize,
        )
    };
    println!("encrypted_ppid: {:?}", hex::encode(encrypted_ppid));
    let qe_id_str = hex::encode(qe_id);
    let cpu_svn_str = hex::encode(unsafe { *pck_cert_id.p_platform_cpu_svn }.cpu_svn);
    let pce_svn_str = hex::encode(
        unsafe { *pck_cert_id.p_platform_pce_isv_svn }
            .isv_svn
            .to_le_bytes(),
    );
    let pce_id_str = hex::encode(pck_cert_id.pce_id.to_le_bytes());

    if data_source == DataSource::All || data_source == DataSource::Azure {
        std::env::set_var("AZDCAP_COLLATERAL_VERSION", collateral_version.clone());
        let p_pck_cert_id: *const SgxQlPckCertId = &pck_cert_id as *const SgxQlPckCertId;
        let mut sgx_ql_config: SgxQlConfig =
            SgxQlConfig::new([0_u8; 16], 0, std::ptr::null_mut(), 0);
        let mut p_sgx_ql_config: *mut SgxQlConfig = &mut sgx_ql_config as *mut SgxQlConfig;
        let pp_sgx_ql_config: *mut *mut SgxQlConfig = &mut p_sgx_ql_config as *mut *mut SgxQlConfig;
        let ret = azure::az_dcap_sgx_ql_get_quote_config(p_pck_cert_id, pp_sgx_ql_config);
        println!("azure dcap sgx_ql_get_quote_config func: {:?}", ret);
        let quote_config = unsafe { *pp_sgx_ql_config };
        unsafe { quote_config.as_ref().unwrap().print() };

        let tcbm = unsafe {
            let mut tcbm_vec = Vec::new();
            let mut tcbm_cpu_svn = quote_config
                .as_ref()
                .unwrap()
                .cert_cpu_svn
                .cpu_svn
                .clone()
                .to_vec();
            tcbm_vec.append(&mut tcbm_cpu_svn);
            let tcbm_pce_svn = quote_config.as_ref().unwrap().cert_pce_isv_svn.isv_svn;
            let mut tcbm_pce_svn = tcbm_pce_svn.to_le_bytes().to_vec();
            tcbm_vec.append(&mut tcbm_pce_svn);
            hex::encode(tcbm_vec)
        };
        let cert_data = unsafe {
            std::slice::from_raw_parts(
                quote_config.as_ref().unwrap().cert_data,
                quote_config.as_ref().unwrap().cert_data_size as usize,
            )
        };
        let cert_data = std::str::from_utf8(cert_data).unwrap();
        upsert_pck_cert(
            &private_key,
            rpc_url,
            chain_id,
            CAID::Platform,
            qe_id_str,
            pce_id_str,
            cpu_svn_str,
            pce_svn_str,
            tcbm,
            cert_data,
        );

        let ret = azure::az_dcap_sgx_ql_free_quote_config(quote_config);
        println!("azure dcap sgx_ql_free_quote_config func: {:?}", ret);
    }
    if data_source == DataSource::All || data_source == DataSource::Local {
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.1 Section
        let encrypted_ppid = if encrypted_ppid.len() > 0 {
            hex::encode(encrypted_ppid)
        } else {
            hex::encode([0; 384])
        };
        let req_url = format!(
            "{}/sgx/certification/{}/pckcert",
            pccs_url,
            collateral_version.clone(),
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let client = reqwest::Client::new();
        let query_params = vec![
            ("cpusvn".to_string(), hex::encode(cpu_svn.cpu_svn)),
            (
                "pcesvn".to_string(),
                hex::encode(pce_svn.isv_svn.to_le_bytes()),
            ),
            (
                "pceid".to_string(),
                hex::encode(pck_cert_id.pce_id.to_le_bytes()),
            ),
            ("encrypted_ppid".to_string(), encrypted_ppid),
        ];
        let mut req_builder = client.get(req_url.clone()).query(&query_params);
        if collateral_version == "v3" {
            let intel_pcs_subscription_key = get_intel_pcs_subscription_key();
            if intel_pcs_subscription_key.is_empty() {
                return;
            }
            let intel_pcs_subscription_key_str = intel_pcs_subscription_key.as_str();
            req_builder =
                req_builder.header("Ocp-Apim-Subscription-Key", intel_pcs_subscription_key_str);
        }
        let response = match rt.block_on(req_builder.send()) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        if response.status().is_success() {
            let headers = response.headers();
            if let Some(cert) = headers.get("SGX-PCK-Certificate-Issuer-Chain") {
                println!("cert: {:?}", cert);
            }
            if let Some(tcbm) = headers.get("SGX-TCBm") {
                println!("tcbm: {:?}", tcbm);
            }
            let content = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("SGX-PCK-Cert: {:?}", content);
        }
    }
}

pub fn sgx_ql_get_quote_verification_collateral(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    fmspc: String,
    pck_ca: String,
    data_source: DataSource,
    collateral_version: String,
    pccs_url: String,
    all_verification_collateral: u64,
) {
    let pck_id = if pck_ca == "platform" {
        CAID::Platform
    } else {
        CAID::Processor
    };
    let enclave_id = EnclaveID::QE;

    if data_source == DataSource::All || data_source == DataSource::Azure {
        std::env::set_var("AZDCAP_COLLATERAL_VERSION", collateral_version.clone());
        let fmspc_slices =
            hex::decode(fmspc.trim_start_matches("0x")).expect("Failed to decode hex string");
        let fmspc_size = fmspc_slices.len() as u16;
        println!(
            "fmspc_slices: {:?}, fmspc_size: {:?}",
            fmspc_slices, fmspc_size
        );
        let fmspc_pointer = fmspc_slices.as_ptr();
        let pck_ca_c_string = CString::new(pck_ca.clone()).expect("CString conversion failed");
        let pck_ca_pointer = pck_ca_c_string.as_ptr();
        let mut p_quote_collateral: *mut SgxQlQveCollateral = std::ptr::null_mut();
        let pp_quote_collateral: *mut *mut SgxQlQveCollateral =
            &mut p_quote_collateral as *mut *mut SgxQlQveCollateral;
        let ret = azure::az_dcap_sgx_ql_get_quote_verification_collateral(
            fmspc_pointer,
            fmspc_size,
            pck_ca_pointer,
            pp_quote_collateral,
        );
        println!(
            "azure dcap sgx_ql_get_quote_verification_collateral func: {:?}",
            ret
        );
        let p_quote_collateral = unsafe { *pp_quote_collateral };
        unsafe { p_quote_collateral.as_ref().unwrap().print() };

        let root_ca_crl = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().root_ca_crl,
                p_quote_collateral.as_ref().unwrap().root_ca_crl_size as usize,
            )
        };
        let root_ca_crl: Vec<u8> = root_ca_crl.to_vec().into_iter().map(|x| x as u8).collect();
        let root_ca_crl = std::str::from_utf8(&root_ca_crl).unwrap();
        let pck_crl = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().pck_crl,
                p_quote_collateral.as_ref().unwrap().pck_crl_size as usize,
            )
        };
        let pck_crl: Vec<u8> = pck_crl.to_vec().into_iter().map(|x| x as u8).collect();
        let pck_crl = std::str::from_utf8(&pck_crl)
            .unwrap()
            .trim_end_matches("\0");
        let tcb_info_str = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().tcb_info,
                p_quote_collateral.as_ref().unwrap().tcb_info_size as usize,
            )
        };
        let tcb_info_str: Vec<u8> = tcb_info_str.to_vec().into_iter().map(|x| x as u8).collect();
        let tcb_info_str = std::str::from_utf8(&tcb_info_str)
            .unwrap()
            .trim_end_matches("\0");
        let enclave_identity_str = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().qe_identity,
                p_quote_collateral.as_ref().unwrap().qe_identity_size as usize,
            )
        };
        let enclave_identity_str: Vec<u8> = enclave_identity_str
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let enclave_identity_str = std::str::from_utf8(&enclave_identity_str)
            .unwrap()
            .trim_end_matches("\0");
        let enclave_identity_issuer_chains_str = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral
                    .as_ref()
                    .unwrap()
                    .qe_identity_issuer_chain,
                p_quote_collateral
                    .as_ref()
                    .unwrap()
                    .qe_identity_issuer_chain_size as usize,
            )
        };
        let enclave_identity_issuer_chains_str: Vec<u8> = enclave_identity_issuer_chains_str
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let enclave_identity_issuer_chains_str =
            std::str::from_utf8(&enclave_identity_issuer_chains_str)
                .unwrap()
                .trim_end_matches("\0");
        println!("root_ca_crl: {}", root_ca_crl);
        println!("pck_crl: {}", pck_crl);
        println!("tcb_info_str: {:?}", tcb_info_str);
        println!("enclave_identity_str: {:?}", enclave_identity_str);
        println!(
            "enclave_identity_issuer_chains_str: {}",
            enclave_identity_issuer_chains_str
        );
        update_verification_collateral(
            &private_key,
            rpc_url.clone(),
            chain_id,
            Some(root_ca_crl),
            pck_id,
            pck_crl,
            tcb_info_str,
            enclave_id,
            collateral_version.clone(),
            enclave_identity_str,
            enclave_identity_issuer_chains_str,
            all_verification_collateral,
        );

        let ret = azure::az_dcap_sgx_ql_free_quote_verification_collateral(p_quote_collateral);
        println!(
            "azure dcap sgx_ql_free_quote_verification_collateral func: {:?}",
            ret
        );
    }
    if data_source == DataSource::All || data_source == DataSource::Local {
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.2 Section Get PCK Cert CRL
        let req_url = format!(
            "{}/sgx/certification/{}/pckcrl?ca={}",
            pccs_url,
            collateral_version.clone(),
            pck_ca.clone()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        let pck_crl = if response.status().is_success() {
            let headers = response.headers();
            if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
                println!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
            }
            let content = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("SGX-PCK-CRL: {}", content);
            content
        } else {
            println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
            return;
        };
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.7 Section Get Root CA CRL
        // This API endpoint is not found in Intel PCS: https://api.portal.trustedservices.intel.com/content/documentation.html
        // let req_url = format!(
        //     "{}/sgx/certification/{}/rootcacrl",
        //     pccs_url,
        //     collateral_version.clone()
        // );
        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()
        //     .unwrap();
        // let response = match rt.block_on(reqwest::get(req_url.clone())) {
        //     Ok(v) => v,
        //     Err(_) => {
        //         println!("Unable to get {}", req_url);
        //         return;
        //     }
        // };
        // if response.status().is_success() {
        //     let content = match rt.block_on(response.text()) {
        //         Ok(v) => v,
        //         Err(_) => {
        //             println!("Unable to get the content of {}", req_url);
        //             return;
        //         }
        //     };
        //     println!("SGX-Root-CA-Crl: {:?}", content);
        // }
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.3 Section Get TCB Info
        let req_url = format!(
            "{}/sgx/certification/{}/tcb?fmspc={}",
            pccs_url,
            collateral_version.clone(),
            fmspc
        );
        println!("req_url: {:?}", req_url);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        let tcb_info_str = if response.status().is_success() {
            let headers = response.headers();
            // v3
            if let Some(cert) = headers.get("SGX-TCB-Info-Issuer-Chain") {
                println!("SGX-TCB-Info-Issuer-Chain: {:?}", cert);
            }
            // v4
            if let Some(cert) = headers.get("TCB-Info-Issuer-Chain") {
                println!("TCB-Info-Issuer-Chain: {:?}", cert);
            }
            let content = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("TCB-Info: {}", content);
            content
        } else {
            println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
            return;
        };
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.4 Section Get QE Identity
        let req_url = format!(
            "{}/sgx/certification/{}/qe/identity",
            pccs_url,
            collateral_version.clone()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        if response.status().is_success() {
            let headers = response.headers();
            let enclave_identity_issuer_chains_str =
                if let Some(cert) = headers.get("SGX-Enclave-Identity-Issuer-Chain") {
                    cert.to_str().unwrap().to_string()
                } else {
                    println!(
                        "Cannot find SGX-Enclave-Identity-Issuer-Chain in {:?}, exit",
                        req_url
                    );
                    return;
                };
            let enclave_identity_issuer_chains_str =
                urlencoding::decode(&enclave_identity_issuer_chains_str).expect("Invalid UTF-8");
            let enclave_identity_issuer_chains_str = enclave_identity_issuer_chains_str.to_string();
            let qe_identity_str = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!(
                "SGX-Enclave-Identity-Issuer-Chain: {}",
                enclave_identity_issuer_chains_str
            );
            println!("SGX-QE-Identity: {}", qe_identity_str);

            update_verification_collateral(
                &private_key,
                rpc_url,
                chain_id,
                None,
                pck_id,
                pck_crl.as_str(),
                tcb_info_str.as_str(),
                enclave_id,
                collateral_version.clone(),
                qe_identity_str.as_str(),
                enclave_identity_issuer_chains_str.as_str(),
                all_verification_collateral,
            );
        } else {
            println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
            return;
        };
    }
}

pub fn tdx_ql_get_quote_verification_collateral(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    fmspc: String,
    pck_ca: String,
    data_source: DataSource,
    collateral_version: String,
    pccs_url: String,
    all_verification_collateral: u64,
) {
    let pck_id = if pck_ca == "platform" {
        CAID::Platform
    } else {
        CAID::Processor
    };
    let enclave_id = EnclaveID::TD_QE;

    if data_source == DataSource::All || data_source == DataSource::Azure {
        std::env::set_var("AZDCAP_COLLATERAL_VERSION", collateral_version.clone());
        let fmspc_slices =
            hex::decode(fmspc.trim_start_matches("0x")).expect("Failed to decode hex string");
        let fmspc_size = fmspc_slices.len() as u16;
        println!(
            "fmspc_slices: {:?}, fmspc_size: {:?}",
            fmspc_slices, fmspc_size
        );
        let fmspc_pointer = fmspc_slices.as_ptr();
        let pck_ca_c_string = CString::new(pck_ca.clone()).expect("CString conversion failed");
        let pck_ca_pointer = pck_ca_c_string.as_ptr();
        let mut p_quote_collateral: *mut SgxQlQveCollateral = std::ptr::null_mut();
        let pp_quote_collateral: *mut *mut SgxQlQveCollateral =
            &mut p_quote_collateral as *mut *mut SgxQlQveCollateral;
        let ret = azure::az_dcap_tdx_ql_get_quote_verification_collateral(
            fmspc_pointer,
            fmspc_size,
            pck_ca_pointer,
            pp_quote_collateral,
        );
        println!(
            "azure dcap tdx_ql_get_quote_verification_collateral func: {:?}",
            ret
        );
        let p_quote_collateral = unsafe { *pp_quote_collateral };
        unsafe { p_quote_collateral.as_ref().unwrap().print() };

        let root_ca_crl = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().root_ca_crl,
                p_quote_collateral.as_ref().unwrap().root_ca_crl_size as usize,
            )
        };
        let root_ca_crl: Vec<u8> = root_ca_crl.to_vec().into_iter().map(|x| x as u8).collect();
        let root_ca_crl = std::str::from_utf8(&root_ca_crl).unwrap();
        let pck_crl = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().pck_crl,
                p_quote_collateral.as_ref().unwrap().pck_crl_size as usize,
            )
        };
        let pck_crl: Vec<u8> = pck_crl.to_vec().into_iter().map(|x| x as u8).collect();
        let pck_crl = std::str::from_utf8(&pck_crl)
            .unwrap()
            .trim_end_matches("\0");
        let tcb_info_str = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().tcb_info,
                p_quote_collateral.as_ref().unwrap().tcb_info_size as usize,
            )
        };
        let tcb_info_str: Vec<u8> = tcb_info_str.to_vec().into_iter().map(|x| x as u8).collect();
        let tcb_info_str = std::str::from_utf8(&tcb_info_str)
            .unwrap()
            .trim_end_matches("\0");
        let enclave_identity_str = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral.as_ref().unwrap().qe_identity,
                p_quote_collateral.as_ref().unwrap().qe_identity_size as usize,
            )
        };
        let enclave_identity_str: Vec<u8> = enclave_identity_str
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let enclave_identity_str = std::str::from_utf8(&enclave_identity_str)
            .unwrap()
            .trim_end_matches("\0");
        let enclave_identity_issuer_chains_str = unsafe {
            std::slice::from_raw_parts(
                p_quote_collateral
                    .as_ref()
                    .unwrap()
                    .qe_identity_issuer_chain,
                p_quote_collateral
                    .as_ref()
                    .unwrap()
                    .qe_identity_issuer_chain_size as usize,
            )
        };
        let enclave_identity_issuer_chains_str: Vec<u8> = enclave_identity_issuer_chains_str
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let enclave_identity_issuer_chains_str =
            std::str::from_utf8(&enclave_identity_issuer_chains_str)
                .unwrap()
                .trim_end_matches("\0");
        println!("root_ca_crl: {}", root_ca_crl);
        println!("pck_crl: {}", pck_crl);
        println!("tcb_info_str: {:?}", tcb_info_str);
        println!("enclave_identity_str: {:?}", enclave_identity_str);
        println!(
            "enclave_identity_issuer_chains_str: {}",
            enclave_identity_issuer_chains_str
        );
        update_verification_collateral(
            &private_key,
            rpc_url.clone(),
            chain_id,
            Some(root_ca_crl),
            pck_id,
            pck_crl,
            tcb_info_str,
            enclave_id,
            collateral_version.clone(),
            enclave_identity_str,
            enclave_identity_issuer_chains_str,
            all_verification_collateral,
        );

        let ret = azure::az_dcap_tdx_ql_free_quote_verification_collateral(p_quote_collateral);
        println!(
            "azure dcap tdx_ql_free_quote_verification_collateral func: {:?}",
            ret
        );
    }
    if data_source == DataSource::All || data_source == DataSource::Local {
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.2 Section Get PCK Cert CRL
        let req_url = format!(
            "{}/sgx/certification/{}/pckcrl?ca={}",
            pccs_url,
            collateral_version.clone(),
            pck_ca.clone()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        let pck_crl = if response.status().is_success() {
            let headers = response.headers();
            if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
                println!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
            }
            let content = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("SGX-PCK-CRL: {}", content);
            content
        } else {
            println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
            return;
        };
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.7 Section Get Root CA CRL
        // This API endpoint is not found in Intel PCS: https://api.portal.trustedservices.intel.com/content/documentation.html
        // let req_url = format!(
        //     "{}/sgx/certification/{}/rootcacrl",
        //     pccs_url,
        //     collateral_version.clone()
        // );
        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()
        //     .unwrap();
        // let response = match rt.block_on(reqwest::get(req_url.clone())) {
        //     Ok(v) => v,
        //     Err(_) => {
        //         println!("Unable to get {}", req_url);
        //         return;
        //     }
        // };
        // if response.status().is_success() {
        //     let content = match rt.block_on(response.text()) {
        //         Ok(v) => v,
        //         Err(_) => {
        //             println!("Unable to get the content of {}", req_url);
        //             return;
        //         }
        //     };
        //     println!("SGX-Root-CA-Crl: {:?}", content);
        // }
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.3 Section Get TCB Info
        let req_url = format!(
            "{}/tdx/certification/{}/tcb?fmspc={}",
            pccs_url,
            collateral_version.clone(),
            fmspc
        );
        println!("req_url: {:?}", req_url);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        let tcb_info_str = if response.status().is_success() {
            let headers = response.headers();
            // v3
            if let Some(cert) = headers.get("SGX-TCB-Info-Issuer-Chain") {
                println!("SGX-TCB-Info-Issuer-Chain: {:?}", cert);
            }
            // v4
            if let Some(cert) = headers.get("TCB-Info-Issuer-Chain") {
                println!("TCB-Info-Issuer-Chain: {:?}", cert);
            }
            let content = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("TCB-Info: {}", content);
            content
        } else {
            println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
            return;
        };
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.6 Section Get TD QE Identity
        let req_url = format!(
            "{}/tdx/certification/{}/qe/identity",
            pccs_url,
            collateral_version.clone()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        if response.status().is_success() {
            let headers = response.headers();
            let enclave_identity_issuer_chains_str =
                if let Some(cert) = headers.get("SGX-Enclave-Identity-Issuer-Chain") {
                    cert.to_str().unwrap().to_string()
                } else {
                    println!(
                        "Cannot find SGX-Enclave-Identity-Issuer-Chain in {:?}, exit",
                        req_url
                    );
                    return;
                };
            let enclave_identity_issuer_chains_str =
                urlencoding::decode(&enclave_identity_issuer_chains_str).expect("Invalid UTF-8");
            let enclave_identity_issuer_chains_str = enclave_identity_issuer_chains_str.to_string();
            let qe_identity_str = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!(
                "SGX-Enclave-Identity-Issuer-Chain: {}",
                enclave_identity_issuer_chains_str
            );
            println!("TDX-QE-Identity: {}", qe_identity_str);
            update_verification_collateral(
                &private_key,
                rpc_url,
                chain_id,
                None,
                pck_id,
                pck_crl.as_str(),
                tcb_info_str.as_str(),
                enclave_id,
                collateral_version.clone(),
                qe_identity_str.as_str(),
                enclave_identity_issuer_chains_str.as_str(),
                all_verification_collateral,
            );
        } else {
            println!("[ERROR] {} returns {:?}, exit", req_url, response.status());
            return;
        };
    }
}

pub fn sgx_ql_get_qve_identity(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    data_source: DataSource,
    collateral_version: String,
    pccs_url: String,
) {
    let mut p_qve_identity: *mut c_char = std::ptr::null_mut();
    let pp_qve_identity: *mut *mut c_char = &mut p_qve_identity as *mut *mut c_char;
    let mut qve_identity_size: u32 = 0u32;
    let p_qve_identity_size: *mut u32 = &mut qve_identity_size;
    let mut p_qve_identity_issuer_chain: *mut c_char = std::ptr::null_mut();
    let pp_qve_identity_issuer_chain: *mut *mut c_char =
        &mut p_qve_identity_issuer_chain as *mut *mut c_char;
    let mut qve_identity_issuer_chain_size: u32 = 0u32;
    let p_qve_identity_issuer_chain_size: *mut u32 = &mut qve_identity_issuer_chain_size;

    if data_source == DataSource::All || data_source == DataSource::Azure {
        std::env::set_var("AZDCAP_COLLATERAL_VERSION", collateral_version.clone());
        let ret = azure::az_dcap_sgx_ql_get_qve_identity(
            pp_qve_identity,
            p_qve_identity_size,
            pp_qve_identity_issuer_chain,
            p_qve_identity_issuer_chain_size,
        );
        println!("azure dcap sgx_ql_get_qve_identity func: {:?}", ret);
        println!("qve_identity_size: {:?}", qve_identity_size);
        let qve_identity = unsafe { *pp_qve_identity };
        let qve_identity_c_str = unsafe {
            CStr::from_bytes_with_nul(std::slice::from_raw_parts(
                qve_identity as *const u8,
                qve_identity_size as usize,
            ))
        };
        let qve_identity_str = if let Ok(s) = qve_identity_c_str.map(|c| c.to_string_lossy()) {
            println!("qve_identity: {}", s);
            s.to_string()
        } else {
            println!("Unable to convert qve_identity");
            return;
        };

        println!(
            "qve_identity_issuer_chain_size: {:?}",
            qve_identity_issuer_chain_size
        );
        let qve_identity_issuer_chain = unsafe { *pp_qve_identity_issuer_chain };
        let qve_identity_issuer_chain_c_str = unsafe {
            CStr::from_bytes_with_nul(std::slice::from_raw_parts(
                qve_identity_issuer_chain as *const u8,
                qve_identity_issuer_chain_size as usize,
            ))
        };
        let issuer_chains_str =
            if let Ok(s) = qve_identity_issuer_chain_c_str.map(|c| c.to_string_lossy()) {
                println!("qve_identity_issuer_chain: {}", s);
                s.to_string()
            } else {
                println!("Unable to convert qve_identity_issuer_chain");
                return;
            };
        upsert_enclave_identity(
            &private_key,
            rpc_url.clone(),
            chain_id,
            EnclaveID::QVE,
            collateral_version.clone(),
            &qve_identity_str,
            &issuer_chains_str,
        );

        let ret = azure::az_dcap_sgx_ql_free_qve_identity(qve_identity, qve_identity_issuer_chain);
        println!("azure dcap sgx_ql_free_qve_identity func: {:?}", ret);
    }
    if data_source == DataSource::All || data_source == DataSource::Local {
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.5 Section Get QvE Identity
        let req_url = format!(
            "{}/sgx/certification/{}/qve/identity",
            pccs_url,
            collateral_version.clone()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        if response.status().is_success() {
            let headers = response.headers();
            let issuer_chains_str =
                if let Some(cert) = headers.get("SGX-Enclave-Identity-Issuer-Chain") {
                    cert.to_str().unwrap().to_string()
                } else {
                    println!(
                        "Cannot find SGX-Enclave-Identity-Issuer-Chain in {:?}, exit",
                        req_url
                    );
                    return;
                };
            let issuer_chains_str = urlencoding::decode(&issuer_chains_str).expect("Invalid UTF-8");
            let issuer_chains_str = issuer_chains_str.to_string();
            let qve_identity_str = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("SGX-Enclave-Identity-Issuer-Chain: {}", issuer_chains_str);
            println!("SGX-QvE-Identity: {}", qve_identity_str);

            upsert_enclave_identity(
                &private_key,
                rpc_url,
                chain_id,
                EnclaveID::QVE,
                collateral_version.clone(),
                &qve_identity_str,
                &issuer_chains_str,
            );
        }
    }
}

pub fn sgx_ql_get_root_ca_crl(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    data_source: DataSource,
    collateral_version: String,
    pccs_url: String,
) {
    let mut p_root_ca_crl: *mut c_char = std::ptr::null_mut();
    let pp_root_ca_crl: *mut *mut c_char = &mut p_root_ca_crl as *mut *mut c_char;

    let mut root_ca_crl_size: u16 = 0;
    let p_root_ca_crl_size: *mut u16 = &mut root_ca_crl_size;

    if data_source == DataSource::All || data_source == DataSource::Azure {
        std::env::set_var("AZDCAP_COLLATERAL_VERSION", collateral_version.clone());
        let ret = azure::az_dcap_sgx_ql_get_root_ca_crl(pp_root_ca_crl, p_root_ca_crl_size);
        println!("azure dcap sgx_ql_get_root_ca_crl func: {:?}", ret);
        println!("root_ca_crl_size: {:?}", root_ca_crl_size);
        let root_ca_crl = unsafe { *pp_root_ca_crl };
        let root_ca_crl_c_str = unsafe {
            CStr::from_bytes_with_nul(std::slice::from_raw_parts(
                root_ca_crl as *const u8,
                root_ca_crl_size as usize,
            ))
        };

        let crl = if let Ok(s) = root_ca_crl_c_str.map(|c| c.to_string_lossy()) {
            println!("root_ca_crl: {}", s);
            s.to_string()
        } else {
            println!("Unable to convert root_ca_crl");
            return;
        };
        upsert_root_ca_crl(&private_key, rpc_url, chain_id, &crl);

        let ret = azure::az_dcap_sgx_ql_free_root_ca_crl(root_ca_crl);
        println!("azure dcap sgx_ql_free_root_ca_crl func: {:?}", ret);
    }
    if data_source == DataSource::All || data_source == DataSource::Local {
        // Ref: https://download.01.org/intel-sgx/sgx-dcap/1.19/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf || 3.7 Section Get Root CA CRL
        let req_url = format!(
            "{}/sgx/certification/{}/rootcacrl",
            pccs_url,
            collateral_version.clone()
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let response = match rt.block_on(reqwest::get(req_url.clone())) {
            Ok(v) => v,
            Err(_) => {
                println!("Unable to get {}", req_url);
                return;
            }
        };
        if response.status().is_success() {
            let content = match rt.block_on(response.text()) {
                Ok(v) => v,
                Err(_) => {
                    println!("Unable to get the content of {}", req_url);
                    return;
                }
            };
            println!("SGX-Root-CA-Crl: {:?}", content);
        }
    }
}

pub async fn upsert_tcb_eval_data_number_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    platform: &str, // "tdx" or "sgx"
    tcb_eval_data_number_dao: &str,
) -> bool {
    // Ref: https://api.portal.trustedservices.intel.com/content/documentation.html#pcs-retrieve-tcbevalnumbers-v4
    let req_url = format!(
        "https://api.trustedservices.intel.com/{}/certification/v4/tcbevaluationdatanumbers",
        platform
    );
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    if response.status().is_success() {
        let tcb_eval_data_number = match response.text().await {
            Ok(v) => v,
            Err(e) => {
                log::error!("Unable to get the content of {}, error = {:?}", req_url, e);
                return false;
            }
        };
        log::info!("TCB Evaluation Data Number: {}", tcb_eval_data_number);
        upsert_tcb_eval_data_number(
            &private_key,
            rpc_url,
            chain_id,
            gas_price,
            tcb_eval_data_number_dao,
            &tcb_eval_data_number,
        )
        .await
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    }
}

pub async fn upsert_root_ca_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, pcs_dao_contract_addr);
    let req_url =
        format!("https://api.trustedservices.intel.com/sgx/certification/v4/pckcrl?ca=platform",);
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    let cert_chains = if response.status().is_success() {
        let headers = response.headers();
        if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
            log::info!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
            let cert_chain = cert.to_str().unwrap().to_string();
            let issuer_chains_str = match urlencoding::decode(&cert_chain) {
                Ok(v) => v,
                Err(_) => {
                    log::error!("Invalid UTF-8 in SGX-PCK-CRL-Issuer-Chain");
                    return false;
                }
            };
            issuer_chains_str.to_string()
        } else {
            log::error!(
                "Cannot find SGX-PCK-CRL-Issuer-Chain in {:?}, exit",
                req_url
            );
            return false;
        }
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    };
    let cert_chains_str = cert_chains.as_str();
    let certs_str: Vec<&str> = cert_chains_str.split("-----END CERTIFICATE-----").collect();
    let mut certs = Vec::new();
    for cert in certs_str {
        let current_cert = cert.trim();
        if current_cert == "\0" {
            continue;
        }
        if !current_cert.is_empty() {
            let cert_str: String = format!("{}\n-----END CERTIFICATE-----\n", current_cert);
            match X509::from_pem(&cert_str.as_bytes()) {
                Ok(cert) => {
                    certs.push(hex::encode(cert.to_der().unwrap()));
                }
                Err(err) => {
                    log::error!("Error parsing certificate: {:?}", err);
                    return false;
                }
            }
        }
    }
    assert_eq!(certs.len(), 2);

    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao = PcsDao::new(
        parse_address_from_str(pcs_dao_contract_addr),
        signer.clone(),
    );

    let call = pcs_dao
        .upsert_pcs_certificates(CAID::Root as u8, Bytes::from_str(&certs[1]).unwrap())
        .gas_price(gas_price);
    let gas_with_buf = match estimate_gas_at_latest(
        signer.as_ref(),
        &call.tx,
        &log_prefix,
        "upsert_pcs_certificates][root",
    )
    .await
    {
        Some(g) => g,
        None => return false,
    };
    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!(
                "{} txn[upsert_pcs_certificates][root] hash: {:?}",
                log_prefix,
                pending_tx.tx_hash()
            );
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!(
                        "{} txn[upsert_pcs_certificates][root] receipt: {:?}",
                        log_prefix,
                        receipt
                    );
                    return true;
                }
                Ok(Err(err)) => {
                    log::error!(
                        "{} txn[upsert_pcs_certificates][root] receipt meet error: {:?}",
                        log_prefix,
                        err
                    );
                    return false;
                }
                Err(_) => {
                    log::error!("{} txn[upsert_pcs_certificates][root] timeout waiting for confirmation after {:?}", log_prefix, TX_CONFIRMATION_TIMEOUT);
                    return false;
                }
            }
        }
        Err(err) => {
            log::error!(
                "{} txn[upsert_pcs_certificates][root] meet error: {:?}",
                log_prefix,
                err
            );
            return false;
        }
    }
}

pub async fn upsert_root_ca_crl_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    upsert_root_ca_crl_internal(
        private_key,
        rpc_url,
        chain_id,
        gas_price,
        pcs_dao_contract_addr,
    )
    .await
}

async fn upsert_root_ca_crl_internal(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, pcs_dao_contract_addr);
    let req_url = format!("https://certificates.trustedservices.intel.com/IntelSGXRootCA.crl",);
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    let root_ca_crl = if response.status().is_success() {
        let content = match response.text().await {
            Ok(v) => v,
            Err(e) => {
                log::error!("Unable to get the content of {}, error = {:?}", req_url, e);
                return false;
            }
        };
        log::info!("ROOT-CA-CRL: {}", content);
        content
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    };
    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao = PcsDao::new(
        parse_address_from_str(pcs_dao_contract_addr),
        signer.clone(),
    );
    let is_eager_v2 = match detect_eager_crl_v2(&pcs_dao, &log_prefix).await {
        Some(value) => value,
        None => return false,
    };
    let root_ca_crl = match X509Crl::from_pem(root_ca_crl.as_bytes()) {
        Ok(c) => {
            if !crl_is_current(&c) {
                log::error!("{} Intel ROOT CA CRL is not currently valid", log_prefix);
                return false;
            }
            Bytes::from(c.to_der().unwrap())
        }
        Err(err) => {
            log::error!("Error parsing certificate: {:?}", err);
            return false;
        }
    };
    let crl_for_readiness = if let Some(stored_crl) =
        reusable_stored_crl(&pcs_dao, CAID::Root as u8, &root_ca_crl, &log_prefix).await
    {
        if is_eager_v2 {
            log::info!(
                "{} ROOT CA CRL is already current; verifying its V2 exact index",
                log_prefix
            );
        } else {
            log::info!(
                "{} ROOT CA CRL is already current; no legacy upsert is needed",
                log_prefix
            );
        }
        stored_crl
    } else {
        let call = pcs_dao
            .upsert_root_ca_crl(root_ca_crl.clone())
            .gas_price(gas_price);
        let gas_with_buf = match estimate_gas_at_latest(
            signer.as_ref(),
            &call.tx,
            &log_prefix,
            "upsert_root_ca_crl",
        )
        .await
        {
            Some(g) => g,
            None => return false,
        };
        let call = call.gas(gas_with_buf);
        let pending_tx = match call.send().await {
            Ok(tx) => tx,
            Err(err) => {
                log::error!(
                    "{} txn[upsert_root_ca_crl] send failed: {:?}",
                    log_prefix,
                    err
                );
                return false;
            }
        };
        log::info!(
            "{} txn[upsert_root_ca_crl] hash: {:?}",
            log_prefix,
            pending_tx.tx_hash()
        );
        match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
            Ok(Ok(Some(receipt))) if receipt.status == Some(U64::from(1)) => {
                log::info!(
                    "{} txn[upsert_root_ca_crl] receipt: {:?}",
                    log_prefix,
                    receipt
                );
            }
            Ok(Ok(receipt)) => {
                log::error!(
                    "{} txn[upsert_root_ca_crl] missing or failed receipt: {:?}",
                    log_prefix,
                    receipt
                );
                return false;
            }
            Ok(Err(err)) => {
                log::error!(
                    "{} txn[upsert_root_ca_crl] receipt error: {:?}",
                    log_prefix,
                    err
                );
                return false;
            }
            Err(_) => {
                log::error!(
                    "{} txn[upsert_root_ca_crl] timeout waiting for confirmation after {:?}",
                    log_prefix,
                    TX_CONFIRMATION_TIMEOUT
                );
                return false;
            }
        }
        root_ca_crl
    };

    if is_eager_v2 {
        eager_crl_index_is_ready(&pcs_dao, &crl_for_readiness, &log_prefix).await
    } else {
        true
    }
}

pub async fn upsert_platform_ca_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, pcs_dao_contract_addr);
    let req_url =
        format!("https://api.trustedservices.intel.com/sgx/certification/v4/pckcrl?ca=platform",);
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    let cert_chains = if response.status().is_success() {
        let headers = response.headers();
        if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
            log::info!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
            let cert_chain = cert.to_str().unwrap().to_string();
            let issuer_chains_str = match urlencoding::decode(&cert_chain) {
                Ok(v) => v,
                Err(_) => {
                    log::error!("Invalid UTF-8 in SGX-PCK-CRL-Issuer-Chain");
                    return false;
                }
            };
            issuer_chains_str.to_string()
        } else {
            log::error!(
                "Cannot find SGX-PCK-CRL-Issuer-Chain in {:?}, exit",
                req_url
            );
            return false;
        }
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    };
    let cert_chains_str = cert_chains.as_str();
    let certs_str: Vec<&str> = cert_chains_str.split("-----END CERTIFICATE-----").collect();
    let mut certs = Vec::new();
    for cert in certs_str {
        let current_cert = cert.trim();
        if current_cert == "\0" {
            continue;
        }
        if !current_cert.is_empty() {
            let cert_str: String = format!("{}\n-----END CERTIFICATE-----\n", current_cert);
            match X509::from_pem(&cert_str.as_bytes()) {
                Ok(cert) => {
                    certs.push(hex::encode(cert.to_der().unwrap()));
                }
                Err(err) => {
                    log::error!("Error parsing certificate: {:?}", err);
                    return false;
                }
            }
        }
    }
    assert_eq!(certs.len(), 2);

    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao = PcsDao::new(
        parse_address_from_str(pcs_dao_contract_addr),
        signer.clone(),
    );

    let call = pcs_dao
        .upsert_pcs_certificates(CAID::Platform as u8, Bytes::from_str(&certs[0]).unwrap())
        .gas_price(gas_price);
    let gas_with_buf = match estimate_gas_at_latest(
        signer.as_ref(),
        &call.tx,
        &log_prefix,
        "upsert_pcs_certificates][platform",
    )
    .await
    {
        Some(g) => g,
        None => return false,
    };
    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!(
                "{} txn[upsert_pcs_certificates][platform] hash: {:?}",
                log_prefix,
                pending_tx.tx_hash()
            );
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!(
                        "{} txn[upsert_pcs_certificates][platform] receipt: {:?}",
                        log_prefix,
                        receipt
                    );
                    return true;
                }
                Ok(Err(err)) => {
                    log::error!(
                        "{} txn[upsert_pcs_certificates][platform] receipt meet error: {:?}",
                        log_prefix,
                        err
                    );
                    return false;
                }
                Err(_) => {
                    log::error!("{} txn[upsert_pcs_certificates][platform] timeout waiting for confirmation after {:?}", log_prefix, TX_CONFIRMATION_TIMEOUT);
                    return false;
                }
            }
        }
        Err(err) => {
            log::error!(
                "{} txn[upsert_pcs_certificates][platform] meet error: {:?}",
                log_prefix,
                err
            );
            return false;
        }
    }
}

pub async fn upsert_platform_ca_crl_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    upsert_platform_processor_ca_crl(
        private_key,
        rpc_url,
        chain_id,
        gas_price,
        pcs_dao_contract_addr,
        "platform",
    )
    .await
}

pub async fn upsert_processor_ca_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, pcs_dao_contract_addr);
    let req_url =
        format!("https://api.trustedservices.intel.com/sgx/certification/v4/pckcrl?ca=processor",);
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    let cert_chains = if response.status().is_success() {
        let headers = response.headers();
        if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
            log::info!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
            let cert_chain = cert.to_str().unwrap().to_string();
            let issuer_chains_str = match urlencoding::decode(&cert_chain) {
                Ok(v) => v,
                Err(_) => {
                    log::error!("Invalid UTF-8 in SGX-PCK-CRL-Issuer-Chain");
                    return false;
                }
            };
            issuer_chains_str.to_string()
        } else {
            log::error!(
                "Cannot find SGX-PCK-CRL-Issuer-Chain in {:?}, exit",
                req_url
            );
            return false;
        }
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    };
    let cert_chains_str = cert_chains.as_str();
    let certs_str: Vec<&str> = cert_chains_str.split("-----END CERTIFICATE-----").collect();
    let mut certs = Vec::new();
    for cert in certs_str {
        let current_cert = cert.trim();
        if current_cert == "\0" {
            continue;
        }
        if !current_cert.is_empty() {
            let cert_str: String = format!("{}\n-----END CERTIFICATE-----\n", current_cert);
            match X509::from_pem(&cert_str.as_bytes()) {
                Ok(cert) => {
                    certs.push(hex::encode(cert.to_der().unwrap()));
                }
                Err(err) => {
                    log::error!("Error parsing certificate: {:?}", err);
                    return false;
                }
            }
        }
    }
    assert_eq!(certs.len(), 2);

    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao = PcsDao::new(
        parse_address_from_str(pcs_dao_contract_addr),
        signer.clone(),
    );

    let call = pcs_dao
        .upsert_pcs_certificates(CAID::Processor as u8, Bytes::from_str(&certs[0]).unwrap())
        .gas_price(gas_price);
    let gas_with_buf = match estimate_gas_at_latest(
        signer.as_ref(),
        &call.tx,
        &log_prefix,
        "upsert_pcs_certificates][processor",
    )
    .await
    {
        Some(g) => g,
        None => return false,
    };
    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!(
                "{} txn[upsert_pcs_certificates][processor] hash: {:?}",
                log_prefix,
                pending_tx.tx_hash()
            );
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!(
                        "{} txn[upsert_pcs_certificates][processor] receipt: {:?}",
                        log_prefix,
                        receipt
                    );
                    return true;
                }
                Ok(Err(err)) => {
                    log::error!(
                        "{} txn[upsert_pcs_certificates][processor] receipt meet error: {:?}",
                        log_prefix,
                        err
                    );
                    return false;
                }
                Err(_) => {
                    log::error!("{} txn[upsert_pcs_certificates][processor] timeout waiting for confirmation after {:?}", log_prefix, TX_CONFIRMATION_TIMEOUT);
                    return false;
                }
            }
        }
        Err(err) => {
            log::error!(
                "{} txn[upsert_pcs_certificates][processor] meet error: {:?}",
                log_prefix,
                err
            );
            return false;
        }
    }
}

pub async fn upsert_processor_ca_crl_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    upsert_platform_processor_ca_crl(
        private_key,
        rpc_url,
        chain_id,
        gas_price,
        pcs_dao_contract_addr,
        "processor",
    )
    .await
}

pub async fn upsert_tcb_signing_ca_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, pcs_dao_contract_addr);
    // Ref: https://api.portal.trustedservices.intel.com/content/documentation.html#pcs-retrieve-tcbevalnumbers-v4
    let req_url = format!(
        "https://api.trustedservices.intel.com/tdx/certification/v4/tcbevaluationdatanumbers",
    );
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    let cert_chains = if response.status().is_success() {
        let headers = response.headers();
        if let Some(cert) = headers.get("TCB-Evaluation-Data-Numbers-Issuer-Chain") {
            log::info!("TCB-Evaluation-Data-Numbers-Issuer-Chain: {:?}", cert);
            let cert_chain = cert.to_str().unwrap().to_string();
            let issuer_chains_str = match urlencoding::decode(&cert_chain) {
                Ok(v) => v,
                Err(_) => {
                    log::error!("Invalid UTF-8 in TCB-Evaluation-Data-Numbers-Issuer-Chain");
                    return false;
                }
            };
            issuer_chains_str.to_string()
        } else {
            log::error!(
                "Cannot find TCB-Evaluation-Data-Numbers-Issuer-Chain in {:?}, exit",
                req_url
            );
            return false;
        }
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    };
    let cert_chains_str = cert_chains.as_str();
    let certs_str: Vec<&str> = cert_chains_str.split("-----END CERTIFICATE-----").collect();
    let mut certs = Vec::new();
    for cert in certs_str {
        let current_cert = cert.trim();
        if current_cert == "\0" {
            continue;
        }
        if !current_cert.is_empty() {
            let cert_str: String = format!("{}\n-----END CERTIFICATE-----\n", current_cert);
            match X509::from_pem(&cert_str.as_bytes()) {
                Ok(cert) => {
                    certs.push(hex::encode(cert.to_der().unwrap()));
                }
                Err(err) => {
                    log::error!("Error parsing certificate: {:?}", err);
                    return false;
                }
            }
        }
    }
    assert_eq!(certs.len(), 2);

    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao = PcsDao::new(
        parse_address_from_str(pcs_dao_contract_addr),
        signer.clone(),
    );

    let call = pcs_dao
        .upsert_pcs_certificates(CAID::Signing as u8, Bytes::from_str(&certs[0]).unwrap())
        .gas_price(gas_price);
    let gas_with_buf = match estimate_gas_at_latest(
        signer.as_ref(),
        &call.tx,
        &log_prefix,
        "upsert_pcs_certificates][signing",
    )
    .await
    {
        Some(g) => g,
        None => return false,
    };
    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!(
                "{} txn[upsert_pcs_certificates][signing] hash: {:?}",
                log_prefix,
                pending_tx.tx_hash()
            );
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!(
                        "{} txn[upsert_pcs_certificates][signing] receipt: {:?}",
                        log_prefix,
                        receipt
                    );
                    return true;
                }
                Ok(Err(err)) => {
                    log::error!(
                        "{} txn[upsert_pcs_certificates][signing] receipt meet error: {:?}",
                        log_prefix,
                        err
                    );
                    return false;
                }
                Err(_) => {
                    log::error!("{} txn[upsert_pcs_certificates][signing] timeout waiting for confirmation after {:?}", log_prefix, TX_CONFIRMATION_TIMEOUT);
                    return false;
                }
            }
        }
        Err(err) => {
            log::error!(
                "{} txn[upsert_pcs_certificates][signing] meet error: {:?}",
                log_prefix,
                err
            );
            return false;
        }
    }
}

pub async fn upsert_enclave_identity_func(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    platform: &str,                       // "tdx" or "sgx"
    version: &str,                        // "v3" or "v4" or "v5"
    collateral_update_type: Option<&str>, // "standard" or "early"
    tcb_evaluation_data_number: Option<u32>,
    enclave_identity_dao_contract_addr: &str,
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, enclave_identity_dao_contract_addr);
    let base_url = format!(
        "https://api.trustedservices.intel.com/{}/certification/{}/qe/identity",
        platform, version
    );
    let qe_identity_str = match fetch_intel_collateral_with_eval_fallback(
        &base_url,
        collateral_update_type,
        tcb_evaluation_data_number,
        "enclaveIdentity",
        &log_prefix,
    )
    .await
    {
        Ok(body) => body,
        Err(err) => {
            log::error!("{} failed to fetch QE identity: {}", log_prefix, err);
            return false;
        }
    };
    log::info!("QE-Identity: {}", qe_identity_str);

    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));

    let enclave_identity_dao = EnclaveIdentityDao::new(
        parse_address_from_str(enclave_identity_dao_contract_addr),
        signer.clone(),
    );
    let enclave_id = if platform == "tdx" {
        EnclaveID::TD_QE
    } else if platform == "sgx" {
        EnclaveID::QE
    } else {
        log::error!("Invalid platform: {}", platform);
        return false;
    };
    let id = U256::from(enclave_id as u32);
    let version = if version == "v3" {
        U256::from(3u32)
    } else if version == "v4" {
        U256::from(4u32)
    } else if version == "v5" {
        U256::from(5u32)
    } else {
        log::error!("Invalid version: {}", version);
        return false;
    };
    let enclave_identity_str = qe_identity_str.as_str();
    let enclave_identity: EnclaveIdentity = serde_json::from_str(enclave_identity_str).unwrap();
    let enclave_identity_str = &enclave_identity_str[r#""enclaveIdentity":{"#.len()..];
    let end_idx = enclave_identity_str.find(r#","signature""#).unwrap();
    let enclave_identity_str = &enclave_identity_str[..end_idx];
    let enclave_identity_obj = EnclaveIdentityJsonObj {
        identity_str: enclave_identity_str.to_string(),
        signature: Bytes::from_hex(&enclave_identity.signature).unwrap(),
    };
    log::info!("identity_str = {}", enclave_identity_obj.identity_str);
    log::info!("signature = {}", enclave_identity_obj.signature);
    let call = enclave_identity_dao
        .upsert_enclave_identity(id, version, enclave_identity_obj)
        .gas_price(gas_price);
    let gas_with_buf = match estimate_gas_at_latest(
        signer.as_ref(),
        &call.tx,
        &log_prefix,
        "upsert_enclave_identity",
    )
    .await
    {
        Some(g) => g,
        None => return false,
    };
    match call.gas(gas_with_buf).send().await {
        Ok(pending_tx) => {
            log::info!(
                "{} txn[upsert_enclave_identity] hash: {:?}",
                log_prefix,
                pending_tx.tx_hash()
            );
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    log::info!(
                        "{} txn[upsert_enclave_identity] receipt: {:?}",
                        log_prefix,
                        receipt
                    );
                    return true;
                }
                Ok(Err(err)) => {
                    log::error!(
                        "{} txn[upsert_enclave_identity] receipt meet error: {:?}",
                        log_prefix,
                        err
                    );
                    return false;
                }
                Err(_) => {
                    log::error!("{} txn[upsert_enclave_identity] timeout waiting for confirmation after {:?}", log_prefix, TX_CONFIRMATION_TIMEOUT);
                    return false;
                }
            }
        }
        Err(err) => {
            log::error!(
                "{} txn[upsert_enclave_identity] meet error: {:?}",
                log_prefix,
                err
            );
            return false;
        }
    }
}

async fn upsert_platform_processor_ca_crl(
    private_key: String,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    pcs_dao_contract_addr: &str,
    ca_type: &str, // "platform" or "processor"
) -> bool {
    let log_prefix = format!("[{}][{}]", chain_id, pcs_dao_contract_addr);
    let req_url = format!(
        "https://api.trustedservices.intel.com/sgx/certification/v4/pckcrl?ca={}",
        ca_type,
    );
    let response = match reqwest::get(req_url.clone()).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Unable to get {}, error = {:?}", req_url, e);
            return false;
        }
    };
    let pck_crl = if response.status().is_success() {
        let headers = response.headers();
        if let Some(cert) = headers.get("SGX-PCK-CRL-Issuer-Chain") {
            log::info!("SGX-PCK-CRL-Issuer-Chain: {:?}", cert);
        }
        let content = match response.text().await {
            Ok(v) => v,
            Err(e) => {
                log::error!("Unable to get the content of {}, error = {:?}", req_url, e);
                return false;
            }
        };
        log::info!("SGX-PCK-CRL: {}", content);
        content
    } else {
        log::error!("[ERROR] {} returns {:?}, exit", req_url, response.status());
        return false;
    };
    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = private_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao = PcsDao::new(
        parse_address_from_str(pcs_dao_contract_addr),
        signer.clone(),
    );
    let is_eager_v2 = match detect_eager_crl_v2(&pcs_dao, &log_prefix).await {
        Some(value) => value,
        None => return false,
    };
    let pck_crl = match X509Crl::from_pem(pck_crl.as_bytes()) {
        Ok(c) => {
            if !crl_is_current(&c) {
                log::error!("{} Intel PCK CRL is not currently valid", log_prefix);
                return false;
            }
            Bytes::from(c.to_der().unwrap())
        }
        Err(err) => {
            log::error!("Error parsing certificate: {:?}", err);
            return false;
        }
    };
    let ca = if ca_type == "platform" {
        CAID::Platform as u8
    } else if ca_type == "processor" {
        CAID::Processor as u8
    } else {
        log::error!("Invalid CA type: {}", ca_type);
        return false;
    };
    let crl_for_readiness = if let Some(stored_crl) =
        reusable_stored_crl(&pcs_dao, ca, &pck_crl, &log_prefix).await
    {
        if is_eager_v2 {
            log::info!(
                "{} PCK {} CA CRL is already current; verifying its V2 exact index",
                log_prefix,
                ca_type
            );
        } else {
            log::info!(
                "{} PCK {} CA CRL is already current; no legacy upsert is needed",
                log_prefix,
                ca_type
            );
        }
        stored_crl
    } else {
        let call = pcs_dao
            .upsert_pck_crl(ca, pck_crl.clone())
            .gas_price(gas_price);
        let gas_with_buf =
            match estimate_gas_at_latest(signer.as_ref(), &call.tx, &log_prefix, "upsert_pck_crl")
                .await
            {
                Some(g) => g,
                None => return false,
            };
        let call = call.gas(gas_with_buf);
        let pending_tx = match call.send().await {
            Ok(tx) => tx,
            Err(err) => {
                log::error!("{} txn[upsert_pck_crl] send failed: {:?}", log_prefix, err);
                return false;
            }
        };
        log::info!(
            "{} txn[upsert_pck_crl] hash: {:?}",
            log_prefix,
            pending_tx.tx_hash()
        );
        match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
            Ok(Ok(Some(receipt))) if receipt.status == Some(U64::from(1)) => {
                log::info!("{} txn[upsert_pck_crl] receipt: {:?}", log_prefix, receipt);
            }
            Ok(Ok(receipt)) => {
                log::error!(
                    "{} txn[upsert_pck_crl] missing or failed receipt: {:?}",
                    log_prefix,
                    receipt
                );
                return false;
            }
            Ok(Err(err)) => {
                log::error!(
                    "{} txn[upsert_pck_crl] receipt error: {:?}",
                    log_prefix,
                    err
                );
                return false;
            }
            Err(_) => {
                log::error!(
                    "{} txn[upsert_pck_crl] timeout waiting for confirmation after {:?}",
                    log_prefix,
                    TX_CONFIRMATION_TIMEOUT
                );
                return false;
            }
        }
        pck_crl
    };

    if is_eager_v2 {
        eager_crl_index_is_ready(&pcs_dao, &crl_for_readiness, &log_prefix).await
    } else {
        true
    }
}

#[cfg(test)]
mod intel_eval_fallback_tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn spawn_http_server(
        responses: Vec<(u16, String)>,
    ) -> (String, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured_requests = Arc::clone(&requests);
        let handle = thread::spawn(move || {
            for (status, body) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                let mut raw_request = Vec::new();
                let mut buffer = [0u8; 1024];
                loop {
                    let count = stream.read(&mut buffer).unwrap();
                    if count == 0 {
                        break;
                    }
                    raw_request.extend_from_slice(&buffer[..count]);
                    if raw_request.windows(4).any(|window| window == b"\r\n\r\n") {
                        break;
                    }
                }
                captured_requests
                    .lock()
                    .unwrap()
                    .push(String::from_utf8(raw_request).unwrap());

                let reason = if status == 200 { "OK" } else { "Not Found" };
                let response = format!(
                    "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status,
                    reason,
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).unwrap();
            }
        });
        (
            format!("http://{}/tcb?fmspc=test", address),
            requests,
            handle,
        )
    }

    fn tcb_payload(eval_number: u32) -> String {
        format!(
            r#"{{"tcbInfo":{{"tcbEvaluationDataNumber":{}}},"signature":"aa"}}"#,
            eval_number
        )
    }

    #[tokio::test]
    async fn exact_eval_request_is_preferred() {
        let expected_body = tcb_payload(19);
        let (base_url, requests, server) = spawn_http_server(vec![(200, expected_body.clone())]);

        let body = fetch_intel_collateral_with_eval_fallback(
            &base_url,
            Some("standard"),
            Some(19),
            "tcbInfo",
            "[test]",
        )
        .await
        .unwrap();
        server.join().unwrap();

        assert_eq!(body, expected_body);
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].contains("tcbEvaluationDataNumber=19"));
    }

    #[tokio::test]
    async fn matching_standard_response_is_used_after_exact_404() {
        let expected_body = tcb_payload(19);
        let (base_url, requests, server) =
            spawn_http_server(vec![(404, String::new()), (200, expected_body.clone())]);

        let body = fetch_intel_collateral_with_eval_fallback(
            &base_url,
            Some("standard"),
            Some(19),
            "tcbInfo",
            "[test]",
        )
        .await
        .unwrap();
        server.join().unwrap();

        assert_eq!(body, expected_body);
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert!(requests[0].contains("tcbEvaluationDataNumber=19"));
        assert!(requests[1].starts_with("GET /tcb?fmspc=test HTTP/1.1"));
        assert!(!requests[1].contains("tcbEvaluationDataNumber"));
        assert!(!requests[1].contains("update="));
    }

    #[tokio::test]
    async fn mismatched_standard_response_is_rejected_after_exact_404() {
        let (base_url, _, server) =
            spawn_http_server(vec![(404, String::new()), (200, tcb_payload(19))]);

        let error = fetch_intel_collateral_with_eval_fallback(
            &base_url,
            Some("standard"),
            Some(20),
            "tcbInfo",
            "[test]",
        )
        .await
        .unwrap_err();
        server.join().unwrap();

        assert!(error.contains("expected 20, got 19"));
    }
}

#[cfg(test)]
mod crl_v2_tests {
    use super::*;
    use ethers::abi::{encode, Token};
    use ethers::providers::{JsonRpcError, MockProvider, MockResponse};

    fn signed_mock_pcs_dao() -> (
        PcsDao<SignerMiddleware<Provider<MockProvider>, LocalWallet>>,
        MockProvider,
        Address,
    ) {
        let (provider, mock) = Provider::mocked();
        let wallet = LocalWallet::from_bytes(&[1u8; 32]).unwrap();
        let signer_address = wallet.address();
        let signer = Arc::new(SignerMiddleware::new(provider, wallet));
        (
            PcsDao::new(Address::from_low_u64_be(1), signer),
            mock,
            signer_address,
        )
    }

    fn eth_call_params(tx: &TypedTransaction) -> [serde_json::Value; 2] {
        [
            serde_json::to_value(tx).unwrap(),
            serde_json::to_value(BlockId::Number(BlockNumber::Latest)).unwrap(),
        ]
    }

    fn mock_pcs_dao(response: MockResponse) -> (PcsDao<Provider<MockProvider>>, MockProvider) {
        let (provider, mock) = Provider::mocked();
        let helper = Address::from_low_u64_be(2);

        // MockProvider consumes responses from the back: crlLib() is called
        // first, followed by the final V2 marker getter.
        mock.push_response(response);
        mock.push::<Bytes, _>(Bytes::from(encode(&[Token::Address(helper)])))
            .unwrap();

        (
            PcsDao::new(Address::from_low_u64_be(1), Arc::new(provider)),
            mock,
        )
    }

    #[test]
    fn v2_crl_selectors_match_solidity_interfaces() {
        assert_eq!(
            &ethers::utils::id("crlRevokedSetHashes(bytes32)")[..4],
            &[0x22, 0xad, 0xe4, 0x9d]
        );
        assert_eq!(
            &ethers::utils::id("indexedCrls(bytes32)")[..4],
            &[0x22, 0xb8, 0xcf, 0x17]
        );
        assert_eq!(
            &ethers::utils::id("getTbsAndSig(bytes)")[..4],
            &[0xfc, 0xf0, 0xbe, 0x24]
        );
    }

    #[tokio::test]
    async fn compatible_probe_enables_exact_index_v2() {
        let response = MockResponse::Value(
            serde_json::to_value(Bytes::from(encode(&[Token::FixedBytes(vec![0u8; 32])]))).unwrap(),
        );
        let (pcs_dao, _) = mock_pcs_dao(response);

        assert_eq!(
            detect_eager_crl_v2(&pcs_dao, "[test]").await,
            Some(true)
        );
    }

    #[tokio::test]
    async fn compatible_probe_accepts_v1_without_indexing() {
        let response = MockResponse::Error(JsonRpcError {
            code: 3,
            message: "execution reverted".to_string(),
            data: Some(serde_json::Value::String("0x".to_string())),
        });
        let (pcs_dao, _) = mock_pcs_dao(response);

        assert_eq!(
            detect_eager_crl_v2(&pcs_dao, "[test]").await,
            Some(false)
        );
    }

    #[tokio::test]
    async fn compatible_probe_does_not_downgrade_transport_failure() {
        let (provider, _) = Provider::mocked();
        let pcs_dao = PcsDao::new(Address::from_low_u64_be(1), Arc::new(provider));

        assert_eq!(
            detect_eager_crl_v2(&pcs_dao, "[test]").await,
            None
        );
    }

    #[tokio::test]
    async fn eager_v2_requires_index_ready_after_upsert() {
        let ready_response = MockResponse::Value(
            serde_json::to_value(Bytes::from(encode(&[Token::Bool(true)]))).unwrap(),
        );
        let (pcs_dao, _) = mock_pcs_dao(ready_response);
        assert!(eager_crl_index_is_ready(&pcs_dao, &Bytes::from_static(b"crl"), "[test]").await);

        let missing_response = MockResponse::Value(
            serde_json::to_value(Bytes::from(encode(&[Token::Bool(false)]))).unwrap(),
        );
        let (pcs_dao, _) = mock_pcs_dao(missing_response);
        assert!(!eager_crl_index_is_ready(&pcs_dao, &Bytes::from_static(b"crl"), "[test]").await);
    }

    #[tokio::test]
    async fn reusable_exact_crl_reads_as_authorized_zero_address() {
        let (pcs_dao, mock, _) = signed_mock_pcs_dao();
        let ca = CAID::Platform as u8;
        let stored_crl = Bytes::from_static(b"stored-crl");

        mock.push::<Bytes, _>(Bytes::from(encode(&[
            Token::Bytes(Vec::new()),
            Token::Bytes(stored_crl.to_vec()),
        ])))
        .unwrap();

        assert_eq!(
            reusable_stored_crl(&pcs_dao, ca, &stored_crl, "[test]").await,
            Some(stored_crl)
        );

        let expected = pcs_dao.get_certificate_by_id(ca).from(Address::zero());
        mock.assert_request("eth_call", eth_call_params(&expected.tx))
            .unwrap();
    }

    #[tokio::test]
    async fn reusable_signature_only_reissue_uses_zero_address_for_protected_reads() {
        let (pcs_dao, mock, signer_address) = signed_mock_pcs_dao();
        let ca = CAID::Platform as u8;
        let stored_crl = Bytes::from_static(b"stored-crl");
        let fetched_crl = Bytes::from_static(b"signature-only-reissue");
        let helper_address = Address::from_low_u64_be(2);
        let fetched_tbs = b"identical-tbs".to_vec();
        let key = [0x11; 32];
        let stored_tbs_hash = ethers::utils::keccak256(&fetched_tbs);

        // MockProvider pops responses from the back, so enqueue them in reverse
        // request order.
        mock.push::<Bytes, _>(Bytes::from(encode(&[Token::FixedBytes(
            stored_tbs_hash.to_vec(),
        )])))
        .unwrap();
        mock.push::<Bytes, _>(Bytes::from(encode(&[Token::FixedBytes(key.to_vec())])))
            .unwrap();
        mock.push::<Bytes, _>(Bytes::from(encode(&[
            Token::Bytes(fetched_tbs.clone()),
            Token::Bytes(vec![0x22; 64]),
        ])))
        .unwrap();
        mock.push::<Bytes, _>(Bytes::from(encode(&[Token::Address(helper_address)])))
            .unwrap();
        mock.push::<Bytes, _>(Bytes::from(encode(&[
            Token::Bytes(Vec::new()),
            Token::Bytes(stored_crl.to_vec()),
        ])))
        .unwrap();

        assert_eq!(
            reusable_stored_crl(&pcs_dao, ca, &fetched_crl, "[test]").await,
            Some(stored_crl)
        );

        let expected_certificate_read = pcs_dao.get_certificate_by_id(ca).from(Address::zero());
        mock.assert_request("eth_call", eth_call_params(&expected_certificate_read.tx))
            .unwrap();

        let expected_helper_read = pcs_dao.crl_lib().from(signer_address);
        mock.assert_request("eth_call", eth_call_params(&expected_helper_read.tx))
            .unwrap();

        let crl_helper = X509CrlHelperV2::new(helper_address, pcs_dao.client());
        let expected_tbs_read = crl_helper.get_tbs_and_sig(fetched_crl).from(signer_address);
        mock.assert_request("eth_call", eth_call_params(&expected_tbs_read.tx))
            .unwrap();

        let expected_key_read = pcs_dao.pcs_key(ca, true).from(signer_address);
        mock.assert_request("eth_call", eth_call_params(&expected_key_read.tx))
            .unwrap();

        let expected_hash_read = pcs_dao.get_collateral_hash(key).from(Address::zero());
        mock.assert_request("eth_call", eth_call_params(&expected_hash_read.tx))
            .unwrap();
    }
}
