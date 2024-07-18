# Automata DCAP QPL

## About

We present a Web3 implementation of the Intel SGX DCAP v3 attestation using on-chain capabilities, available at [on-chain Intel SGX DCAP v3 attestation repo](https://github.com/automata-network/automata-dcap-v3-attestation). This implementation aims to transition the DCAP attestation verification process from an off-chain entity to a decentralized on-chain environment. However, a notable drawback is its reliance on the contract owner to periodically upload verification collaterals to fulfill the attestation requirements. Additionally, the manually uploaded collaterals lack Trusted Computing Base (TCB) signature verification, rendering them insecure. Any compromise of the contract owner jeopardizes all verification processes.

To address this vulnerability, we introduce an on-chain implementation of the Intel SGX DCAP attestation Provisioning Certification Caching Service (PCCS) [here](https://github.com/automata-network/automata-on-chain-pccs/tree/default). This service enables both the quote generator and verifier to conveniently fetch and upload collaterals through smart contracts. Enhancing security, we incorporate read/write validation for all uploaded collaterals before inserting them to the on-chain PCCS.

Within this repository, two key components are featured:
* **Automata DCAP QPL LIB**: The customized platform quote provider library when generating and verifying the Intel SGX / TDX DCAP attestation quote. It will try to fetch the necessary collaterals from on-chain PCCS, and if cache miss, it will try to use cloud provider PCCS as the fallback option, and use Intel PCS as the final fallback choice.
* **Automata DCAP QPL TOOL** This tool facilitates interaction with the on-chain registry, enabling quote generators to upload necessary collaterals for use in both quote generation and verification processes.

## Usage

Take [SGXDataCenterAttestationPrimitives](https://github.com/intel/SGXDataCenterAttestationPrimitives/tree/dcap_1.16_reproducible) repo as the example to demonstrate how to use this lib and tool.

### **Automata DCAP QPL LIB**
1. Build the lib with the following commands, you can find the lib at `automata-dcap-qpl/automata-dcap-qpl-lib/target/release` path.
```
$ git clone git@github.com:automata-network/automata-dcap-qpl.git
$ cd automata-dcap-qpl-lib
$ cargo build --release
```

2. Move the lib to override the default platform quote provider library, or you can edit the [sample code](https://github.com/intel/SGXDataCenterAttestationPrimitives/blob/dcap_1.16_reproducible/SampleCode/QuoteGenerationSample/App/App.cpp#L198-L205) to set the path to the previous rust target output path.
```
$ cd automata-dcap-qpl/automata-dcap-qpl-lib/target/release
$ cp libautomata_dcap_qpl.so /usr/lib64/libdcap_quoteprov.so.1
```

3. Build the SGXDataCenterAttestationPrimitives Quote Generation sample code.
```
$ cd SGXDataCenterAttestationPrimitives/SampleCode/QuoteGenerationSample
$ make
```

4. Unset the Intel SGX AESM flag to use the customized quote provider library.
```
$ unset SGX_AESM_ADDR
```

5. Execute the binary to generate a dcap quote by using the customized quote provider library.
```
$ ./app
```

Find the generated quote at `SGXDataCenterAttestationPrimitives/SampleCode/QuoteGenerationSample/quote.dat`

6. Post this quote data to on-chain dcap attestation verification smart contract to verify it, or you can do it by using the local quote verification sample code.
```
$ cd SGXDataCenterAttestationPrimitives/SampleCode/QuoteVerificationSample
$ make SGX_DEBUG=1
$ ./app
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

#### Available environment variables:
| variables | description | default value |
|----------|----------|----------|
| INTEL_PCS_SUBSCRIPTION_KEY | The subscription key to be used when fallback to [Intel PCS API](https://api.portal.trustedservices.intel.com/content/documentation.html) | empty |

### Fallback logics
* Use Azure PCCS as the fallback option when the collaterals are not available in on-chain PCCS, make sure you're executing the enclave in Azure VM and install the [Azure DCAP client](https://github.com/microsoft/Azure-DCAP-Client).
* Use Intel PCS as the final fallback option, you should specify your `Ocp-Apim-Subscription-Key` to `INTEL_PCS_SUBSCRIPTION_KEY` environment variable before using it.

## See also

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
