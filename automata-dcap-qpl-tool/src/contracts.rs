use crate::pccs_types::*;
use automata_dcap_qpl_common::*;
use automata_dcap_qpl_contracts::{
    enclave_identity_dao::{EnclaveIdentityDao, EnclaveIdentityJsonObj},
    fmspc_tcb_dao::{FmspcTcbDao, TcbInfoJsonObj},
    pck_dao::PckDao,
    pcs_dao::PcsDao,
    ENCLAVE_IDENTITY_DAO_PORTAL_CONTRACT_ADDRESS, FMSPC_TCB_DAO_PORTAL_CONTRACT_ADDRESS,
    PCK_DAO_PORTAL_CONTRACT_ADDRESS, PCS_DAO_PORTAL_CONTRACT_ADDRESS, VERAX_RPC_URL,
};
use ethers::prelude::*;
use hex::FromHex;
use openssl::x509::{X509Crl, X509};
use std::{str::FromStr, sync::Arc};

pub fn upsert_pck_cert(
    prv_key: &str,
    chain_id: u64,
    ca: CAID,
    qe_id: String,
    pce_id: String,
    cpu_svn: String,
    pce_svn: String,
    tcbm: String,
    cert_chains_str: &str,
) {
    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pck_dao_address = PCK_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pck_dao = PckDao::new(pck_dao_address, signer.clone());
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

    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, signer.clone());
    // TODO: Check the Root and Platform/Process intermediate certs before upsert
    match rt.block_on(
        pcs_dao
            .upsert_pcs_certificates(CAID::Root as u8, Bytes::from_str(&certs[2]).unwrap())
            .send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][root] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_pcs_certificates][root] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!(
                        "txn[upsert_pcs_certificates][root] receipt meet error: {:?}",
                        err
                    );
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
            .send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][intermediate] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!(
                        "txn[upsert_pcs_certificates][intermediate] receipt: {:?}",
                        receipt
                    );
                }
                Err(err) => {
                    println!(
                        "txn[upsert_pcs_certificates][intermediate] receipt meet error: {:?}",
                        err
                    );
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
            .send(),
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_pck_cert] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_pck_cert] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!("txn[upsert_pck_cert] receipt meet error: {:?}", err);
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
            .send()
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_platform_tcbs] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_platform_tcbs] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!("txn[upsert_platform_tcbs] receipt meet error: {:?}", err);
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
    chain_id: u64,
    enclave_id: EnclaveID,
    enclave_identity_str: &str,
    enclave_identity_issuer_chains_str: &str,
) {
    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, signer.clone());
    let enclave_identity_dao_address = ENCLAVE_IDENTITY_DAO_PORTAL_CONTRACT_ADDRESS
        .parse::<Address>()
        .unwrap();
    let enclave_identity_dao =
        EnclaveIdentityDao::new(enclave_identity_dao_address, signer.clone());
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
            .send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][root] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_pcs_certificates][root] receipt: {:?}", receipt);
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    match rt.block_on(
        pcs_dao
            .upsert_pcs_certificates(CAID::Signing as u8, Bytes::from_str(&certs[0]).unwrap())
            .send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_pcs_certificates][signing] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!(
                        "txn[upsert_pcs_certificates][signing] receipt: {:?}",
                        receipt
                    );
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
    let id = U256::from(enclave_id as u32);
    let version = U256::from(2u32);
    let enclave_identity: EnclaveIdentity = serde_json::from_str(enclave_identity_str).unwrap();
    let enclave_identity_obj = EnclaveIdentityJsonObj {
        identity_str: serde_json::to_string(&enclave_identity.enclave_identity).unwrap(),
        signature: Bytes::from_hex(&enclave_identity.signature).unwrap(),
    };
    println!("identity_str = {}", enclave_identity_obj.identity_str);
    println!("signature = {}", enclave_identity_obj.signature);
    // println!("{:?}", enclave_identity_dao.upsert_enclave_identity(id, version, enclave_identity_obj));
    match rt.block_on(
        enclave_identity_dao
            .upsert_enclave_identity(id, version, enclave_identity_obj)
            .send(),
    ) {
        Ok(pending_tx) => {
            println!(
                "txn[upsert_enclave_identity] hash: {:?}",
                pending_tx.tx_hash()
            );
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_enclave_identity] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!("txn[upsert_enclave_identity] receipt meet error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_enclave_identity] meet error: {:?}", err);
        }
    };
}

pub fn upsert_root_ca_crl(prv_key: &str, chain_id: u64, crl: &str) {
    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, signer.clone());
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
            .send(),
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_root_ca_crl] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_root_ca_crl] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!("txn[upsert_root_ca_crl] receipt meet error: {:?}", err);
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
    chain_id: u64,
    root_ca_crl: &str,
    pck: CAID,
    pck_crl: &str,
    tcb_info_str: &str,
    enclave_id: EnclaveID,
    enclave_identity_str: &str,
    enclave_identity_issuer_chains_str: &str,
) {
    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let wallet = prv_key.parse::<LocalWallet>().unwrap();
    let signer = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(chain_id),
    ));
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, signer.clone());
    let fmspc_tcb_dao_address = FMSPC_TCB_DAO_PORTAL_CONTRACT_ADDRESS
        .parse::<Address>()
        .unwrap();
    let fmspc_tcb_dao = FmspcTcbDao::new(fmspc_tcb_dao_address, signer.clone());

    // Root CA CRL
    upsert_root_ca_crl(prv_key, chain_id, root_ca_crl);

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
            .send(),
    ) {
        Ok(pending_tx) => {
            println!("txn[upsert_pck_crl] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_pck_crl] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!("txn[upsert_pck_crl] receipt meet error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_pck_crl] meet error: {:?}", err);
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
    match rt.block_on(fmspc_tcb_dao.upsert_fmspc_tcb(tcb_info_obj).send()) {
        Ok(pending_tx) => {
            println!("txn[upsert_fmspc_tcb] hash: {:?}", pending_tx.tx_hash());
            match rt.block_on(pending_tx) {
                Ok(receipt) => {
                    println!("txn[upsert_fmspc_tcb] receipt: {:?}", receipt);
                }
                Err(err) => {
                    println!("txn[upsert_fmspc_tcb] receipt meet error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("txn[upsert_fmspc_tcb] meet error: {:?}", err);
        }
    }

    // QE/TDX Identity
    upsert_enclave_identity(
        prv_key,
        chain_id,
        enclave_id,
        enclave_identity_str,
        enclave_identity_issuer_chains_str,
    );
}
