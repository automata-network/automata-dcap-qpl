<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_Black%20Text%20with%20Color%20Logo.png">
    <img src="https://raw.githubusercontent.com/automata-network/automata-brand-kit/main/PNG/ATA_White%20Text%20with%20Color%20Logo.png" width="50%">
  </picture>
</div>

# Automata DCAP QPL
[![Automata DCAP QPL](https://img.shields.io/badge/Power%20By-Automata-orange.svg)](https://github.com/automata-network)

## About

We present a Web3 implementation of the Intel SGX DCAP V3 attestation using on-chain capabilities, available at [on-chain Intel SGX DCAP v3 attestation repo](https://github.com/automata-network/automata-dcap-v3-attestation). This implementation aims to transition the DCAP attestation verification process from an off-chain entity to a decentralized on-chain environment. However, a notable drawback is its reliance on the contract owner to periodically upload verification collaterals to fulfill the attestation requirements. Additionally, the manually uploaded collaterals lack Trusted Computing Base (TCB) signature verification, rendering them insecure. Any compromise of the contract owner jeopardizes all verification processes.

To address this vulnerability, we introduce an on-chain implementation of the Intel SGX DCAP attestation Provisioning Certification Caching Service (PCCS) [here](https://github.com/automata-network/automata-on-chain-pccs/tree/default). This service enables both the quote generator and verifier to conveniently fetch and upload collaterals through smart contracts. To enhance security, we incorporate read/write validation for all uploaded collaterals before inserting them to the on-chain PCCS. Simultaneously, we are releasing the [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation) as the next iteration of the Automata DCAP V3 attestation, as mentioned in the previous paragraph, to streamline the verification process while providing enhanced security and availability guarantees.

Within this repository, two key components are featured:
* **Automata DCAP QPL LIB**: The customized platform quote provider library when generating and verifying the Intel SGX / TDX DCAP attestation quote. It will try to fetch the necessary collaterals from on-chain PCCS, and if cache miss, it will try to use cloud provider PCCS as the fallback option, and use Intel PCS as the final fallback choice.
* **Automata DCAP QPL TOOL**: This tool facilitates interaction with the on-chain registry, enabling quote generators to upload necessary collaterals for use in both quote generation and verification processes.

## Usage

Take [Automata SGX Scaffold](https://github.com/automata-network/sgx-scaffold) repo as the example to demonstrate how to use this lib and tool.

### **Automata DCAP QPL LIB**
1. Build the lib with the following commands, you can find the lib at `automata-dcap-qpl/automata-dcap-qpl-lib/target/release` path.
```
$ git clone git@github.com:automata-network/automata-dcap-qpl.git
$ cd automata-dcap-qpl-lib
$ cargo build --release
```

2. Move the lib to override the default platform quote provider library, please make sure you already follow the Automata SGX Scaffold tutorial to setup the environment.
```
$ for f in /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so*; do sudo mv "$f" "$f.bak"; done
$ cd automata-dcap-qpl/automata-dcap-qpl-lib/target/release
$ sudo cp libautomata_dcap_qpl.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so
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

Use `./automata-dcap-qpl-tool -h` to see the details, or edit the [code](./automata-dcap-qpl-tool/src/main.rs) to set the necessary inputs.

Example:
Use the quote to check whether there is any missing collateral on-chain, and the tool will help you to fetch and upsert it before you perform the on-chain verification.
```
$ ./automata-dcap-qpl-tool --quote_hex <quote hex string> -p <wallet_private_key>
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

#### [Automata Testnet](https://docs.ata.network/protocol/testnet)

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0xefE368b17D137E86298eec8EbC5502fb56d27832](https://explorer-testnet.ata.network/address/0xefE368b17D137E86298eec8EbC5502fb56d27832) |
| PCCS_ROUTER | [0xbFDeE7A1f1bFA2267cD0DA50BE76D8c4a3864543](https://explorer-testnet.ata.network/address/0xbFDeE7A1f1bFA2267cD0DA50BE76D8c4a3864543) |
| PCCS_Storage | [0xe8599DD2366230B7EfDD526985c64C7325b27569](https://explorer-testnet.ata.network/address/0xe8599DD2366230B7EfDD526985c64C7325b27569) |
| V3_VERIFIER | [0x67042D171b8B7Da1A4a98Df787bDce79190DAc3c](https://explorer-testnet.ata.network/address/0x67042D171b8B7Da1A4a98Df787bDce79190DAc3c) |
| V4_VERIFIER | [0x921B8F6Ec83E405B715111eC1AE8B54A3ea063EB](https://explorer-testnet.ata.network/address/0x921B8F6Ec83E405B715111eC1AE8B54A3ea063EB) |
| ENCLAVE_ID_DAO | [0x413272890ab9F155a47A5F90a404Fb51aa259087](https://explorer-testnet.ata.network/address/0x413272890ab9F155a47A5F90a404Fb51aa259087) |
| FMSPC_TCB_DAO | [0x7c04B466DebA13D48116b1339C62b35B9805E5A0](https://explorer-testnet.ata.network/address/0x7c04B466DebA13D48116b1339C62b35B9805E5A0) |
| PCK_DAO | [0x6D4cA6AE5315EBBcb4331c82531db0ad8853Eb31](https://explorer-testnet.ata.network/address/0x6D4cA6AE5315EBBcb4331c82531db0ad8853Eb31) |
| PCS_DAO | [0xD0335cbC73CA2f8EDd98a2BE3909f55642F414D7](https://explorer-testnet.ata.network/address/0xD0335cbC73CA2f8EDd98a2BE3909f55642F414D7) |
| ENCLAVE_IDENTITY_HELPER | [0xfd4a34b578B352FE1896CDafaEb0f45f993352Bf](https://explorer-testnet.ata.network/address/0xfd4a34b578B352FE1896CDafaEb0f45f993352Bf) |
| FMSPC_TCB_HELPER | [0xC2A662e08A35513596E22D0aC236Ce72e59125EE](https://explorer-testnet.ata.network/address/0xC2A662e08A35513596E22D0aC236Ce72e59125EE) |
| PCK_HELPER | [0x5213c0e3Ab478dbc83E8afFF8909717332E4f8E1](https://explorer-testnet.ata.network/address/0x5213c0e3Ab478dbc83E8afFF8909717332E4f8E1) |
| CRL_HELPER | [0x12C1E13Aa2a238EAb15c2e2b6AC670266bc3C814](https://explorer-testnet.ata.network/address/0x12C1E13Aa2a238EAb15c2e2b6AC670266bc3C814) |

#### [Automata Mainnet](https://docs.ata.network/protocol/mainnet)

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0xE26E11B257856B0bEBc4C759aaBDdea72B64351F](https://explorer.ata.network/address/0xE26E11B257856B0bEBc4C759aaBDdea72B64351F) |
| PCCS_ROUTER | [0xb76834729717868fa203b9D90fc88F859A4E594D](https://explorer.ata.network/address/0xb76834729717868fa203b9D90fc88F859A4E594D) |
| PCCS_STORAGE | [0xE2636fdbd053da8E798D959304e20fADa934E8c0](https://explorer.ata.network/address/0xE2636fdbd053da8E798D959304e20fADa934E8c0) |
| V3_VERIFIER | [0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96](https://explorer.ata.network/address/0xF38a49322cAA0Ead71D4B1cF2afBb6d02BE5FC96) |
| V4_VERIFIER | [0xfF47ecA64898692a86926CDDa794807be3f6567D](https://explorer.ata.network/address/0xfF47ecA64898692a86926CDDa794807be3f6567D) |
| ENCLAVE_ID_DAO | [0x28111536292b34f37120861A46B39BF39187d73a](https://explorer.ata.network/address/0x28111536292b34f37120861A46B39BF39187d73a) |
| FMSPC_TCB_DAO | [0x868c18869f68E0E0b0b7B2B4439f7fDDd0421e6b](https://explorer.ata.network/address/0x868c18869f68E0E0b0b7B2B4439f7fDDd0421e6b) |
| PCK_DAO | [0xeCc198936FcA3Ca1fDc97B8612B32185908917B0](https://explorer.ata.network/address/0xeCc198936FcA3Ca1fDc97B8612B32185908917B0) |
| PCS_DAO | [0x86f8865BCe8BE62CB8096b5B94fA3fB3a6ED330c](https://explorer.ata.network/address/0x86f8865BCe8BE62CB8096b5B94fA3fB3a6ED330c) |
| ENCLAVE_IDENTITY_HELPER | [0x13BECaa512713Ac7C2d7a04ba221aD5E02D43DFE](https://explorer.ata.network/address/0x13BECaa512713Ac7C2d7a04ba221aD5E02D43DFE) |
| FMSPC_TCB_HELPER | [0xc99bF04C31bF3d026B5B47b2574FC19C1459B732](https://explorer.ata.network/address/0xc99bF04C31bF3d026B5B47b2574FC19C1459B732) |
| X509_HELPER | [0x3e2fe733E444313A93Fa3f9AEd3bB203048dDE70](https://explorer.ata.network/address/0x3e2fe733E444313A93Fa3f9AEd3bB203048dDE70) |
| X509_CRL_HELPER | [0x2567245dE6E349C8B7AA82fD6FF854b844A0aEF9](https://explorer.ata.network/address/0x2567245dE6E349C8B7AA82fD6FF854b844A0aEF9) |

#### Ethereum Holesky Testnet

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0x133303659F51d75ED216FD98a0B70CbCD75339b2](https://holesky.etherscan.io/address/0x133303659F51d75ED216FD98a0B70CbCD75339b2) |
| PCCS_ROUTER | [0xdE5e69A2ca2556fe46883d754d987703bF28Cc51](https://holesky.etherscan.io/address/0xdE5e69A2ca2556fe46883d754d987703bF28Cc51) |
| V3_VERIFIER | [0x12d7d59Ae1e4dbF83b08C82958Ac3FcEB84fB164](https://holesky.etherscan.io/address/0x12d7d59Ae1e4dbF83b08C82958Ac3FcEB84fB164) |
| V4_VERIFIER | [0x3Cb24c454a29e796edF47a96dF32DD1855058258](https://holesky.etherscan.io/address/0x3Cb24c454a29e796edF47a96dF32DD1855058258) |
| ENCLAVE_ID_DAO | [0x9f4b0fB3A95072bD133082e9683A3536669EFE07](https://holesky.etherscan.io/address/0x9f4b0fB3A95072bD133082e9683A3536669EFE07) |
| FMSPC_TCB_DAO | [0xaB5074445E5ae3C650553d5a7560B3A7121635B9](https://holesky.etherscan.io/address/0xaB5074445E5ae3C650553d5a7560B3A7121635B9) |
| PCK_DAO | [0x5B2d7781E3c44966769484daBCdc435EFD281c34](https://holesky.etherscan.io/address/0x5B2d7781E3c44966769484daBCdc435EFD281c34) |
| PCS_DAO | [0x66FdB4E72d2F4a7e2081bf83F1FfACC9bbCb384b](https://holesky.etherscan.io/address/0x66FdB4E72d2F4a7e2081bf83F1FfACC9bbCb384b) |

#### Ethereum Sepolia Testnet

| Contract | Address |
| --- | --- |
| DCAP_ATTESTATION | [0x76A3657F2d6c5C66733e9b69ACaDadCd0B68788b](https://sepolia.etherscan.io/address/0x76A3657F2d6c5C66733e9b69ACaDadCd0B68788b) |
| PCCS_ROUTER | [0xdc7dcF60b9580980128539Ed805D03BC60F84fd4](https://sepolia.etherscan.io/address/0xdc7dcF60b9580980128539Ed805D03BC60F84fd4) |
| V3_VERIFIER | [0x85E156d702bb3e45690DAa812238C1A841E2c3C5](https://sepolia.etherscan.io/address/0x85E156d702bb3e45690DAa812238C1A841E2c3C5) |
| V4_VERIFIER | [0xdc25e1c7ACAdBdE8C1E2c2b9511B7Dbd98B44700](https://sepolia.etherscan.io/address/0xdc25e1c7ACAdBdE8C1E2c2b9511B7Dbd98B44700) |
| ENCLAVE_ID_DAO | [0x4bb680A5e6Ad6228E7d334903B0Ce10EF60c961C](https://sepolia.etherscan.io/address/0x4bb680A5e6Ad6228E7d334903B0Ce10EF60c961C) |
| FMSPC_TCB_DAO | [0xF790b1C23e6508A6135Ce88450eC0A59Af0B9896](https://sepolia.etherscan.io/address/0xF790b1C23e6508A6135Ce88450eC0A59Af0B9896) |
| PCK_DAO | [0x3eA9D905Cb79586C2184f329e6a651D97F2ebee3](https://sepolia.etherscan.io/address/0x3eA9D905Cb79586C2184f329e6a651D97F2ebee3) |
| PCS_DAO | [0x348DA46aA11188f641f01dbe247b25FFA5FFB9c4](https://sepolia.etherscan.io/address/0x348DA46aA11188f641f01dbe247b25FFA5FFB9c4) |

## See also

* [Automata On Chain PCCS](https://github.com/automata-network/automata-on-chain-pccs)
* [Automata DCAP Attestation](https://github.com/automata-network/automata-dcap-attestation)
* [Automata On-chain PCCS Reader](https://github.com/automata-network/pccs-reader-rs)
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
