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
                    ::std::borrow::ToOwned::to_owned("pcsCertAttestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pcsCertAttestations",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
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
                    ::std::borrow::ToOwned::to_owned("pcsCertSchemaID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pcsCertSchemaID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "PCS_CERT_SCHEMA_ID",
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
                    ::std::borrow::ToOwned::to_owned("pcsCrlAttestations"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pcsCrlAttestations"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum CA"),
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
                    ::std::borrow::ToOwned::to_owned("pcsCrlSchemaID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pcsCrlSchemaID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("PCS_CRL_SCHEMA_ID"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
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
        ///Calls the contract's `pcsCertAttestations` (0x974ddd95) function
        pub fn pcs_cert_attestations(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 77, 221, 149], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pcsCertSchemaID` (0xfb1c0125) function
        pub fn pcs_cert_schema_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([251, 28, 1, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pcsCrlAttestations` (0x189d97f7) function
        pub fn pcs_crl_attestations(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([24, 157, 151, 247], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pcsCrlSchemaID` (0x2bce0147) function
        pub fn pcs_crl_schema_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([43, 206, 1, 71], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PcsDao<M> {
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PcsDaoErrors {
        Invalid_PCK_CA(Invalid_PCK_CA),
        Missing_Certificate(Missing_Certificate),
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
            if let Ok(decoded) = <Invalid_PCK_CA as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid_PCK_CA(decoded));
            }
            if let Ok(decoded) = <Missing_Certificate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Missing_Certificate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PcsDaoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Invalid_PCK_CA(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Missing_Certificate(element) => {
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
                    == <Invalid_PCK_CA as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Missing_Certificate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PcsDaoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Invalid_PCK_CA(element) => ::core::fmt::Display::fmt(element, f),
                Self::Missing_Certificate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PcsDaoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Invalid_PCK_CA> for PcsDaoErrors {
        fn from(value: Invalid_PCK_CA) -> Self {
            Self::Invalid_PCK_CA(value)
        }
    }
    impl ::core::convert::From<Missing_Certificate> for PcsDaoErrors {
        fn from(value: Missing_Certificate) -> Self {
            Self::Missing_Certificate(value)
        }
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
    ///Container type for all input parameters for the `pcsCertAttestations` function with signature `pcsCertAttestations(uint8)` and selector `0x974ddd95`
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
    #[ethcall(name = "pcsCertAttestations", abi = "pcsCertAttestations(uint8)")]
    pub struct PcsCertAttestationsCall(pub u8);
    ///Container type for all input parameters for the `pcsCertSchemaID` function with signature `pcsCertSchemaID()` and selector `0xfb1c0125`
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
    #[ethcall(name = "pcsCertSchemaID", abi = "pcsCertSchemaID()")]
    pub struct PcsCertSchemaIDCall;
    ///Container type for all input parameters for the `pcsCrlAttestations` function with signature `pcsCrlAttestations(uint8)` and selector `0x189d97f7`
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
    #[ethcall(name = "pcsCrlAttestations", abi = "pcsCrlAttestations(uint8)")]
    pub struct PcsCrlAttestationsCall(pub u8);
    ///Container type for all input parameters for the `pcsCrlSchemaID` function with signature `pcsCrlSchemaID()` and selector `0x2bce0147`
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
    #[ethcall(name = "pcsCrlSchemaID", abi = "pcsCrlSchemaID()")]
    pub struct PcsCrlSchemaIDCall;
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PcsDaoCalls {
        GetCertificateById(GetCertificateByIdCall),
        PcsCertAttestations(PcsCertAttestationsCall),
        PcsCertSchemaID(PcsCertSchemaIDCall),
        PcsCrlAttestations(PcsCrlAttestationsCall),
        PcsCrlSchemaID(PcsCrlSchemaIDCall),
        UpsertPckCrl(UpsertPckCrlCall),
        UpsertPcsCertificates(UpsertPcsCertificatesCall),
        UpsertRootCACrl(UpsertRootCACrlCall),
    }
    impl ::ethers::core::abi::AbiDecode for PcsDaoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCertificateByIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCertificateById(decoded));
            }
            if let Ok(decoded) = <PcsCertAttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PcsCertAttestations(decoded));
            }
            if let Ok(decoded) = <PcsCertSchemaIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PcsCertSchemaID(decoded));
            }
            if let Ok(decoded) = <PcsCrlAttestationsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PcsCrlAttestations(decoded));
            }
            if let Ok(decoded) = <PcsCrlSchemaIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PcsCrlSchemaID(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PcsDaoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCertificateById(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PcsCertAttestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PcsCertSchemaID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PcsCrlAttestations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PcsCrlSchemaID(element) => {
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
            }
        }
    }
    impl ::core::fmt::Display for PcsDaoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCertificateById(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PcsCertAttestations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PcsCertSchemaID(element) => ::core::fmt::Display::fmt(element, f),
                Self::PcsCrlAttestations(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PcsCrlSchemaID(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPckCrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpsertPcsCertificates(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpsertRootCACrl(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetCertificateByIdCall> for PcsDaoCalls {
        fn from(value: GetCertificateByIdCall) -> Self {
            Self::GetCertificateById(value)
        }
    }
    impl ::core::convert::From<PcsCertAttestationsCall> for PcsDaoCalls {
        fn from(value: PcsCertAttestationsCall) -> Self {
            Self::PcsCertAttestations(value)
        }
    }
    impl ::core::convert::From<PcsCertSchemaIDCall> for PcsDaoCalls {
        fn from(value: PcsCertSchemaIDCall) -> Self {
            Self::PcsCertSchemaID(value)
        }
    }
    impl ::core::convert::From<PcsCrlAttestationsCall> for PcsDaoCalls {
        fn from(value: PcsCrlAttestationsCall) -> Self {
            Self::PcsCrlAttestations(value)
        }
    }
    impl ::core::convert::From<PcsCrlSchemaIDCall> for PcsDaoCalls {
        fn from(value: PcsCrlSchemaIDCall) -> Self {
            Self::PcsCrlSchemaID(value)
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
    ///Container type for all return fields from the `pcsCertAttestations` function with signature `pcsCertAttestations(uint8)` and selector `0x974ddd95`
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
    pub struct PcsCertAttestationsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `pcsCertSchemaID` function with signature `pcsCertSchemaID()` and selector `0xfb1c0125`
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
    pub struct PcsCertSchemaIDReturn {
        pub pcs_cert_schema_id: [u8; 32],
    }
    ///Container type for all return fields from the `pcsCrlAttestations` function with signature `pcsCrlAttestations(uint8)` and selector `0x189d97f7`
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
    pub struct PcsCrlAttestationsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `pcsCrlSchemaID` function with signature `pcsCrlSchemaID()` and selector `0x2bce0147`
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
    pub struct PcsCrlSchemaIDReturn {
        pub pcs_crl_schema_id: [u8; 32],
    }
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
}
