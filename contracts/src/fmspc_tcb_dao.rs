pub use fmspc_tcb_dao::*;
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
pub mod fmspc_tcb_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FmspcTcbLib"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FmspcTcbLib"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract FmspcTcbHelper"),
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
                    ::std::borrow::ToOwned::to_owned("fmspcTcbInfoAttestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "fmspcTcbInfoAttestations",
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
                    ::std::borrow::ToOwned::to_owned("fmspcTcbSchemaID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fmspcTcbSchemaID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "FMSPC_TCB_SCHEMA_ID",
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
                    ::std::borrow::ToOwned::to_owned("getTcbInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTcbInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fmspc"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tcbObj"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TcbInfoJsonObj"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTcbIssuerChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTcbIssuerChain"),
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
                    ::std::borrow::ToOwned::to_owned("upsertFmspcTcb"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsertFmspcTcb"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbInfoObj"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TcbInfoJsonObj"),
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
                    ::std::borrow::ToOwned::to_owned("TCBInfoMissing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TCBInfoMissing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tcbType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fmspc"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FMSPCTCBDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct FmspcTcbDao<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FmspcTcbDao<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FmspcTcbDao<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FmspcTcbDao<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FmspcTcbDao<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FmspcTcbDao))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FmspcTcbDao<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FMSPCTCBDAO_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `FmspcTcbLib` (0x4ba52fa5) function
        pub fn fmspc_tcb_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([75, 165, 47, 165], ())
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
        ///Calls the contract's `fmspcTcbInfoAttestations` (0xc9d55de4) function
        pub fn fmspc_tcb_info_attestations(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([201, 213, 93, 228], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fmspcTcbSchemaID` (0x3526fd9e) function
        pub fn fmspc_tcb_schema_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([53, 38, 253, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTcbInfo` (0xcfbc42fb) function
        pub fn get_tcb_info(
            &self,
            tcb_type: ::ethers::core::types::U256,
            fmspc: ::std::string::String,
            version: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, TcbInfoJsonObj> {
            self.0
                .method_hash([207, 188, 66, 251], (tcb_type, fmspc, version))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTcbIssuerChain` (0xa53e7275) function
        pub fn get_tcb_issuer_chain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([165, 62, 114, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertFmspcTcb` (0xa8349fb7) function
        pub fn upsert_fmspc_tcb(
            &self,
            tcb_info_obj: TcbInfoJsonObj,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([168, 52, 159, 183], (tcb_info_obj,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TCBInfoMissing` event
        pub fn tcb_info_missing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TcbinfoMissingFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TcbinfoMissingFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FmspcTcbDao<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    #[ethevent(name = "TCBInfoMissing", abi = "TCBInfoMissing(uint256,string,uint256)")]
    pub struct TcbinfoMissingFilter {
        pub tcb_type: ::ethers::core::types::U256,
        pub fmspc: ::std::string::String,
        pub version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `FmspcTcbLib` function with signature `FmspcTcbLib()` and selector `0x4ba52fa5`
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
    #[ethcall(name = "FmspcTcbLib", abi = "FmspcTcbLib()")]
    pub struct FmspcTcbLibCall;
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
    ///Container type for all input parameters for the `fmspcTcbInfoAttestations` function with signature `fmspcTcbInfoAttestations(bytes32)` and selector `0xc9d55de4`
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
        name = "fmspcTcbInfoAttestations",
        abi = "fmspcTcbInfoAttestations(bytes32)"
    )]
    pub struct FmspcTcbInfoAttestationsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `fmspcTcbSchemaID` function with signature `fmspcTcbSchemaID()` and selector `0x3526fd9e`
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
    #[ethcall(name = "fmspcTcbSchemaID", abi = "fmspcTcbSchemaID()")]
    pub struct FmspcTcbSchemaIDCall;
    ///Container type for all input parameters for the `getTcbInfo` function with signature `getTcbInfo(uint256,string,uint256)` and selector `0xcfbc42fb`
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
    #[ethcall(name = "getTcbInfo", abi = "getTcbInfo(uint256,string,uint256)")]
    pub struct GetTcbInfoCall {
        pub tcb_type: ::ethers::core::types::U256,
        pub fmspc: ::std::string::String,
        pub version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTcbIssuerChain` function with signature `getTcbIssuerChain()` and selector `0xa53e7275`
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
    #[ethcall(name = "getTcbIssuerChain", abi = "getTcbIssuerChain()")]
    pub struct GetTcbIssuerChainCall;
    ///Container type for all input parameters for the `upsertFmspcTcb` function with signature `upsertFmspcTcb((string,bytes))` and selector `0xa8349fb7`
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
    #[ethcall(name = "upsertFmspcTcb", abi = "upsertFmspcTcb((string,bytes))")]
    pub struct UpsertFmspcTcbCall {
        pub tcb_info_obj: TcbInfoJsonObj,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FmspcTcbDaoCalls {
        FmspcTcbLib(FmspcTcbLibCall),
        Pcs(PcsCall),
        FmspcTcbInfoAttestations(FmspcTcbInfoAttestationsCall),
        FmspcTcbSchemaID(FmspcTcbSchemaIDCall),
        GetTcbInfo(GetTcbInfoCall),
        GetTcbIssuerChain(GetTcbIssuerChainCall),
        UpsertFmspcTcb(UpsertFmspcTcbCall),
    }
    impl ::ethers::core::abi::AbiDecode for FmspcTcbDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FmspcTcbLibCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FmspcTcbLib(decoded));
            }
            if let Ok(decoded) = <PcsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pcs(decoded));
            }
            if let Ok(decoded) = <FmspcTcbInfoAttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FmspcTcbInfoAttestations(decoded));
            }
            if let Ok(decoded) = <FmspcTcbSchemaIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FmspcTcbSchemaID(decoded));
            }
            if let Ok(decoded) = <GetTcbInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTcbInfo(decoded));
            }
            if let Ok(decoded) = <GetTcbIssuerChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTcbIssuerChain(decoded));
            }
            if let Ok(decoded) = <UpsertFmspcTcbCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertFmspcTcb(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FmspcTcbDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FmspcTcbLib(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FmspcTcbInfoAttestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FmspcTcbSchemaID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbIssuerChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertFmspcTcb(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FmspcTcbDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FmspcTcbLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::FmspcTcbInfoAttestations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FmspcTcbSchemaID(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTcbInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTcbIssuerChain(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertFmspcTcb(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FmspcTcbLibCall> for FmspcTcbDaoCalls {
        fn from(value: FmspcTcbLibCall) -> Self {
            Self::FmspcTcbLib(value)
        }
    }
    impl ::core::convert::From<PcsCall> for FmspcTcbDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<FmspcTcbInfoAttestationsCall> for FmspcTcbDaoCalls {
        fn from(value: FmspcTcbInfoAttestationsCall) -> Self {
            Self::FmspcTcbInfoAttestations(value)
        }
    }
    impl ::core::convert::From<FmspcTcbSchemaIDCall> for FmspcTcbDaoCalls {
        fn from(value: FmspcTcbSchemaIDCall) -> Self {
            Self::FmspcTcbSchemaID(value)
        }
    }
    impl ::core::convert::From<GetTcbInfoCall> for FmspcTcbDaoCalls {
        fn from(value: GetTcbInfoCall) -> Self {
            Self::GetTcbInfo(value)
        }
    }
    impl ::core::convert::From<GetTcbIssuerChainCall> for FmspcTcbDaoCalls {
        fn from(value: GetTcbIssuerChainCall) -> Self {
            Self::GetTcbIssuerChain(value)
        }
    }
    impl ::core::convert::From<UpsertFmspcTcbCall> for FmspcTcbDaoCalls {
        fn from(value: UpsertFmspcTcbCall) -> Self {
            Self::UpsertFmspcTcb(value)
        }
    }
    ///Container type for all return fields from the `FmspcTcbLib` function with signature `FmspcTcbLib()` and selector `0x4ba52fa5`
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
    pub struct FmspcTcbLibReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `fmspcTcbInfoAttestations` function with signature `fmspcTcbInfoAttestations(bytes32)` and selector `0xc9d55de4`
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
    pub struct FmspcTcbInfoAttestationsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `fmspcTcbSchemaID` function with signature `fmspcTcbSchemaID()` and selector `0x3526fd9e`
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
    pub struct FmspcTcbSchemaIDReturn {
        pub fmspc_tcb_schema_id: [u8; 32],
    }
    ///Container type for all return fields from the `getTcbInfo` function with signature `getTcbInfo(uint256,string,uint256)` and selector `0xcfbc42fb`
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
    pub struct GetTcbInfoReturn {
        pub tcb_obj: TcbInfoJsonObj,
    }
    ///Container type for all return fields from the `getTcbIssuerChain` function with signature `getTcbIssuerChain()` and selector `0xa53e7275`
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
    pub struct GetTcbIssuerChainReturn {
        pub signing_cert: ::ethers::core::types::Bytes,
        pub root_cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `upsertFmspcTcb` function with signature `upsertFmspcTcb((string,bytes))` and selector `0xa8349fb7`
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
    pub struct UpsertFmspcTcbReturn {
        pub attestation_id: [u8; 32],
    }
    ///`TcbInfoJsonObj(string,bytes)`
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
    pub struct TcbInfoJsonObj {
        pub tcb_info_str: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
}
