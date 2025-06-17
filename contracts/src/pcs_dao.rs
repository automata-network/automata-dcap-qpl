pub use pcs_dao::*;
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
pub mod pcs_dao {
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
                    ::std::borrow::ToOwned::to_owned("PCS_KEY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PCS_KEY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isCrl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("crlLib"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("crlLib"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract X509CRLHelper"),
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
                    ::std::borrow::ToOwned::to_owned("getCertificateById"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCertificateById"),
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
                                    name: ::std::borrow::ToOwned::to_owned("cert"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("crl"),
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
                    ::std::borrow::ToOwned::to_owned("upsertPckCrl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsertPckCrl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("crl"),
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
                    ::std::borrow::ToOwned::to_owned("upsertPcsCertificates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "upsertPcsCertificates",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
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
                    ::std::borrow::ToOwned::to_owned("upsertRootCACrl"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upsertRootCACrl"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootcacrl"),
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
                    ::std::borrow::ToOwned::to_owned("UpsertedPCSCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UpsertedPCSCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("ca"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isCrl"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
                    ::std::borrow::ToOwned::to_owned("Certificate_Expired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Certificate_Expired",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("Certificate_Out_Of_Date"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Certificate_Out_Of_Date",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Certificate_Revoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Certificate_Revoked",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("Crl_Expired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Crl_Expired"),
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
                    ::std::borrow::ToOwned::to_owned("Expired_Certificates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Expired_Certificates",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_Issuer_Name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Invalid_Issuer_Name",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("Invalid_Signature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid_Signature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_Subject_Name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Invalid_Subject_Name",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Missing_Certificate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Missing_Certificate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("Missing_Issuer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Missing_Issuer"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Root_Key_Mismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Root_Key_Mismatch"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Mismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TCB_Mismatch"),
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
    pub static PCSDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct PcsDao<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PcsDao<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PcsDao<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PcsDao<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PcsDao<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PcsDao)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PcsDao<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PCSDAO_ABI.clone(),
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
        ///Calls the contract's `PCS_KEY` (0xb13bf290) function
        pub fn pcs_key(
            &self,
            ca: u8,
            is_crl: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([177, 59, 242, 144], (ca, is_crl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `crlLib` (0x37b8762d) function
        pub fn crl_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([55, 184, 118, 45], ())
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
        ///Calls the contract's `getCertificateById` (0x722f1327) function
        pub fn get_certificate_by_id(
            &self,
            ca: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([114, 47, 19, 39], ca)
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
        ///Calls the contract's `upsertPckCrl` (0x08854e04) function
        pub fn upsert_pck_crl(
            &self,
            ca: u8,
            crl: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 133, 78, 4], (ca, crl))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertPcsCertificates` (0x3b395455) function
        pub fn upsert_pcs_certificates(
            &self,
            ca: u8,
            cert: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([59, 57, 84, 85], (ca, cert))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upsertRootCACrl` (0x6b1c5399) function
        pub fn upsert_root_ca_crl(
            &self,
            rootcacrl: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([107, 28, 83, 153], rootcacrl)
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
        ///Gets the contract's `UpsertedPCSCollateral` event
        pub fn upserted_pcs_collateral_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpsertedPCSCollateralFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpsertedPCSCollateralFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PcsDao<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Certificate_Expired` with signature `Certificate_Expired(uint8)` and selector `0x5f066611`
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
    #[etherror(name = "Certificate_Expired", abi = "Certificate_Expired(uint8)")]
    pub struct Certificate_Expired {
        pub ca: u8,
    }
    ///Custom Error type `Certificate_Out_Of_Date` with signature `Certificate_Out_Of_Date()` and selector `0x9f4daa9e`
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
    #[etherror(name = "Certificate_Out_Of_Date", abi = "Certificate_Out_Of_Date()")]
    pub struct Certificate_Out_Of_Date;
    ///Custom Error type `Certificate_Revoked` with signature `Certificate_Revoked(uint8,uint256)` and selector `0x291990cd`
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
    #[etherror(name = "Certificate_Revoked", abi = "Certificate_Revoked(uint8,uint256)")]
    pub struct Certificate_Revoked {
        pub ca: u8,
        pub serial_num: ::ethers::core::types::U256,
    }
    ///Custom Error type `Crl_Expired` with signature `Crl_Expired(uint8)` and selector `0x6d8932ad`
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
    #[etherror(name = "Crl_Expired", abi = "Crl_Expired(uint8)")]
    pub struct Crl_Expired {
        pub ca: u8,
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
    ///Custom Error type `Expired_Certificates` with signature `Expired_Certificates()` and selector `0xe6612a12`
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[etherror(name = "Invalid_Subject_Name", abi = "Invalid_Subject_Name()")]
    pub struct Invalid_Subject_Name;
    ///Custom Error type `Missing_Certificate` with signature `Missing_Certificate(uint8)` and selector `0x33247a8a`
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
    #[etherror(name = "Missing_Certificate", abi = "Missing_Certificate(uint8)")]
    pub struct Missing_Certificate {
        pub ca: u8,
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
        Hash
    )]
    #[etherror(name = "Missing_Issuer", abi = "Missing_Issuer()")]
    pub struct Missing_Issuer;
    ///Custom Error type `Root_Key_Mismatch` with signature `Root_Key_Mismatch()` and selector `0xe1406f79`
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
    #[etherror(name = "Root_Key_Mismatch", abi = "Root_Key_Mismatch()")]
    pub struct Root_Key_Mismatch;
    ///Custom Error type `TCB_Mismatch` with signature `TCB_Mismatch()` and selector `0x4a629e24`
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
    #[etherror(name = "TCB_Mismatch", abi = "TCB_Mismatch()")]
    pub struct TCB_Mismatch;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PcsDaoErrors {
        Certificate_Expired(Certificate_Expired),
        Certificate_Out_Of_Date(Certificate_Out_Of_Date),
        Certificate_Revoked(Certificate_Revoked),
        Crl_Expired(Crl_Expired),
        Duplicate_Collateral(Duplicate_Collateral),
        Expired_Certificates(Expired_Certificates),
        Invalid_Issuer_Name(Invalid_Issuer_Name),
        Invalid_PCK_CA(Invalid_PCK_CA),
        Invalid_Signature(Invalid_Signature),
        Invalid_Subject_Name(Invalid_Subject_Name),
        Missing_Certificate(Missing_Certificate),
        Missing_Issuer(Missing_Issuer),
        Root_Key_Mismatch(Root_Key_Mismatch),
        TCB_Mismatch(TCB_Mismatch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PcsDaoErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Certificate_Expired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Certificate_Expired(decoded));
            }
            if let Ok(decoded) = <Certificate_Out_Of_Date as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Certificate_Out_Of_Date(decoded));
            }
            if let Ok(decoded) = <Certificate_Revoked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Certificate_Revoked(decoded));
            }
            if let Ok(decoded) = <Crl_Expired as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Crl_Expired(decoded));
            }
            if let Ok(decoded) = <Duplicate_Collateral as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Duplicate_Collateral(decoded));
            }
            if let Ok(decoded) = <Expired_Certificates as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Expired_Certificates(decoded));
            }
            if let Ok(decoded) = <Invalid_Issuer_Name as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_Issuer_Name(decoded));
            }
            if let Ok(decoded) = <Invalid_PCK_CA as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_PCK_CA(decoded));
            }
            if let Ok(decoded) = <Invalid_Signature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_Signature(decoded));
            }
            if let Ok(decoded) = <Invalid_Subject_Name as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_Subject_Name(decoded));
            }
            if let Ok(decoded) = <Missing_Certificate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Missing_Certificate(decoded));
            }
            if let Ok(decoded) = <Missing_Issuer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Missing_Issuer(decoded));
            }
            if let Ok(decoded) = <Root_Key_Mismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Root_Key_Mismatch(decoded));
            }
            if let Ok(decoded) = <TCB_Mismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TCB_Mismatch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PcsDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Certificate_Expired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Certificate_Out_Of_Date(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Certificate_Revoked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Crl_Expired(element) => {
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
                Self::Invalid_PCK_CA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_Signature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_Subject_Name(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Missing_Certificate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Missing_Issuer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Root_Key_Mismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TCB_Mismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PcsDaoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Certificate_Expired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Certificate_Out_Of_Date as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Certificate_Revoked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Crl_Expired as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <Duplicate_Collateral as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Expired_Certificates as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid_Issuer_Name as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid_PCK_CA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid_Signature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid_Subject_Name as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Missing_Certificate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Missing_Issuer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Root_Key_Mismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TCB_Mismatch as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PcsDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Certificate_Expired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Certificate_Out_Of_Date(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Certificate_Revoked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Crl_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::Duplicate_Collateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Expired_Certificates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid_Issuer_Name(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid_PCK_CA(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_Signature(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_Subject_Name(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Missing_Certificate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Missing_Issuer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Root_Key_Mismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Mismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PcsDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Certificate_Expired> for PcsDaoErrors {
        fn from(value: Certificate_Expired) -> Self {
            Self::Certificate_Expired(value)
        }
    }
    impl ::core::convert::From<Certificate_Out_Of_Date> for PcsDaoErrors {
        fn from(value: Certificate_Out_Of_Date) -> Self {
            Self::Certificate_Out_Of_Date(value)
        }
    }
    impl ::core::convert::From<Certificate_Revoked> for PcsDaoErrors {
        fn from(value: Certificate_Revoked) -> Self {
            Self::Certificate_Revoked(value)
        }
    }
    impl ::core::convert::From<Crl_Expired> for PcsDaoErrors {
        fn from(value: Crl_Expired) -> Self {
            Self::Crl_Expired(value)
        }
    }
    impl ::core::convert::From<Duplicate_Collateral> for PcsDaoErrors {
        fn from(value: Duplicate_Collateral) -> Self {
            Self::Duplicate_Collateral(value)
        }
    }
    impl ::core::convert::From<Expired_Certificates> for PcsDaoErrors {
        fn from(value: Expired_Certificates) -> Self {
            Self::Expired_Certificates(value)
        }
    }
    impl ::core::convert::From<Invalid_Issuer_Name> for PcsDaoErrors {
        fn from(value: Invalid_Issuer_Name) -> Self {
            Self::Invalid_Issuer_Name(value)
        }
    }
    impl ::core::convert::From<Invalid_PCK_CA> for PcsDaoErrors {
        fn from(value: Invalid_PCK_CA) -> Self {
            Self::Invalid_PCK_CA(value)
        }
    }
    impl ::core::convert::From<Invalid_Signature> for PcsDaoErrors {
        fn from(value: Invalid_Signature) -> Self {
            Self::Invalid_Signature(value)
        }
    }
    impl ::core::convert::From<Invalid_Subject_Name> for PcsDaoErrors {
        fn from(value: Invalid_Subject_Name) -> Self {
            Self::Invalid_Subject_Name(value)
        }
    }
    impl ::core::convert::From<Missing_Certificate> for PcsDaoErrors {
        fn from(value: Missing_Certificate) -> Self {
            Self::Missing_Certificate(value)
        }
    }
    impl ::core::convert::From<Missing_Issuer> for PcsDaoErrors {
        fn from(value: Missing_Issuer) -> Self {
            Self::Missing_Issuer(value)
        }
    }
    impl ::core::convert::From<Root_Key_Mismatch> for PcsDaoErrors {
        fn from(value: Root_Key_Mismatch) -> Self {
            Self::Root_Key_Mismatch(value)
        }
    }
    impl ::core::convert::From<TCB_Mismatch> for PcsDaoErrors {
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
        Hash
    )]
    #[ethevent(
        name = "UpsertedPCSCollateral",
        abi = "UpsertedPCSCollateral(uint8,bool)"
    )]
    pub struct UpsertedPCSCollateralFilter {
        #[ethevent(indexed)]
        pub ca: u8,
        pub is_crl: bool,
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
    ///Container type for all input parameters for the `PCS_KEY` function with signature `PCS_KEY(uint8,bool)` and selector `0xb13bf290`
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
    #[ethcall(name = "PCS_KEY", abi = "PCS_KEY(uint8,bool)")]
    pub struct PcsKeyCall {
        pub ca: u8,
        pub is_crl: bool,
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "getAttestedData", abi = "getAttestedData(bytes32)")]
    pub struct GetAttestedDataCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getCertificateById` function with signature `getCertificateById(uint8)` and selector `0x722f1327`
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
    #[ethcall(name = "getCertificateById", abi = "getCertificateById(uint8)")]
    pub struct GetCertificateByIdCall {
        pub ca: u8,
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
    ///Container type for all input parameters for the `upsertPckCrl` function with signature `upsertPckCrl(uint8,bytes)` and selector `0x08854e04`
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
    #[ethcall(name = "upsertPckCrl", abi = "upsertPckCrl(uint8,bytes)")]
    pub struct UpsertPckCrlCall {
        pub ca: u8,
        pub crl: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `upsertPcsCertificates` function with signature `upsertPcsCertificates(uint8,bytes)` and selector `0x3b395455`
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
        name = "upsertPcsCertificates",
        abi = "upsertPcsCertificates(uint8,bytes)"
    )]
    pub struct UpsertPcsCertificatesCall {
        pub ca: u8,
        pub cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `upsertRootCACrl` function with signature `upsertRootCACrl(bytes)` and selector `0x6b1c5399`
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
    #[ethcall(name = "upsertRootCACrl", abi = "upsertRootCACrl(bytes)")]
    pub struct UpsertRootCACrlCall {
        pub rootcacrl: ::ethers::core::types::Bytes,
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
    pub enum PcsDaoCalls {
        P256Verifier(P256VerifierCall),
        PcsKey(PcsKeyCall),
        CrlLib(CrlLibCall),
        GetAttestedData(GetAttestedDataCall),
        GetCertificateById(GetCertificateByIdCall),
        GetCollateralHash(GetCollateralHashCall),
        GetCollateralValidity(GetCollateralValidityCall),
        Resolver(ResolverCall),
        UpsertPckCrl(UpsertPckCrlCall),
        UpsertPcsCertificates(UpsertPcsCertificatesCall),
        UpsertRootCACrl(UpsertRootCACrlCall),
        X509(X509Call),
    }
    impl ::ethers::core::abi::AbiDecode for PcsDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <P256VerifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::P256Verifier(decoded));
            }
            if let Ok(decoded) = <PcsKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PcsKey(decoded));
            }
            if let Ok(decoded) = <CrlLibCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CrlLib(decoded));
            }
            if let Ok(decoded) = <GetAttestedDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAttestedData(decoded));
            }
            if let Ok(decoded) = <GetCertificateByIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCertificateById(decoded));
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
            if let Ok(decoded) = <ResolverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Resolver(decoded));
            }
            if let Ok(decoded) = <UpsertPckCrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertPckCrl(decoded));
            }
            if let Ok(decoded) = <UpsertPcsCertificatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertPcsCertificates(decoded));
            }
            if let Ok(decoded) = <UpsertRootCACrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpsertRootCACrl(decoded));
            }
            if let Ok(decoded) = <X509Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::X509(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PcsDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::P256Verifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PcsKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CrlLib(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAttestedData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCertificateById(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollateralHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCollateralValidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resolver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertPckCrl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertPcsCertificates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpsertRootCACrl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::X509(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PcsDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::P256Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::PcsKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrlLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAttestedData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCertificateById(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCollateralHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralValidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Resolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPckCrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPcsCertificates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpsertRootCACrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::X509(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<P256VerifierCall> for PcsDaoCalls {
        fn from(value: P256VerifierCall) -> Self {
            Self::P256Verifier(value)
        }
    }
    impl ::core::convert::From<PcsKeyCall> for PcsDaoCalls {
        fn from(value: PcsKeyCall) -> Self {
            Self::PcsKey(value)
        }
    }
    impl ::core::convert::From<CrlLibCall> for PcsDaoCalls {
        fn from(value: CrlLibCall) -> Self {
            Self::CrlLib(value)
        }
    }
    impl ::core::convert::From<GetAttestedDataCall> for PcsDaoCalls {
        fn from(value: GetAttestedDataCall) -> Self {
            Self::GetAttestedData(value)
        }
    }
    impl ::core::convert::From<GetCertificateByIdCall> for PcsDaoCalls {
        fn from(value: GetCertificateByIdCall) -> Self {
            Self::GetCertificateById(value)
        }
    }
    impl ::core::convert::From<GetCollateralHashCall> for PcsDaoCalls {
        fn from(value: GetCollateralHashCall) -> Self {
            Self::GetCollateralHash(value)
        }
    }
    impl ::core::convert::From<GetCollateralValidityCall> for PcsDaoCalls {
        fn from(value: GetCollateralValidityCall) -> Self {
            Self::GetCollateralValidity(value)
        }
    }
    impl ::core::convert::From<ResolverCall> for PcsDaoCalls {
        fn from(value: ResolverCall) -> Self {
            Self::Resolver(value)
        }
    }
    impl ::core::convert::From<UpsertPckCrlCall> for PcsDaoCalls {
        fn from(value: UpsertPckCrlCall) -> Self {
            Self::UpsertPckCrl(value)
        }
    }
    impl ::core::convert::From<UpsertPcsCertificatesCall> for PcsDaoCalls {
        fn from(value: UpsertPcsCertificatesCall) -> Self {
            Self::UpsertPcsCertificates(value)
        }
    }
    impl ::core::convert::From<UpsertRootCACrlCall> for PcsDaoCalls {
        fn from(value: UpsertRootCACrlCall) -> Self {
            Self::UpsertRootCACrl(value)
        }
    }
    impl ::core::convert::From<X509Call> for PcsDaoCalls {
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
    ///Container type for all return fields from the `PCS_KEY` function with signature `PCS_KEY(uint8,bool)` and selector `0xb13bf290`
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
    pub struct PcsKeyReturn {
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
        Hash
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
        Hash
    )]
    pub struct GetAttestedDataReturn {
        pub attestation_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getCertificateById` function with signature `getCertificateById(uint8)` and selector `0x722f1327`
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
    pub struct GetCertificateByIdReturn {
        pub cert: ::ethers::core::types::Bytes,
        pub crl: ::ethers::core::types::Bytes,
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
        pub not_valid_before: u64,
        pub not_valid_after: u64,
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
    ///Container type for all return fields from the `upsertPckCrl` function with signature `upsertPckCrl(uint8,bytes)` and selector `0x08854e04`
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
    pub struct UpsertPckCrlReturn {
        pub attestation_id: [u8; 32],
    }
    ///Container type for all return fields from the `upsertPcsCertificates` function with signature `upsertPcsCertificates(uint8,bytes)` and selector `0x3b395455`
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
    pub struct UpsertPcsCertificatesReturn {
        pub attestation_id: [u8; 32],
    }
    ///Container type for all return fields from the `upsertRootCACrl` function with signature `upsertRootCACrl(bytes)` and selector `0x6b1c5399`
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
    pub struct UpsertRootCACrlReturn {
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
}
