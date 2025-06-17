pub use tcb_eval_dao::*;
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
pub mod tcb_eval_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("P256_VERIFIER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("P256_VERIFIER"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("TCB_EVAL_KEY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TCB_EVAL_KEY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TcbId"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("TcbEvalLib"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("TcbEvalLib"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract TcbEvalHelper"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("crlLibAddr"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("crlLibAddr"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("early"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("early"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TcbId"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "tcbEvaluationNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAttestedData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAttestedData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                                    name: ::std::borrow::ToOwned::to_owned("attestationData"),
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
                    ::std::borrow::ToOwned::to_owned("getCollateralHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCollateralHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                                    name: ::std::borrow::ToOwned::to_owned("collateralHash"),
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
                    ::std::borrow::ToOwned::to_owned("getCollateralValidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCollateralValidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "issueDateTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nextUpdateTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTcbEvalIssuerChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTcbEvalIssuerChain",
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
                    ::std::borrow::ToOwned::to_owned("getTcbEvaluationDataNumbers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTcbEvaluationDataNumbers",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TcbId"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "tcbEvalDataNumbers",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTcbEvaluationObject"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTcbEvaluationObject",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TcbId"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbEvalObj"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TcbEvalJsonObj"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resolver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resolver"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IDaoAttestationResolver",
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
                    ::std::borrow::ToOwned::to_owned("standard"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("standard"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TcbId"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "tcbEvaluationNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upsertTcbEvaluationData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "upsertTcbEvaluationData",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tcbEvalObj"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct TcbEvalJsonObj"),
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
                    ::std::borrow::ToOwned::to_owned("x509"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("x509"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UpsertedTcbEval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("UpsertedTcbEval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tcbId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Duplicate_Collateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Duplicate_Collateral",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_TCB_Eval_Cert_Signature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Invalid_TCB_Eval_Cert_Signature",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Missing_TCB_Eval_Cert"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Missing_TCB_Eval_Cert",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Eval_Cert_Expired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TCB_Eval_Cert_Expired",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Eval_Cert_Revoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TCB_Eval_Cert_Revoked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("serialNum"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Eval_Expired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TCB_Eval_Expired"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Eval_Missing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TCB_Eval_Missing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TcbId"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Eval_Out_Of_Date"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TCB_Eval_Out_Of_Date",
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
    pub static TCBEVALDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct TcbEvalDao<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TcbEvalDao<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TcbEvalDao<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TcbEvalDao<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TcbEvalDao<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TcbEvalDao)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TcbEvalDao<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TCBEVALDAO_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `P256_VERIFIER` (0x536c633d) function
        pub fn p256_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 108, 99, 61], ())
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
        ///Calls the contract's `TCB_EVAL_KEY` (0x9c381d64) function
        pub fn tcb_eval_key(
            &self,
            id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([156, 56, 29, 100], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TcbEvalLib` (0x6038acb8) function
        pub fn tcb_eval_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([96, 56, 172, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crlLibAddr` (0x37c6d028) function
        pub fn crl_lib_addr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([55, 198, 208, 40], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `early` (0xecae23e6) function
        pub fn early(
            &self,
            id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([236, 174, 35, 230], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAttestedData` (0xb414d0b2) function
        pub fn get_attested_data(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([180, 20, 208, 178], key)
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
        ///Calls the contract's `getTcbEvalIssuerChain` (0x31f92a86) function
        pub fn get_tcb_eval_issuer_chain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([49, 249, 42, 134], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTcbEvaluationDataNumbers` (0x309761c4) function
        pub fn get_tcb_evaluation_data_numbers(
            &self,
            id: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([48, 151, 97, 196], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTcbEvaluationObject` (0xa92bf07d) function
        pub fn get_tcb_evaluation_object(
            &self,
            id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, TcbEvalJsonObj> {
            self.0
                .method_hash([169, 43, 240, 125], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resolver` (0x04f3bcec) function
        pub fn resolver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([4, 243, 188, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `standard` (0x13ec5575) function
        pub fn standard(
            &self,
            id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([19, 236, 85, 117], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertTcbEvaluationData` (0xa6aaf75c) function
        pub fn upsert_tcb_evaluation_data(
            &self,
            tcb_eval_obj: TcbEvalJsonObj,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([166, 170, 247, 92], (tcb_eval_obj,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `x509` (0xec950d33) function
        pub fn x_509(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([236, 149, 13, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `UpsertedTcbEval` event
        pub fn upserted_tcb_eval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpsertedTcbEvalFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpsertedTcbEvalFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TcbEvalDao<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
        Hash
    )]
    #[etherror(name = "Duplicate_Collateral", abi = "Duplicate_Collateral()")]
    pub struct Duplicate_Collateral;
    ///Custom Error type `Invalid_TCB_Eval_Cert_Signature` with signature `Invalid_TCB_Eval_Cert_Signature()` and selector `0xeca8017e`
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
    #[etherror(
        name = "Invalid_TCB_Eval_Cert_Signature",
        abi = "Invalid_TCB_Eval_Cert_Signature()"
    )]
    pub struct Invalid_TCB_Eval_Cert_Signature;
    ///Custom Error type `Missing_TCB_Eval_Cert` with signature `Missing_TCB_Eval_Cert()` and selector `0xc9220efa`
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
    #[etherror(name = "Missing_TCB_Eval_Cert", abi = "Missing_TCB_Eval_Cert()")]
    pub struct Missing_TCB_Eval_Cert;
    ///Custom Error type `TCB_Eval_Cert_Expired` with signature `TCB_Eval_Cert_Expired()` and selector `0x925ca6d8`
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
    #[etherror(name = "TCB_Eval_Cert_Expired", abi = "TCB_Eval_Cert_Expired()")]
    pub struct TCB_Eval_Cert_Expired;
    ///Custom Error type `TCB_Eval_Cert_Revoked` with signature `TCB_Eval_Cert_Revoked(uint256)` and selector `0x49c53e1e`
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
    #[etherror(name = "TCB_Eval_Cert_Revoked", abi = "TCB_Eval_Cert_Revoked(uint256)")]
    pub struct TCB_Eval_Cert_Revoked {
        pub serial_num: ::ethers::core::types::U256,
    }
    ///Custom Error type `TCB_Eval_Expired` with signature `TCB_Eval_Expired()` and selector `0xc750d267`
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
    #[etherror(name = "TCB_Eval_Expired", abi = "TCB_Eval_Expired()")]
    pub struct TCB_Eval_Expired;
    ///Custom Error type `TCB_Eval_Missing` with signature `TCB_Eval_Missing(uint8)` and selector `0xfe17888f`
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
    #[etherror(name = "TCB_Eval_Missing", abi = "TCB_Eval_Missing(uint8)")]
    pub struct TCB_Eval_Missing {
        pub id: u8,
    }
    ///Custom Error type `TCB_Eval_Out_Of_Date` with signature `TCB_Eval_Out_Of_Date()` and selector `0x9ddee474`
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
    #[etherror(name = "TCB_Eval_Out_Of_Date", abi = "TCB_Eval_Out_Of_Date()")]
    pub struct TCB_Eval_Out_Of_Date;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TcbEvalDaoErrors {
        Duplicate_Collateral(Duplicate_Collateral),
        Invalid_TCB_Eval_Cert_Signature(Invalid_TCB_Eval_Cert_Signature),
        Missing_TCB_Eval_Cert(Missing_TCB_Eval_Cert),
        TCB_Eval_Cert_Expired(TCB_Eval_Cert_Expired),
        TCB_Eval_Cert_Revoked(TCB_Eval_Cert_Revoked),
        TCB_Eval_Expired(TCB_Eval_Expired),
        TCB_Eval_Missing(TCB_Eval_Missing),
        TCB_Eval_Out_Of_Date(TCB_Eval_Out_Of_Date),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TcbEvalDaoErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Duplicate_Collateral as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Duplicate_Collateral(decoded));
            }
            if let Ok(decoded) = <Invalid_TCB_Eval_Cert_Signature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_TCB_Eval_Cert_Signature(decoded));
            }
            if let Ok(decoded) = <Missing_TCB_Eval_Cert as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Missing_TCB_Eval_Cert(decoded));
            }
            if let Ok(decoded) = <TCB_Eval_Cert_Expired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TCB_Eval_Cert_Expired(decoded));
            }
            if let Ok(decoded) = <TCB_Eval_Cert_Revoked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TCB_Eval_Cert_Revoked(decoded));
            }
            if let Ok(decoded) = <TCB_Eval_Expired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TCB_Eval_Expired(decoded));
            }
            if let Ok(decoded) = <TCB_Eval_Missing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TCB_Eval_Missing(decoded));
            }
            if let Ok(decoded) = <TCB_Eval_Out_Of_Date as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TCB_Eval_Out_Of_Date(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TcbEvalDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Duplicate_Collateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_TCB_Eval_Cert_Signature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Missing_TCB_Eval_Cert(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TCB_Eval_Cert_Expired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TCB_Eval_Cert_Revoked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TCB_Eval_Expired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TCB_Eval_Missing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TCB_Eval_Out_Of_Date(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for TcbEvalDaoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Duplicate_Collateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid_TCB_Eval_Cert_Signature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Missing_TCB_Eval_Cert as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TCB_Eval_Cert_Expired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TCB_Eval_Cert_Revoked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TCB_Eval_Expired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TCB_Eval_Missing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TCB_Eval_Out_Of_Date as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for TcbEvalDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Duplicate_Collateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid_TCB_Eval_Cert_Signature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Missing_TCB_Eval_Cert(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TCB_Eval_Cert_Expired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TCB_Eval_Cert_Revoked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TCB_Eval_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Eval_Missing(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Eval_Out_Of_Date(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for TcbEvalDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Duplicate_Collateral> for TcbEvalDaoErrors {
        fn from(value: Duplicate_Collateral) -> Self {
            Self::Duplicate_Collateral(value)
        }
    }
    impl ::core::convert::From<Invalid_TCB_Eval_Cert_Signature> for TcbEvalDaoErrors {
        fn from(value: Invalid_TCB_Eval_Cert_Signature) -> Self {
            Self::Invalid_TCB_Eval_Cert_Signature(value)
        }
    }
    impl ::core::convert::From<Missing_TCB_Eval_Cert> for TcbEvalDaoErrors {
        fn from(value: Missing_TCB_Eval_Cert) -> Self {
            Self::Missing_TCB_Eval_Cert(value)
        }
    }
    impl ::core::convert::From<TCB_Eval_Cert_Expired> for TcbEvalDaoErrors {
        fn from(value: TCB_Eval_Cert_Expired) -> Self {
            Self::TCB_Eval_Cert_Expired(value)
        }
    }
    impl ::core::convert::From<TCB_Eval_Cert_Revoked> for TcbEvalDaoErrors {
        fn from(value: TCB_Eval_Cert_Revoked) -> Self {
            Self::TCB_Eval_Cert_Revoked(value)
        }
    }
    impl ::core::convert::From<TCB_Eval_Expired> for TcbEvalDaoErrors {
        fn from(value: TCB_Eval_Expired) -> Self {
            Self::TCB_Eval_Expired(value)
        }
    }
    impl ::core::convert::From<TCB_Eval_Missing> for TcbEvalDaoErrors {
        fn from(value: TCB_Eval_Missing) -> Self {
            Self::TCB_Eval_Missing(value)
        }
    }
    impl ::core::convert::From<TCB_Eval_Out_Of_Date> for TcbEvalDaoErrors {
        fn from(value: TCB_Eval_Out_Of_Date) -> Self {
            Self::TCB_Eval_Out_Of_Date(value)
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
    #[ethevent(name = "UpsertedTcbEval", abi = "UpsertedTcbEval(uint8)")]
    pub struct UpsertedTcbEvalFilter {
        #[ethevent(indexed)]
        pub tcb_id: u8,
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
        Hash
    )]
    #[ethcall(name = "P256_VERIFIER", abi = "P256_VERIFIER()")]
    pub struct P256VerifierCall;
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
    ///Container type for all input parameters for the `TCB_EVAL_KEY` function with signature `TCB_EVAL_KEY(uint8)` and selector `0x9c381d64`
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
    #[ethcall(name = "TCB_EVAL_KEY", abi = "TCB_EVAL_KEY(uint8)")]
    pub struct TcbEvalKeyCall {
        pub id: u8,
    }
    ///Container type for all input parameters for the `TcbEvalLib` function with signature `TcbEvalLib()` and selector `0x6038acb8`
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
    #[ethcall(name = "TcbEvalLib", abi = "TcbEvalLib()")]
    pub struct TcbEvalLibCall;
    ///Container type for all input parameters for the `crlLibAddr` function with signature `crlLibAddr()` and selector `0x37c6d028`
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
    #[ethcall(name = "crlLibAddr", abi = "crlLibAddr()")]
    pub struct CrlLibAddrCall;
    ///Container type for all input parameters for the `early` function with signature `early(uint8)` and selector `0xecae23e6`
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
    #[ethcall(name = "early", abi = "early(uint8)")]
    pub struct EarlyCall {
        pub id: u8,
    }
    ///Container type for all input parameters for the `getAttestedData` function with signature `getAttestedData(bytes32)` and selector `0xb414d0b2`
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
    #[ethcall(name = "getAttestedData", abi = "getAttestedData(bytes32)")]
    pub struct GetAttestedDataCall {
        pub key: [u8; 32],
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "getCollateralValidity", abi = "getCollateralValidity(bytes32)")]
    pub struct GetCollateralValidityCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getTcbEvalIssuerChain` function with signature `getTcbEvalIssuerChain()` and selector `0x31f92a86`
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
    #[ethcall(name = "getTcbEvalIssuerChain", abi = "getTcbEvalIssuerChain()")]
    pub struct GetTcbEvalIssuerChainCall;
    ///Container type for all input parameters for the `getTcbEvaluationDataNumbers` function with signature `getTcbEvaluationDataNumbers(uint8)` and selector `0x309761c4`
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
        name = "getTcbEvaluationDataNumbers",
        abi = "getTcbEvaluationDataNumbers(uint8)"
    )]
    pub struct GetTcbEvaluationDataNumbersCall {
        pub id: u8,
    }
    ///Container type for all input parameters for the `getTcbEvaluationObject` function with signature `getTcbEvaluationObject(uint8)` and selector `0xa92bf07d`
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
    #[ethcall(name = "getTcbEvaluationObject", abi = "getTcbEvaluationObject(uint8)")]
    pub struct GetTcbEvaluationObjectCall {
        pub id: u8,
    }
    ///Container type for all input parameters for the `resolver` function with signature `resolver()` and selector `0x04f3bcec`
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
    #[ethcall(name = "resolver", abi = "resolver()")]
    pub struct ResolverCall;
    ///Container type for all input parameters for the `standard` function with signature `standard(uint8)` and selector `0x13ec5575`
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
    #[ethcall(name = "standard", abi = "standard(uint8)")]
    pub struct StandardCall {
        pub id: u8,
    }
    ///Container type for all input parameters for the `upsertTcbEvaluationData` function with signature `upsertTcbEvaluationData((string,bytes))` and selector `0xa6aaf75c`
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
        name = "upsertTcbEvaluationData",
        abi = "upsertTcbEvaluationData((string,bytes))"
    )]
    pub struct UpsertTcbEvaluationDataCall {
        pub tcb_eval_obj: TcbEvalJsonObj,
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
        Hash
    )]
    #[ethcall(name = "x509", abi = "x509()")]
    pub struct X509Call;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TcbEvalDaoCalls {
        P256Verifier(P256VerifierCall),
        Pcs(PcsCall),
        TcbEvalKey(TcbEvalKeyCall),
        TcbEvalLib(TcbEvalLibCall),
        CrlLibAddr(CrlLibAddrCall),
        Early(EarlyCall),
        GetAttestedData(GetAttestedDataCall),
        GetCollateralHash(GetCollateralHashCall),
        GetCollateralValidity(GetCollateralValidityCall),
        GetTcbEvalIssuerChain(GetTcbEvalIssuerChainCall),
        GetTcbEvaluationDataNumbers(GetTcbEvaluationDataNumbersCall),
        GetTcbEvaluationObject(GetTcbEvaluationObjectCall),
        Resolver(ResolverCall),
        Standard(StandardCall),
        UpsertTcbEvaluationData(UpsertTcbEvaluationDataCall),
        X509(X509Call),
    }
    impl ::ethers::core::abi::AbiDecode for TcbEvalDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <P256VerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::P256Verifier(decoded));
            }
            if let Ok(decoded) = <PcsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pcs(decoded));
            }
            if let Ok(decoded) = <TcbEvalKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TcbEvalKey(decoded));
            }
            if let Ok(decoded) = <TcbEvalLibCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TcbEvalLib(decoded));
            }
            if let Ok(decoded) = <CrlLibAddrCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CrlLibAddr(decoded));
            }
            if let Ok(decoded) = <EarlyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Early(decoded));
            }
            if let Ok(decoded) = <GetAttestedDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAttestedData(decoded));
            }
            if let Ok(decoded) = <GetCollateralHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCollateralHash(decoded));
            }
            if let Ok(decoded) = <GetCollateralValidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCollateralValidity(decoded));
            }
            if let Ok(decoded) = <GetTcbEvalIssuerChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTcbEvalIssuerChain(decoded));
            }
            if let Ok(decoded) = <GetTcbEvaluationDataNumbersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTcbEvaluationDataNumbers(decoded));
            }
            if let Ok(decoded) = <GetTcbEvaluationObjectCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTcbEvaluationObject(decoded));
            }
            if let Ok(decoded) = <ResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Resolver(decoded));
            }
            if let Ok(decoded) = <StandardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Standard(decoded));
            }
            if let Ok(decoded) = <UpsertTcbEvaluationDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertTcbEvaluationData(decoded));
            }
            if let Ok(decoded) = <X509Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::X509(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TcbEvalDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::P256Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TcbEvalKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TcbEvalLib(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CrlLibAddr(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Early(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAttestedData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollateralHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollateralValidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbEvalIssuerChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbEvaluationDataNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbEvaluationObject(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Standard(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertTcbEvaluationData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::X509(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TcbEvalDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::P256Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::TcbEvalKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::TcbEvalLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrlLibAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::Early(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAttestedData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralValidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTcbEvalIssuerChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTcbEvaluationDataNumbers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTcbEvaluationObject(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Resolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Standard(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertTcbEvaluationData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::X509(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<P256VerifierCall> for TcbEvalDaoCalls {
        fn from(value: P256VerifierCall) -> Self {
            Self::P256Verifier(value)
        }
    }
    impl ::core::convert::From<PcsCall> for TcbEvalDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<TcbEvalKeyCall> for TcbEvalDaoCalls {
        fn from(value: TcbEvalKeyCall) -> Self {
            Self::TcbEvalKey(value)
        }
    }
    impl ::core::convert::From<TcbEvalLibCall> for TcbEvalDaoCalls {
        fn from(value: TcbEvalLibCall) -> Self {
            Self::TcbEvalLib(value)
        }
    }
    impl ::core::convert::From<CrlLibAddrCall> for TcbEvalDaoCalls {
        fn from(value: CrlLibAddrCall) -> Self {
            Self::CrlLibAddr(value)
        }
    }
    impl ::core::convert::From<EarlyCall> for TcbEvalDaoCalls {
        fn from(value: EarlyCall) -> Self {
            Self::Early(value)
        }
    }
    impl ::core::convert::From<GetAttestedDataCall> for TcbEvalDaoCalls {
        fn from(value: GetAttestedDataCall) -> Self {
            Self::GetAttestedData(value)
        }
    }
    impl ::core::convert::From<GetCollateralHashCall> for TcbEvalDaoCalls {
        fn from(value: GetCollateralHashCall) -> Self {
            Self::GetCollateralHash(value)
        }
    }
    impl ::core::convert::From<GetCollateralValidityCall> for TcbEvalDaoCalls {
        fn from(value: GetCollateralValidityCall) -> Self {
            Self::GetCollateralValidity(value)
        }
    }
    impl ::core::convert::From<GetTcbEvalIssuerChainCall> for TcbEvalDaoCalls {
        fn from(value: GetTcbEvalIssuerChainCall) -> Self {
            Self::GetTcbEvalIssuerChain(value)
        }
    }
    impl ::core::convert::From<GetTcbEvaluationDataNumbersCall> for TcbEvalDaoCalls {
        fn from(value: GetTcbEvaluationDataNumbersCall) -> Self {
            Self::GetTcbEvaluationDataNumbers(value)
        }
    }
    impl ::core::convert::From<GetTcbEvaluationObjectCall> for TcbEvalDaoCalls {
        fn from(value: GetTcbEvaluationObjectCall) -> Self {
            Self::GetTcbEvaluationObject(value)
        }
    }
    impl ::core::convert::From<ResolverCall> for TcbEvalDaoCalls {
        fn from(value: ResolverCall) -> Self {
            Self::Resolver(value)
        }
    }
    impl ::core::convert::From<StandardCall> for TcbEvalDaoCalls {
        fn from(value: StandardCall) -> Self {
            Self::Standard(value)
        }
    }
    impl ::core::convert::From<UpsertTcbEvaluationDataCall> for TcbEvalDaoCalls {
        fn from(value: UpsertTcbEvaluationDataCall) -> Self {
            Self::UpsertTcbEvaluationData(value)
        }
    }
    impl ::core::convert::From<X509Call> for TcbEvalDaoCalls {
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
        Hash
    )]
    pub struct P256VerifierReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `TCB_EVAL_KEY` function with signature `TCB_EVAL_KEY(uint8)` and selector `0x9c381d64`
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
    pub struct TcbEvalKeyReturn {
        pub key: [u8; 32],
    }
    ///Container type for all return fields from the `TcbEvalLib` function with signature `TcbEvalLib()` and selector `0x6038acb8`
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
    pub struct TcbEvalLibReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `crlLibAddr` function with signature `crlLibAddr()` and selector `0x37c6d028`
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
    pub struct CrlLibAddrReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `early` function with signature `early(uint8)` and selector `0xecae23e6`
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
    pub struct EarlyReturn {
        pub tcb_evaluation_number: u32,
    }
    ///Container type for all return fields from the `getAttestedData` function with signature `getAttestedData(bytes32)` and selector `0xb414d0b2`
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
    pub struct GetAttestedDataReturn {
        pub attestation_data: ::ethers::core::types::Bytes,
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
        Hash
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
        Hash
    )]
    pub struct GetCollateralValidityReturn {
        pub issue_date_timestamp: u64,
        pub next_update_timestamp: u64,
    }
    ///Container type for all return fields from the `getTcbEvalIssuerChain` function with signature `getTcbEvalIssuerChain()` and selector `0x31f92a86`
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
    pub struct GetTcbEvalIssuerChainReturn {
        pub signing_cert: ::ethers::core::types::Bytes,
        pub root_cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getTcbEvaluationDataNumbers` function with signature `getTcbEvaluationDataNumbers(uint8)` and selector `0x309761c4`
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
    pub struct GetTcbEvaluationDataNumbersReturn {
        pub tcb_eval_data_numbers: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getTcbEvaluationObject` function with signature `getTcbEvaluationObject(uint8)` and selector `0xa92bf07d`
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
    pub struct GetTcbEvaluationObjectReturn {
        pub tcb_eval_obj: TcbEvalJsonObj,
    }
    ///Container type for all return fields from the `resolver` function with signature `resolver()` and selector `0x04f3bcec`
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
    pub struct ResolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `standard` function with signature `standard(uint8)` and selector `0x13ec5575`
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
    pub struct StandardReturn {
        pub tcb_evaluation_number: u32,
    }
    ///Container type for all return fields from the `upsertTcbEvaluationData` function with signature `upsertTcbEvaluationData((string,bytes))` and selector `0xa6aaf75c`
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
    pub struct UpsertTcbEvaluationDataReturn {
        pub attestation_id: [u8; 32],
    }
    ///Container type for all return fields from the `x509` function with signature `x509()` and selector `0xec950d33`
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
    pub struct X509Return(pub ::ethers::core::types::Address);
    ///`TcbEvalJsonObj(string,bytes)`
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
    pub struct TcbEvalJsonObj {
        pub tcb_evaluation_data_numbers: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
}
