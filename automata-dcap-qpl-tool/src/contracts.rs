use crate::pccs_types::*;
use automata_dcap_qpl_common::*;
use automata_dcap_qpl_contracts::{
    enclave_identity_dao::{EnclaveIdentityDao, EnclaveIdentityJsonObj},
    fmspc_tcb_dao::{FmspcTcbDao, TcbInfoJsonObj},
    pck_dao::PckDao,
    pcs_dao::PcsDao,
    tcb_eval_dao::{TcbEvalDao, TcbEvalJsonObj},
};
use automata_dcap_qpl_contracts::parse_address_from_env_var::*;
use ethers::prelude::*;
use hex::FromHex;
use openssl::x509::{X509Crl, X509};
use std::{str::FromStr, sync::Arc};
use tokio::time::{timeout, Duration};

/// Timeout for waiting for transaction confirmation (2 minutes)
const TX_CONFIRMATION_TIMEOUT: Duration = Duration::from_secs(120);

lazy_static::lazy_static! {
    pub static ref GAS_PRICE: String = {
        match std::env::var("GAS_PRICE") {
            Ok(g) => g,
            Err(_) => "10000".to_string()
        }
    };
}

pub fn upsert_pck_cert(
    prv_key: &str,
    rpc_url: String,
    chain_id: u64,
    ca: CAID,
    qe_id: String,
    pce_id: String,
    cpu_svn: String,
    pce_svn: String,
    tcbm: String,
    cert_chains_str: &str,
) {
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));

    let pck_dao = PckDao::new(parse_address_from_env_var("PCK_DAO"), signer.clone());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

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
                    println!("Error parsing certificate: {:?}", err);
                    return;
                }
            }
        }
    }
    assert_eq!(certs.len(), 3);

    let pcs_dao = PcsDao::new(parse_address_from_env_var("PCS_DAO"), signer.clone());

    // TODO: Check the Root and Platform/Process intermediate certs before upsert
    match rt.block_on(
        pcs_dao
            .upsert_pcs_certificates(CAID::Root as u8, Bytes::from_str(&certs[2]).unwrap())
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][root] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_pcs_certificates][root] receipt: {:?}", receipt);
                }
                Ok(Err(err)) => {
                    println!(
                        "txn[upsert_pcs_certificates][root] receipt meet error: {:?}",
                        err
                    );
                }
                Err(_) => {
                    println!("txn[upsert_pcs_certificates][root] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_pcs_certificates][root] meet error: {:?}", err);
        }
    }
    match rt.block_on(
        pcs_dao
            .upsert_pcs_certificates(ca as u8, Bytes::from_str(&certs[1]).unwrap())
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][intermediate] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!(
                        "txn[upsert_pcs_certificates][intermediate] receipt: {:?}",
                        receipt
                    );
                }
                Ok(Err(err)) => {
                    println!(
                        "txn[upsert_pcs_certificates][intermediate] receipt meet error: {:?}",
                        err
                    );
                }
                Err(_) => {
                    println!("txn[upsert_pcs_certificates][intermediate] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!(
                "txn[upsert_pcs_certificates][intermediate] meet error: {:?}",
                err
            );
        }
    }
    match rt.block_on(
        pck_dao
            .upsert_pck_cert(
                ca as u8,
                qe_id.clone(),
                pce_id.clone(),
                tcbm.clone(),
                Bytes::from_str(&certs[0]).unwrap(),
            )
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_pck_cert] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_pck_cert] receipt: {:?}", receipt);
                }
                Ok(Err(err)) => {
                    println!("txn[upsert_pck_cert] receipt meet error: {:?}", err);
                }
                Err(_) => {
                    println!("txn[upsert_pck_cert] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_pck_cert] meet error: {:?}", err);
        }
    };
    match rt.block_on(
        pck_dao
            .upsert_platform_tcbs(qe_id, pce_id, cpu_svn, pce_svn, tcbm)
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send()
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_platform_tcbs] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_platform_tcbs] receipt: {:?}", receipt);
                }
                Ok(Err(err)) => {
                    println!("txn[upsert_platform_tcbs] receipt meet error: {:?}", err);
                }
                Err(_) => {
                    println!("txn[upsert_platform_tcbs] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_platform_tcbs] meet error: {:?}", err);
        }
    };
}

