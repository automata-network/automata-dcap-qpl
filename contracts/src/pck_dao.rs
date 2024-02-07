pub use pck_dao::*;
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
pub mod pck_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("getCert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pckCert"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCerts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCerts"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbms"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pckCerts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPckCertChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPckCertChain"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intermediateCert"),
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
                    ::std::borrow::ToOwned::to_owned("getPlatformTcbByIdAndSvns"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getPlatformTcbByIdAndSvns",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbm"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pckCertAttestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pckCertAttestations",
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
                    ::std::borrow::ToOwned::to_owned("pckSchemaID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pckSchemaID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCK_SCHEMA_ID"),
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
                    ::std::borrow::ToOwned::to_owned("tcbmAttestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tcbmAttestations"),
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
                    ::std::borrow::ToOwned::to_owned("tcbmSchemaId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tcbmSchemaId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("TCBM_SCHEMA_ID"),
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
                    ::std::borrow::ToOwned::to_owned("upsertPckCert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsertPckCert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbm"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cert"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                (
                    ::std::borrow::ToOwned::to_owned("upsertPlatformTcbs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsertPlatformTcbs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbm"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("PCKMissing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PCKMissing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PCKsMissing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PCKsMissing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCBmMissing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TCBmMissing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("qeid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pceid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                    ::std::borrow::ToOwned::to_owned("Invalid_PCK_CA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid_PCK_CA"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Not_An_Admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Not_An_Admin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Pck_Not_Found"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Pck_Not_Found"),
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
    pub static PCKDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct PckDao<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PckDao<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PckDao<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PckDao<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PckDao<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PckDao)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PckDao<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PCKDAO_ABI.clone(),
                    client,
                ),
            )
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
        ///Calls the contract's `getCert` (0x3c7d7114) function
        pub fn get_cert(
            &self,
            qeid: ::std::string::String,
            platform_cpu_svn: ::std::string::String,
            platform_pce_svn: ::std::string::String,
            pceid: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash(
                    [60, 125, 113, 20],
                    (qeid, platform_cpu_svn, platform_pce_svn, pceid),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCerts` (0x28685839) function
        pub fn get_certs(
            &self,
            qeid: ::std::string::String,
            pceid: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::std::string::String>,
                ::std::vec::Vec<::ethers::core::types::Bytes>,
            ),
        > {
            self.0
                .method_hash([40, 104, 88, 57], (qeid, pceid))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPckCertChain` (0x48ac8059) function
        pub fn get_pck_cert_chain(
            &self,
            ca: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([72, 172, 128, 89], ca)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPlatformTcbByIdAndSvns` (0x7eb619c7) function
        pub fn get_platform_tcb_by_id_and_svns(
            &self,
            qeid: ::std::string::String,
            pceid: ::std::string::String,
            platform_cpu_svn: ::std::string::String,
            platform_pce_svn: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash(
                    [126, 182, 25, 199],
                    (qeid, pceid, platform_cpu_svn, platform_pce_svn),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pckCertAttestations` (0x68ef0c53) function
        pub fn pck_cert_attestations(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([104, 239, 12, 83], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pckSchemaID` (0x36c33bb6) function
        pub fn pck_schema_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 195, 59, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tcbmAttestations` (0xe4a4171a) function
        pub fn tcbm_attestations(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([228, 164, 23, 26], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tcbmSchemaId` (0xcbd3805e) function
        pub fn tcbm_schema_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([203, 211, 128, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertPckCert` (0x5be0fa4b) function
        pub fn upsert_pck_cert(
            &self,
            ca: u8,
            qeid: ::std::string::String,
            pceid: ::std::string::String,
            tcbm: ::std::string::String,
            cert: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 224, 250, 75], (ca, qeid, pceid, tcbm, cert))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertPlatformTcbs` (0xc925d17a) function
        pub fn upsert_platform_tcbs(
            &self,
            qeid: ::std::string::String,
            pceid: ::std::string::String,
            platform_cpu_svn: ::std::string::String,
            platform_pce_svn: ::std::string::String,
            tcbm: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [201, 37, 209, 122],
                    (qeid, pceid, platform_cpu_svn, platform_pce_svn, tcbm),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PCKMissing` event
        pub fn pck_missing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PckmissingFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PCKsMissing` event
        pub fn pc_ks_missing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PcksMissingFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TCBmMissing` event
        pub fn tc_bm_missing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TcbmMissingFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PckDaoEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PckDao<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Invalid_PCK_CA` with signature `Invalid_PCK_CA(uint8)` and selector `0x9849e774`
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
    #[etherror(name = "Invalid_PCK_CA", abi = "Invalid_PCK_CA(uint8)")]
    pub struct Invalid_PCK_CA {
        pub ca: u8,
    }
    ///Custom Error type `Not_An_Admin` with signature `Not_An_Admin(address)` and selector `0x0d7277d9`
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
    #[etherror(name = "Not_An_Admin", abi = "Not_An_Admin(address)")]
    pub struct Not_An_Admin {
        pub caller: ::ethers::core::types::Address,
    }
    ///Custom Error type `Pck_Not_Found` with signature `Pck_Not_Found()` and selector `0x82fba295`
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
    #[etherror(name = "Pck_Not_Found", abi = "Pck_Not_Found()")]
    pub struct Pck_Not_Found;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PckDaoErrors {
        Invalid_PCK_CA(Invalid_PCK_CA),
        Not_An_Admin(Not_An_Admin),
        Pck_Not_Found(Pck_Not_Found),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PckDaoErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Invalid_PCK_CA as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_PCK_CA(decoded));
            }
            if let Ok(decoded) = <Not_An_Admin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Not_An_Admin(decoded));
            }
            if let Ok(decoded) = <Pck_Not_Found as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pck_Not_Found(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PckDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Invalid_PCK_CA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Not_An_Admin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pck_Not_Found(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PckDaoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Invalid_PCK_CA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Not_An_Admin as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <Pck_Not_Found as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PckDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Invalid_PCK_CA(element) => ::core::fmt::Display::fmt(element, f),
                Self::Not_An_Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pck_Not_Found(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PckDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Invalid_PCK_CA> for PckDaoErrors {
        fn from(value: Invalid_PCK_CA) -> Self {
            Self::Invalid_PCK_CA(value)
        }
    }
    impl ::core::convert::From<Not_An_Admin> for PckDaoErrors {
        fn from(value: Not_An_Admin) -> Self {
            Self::Not_An_Admin(value)
        }
    }
    impl ::core::convert::From<Pck_Not_Found> for PckDaoErrors {
        fn from(value: Pck_Not_Found) -> Self {
            Self::Pck_Not_Found(value)
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
    #[ethevent(name = "PCKMissing", abi = "PCKMissing(string,string,string,string)")]
    pub struct PckmissingFilter {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
        pub platform_cpu_svn: ::std::string::String,
        pub platform_pce_svn: ::std::string::String,
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
    #[ethevent(name = "PCKsMissing", abi = "PCKsMissing(string,string)")]
    pub struct PcksMissingFilter {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
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
    #[ethevent(name = "TCBmMissing", abi = "TCBmMissing(string,string,string,string)")]
    pub struct TcbmMissingFilter {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
        pub platform_cpu_svn: ::std::string::String,
        pub platform_pce_svn: ::std::string::String,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PckDaoEvents {
        PckmissingFilter(PckmissingFilter),
        PcksMissingFilter(PcksMissingFilter),
        TcbmMissingFilter(TcbmMissingFilter),
    }
    impl ::ethers::contract::EthLogDecode for PckDaoEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PckmissingFilter::decode_log(log) {
                return Ok(PckDaoEvents::PckmissingFilter(decoded));
            }
            if let Ok(decoded) = PcksMissingFilter::decode_log(log) {
                return Ok(PckDaoEvents::PcksMissingFilter(decoded));
            }
            if let Ok(decoded) = TcbmMissingFilter::decode_log(log) {
                return Ok(PckDaoEvents::TcbmMissingFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PckDaoEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PckmissingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PcksMissingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TcbmMissingFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PckmissingFilter> for PckDaoEvents {
        fn from(value: PckmissingFilter) -> Self {
            Self::PckmissingFilter(value)
        }
    }
    impl ::core::convert::From<PcksMissingFilter> for PckDaoEvents {
        fn from(value: PcksMissingFilter) -> Self {
            Self::PcksMissingFilter(value)
        }
    }
    impl ::core::convert::From<TcbmMissingFilter> for PckDaoEvents {
        fn from(value: TcbmMissingFilter) -> Self {
            Self::TcbmMissingFilter(value)
        }
    }
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
    ///Container type for all input parameters for the `getCert` function with signature `getCert(string,string,string,string)` and selector `0x3c7d7114`
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
    #[ethcall(name = "getCert", abi = "getCert(string,string,string,string)")]
    pub struct GetCertCall {
        pub qeid: ::std::string::String,
        pub platform_cpu_svn: ::std::string::String,
        pub platform_pce_svn: ::std::string::String,
        pub pceid: ::std::string::String,
    }
    ///Container type for all input parameters for the `getCerts` function with signature `getCerts(string,string)` and selector `0x28685839`
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
    #[ethcall(name = "getCerts", abi = "getCerts(string,string)")]
    pub struct GetCertsCall {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
    }
    ///Container type for all input parameters for the `getPckCertChain` function with signature `getPckCertChain(uint8)` and selector `0x48ac8059`
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
    #[ethcall(name = "getPckCertChain", abi = "getPckCertChain(uint8)")]
    pub struct GetPckCertChainCall {
        pub ca: u8,
    }
    ///Container type for all input parameters for the `getPlatformTcbByIdAndSvns` function with signature `getPlatformTcbByIdAndSvns(string,string,string,string)` and selector `0x7eb619c7`
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
        name = "getPlatformTcbByIdAndSvns",
        abi = "getPlatformTcbByIdAndSvns(string,string,string,string)"
    )]
    pub struct GetPlatformTcbByIdAndSvnsCall {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
        pub platform_cpu_svn: ::std::string::String,
        pub platform_pce_svn: ::std::string::String,
    }
    ///Container type for all input parameters for the `pckCertAttestations` function with signature `pckCertAttestations(bytes32)` and selector `0x68ef0c53`
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
    #[ethcall(name = "pckCertAttestations", abi = "pckCertAttestations(bytes32)")]
    pub struct PckCertAttestationsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `pckSchemaID` function with signature `pckSchemaID()` and selector `0x36c33bb6`
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
    #[ethcall(name = "pckSchemaID", abi = "pckSchemaID()")]
    pub struct PckSchemaIDCall;
    ///Container type for all input parameters for the `tcbmAttestations` function with signature `tcbmAttestations(bytes32)` and selector `0xe4a4171a`
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
    #[ethcall(name = "tcbmAttestations", abi = "tcbmAttestations(bytes32)")]
    pub struct TcbmAttestationsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `tcbmSchemaId` function with signature `tcbmSchemaId()` and selector `0xcbd3805e`
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
    #[ethcall(name = "tcbmSchemaId", abi = "tcbmSchemaId()")]
    pub struct TcbmSchemaIdCall;
    ///Container type for all input parameters for the `upsertPckCert` function with signature `upsertPckCert(uint8,string,string,string,bytes)` and selector `0x5be0fa4b`
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
        name = "upsertPckCert",
        abi = "upsertPckCert(uint8,string,string,string,bytes)"
    )]
    pub struct UpsertPckCertCall {
        pub ca: u8,
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
        pub tcbm: ::std::string::String,
        pub cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `upsertPlatformTcbs` function with signature `upsertPlatformTcbs(string,string,string,string,string)` and selector `0xc925d17a`
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
        name = "upsertPlatformTcbs",
        abi = "upsertPlatformTcbs(string,string,string,string,string)"
    )]
    pub struct UpsertPlatformTcbsCall {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
        pub platform_cpu_svn: ::std::string::String,
        pub platform_pce_svn: ::std::string::String,
        pub tcbm: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PckDaoCalls {
        Pcs(PcsCall),
        GetCert(GetCertCall),
        GetCerts(GetCertsCall),
        GetPckCertChain(GetPckCertChainCall),
        GetPlatformTcbByIdAndSvns(GetPlatformTcbByIdAndSvnsCall),
        PckCertAttestations(PckCertAttestationsCall),
        PckSchemaID(PckSchemaIDCall),
        TcbmAttestations(TcbmAttestationsCall),
        TcbmSchemaId(TcbmSchemaIdCall),
        UpsertPckCert(UpsertPckCertCall),
        UpsertPlatformTcbs(UpsertPlatformTcbsCall),
    }
    impl ::ethers::core::abi::AbiDecode for PckDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <PcsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pcs(decoded));
            }
            if let Ok(decoded) = <GetCertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCert(decoded));
            }
            if let Ok(decoded) = <GetCertsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCerts(decoded));
            }
            if let Ok(decoded) = <GetPckCertChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPckCertChain(decoded));
            }
            if let Ok(decoded) = <GetPlatformTcbByIdAndSvnsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPlatformTcbByIdAndSvns(decoded));
            }
            if let Ok(decoded) = <PckCertAttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PckCertAttestations(decoded));
            }
            if let Ok(decoded) = <PckSchemaIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PckSchemaID(decoded));
            }
            if let Ok(decoded) = <TcbmAttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TcbmAttestations(decoded));
            }
            if let Ok(decoded) = <TcbmSchemaIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TcbmSchemaId(decoded));
            }
            if let Ok(decoded) = <UpsertPckCertCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertPckCert(decoded));
            }
            if let Ok(decoded) = <UpsertPlatformTcbsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertPlatformTcbs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PckDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCerts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPckCertChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPlatformTcbByIdAndSvns(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PckCertAttestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PckSchemaID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TcbmAttestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TcbmSchemaId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertPckCert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertPlatformTcbs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PckDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCert(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCerts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPckCertChain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPlatformTcbByIdAndSvns(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PckCertAttestations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PckSchemaID(element) => ::core::fmt::Display::fmt(element, f),
                Self::TcbmAttestations(element) => ::core::fmt::Display::fmt(element, f),
                Self::TcbmSchemaId(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPckCert(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPlatformTcbs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PcsCall> for PckDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<GetCertCall> for PckDaoCalls {
        fn from(value: GetCertCall) -> Self {
            Self::GetCert(value)
        }
    }
    impl ::core::convert::From<GetCertsCall> for PckDaoCalls {
        fn from(value: GetCertsCall) -> Self {
            Self::GetCerts(value)
        }
    }
    impl ::core::convert::From<GetPckCertChainCall> for PckDaoCalls {
        fn from(value: GetPckCertChainCall) -> Self {
            Self::GetPckCertChain(value)
        }
    }
    impl ::core::convert::From<GetPlatformTcbByIdAndSvnsCall> for PckDaoCalls {
        fn from(value: GetPlatformTcbByIdAndSvnsCall) -> Self {
            Self::GetPlatformTcbByIdAndSvns(value)
        }
    }
    impl ::core::convert::From<PckCertAttestationsCall> for PckDaoCalls {
        fn from(value: PckCertAttestationsCall) -> Self {
            Self::PckCertAttestations(value)
        }
    }
    impl ::core::convert::From<PckSchemaIDCall> for PckDaoCalls {
        fn from(value: PckSchemaIDCall) -> Self {
            Self::PckSchemaID(value)
        }
    }
    impl ::core::convert::From<TcbmAttestationsCall> for PckDaoCalls {
        fn from(value: TcbmAttestationsCall) -> Self {
            Self::TcbmAttestations(value)
        }
    }
    impl ::core::convert::From<TcbmSchemaIdCall> for PckDaoCalls {
        fn from(value: TcbmSchemaIdCall) -> Self {
            Self::TcbmSchemaId(value)
        }
    }
    impl ::core::convert::From<UpsertPckCertCall> for PckDaoCalls {
        fn from(value: UpsertPckCertCall) -> Self {
            Self::UpsertPckCert(value)
        }
    }
    impl ::core::convert::From<UpsertPlatformTcbsCall> for PckDaoCalls {
        fn from(value: UpsertPlatformTcbsCall) -> Self {
            Self::UpsertPlatformTcbs(value)
        }
    }
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
    ///Container type for all return fields from the `getCert` function with signature `getCert(string,string,string,string)` and selector `0x3c7d7114`
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
    pub struct GetCertReturn {
        pub pck_cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getCerts` function with signature `getCerts(string,string)` and selector `0x28685839`
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
    pub struct GetCertsReturn {
        pub tcbms: ::std::vec::Vec<::std::string::String>,
        pub pck_certs: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `getPckCertChain` function with signature `getPckCertChain(uint8)` and selector `0x48ac8059`
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
    pub struct GetPckCertChainReturn {
        pub intermediate_cert: ::ethers::core::types::Bytes,
        pub root_cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getPlatformTcbByIdAndSvns` function with signature `getPlatformTcbByIdAndSvns(string,string,string,string)` and selector `0x7eb619c7`
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
    pub struct GetPlatformTcbByIdAndSvnsReturn {
        pub tcbm: ::std::string::String,
    }
    ///Container type for all return fields from the `pckCertAttestations` function with signature `pckCertAttestations(bytes32)` and selector `0x68ef0c53`
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
    pub struct PckCertAttestationsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `pckSchemaID` function with signature `pckSchemaID()` and selector `0x36c33bb6`
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
    pub struct PckSchemaIDReturn {
        pub pck_schema_id: [u8; 32],
    }
    ///Container type for all return fields from the `tcbmAttestations` function with signature `tcbmAttestations(bytes32)` and selector `0xe4a4171a`
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
    pub struct TcbmAttestationsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `tcbmSchemaId` function with signature `tcbmSchemaId()` and selector `0xcbd3805e`
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
    pub struct TcbmSchemaIdReturn {
        pub tcbm_schema_id: [u8; 32],
    }
    ///Container type for all return fields from the `upsertPckCert` function with signature `upsertPckCert(uint8,string,string,string,bytes)` and selector `0x5be0fa4b`
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
    pub struct UpsertPckCertReturn {
        pub attestation_id: [u8; 32],
    }
    ///Container type for all return fields from the `upsertPlatformTcbs` function with signature `upsertPlatformTcbs(string,string,string,string,string)` and selector `0xc925d17a`
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
    pub struct UpsertPlatformTcbsReturn {
        pub attestation_id: [u8; 32],
    }
}
