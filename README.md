<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata DCAP QPL
[![Automata DCAP QPL](https://img.shields.io/badge/Powered%20By-Automata-orange.svg)](https://github.com/automata-network)

## About

We present a Web3 implementation of the Intel SGX DCAP V3 attestation using on-chain capabilities, available at [on-chain Intel SGX DCAP v3 attestation repo](https://github.com/automata-network/automata-dcap-v3-attestation). This implementation aims to transition the DCAP attestation verification process from an off-chain entity to a decentralized on-chain environment. However, a notable drawback is its reliance on the contract owner to periodically upload verification collaterals to fulfill the attestation requirements. Additionally, the manually uploaded collaterals lack Trusted Computing Base (TCB) signature verification, rendering them insecure. Any compromise of the contract owner jeopardizes all verification processes.

To address this vulnerability, we introduce an on-chain implementation of the Intel SGX DCAP attestation Provisioning Certification Caching Service (PCCS) [here](https://github.com/automata-network/automata-on-chain-pccs/tree/default). This service enables both the quote generator and verifier to conveniently fetch and upload collaterals through smart contracts. To enhance security, we incorporate read/write validation for all uploaded collaterals before inserting them to the on-chain PCCS. Simultaneously, we are releasing the [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation) as the next iteration of the Automata DCAP V3 attestation, as mentioned in the previous paragraph, to streamline the verification process while providing enhanced security and availability guarantees.

Within this repository, two key components are featured:
* **Automata DCAP QPL LIB**: The customized platform quote provider library when generating and verifying the Intel SGX / TDX DCAP attestation quote. It will try to fetch the necessary collaterals from on-chain PCCS, and if cache miss, it will try to use cloud provider PCCS as the fallback option, and use Intel PCS as the final fallback choice.
* **Automata DCAP QPL TOOL**: This tool facilitates interaction with the on-chain registry, enabling quote generators to upload necessary collaterals for use in both quote generation and verification processes.

## Usage

Take [Automata SGX Scaffold](https://github.com/automata-network/sgx-scaffold) repo as the example to demonstrate how to use this lib and tool.

### **Automata DCAP QPL LIB**
1. Clone with submodules, then choose either the repository-local implementation or the maintained
upstream implementation. Each command uses its own locked dependency graph and writes the library
to a separate target directory.
```
$ git clone --recurse-submodules git@github.com:automata-network/automata-dcap-qpl.git
$ cd automata-dcap-qpl

# Option A: build this repository's local implementation
$ cargo build --release --locked \
    --manifest-path automata-dcap-qpl-lib/Cargo.toml \
    --target-dir target/local-qpl

# Option B: build the maintained automata-dcap-attestation implementation
$ cargo build --release --locked \
    --manifest-path automata-dcap-qpl-tool/pccs-reader-rs/rust-crates/Cargo.toml \
    --target-dir target/upstream-qpl \
    -p automata-dcap-qpl
```

The submodule tracks the maintained DCAP implementation in
[`automata-dcap-attestation`](https://github.com/automata-network/automata-dcap-attestation/tree/main/rust-crates/libraries).
The upstream option builds it directly without a patched or adapted source copy. The local option
continues to use `automata-dcap-qpl-lib/`, `common/`, `contracts/`, and the independently maintained
`automata-dcap-qpl-lib/Cargo.lock`.

2. Move the lib to override the default platform quote provider library, please make sure you already follow the Automata SGX Scaffold tutorial to setup the environment.
```
$ for f in /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so*; do sudo mv "$f" "$f.bak"; done
$ sudo cp target/local-qpl/release/libautomata_dcap_qpl.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so
# Or, when Option B was built:
$ sudo cp target/upstream-qpl/release/libautomata_dcap_qpl.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so
```

3. Build and run the SGX Scaffold, find more details at [Automata SGX Scaffold](https://github.com/automata-network/sgx-scaffold) repo.
```
$ cd sgx-scaffold
$ cargo sgx build
$ cargo sgx run
```

#### Available environment variables:
| variables | description | default value |
|----------|----------|----------|
| AUTOMATA_DCAP_COLLATERAL_VERSION | The API version of the collateral and quote | "v3" |
| INTEL_PCS_SUBSCRIPTION_KEY | The subscription key to be used when fallback to [Intel PCS API](https://api.portal.trustedservices.intel.com/content/documentation.html) | empty |

---

### **Automata DCAP QPL TOOL**

Build the tool and upload the necessary collaterals on chain to satisfy the quote generation and verification requirements, you need to prepare a wallet with enough balance in Automata Testnet for the transactions.

The tool is a compatibility wrapper around the maintained QPL implementation. Its existing flat
flags (`--quote_hex`, `--quote_file`, `--function`, `--private_key`, and the remaining advanced
arguments) are preserved. Its Cargo dependencies point directly at the pinned
`automata-dcap-attestation` revision, so future upstream dependency fixes only require updating
that revision, the matching library submodule revision, and the lock file.

Use `./automata-dcap-qpl-tool -h` to see the details, or edit the [code](./automata-dcap-qpl-tool/src/main.rs) to set the necessary inputs.

Example:
Use the quote to check whether there is any missing collateral on-chain, and the tool will help you to fetch and upsert it before you perform the on-chain verification.
```
cd automata-dcap-qpl
source automata-dcap-qpl-tool/env/automata_testnet
cargo build --release -p automata-dcap-qpl-tool
./target/release/automata-dcap-qpl-tool --quote_hex <quote hex string> -p <wallet_private_key> --chain_id=$CHAIN_ID --rpc_url=$RPC_URL
```
See C.1. section in [Intel SGX ECDSA Quote Lib Reference DCAP API](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/Intel_SGX_ECDSA_QuoteLibReference_DCAP_API.pdf).

#### Upsert transaction samples
* [Upsert FMSPC TCB](https://explorer-testnet.ata.network/tx/0x7a52a2da895cbe3996924e8baa519eb074b8d34b034b5a659b20555947537336)
* [Upsert Enclave Identity](https://explorer-testnet.ata.network/tx/0x1561bb7f08cfa865bc03dd25ce233ad8a7bf7a943c8960e6a8a2a487bc436e58)

#### Available environment variables:
| variables | description | default value |
|----------|----------|----------|
| INTEL_PCS_SUBSCRIPTION_KEY | The subscription key to be used when fallback to [Intel PCS API](https://api.portal.trustedservices.intel.com/content/documentation.html) | empty |

### Fallback logics
* Use Azure PCCS as the fallback option when the collaterals are not available in on-chain PCCS, make sure you're executing the enclave in Azure VM and install the [Azure DCAP client](https://github.com/microsoft/Azure-DCAP-Client).
* Use Intel PCS as the final fallback option, you should specify your `Ocp-Apim-Subscription-Key` to `INTEL_PCS_SUBSCRIPTION_KEY` environment variable before using it.

---

### **Automata On-chain PCCS resources**

Find latest PCCS contracts in [Automata On Chain PCCS](https://github.com/automata-network/automata-on-chain-pccs) repo.

## See also

* [Automata On Chain PCCS](https://github.com/automata-network/automata-on-chain-pccs)
* [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation)
* [Automata On-chain PCCS Reader](https://github.com/automata-network/automata-dcap-attestation/tree/main/rust-crates/libraries/pccs-reader)
* [SGX DCAP Caching Service Design Guide](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf)
* [Intel SGX ECDSA Quote Lib Reference DCAP API](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/Intel_SGX_ECDSA_QuoteLibReference_DCAP_API.pdf)
* [DCAP ECDSA Orientation](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/DCAP_ECDSA_Orientation.pdf)
* [Intel PCS API](https://api.portal.trustedservices.intel.com/content/documentation.html)

## Contributing

**Before You Contribute**:
* **Raise an Issue**: If you find a bug or wish to suggest a feature, please open an issue first to discuss it. Detail the bug or feature so we understand your intention.
* **Pull Requests (PR)**: Before submitting a PR, ensure:
    * Your contribution successfully builds.
    * It includes tests, if applicable.

## License

Apache License
