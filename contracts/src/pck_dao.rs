pub use pck_dao::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod pck_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("P256_VERIFIER"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("P256_VERIFIER"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PCK_KEY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PCK_KEY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("qeidBytes"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(16usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pceidBytes"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(2usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes2"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tcbmBytes"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(18usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes18"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Pcs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("Pcs"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract PcsDao"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_MAPPING_KEY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("TCB_MAPPING_KEY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("qeid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(16usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pceid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(2usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes2"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(16usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(2usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes2"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("crlLib"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("crlLib"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract X509CRLHelper"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAttestedData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAttestedData"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("attestationData"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pckCert"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCerts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCollateralHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCollateralHash"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("collateralHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCollateralValidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCollateralValidity",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("notValidBefore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("notValidAfter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPckCertChain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPckCertChain"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("ca"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum CA"),
                            ),
                        },],
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPlatformTcbByIdAndSvns"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPlatformTcbByIdAndSvns",),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tcbm"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pckLib"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pckLib"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract PCKHelper"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resolver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("resolver"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "contract IDaoAttestationResolver",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upsertPckCert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("attestationId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upsertPlatformTcbs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("x509"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("x509"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UpsertPlatformTcb"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpsertPlatformTcb"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("qeid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(16usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pceid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(2usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("platformCpuSvn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(16usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("platformPceSvn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(2usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tcbm"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(18usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpsertedPckCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpsertedPckCollateral",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("ca"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("qeid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(16usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pceid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(2usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tcbm"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(18usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Certificate_Expired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Certificate_Expired",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Certificate_Revoked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Certificate_Revoked",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("serialNum"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Duplicate_Collateral"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Duplicate_Collateral",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Expired_Certificates"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Expired_Certificates",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_Issuer_Name"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid_Issuer_Name",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_PCK_CA"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid_PCK_CA"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("ca"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum CA"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_Signature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid_Signature"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_Subject_Name"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid_Subject_Name",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Issuer_Expired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Issuer_Expired"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("ca"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum CA"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Issuer_Revoked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Issuer_Revoked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ca"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum CA"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serialNum"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Missing_Issuer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Missing_Issuer"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Pck_Not_Found"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Pck_Not_Found"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Pck_Out_Of_Date"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Pck_Out_Of_Date"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Mismatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TCB_Mismatch"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PCKDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
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
            f.debug_tuple(::core::stringify!(PckDao))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PckDao<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PCKDAO_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `P256_VERIFIER` (0x536c633d) function
        pub fn p256_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([83, 108, 99, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PCK_KEY` (0x20348d8c) function
        pub fn pck_key(
            &self,
            qeid_bytes: [u8; 16],
            pceid_bytes: [u8; 2],
            tcbm_bytes: [u8; 18],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 52, 141, 140], (qeid_bytes, pceid_bytes, tcbm_bytes))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `Pcs` (0xd88d1df6) function
        pub fn pcs(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([216, 141, 29, 246], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TCB_MAPPING_KEY` (0x784aecf0) function
        pub fn tcb_mapping_key(
            &self,
            qeid: [u8; 16],
            pceid: [u8; 2],
            platform_cpu_svn: [u8; 16],
            platform_pce_svn: [u8; 2],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [120, 74, 236, 240],
                    (qeid, pceid, platform_cpu_svn, platform_pce_svn),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crlLib` (0x37b8762d) function
        pub fn crl_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([55, 184, 118, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAttestedData` (0xb414d0b2) function
        pub fn get_attested_data(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([180, 20, 208, 178], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCert` (0x3c7d7114) function
        pub fn get_cert(
            &self,
            qeid: ::std::string::String,
            platform_cpu_svn: ::std::string::String,
            platform_pce_svn: ::std::string::String,
            pceid: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
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
        ///Calls the contract's `getCollateralHash` (0xbf721aaf) function
        pub fn get_collateral_hash(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([191, 114, 26, 175], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCollateralValidity` (0x3e960426) function
        pub fn get_collateral_validity(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([62, 150, 4, 38], key)
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
        ///Calls the contract's `pckLib` (0x59a517ff) function
        pub fn pck_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([89, 165, 23, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resolver` (0x04f3bcec) function
        pub fn resolver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([4, 243, 188, 236], ())
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
        ///Calls the contract's `x509` (0xec950d33) function
        pub fn x_509(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([236, 149, 13, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `UpsertPlatformTcb` event
        pub fn upsert_platform_tcb_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpsertPlatformTcbFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `UpsertedPckCollateral` event
        pub fn upserted_pck_collateral_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpsertedPckCollateralFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PckDaoEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PckDao<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Certificate_Expired` with signature `Certificate_Expired()` and selector `0xdba942a2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Certificate_Expired", abi = "Certificate_Expired()")]
    pub struct Certificate_Expired;
    ///Custom Error type `Certificate_Revoked` with signature `Certificate_Revoked(uint256)` and selector `0x167c231a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Certificate_Revoked", abi = "Certificate_Revoked(uint256)")]
    pub struct Certificate_Revoked {
        pub serial_num: ::ethers::core::types::U256,
    }
    ///Custom Error type `Duplicate_Collateral` with signature `Duplicate_Collateral()` and selector `0x72bd8361`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Duplicate_Collateral", abi = "Duplicate_Collateral()")]
    pub struct Duplicate_Collateral;
    ///Custom Error type `Expired_Certificates` with signature `Expired_Certificates()` and selector `0xe6612a12`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Expired_Certificates", abi = "Expired_Certificates()")]
    pub struct Expired_Certificates;
    ///Custom Error type `Invalid_Issuer_Name` with signature `Invalid_Issuer_Name()` and selector `0x1e7ab599`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Invalid_Issuer_Name", abi = "Invalid_Issuer_Name()")]
    pub struct Invalid_Issuer_Name;
    ///Custom Error type `Invalid_PCK_CA` with signature `Invalid_PCK_CA(uint8)` and selector `0x9849e774`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Invalid_PCK_CA", abi = "Invalid_PCK_CA(uint8)")]
    pub struct Invalid_PCK_CA {
        pub ca: u8,
    }
    ///Custom Error type `Invalid_Signature` with signature `Invalid_Signature()` and selector `0xe7ef341f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Invalid_Signature", abi = "Invalid_Signature()")]
    pub struct Invalid_Signature;
    ///Custom Error type `Invalid_Subject_Name` with signature `Invalid_Subject_Name()` and selector `0x92ec707e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Invalid_Subject_Name", abi = "Invalid_Subject_Name()")]
    pub struct Invalid_Subject_Name;
    ///Custom Error type `Issuer_Expired` with signature `Issuer_Expired(uint8)` and selector `0xa7ee790d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Issuer_Expired", abi = "Issuer_Expired(uint8)")]
    pub struct Issuer_Expired {
        pub ca: u8,
    }
    ///Custom Error type `Issuer_Revoked` with signature `Issuer_Revoked(uint8,uint256)` and selector `0xf465bfb2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Issuer_Revoked", abi = "Issuer_Revoked(uint8,uint256)")]
    pub struct Issuer_Revoked {
        pub ca: u8,
        pub serial_num: ::ethers::core::types::U256,
    }
    ///Custom Error type `Missing_Issuer` with signature `Missing_Issuer()` and selector `0xcd69d374`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Missing_Issuer", abi = "Missing_Issuer()")]
    pub struct Missing_Issuer;
    ///Custom Error type `Pck_Not_Found` with signature `Pck_Not_Found()` and selector `0x82fba295`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Pck_Not_Found", abi = "Pck_Not_Found()")]
    pub struct Pck_Not_Found;
    ///Custom Error type `Pck_Out_Of_Date` with signature `Pck_Out_Of_Date()` and selector `0xbf00a30d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Pck_Out_Of_Date", abi = "Pck_Out_Of_Date()")]
    pub struct Pck_Out_Of_Date;
    ///Custom Error type `TCB_Mismatch` with signature `TCB_Mismatch()` and selector `0x4a629e24`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TCB_Mismatch", abi = "TCB_Mismatch()")]
    pub struct TCB_Mismatch;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PckDaoErrors {
        Certificate_Expired(Certificate_Expired),
        Certificate_Revoked(Certificate_Revoked),
        Duplicate_Collateral(Duplicate_Collateral),
        Expired_Certificates(Expired_Certificates),
        Invalid_Issuer_Name(Invalid_Issuer_Name),
        Invalid_PCK_CA(Invalid_PCK_CA),
        Invalid_Signature(Invalid_Signature),
        Invalid_Subject_Name(Invalid_Subject_Name),
        Issuer_Expired(Issuer_Expired),
        Issuer_Revoked(Issuer_Revoked),
        Missing_Issuer(Missing_Issuer),
        Pck_Not_Found(Pck_Not_Found),
        Pck_Out_Of_Date(Pck_Out_Of_Date),
        TCB_Mismatch(TCB_Mismatch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PckDaoErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <Certificate_Expired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Certificate_Expired(decoded));
            }
            if let Ok(decoded) =
                <Certificate_Revoked as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Certificate_Revoked(decoded));
            }
            if let Ok(decoded) =
                <Duplicate_Collateral as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Duplicate_Collateral(decoded));
            }
            if let Ok(decoded) =
                <Expired_Certificates as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Expired_Certificates(decoded));
            }
            if let Ok(decoded) =
                <Invalid_Issuer_Name as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Invalid_Issuer_Name(decoded));
            }
            if let Ok(decoded) = <Invalid_PCK_CA as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Invalid_PCK_CA(decoded));
            }
            if let Ok(decoded) = <Invalid_Signature as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Invalid_Signature(decoded));
            }
            if let Ok(decoded) =
                <Invalid_Subject_Name as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Invalid_Subject_Name(decoded));
            }
            if let Ok(decoded) = <Issuer_Expired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Issuer_Expired(decoded));
            }
            if let Ok(decoded) = <Issuer_Revoked as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Issuer_Revoked(decoded));
            }
            if let Ok(decoded) = <Missing_Issuer as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Missing_Issuer(decoded));
            }
            if let Ok(decoded) = <Pck_Not_Found as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pck_Not_Found(decoded));
            }
            if let Ok(decoded) = <Pck_Out_Of_Date as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pck_Out_Of_Date(decoded));
            }
            if let Ok(decoded) = <TCB_Mismatch as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TCB_Mismatch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PckDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Certificate_Expired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Certificate_Revoked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Duplicate_Collateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Expired_Certificates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_Issuer_Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_PCK_CA(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Invalid_Signature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Invalid_Subject_Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Issuer_Expired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Issuer_Revoked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Missing_Issuer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pck_Not_Found(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pck_Out_Of_Date(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Mismatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PckDaoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Certificate_Expired as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Certificate_Revoked as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Duplicate_Collateral as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Expired_Certificates as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Invalid_Issuer_Name as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Invalid_PCK_CA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid_Signature as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Invalid_Subject_Name as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Issuer_Expired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Issuer_Revoked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Missing_Issuer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Pck_Not_Found as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Pck_Out_Of_Date as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <TCB_Mismatch as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PckDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Certificate_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::Certificate_Revoked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Duplicate_Collateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::Expired_Certificates(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_Issuer_Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_PCK_CA(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_Signature(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_Subject_Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Issuer_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::Issuer_Revoked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Missing_Issuer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pck_Not_Found(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pck_Out_Of_Date(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Mismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PckDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Certificate_Expired> for PckDaoErrors {
        fn from(value: Certificate_Expired) -> Self {
            Self::Certificate_Expired(value)
        }
    }
    impl ::core::convert::From<Certificate_Revoked> for PckDaoErrors {
        fn from(value: Certificate_Revoked) -> Self {
            Self::Certificate_Revoked(value)
        }
    }
    impl ::core::convert::From<Duplicate_Collateral> for PckDaoErrors {
        fn from(value: Duplicate_Collateral) -> Self {
            Self::Duplicate_Collateral(value)
        }
    }
    impl ::core::convert::From<Expired_Certificates> for PckDaoErrors {
        fn from(value: Expired_Certificates) -> Self {
            Self::Expired_Certificates(value)
        }
    }
    impl ::core::convert::From<Invalid_Issuer_Name> for PckDaoErrors {
        fn from(value: Invalid_Issuer_Name) -> Self {
            Self::Invalid_Issuer_Name(value)
        }
    }
    impl ::core::convert::From<Invalid_PCK_CA> for PckDaoErrors {
        fn from(value: Invalid_PCK_CA) -> Self {
            Self::Invalid_PCK_CA(value)
        }
    }
    impl ::core::convert::From<Invalid_Signature> for PckDaoErrors {
        fn from(value: Invalid_Signature) -> Self {
            Self::Invalid_Signature(value)
        }
    }
    impl ::core::convert::From<Invalid_Subject_Name> for PckDaoErrors {
        fn from(value: Invalid_Subject_Name) -> Self {
            Self::Invalid_Subject_Name(value)
        }
    }
    impl ::core::convert::From<Issuer_Expired> for PckDaoErrors {
        fn from(value: Issuer_Expired) -> Self {
            Self::Issuer_Expired(value)
        }
    }
    impl ::core::convert::From<Issuer_Revoked> for PckDaoErrors {
        fn from(value: Issuer_Revoked) -> Self {
            Self::Issuer_Revoked(value)
        }
    }
    impl ::core::convert::From<Missing_Issuer> for PckDaoErrors {
        fn from(value: Missing_Issuer) -> Self {
            Self::Missing_Issuer(value)
        }
    }
    impl ::core::convert::From<Pck_Not_Found> for PckDaoErrors {
        fn from(value: Pck_Not_Found) -> Self {
            Self::Pck_Not_Found(value)
        }
    }
    impl ::core::convert::From<Pck_Out_Of_Date> for PckDaoErrors {
        fn from(value: Pck_Out_Of_Date) -> Self {
            Self::Pck_Out_Of_Date(value)
        }
    }
    impl ::core::convert::From<TCB_Mismatch> for PckDaoErrors {
        fn from(value: TCB_Mismatch) -> Self {
            Self::TCB_Mismatch(value)
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
        Hash,
    )]
    #[ethevent(
        name = "UpsertPlatformTcb",
        abi = "UpsertPlatformTcb(bytes16,bytes2,bytes16,bytes2,bytes18)"
    )]
    pub struct UpsertPlatformTcbFilter {
        #[ethevent(indexed)]
        pub qeid: [u8; 16],
        #[ethevent(indexed)]
        pub pceid: [u8; 2],
        pub platform_cpu_svn: [u8; 16],
        pub platform_pce_svn: [u8; 2],
        pub tcbm: [u8; 18],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "UpsertedPckCollateral",
        abi = "UpsertedPckCollateral(uint8,bytes16,bytes2,bytes18)"
    )]
    pub struct UpsertedPckCollateralFilter {
        #[ethevent(indexed)]
        pub ca: u8,
        #[ethevent(indexed)]
        pub qeid: [u8; 16],
        #[ethevent(indexed)]
        pub pceid: [u8; 2],
        pub tcbm: [u8; 18],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PckDaoEvents {
        UpsertPlatformTcbFilter(UpsertPlatformTcbFilter),
        UpsertedPckCollateralFilter(UpsertedPckCollateralFilter),
    }
    impl ::ethers::contract::EthLogDecode for PckDaoEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = UpsertPlatformTcbFilter::decode_log(log) {
                return Ok(PckDaoEvents::UpsertPlatformTcbFilter(decoded));
            }
            if let Ok(decoded) = UpsertedPckCollateralFilter::decode_log(log) {
                return Ok(PckDaoEvents::UpsertedPckCollateralFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PckDaoEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::UpsertPlatformTcbFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertedPckCollateralFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<UpsertPlatformTcbFilter> for PckDaoEvents {
        fn from(value: UpsertPlatformTcbFilter) -> Self {
            Self::UpsertPlatformTcbFilter(value)
        }
    }
    impl ::core::convert::From<UpsertedPckCollateralFilter> for PckDaoEvents {
        fn from(value: UpsertedPckCollateralFilter) -> Self {
            Self::UpsertedPckCollateralFilter(value)
        }
    }
    ///Container type for all input parameters for the `P256_VERIFIER` function with signature `P256_VERIFIER()` and selector `0x536c633d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "P256_VERIFIER", abi = "P256_VERIFIER()")]
    pub struct P256VerifierCall;
    ///Container type for all input parameters for the `PCK_KEY` function with signature `PCK_KEY(bytes16,bytes2,bytes18)` and selector `0x20348d8c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "PCK_KEY", abi = "PCK_KEY(bytes16,bytes2,bytes18)")]
    pub struct PckKeyCall {
        pub qeid_bytes: [u8; 16],
        pub pceid_bytes: [u8; 2],
        pub tcbm_bytes: [u8; 18],
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
        Hash,
    )]
    #[ethcall(name = "Pcs", abi = "Pcs()")]
    pub struct PcsCall;
    ///Container type for all input parameters for the `TCB_MAPPING_KEY` function with signature `TCB_MAPPING_KEY(bytes16,bytes2,bytes16,bytes2)` and selector `0x784aecf0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "TCB_MAPPING_KEY",
        abi = "TCB_MAPPING_KEY(bytes16,bytes2,bytes16,bytes2)"
    )]
    pub struct TcbMappingKeyCall {
        pub qeid: [u8; 16],
        pub pceid: [u8; 2],
        pub platform_cpu_svn: [u8; 16],
        pub platform_pce_svn: [u8; 2],
    }
    ///Container type for all input parameters for the `crlLib` function with signature `crlLib()` and selector `0x37b8762d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "crlLib", abi = "crlLib()")]
    pub struct CrlLibCall;
    ///Container type for all input parameters for the `getAttestedData` function with signature `getAttestedData(bytes32)` and selector `0xb414d0b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getAttestedData", abi = "getAttestedData(bytes32)")]
    pub struct GetAttestedDataCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getCert` function with signature `getCert(string,string,string,string)` and selector `0x3c7d7114`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "getCerts", abi = "getCerts(string,string)")]
    pub struct GetCertsCall {
        pub qeid: ::std::string::String,
        pub pceid: ::std::string::String,
    }
    ///Container type for all input parameters for the `getCollateralHash` function with signature `getCollateralHash(bytes32)` and selector `0xbf721aaf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCollateralHash", abi = "getCollateralHash(bytes32)")]
    pub struct GetCollateralHashCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getCollateralValidity` function with signature `getCollateralValidity(bytes32)` and selector `0x3e960426`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCollateralValidity", abi = "getCollateralValidity(bytes32)")]
    pub struct GetCollateralValidityCall {
        pub key: [u8; 32],
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
        Hash,
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
        Hash,
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
    ///Container type for all input parameters for the `pckLib` function with signature `pckLib()` and selector `0x59a517ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pckLib", abi = "pckLib()")]
    pub struct PckLibCall;
    ///Container type for all input parameters for the `resolver` function with signature `resolver()` and selector `0x04f3bcec`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "resolver", abi = "resolver()")]
    pub struct ResolverCall;
    ///Container type for all input parameters for the `upsertPckCert` function with signature `upsertPckCert(uint8,string,string,string,bytes)` and selector `0x5be0fa4b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
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
    ///Container type for all input parameters for the `x509` function with signature `x509()` and selector `0xec950d33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "x509", abi = "x509()")]
    pub struct X509Call;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PckDaoCalls {
        P256Verifier(P256VerifierCall),
        PckKey(PckKeyCall),
        Pcs(PcsCall),
        TcbMappingKey(TcbMappingKeyCall),
        CrlLib(CrlLibCall),
        GetAttestedData(GetAttestedDataCall),
        GetCert(GetCertCall),
        GetCerts(GetCertsCall),
        GetCollateralHash(GetCollateralHashCall),
        GetCollateralValidity(GetCollateralValidityCall),
        GetPckCertChain(GetPckCertChainCall),
        GetPlatformTcbByIdAndSvns(GetPlatformTcbByIdAndSvnsCall),
        PckLib(PckLibCall),
        Resolver(ResolverCall),
        UpsertPckCert(UpsertPckCertCall),
        UpsertPlatformTcbs(UpsertPlatformTcbsCall),
        X509(X509Call),
    }
    impl ::ethers::core::abi::AbiDecode for PckDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <P256VerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::P256Verifier(decoded));
            }
            if let Ok(decoded) = <PckKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PckKey(decoded));
            }
            if let Ok(decoded) = <PcsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pcs(decoded));
            }
            if let Ok(decoded) = <TcbMappingKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TcbMappingKey(decoded));
            }
            if let Ok(decoded) = <CrlLibCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CrlLib(decoded));
            }
            if let Ok(decoded) =
                <GetAttestedDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAttestedData(decoded));
            }
            if let Ok(decoded) = <GetCertCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCert(decoded));
            }
            if let Ok(decoded) = <GetCertsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCerts(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCollateralHash(decoded));
            }
            if let Ok(decoded) =
                <GetCollateralValidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCollateralValidity(decoded));
            }
            if let Ok(decoded) =
                <GetPckCertChainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPckCertChain(decoded));
            }
            if let Ok(decoded) =
                <GetPlatformTcbByIdAndSvnsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPlatformTcbByIdAndSvns(decoded));
            }
            if let Ok(decoded) = <PckLibCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PckLib(decoded));
            }
            if let Ok(decoded) = <ResolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Resolver(decoded));
            }
            if let Ok(decoded) = <UpsertPckCertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpsertPckCert(decoded));
            }
            if let Ok(decoded) =
                <UpsertPlatformTcbsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpsertPlatformTcbs(decoded));
            }
            if let Ok(decoded) = <X509Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::X509(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PckDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::P256Verifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PckKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TcbMappingKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CrlLib(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAttestedData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCerts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollateralHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollateralValidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPckCertChain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPlatformTcbByIdAndSvns(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PckLib(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Resolver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpsertPckCert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpsertPlatformTcbs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::X509(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PckDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::P256Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::PckKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::TcbMappingKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrlLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAttestedData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCert(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCerts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralValidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPckCertChain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPlatformTcbByIdAndSvns(element) => ::core::fmt::Display::fmt(element, f),
                Self::PckLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPckCert(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPlatformTcbs(element) => ::core::fmt::Display::fmt(element, f),
                Self::X509(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<P256VerifierCall> for PckDaoCalls {
        fn from(value: P256VerifierCall) -> Self {
            Self::P256Verifier(value)
        }
    }
    impl ::core::convert::From<PckKeyCall> for PckDaoCalls {
        fn from(value: PckKeyCall) -> Self {
            Self::PckKey(value)
        }
    }
    impl ::core::convert::From<PcsCall> for PckDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<TcbMappingKeyCall> for PckDaoCalls {
        fn from(value: TcbMappingKeyCall) -> Self {
            Self::TcbMappingKey(value)
        }
    }
    impl ::core::convert::From<CrlLibCall> for PckDaoCalls {
        fn from(value: CrlLibCall) -> Self {
            Self::CrlLib(value)
        }
    }
    impl ::core::convert::From<GetAttestedDataCall> for PckDaoCalls {
        fn from(value: GetAttestedDataCall) -> Self {
            Self::GetAttestedData(value)
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
    impl ::core::convert::From<GetCollateralHashCall> for PckDaoCalls {
        fn from(value: GetCollateralHashCall) -> Self {
            Self::GetCollateralHash(value)
        }
    }
    impl ::core::convert::From<GetCollateralValidityCall> for PckDaoCalls {
        fn from(value: GetCollateralValidityCall) -> Self {
            Self::GetCollateralValidity(value)
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
    impl ::core::convert::From<PckLibCall> for PckDaoCalls {
        fn from(value: PckLibCall) -> Self {
            Self::PckLib(value)
        }
    }
    impl ::core::convert::From<ResolverCall> for PckDaoCalls {
        fn from(value: ResolverCall) -> Self {
            Self::Resolver(value)
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
    impl ::core::convert::From<X509Call> for PckDaoCalls {
        fn from(value: X509Call) -> Self {
            Self::X509(value)
        }
    }
    ///Container type for all return fields from the `P256_VERIFIER` function with signature `P256_VERIFIER()` and selector `0x536c633d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct P256VerifierReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `PCK_KEY` function with signature `PCK_KEY(bytes16,bytes2,bytes18)` and selector `0x20348d8c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PckKeyReturn {
        pub key: [u8; 32],
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
        Hash,
    )]
    pub struct PcsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `TCB_MAPPING_KEY` function with signature `TCB_MAPPING_KEY(bytes16,bytes2,bytes16,bytes2)` and selector `0x784aecf0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TcbMappingKeyReturn {
        pub key: [u8; 32],
    }
    ///Container type for all return fields from the `crlLib` function with signature `crlLib()` and selector `0x37b8762d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CrlLibReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAttestedData` function with signature `getAttestedData(bytes32)` and selector `0xb414d0b2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAttestedDataReturn {
        pub attestation_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getCert` function with signature `getCert(string,string,string,string)` and selector `0x3c7d7114`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    pub struct GetCertsReturn {
        pub tcbms: ::std::vec::Vec<::std::string::String>,
        pub pck_certs: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `getCollateralHash` function with signature `getCollateralHash(bytes32)` and selector `0xbf721aaf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCollateralHashReturn {
        pub collateral_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getCollateralValidity` function with signature `getCollateralValidity(bytes32)` and selector `0x3e960426`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCollateralValidityReturn {
        pub not_valid_before: u64,
        pub not_valid_after: u64,
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
        Hash,
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
        Hash,
    )]
    pub struct GetPlatformTcbByIdAndSvnsReturn {
        pub tcbm: ::std::string::String,
    }
    ///Container type for all return fields from the `pckLib` function with signature `pckLib()` and selector `0x59a517ff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PckLibReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `resolver` function with signature `resolver()` and selector `0x04f3bcec`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ResolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `upsertPckCert` function with signature `upsertPckCert(uint8,string,string,string,bytes)` and selector `0x5be0fa4b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    pub struct UpsertPlatformTcbsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `x509` function with signature `x509()` and selector `0xec950d33`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct X509Return(pub ::ethers::core::types::Address);
}
