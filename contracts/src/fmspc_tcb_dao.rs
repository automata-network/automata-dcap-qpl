pub use fmspc_tcb_dao::*;
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
pub mod fmspc_tcb_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FMSPC_TCB_KEY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("FMSPC_TCB_KEY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tcbType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fmspc"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(6usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes6"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("version"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FmspcTcbLib"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("FmspcTcbLib"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract FmspcTcbHelper"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
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
                    ::std::borrow::ToOwned::to_owned("crlLibAddr"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("crlLibAddr"),
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
                                name: ::std::borrow::ToOwned::to_owned("issueDateTimestamp",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextUpdateTimestamp",),
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
                    ::std::borrow::ToOwned::to_owned("getTcbInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTcbInfo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tcbType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tcbObj"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct TcbInfoJsonObj"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTcbInfoContentHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTcbInfoContentHash",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getTcbIssuerChain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("upsertFmspcTcb"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upsertFmspcTcb"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tcbInfoObj"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct TcbInfoJsonObj"),
                            ),
                        },],
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
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("UpsertedFmpscTcb"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("UpsertedFmpscTcb"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("tcbType"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("fmspcTcbBytes"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(6usize,),
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: true,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Duplicate_Collateral"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Duplicate_Collateral",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid_TCB_Cert_Signature"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Invalid_TCB_Cert_Signature",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Missing_TCB_Cert"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Missing_TCB_Cert"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Cert_Expired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TCB_Cert_Expired"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Cert_Revoked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TCB_Cert_Revoked"),
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
                    ::std::borrow::ToOwned::to_owned("TCB_Expired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TCB_Expired"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TCB_Out_Of_Date"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TCB_Out_Of_Date"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FMSPCTCBDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                FMSPCTCBDAO_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `FMSPC_TCB_KEY` (0xb63e9e7b) function
        pub fn fmspc_tcb_key(
            &self,
            tcb_type: u8,
            fmspc: [u8; 6],
            version: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([182, 62, 158, 123], (tcb_type, fmspc, version))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FmspcTcbLib` (0x4ba52fa5) function
        pub fn fmspc_tcb_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([75, 165, 47, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `P256_VERIFIER` (0x536c633d) function
        pub fn p256_verifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([83, 108, 99, 61], ())
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
        ///Calls the contract's `crlLibAddr` (0x37c6d028) function
        pub fn crl_lib_addr(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([55, 198, 208, 40], ())
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
        ///Calls the contract's `getTcbInfoContentHash` (0x0808e59b) function
        pub fn get_tcb_info_content_hash(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 8, 229, 155], key)
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
        ///Calls the contract's `resolver` (0x04f3bcec) function
        pub fn resolver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([4, 243, 188, 236], ())
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
        ///Calls the contract's `x509` (0xec950d33) function
        pub fn x_509(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([236, 149, 13, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `UpsertedFmpscTcb` event
        pub fn upserted_fmpsc_tcb_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpsertedFmpscTcbFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpsertedFmpscTcbFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for FmspcTcbDao<M> {
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
        Hash,
    )]
    #[etherror(name = "Duplicate_Collateral", abi = "Duplicate_Collateral()")]
    pub struct Duplicate_Collateral;
    ///Custom Error type `Invalid_TCB_Cert_Signature` with signature `Invalid_TCB_Cert_Signature()` and selector `0x8de7233f`
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
    #[etherror(
        name = "Invalid_TCB_Cert_Signature",
        abi = "Invalid_TCB_Cert_Signature()"
    )]
    pub struct Invalid_TCB_Cert_Signature;
    ///Custom Error type `Missing_TCB_Cert` with signature `Missing_TCB_Cert()` and selector `0x841a0280`
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
    #[etherror(name = "Missing_TCB_Cert", abi = "Missing_TCB_Cert()")]
    pub struct Missing_TCB_Cert;
    ///Custom Error type `TCB_Cert_Expired` with signature `TCB_Cert_Expired()` and selector `0xea8cd522`
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
    #[etherror(name = "TCB_Cert_Expired", abi = "TCB_Cert_Expired()")]
    pub struct TCB_Cert_Expired;
    ///Custom Error type `TCB_Cert_Revoked` with signature `TCB_Cert_Revoked(uint256)` and selector `0x7fb57a7a`
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
    #[etherror(name = "TCB_Cert_Revoked", abi = "TCB_Cert_Revoked(uint256)")]
    pub struct TCB_Cert_Revoked {
        pub serial_num: ::ethers::core::types::U256,
    }
    ///Custom Error type `TCB_Expired` with signature `TCB_Expired()` and selector `0xbae57649`
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
    #[etherror(name = "TCB_Expired", abi = "TCB_Expired()")]
    pub struct TCB_Expired;
    ///Custom Error type `TCB_Out_Of_Date` with signature `TCB_Out_Of_Date()` and selector `0x3d78f9f9`
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
    #[etherror(name = "TCB_Out_Of_Date", abi = "TCB_Out_Of_Date()")]
    pub struct TCB_Out_Of_Date;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FmspcTcbDaoErrors {
        Duplicate_Collateral(Duplicate_Collateral),
        Invalid_TCB_Cert_Signature(Invalid_TCB_Cert_Signature),
        Missing_TCB_Cert(Missing_TCB_Cert),
        TCB_Cert_Expired(TCB_Cert_Expired),
        TCB_Cert_Revoked(TCB_Cert_Revoked),
        TCB_Expired(TCB_Expired),
        TCB_Out_Of_Date(TCB_Out_Of_Date),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FmspcTcbDaoErrors {
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
                <Duplicate_Collateral as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Duplicate_Collateral(decoded));
            }
            if let Ok(decoded) =
                <Invalid_TCB_Cert_Signature as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Invalid_TCB_Cert_Signature(decoded));
            }
            if let Ok(decoded) = <Missing_TCB_Cert as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Missing_TCB_Cert(decoded));
            }
            if let Ok(decoded) = <TCB_Cert_Expired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TCB_Cert_Expired(decoded));
            }
            if let Ok(decoded) = <TCB_Cert_Revoked as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TCB_Cert_Revoked(decoded));
            }
            if let Ok(decoded) = <TCB_Expired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TCB_Expired(decoded));
            }
            if let Ok(decoded) = <TCB_Out_Of_Date as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TCB_Out_Of_Date(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FmspcTcbDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Duplicate_Collateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_TCB_Cert_Signature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Missing_TCB_Cert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Cert_Expired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Cert_Revoked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Expired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Out_Of_Date(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FmspcTcbDaoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Duplicate_Collateral as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Invalid_TCB_Cert_Signature as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Missing_TCB_Cert as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <TCB_Cert_Expired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <TCB_Cert_Revoked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <TCB_Expired as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <TCB_Out_Of_Date as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FmspcTcbDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Duplicate_Collateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid_TCB_Cert_Signature(element) => ::core::fmt::Display::fmt(element, f),
                Self::Missing_TCB_Cert(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Cert_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Cert_Revoked(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Out_Of_Date(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FmspcTcbDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Duplicate_Collateral> for FmspcTcbDaoErrors {
        fn from(value: Duplicate_Collateral) -> Self {
            Self::Duplicate_Collateral(value)
        }
    }
    impl ::core::convert::From<Invalid_TCB_Cert_Signature> for FmspcTcbDaoErrors {
        fn from(value: Invalid_TCB_Cert_Signature) -> Self {
            Self::Invalid_TCB_Cert_Signature(value)
        }
    }
    impl ::core::convert::From<Missing_TCB_Cert> for FmspcTcbDaoErrors {
        fn from(value: Missing_TCB_Cert) -> Self {
            Self::Missing_TCB_Cert(value)
        }
    }
    impl ::core::convert::From<TCB_Cert_Expired> for FmspcTcbDaoErrors {
        fn from(value: TCB_Cert_Expired) -> Self {
            Self::TCB_Cert_Expired(value)
        }
    }
    impl ::core::convert::From<TCB_Cert_Revoked> for FmspcTcbDaoErrors {
        fn from(value: TCB_Cert_Revoked) -> Self {
            Self::TCB_Cert_Revoked(value)
        }
    }
    impl ::core::convert::From<TCB_Expired> for FmspcTcbDaoErrors {
        fn from(value: TCB_Expired) -> Self {
            Self::TCB_Expired(value)
        }
    }
    impl ::core::convert::From<TCB_Out_Of_Date> for FmspcTcbDaoErrors {
        fn from(value: TCB_Out_Of_Date) -> Self {
            Self::TCB_Out_Of_Date(value)
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
        name = "UpsertedFmpscTcb",
        abi = "UpsertedFmpscTcb(uint8,bytes6,uint32)"
    )]
    pub struct UpsertedFmpscTcbFilter {
        #[ethevent(indexed)]
        pub tcb_type: u8,
        #[ethevent(indexed)]
        pub fmspc_tcb_bytes: [u8; 6],
        #[ethevent(indexed)]
        pub version: u32,
    }
    ///Container type for all input parameters for the `FMSPC_TCB_KEY` function with signature `FMSPC_TCB_KEY(uint8,bytes6,uint32)` and selector `0xb63e9e7b`
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
    #[ethcall(name = "FMSPC_TCB_KEY", abi = "FMSPC_TCB_KEY(uint8,bytes6,uint32)")]
    pub struct FmspcTcbKeyCall {
        pub tcb_type: u8,
        pub fmspc: [u8; 6],
        pub version: u32,
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
        Hash,
    )]
    #[ethcall(name = "FmspcTcbLib", abi = "FmspcTcbLib()")]
    pub struct FmspcTcbLibCall;
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
    ///Container type for all input parameters for the `crlLibAddr` function with signature `crlLibAddr()` and selector `0x37c6d028`
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
    #[ethcall(name = "crlLibAddr", abi = "crlLibAddr()")]
    pub struct CrlLibAddrCall;
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
    ///Container type for all input parameters for the `getTcbInfo` function with signature `getTcbInfo(uint256,string,uint256)` and selector `0xcfbc42fb`
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
    #[ethcall(name = "getTcbInfo", abi = "getTcbInfo(uint256,string,uint256)")]
    pub struct GetTcbInfoCall {
        pub tcb_type: ::ethers::core::types::U256,
        pub fmspc: ::std::string::String,
        pub version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTcbInfoContentHash` function with signature `getTcbInfoContentHash(bytes32)` and selector `0x0808e59b`
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
    #[ethcall(name = "getTcbInfoContentHash", abi = "getTcbInfoContentHash(bytes32)")]
    pub struct GetTcbInfoContentHashCall {
        pub key: [u8; 32],
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
        Hash,
    )]
    #[ethcall(name = "getTcbIssuerChain", abi = "getTcbIssuerChain()")]
    pub struct GetTcbIssuerChainCall;
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
    ///Container type for all input parameters for the `upsertFmspcTcb` function with signature `upsertFmspcTcb((string,bytes))` and selector `0xa8349fb7`
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
    #[ethcall(name = "upsertFmspcTcb", abi = "upsertFmspcTcb((string,bytes))")]
    pub struct UpsertFmspcTcbCall {
        pub tcb_info_obj: TcbInfoJsonObj,
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
    pub enum FmspcTcbDaoCalls {
        FmspcTcbKey(FmspcTcbKeyCall),
        FmspcTcbLib(FmspcTcbLibCall),
        P256Verifier(P256VerifierCall),
        Pcs(PcsCall),
        CrlLibAddr(CrlLibAddrCall),
        GetAttestedData(GetAttestedDataCall),
        GetCollateralHash(GetCollateralHashCall),
        GetCollateralValidity(GetCollateralValidityCall),
        GetTcbInfo(GetTcbInfoCall),
        GetTcbInfoContentHash(GetTcbInfoContentHashCall),
        GetTcbIssuerChain(GetTcbIssuerChainCall),
        Resolver(ResolverCall),
        UpsertFmspcTcb(UpsertFmspcTcbCall),
        X509(X509Call),
    }
    impl ::ethers::core::abi::AbiDecode for FmspcTcbDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FmspcTcbKeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FmspcTcbKey(decoded));
            }
            if let Ok(decoded) = <FmspcTcbLibCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FmspcTcbLib(decoded));
            }
            if let Ok(decoded) = <P256VerifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::P256Verifier(decoded));
            }
            if let Ok(decoded) = <PcsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pcs(decoded));
            }
            if let Ok(decoded) = <CrlLibAddrCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CrlLibAddr(decoded));
            }
            if let Ok(decoded) =
                <GetAttestedDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAttestedData(decoded));
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
            if let Ok(decoded) = <GetTcbInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTcbInfo(decoded));
            }
            if let Ok(decoded) =
                <GetTcbInfoContentHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTcbInfoContentHash(decoded));
            }
            if let Ok(decoded) =
                <GetTcbIssuerChainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTcbIssuerChain(decoded));
            }
            if let Ok(decoded) = <ResolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Resolver(decoded));
            }
            if let Ok(decoded) =
                <UpsertFmspcTcbCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpsertFmspcTcb(decoded));
            }
            if let Ok(decoded) = <X509Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::X509(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FmspcTcbDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FmspcTcbKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FmspcTcbLib(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::P256Verifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CrlLibAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAttestedData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollateralHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollateralValidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTcbInfoContentHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTcbIssuerChain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Resolver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpsertFmspcTcb(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::X509(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FmspcTcbDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FmspcTcbKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::FmspcTcbLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::P256Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrlLibAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAttestedData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralValidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTcbInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTcbInfoContentHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTcbIssuerChain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertFmspcTcb(element) => ::core::fmt::Display::fmt(element, f),
                Self::X509(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FmspcTcbKeyCall> for FmspcTcbDaoCalls {
        fn from(value: FmspcTcbKeyCall) -> Self {
            Self::FmspcTcbKey(value)
        }
    }
    impl ::core::convert::From<FmspcTcbLibCall> for FmspcTcbDaoCalls {
        fn from(value: FmspcTcbLibCall) -> Self {
            Self::FmspcTcbLib(value)
        }
    }
    impl ::core::convert::From<P256VerifierCall> for FmspcTcbDaoCalls {
        fn from(value: P256VerifierCall) -> Self {
            Self::P256Verifier(value)
        }
    }
    impl ::core::convert::From<PcsCall> for FmspcTcbDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<CrlLibAddrCall> for FmspcTcbDaoCalls {
        fn from(value: CrlLibAddrCall) -> Self {
            Self::CrlLibAddr(value)
        }
    }
    impl ::core::convert::From<GetAttestedDataCall> for FmspcTcbDaoCalls {
        fn from(value: GetAttestedDataCall) -> Self {
            Self::GetAttestedData(value)
        }
    }
    impl ::core::convert::From<GetCollateralHashCall> for FmspcTcbDaoCalls {
        fn from(value: GetCollateralHashCall) -> Self {
            Self::GetCollateralHash(value)
        }
    }
    impl ::core::convert::From<GetCollateralValidityCall> for FmspcTcbDaoCalls {
        fn from(value: GetCollateralValidityCall) -> Self {
            Self::GetCollateralValidity(value)
        }
    }
    impl ::core::convert::From<GetTcbInfoCall> for FmspcTcbDaoCalls {
        fn from(value: GetTcbInfoCall) -> Self {
            Self::GetTcbInfo(value)
        }
    }
    impl ::core::convert::From<GetTcbInfoContentHashCall> for FmspcTcbDaoCalls {
        fn from(value: GetTcbInfoContentHashCall) -> Self {
            Self::GetTcbInfoContentHash(value)
        }
    }
    impl ::core::convert::From<GetTcbIssuerChainCall> for FmspcTcbDaoCalls {
        fn from(value: GetTcbIssuerChainCall) -> Self {
            Self::GetTcbIssuerChain(value)
        }
    }
    impl ::core::convert::From<ResolverCall> for FmspcTcbDaoCalls {
        fn from(value: ResolverCall) -> Self {
            Self::Resolver(value)
        }
    }
    impl ::core::convert::From<UpsertFmspcTcbCall> for FmspcTcbDaoCalls {
        fn from(value: UpsertFmspcTcbCall) -> Self {
            Self::UpsertFmspcTcb(value)
        }
    }
    impl ::core::convert::From<X509Call> for FmspcTcbDaoCalls {
        fn from(value: X509Call) -> Self {
            Self::X509(value)
        }
    }
    ///Container type for all return fields from the `FMSPC_TCB_KEY` function with signature `FMSPC_TCB_KEY(uint8,bytes6,uint32)` and selector `0xb63e9e7b`
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
    pub struct FmspcTcbKeyReturn {
        pub key: [u8; 32],
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
        Hash,
    )]
    pub struct FmspcTcbLibReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `crlLibAddr` function with signature `crlLibAddr()` and selector `0x37c6d028`
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
    pub struct CrlLibAddrReturn(pub ::ethers::core::types::Address);
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
        pub issue_date_timestamp: u64,
        pub next_update_timestamp: u64,
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
        Hash,
    )]
    pub struct GetTcbInfoReturn {
        pub tcb_obj: TcbInfoJsonObj,
    }
    ///Container type for all return fields from the `getTcbInfoContentHash` function with signature `getTcbInfoContentHash(bytes32)` and selector `0x0808e59b`
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
    pub struct GetTcbInfoContentHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getTcbIssuerChain` function with signature `getTcbIssuerChain()` and selector `0xa53e7275`
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
    pub struct GetTcbIssuerChainReturn {
        pub signing_cert: ::ethers::core::types::Bytes,
        pub root_cert: ::ethers::core::types::Bytes,
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
        Hash,
    )]
    pub struct ResolverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `upsertFmspcTcb` function with signature `upsertFmspcTcb((string,bytes))` and selector `0xa8349fb7`
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
    pub struct UpsertFmspcTcbReturn {
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
        Hash,
    )]
    pub struct X509Return(pub ::ethers::core::types::Address);
    ///`TcbInfoJsonObj(string,bytes)`
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
    pub struct TcbInfoJsonObj {
        pub tcb_info_str: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
}
