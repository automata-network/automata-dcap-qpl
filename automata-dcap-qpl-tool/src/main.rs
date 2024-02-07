mod cloud_providers;
mod contracts;
mod helper;
mod pccs_types;

use automata_dcap_qpl_common::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(
        short = "f",
        long = "function",
        default_value = "",
        help = "The function needs to be operated and updated on chain, acceptable values: sgx_ql_get_quote_config, sgx_ql_get_quote_verification_collateral, tdx_ql_get_quote_verification_collateral, sgx_ql_get_qve_identity and sgx_ql_get_root_ca_crl"
    )]
    func: String,
    #[structopt(
        short = "s",
        long = "source",
        default_value = "Azure",
        help = "Acceptable values: Azure, Local, All. You need to specify the PCCS URL when you configure as Local"
    )]
    source: String,
    #[structopt(
        short = "v",
        long = "collateral_version",
        default_value = "v3",
        help = "Acceptable values for sgx collateral version are empty, v1, v2, v3 or v4"
    )]
    version: String,
    #[structopt(
        short = "u",
        long = "pccs_url",
        default_value = "",
        help = "The local PCCS URL when the source is Local or All"
    )]
    pccs_url: String,
    #[structopt(
        short = "p",
        long = "private_key",
        default_value = "",
        help = "Wallet's private key to perform the transaction to update the on-chain PCCS"
    )]
    private_key: String,
    #[structopt(
        short = "c",
        long = "chain_id",
        default_value = "59140",
        help = "Default: Linea Goerli Testnet(Verax Attestation Registry)"
    )]
    chain_id: u64,
}

fn main() {
    // let abi_source = "/data/jiaquan/code/automata-on-chain-pccs/out/EnclaveIdentityDao.sol/EnclaveIdentityDao.json";
    // let out_file = std::env::temp_dir().join("EnclaveIdentityDao.rs");
    // if out_file.exists() {
    //     std::fs::remove_file(&out_file).unwrap();
    // }
    // ethers::prelude::Abigen::new("EnclaveIdentityDao", abi_source).unwrap().generate().unwrap().write_to_file(out_file).unwrap();

    // let abi_source = "/data/jiaquan/code/automata-on-chain-pccs/out/FmspcTcbDao.sol/FmspcTcbDao.json";
    // let out_file = std::env::temp_dir().join("FmspcTcbDao.rs");
    // if out_file.exists() {
    //     std::fs::remove_file(&out_file).unwrap();
    // }
    // ethers::prelude::Abigen::new("FmspcTcbDao", abi_source).unwrap().generate().unwrap().write_to_file(out_file).unwrap();

    // let abi_source = "/data/jiaquan/code/automata-on-chain-pccs/out/PckDao.sol/PckDao.json";
    // let out_file = std::env::temp_dir().join("PckDao.rs");
    // if out_file.exists() {
    //     std::fs::remove_file(&out_file).unwrap();
    // }
    // ethers::prelude::Abigen::new("PckDao", abi_source).unwrap().generate().unwrap().write_to_file(out_file).unwrap();

    // let abi_source = "/data/jiaquan/code/automata-on-chain-pccs/out/PcsDao.sol/PcsDao.json";
    // let out_file = std::env::temp_dir().join("PcsDao.rs");
    // if out_file.exists() {
    //     std::fs::remove_file(&out_file).unwrap();
    // }
    // ethers::prelude::Abigen::new("PcsDao", abi_source).unwrap().generate().unwrap().write_to_file(out_file).unwrap();

    // let abi_source = "/data/jiaquan/code/automata-on-chain-pccs/out/PlatformTcbsDao.sol/PlatformTcbsDao.json";
    // let out_file = std::env::temp_dir().join("PlatformTcbsDao.rs");
    // if out_file.exists() {
    //     std::fs::remove_file(&out_file).unwrap();
    // }
    // ethers::prelude::Abigen::new("PlatformTcbsDao", abi_source).unwrap().generate().unwrap().write_to_file(out_file).unwrap();
    // return;

    let opt = Opt::from_args();
    let data_source = if opt.source == "Local".to_string() {
        DataSource::Local
    } else if opt.source == "Azure".to_string() {
        DataSource::Azure
    } else {
        DataSource::All
    };
    if opt.version != "v1".to_string()
        && opt.version != "v2".to_string()
        && opt.version != "v3".to_string()
        && opt.version != "v4".to_string()
    {
        println!("Invalid sgx collateral version");
        return;
    }

    // TODO How to parse the user's input parameters for each function
    if opt.func == "sgx_ql_get_quote_config".to_string() {
        let mut qe_id =
            hex::decode("ad04024c9dfb382baf51ca3e5d6cb6e6").expect("Failed to decode hex string");
        let mut cpu_svn = SgxCpuSvn {
            cpu_svn: [12, 12, 16, 15, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let mut isv_svn = SgxIsvSvn { isv_svn: 14 };
        let pck_cert_id: SgxQlPckCertId = SgxQlPckCertId {
            p_qe3_id: qe_id.as_mut_ptr(),
            qe3_id_size: 16,
            p_platform_cpu_svn: &mut cpu_svn as *mut SgxCpuSvn,
            p_platform_pce_isv_svn: &mut isv_svn as *mut SgxIsvSvn,
            p_encrypted_ppid: std::ptr::null_mut(),
            encrypted_ppid_size: 0,
            crypto_suite: 1,
            pce_id: 0,
        };
        helper::sgx_ql_get_quote_config(
            opt.private_key,
            opt.chain_id,
            pck_cert_id,
            data_source,
            opt.version,
            opt.pccs_url,
        );
    } else if opt.func == "sgx_ql_get_quote_verification_collateral".to_string() {
        let hex_string = "00606a000000";
        let decoded_bytes = hex::decode(hex_string).expect("Failed to decode hex string");
        let decoded_string = String::from_utf8_lossy(&decoded_bytes);
        println!("{}", decoded_string);
        helper::sgx_ql_get_quote_verification_collateral(
            opt.private_key,
            opt.chain_id,
            decoded_string.to_string(),
            "platform".to_string(),
            data_source,
            opt.version,
            opt.pccs_url,
        );
    } else if opt.func == "tdx_ql_get_quote_verification_collateral".to_string() {
        // let hex_string = "00606a000000";
        // let decoded_bytes = hex::decode(hex_string).expect("Failed to decode hex string");
        // let decoded_string = String::from_utf8_lossy(&decoded_bytes);
        // println!("{}", decoded_string);
        let decoded_string = "";
        helper::tdx_ql_get_quote_verification_collateral(
            opt.private_key,
            opt.chain_id,
            decoded_string.to_string(),
            "platform".to_string(),
            data_source,
            opt.version,
            opt.pccs_url,
        );
    } else if opt.func == "sgx_ql_get_qve_identity".to_string() {
        helper::sgx_ql_get_qve_identity(
            opt.private_key,
            opt.chain_id,
            data_source,
            opt.version,
            opt.pccs_url,
        );
    } else if opt.func == "sgx_ql_get_root_ca_crl".to_string() {
        helper::sgx_ql_get_root_ca_crl(
            opt.private_key,
            opt.chain_id,
            data_source,
            opt.version,
            opt.pccs_url,
        );
    }
}
