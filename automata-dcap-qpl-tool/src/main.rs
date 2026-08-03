use automata_dcap_network_registry::Network;
use automata_dcap_qpl::{
    sgx_ql_get_quote_config, sgx_ql_get_quote_verification_collateral, sgx_ql_get_qve_identity,
    sgx_ql_get_root_ca_crl, tdx_ql_get_quote_verification_collateral, upload_missing_collaterals,
    DataSource, SgxCpuSvn, SgxIsvSvn, SgxQlPckCertId,
};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    // basic usage
    #[arg(
        long = "quote_hex",
        default_value = "",
        help = "The quote that needs to check the missing collaterals"
    )]
    quote_hex: String,
    #[arg(
        long = "quote_file",
        default_value = "",
        help = "The quote that needs to check the missing collaterals"
    )]
    quote_file: String,
    #[arg(
        short = 'p',
        long = "private_key",
        default_value = "",
        help = "Wallet's private key to perform the transaction to update the on-chain PCCS"
    )]
    private_key: String,
    #[arg(
        short = 'u',
        long = "pccs_url",
        default_value = "https://api.trustedservices.intel.com",
        help = "The local PCCS URL when the source is Local or All, default URL is the Intel PCS endpoint"
    )]
    pccs_url: String,
    #[arg(
        short = 'r',
        long = "rpc_url",
        default_value = "https://rpc-testnet.ata.network",
        help = "Default: Automata Testnet RPC"
    )]
    rpc_url: String,
    #[arg(
        short = 'c',
        long = "chain_id",
        default_value = "1398243",
        help = "Default: Automata Testnet Chain ID"
    )]
    chain_id: u64,

    // advanced usage
    #[arg(
        short = 'f',
        long = "function",
        default_value = "",
        help = "The function needs to be operated and updated on chain, acceptable values: sgx_ql_get_quote_config, sgx_ql_get_quote_verification_collateral, tdx_ql_get_quote_verification_collateral, sgx_ql_get_qve_identity and sgx_ql_get_root_ca_crl"
    )]
    func: String,
    #[arg(
        short = 's',
        long = "source",
        default_value = "All",
        help = "Acceptable values: Azure, Local, All. You need to specify the PCCS URL when you configure as Local"
    )]
    source: String,
    #[arg(
        short = 'v',
        long = "collateral_version",
        default_value = "v3",
        help = "Acceptable values for sgx collateral version are empty, v1, v2, v3 or v4"
    )]
    version: String,
    #[arg(
        long = "qe_id",
        default_value = "", // f28dda234595e56eaeb7ce9b681a62cd
        help = "qe_id used in sgx_ql_get_quote_config"
    )]
    qe_id: String,
    #[arg(
        long = "pce_id",
        default_value = "", // 0000
        help = "pce_id used in sgx_ql_get_quote_config"
    )]
    pce_id: String,
    #[arg(
        long = "platform_cpu_svn",
        default_value = "", // 0e0e100fffff01000000000000000000
        help = "platform_cpu_svn used in sgx_ql_get_quote_config"
    )]
    platform_cpu_svn: String,
    #[arg(
        long = "platform_pce_isv_svn",
        default_value = "", // 0e00
        help = "platform_pce_isv_svn used in sgx_ql_get_quote_config"
    )]
    platform_pce_isv_svn: String,
    #[arg(
        long = "encrypted_ppid",
        default_value = "", // 096dc5bebd0be1e7786768e390eec435558a2660d4649822b71d0d521d87f09e7f1ad1fbd239d57e2e57fc80261985144187dbcfe94c9f277f7974e3f823ed833a0473646844ef0f7e6da6e36b0d1eb86de2a3d169b3559ec54b09cd7fef64ae834629dcb89276c4eac37982057e71737341e77d7a3befba80739bf07407a51cfe108223f9aa01145bce895c48a322435c626f6f78d95fba0b64d07dd65d4cfe3ef586696f242c5e5548421fbd2056c3e1a5e40454369d38ae9ca0f8cb0c5cc8b5f5f66602c4cbb683357383e29cb09e35bbcc4e630a151f885ef486bf89e06b3348f135441ab0a6ac3cc284524e1137bbc1271021db2917294451c6e628a9094a64ab48a49568bbbde1026c0a19c6cf4f48b0ce8686a33c7eedf16aef2039b0729738c7484aa9b78f7f43654cacb6c2d536e0ebd8759620237a174b5c8b1ac893b16d762c8c18010feab74d5aafb35b1374e711c9d880adc8a5aff2b77be825084f9db216cbb426c8fbae89266126aa22c36dbb1e0e9f7c3baa9f8a5f743e4e
        help = "encrypted_ppid used in sgx_ql_get_quote_config"
    )]
    encrypted_ppid: String,
    #[arg(
        long = "fmspc",
        default_value = "", // 00606a000000
        help = "fmspc used in sgx_ql_get_quote_verification_collateral or tdx_ql_get_quote_verification_collateral"
    )]
    fmspc: String,
    #[arg(
        long = "pck_ca",
        default_value = "", // platform or processor
        help = "pck_ca used in sgx_ql_get_quote_verification_collateral or tdx_ql_get_quote_verification_collateral, acceptable value: platform or processor"
    )]
    pck_ca: String,
    #[arg(
        long = "all_verification_collateral",
        default_value = "0",
        help = "Default: Automata Testnet Chain ID"
    )]
    all_verification_collateral: u64,
}

