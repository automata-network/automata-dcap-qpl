pub use enclave_identity_dao::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod enclave_identity_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveIdentityLib"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EnclaveIdentityLib"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract EnclaveIdentityHelper",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Pcs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("Pcs"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract PcsDao"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enclaveIdentityAttestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "enclaveIdentityAttestations",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enclaveIdentitySchemaID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "enclaveIdentitySchemaID",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "ENCLAVE_IDENTITY_SCHEMA_ID",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEnclaveIdentity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getEnclaveIdentity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("enclaveIdObj"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EnclaveIdentityJsonObj",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEnclaveIdentityIssuerChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getEnclaveIdentityIssuerChain",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signingCert"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootCert"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upsertEnclaveIdentity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "upsertEnclaveIdentity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "enclaveIdentityObj",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EnclaveIdentityJsonObj",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("attestationId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EnclaveIdentityMissing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EnclaveIdentityMissing",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Enclave_Id_Mismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Enclave_Id_Mismatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ENCLAVEIDENTITYDAO_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct EnclaveIdentityDao<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EnclaveIdentityDao<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EnclaveIdentityDao<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EnclaveIdentityDao<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EnclaveIdentityDao<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EnclaveIdentityDao))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EnclaveIdentityDao<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ENCLAVEIDENTITYDAO_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `EnclaveIdentityLib` (0x61d20bea) function
        pub fn enclave_identity_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([97, 210, 11, 234], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `Pcs` (0xd88d1df6) function
        pub fn pcs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([216, 141, 29, 246], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enclaveIdentityAttestations` (0x5e3d4711) function
        pub fn enclave_identity_attestations(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([94, 61, 71, 17], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enclaveIdentitySchemaID` (0x0cac6378) function
        pub fn enclave_identity_schema_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([12, 172, 99, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEnclaveIdentity` (0xf0f074f7) function
        pub fn get_enclave_identity(
            &self,
            id: ::ethers::core::types::U256,
            version: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, EnclaveIdentityJsonObj> {
            self.0
                .method_hash([240, 240, 116, 247], (id, version))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEnclaveIdentityIssuerChain` (0x7ecda5f0) function
        pub fn get_enclave_identity_issuer_chain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([126, 205, 165, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertEnclaveIdentity` (0x30f704ea) function
        pub fn upsert_enclave_identity(
            &self,
            id: ::ethers::core::types::U256,
            version: ::ethers::core::types::U256,
            enclave_identity_obj: EnclaveIdentityJsonObj,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 247, 4, 234], (id, version, enclave_identity_obj))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EnclaveIdentityMissing` event
        pub fn enclave_identity_missing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveIdentityMissingFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnclaveIdentityMissingFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for EnclaveIdentityDao<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Enclave_Id_Mismatch` with signature `Enclave_Id_Mismatch()` and selector `0x289fa0cb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Enclave_Id_Mismatch", abi = "Enclave_Id_Mismatch()")]
    pub struct Enclave_Id_Mismatch;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "EnclaveIdentityMissing",
        abi = "EnclaveIdentityMissing(uint256,uint256)"
    )]
    pub struct EnclaveIdentityMissingFilter {
        pub id: ::ethers::core::types::U256,
        pub version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `EnclaveIdentityLib` function with signature `EnclaveIdentityLib()` and selector `0x61d20bea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "EnclaveIdentityLib", abi = "EnclaveIdentityLib()")]
    pub struct EnclaveIdentityLibCall;
    ///Container type for all input parameters for the `Pcs` function with signature `Pcs()` and selector `0xd88d1df6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "Pcs", abi = "Pcs()")]
    pub struct PcsCall;
    ///Container type for all input parameters for the `enclaveIdentityAttestations` function with signature `enclaveIdentityAttestations(bytes32)` and selector `0x5e3d4711`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "enclaveIdentityAttestations",
        abi = "enclaveIdentityAttestations(bytes32)"
    )]
    pub struct EnclaveIdentityAttestationsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `enclaveIdentitySchemaID` function with signature `enclaveIdentitySchemaID()` and selector `0x0cac6378`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "enclaveIdentitySchemaID", abi = "enclaveIdentitySchemaID()")]
    pub struct EnclaveIdentitySchemaIDCall;
    ///Container type for all input parameters for the `getEnclaveIdentity` function with signature `getEnclaveIdentity(uint256,uint256)` and selector `0xf0f074f7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getEnclaveIdentity", abi = "getEnclaveIdentity(uint256,uint256)")]
    pub struct GetEnclaveIdentityCall {
        pub id: ::ethers::core::types::U256,
        pub version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getEnclaveIdentityIssuerChain` function with signature `getEnclaveIdentityIssuerChain()` and selector `0x7ecda5f0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getEnclaveIdentityIssuerChain",
        abi = "getEnclaveIdentityIssuerChain()"
    )]
    pub struct GetEnclaveIdentityIssuerChainCall;
    ///Container type for all input parameters for the `upsertEnclaveIdentity` function with signature `upsertEnclaveIdentity(uint256,uint256,(string,bytes))` and selector `0x30f704ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "upsertEnclaveIdentity",
        abi = "upsertEnclaveIdentity(uint256,uint256,(string,bytes))"
    )]
    pub struct UpsertEnclaveIdentityCall {
        pub id: ::ethers::core::types::U256,
        pub version: ::ethers::core::types::U256,
        pub enclave_identity_obj: EnclaveIdentityJsonObj,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EnclaveIdentityDaoCalls {
        EnclaveIdentityLib(EnclaveIdentityLibCall),
        Pcs(PcsCall),
        EnclaveIdentityAttestations(EnclaveIdentityAttestationsCall),
        EnclaveIdentitySchemaID(EnclaveIdentitySchemaIDCall),
        GetEnclaveIdentity(GetEnclaveIdentityCall),
        GetEnclaveIdentityIssuerChain(GetEnclaveIdentityIssuerChainCall),
        UpsertEnclaveIdentity(UpsertEnclaveIdentityCall),
    }
    impl ::ethers::core::abi::AbiDecode for EnclaveIdentityDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EnclaveIdentityLibCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnclaveIdentityLib(decoded));
            }
            if let Ok(decoded) = <PcsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pcs(decoded));
            }
            if let Ok(decoded) = <EnclaveIdentityAttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnclaveIdentityAttestations(decoded));
            }
            if let Ok(decoded) = <EnclaveIdentitySchemaIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnclaveIdentitySchemaID(decoded));
            }
            if let Ok(decoded) = <GetEnclaveIdentityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEnclaveIdentity(decoded));
            }
            if let Ok(decoded) = <GetEnclaveIdentityIssuerChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetEnclaveIdentityIssuerChain(decoded));
            }
            if let Ok(decoded) = <UpsertEnclaveIdentityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertEnclaveIdentity(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EnclaveIdentityDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EnclaveIdentityLib(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnclaveIdentityAttestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnclaveIdentitySchemaID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEnclaveIdentity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEnclaveIdentityIssuerChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertEnclaveIdentity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EnclaveIdentityDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EnclaveIdentityLib(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveIdentityAttestations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnclaveIdentitySchemaID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEnclaveIdentity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetEnclaveIdentityIssuerChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpsertEnclaveIdentity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EnclaveIdentityLibCall> for EnclaveIdentityDaoCalls {
        fn from(value: EnclaveIdentityLibCall) -> Self {
            Self::EnclaveIdentityLib(value)
        }
    }
    impl ::core::convert::From<PcsCall> for EnclaveIdentityDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<EnclaveIdentityAttestationsCall>
    for EnclaveIdentityDaoCalls {
        fn from(value: EnclaveIdentityAttestationsCall) -> Self {
            Self::EnclaveIdentityAttestations(value)
        }
    }
    impl ::core::convert::From<EnclaveIdentitySchemaIDCall> for EnclaveIdentityDaoCalls {
        fn from(value: EnclaveIdentitySchemaIDCall) -> Self {
            Self::EnclaveIdentitySchemaID(value)
        }
    }
    impl ::core::convert::From<GetEnclaveIdentityCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetEnclaveIdentityCall) -> Self {
            Self::GetEnclaveIdentity(value)
        }
    }
    impl ::core::convert::From<GetEnclaveIdentityIssuerChainCall>
    for EnclaveIdentityDaoCalls {
        fn from(value: GetEnclaveIdentityIssuerChainCall) -> Self {
            Self::GetEnclaveIdentityIssuerChain(value)
        }
    }
    impl ::core::convert::From<UpsertEnclaveIdentityCall> for EnclaveIdentityDaoCalls {
        fn from(value: UpsertEnclaveIdentityCall) -> Self {
            Self::UpsertEnclaveIdentity(value)
        }
    }
    ///Container type for all return fields from the `EnclaveIdentityLib` function with signature `EnclaveIdentityLib()` and selector `0x61d20bea`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnclaveIdentityLibReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `Pcs` function with signature `Pcs()` and selector `0xd88d1df6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PcsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `enclaveIdentityAttestations` function with signature `enclaveIdentityAttestations(bytes32)` and selector `0x5e3d4711`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnclaveIdentityAttestationsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `enclaveIdentitySchemaID` function with signature `enclaveIdentitySchemaID()` and selector `0x0cac6378`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnclaveIdentitySchemaIDReturn {
        pub enclave_identity_schema_id: [u8; 32],
    }
    ///Container type for all return fields from the `getEnclaveIdentity` function with signature `getEnclaveIdentity(uint256,uint256)` and selector `0xf0f074f7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetEnclaveIdentityReturn {
        pub enclave_id_obj: EnclaveIdentityJsonObj,
    }
    ///Container type for all return fields from the `getEnclaveIdentityIssuerChain` function with signature `getEnclaveIdentityIssuerChain()` and selector `0x7ecda5f0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetEnclaveIdentityIssuerChainReturn {
        pub signing_cert: ::ethers::core::types::Bytes,
        pub root_cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `upsertEnclaveIdentity` function with signature `upsertEnclaveIdentity(uint256,uint256,(string,bytes))` and selector `0x30f704ea`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UpsertEnclaveIdentityReturn {
        pub attestation_id: [u8; 32],
    }
    ///`EnclaveIdentityJsonObj(string,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnclaveIdentityJsonObj {
        pub identity_str: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
}
