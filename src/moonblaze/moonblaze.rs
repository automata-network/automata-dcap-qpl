pub use moonblaze::*;
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
pub mod moonblaze {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getCert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qe_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cpu_svn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_svn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("pck_certs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pck_certs"),
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
                                    name: ::std::borrow::ToOwned::to_owned("pck_cert"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("created_time"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updated_time"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("platform_tcbs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("platform_tcbs"),
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
                                    name: ::std::borrow::ToOwned::to_owned("tcbm"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("created_time"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("updated_time"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removePckCert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePckCert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qe_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_id"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removePlatformTcb"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePlatformTcb"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qe_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cpu_svn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_svn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upsertPlatformTcb"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsertPlatformTcb"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qe_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cpu_svn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_svn"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upsetPckCert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsetPckCert"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("qe_id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pce_id"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOONBLAZE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0E$\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c<}q\x14\x11a\0[W\x80c<}q\x14\x14a\0\xD5W\x80ctu\xD2\xBC\x14a\0\xF5W\x80c\x87\xDE\x92\x1F\x14a\x01\x08W\x80c\xB4\x0CB\xED\x14a\x01\x1BW`\0\x80\xFD[\x80c\r\x18p\xD5\x14a\0\x82W\x80c\"m\xFC\xF8\x14a\0\x97W\x80c2\x0C\xB3\x16\x14a\0\xC2W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x08\xF5V[a\x01.V[\0[a\0\xAAa\0\xA56`\x04a\t\xE6V[a\x02\xC3V[`@Qa\0\xB9\x93\x92\x91\x90a\nOV[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\xD06`\x04a\ntV[a\x03mV[a\0\xE8a\0\xE36`\x04a\ntV[a\x03\xDBV[`@Qa\0\xB9\x91\x90a\x0B8V[a\0\x95a\x01\x036`\x04a\x0BRV[a\x06BV[a\0\x95a\x01\x166`\x04a\ntV[a\x06\xABV[a\0\xAAa\x01)6`\x04a\t\xE6V[a\x08;V[`\0\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8A`@Q` \x01a\x01O\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x01\x96\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xC2\x90a\x0C*V[\x80\x15a\x02\x0FW\x80`\x1F\x10a\x01\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x80` \x01Q`\0\x03a\x02AWB` \x82\x01R[\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x85RPPB`@\x80\x85\x01\x91\x90\x91R\x84\x83R` \x83\x90R\x90\x91 \x82Q\x83\x92P\x81\x90a\x02\x9E\x90\x82a\x0C\xC9V[P` \x82\x01Q`\x01\x82\x01U`@\x90\x91\x01Q`\x02\x90\x91\x01UPPPPPPPPPPPPV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T\x81\x90a\x02\xDE\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\n\x90a\x0C*V[\x80\x15a\x03WW\x80`\x1F\x10a\x03,Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03WV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03:W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01T\x90\x80`\x02\x01T\x90P\x83V[`\0\x88\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a\x03\x8E\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xECV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x90\x82 \x90\x92P\x90a\x03\xC0\x82\x82a\x08VV[P`\0`\x01\x82\x01\x81\x90U`\x02\x90\x91\x01UPPPPPPPPPV[```\0\x89\x89\x85\x85\x8B\x8B\x8B\x8B`@Q` \x01a\x03\xFE\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x04E\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04q\x90a\x0C*V[\x80\x15a\x04\xBEW\x80`\x1F\x10a\x04\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x80` \x01Q`\0\x03a\x055W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPlatform TCB does not exist!\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`@Q`\0\x91a\x05Q\x91\x8E\x91\x8E\x91\x8A\x91\x8A\x91` \x01a\r\x89V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x05\x99\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xC5\x90a\x0C*V[\x80\x15a\x06\x12W\x80`\x1F\x10a\x05\xE7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x12V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xF5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x01\x82\x01T` \x82\x01R`\x02\x90\x91\x01T`@\x90\x91\x01RQ\x9D\x9CPPPPPPPPPPPPPV[`\0\x86\x86\x86\x86\x86\x86`@Q` \x01a\x06_\x96\x95\x94\x93\x92\x91\x90a\r\xBEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x01\x90\x93R\x90\x82 \x90\x92P\x90a\x06\x92\x82\x82a\x08VV[P`\0`\x01\x82\x01\x81\x90U`\x02\x90\x91\x01UPPPPPPPV[`\0\x88\x88\x88\x88\x88\x88`@Q` \x01a\x06\xC8\x96\x95\x94\x93\x92\x91\x90a\r\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x07\x10\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07<\x90a\x0C*V[\x80\x15a\x07\x89W\x80`\x1F\x10a\x07^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x80` \x01Q`\0\x03a\x07\xBBWB` \x82\x01R[\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x85RPPB`@\x80\x85\x01\x91\x90\x91R\x84\x83R`\x01` R\x90\x91 \x82Q\x83\x92P\x81\x90a\x08\x18\x90\x82a\x0C\xC9V[P` \x82\x01Q`\x01\x82\x01U`@\x90\x91\x01Q`\x02\x90\x91\x01UPPPPPPPPPPV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x02\xDE\x90a\x0C*V[P\x80Ta\x08b\x90a\x0C*V[`\0\x82U\x80`\x1F\x10a\x08rWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x08\x90\x91\x90a\x08\x93V[PV[[\x80\x82\x11\x15a\x08\xA8W`\0\x81U`\x01\x01a\x08\x94V[P\x90V[`\0\x80\x83`\x1F\x84\x01\x12a\x08\xBEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08\xEEW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x8B\x8D\x03\x12\x15a\t\x14W`\0\x80\xFD[\x8A5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t,W`\0\x80\xFD[a\t8\x8E\x83\x8F\x01a\x08\xACV[\x90\x9CP\x9AP` \x8D\x015\x91P\x80\x82\x11\x15a\tQW`\0\x80\xFD[a\t]\x8E\x83\x8F\x01a\x08\xACV[\x90\x9AP\x98P`@\x8D\x015\x91P\x80\x82\x11\x15a\tvW`\0\x80\xFD[a\t\x82\x8E\x83\x8F\x01a\x08\xACV[\x90\x98P\x96P``\x8D\x015\x91P\x80\x82\x11\x15a\t\x9BW`\0\x80\xFD[a\t\xA7\x8E\x83\x8F\x01a\x08\xACV[\x90\x96P\x94P`\x80\x8D\x015\x91P\x80\x82\x11\x15a\t\xC0W`\0\x80\xFD[Pa\t\xCD\x8D\x82\x8E\x01a\x08\xACV[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a\t\xF8W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\n\x1AW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\x02V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n;\x81` \x86\x01` \x86\x01a\t\xFFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0a\nb``\x83\x01\x86a\n#V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15a\n\x90W`\0\x80\xFD[\x885g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xA8W`\0\x80\xFD[a\n\xB4\x8C\x83\x8D\x01a\x08\xACV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15a\n\xCDW`\0\x80\xFD[a\n\xD9\x8C\x83\x8D\x01a\x08\xACV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15a\n\xF2W`\0\x80\xFD[a\n\xFE\x8C\x83\x8D\x01a\x08\xACV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15a\x0B\x17W`\0\x80\xFD[Pa\x0B$\x8B\x82\x8C\x01a\x08\xACV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[` \x81R`\0a\x0BK` \x83\x01\x84a\n#V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x0BkW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x83W`\0\x80\xFD[a\x0B\x8F\x8A\x83\x8B\x01a\x08\xACV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a\x0B\xA8W`\0\x80\xFD[a\x0B\xB4\x8A\x83\x8B\x01a\x08\xACV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a\x0B\xCDW`\0\x80\xFD[Pa\x0B\xDA\x89\x82\x8A\x01a\x08\xACV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[\x87\x89\x827`\0\x88\x82\x01`\0\x81R\x87\x89\x827`\0\x90\x88\x01\x90\x81R\x85\x87\x827`\0\x90\x86\x01\x90\x81R\x83\x85\x827`\0\x93\x01\x92\x83RP\x90\x98\x97PPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C^WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x0C\xC4W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\xA1WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xC0W\x82\x81U`\x01\x01a\x0C\xADV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xE3Wa\x0C\xE3a\x0CdV[a\x0C\xF7\x81a\x0C\xF1\x84Ta\x0C*V[\x84a\x0CzV[` \x80`\x1F\x83\x11`\x01\x81\x14a\r,W`\0\x84\x15a\r\x14WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xC0V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\r[W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\r<V[P\x85\x82\x10\x15a\ryW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x84\x86\x827`\0\x85\x82\x01`\0\x81R\x84\x86\x827`\0\x90\x85\x01\x90\x81R\x83Qa\r\xB2\x81\x83` \x88\x01a\t\xFFV[\x01\x97\x96PPPPPPPV[\x85\x87\x827`\0\x86\x82\x01`\0\x81R\x85\x87\x827`\0\x90\x86\x01\x90\x81R\x83\x85\x827`\0\x93\x01\x92\x83RP\x90\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xB8+\xBEV\xA4\x07;c/<\xDF \x13\xE9\xCE\"e\xE2\x166\x8B\xB8\x9F\n\xE4[ih\xD9\x92q\x95dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static MOONBLAZE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c<}q\x14\x11a\0[W\x80c<}q\x14\x14a\0\xD5W\x80ctu\xD2\xBC\x14a\0\xF5W\x80c\x87\xDE\x92\x1F\x14a\x01\x08W\x80c\xB4\x0CB\xED\x14a\x01\x1BW`\0\x80\xFD[\x80c\r\x18p\xD5\x14a\0\x82W\x80c\"m\xFC\xF8\x14a\0\x97W\x80c2\x0C\xB3\x16\x14a\0\xC2W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x08\xF5V[a\x01.V[\0[a\0\xAAa\0\xA56`\x04a\t\xE6V[a\x02\xC3V[`@Qa\0\xB9\x93\x92\x91\x90a\nOV[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\xD06`\x04a\ntV[a\x03mV[a\0\xE8a\0\xE36`\x04a\ntV[a\x03\xDBV[`@Qa\0\xB9\x91\x90a\x0B8V[a\0\x95a\x01\x036`\x04a\x0BRV[a\x06BV[a\0\x95a\x01\x166`\x04a\ntV[a\x06\xABV[a\0\xAAa\x01)6`\x04a\t\xE6V[a\x08;V[`\0\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8A`@Q` \x01a\x01O\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x01\x96\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xC2\x90a\x0C*V[\x80\x15a\x02\x0FW\x80`\x1F\x10a\x01\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x80` \x01Q`\0\x03a\x02AWB` \x82\x01R[\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x85RPPB`@\x80\x85\x01\x91\x90\x91R\x84\x83R` \x83\x90R\x90\x91 \x82Q\x83\x92P\x81\x90a\x02\x9E\x90\x82a\x0C\xC9V[P` \x82\x01Q`\x01\x82\x01U`@\x90\x91\x01Q`\x02\x90\x91\x01UPPPPPPPPPPPPV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T\x81\x90a\x02\xDE\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\n\x90a\x0C*V[\x80\x15a\x03WW\x80`\x1F\x10a\x03,Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03WV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03:W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01T\x90\x80`\x02\x01T\x90P\x83V[`\0\x88\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a\x03\x8E\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xECV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R\x92\x83\x90R\x90\x82 \x90\x92P\x90a\x03\xC0\x82\x82a\x08VV[P`\0`\x01\x82\x01\x81\x90U`\x02\x90\x91\x01UPPPPPPPPPV[```\0\x89\x89\x85\x85\x8B\x8B\x8B\x8B`@Q` \x01a\x03\xFE\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xECV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x04E\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04q\x90a\x0C*V[\x80\x15a\x04\xBEW\x80`\x1F\x10a\x04\x93Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x80` \x01Q`\0\x03a\x055W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPlatform TCB does not exist!\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`@Q`\0\x91a\x05Q\x91\x8E\x91\x8E\x91\x8A\x91\x8A\x91` \x01a\r\x89V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x05\x99\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xC5\x90a\x0C*V[\x80\x15a\x06\x12W\x80`\x1F\x10a\x05\xE7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x12V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xF5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x01\x82\x01T` \x82\x01R`\x02\x90\x91\x01T`@\x90\x91\x01RQ\x9D\x9CPPPPPPPPPPPPPV[`\0\x86\x86\x86\x86\x86\x86`@Q` \x01a\x06_\x96\x95\x94\x93\x92\x91\x90a\r\xBEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x01\x90\x93R\x90\x82 \x90\x92P\x90a\x06\x92\x82\x82a\x08VV[P`\0`\x01\x82\x01\x81\x90U`\x02\x90\x91\x01UPPPPPPPV[`\0\x88\x88\x88\x88\x88\x88`@Q` \x01a\x06\xC8\x96\x95\x94\x93\x92\x91\x90a\r\xBEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0`\x01`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80``\x01`@R\x90\x81`\0\x82\x01\x80Ta\x07\x10\x90a\x0C*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07<\x90a\x0C*V[\x80\x15a\x07\x89W\x80`\x1F\x10a\x07^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81RPP\x90P\x80` \x01Q`\0\x03a\x07\xBBWB` \x82\x01R[\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x85RPPB`@\x80\x85\x01\x91\x90\x91R\x84\x83R`\x01` R\x90\x91 \x82Q\x83\x92P\x81\x90a\x08\x18\x90\x82a\x0C\xC9V[P` \x82\x01Q`\x01\x82\x01U`@\x90\x91\x01Q`\x02\x90\x91\x01UPPPPPPPPPPV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x02\xDE\x90a\x0C*V[P\x80Ta\x08b\x90a\x0C*V[`\0\x82U\x80`\x1F\x10a\x08rWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\x08\x90\x91\x90a\x08\x93V[PV[[\x80\x82\x11\x15a\x08\xA8W`\0\x81U`\x01\x01a\x08\x94V[P\x90V[`\0\x80\x83`\x1F\x84\x01\x12a\x08\xBEW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08\xEEW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x8B\x8D\x03\x12\x15a\t\x14W`\0\x80\xFD[\x8A5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t,W`\0\x80\xFD[a\t8\x8E\x83\x8F\x01a\x08\xACV[\x90\x9CP\x9AP` \x8D\x015\x91P\x80\x82\x11\x15a\tQW`\0\x80\xFD[a\t]\x8E\x83\x8F\x01a\x08\xACV[\x90\x9AP\x98P`@\x8D\x015\x91P\x80\x82\x11\x15a\tvW`\0\x80\xFD[a\t\x82\x8E\x83\x8F\x01a\x08\xACV[\x90\x98P\x96P``\x8D\x015\x91P\x80\x82\x11\x15a\t\x9BW`\0\x80\xFD[a\t\xA7\x8E\x83\x8F\x01a\x08\xACV[\x90\x96P\x94P`\x80\x8D\x015\x91P\x80\x82\x11\x15a\t\xC0W`\0\x80\xFD[Pa\t\xCD\x8D\x82\x8E\x01a\x08\xACV[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a\t\xF8W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\n\x1AW\x81\x81\x01Q\x83\x82\x01R` \x01a\n\x02V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\n;\x81` \x86\x01` \x86\x01a\t\xFFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[``\x81R`\0a\nb``\x83\x01\x86a\n#V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15a\n\x90W`\0\x80\xFD[\x885g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xA8W`\0\x80\xFD[a\n\xB4\x8C\x83\x8D\x01a\x08\xACV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15a\n\xCDW`\0\x80\xFD[a\n\xD9\x8C\x83\x8D\x01a\x08\xACV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15a\n\xF2W`\0\x80\xFD[a\n\xFE\x8C\x83\x8D\x01a\x08\xACV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15a\x0B\x17W`\0\x80\xFD[Pa\x0B$\x8B\x82\x8C\x01a\x08\xACV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[` \x81R`\0a\x0BK` \x83\x01\x84a\n#V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x0BkW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x83W`\0\x80\xFD[a\x0B\x8F\x8A\x83\x8B\x01a\x08\xACV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a\x0B\xA8W`\0\x80\xFD[a\x0B\xB4\x8A\x83\x8B\x01a\x08\xACV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a\x0B\xCDW`\0\x80\xFD[Pa\x0B\xDA\x89\x82\x8A\x01a\x08\xACV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[\x87\x89\x827`\0\x88\x82\x01`\0\x81R\x87\x89\x827`\0\x90\x88\x01\x90\x81R\x85\x87\x827`\0\x90\x86\x01\x90\x81R\x83\x85\x827`\0\x93\x01\x92\x83RP\x90\x98\x97PPPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C^WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x0C\xC4W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\xA1WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xC0W\x82\x81U`\x01\x01a\x0C\xADV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xE3Wa\x0C\xE3a\x0CdV[a\x0C\xF7\x81a\x0C\xF1\x84Ta\x0C*V[\x84a\x0CzV[` \x80`\x1F\x83\x11`\x01\x81\x14a\r,W`\0\x84\x15a\r\x14WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xC0V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\r[W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\r<V[P\x85\x82\x10\x15a\ryW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x84\x86\x827`\0\x85\x82\x01`\0\x81R\x84\x86\x827`\0\x90\x85\x01\x90\x81R\x83Qa\r\xB2\x81\x83` \x88\x01a\t\xFFV[\x01\x97\x96PPPPPPPV[\x85\x87\x827`\0\x86\x82\x01`\0\x81R\x85\x87\x827`\0\x90\x86\x01\x90\x81R\x83\x85\x827`\0\x93\x01\x92\x83RP\x90\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xB8+\xBEV\xA4\x07;c/<\xDF \x13\xE9\xCE\"e\xE2\x166\x8B\xB8\x9F\n\xE4[ih\xD9\x92q\x95dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static MOONBLAZE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Moonblaze<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Moonblaze<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Moonblaze<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Moonblaze<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Moonblaze<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Moonblaze)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Moonblaze<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOONBLAZE_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MOONBLAZE_ABI.clone(),
                MOONBLAZE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getCert` (0x3c7d7114) function
        pub fn get_cert(
            &self,
            qe_id: ::std::string::String,
            cpu_svn: ::std::string::String,
            pce_svn: ::std::string::String,
            pce_id: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([60, 125, 113, 20], (qe_id, cpu_svn, pce_svn, pce_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pck_certs` (0xb40c42ed) function
        pub fn pck_certs(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([180, 12, 66, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `platform_tcbs` (0x226dfcf8) function
        pub fn platform_tcbs(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([34, 109, 252, 248], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePckCert` (0x7475d2bc) function
        pub fn remove_pck_cert(
            &self,
            qe_id: ::std::string::String,
            pce_id: ::std::string::String,
            tcbm: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 117, 210, 188], (qe_id, pce_id, tcbm))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePlatformTcb` (0x320cb316) function
        pub fn remove_platform_tcb(
            &self,
            qe_id: ::std::string::String,
            pce_id: ::std::string::String,
            cpu_svn: ::std::string::String,
            pce_svn: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 12, 179, 22], (qe_id, pce_id, cpu_svn, pce_svn))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertPlatformTcb` (0x0d1870d5) function
        pub fn upsert_platform_tcb(
            &self,
            qe_id: ::std::string::String,
            pce_id: ::std::string::String,
            cpu_svn: ::std::string::String,
            pce_svn: ::std::string::String,
            tcbm: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 24, 112, 213], (qe_id, pce_id, cpu_svn, pce_svn, tcbm))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsetPckCert` (0x87de921f) function
        pub fn upset_pck_cert(
            &self,
            qe_id: ::std::string::String,
            pce_id: ::std::string::String,
            tcbm: ::std::string::String,
            cert: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 222, 146, 31], (qe_id, pce_id, tcbm, cert))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Moonblaze<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        Hash
    )]
    #[ethcall(name = "getCert", abi = "getCert(string,string,string,string)")]
    pub struct GetCertCall {
        pub qe_id: ::std::string::String,
        pub cpu_svn: ::std::string::String,
        pub pce_svn: ::std::string::String,
        pub pce_id: ::std::string::String,
    }
    ///Container type for all input parameters for the `pck_certs` function with signature `pck_certs(bytes32)` and selector `0xb40c42ed`
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
    #[ethcall(name = "pck_certs", abi = "pck_certs(bytes32)")]
    pub struct PckCertsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `platform_tcbs` function with signature `platform_tcbs(bytes32)` and selector `0x226dfcf8`
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
    #[ethcall(name = "platform_tcbs", abi = "platform_tcbs(bytes32)")]
    pub struct PlatformTcbsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `removePckCert` function with signature `removePckCert(string,string,string)` and selector `0x7475d2bc`
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
    #[ethcall(name = "removePckCert", abi = "removePckCert(string,string,string)")]
    pub struct RemovePckCertCall {
        pub qe_id: ::std::string::String,
        pub pce_id: ::std::string::String,
        pub tcbm: ::std::string::String,
    }
    ///Container type for all input parameters for the `removePlatformTcb` function with signature `removePlatformTcb(string,string,string,string)` and selector `0x320cb316`
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
        name = "removePlatformTcb",
        abi = "removePlatformTcb(string,string,string,string)"
    )]
    pub struct RemovePlatformTcbCall {
        pub qe_id: ::std::string::String,
        pub pce_id: ::std::string::String,
        pub cpu_svn: ::std::string::String,
        pub pce_svn: ::std::string::String,
    }
    ///Container type for all input parameters for the `upsertPlatformTcb` function with signature `upsertPlatformTcb(string,string,string,string,string)` and selector `0x0d1870d5`
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
        name = "upsertPlatformTcb",
        abi = "upsertPlatformTcb(string,string,string,string,string)"
    )]
    pub struct UpsertPlatformTcbCall {
        pub qe_id: ::std::string::String,
        pub pce_id: ::std::string::String,
        pub cpu_svn: ::std::string::String,
        pub pce_svn: ::std::string::String,
        pub tcbm: ::std::string::String,
    }
    ///Container type for all input parameters for the `upsetPckCert` function with signature `upsetPckCert(string,string,string,bytes)` and selector `0x87de921f`
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
    #[ethcall(name = "upsetPckCert", abi = "upsetPckCert(string,string,string,bytes)")]
    pub struct UpsetPckCertCall {
        pub qe_id: ::std::string::String,
        pub pce_id: ::std::string::String,
        pub tcbm: ::std::string::String,
        pub cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MoonblazeCalls {
        GetCert(GetCertCall),
        PckCerts(PckCertsCall),
        PlatformTcbs(PlatformTcbsCall),
        RemovePckCert(RemovePckCertCall),
        RemovePlatformTcb(RemovePlatformTcbCall),
        UpsertPlatformTcb(UpsertPlatformTcbCall),
        UpsetPckCert(UpsetPckCertCall),
    }
    impl ::ethers::core::abi::AbiDecode for MoonblazeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetCertCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCert(decoded));
            }
            if let Ok(decoded)
                = <PckCertsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PckCerts(decoded));
            }
            if let Ok(decoded)
                = <PlatformTcbsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PlatformTcbs(decoded));
            }
            if let Ok(decoded)
                = <RemovePckCertCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RemovePckCert(decoded));
            }
            if let Ok(decoded)
                = <RemovePlatformTcbCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RemovePlatformTcb(decoded));
            }
            if let Ok(decoded)
                = <UpsertPlatformTcbCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpsertPlatformTcb(decoded));
            }
            if let Ok(decoded)
                = <UpsetPckCertCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpsetPckCert(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MoonblazeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PckCerts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PlatformTcbs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePckCert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePlatformTcb(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertPlatformTcb(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsetPckCert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MoonblazeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCert(element) => ::core::fmt::Display::fmt(element, f),
                Self::PckCerts(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlatformTcbs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePckCert(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePlatformTcb(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPlatformTcb(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsetPckCert(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCertCall> for MoonblazeCalls {
        fn from(value: GetCertCall) -> Self {
            Self::GetCert(value)
        }
    }
    impl ::core::convert::From<PckCertsCall> for MoonblazeCalls {
        fn from(value: PckCertsCall) -> Self {
            Self::PckCerts(value)
        }
    }
    impl ::core::convert::From<PlatformTcbsCall> for MoonblazeCalls {
        fn from(value: PlatformTcbsCall) -> Self {
            Self::PlatformTcbs(value)
        }
    }
    impl ::core::convert::From<RemovePckCertCall> for MoonblazeCalls {
        fn from(value: RemovePckCertCall) -> Self {
            Self::RemovePckCert(value)
        }
    }
    impl ::core::convert::From<RemovePlatformTcbCall> for MoonblazeCalls {
        fn from(value: RemovePlatformTcbCall) -> Self {
            Self::RemovePlatformTcb(value)
        }
    }
    impl ::core::convert::From<UpsertPlatformTcbCall> for MoonblazeCalls {
        fn from(value: UpsertPlatformTcbCall) -> Self {
            Self::UpsertPlatformTcb(value)
        }
    }
    impl ::core::convert::From<UpsetPckCertCall> for MoonblazeCalls {
        fn from(value: UpsetPckCertCall) -> Self {
            Self::UpsetPckCert(value)
        }
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
        Hash
    )]
    pub struct GetCertReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `pck_certs` function with signature `pck_certs(bytes32)` and selector `0xb40c42ed`
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
    pub struct PckCertsReturn {
        pub pck_cert: ::ethers::core::types::Bytes,
        pub created_time: ::ethers::core::types::U256,
        pub updated_time: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `platform_tcbs` function with signature `platform_tcbs(bytes32)` and selector `0x226dfcf8`
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
    pub struct PlatformTcbsReturn {
        pub tcbm: ::std::string::String,
        pub created_time: ::ethers::core::types::U256,
        pub updated_time: ::ethers::core::types::U256,
    }
}
