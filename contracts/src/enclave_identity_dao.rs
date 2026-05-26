pub use enclave_identity_dao::*;
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
pub mod enclave_identity_dao {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ENCLAVE_ID_KEY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ENCLAVE_ID_KEY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("EnclaveIdentityLib"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("EnclaveIdentityLib"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract EnclaveIdentityHelper",),
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
                    ::std::borrow::ToOwned::to_owned("getEnclaveIdentity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEnclaveIdentity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                            name: ::std::borrow::ToOwned::to_owned("enclaveIdObj"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct EnclaveIdentityJsonObj",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEnclaveIdentityIssuerChain"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEnclaveIdentityIssuerChain",),
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
                    ::std::borrow::ToOwned::to_owned("getIdentityContentHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getIdentityContentHash",),
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
                    ::std::borrow::ToOwned::to_owned("upsertEnclaveIdentity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("upsertEnclaveIdentity",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("version"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enclaveIdentityObj",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct EnclaveIdentityJsonObj",
                                    ),
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
                ::std::borrow::ToOwned::to_owned("UpsertedEnclaveIdentity"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("UpsertedEnclaveIdentity",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    ::std::borrow::ToOwned::to_owned("Enclave_Id_Expired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Enclave_Id_Expired"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Enclave_Id_Mismatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Enclave_Id_Mismatch",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Enclave_Id_Out_Of_Date"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Enclave_Id_Out_Of_Date",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Incorrect_Enclave_Id_Version"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Incorrect_Enclave_Id_Version",),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ENCLAVEIDENTITYDAO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                ENCLAVEIDENTITYDAO_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `ENCLAVE_ID_KEY` (0xca108769) function
        pub fn enclave_id_key(
            &self,
            id: ::ethers::core::types::U256,
            version: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([202, 16, 135, 105], (id, version))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EnclaveIdentityLib` (0x61d20bea) function
        pub fn enclave_identity_lib(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([97, 210, 11, 234], ())
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
        ///Calls the contract's `getIdentityContentHash` (0x7a9e1379) function
        pub fn get_identity_content_hash(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([122, 158, 19, 121], key)
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
        ///Calls the contract's `x509` (0xec950d33) function
        pub fn x_509(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([236, 149, 13, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `UpsertedEnclaveIdentity` event
        pub fn upserted_enclave_identity_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpsertedEnclaveIdentityFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpsertedEnclaveIdentityFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for EnclaveIdentityDao<M>
    {
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
    ///Custom Error type `Enclave_Id_Expired` with signature `Enclave_Id_Expired()` and selector `0x9ac04499`
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
    #[etherror(name = "Enclave_Id_Expired", abi = "Enclave_Id_Expired()")]
    pub struct Enclave_Id_Expired;
    ///Custom Error type `Enclave_Id_Mismatch` with signature `Enclave_Id_Mismatch()` and selector `0x289fa0cb`
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
    #[etherror(name = "Enclave_Id_Mismatch", abi = "Enclave_Id_Mismatch()")]
    pub struct Enclave_Id_Mismatch;
    ///Custom Error type `Enclave_Id_Out_Of_Date` with signature `Enclave_Id_Out_Of_Date()` and selector `0x7a204327`
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
    #[etherror(name = "Enclave_Id_Out_Of_Date", abi = "Enclave_Id_Out_Of_Date()")]
    pub struct Enclave_Id_Out_Of_Date;
    ///Custom Error type `Incorrect_Enclave_Id_Version` with signature `Incorrect_Enclave_Id_Version()` and selector `0x4e0f5696`
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
        name = "Incorrect_Enclave_Id_Version",
        abi = "Incorrect_Enclave_Id_Version()"
    )]
    pub struct Incorrect_Enclave_Id_Version;
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EnclaveIdentityDaoErrors {
        Duplicate_Collateral(Duplicate_Collateral),
        Enclave_Id_Expired(Enclave_Id_Expired),
        Enclave_Id_Mismatch(Enclave_Id_Mismatch),
        Enclave_Id_Out_Of_Date(Enclave_Id_Out_Of_Date),
        Incorrect_Enclave_Id_Version(Incorrect_Enclave_Id_Version),
        Invalid_TCB_Cert_Signature(Invalid_TCB_Cert_Signature),
        Missing_TCB_Cert(Missing_TCB_Cert),
        TCB_Cert_Expired(TCB_Cert_Expired),
        TCB_Cert_Revoked(TCB_Cert_Revoked),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for EnclaveIdentityDaoErrors {
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
                <Enclave_Id_Expired as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Enclave_Id_Expired(decoded));
            }
            if let Ok(decoded) =
                <Enclave_Id_Mismatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Enclave_Id_Mismatch(decoded));
            }
            if let Ok(decoded) =
                <Enclave_Id_Out_Of_Date as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Enclave_Id_Out_Of_Date(decoded));
            }
            if let Ok(decoded) =
                <Incorrect_Enclave_Id_Version as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Incorrect_Enclave_Id_Version(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EnclaveIdentityDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Duplicate_Collateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Enclave_Id_Expired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Enclave_Id_Mismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Enclave_Id_Out_Of_Date(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Incorrect_Enclave_Id_Version(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid_TCB_Cert_Signature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Missing_TCB_Cert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Cert_Expired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TCB_Cert_Revoked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for EnclaveIdentityDaoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Duplicate_Collateral as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Enclave_Id_Expired as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Enclave_Id_Mismatch as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Enclave_Id_Out_Of_Date as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <Incorrect_Enclave_Id_Version as ::ethers::contract::EthError>::selector(
                    ) =>
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for EnclaveIdentityDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Duplicate_Collateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::Enclave_Id_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::Enclave_Id_Mismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Enclave_Id_Out_Of_Date(element) => ::core::fmt::Display::fmt(element, f),
                Self::Incorrect_Enclave_Id_Version(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid_TCB_Cert_Signature(element) => ::core::fmt::Display::fmt(element, f),
                Self::Missing_TCB_Cert(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Cert_Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::TCB_Cert_Revoked(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for EnclaveIdentityDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Duplicate_Collateral> for EnclaveIdentityDaoErrors {
        fn from(value: Duplicate_Collateral) -> Self {
            Self::Duplicate_Collateral(value)
        }
    }
    impl ::core::convert::From<Enclave_Id_Expired> for EnclaveIdentityDaoErrors {
        fn from(value: Enclave_Id_Expired) -> Self {
            Self::Enclave_Id_Expired(value)
        }
    }
    impl ::core::convert::From<Enclave_Id_Mismatch> for EnclaveIdentityDaoErrors {
        fn from(value: Enclave_Id_Mismatch) -> Self {
            Self::Enclave_Id_Mismatch(value)
        }
    }
    impl ::core::convert::From<Enclave_Id_Out_Of_Date> for EnclaveIdentityDaoErrors {
        fn from(value: Enclave_Id_Out_Of_Date) -> Self {
            Self::Enclave_Id_Out_Of_Date(value)
        }
    }
    impl ::core::convert::From<Incorrect_Enclave_Id_Version> for EnclaveIdentityDaoErrors {
        fn from(value: Incorrect_Enclave_Id_Version) -> Self {
            Self::Incorrect_Enclave_Id_Version(value)
        }
    }
    impl ::core::convert::From<Invalid_TCB_Cert_Signature> for EnclaveIdentityDaoErrors {
        fn from(value: Invalid_TCB_Cert_Signature) -> Self {
            Self::Invalid_TCB_Cert_Signature(value)
        }
    }
    impl ::core::convert::From<Missing_TCB_Cert> for EnclaveIdentityDaoErrors {
        fn from(value: Missing_TCB_Cert) -> Self {
            Self::Missing_TCB_Cert(value)
        }
    }
    impl ::core::convert::From<TCB_Cert_Expired> for EnclaveIdentityDaoErrors {
        fn from(value: TCB_Cert_Expired) -> Self {
            Self::TCB_Cert_Expired(value)
        }
    }
    impl ::core::convert::From<TCB_Cert_Revoked> for EnclaveIdentityDaoErrors {
        fn from(value: TCB_Cert_Revoked) -> Self {
            Self::TCB_Cert_Revoked(value)
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
        name = "UpsertedEnclaveIdentity",
        abi = "UpsertedEnclaveIdentity(uint256,uint256)"
    )]
    pub struct UpsertedEnclaveIdentityFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub version: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `ENCLAVE_ID_KEY` function with signature `ENCLAVE_ID_KEY(uint256,uint256)` and selector `0xca108769`
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
    #[ethcall(name = "ENCLAVE_ID_KEY", abi = "ENCLAVE_ID_KEY(uint256,uint256)")]
    pub struct EnclaveIdKeyCall {
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
        Hash,
    )]
    #[ethcall(name = "EnclaveIdentityLib", abi = "EnclaveIdentityLib()")]
    pub struct EnclaveIdentityLibCall;
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
    ///Container type for all input parameters for the `getEnclaveIdentity` function with signature `getEnclaveIdentity(uint256,uint256)` and selector `0xf0f074f7`
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
        name = "getEnclaveIdentity",
        abi = "getEnclaveIdentity(uint256,uint256)"
    )]
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
        Hash,
    )]
    #[ethcall(
        name = "getEnclaveIdentityIssuerChain",
        abi = "getEnclaveIdentityIssuerChain()"
    )]
    pub struct GetEnclaveIdentityIssuerChainCall;
    ///Container type for all input parameters for the `getIdentityContentHash` function with signature `getIdentityContentHash(bytes32)` and selector `0x7a9e1379`
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
        name = "getIdentityContentHash",
        abi = "getIdentityContentHash(bytes32)"
    )]
    pub struct GetIdentityContentHashCall {
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
        Hash,
    )]
    #[ethcall(name = "resolver", abi = "resolver()")]
    pub struct ResolverCall;
    ///Container type for all input parameters for the `upsertEnclaveIdentity` function with signature `upsertEnclaveIdentity(uint256,uint256,(string,bytes))` and selector `0x30f704ea`
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
        name = "upsertEnclaveIdentity",
        abi = "upsertEnclaveIdentity(uint256,uint256,(string,bytes))"
    )]
    pub struct UpsertEnclaveIdentityCall {
        pub id: ::ethers::core::types::U256,
        pub version: ::ethers::core::types::U256,
        pub enclave_identity_obj: EnclaveIdentityJsonObj,
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
    pub enum EnclaveIdentityDaoCalls {
        EnclaveIdKey(EnclaveIdKeyCall),
        EnclaveIdentityLib(EnclaveIdentityLibCall),
        P256Verifier(P256VerifierCall),
        Pcs(PcsCall),
        CrlLibAddr(CrlLibAddrCall),
        GetAttestedData(GetAttestedDataCall),
        GetCollateralHash(GetCollateralHashCall),
        GetCollateralValidity(GetCollateralValidityCall),
        GetEnclaveIdentity(GetEnclaveIdentityCall),
        GetEnclaveIdentityIssuerChain(GetEnclaveIdentityIssuerChainCall),
        GetIdentityContentHash(GetIdentityContentHashCall),
        Resolver(ResolverCall),
        UpsertEnclaveIdentity(UpsertEnclaveIdentityCall),
        X509(X509Call),
    }
    impl ::ethers::core::abi::AbiDecode for EnclaveIdentityDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EnclaveIdKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnclaveIdKey(decoded));
            }
            if let Ok(decoded) =
                <EnclaveIdentityLibCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnclaveIdentityLib(decoded));
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
            if let Ok(decoded) =
                <GetEnclaveIdentityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetEnclaveIdentity(decoded));
            }
            if let Ok(decoded) =
                <GetEnclaveIdentityIssuerChainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetEnclaveIdentityIssuerChain(decoded));
            }
            if let Ok(decoded) =
                <GetIdentityContentHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetIdentityContentHash(decoded));
            }
            if let Ok(decoded) = <ResolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Resolver(decoded));
            }
            if let Ok(decoded) =
                <UpsertEnclaveIdentityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpsertEnclaveIdentity(decoded));
            }
            if let Ok(decoded) = <X509Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::X509(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EnclaveIdentityDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EnclaveIdKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnclaveIdentityLib(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::P256Verifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pcs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CrlLibAddr(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAttestedData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollateralHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollateralValidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEnclaveIdentity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetEnclaveIdentityIssuerChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetIdentityContentHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resolver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpsertEnclaveIdentity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::X509(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for EnclaveIdentityDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EnclaveIdKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnclaveIdentityLib(element) => ::core::fmt::Display::fmt(element, f),
                Self::P256Verifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pcs(element) => ::core::fmt::Display::fmt(element, f),
                Self::CrlLibAddr(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAttestedData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollateralValidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEnclaveIdentity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEnclaveIdentityIssuerChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetIdentityContentHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resolver(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertEnclaveIdentity(element) => ::core::fmt::Display::fmt(element, f),
                Self::X509(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnclaveIdKeyCall> for EnclaveIdentityDaoCalls {
        fn from(value: EnclaveIdKeyCall) -> Self {
            Self::EnclaveIdKey(value)
        }
    }
    impl ::core::convert::From<EnclaveIdentityLibCall> for EnclaveIdentityDaoCalls {
        fn from(value: EnclaveIdentityLibCall) -> Self {
            Self::EnclaveIdentityLib(value)
        }
    }
    impl ::core::convert::From<P256VerifierCall> for EnclaveIdentityDaoCalls {
        fn from(value: P256VerifierCall) -> Self {
            Self::P256Verifier(value)
        }
    }
    impl ::core::convert::From<PcsCall> for EnclaveIdentityDaoCalls {
        fn from(value: PcsCall) -> Self {
            Self::Pcs(value)
        }
    }
    impl ::core::convert::From<CrlLibAddrCall> for EnclaveIdentityDaoCalls {
        fn from(value: CrlLibAddrCall) -> Self {
            Self::CrlLibAddr(value)
        }
    }
    impl ::core::convert::From<GetAttestedDataCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetAttestedDataCall) -> Self {
            Self::GetAttestedData(value)
        }
    }
    impl ::core::convert::From<GetCollateralHashCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetCollateralHashCall) -> Self {
            Self::GetCollateralHash(value)
        }
    }
    impl ::core::convert::From<GetCollateralValidityCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetCollateralValidityCall) -> Self {
            Self::GetCollateralValidity(value)
        }
    }
    impl ::core::convert::From<GetEnclaveIdentityCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetEnclaveIdentityCall) -> Self {
            Self::GetEnclaveIdentity(value)
        }
    }
    impl ::core::convert::From<GetEnclaveIdentityIssuerChainCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetEnclaveIdentityIssuerChainCall) -> Self {
            Self::GetEnclaveIdentityIssuerChain(value)
        }
    }
    impl ::core::convert::From<GetIdentityContentHashCall> for EnclaveIdentityDaoCalls {
        fn from(value: GetIdentityContentHashCall) -> Self {
            Self::GetIdentityContentHash(value)
        }
    }
    impl ::core::convert::From<ResolverCall> for EnclaveIdentityDaoCalls {
        fn from(value: ResolverCall) -> Self {
            Self::Resolver(value)
        }
    }
    impl ::core::convert::From<UpsertEnclaveIdentityCall> for EnclaveIdentityDaoCalls {
        fn from(value: UpsertEnclaveIdentityCall) -> Self {
            Self::UpsertEnclaveIdentity(value)
        }
    }
    impl ::core::convert::From<X509Call> for EnclaveIdentityDaoCalls {
        fn from(value: X509Call) -> Self {
            Self::X509(value)
        }
    }
    ///Container type for all return fields from the `ENCLAVE_ID_KEY` function with signature `ENCLAVE_ID_KEY(uint256,uint256)` and selector `0xca108769`
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
    pub struct EnclaveIdKeyReturn {
        pub key: [u8; 32],
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
        Hash,
    )]
    pub struct EnclaveIdentityLibReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getEnclaveIdentity` function with signature `getEnclaveIdentity(uint256,uint256)` and selector `0xf0f074f7`
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
        Hash,
    )]
    pub struct GetEnclaveIdentityIssuerChainReturn {
        pub signing_cert: ::ethers::core::types::Bytes,
        pub root_cert: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `getIdentityContentHash` function with signature `getIdentityContentHash(bytes32)` and selector `0x7a9e1379`
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
    pub struct GetIdentityContentHashReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `upsertEnclaveIdentity` function with signature `upsertEnclaveIdentity(uint256,uint256,(string,bytes))` and selector `0x30f704ea`
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
    pub struct UpsertEnclaveIdentityReturn {
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
    ///`EnclaveIdentityJsonObj(string,bytes)`
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
    pub struct EnclaveIdentityJsonObj {
        pub identity_str: ::std::string::String,
        pub signature: ::ethers::core::types::Bytes,
    }
}
