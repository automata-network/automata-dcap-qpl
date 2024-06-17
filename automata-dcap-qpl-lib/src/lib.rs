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
use libloading::{Library, Symbol};
use openssl::x509::X509;
use std::ffi::{c_char, CStr, CString};
use std::mem::ManuallyDrop;
use std::sync::Arc;

const COLLATERAL_VERSION_ENV: &str = "AUTOMATA_DCAP_COLLATERAL_VERSION";

pub fn into_raw_parts<T>(vec: Vec<T>) -> (*mut T, usize, usize) {
    let mut vec = ManuallyDrop::new(vec);
    let length = vec.len();
    let capacity = vec.capacity();
    (vec.as_mut_ptr(), length, capacity)
}

// quote3_error_t sgx_ql_get_quote_config(const sgx_ql_pck_cert_id_t *p_pck_cert_id, sgx_ql_config_t **pp_quote_config);
#[no_mangle]
pub extern "C" fn sgx_ql_get_quote_config(
    p_pck_cert_id: *const SgxQlPckCertId,
    pp_quote_config: *mut *mut SgxQlConfig,
) -> Quote3Error {
    let pck_cert_id = unsafe { *p_pck_cert_id };

    println!("[Automata DCAP QPL] Call sgx_ql_get_quote_config");
    println!("[Automata DCAP QPL] pck_cert_id: {:?}", pck_cert_id);

    println!("[Automata DCAP QPL] cpu_svn: {:?}", unsafe {
        *pck_cert_id.p_platform_cpu_svn
    });
    println!("[Automata DCAP QPL] pce_svn: {:?}", unsafe {
        *pck_cert_id.p_platform_pce_isv_svn
    });

    let cpu_svn = unsafe { *pck_cert_id.p_platform_cpu_svn };
    let pce_svn = unsafe { *pck_cert_id.p_platform_pce_isv_svn };

    println!(
        "[Automata DCAP QPL] cpu_svn: {:?}",
        hex::encode(cpu_svn.cpu_svn)
    );
    println!(
        "[Automata DCAP QPL] pce_svn: {:?}",
        hex::encode(pce_svn.isv_svn.to_le_bytes())
    );
    println!(
        "[Automata DCAP QPL] pce_id: {:?}",
        hex::encode(pck_cert_id.pce_id.to_le_bytes())
    );
    let qe_id = unsafe {
        std::slice::from_raw_parts(pck_cert_id.p_qe3_id, pck_cert_id.qe3_id_size as usize)
    };
    println!("[Automata DCAP QPL] qe_id: {:?}", hex::encode(qe_id));
    let encrypted_ppid = unsafe {
        std::slice::from_raw_parts(
            pck_cert_id.p_encrypted_ppid,
            pck_cert_id.encrypted_ppid_size as usize,
        )
    };
    println!(
        "[Automata DCAP QPL] encrypted_ppid: {:?}",
        hex::encode(encrypted_ppid)
    );

    let qe_id_str = hex::encode(qe_id);
    let cpu_svn_str = hex::encode(unsafe { *pck_cert_id.p_platform_cpu_svn }.cpu_svn);
    let pce_svn_str = hex::encode(
        unsafe { *pck_cert_id.p_platform_pce_isv_svn }
            .isv_svn
            .to_le_bytes(),
    );
    let pce_id_str = hex::encode(pck_cert_id.pce_id.to_le_bytes());

    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let client = Arc::new(provider);
    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, client.clone());
    let pck_dao_address = PCK_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pck_dao = PckDao::new(pck_dao_address, client.clone());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let tcbm = match rt.block_on(
        pck_dao
            .get_platform_tcb_by_id_and_svns(
                qe_id_str.clone(),
                pce_id_str.clone(),
                cpu_svn_str.clone(),
                pce_svn_str.clone(),
            )
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_platform_tcb_by_id_and_svns func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    if tcbm.len() == 0 {
        println!("[Automata DCAP QPL] cannot find tcbm on chain, fallback to Azure DCAP lib");
        return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
            p_pck_cert_id,
            pp_quote_config,
        );
    }
    let leaf_cert: Bytes = match rt.block_on(
        pck_dao
            .get_cert(qe_id_str, cpu_svn_str, pce_svn_str, pce_id_str)
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!(
                "[Automata DCAP QPL] get_cert func meets error: {:?}, fallback to Azure DCAP lib",
                err
            );
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    if leaf_cert.len() == 0 {
        println!("[Automata DCAP QPL] cannot find leaf cert on chain, fallback to Azure DCAP lib");
        return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
            p_pck_cert_id,
            pp_quote_config,
        );
    }
    let leaf_cert_str = leaf_cert.to_string();
    let leaf_cert_slices = hex::decode(&leaf_cert_str.trim_start_matches("0x")).unwrap();
    let (mut cert, issuer) = match X509::from_der(leaf_cert_slices.as_slice()) {
        Ok(c) => {
            let issuer = format!("{:?}", c.issuer_name());
            (c.to_pem().unwrap(), issuer)
        }
        Err(err) => {
            println!("[Automata DCAP QPL] meets error when decoding the cert result from chain: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    let mut certs = Vec::new();
    certs.append(&mut cert);

    let ca_id = if issuer.contains("Platform") {
        CAID::Platform as u8
    } else {
        CAID::Processor as u8
    };
    let (intermediate_cert, _) = match rt.block_on(pcs_dao.get_certificate_by_id(ca_id).call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_certificate_by_id func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    if intermediate_cert.len() == 0 {
        println!("[Automata DCAP QPL] cannot find intermediate cert on chain, fallback to Azure DCAP lib");
        return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
            p_pck_cert_id,
            pp_quote_config,
        );
    }
    let intermediate_cert_str = intermediate_cert.to_string();
    let intermediate_cert_slices =
        hex::decode(&intermediate_cert_str.trim_start_matches("0x")).unwrap();
    let mut cert = match X509::from_der(intermediate_cert_slices.as_slice()) {
        Ok(c) => c.to_pem().unwrap(),
        Err(err) => {
            println!("[Automata DCAP QPL] meets error when decoding the cert result from chain: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    certs.append(&mut cert);

    let (root_cert, _) = match rt.block_on(pcs_dao.get_certificate_by_id(CAID::Root as u8).call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_certificate_by_id func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    if root_cert.len() == 0 {
        println!("[Automata DCAP QPL] cannot find root cert on chain, fallback to Azure DCAP lib");
        return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
            p_pck_cert_id,
            pp_quote_config,
        );
    }
    let root_cert_str = root_cert.to_string();
    let root_cert_slices = hex::decode(&root_cert_str.trim_start_matches("0x")).unwrap();
    let mut cert = match X509::from_der(root_cert_slices.as_slice()) {
        Ok(c) => c.to_pem().unwrap(),
        Err(err) => {
            println!("[Automata DCAP QPL] meets error when decoding the cert result from chain: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
                p_pck_cert_id,
                pp_quote_config,
            );
        }
    };
    certs.append(&mut cert);
    let mut ending = vec![0u8];
    certs.append(&mut ending);

    certs.truncate(certs.len());
    let (certs, cert_len, _) = into_raw_parts(certs);

    let tcbm = hex::decode(tcbm.trim_start_matches("0x")).unwrap();
    assert!(tcbm.len() == 18);
    let tcbm_cpu_svn_slices = &tcbm[0..16];
    let tcbm_cpu_svn: [u8; 16] = tcbm_cpu_svn_slices.try_into().unwrap();
    let tcbm_pce_svn = tcbm[17] as u16 * 16 + tcbm[16] as u16;
    println!(
        "[Automata DCAP QPL] tcbm.cpu_svn: {:?}",
        hex::encode(tcbm_cpu_svn)
    );
    println!(
        "[Automata DCAP QPL] tcbm.pce_svn: {:?}",
        hex::encode(tcbm_pce_svn.to_le_bytes())
    );
    let quote_config = SgxQlConfig::new(tcbm_cpu_svn, tcbm_pce_svn, certs, cert_len);
    unsafe {
        *pp_quote_config = Box::into_raw(Box::new(quote_config));
    }
    let quote_config = unsafe { *pp_quote_config };
    println!("[Automata DCAP QPL] quote_config: {:?}", quote_config);
    let cert_data = unsafe {
        std::slice::from_raw_parts(
            (*quote_config).cert_data,
            (*quote_config).cert_data_size as usize,
        )
    };
    println!("[Automata DCAP QPL] cert len: {}", unsafe {
        (*quote_config).cert_data_size as usize
    });
    println!("[Automata DCAP QPL] cert: {:?}", cert_data);
    println!(
        "[Automata DCAP QPL] certdata: {}",
        std::str::from_utf8(cert_data).unwrap()
    );

    return Quote3Error::SgxQlSuccess;
}

#[no_mangle]
pub extern "C" fn sgx_ql_free_quote_config(quote_config: *mut SgxQlConfig) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_free_quote_config");
    // free quote_config
    unsafe {
        drop(Box::from_raw(quote_config));
    }
    return Quote3Error::SgxQlSuccess;
}

// quote3_error_t sgx_ql_get_quote_verification_collateral(
//    const uint8_t *fmspc,
//    const uint16_t fmspc_size,
//    const char *pck_ca,
//    sgx_ql_qve_collateral_t **pp_quote_collateral);
#[no_mangle]
pub extern "C" fn sgx_ql_get_quote_verification_collateral(
    fmspc: *const u8,
    fmspc_size: u16,
    pck_ca: *const c_char,
    pp_quote_collateral: *mut *mut SgxQlQveCollateral,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_get_quote_verification_collateral");
    return sgx_ql_fetch_quote_verification_collateral(
        SgxProdType::SgxProdTypeSgx,
        fmspc,
        fmspc_size,
        pck_ca,
        pp_quote_collateral,
    );
}

// quote3_error_t sgx_ql_free_quote_verification_collateral(
//    sgx_ql_qve_collateral_t *p_quote_collateral);
#[no_mangle]
pub extern "C" fn sgx_ql_free_quote_verification_collateral(
    p_quote_collateral: *mut SgxQlQveCollateral,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_free_quote_verification_collateral");
    unsafe {
        drop(Box::from_raw(p_quote_collateral));
    }
    return Quote3Error::SgxQlSuccess;
}

// quote3_error_t tdx_ql_get_quote_verification_collateral(
//    const uint8_t *fmspc,
//    const uint16_t fmspc_size,
//    const char *pck_ca,
//    sgx_ql_qve_collateral_t **pp_quote_collateral);
#[no_mangle]
pub extern "C" fn tdx_ql_get_quote_verification_collateral(
    fmspc: *const u8,
    fmspc_size: u16,
    pck_ca: *const c_char,
    pp_quote_collateral: *mut *mut SgxQlQveCollateral,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call tdx_ql_get_quote_verification_collateral");
    return sgx_ql_fetch_quote_verification_collateral(
        SgxProdType::SgxProdTypeTdx,
        fmspc,
        fmspc_size,
        pck_ca,
        pp_quote_collateral,
    );
}

// quote3_error_t tdx_ql_free_quote_verification_collateral(
//    sgx_ql_qve_collateral_t *p_quote_collateral);
#[no_mangle]
pub extern "C" fn tdx_ql_free_quote_verification_collateral(
    p_quote_collateral: *mut SgxQlQveCollateral,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call tdx_ql_free_quote_verification_collateral");
    unsafe {
        drop(Box::from_raw(p_quote_collateral));
    }
    return Quote3Error::SgxQlSuccess;
}

// quote3_error_t sgx_ql_get_qve_identity(
//    char **pp_qve_identity,
//    uint32_t *p_qve_identity_size,
//    char **pp_qve_identity_issuer_chain,
//    uint32_t *p_qve_identity_issuer_chain_size);
#[no_mangle]
pub extern "C" fn sgx_ql_get_qve_identity(
    pp_qve_identity: *mut *mut c_char,
    p_qve_identity_size: *mut u32,
    pp_qve_identity_issuer_chain: *mut *mut c_char,
    p_qve_identity_issuer_chain_size: *mut u32,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_get_qve_identity");
    if pp_qve_identity.is_null() {
        println!("[Automata DCAP QPL] Pointer to qve identity pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_qve_identity }.is_null() {
        println!("[Automata DCAP QPL] Qve identity pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if pp_qve_identity_issuer_chain.is_null() {
        println!("[Automata DCAP QPL] Pointer to issuer chain pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_qve_identity_issuer_chain }.is_null() {
        println!("[Automata DCAP QPL] Issuer chain pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    }

    let id = U256::from(EnclaveID::QVE as u32);
    let collateral_version = get_collateral_version();
    let version = U256::from(collateral_version);

    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let client = Arc::new(provider);
    let enclave_identity_dao_address = ENCLAVE_IDENTITY_DAO_PORTAL_CONTRACT_ADDRESS
        .parse::<Address>()
        .unwrap();
    let enclave_identity_dao = EnclaveIdentityDao::new(enclave_identity_dao_address, client);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let qve: EnclaveIdentityJsonObj = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity(id, version)
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_enclave_identity func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_sgx_ql_get_qve_identity(
                pp_qve_identity,
                p_qve_identity_size,
                pp_qve_identity_issuer_chain,
                p_qve_identity_issuer_chain_size,
            );
        }
    };
    if qve == EnclaveIdentityJsonObj::default() {
        println!(
            "[Automata DCAP QPL] cannot find qve identity on chain, fallback to Azure DCAP lib"
        );
        return fallback_to_az_dcap_call_sgx_ql_get_qve_identity(
            pp_qve_identity,
            p_qve_identity_size,
            pp_qve_identity_issuer_chain,
            p_qve_identity_issuer_chain_size,
        );
    }
    let issuer_chain: (Bytes, Bytes) = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity_issuer_chain()
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_enclave_identity_issuer_chain func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_sgx_ql_get_qve_identity(
                pp_qve_identity,
                p_qve_identity_size,
                pp_qve_identity_issuer_chain,
                p_qve_identity_issuer_chain_size,
            );
        }
    };
    if issuer_chain.0.len() == 0 || issuer_chain.1.len() == 0 {
        println!("[Automata DCAP QPL] cannot find enclave identity issuer chains on chain, fallback to Azure DCAP lib");
        return fallback_to_az_dcap_call_sgx_ql_get_qve_identity(
            pp_qve_identity,
            p_qve_identity_size,
            pp_qve_identity_issuer_chain,
            p_qve_identity_issuer_chain_size,
        );
    }

    let qve_identity_str = format!(
        "{{\"enclaveIdentity\":{},\"signature\":\"{}\"}}",
        qve.identity_str,
        qve.signature.to_string().trim_start_matches("0x")
    );
    let qve_identity_c_str = CString::new(qve_identity_str).unwrap();
    let qve_identity_len = qve_identity_c_str.as_bytes_with_nul().len();
    let qve_identity_issuer_chain_str = format!(
        "{}{}",
        issuer_chain.0.to_string(),
        issuer_chain.1.to_string()
    );
    let qve_identity_issuer_chain_c_str = CString::new(qve_identity_issuer_chain_str).unwrap();
    let qve_identity_issuer_chain_len = qve_identity_issuer_chain_c_str.as_bytes_with_nul().len();
    unsafe {
        *p_qve_identity_size = qve_identity_len as u32;
        *p_qve_identity_issuer_chain_size = qve_identity_issuer_chain_len as u32;
        *pp_qve_identity = qve_identity_c_str.into_raw();
        *pp_qve_identity_issuer_chain = qve_identity_issuer_chain_c_str.into_raw();
    }
    return Quote3Error::SgxQlSuccess;
}

// quote3_error_t sgx_ql_free_qve_identity(
//    char *p_qve_identity,
//    char *p_qve_identity_issuer_chain);
#[no_mangle]
pub extern "C" fn sgx_ql_free_qve_identity(
    p_qve_identity: *mut c_char,
    p_qve_identity_issuer_chain: *mut c_char,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_free_qve_identity");
    unsafe {
        let _ = CStr::from_ptr(p_qve_identity);
        let _ = CStr::from_ptr(p_qve_identity_issuer_chain);
        *p_qve_identity = 0;
        *p_qve_identity_issuer_chain = 0;
    }
    return Quote3Error::SgxQlSuccess;
}

// quote3_error_t sgx_ql_get_root_ca_crl (
//    char **pp_root_ca_crl,
//    uint16_t *p_root_ca_crl_size);
#[no_mangle]
pub extern "C" fn sgx_ql_get_root_ca_crl(
    pp_root_ca_crl: *mut *mut c_char,
    p_root_ca_crl_size: *mut u16,
) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_get_root_ca_crl");
    if pp_root_ca_crl.is_null() {
        println!("[Automata DCAP QPL] Pointer to crl pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_root_ca_crl }.is_null() {
        println!("[Automata DCAP QPL] Crl pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    }

    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let client = Arc::new(provider);
    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, client);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let ca_id = CAID::Root as u8;
    let (_, root_ca_crl_bytes) = match rt.block_on(pcs_dao.get_certificate_by_id(ca_id).call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_certificate_by_id func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_sgx_ql_get_root_ca_crl(
                pp_root_ca_crl,
                p_root_ca_crl_size,
            );
        }
    };
    if root_ca_crl_bytes.len() == 0 {
        println!(
            "[Automata DCAP QPL] cannot find root ca crl on chain, fallback to Azure DCAP lib"
        );
        return fallback_to_az_dcap_call_sgx_ql_get_root_ca_crl(pp_root_ca_crl, p_root_ca_crl_size);
    }

    let root_ca_crl_c_str = CString::new(root_ca_crl_bytes.to_string()).unwrap();
    let root_ca_crl_len = root_ca_crl_c_str.as_bytes_with_nul().len();
    unsafe {
        *p_root_ca_crl_size = root_ca_crl_len as u16;
        *pp_root_ca_crl = root_ca_crl_c_str.into_raw();
    }
    return Quote3Error::SgxQlSuccess;
}

// quote3_error_t sgx_ql_free_root_ca_crl (
//    char *p_root_ca_crl);
#[no_mangle]
pub extern "C" fn sgx_ql_free_root_ca_crl(p_root_ca_crl: *mut c_char) -> Quote3Error {
    println!("[Automata DCAP QPL] Call sgx_ql_free_root_ca_crl");
    unsafe {
        let _ = CStr::from_ptr(p_root_ca_crl);
        *p_root_ca_crl = 0;
    }
    return Quote3Error::SgxQlSuccess;
}

fn sgx_ql_fetch_quote_verification_collateral(
    sgx_prod_type: SgxProdType,
    fmspc: *const u8,
    fmspc_size: u16,
    pck_ca: *const c_char,
    pp_quote_collateral: *mut *mut SgxQlQveCollateral,
) -> Quote3Error {
    if fmspc.is_null() {
        println!("[Automata DCAP QPL] FMSPC is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if fmspc_size == 0 {
        println!("[Automata DCAP QPL] FMSPC buffer size is 0");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if pck_ca.is_null() {
        println!("[Automata DCAP QPL] PCK CA is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if pp_quote_collateral.is_null() {
        println!("[Automata DCAP QPL] Pointer to collateral pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_quote_collateral }.is_null() {
        println!("[Automata DCAP QPL] Collateral pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    }

    let pck_ca_cstr = unsafe { CStr::from_ptr(pck_ca) };
    let pck_ca_string: String = pck_ca_cstr.to_string_lossy().into_owned();
    if pck_ca_string != "processor".to_string() && pck_ca_string != "platform".to_string() {
        println!(
            "[Automata DCAP QPL] pck_ca must be either processor or platform, but the value is {:?}",
            pck_ca_string
        );
        return Quote3Error::SgxQlErrorInvalidParameter;
    }
    println!("[Automata DCAP QPL] pck_ca: {:?}", pck_ca_string);
    let fmspc_u8_vec = unsafe { std::slice::from_raw_parts(fmspc, fmspc_size as usize) };
    let fmspc_string = String::from_utf8_lossy(fmspc_u8_vec).into_owned();
    println!("[Automata DCAP QPL] fmspc: {:?}", fmspc_string);

    // Get PCK CRL
    let provider = Provider::<Http>::try_from(VERAX_RPC_URL).unwrap();
    let client = Arc::new(provider);
    let pcs_dao_address = PCS_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pcs_dao = PcsDao::new(pcs_dao_address, client.clone());
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    // https://github.com/automata-network/automata-on-chain-pccs/blob/main/src/Common.sol#L4-L9
    let ca_id = if pck_ca_string == "processor".to_string() {
        CAID::Processor as u8
    } else {
        CAID::Platform as u8
    };
    let (_, pck_crl_bytes) = match rt.block_on(pcs_dao.get_certificate_by_id(ca_id).call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_certificate_by_id func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let pck_crl_c_str = CString::new(pck_crl_bytes.to_string()).unwrap();
    let pck_dao_address = PCK_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pck_dao = PckDao::new(pck_dao_address, client.clone());
    let pck_cert_chains = match rt.block_on(pck_dao.get_pck_cert_chain(ca_id).call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_pck_cert_chain func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let pck_crl_issuer_chain_str = format!(
        "{}{}",
        pck_cert_chains.0.to_string(),
        pck_cert_chains.1.to_string()
    );
    let pck_crl_issuer_chain_c_str = CString::new(pck_crl_issuer_chain_str).unwrap();

    // Get Root CA CRL
    let ca_id = CAID::Root as u8;
    let (_, root_ca_crl_bytes) = match rt.block_on(pcs_dao.get_certificate_by_id(ca_id).call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_certificate_by_id func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let root_ca_crl_c_str = CString::new(root_ca_crl_bytes.to_string()).unwrap();

    // Get Tcb Info & Issuer Chain
    // Input: sgx_prod_type, fmspc_string
    // Output: tcb_info, tcb_info_issuer_chain
    let fmspc_tcb_dao_address = FMSPC_TCB_DAO_PORTAL_CONTRACT_ADDRESS
        .parse::<Address>()
        .unwrap();
    let fmspc_tcb_dao = FmspcTcbDao::new(fmspc_tcb_dao_address, client.clone());
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let tcb_type = U256::from(sgx_prod_type as u32);
    let collateral_version = get_collateral_version();
    let version = U256::from(collateral_version);
    let tcb_info: TcbInfoJsonObj = match rt.block_on(
        fmspc_tcb_dao
            .get_tcb_info(tcb_type, fmspc_string, version)
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_tcb_info func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let tcb_info_str = format!(
        "{{\"tcbInfo\":{},\"signature\":\"{}\"}}",
        tcb_info.tcb_info_str,
        tcb_info.signature.to_string().trim_start_matches("0x")
    );
    let tcb_info_c_str = CString::new(tcb_info_str).unwrap();
    let tcb_info_issuer_chains = match rt.block_on(fmspc_tcb_dao.get_tcb_issuer_chain().call()) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_tcb_issuer_chain func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let tcb_info_issuer_chain_str = format!(
        "{}{}",
        tcb_info_issuer_chains.0.to_string(),
        tcb_info_issuer_chains.1.to_string()
    );
    let tcb_info_issuer_chain_c_str = CString::new(tcb_info_issuer_chain_str).unwrap();

    // Get QE Identity & Issuer Chain
    // Input: sgx_prod_type, api_version
    // Output qe_identity, qe_identity_issuer_chain
    let enclave_identity_dao_address = ENCLAVE_IDENTITY_DAO_PORTAL_CONTRACT_ADDRESS
        .parse::<Address>()
        .unwrap();
    let enclave_identity_dao =
        EnclaveIdentityDao::new(enclave_identity_dao_address, client.clone());
    let id = if sgx_prod_type == SgxProdType::SgxProdTypeSgx {
        U256::from(EnclaveID::QE as u32)
    } else {
        U256::from(EnclaveID::TD_QE as u32)
    };
    let collateral_version = get_collateral_version();
    let version = U256::from(collateral_version);
    let qe: EnclaveIdentityJsonObj = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity(id, version)
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_enclave_identity func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let issuer_chain: (Bytes, Bytes) = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity_issuer_chain()
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("[Automata DCAP QPL] get_enclave_identity_issuer_chain func meets error: {:?}, fallback to Azure DCAP lib", err);
            return fallback_to_az_dcap_call_get_quote_verification_collateral(
                sgx_prod_type,
                fmspc,
                fmspc_size,
                pck_ca,
                pp_quote_collateral,
            );
        }
    };
    let qe_identity_str = format!(
        "{{\"enclaveIdentity\":{},\"signature\":\"{}\"}}",
        qe.identity_str,
        qe.signature.to_string().trim_start_matches("0x")
    );
    let qe_identity_c_str = CString::new(qe_identity_str).unwrap();
    let qe_identity_issuer_chain_str = format!(
        "{}{}",
        issuer_chain.0.to_string(),
        issuer_chain.1.to_string()
    );
    let qe_identity_issuer_chain_c_str = CString::new(qe_identity_issuer_chain_str).unwrap();

    let (version, tee_type) = if sgx_prod_type == SgxProdType::SgxProdTypeTdx {
        let v = SgxQlQveCollateralVersions {
            major_version: 4,
            minor_version: 0,
        };
        (SgxQlQveCollateralVersion { versions: v }, 0x81)
    } else {
        (SgxQlQveCollateralVersion { version: 1 }, 0x0)
    };
    let quote_collateral = SgxQlQveCollateral::new(
        version,
        tee_type,
        pck_crl_issuer_chain_c_str,
        root_ca_crl_c_str,
        pck_crl_c_str,
        tcb_info_issuer_chain_c_str,
        tcb_info_c_str,
        qe_identity_issuer_chain_c_str,
        qe_identity_c_str,
    );
    unsafe {
        *pp_quote_collateral = Box::into_raw(Box::new(quote_collateral));
    }
    let quote_collateral = unsafe { *pp_quote_collateral };
    println!(
        "[Automata DCAP QPL] quote_collateral: {:?}",
        quote_collateral
    );

    return Quote3Error::SgxQlSuccess;
}

fn get_collateral_version() -> u32 {
    match std::env::var(COLLATERAL_VERSION_ENV) {
        Ok(v) => {
            if v == "v1" || v == "V1" || v == "1" {
                1
            } else if v == "v2" || v == "V2" || v == "2" {
                2
            } else if v == "v3" || v == "V3" || v == "3" {
                3
            } else if v == "v4" || v == "V4" || v == "4" {
                4
            } else {
                3 // use v3 as default dcap attestation version
            }
        }
        Err(_) => {
            3 // use v3 as default dcap attestation version
        }
    }
}

fn az_dcap_call_sgx_ql_get_quote_config(
    p_pck_cert_id: *const SgxQlPckCertId,
    pp_quote_config: *mut *mut SgxQlConfig,
) -> (Quote3Error, bool) {
    std::env::set_var("AZDCAP_DEBUG_LOG_LEVEL", "info");
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION",
        format!("v{}", get_collateral_version()),
    );
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION_TDX",
        format!("v{}", get_collateral_version()),
    );
    unsafe {
        let lib = match Library::new("libdcap_az_client.so") {
            Ok(l) => l,
            Err(_) => {
                println!(
                    "[Automata DCAP QPL] fail to load libdcap_az_client.so, fallback to Intel PCS"
                );
                // TODO fallback
                return (Quote3Error::SgxQlErrorUnexpected, false);
            }
        };
        let func: Symbol<
            unsafe extern "C" fn(*const SgxQlPckCertId, *mut *mut SgxQlConfig) -> Quote3Error,
        > = lib.get(b"sgx_ql_get_quote_config").unwrap();
        (func(p_pck_cert_id, pp_quote_config), true)
    }
}

fn az_dcap_call_sgx_ql_free_quote_config(p_quote_config: *mut SgxQlConfig) -> Quote3Error {
    unsafe {
        let lib = Library::new("libdcap_az_client.so").unwrap();
        let func: Symbol<unsafe extern "C" fn(*mut SgxQlConfig) -> Quote3Error> =
            lib.get(b"sgx_ql_free_quote_config").unwrap();
        func(p_quote_config)
    }
}

fn az_dcap_call_sgx_ql_get_qve_identity(
    pp_qve_identity: *mut *mut c_char,
    p_qve_identity_size: *mut u32,
    pp_qve_identity_issuer_chain: *mut *mut c_char,
    p_qve_identity_issuer_chain_size: *mut u32,
) -> (Quote3Error, bool) {
    std::env::set_var("AZDCAP_DEBUG_LOG_LEVEL", "info");
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION",
        format!("v{}", get_collateral_version()),
    );
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION_TDX",
        format!("v{}", get_collateral_version()),
    );
    unsafe {
        let lib = match Library::new("libdcap_az_client.so") {
            Ok(l) => l,
            Err(_) => {
                println!(
                    "[Automata DCAP QPL] fail to load libdcap_az_client.so, fallback to Intel PCS"
                );
                // TODO fallback
                return (Quote3Error::SgxQlErrorUnexpected, false);
            }
        };
        let func: Symbol<
            unsafe extern "C" fn(
                *mut *mut c_char,
                *mut u32,
                *mut *mut c_char,
                *mut u32,
            ) -> Quote3Error,
        > = lib.get(b"sgx_ql_get_qve_identity").unwrap();
        (
            func(
                pp_qve_identity,
                p_qve_identity_size,
                pp_qve_identity_issuer_chain,
                p_qve_identity_issuer_chain_size,
            ),
            true,
        )
    }
}

fn az_dcap_call_sgx_ql_free_qve_identity(
    p_qve_identity: *mut c_char,
    p_qve_identity_issuer_chain: *mut c_char,
) -> Quote3Error {
    unsafe {
        let lib = Library::new("libdcap_az_client.so").unwrap();
        let func: Symbol<unsafe extern "C" fn(*mut c_char, *mut c_char) -> Quote3Error> =
            lib.get(b"sgx_ql_free_qve_identity").unwrap();
        func(p_qve_identity, p_qve_identity_issuer_chain)
    }
}

fn az_dcap_call_get_quote_verification_collateral(
    sgx_prod_type: SgxProdType,
    fmspc: *const u8,
    fmspc_size: u16,
    pck_ca: *const c_char,
    pp_quote_collateral: *mut *mut SgxQlQveCollateral,
) -> (Quote3Error, bool) {
    std::env::set_var("AZDCAP_DEBUG_LOG_LEVEL", "info");
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION",
        format!("v{}", get_collateral_version()),
    );
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION_TDX",
        format!("v{}", get_collateral_version()),
    );
    let symbol_name = if sgx_prod_type == SgxProdType::SgxProdTypeSgx {
        b"sgx_ql_get_quote_verification_collateral"
    } else {
        b"tdx_ql_get_quote_verification_collateral"
    };
    unsafe {
        let lib = match Library::new("libdcap_az_client.so") {
            Ok(l) => l,
            Err(_) => {
                println!(
                    "[Automata DCAP QPL] fail to load libdcap_az_client.so, fallback to Intel PCS"
                );
                // TODO fallback
                return (Quote3Error::SgxQlErrorUnexpected, false);
            }
        };
        let func: Symbol<
            unsafe extern "C" fn(
                *const u8,
                u16,
                *const c_char,
                *mut *mut SgxQlQveCollateral,
            ) -> Quote3Error,
        > = lib.get(symbol_name).unwrap();
        (func(fmspc, fmspc_size, pck_ca, pp_quote_collateral), true)
    }
}

fn az_dcap_call_free_quote_verification_collateral(
    sgx_prod_type: SgxProdType,
    p_quote_collateral: *mut SgxQlQveCollateral,
) -> Quote3Error {
    let symbol_name = if sgx_prod_type == SgxProdType::SgxProdTypeSgx {
        b"sgx_ql_free_quote_verification_collateral"
    } else {
        b"tdx_ql_free_quote_verification_collateral"
    };
    unsafe {
        let lib = Library::new("libdcap_az_client.so").unwrap();
        let func: Symbol<unsafe extern "C" fn(*mut SgxQlQveCollateral) -> Quote3Error> =
            lib.get(symbol_name).unwrap();
        func(p_quote_collateral)
    }
}

fn az_dcap_call_sgx_ql_get_root_ca_crl(
    pp_root_ca_crl: *mut *mut c_char,
    p_root_ca_crl_size: *mut u16,
) -> (Quote3Error, bool) {
    std::env::set_var("AZDCAP_DEBUG_LOG_LEVEL", "info");
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION",
        format!("v{}", get_collateral_version()),
    );
    std::env::set_var(
        "AZDCAP_COLLATERAL_VERSION_TDX",
        format!("v{}", get_collateral_version()),
    );
    unsafe {
        let lib = match Library::new("libdcap_az_client.so") {
            Ok(l) => l,
            Err(_) => {
                println!(
                    "[Automata DCAP QPL] fail to load libdcap_az_client.so, fallback to Intel PCS"
                );
                // TODO fallback
                return (Quote3Error::SgxQlErrorUnexpected, false);
            }
        };
        let func: Symbol<unsafe extern "C" fn(*mut *mut c_char, *mut u16) -> Quote3Error> =
            lib.get(b"sgx_ql_get_root_ca_crl").unwrap();
        (func(pp_root_ca_crl, p_root_ca_crl_size), true)
    }
}

fn az_dcap_call_sgx_ql_free_root_ca_crl(p_root_ca_crl: *mut c_char) -> Quote3Error {
    unsafe {
        let lib = Library::new("libdcap_az_client.so").unwrap();
        let func: Symbol<unsafe extern "C" fn(*mut c_char) -> Quote3Error> =
            lib.get(b"sgx_ql_free_root_ca_crl").unwrap();
        func(p_root_ca_crl)
    }
}

fn fallback_to_az_dcap_az_dcap_call_sgx_ql_get_quote_config(
    p_pck_cert_id: *const SgxQlPckCertId,
    pp_quote_config: *mut *mut SgxQlConfig,
) -> Quote3Error {
    let mut az_sgx_ql_config: SgxQlConfig =
        SgxQlConfig::new([0_u8; 16], 0, std::ptr::null_mut(), 0);
    let mut p_az_sgx_ql_config: *mut SgxQlConfig = &mut az_sgx_ql_config as *mut SgxQlConfig;
    let pp_az_sgx_ql_config: *mut *mut SgxQlConfig =
        &mut p_az_sgx_ql_config as *mut *mut SgxQlConfig;
    let (ret, use_azure) = az_dcap_call_sgx_ql_get_quote_config(p_pck_cert_id, pp_az_sgx_ql_config);
    unsafe {
        let new_cert = std::slice::from_raw_parts(
            (**pp_az_sgx_ql_config).cert_data,
            (**pp_az_sgx_ql_config).cert_data_size as usize,
        )
        .to_vec()
        .into_boxed_slice();
        let new_cert_ptr = Box::into_raw(new_cert) as *mut u8;
        let quote_config = SgxQlConfig::new(
            (**pp_az_sgx_ql_config).cert_cpu_svn.cpu_svn.clone(),
            (**pp_az_sgx_ql_config).cert_pce_isv_svn.isv_svn,
            new_cert_ptr,
            (**pp_az_sgx_ql_config).cert_data_size as usize,
        );
        *pp_quote_config = Box::into_raw(Box::new(quote_config));
    }
    let quote_config = unsafe { *pp_az_sgx_ql_config };
    if use_azure {
        az_dcap_call_sgx_ql_free_quote_config(quote_config);
    }
    let quote_config = unsafe { *pp_quote_config };
    unsafe { quote_config.as_ref().unwrap().print() };
    return ret;
}

fn fallback_to_az_dcap_call_sgx_ql_get_qve_identity(
    pp_qve_identity: *mut *mut c_char,
    p_qve_identity_size: *mut u32,
    pp_qve_identity_issuer_chain: *mut *mut c_char,
    p_qve_identity_issuer_chain_size: *mut u32,
) -> Quote3Error {
    let mut p_az_qve_identity: *mut c_char = std::ptr::null_mut();
    let pp_az_qve_identity: *mut *mut c_char = &mut p_az_qve_identity as *mut *mut c_char;
    let mut az_qve_identity_size: u32 = 0u32;
    let p_az_qve_identity_size: *mut u32 = &mut az_qve_identity_size;
    let mut p_az_qve_identity_issuer_chain: *mut c_char = std::ptr::null_mut();
    let pp_az_qve_identity_issuer_chain: *mut *mut c_char =
        &mut p_az_qve_identity_issuer_chain as *mut *mut c_char;
    let mut az_qve_identity_issuer_chain_size: u32 = 0u32;
    let p_az_qve_identity_issuer_chain_size: *mut u32 = &mut az_qve_identity_issuer_chain_size;
    let (ret, use_azure) = az_dcap_call_sgx_ql_get_qve_identity(
        pp_az_qve_identity,
        p_az_qve_identity_size,
        pp_az_qve_identity_issuer_chain,
        p_az_qve_identity_issuer_chain_size,
    );
    unsafe {
        *p_qve_identity_size = az_qve_identity_size;
        *p_qve_identity_issuer_chain_size = az_qve_identity_issuer_chain_size;

        let qve_identity = *pp_az_qve_identity;
        let qve_identity_c_str = CStr::from_bytes_with_nul(std::slice::from_raw_parts(
            qve_identity as *const u8,
            az_qve_identity_size as usize,
        ));
        let qve_identity_str = if let Ok(s) = qve_identity_c_str.map(|c| c.to_string_lossy()) {
            s.to_string()
        } else {
            println!("[Automata DCAP QPL] Unable to convert qve_identity, fallback to Intel PCS");
            // TODO fallback
            return Quote3Error::SgxQlErrorUnexpected;
        };

        let qve_identity_issuer_chain = *pp_az_qve_identity_issuer_chain;
        let qve_identity_issuer_chain_c_str =
            CStr::from_bytes_with_nul(std::slice::from_raw_parts(
                qve_identity_issuer_chain as *const u8,
                az_qve_identity_issuer_chain_size as usize,
            ));
        let issuer_chains_str = if let Ok(s) =
            qve_identity_issuer_chain_c_str.map(|c| c.to_string_lossy())
        {
            s.to_string()
        } else {
            println!("[Automata DCAP QPL] Unable to convert qve_identity_issuer_chain, fallback to Intel PCS");
            // TODO fallback
            return Quote3Error::SgxQlErrorUnexpected;
        };

        let qve_identity_c_str = CString::new(qve_identity_str).unwrap();
        let qve_identity_issuer_chain_c_str = CString::new(issuer_chains_str).unwrap();
        *pp_qve_identity = qve_identity_c_str.into_raw();
        *pp_qve_identity_issuer_chain = qve_identity_issuer_chain_c_str.into_raw();
    }

    if use_azure {
        az_dcap_call_sgx_ql_free_qve_identity(p_az_qve_identity, p_az_qve_identity_issuer_chain);
    }
    return ret;
}

fn fallback_to_az_dcap_call_get_quote_verification_collateral(
    sgx_prod_type: SgxProdType,
    fmspc: *const u8,
    fmspc_size: u16,
    pck_ca: *const c_char,
    pp_quote_collateral: *mut *mut SgxQlQveCollateral,
) -> Quote3Error {
    let mut p_az_quote_collateral: *mut SgxQlQveCollateral = std::ptr::null_mut();
    let pp_az_quote_collateral: *mut *mut SgxQlQveCollateral =
        &mut p_az_quote_collateral as *mut *mut SgxQlQveCollateral;
    let (ret, use_azure) = az_dcap_call_get_quote_verification_collateral(
        sgx_prod_type,
        fmspc,
        fmspc_size,
        pck_ca,
        pp_az_quote_collateral,
    );
    unsafe { p_az_quote_collateral.as_ref().unwrap().print() };
    unsafe {
        let pck_crl_issuer_chain = std::slice::from_raw_parts(
            (*p_az_quote_collateral).pck_crl_issuer_chain,
            (*p_az_quote_collateral).pck_crl_issuer_chain_size as usize,
        );
        let pck_crl_issuer_chain: Vec<u8> = pck_crl_issuer_chain
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let pck_crl_issuer_chain = CString::from_vec_with_nul_unchecked(pck_crl_issuer_chain);

        let root_ca_crl = std::slice::from_raw_parts(
            (*p_az_quote_collateral).root_ca_crl,
            (*p_az_quote_collateral).root_ca_crl_size as usize,
        );
        let root_ca_crl: Vec<u8> = root_ca_crl.to_vec().into_iter().map(|x| x as u8).collect();
        let root_ca_crl = CString::from_vec_with_nul_unchecked(root_ca_crl);

        let pck_crl = std::slice::from_raw_parts(
            (*p_az_quote_collateral).pck_crl,
            (*p_az_quote_collateral).pck_crl_size as usize,
        );
        let pck_crl: Vec<u8> = pck_crl.to_vec().into_iter().map(|x| x as u8).collect();
        let pck_crl = CString::from_vec_with_nul_unchecked(pck_crl);

        let tcb_info_issuer_chain = std::slice::from_raw_parts(
            (*p_az_quote_collateral).tcb_info_issuer_chain,
            (*p_az_quote_collateral).tcb_info_issuer_chain_size as usize,
        );
        let tcb_info_issuer_chain: Vec<u8> = tcb_info_issuer_chain
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let tcb_info_issuer_chain = CString::from_vec_with_nul_unchecked(tcb_info_issuer_chain);

        let tcb_info = std::slice::from_raw_parts(
            (*p_az_quote_collateral).tcb_info,
            (*p_az_quote_collateral).tcb_info_size as usize,
        );
        let tcb_info: Vec<u8> = tcb_info.to_vec().into_iter().map(|x| x as u8).collect();
        let tcb_info = CString::from_vec_with_nul_unchecked(tcb_info);

        let qe_identity_issuer_chain = std::slice::from_raw_parts(
            (*p_az_quote_collateral).qe_identity_issuer_chain,
            (*p_az_quote_collateral).qe_identity_issuer_chain_size as usize,
        );
        let qe_identity_issuer_chain: Vec<u8> = qe_identity_issuer_chain
            .to_vec()
            .into_iter()
            .map(|x| x as u8)
            .collect();
        let qe_identity_issuer_chain =
            CString::from_vec_with_nul_unchecked(qe_identity_issuer_chain);

        let qe_identity = std::slice::from_raw_parts(
            (*p_az_quote_collateral).qe_identity,
            (*p_az_quote_collateral).qe_identity_size as usize,
        );
        let qe_identity: Vec<u8> = qe_identity.to_vec().into_iter().map(|x| x as u8).collect();
        let qe_identity = CString::from_vec_with_nul_unchecked(qe_identity);

        let quote_collateral = SgxQlQveCollateral::new(
            (*p_az_quote_collateral).version.clone(),
            (*p_az_quote_collateral).tee_type,
            pck_crl_issuer_chain,
            root_ca_crl,
            pck_crl,
            tcb_info_issuer_chain,
            tcb_info,
            qe_identity_issuer_chain,
            qe_identity,
        );

        *pp_quote_collateral = Box::into_raw(Box::new(quote_collateral));
    }
    if use_azure {
        az_dcap_call_free_quote_verification_collateral(sgx_prod_type, p_az_quote_collateral);
    }
    unsafe { (*pp_quote_collateral).as_ref().unwrap().print() };
    return ret;
}

fn fallback_to_az_dcap_call_sgx_ql_get_root_ca_crl(
    pp_root_ca_crl: *mut *mut c_char,
    p_root_ca_crl_size: *mut u16,
) -> Quote3Error {
    let mut p_az_root_ca_crl: *mut c_char = std::ptr::null_mut();
    let pp_az_root_ca_crl: *mut *mut c_char = &mut p_az_root_ca_crl as *mut *mut c_char;
    let mut az_root_ca_crl_size: u16 = 0;
    let p_az_root_ca_crl_size: *mut u16 = &mut az_root_ca_crl_size;
    let (ret, use_azure) =
        az_dcap_call_sgx_ql_get_root_ca_crl(pp_az_root_ca_crl, p_az_root_ca_crl_size);
    unsafe {
        *p_root_ca_crl_size = az_root_ca_crl_size;

        let root_ca_crl = *pp_az_root_ca_crl;
        let root_ca_crl_c_str = CStr::from_bytes_with_nul(std::slice::from_raw_parts(
            root_ca_crl as *const u8,
            az_root_ca_crl_size as usize,
        ));
        let root_ca_crl_str = if let Ok(s) = root_ca_crl_c_str.map(|c| c.to_string_lossy()) {
            s.to_string()
        } else {
            println!("[Automata DCAP QPL] Unable to convert root_ca_crl, fallback to Intel PCS");
            // TODO fallback
            return Quote3Error::SgxQlErrorUnexpected;
        };
        let root_ca_crl_c_str = CString::new(root_ca_crl_str).unwrap();
        *pp_root_ca_crl = root_ca_crl_c_str.into_raw();
    }
    if use_azure {
        az_dcap_call_sgx_ql_free_root_ca_crl(p_az_root_ca_crl);
    }
    return ret;
}
