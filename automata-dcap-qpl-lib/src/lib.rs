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
use openssl::x509::X509;
use std::ffi::{c_char, CStr, CString};
use std::mem::ManuallyDrop;
use std::sync::Arc;

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

    println!("Automata DCAP QPL: sgx_ql_get_quote_config");
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
    let leaf_cert: Bytes = match rt.block_on(
        pck_dao
            .get_cert(qe_id_str, pce_id_str, cpu_svn_str, pce_svn_str)
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("sgx_ql_get_quote_config meet error: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    if leaf_cert.len() == 0 {
        println!("cannot find leaf cert on chain, plz use tool to upload first");
        return Quote3Error::SgxQlErrorUnexpected;
    }
    let leaf_cert_str = leaf_cert.to_string();
    let leaf_cert_slices = hex::decode(&leaf_cert_str.trim_start_matches("0x")).unwrap();
    let (mut cert, issuer) = match X509::from_der(leaf_cert_slices.as_slice()) {
        Ok(c) => {
            let issuer = format!("{:?}", c.issuer_name());
            (c.to_pem().unwrap(), issuer)
        },
        Err(err) => {
            println!("meet error when decoding the cert result from chain: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    let mut certs = Vec::new();
    certs.append(&mut cert);

    let ca_id = if issuer.contains("Platform") {
        CAID::Platform as u8
    } else {
        CAID::Processor as u8
    };
    let (intermediate_cert, _) = match rt.block_on(
        pcs_dao.get_certificate_by_id(ca_id).call()
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("sgx_ql_get_quote_config meet error: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    if intermediate_cert.len() == 0 {
        println!("cannot find intermediate cert on chain, plz use tool to upload first");
        return Quote3Error::SgxQlErrorUnexpected;
    }
    let intermediate_cert_str = intermediate_cert.to_string();
    let intermediate_cert_slices = hex::decode(&intermediate_cert_str.trim_start_matches("0x")).unwrap();
    let mut cert = match X509::from_der(intermediate_cert_slices.as_slice()) {
        Ok(c) => {
            c.to_pem().unwrap()
        },
        Err(err) => {
            println!("meet error when decoding the cert result from chain: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    certs.append(&mut cert);

    let (root_cert, _) = match rt.block_on(
        pcs_dao.get_certificate_by_id(CAID::Root as u8).call()
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("sgx_ql_get_quote_config meet error: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    if root_cert.len() == 0 {
        println!("cannot find root cert on chain, plz use tool to upload first");
        return Quote3Error::SgxQlErrorUnexpected;
    }
    let root_cert_str = root_cert.to_string();
    let root_cert_slices = hex::decode(&root_cert_str.trim_start_matches("0x")).unwrap();
    let mut cert = match X509::from_der(root_cert_slices.as_slice()) {
        Ok(c) => {
            c.to_pem().unwrap()
        },
        Err(err) => {
            println!("meet error when decoding the cert result from chain: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    certs.append(&mut cert);
    let mut ending = vec![0u8];
    certs.append(&mut ending);

    certs.truncate(certs.len());
    let (certs, cert_len, _) = into_raw_parts(certs);

    let quote_config = SgxQlConfig::new(cpu_svn.cpu_svn, pce_svn.isv_svn, certs, cert_len);
    unsafe {
        *pp_quote_config = Box::into_raw(Box::new(quote_config));
    }
    let quote_config = unsafe { *pp_quote_config };
    println!("quote_config: {:?}", quote_config);
    let cert_data = unsafe {
        std::slice::from_raw_parts(
            (*quote_config).cert_data,
            (*quote_config).cert_data_size as usize,
        )
    };
    println!("cert len: {}", unsafe {(*quote_config).cert_data_size as usize});
    println!("cert: {:?}", cert_data);
    println!("certdata: {}", std::str::from_utf8(cert_data).unwrap());

    return Quote3Error::SgxQlSuccess;
}

#[no_mangle]
pub extern "C" fn sgx_ql_free_quote_config(quote_config: *mut SgxQlConfig) -> Quote3Error {
    println!("Automata DCAP QPL: sgx_ql_free_quote_config");
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
    println!("Automata DCAP QPL: sgx_ql_get_quote_verification_collateral");
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
    println!("Automata DCAP QPL: sgx_ql_free_quote_verification_collateral");
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
    println!("Automata DCAP QPL: tdx_ql_get_quote_verification_collateral");
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
    println!("Automata DCAP QPL: tdx_ql_free_quote_verification_collateral");
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
    println!("Automata DCAP QPL: sgx_ql_get_qve_identity");
    if pp_qve_identity.is_null() {
        println!("Pointer to qve identity pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_qve_identity }.is_null() {
        println!("Qve identity pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if pp_qve_identity_issuer_chain.is_null() {
        println!("Pointer to issuer chain pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_qve_identity_issuer_chain }.is_null() {
        println!("Issuer chain pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    }

    let id = U256::from(EnclaveID::QVE as u32);
    let version = U256::from(2u32);

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
            println!("sgx_ql_get_qve_identity meet error: {:?}", err);
            return Quote3Error::SgxQlErrorNoQveIdentityData;
        }
    };
    if qve == EnclaveIdentityJsonObj::default() {
        println!("cannot find qve identity on chain, plz use tool to upload first");
        return Quote3Error::SgxQlErrorNoQveIdentityData;
    }
    let issuer_chain: (Bytes, Bytes) = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity_issuer_chain()
            .call(),
    ) {
        Ok(v) => v,
        Err(err) => {
            println!("sgx_ql_get_qve_identity meet error: {:?}", err);
            return Quote3Error::SgxQlErrorNoQveIdentityData;
        }
    };
    if issuer_chain.0.len() == 0 || issuer_chain.1.len() == 0 {
        println!(
            "cannot find enclave identity issuer chains on chain, plz use tool to upload first"
        );
        return Quote3Error::SgxQlErrorNoQveIdentityData;
    }

    let qve_identity_str = format!(
        "{{\"enclaveIdentity\":{},\"signature\":\"{}\"}}",
        qve.identity_str,
        qve.signature.to_string().trim_start_matches("0x")
    );
    let qve_identity_c_str = CString::new(qve_identity_str).unwrap();
    let qve_identity_len = qve_identity_c_str.as_bytes_with_nul().len();
    let qve_identity_issuer_chain_str = format!("{}{}", issuer_chain.0.to_string(), issuer_chain.1.to_string());
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
    println!("Automata DCAP QPL: sgx_ql_free_qve_identity");
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
    println!("Automata DCAP QPL: sgx_ql_get_root_ca_crl");
    if pp_root_ca_crl.is_null() {
        println!("Pointer to crl pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_root_ca_crl }.is_null() {
        println!("Crl pointer is not null. This memory will be allocated by this library");
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
            println!("sgx_ql_get_qve_identity meet error: {:?}", err);
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    if root_ca_crl_bytes.len() == 0 {
        println!("cannot find root ca crl on chain, plz use tool to upload first");
        return Quote3Error::SgxQlErrorUnexpected;
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
    println!("Automata DCAP QPL: sgx_ql_free_root_ca_crl");
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
        println!("FMSPC is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if fmspc_size == 0 {
        println!("FMSPC buffer size is 0");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if pck_ca.is_null() {
        println!("PCK CA is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if pp_quote_collateral.is_null() {
        println!("Pointer to collateral pointer is null");
        return Quote3Error::SgxQlErrorInvalidParameter;
    } else if !unsafe { *pp_quote_collateral }.is_null() {
        println!("Collateral pointer is not null. This memory will be allocated by this library");
        return Quote3Error::SgxQlErrorInvalidParameter;
    }

    let pck_ca_cstr = unsafe { CStr::from_ptr(pck_ca) };
    let pck_ca_string: String = pck_ca_cstr.to_string_lossy().into_owned();
    if pck_ca_string != "processor".to_string() && pck_ca_string != "platform".to_string() {
        println!(
            "pck_ca must be either processor or platform, but the value is {:?}",
            pck_ca_string
        );
        return Quote3Error::SgxQlErrorInvalidParameter;
    }
    println!("pck_ca: {:?}", pck_ca_string);
    let fmspc_u8_vec = unsafe { std::slice::from_raw_parts(fmspc, fmspc_size as usize) };
    let fmspc_string = String::from_utf8_lossy(fmspc_u8_vec).into_owned();
    println!("fmspc: {:?}", fmspc_string);

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
        Err(_) => {
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    let pck_crl_c_str = CString::new(pck_crl_bytes.to_string()).unwrap();
    let pck_dao_address = PCK_DAO_PORTAL_CONTRACT_ADDRESS.parse::<Address>().unwrap();
    let pck_dao = PckDao::new(pck_dao_address, client.clone());
    let pck_cert_chains = match rt.block_on(pck_dao.get_pck_cert_chain(ca_id).call()) {
        Ok(v) => v,
        Err(_) => {
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    let pck_crl_issuer_chain_str = format!("{}{}", pck_cert_chains.0.to_string(), pck_cert_chains.1.to_string());
    let pck_crl_issuer_chain_c_str = CString::new(pck_crl_issuer_chain_str).unwrap();

    // Get Root CA CRL
    let ca_id = CAID::Root as u8;
    let (_, root_ca_crl_bytes) = match rt.block_on(pcs_dao.get_certificate_by_id(ca_id).call()) {
        Ok(v) => v,
        Err(_) => {
            return Quote3Error::SgxQlErrorUnexpected;
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
    // TOOD which version shall we use here? v2 or v3?
    let version = U256::from(2);
    let tcb_info: TcbInfoJsonObj = match rt.block_on(
        fmspc_tcb_dao
            .get_tcb_info(tcb_type, fmspc_string, version)
            .call(),
    ) {
        Ok(v) => v,
        Err(_) => {
            return Quote3Error::SgxQlErrorTcbinfoNotFound;
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
        Err(_) => {
            return Quote3Error::SgxQlErrorTcbinfoNotFound;
        }
    };
    let tcb_info_issuer_chain_str = format!("{}{}", tcb_info_issuer_chains.0.to_string(), tcb_info_issuer_chains.1.to_string());
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
    let version = U256::from(2u32);
    let qe: EnclaveIdentityJsonObj = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity(id, version)
            .call(),
    ) {
        Ok(v) => v,
        Err(_) => {
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    let issuer_chain: (Bytes, Bytes) = match rt.block_on(
        enclave_identity_dao
            .get_enclave_identity_issuer_chain()
            .call(),
    ) {
        Ok(v) => v,
        Err(_) => {
            return Quote3Error::SgxQlErrorUnexpected;
        }
    };
    let qe_identity_str = format!(
        "{{\"enclaveIdentity\":{},\"signature\":\"{}\"}}",
        qe.identity_str,
        qe.signature.to_string().trim_start_matches("0x")
    );
    let qe_identity_c_str = CString::new(qe_identity_str).unwrap();
    let qe_identity_issuer_chain_str = format!("{}{}", issuer_chain.0.to_string(), issuer_chain.1.to_string());
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
    println!("quote_collateral: {:?}", quote_collateral);

    return Quote3Error::SgxQlSuccess;
}
