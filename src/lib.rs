mod common;
use common::{Quote3Error, SgxQlConfig, SgxQlPckCertId};

mod moonblaze;
use moonblaze::moonblaze::Moonblaze;

use libloading::{Library, Symbol};

use ethers::prelude::*;
use std::sync::Arc;

const AUTOMATA_TESTNET_URL: &str = "https://1rpc.io/ata/testnet";

// quote3_error_t sgx_ql_get_quote_config(const sgx_ql_pck_cert_id_t *p_pck_cert_id, sgx_ql_config_t **pp_quote_config);
#[no_mangle]
pub extern "C" fn sgx_ql_get_quote_config(
    p_pck_cert_id: *const SgxQlPckCertId,
    pp_quote_config: *mut *mut SgxQlConfig,
) -> Quote3Error {
    let pck_cert_id = unsafe { *p_pck_cert_id };

    println!("czl's sgx_ql_get_quote_config");
    println!("pck_cert_id: {:?}", pck_cert_id);

    println!("cpu_svn: {:?}", unsafe{*pck_cert_id.p_platform_cpu_svn});
    println!("pce_svn: {:?}", unsafe{*pck_cert_id.p_platform_pce_isv_svn});

    let cpu_svn = unsafe{*pck_cert_id.p_platform_cpu_svn};
    let pce_svn = unsafe{*pck_cert_id.p_platform_pce_isv_svn};

    println!("cpu_svn: {:?}", hex::encode(cpu_svn.cpu_svn));
    println!("pce_svn: {:?}", hex::encode(pce_svn.isv_svn.to_le_bytes()));
    println!("pce_id: {:?}", hex::encode(pck_cert_id.pce_id.to_le_bytes()));
    let qe_id = unsafe{std::slice::from_raw_parts(pck_cert_id.p_qe3_id, pck_cert_id.qe3_id_size as usize)};
    println!("qe_id: {:?}", hex::encode(qe_id));
    let encrypted_ppid = unsafe { std::slice::from_raw_parts(pck_cert_id.p_encrypted_ppid, pck_cert_id.encrypted_ppid_size as usize) };
    println!("encrypted_ppid: {:?}", hex::encode(encrypted_ppid));

    let provider = Provider::<Http>::try_from(AUTOMATA_TESTNET_URL).unwrap();
    let client = Arc::new(provider);

    let moonblaze_address = "0xa1f056d076a4DcA0C60084F415Bd9B5b1717c30D".parse::<Address>().unwrap();

    let moonblaze = Moonblaze::new(moonblaze_address, client);
    let qe_id_str = hex::encode(qe_id);
    let cpu_svn_str = hex::encode(unsafe{*pck_cert_id.p_platform_cpu_svn}.cpu_svn);
    let pce_svn_str = hex::encode(unsafe{*pck_cert_id.p_platform_pce_isv_svn}.isv_svn.to_le_bytes());
    let pce_id_str = hex::encode(pck_cert_id.pce_id.to_le_bytes());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build().unwrap();


    let cert: Bytes = rt.block_on(moonblaze.get_cert(qe_id_str, cpu_svn_str, pce_svn_str, pce_id_str).call()).unwrap();
    let quote_config = SgxQlConfig::new(cpu_svn.cpu_svn, pce_svn.isv_svn, cert.to_vec());
    unsafe {
        *pp_quote_config = Box::into_raw(Box::new(quote_config));
    }
    // az_dcap_call(p_pck_cert_id, pp_quote_config);
    let quote_config = unsafe { *pp_quote_config };
    println!("quote_config: {:?}", quote_config);
    let cert_data = unsafe { std::slice::from_raw_parts((*quote_config).cert_data, (*quote_config).cert_data_size as usize) };
    println!("certdata: {}", std::str::from_utf8(cert_data).unwrap());

    return Quote3Error::SgxQlSuccess;
}

#[no_mangle]
pub extern "C" fn sgx_ql_free_quote_config(quote_config: *mut SgxQlConfig) {
    println!("czl's sgx_ql_free_quote_config");
    // free quote_config
    unsafe {
        drop(Box::from_raw(quote_config));
    }
}

fn az_dcap_call(
    p_pck_cert_id: *const SgxQlPckCertId,
    pp_quote_config: *mut *mut SgxQlConfig,
) {
    unsafe {
        let lib = Library::new("libdcap_az_client.so").unwrap();
        let func: Symbol<unsafe extern "C" fn(*const SgxQlPckCertId, *mut *mut SgxQlConfig) -> ()> =
            lib.get(b"sgx_ql_get_quote_config").unwrap();
        func(p_pck_cert_id, pp_quote_config);
    }
}