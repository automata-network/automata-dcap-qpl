pub mod enclave_identity_dao;
pub mod fmspc_tcb_dao;
pub mod pck_dao;
pub mod pcs_dao;

/*
Verax addresses
"0xeAce48c11258Ec79c941Daee80343298dC084Be3", // EnclaveIdentityDaoPortal
"0x1930D878D4BAbb10b2C20F65D84a54fdf709959A", // FmspcTcbDaoPortal
"0x07984d9D2b42A8566B3540eCd223F7c610F78Eb2", // PckDaoPortal
"0xC4838158D29C7DB6D344dDB4C082dB0f30C8073e", // PcsDaoPortal
 */

pub const VERAX_RPC_URL: &str = "https://rpc.tenderly.co/fork/071487b4-cc0c-4632-8cd1-2cc1148267df"; // Tenderly forked
// pub const VERAX_RPC_URL: &str = "https://rpc.goerli.linea.build"; // Linea Goerli Testnet
pub const ENCLAVE_IDENTITY_DAO_PORTAL_CONTRACT_ADDRESS: &str = "0xeAce48c11258Ec79c941Daee80343298dC084Be3";
pub const FMSPC_TCB_DAO_PORTAL_CONTRACT_ADDRESS: &str = "0x1930D878D4BAbb10b2C20F65D84a54fdf709959A";
pub const PCK_DAO_PORTAL_CONTRACT_ADDRESS: &str = "0x07984d9D2b42A8566B3540eCd223F7c610F78Eb2";
pub const PCS_DAO_PORTAL_CONTRACT_ADDRESS: &str = "0xC4838158D29C7DB6D344dDB4C082dB0f30C8073e";
