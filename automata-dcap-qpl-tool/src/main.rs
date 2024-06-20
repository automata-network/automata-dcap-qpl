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
            hex::decode("f28dda234595e56eaeb7ce9b681a62cd").expect("Failed to decode hex string");
        let mut cpu_svn = SgxCpuSvn {
            cpu_svn: [14, 14, 16, 15, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        };
        let mut isv_svn = SgxIsvSvn { isv_svn: 14 };
        let mut encrypted_ppid = hex::decode("096dc5bebd0be1e7786768e390eec435558a2660d4649822b71d0d521d87f09e7f1ad1fbd239d57e2e57fc80261985144187dbcfe94c9f277f7974e3f823ed833a0473646844ef0f7e6da6e36b0d1eb86de2a3d169b3559ec54b09cd7fef64ae834629dcb89276c4eac37982057e71737341e77d7a3befba80739bf07407a51cfe108223f9aa01145bce895c48a322435c626f6f78d95fba0b64d07dd65d4cfe3ef586696f242c5e5548421fbd2056c3e1a5e40454369d38ae9ca0f8cb0c5cc8b5f5f66602c4cbb683357383e29cb09e35bbcc4e630a151f885ef486bf89e06b3348f135441ab0a6ac3cc284524e1137bbc1271021db2917294451c6e628a9094a64ab48a49568bbbde1026c0a19c6cf4f48b0ce8686a33c7eedf16aef2039b0729738c7484aa9b78f7f43654cacb6c2d536e0ebd8759620237a174b5c8b1ac893b16d762c8c18010feab74d5aafb35b1374e711c9d880adc8a5aff2b77be825084f9db216cbb426c8fbae89266126aa22c36dbb1e0e9f7c3baa9f8a5f743e4e").expect("Failed to decode hex string");
        let pck_cert_id: SgxQlPckCertId = SgxQlPckCertId {
            p_qe3_id: qe_id.as_mut_ptr(),
            qe3_id_size: 16,
            p_platform_cpu_svn: &mut cpu_svn as *mut SgxCpuSvn,
            p_platform_pce_isv_svn: &mut isv_svn as *mut SgxIsvSvn,
            p_encrypted_ppid: encrypted_ppid.as_mut_ptr(),
            encrypted_ppid_size: 384,
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
