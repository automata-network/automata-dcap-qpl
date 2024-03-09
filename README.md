# Automata DCAP QPL

## About

We present a Web3 implementation of the Intel SGX DCAP v3 attestation using on-chain capabilities, available at [on-chain Intel SGX DCAP v3 attestation repo](https://github.com/automata-network/automata-dcap-v3-attestation). This implementation aims to transition the DCAP attestation verification process from an off-chain entity to a decentralized on-chain environment. However, a notable drawback is its reliance on the contract owner to periodically upload verification collaterals to fulfill the attestation requirements. Additionally, the manually uploaded collaterals lack Trusted Computing Base (TCB) signature verification, rendering them insecure. Any compromise of the contract owner jeopardizes all verification processes.

To address this vulnerability, we introduce an on-chain implementation of the Intel SGX DCAP attestation Provisioning Certification Caching Service (PCCS). This service enables both the quote generator and verifier to conveniently fetch and upload collaterals through smart contracts. Enhancing security, we incorporate read/write validation for all uploaded collaterals via the Linea Verax Attestation Registry.

Within this repository, two key components are featured:
* **Automata DCAP QPL LIB**: The customized platform quote provider library when generating the Intel SGX DCAP v3 attestation quote.
* **Automata DCAP QPL TOOL** This tool facilitates interaction with the on-chain registry, enabling quote generators to upload necessary collaterals for use in both quote generation and verification processes.

## Usage

Take [SGXDataCenterAttestationPrimitives](https://github.com/intel/SGXDataCenterAttestationPrimitives/tree/dcap_1.16_reproducible) repo as the example to demonstrate how to use this lib and tool.

**Automata DCAP QPL LIB**
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

**Automata DCAP QPL TOOL**

Build the tool and upload the necessary collaterals on chain to satisfy the quote generation and verification requirements, you need to prepare a wallet with enough balance in Linea Sepolia Testnet for the transactions.

Use `./automata-dcap-qpl-tool -h` to see the details, or edit the [code](./automata-dcap-qpl-tool/src/main.rs) to set the necessary inputs.

Example:
Upload the sgx_ql_get_quote_config necessary collaterals in quote generation.
```
$ ./automata-dcap-qpl-tool -f sgx_ql_get_quote_config -s Azure -p <wallet_private_key>
```
See C.1. section in [Intel SGX ECDSA Quote Lib Reference DCAP API](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/Intel_SGX_ECDSA_QuoteLibReference_DCAP_API.pdf).

## See also

* [SGX DCAP Caching Service Design Guide](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/SGX_DCAP_Caching_Service_Design_Guide.pdf)
* [Intel SGX ECDSA Quote Lib Reference DCAP API](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/Intel_SGX_ECDSA_QuoteLibReference_DCAP_API.pdf)
* [DCAP ECDSA Orientation](https://download.01.org/intel-sgx/sgx-dcap/1.20/linux/docs/DCAP_ECDSA_Orientation.pdf)
* [Linea Verax Attestation Registry](https://docs.ver.ax/verax-documentation/core-concepts/high-level-overview)

## Contributing

**Before You Contribute**:
* **Raise an Issue**: If you find a bug or wish to suggest a feature, please open an issue first to discuss it. Detail the bug or feature so we understand your intention.  
* **Pull Requests (PR)**: Before submitting a PR, ensure:  
    * Your contribution successfully builds.
    * It includes tests, if applicable.

## License

Apache License
