use ethers::prelude::*;
use std::env;

pub fn parse_address_from_env_var(env_var_name: &str) -> Address {
    let mut result_str = env::var(env_var_name)
        .expect(&format!("Environment variable {} is not set", env_var_name));
    result_str = result_str.trim_start_matches("0x").to_string();
    let result_address = result_str.parse::<Address>()
        .expect(&format!("Unable to parse \"{}\" as address", result_str));

    result_address
}