pub fn upsert_enclave_identity(
    prv_key: &str,
    rpc_url: String,
    chain_id: u64,
    enclave_id: EnclaveID,
    collateral_version: String,
    enclave_identity_str: &str,
    enclave_identity_issuer_chains_str: &str,
) {
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));

    let pcs_dao = PcsDao::new(parse_address_from_env_var("PCS_DAO"), signer.clone());

    let enclave_identity_dao =
        EnclaveIdentityDao::new(parse_address_from_env_var("ENCLAVE_ID_DAO"), signer.clone());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let certs_str: Vec<&str> = enclave_identity_issuer_chains_str
        .split("-----END CERTIFICATE-----")
        .collect();
    let mut certs = Vec::new();
    for cert in certs_str {
        let current_cert = cert.trim();
        if current_cert == "\0" {
            continue;
        }
        if !current_cert.is_empty() {
            let cert_str = format!("{}\n-----END CERTIFICATE-----\n", current_cert);
            match X509::from_pem(&cert_str.as_bytes()) {
                Ok(cert) => {
                    certs.push(hex::encode(cert.to_der().unwrap()));
                }
                Err(err) => {
                    println!("Error parsing certificate: {:?}", err);
                    return;
                }
            }
        }
    }
    assert_eq!(certs.len(), 2);
    // TODO: Check the Root and Signing certs before upsert
    match rt.block_on(
        pcs_dao
            .upsert_pcs_certificates(CAID::Root as u8, Bytes::from_str(&certs[1]).unwrap())
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][root] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_pcs_certificates][root] receipt: {:?}", receipt);
                }
                Ok(Err(_)) => {}
                Err(_) => {
                    println!("txn[upsert_pcs_certificates][root] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(_) => {}
    }
    match rt.block_on(
        pcs_dao
            .upsert_pcs_certificates(CAID::Signing as u8, Bytes::from_str(&certs[0]).unwrap())
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][signing] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!(
                        "txn[upsert_pcs_certificates][signing] receipt: {:?}",
                        receipt
                    );
                }
                Ok(Err(err)) => {
                    println!("Error: {:?}", err);
                }
                Err(_) => {
                    println!("txn[upsert_pcs_certificates][signing] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
    let id = U256::from(enclave_id as u32);
    let version = if collateral_version == "v1".to_string() {
        U256::from(1u32)
    } else if collateral_version == "v2".to_string() {
        U256::from(2u32)
    } else if collateral_version == "v3".to_string() {
        U256::from(3u32)
    } else if collateral_version == "v4".to_string() {
        U256::from(4u32)
    } else {
        U256::from(3u32) // use v3 as default dcap attestation version
    };
    let enclave_identity: EnclaveIdentity = serde_json::from_str(enclave_identity_str).unwrap();
    // Jiaquan: we cannot use serde lib to deserialize the enclave_identity_str, because in v4 struct, an inner struct is also indexmap here
    // Need to have a better implementation here
    let enclave_identity_str = &enclave_identity_str[r#""enclaveIdentity":{"#.len()..];
    let end_idx = enclave_identity_str.find(r#","signature""#).unwrap();
    let enclave_identity_str = &enclave_identity_str[..end_idx];
    let enclave_identity_obj = EnclaveIdentityJsonObj {
        identity_str: enclave_identity_str.to_string(),
        signature: Bytes::from_hex(&enclave_identity.signature).unwrap(),
    };
    println!("identity_str = {}", enclave_identity_obj.identity_str);
    println!("signature = {}", enclave_identity_obj.signature);
    // println!("{:?}", enclave_identity_dao.upsert_enclave_identity(id, version, enclave_identity_obj));
    match rt.block_on(
        enclave_identity_dao
            .upsert_enclave_identity(id, version, enclave_identity_obj)
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_enclave_identity] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_enclave_identity] receipt: {:?}", receipt);
                }
                Ok(Err(err)) => {
                    println!("txn[upsert_enclave_identity] receipt meet error: {:?}", err);
                }
                Err(_) => {
                    println!("txn[upsert_enclave_identity] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_enclave_identity] meet error: {:?}", err);
        }
    };
}

pub fn upsert_root_ca_crl(prv_key: &str, rpc_url: String, chain_id: u64, crl: &str) {
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));

    let pcs_dao = PcsDao::new(parse_address_from_env_var("PCS_DAO"), signer.clone());
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let crl = match X509Crl::from_pem(crl.as_bytes()) {
        Ok(c) => hex::encode(c.to_der().unwrap()),
        Err(err) => {
            println!("Error parsing certificate: {:?}", err);
            return;
        }
    };
    match rt.block_on(
        pcs_dao
            .upsert_root_ca_crl(Bytes::from_str(&crl).unwrap())
            .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_root_ca_crl] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(timeout(TX_CONFIRMATION_TIMEOUT, pending_tx)) {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_root_ca_crl] receipt: {:?}", receipt);
                }
                Ok(Err(err)) => {
                    println!("txn[upsert_root_ca_crl] receipt meet error: {:?}", err);
                }
                Err(_) => {
                    println!("txn[upsert_root_ca_crl] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_root_ca_crl] meet error: {:?}", err);
        }
    }
}