fn main() {
    let opt = Opt::parse();
    let data_source = if opt.source == "Local" {
        DataSource::Local
    } else if opt.source == "Azure" {
        DataSource::Azure
    } else {
        DataSource::All
    };
    if opt.version != "v1" && opt.version != "v2" && opt.version != "v3" && opt.version != "v4" {
        println!("Invalid sgx collateral version");
        return;
    }

    let has_operation = !opt.quote_hex.is_empty()
        || !opt.quote_file.is_empty()
        || matches!(
            opt.func.as_str(),
            "sgx_ql_get_quote_config"
                | "sgx_ql_get_quote_verification_collateral"
                | "tdx_ql_get_quote_verification_collateral"
                | "sgx_ql_get_qve_identity"
                | "sgx_ql_get_root_ca_crl"
        );
    if !has_operation {
        return;
    }

    let network = Network::by_chain_id(opt.chain_id, None)
        .unwrap_or_else(|| panic!("Unsupported chain_id: {}", opt.chain_id));
    let private_key = if opt.private_key.is_empty() {
        None
    } else {
        Some(opt.private_key.as_str())
    };
    let provider = network
        .create_provider(Some(&opt.rpc_url), private_key)
        .expect("Failed to create provider");

    if !opt.quote_hex.is_empty() {
        let quote = hex::decode(opt.quote_hex.trim_start_matches("0x"))
            .expect("Failed to decode hex string");
        upload_missing_collaterals(&provider, None, &quote, opt.pccs_url, None);
    } else if !opt.quote_file.is_empty() {
        let quote = std::fs::read(opt.quote_file).expect("Error: Unable to open quote file");
        upload_missing_collaterals(&provider, None, &quote, opt.pccs_url, None);
    } else if opt.func == "sgx_ql_get_quote_config" {
        let mut qe_id =
            hex::decode(opt.qe_id.trim_start_matches("0x")).expect("Failed to decode hex string");
        assert!(qe_id.len() == 16);

        let opt_cpu_svn = hex::decode(opt.platform_cpu_svn.trim_start_matches("0x"))
            .expect("Failed to decode hex string");
        assert!(opt_cpu_svn.len() == 16);
        let mut cpu_svn = SgxCpuSvn { cpu_svn: [0; 16] };
        cpu_svn.cpu_svn.copy_from_slice(&opt_cpu_svn);

        let opt_isv_svn = hex::decode(opt.platform_pce_isv_svn.trim_start_matches("0x"))
            .expect("Failed to decode hex string");
        assert!(opt_isv_svn.len() == 2);
        let mut isv_svn = SgxIsvSvn { isv_svn: 0 };
        isv_svn.isv_svn = (opt_isv_svn[1] as u16) * 256u16 + (opt_isv_svn[0] as u16);

        let mut encrypted_ppid = hex::decode(opt.encrypted_ppid.trim_start_matches("0x"))
            .expect("Failed to decode hex string");
        assert!(encrypted_ppid.len() == 384);

        let opt_pce_id =
            hex::decode(opt.pce_id.trim_start_matches("0x")).expect("Failed to decode hex string");
        assert!(opt_pce_id.len() == 2);
        let pce_id = (opt_pce_id[1] as u16) * 256u16 + (opt_pce_id[0] as u16);

        let pck_cert_id: SgxQlPckCertId = SgxQlPckCertId {
            p_qe3_id: qe_id.as_mut_ptr(),
            qe3_id_size: 16,
            p_platform_cpu_svn: &mut cpu_svn as *mut SgxCpuSvn,
            p_platform_pce_isv_svn: &mut isv_svn as *mut SgxIsvSvn,
            p_encrypted_ppid: encrypted_ppid.as_mut_ptr(),
            encrypted_ppid_size: 384,
            crypto_suite: 1,
            pce_id: pce_id,
        };
        sgx_ql_get_quote_config(
            &provider,
            None,
            pck_cert_id,
            data_source,
            opt.version,
            opt.pccs_url,
        );
    } else if opt.func == "sgx_ql_get_quote_verification_collateral" {
        let fmspc = opt.fmspc.trim_start_matches("0x").to_string();
        // let fmspc = hex::decode(opt.fmspc.trim_start_matches("0x")).expect("Failed to decode hex string");
        // let fmspc = String::from_utf8_lossy(&fmspc);
        println!("fmspc: {}, pck_ca: {}", fmspc, opt.pck_ca);
        sgx_ql_get_quote_verification_collateral(
            &provider,
            None,
            fmspc,
            opt.pck_ca,
            data_source,
            opt.version,
            opt.pccs_url,
            opt.all_verification_collateral,
        );
    } else if opt.func == "tdx_ql_get_quote_verification_collateral" {
        let fmspc = opt.fmspc.trim_start_matches("0x").to_string();
        // let fmspc = hex::decode(opt.fmspc.trim_start_matches("0x")).expect("Failed to decode hex string");
        // let fmspc = String::from_utf8_lossy(&fmspc);
        println!("fmspc: {}, pck_ca: {}", fmspc, opt.pck_ca);
        tdx_ql_get_quote_verification_collateral(
            &provider,
            None,
            fmspc,
            opt.pck_ca,
            data_source,
            opt.version,
            opt.pccs_url,
            opt.all_verification_collateral,
        );
    } else if opt.func == "sgx_ql_get_qve_identity" {
        sgx_ql_get_qve_identity(&provider, None, data_source, opt.version, opt.pccs_url);
    } else if opt.func == "sgx_ql_get_root_ca_crl" {
        sgx_ql_get_root_ca_crl(&provider, None, data_source, opt.version, opt.pccs_url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_defaults_are_preserved() {
        let opt = Opt::try_parse_from(["automata-dcap-qpl-tool"]).unwrap();

        assert_eq!(opt.quote_hex, "");
        assert_eq!(opt.quote_file, "");
        assert_eq!(opt.private_key, "");
        assert_eq!(opt.pccs_url, "https://api.trustedservices.intel.com");
        assert_eq!(opt.rpc_url, "https://rpc-testnet.ata.network");
        assert_eq!(opt.chain_id, 1_398_243);
        assert_eq!(opt.func, "");
        assert_eq!(opt.source, "All");
        assert_eq!(opt.version, "v3");
        assert_eq!(opt.all_verification_collateral, 0);
    }

    #[test]
    fn legacy_flat_flags_are_preserved() {
        let opt = Opt::try_parse_from([
            "automata-dcap-qpl-tool",
            "--quote_hex",
            "0x0102",
            "--quote_file",
            "/tmp/quote.bin",
            "-p",
            "private-key",
            "-u",
            "https://pccs.example",
            "-r",
            "https://rpc.example",
            "-c",
            "1",
            "-f",
            "sgx_ql_get_quote_config",
            "-s",
            "Local",
            "-v",
            "v4",
            "--qe_id",
            "01",
            "--pce_id",
            "02",
            "--platform_cpu_svn",
            "03",
            "--platform_pce_isv_svn",
            "04",
            "--encrypted_ppid",
            "05",
            "--fmspc",
            "06",
            "--pck_ca",
            "platform",
            "--all_verification_collateral",
            "1",
        ])
        .unwrap();

        assert_eq!(opt.quote_hex, "0x0102");
        assert_eq!(opt.quote_file, "/tmp/quote.bin");
        assert_eq!(opt.private_key, "private-key");
        assert_eq!(opt.pccs_url, "https://pccs.example");
        assert_eq!(opt.rpc_url, "https://rpc.example");
        assert_eq!(opt.chain_id, 1);
        assert_eq!(opt.func, "sgx_ql_get_quote_config");
        assert_eq!(opt.source, "Local");
        assert_eq!(opt.version, "v4");
        assert_eq!(opt.qe_id, "01");
        assert_eq!(opt.pce_id, "02");
        assert_eq!(opt.platform_cpu_svn, "03");
        assert_eq!(opt.platform_pce_isv_svn, "04");
        assert_eq!(opt.encrypted_ppid, "05");
        assert_eq!(opt.fmspc, "06");
        assert_eq!(opt.pck_ca, "platform");
        assert_eq!(opt.all_verification_collateral, 1);
    }
}