pub fn update_verification_collateral(
    prv_key: &str,
    rpc_url: String,
    chain_id: u64,
    root_ca_crl: Option<&str>,
    pck: CAID,
    pck_crl: &str,
    tcb_info_str: &str,
    enclave_id: EnclaveID,
    collateral_version: String,
    enclave_identity_str: &str,
    enclave_identity_issuer_chains_str: &str,
    all_verification_collateral: u64,
) {
    let provider = Provider::<Http>::try_from(rpc_url.clone()).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let pcs_dao = PcsDao::new(parse_address_from_env_var("PCS_DAO"), signer.clone());

    let fmspc_tcb_dao =
        FmspcTcbDao::new(parse_address_from_env_var("FMSPC_TCB_DAO"), signer.clone());

    // Root CA CRL
    if all_verification_collateral == 1 {
        if let Some(root_ca_crl) = root_ca_crl {
            upsert_root_ca_crl(prv_key, rpc_url.clone(), chain_id, root_ca_crl);
        }
    }

    // PCK CRL
    if all_verification_collateral == 1 {
        let pck_crl = match X509Crl::from_pem(pck_crl.as_bytes()) {
            Ok(c) => hex::encode(c.to_der().unwrap()),
            Err(err) => {
                println!("Error parsing certificate: {:?}", err);
                return;
            }
        };
        println!("[Jiaquan] pck: {:?}", pck as u8);
        println!("[Jiaquan] pck_crl: {:?}", Bytes::from_str(&pck_crl).unwrap());
        match rt.block_on(
            pcs_dao
                .upsert_pck_crl(pck as u8, Bytes::from_str(&pck_crl).unwrap())
                .gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send(),
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

    // TCB Info
    let tcb_info: TcbInfo = serde_json::from_str(tcb_info_str).unwrap();
    // Jiaquan: we cannot use serde lib to deserialize the tcb_info_str, because tcbLevels inner struct also need to be indexmap here
    // Need to have a better implementation here
    let tcb_info_str = &tcb_info_str[r#""tcbInfo":{"#.len()..];
    let end_idx = tcb_info_str.find(r#","signature""#).unwrap();
    let tcb_info_str = &tcb_info_str[..end_idx];
    let tcb_info_obj = TcbInfoJsonObj {
        tcb_info_str: tcb_info_str.to_string(),
        signature: Bytes::from_hex(tcb_info.signature).unwrap(),
    };
    println!("tcb_info_obj.tcb_info_str: {}", tcb_info_obj.tcb_info_str);
    println!("tcb_info_obj.signature: {:?}", tcb_info_obj.signature);
    match rt.block_on(fmspc_tcb_dao.upsert_fmspc_tcb(tcb_info_obj).gas_price(U256::from_str_radix(&GAS_PRICE, 10).unwrap()).send()) {
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

    // QE/TDX Identity
    if all_verification_collateral == 1 {
        upsert_enclave_identity(
            prv_key,
            rpc_url,
            chain_id,
            enclave_id,
            collateral_version,
            enclave_identity_str,
            enclave_identity_issuer_chains_str,
        );
    }
}

pub async fn upsert_tcb_eval_data_number(
    prv_key: &str,
    rpc_url: String,
    chain_id: u64,
    gas_price: U256,
    tcb_eval_data_number_dao: &str,
    tcb_eval_data_number_str: &str,
) -> bool {
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));

    let tcb_eval_dao = TcbEvalDao::new(parse_address_from_str(tcb_eval_data_number_dao), signer.clone());

    let tcb_eval_data_number: TcbEvalDataNumber = serde_json::from_str(tcb_eval_data_number_str).unwrap();
    let tcb_eval_data_number_str = &tcb_eval_data_number_str[r#""tcbEvaluationDataNumbers":{"#.len()..];
    let end_idx = tcb_eval_data_number_str.find(r#","signature""#).unwrap();
    let tcb_eval_data_number_str = &tcb_eval_data_number_str[..end_idx];
    let tcb_eval_data_number_obj = TcbEvalJsonObj {
        tcb_evaluation_data_numbers: tcb_eval_data_number_str.to_string(),
        signature: Bytes::from_hex(&tcb_eval_data_number.signature).unwrap(),
    };
    println!("tcb_evaluation_data_numbers = {}", tcb_eval_data_number_obj.tcb_evaluation_data_numbers);
    println!("signature = {}", tcb_eval_data_number_obj.signature);
    match tcb_eval_dao
            .upsert_tcb_evaluation_data(tcb_eval_data_number_obj)
            .gas_price(gas_price).send().await
    {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_tcb_evaluation_data] hash: {:?}",
                pending_tx.tx_hash()
            );
            match timeout(TX_CONFIRMATION_TIMEOUT, pending_tx).await {
                Ok(Ok(receipt)) => {
                    println!("txn[upsert_tcb_evaluation_data] receipt: {:?}", receipt);
                    return true;
                }
                Ok(Err(err)) => {
                    println!("txn[upsert_tcb_evaluation_data] receipt meet error: {:?}", err);
                    return false;
                }
                Err(_) => {
                    println!("txn[upsert_tcb_evaluation_data] timeout waiting for confirmation after {:?}", TX_CONFIRMATION_TIMEOUT);
                    return false;
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_tcb_evaluation_data] meet error: {:?}", err);
            return false;
        }
    }
}
