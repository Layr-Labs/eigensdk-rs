pub use i_strategy::*;
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
pub mod i_strategy {
    const _: () = {
        ::core::include_bytes!(
            "/Users/supernovahs/Desktop/eigensdk-rs/crates/contracts/bindings/json/IStrategy.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("explanation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("explanation"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("shares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("shares"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sharesToUnderlying"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sharesToUnderlying"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountShares"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sharesToUnderlyingView"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sharesToUnderlyingView",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountShares"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalShares"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlyingToShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("underlyingToShares"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountUnderlying"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlyingToSharesView"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("underlyingToSharesView",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountUnderlying"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("underlyingToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("underlyingToken"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IERC20"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userUnderlying"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("userUnderlying"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("userUnderlyingView"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("userUnderlyingView"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountShares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISTRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISTRATEGY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `deposit` (0x47e7ef24) function
        pub fn deposit(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 231, 239, 36], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `explanation` (0xab5921e1) function
        pub fn explanation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([171, 89, 33, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shares` (0xce7c2ac2) function
        pub fn shares(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 124, 42, 194], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharesToUnderlying` (0xf3e73875) function
        pub fn shares_to_underlying(
            &self,
            amount_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 231, 56, 117], amount_shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharesToUnderlyingView` (0x7a8b2637) function
        pub fn shares_to_underlying_view(
            &self,
            amount_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 139, 38, 55], amount_shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalShares` (0x3a98ef39) function
        pub fn total_shares(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([58, 152, 239, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToShares` (0x8c871019) function
        pub fn underlying_to_shares(
            &self,
            amount_underlying: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 135, 16, 25], amount_underlying)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToSharesView` (0xe3dae51c) function
        pub fn underlying_to_shares_view(
            &self,
            amount_underlying: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 218, 229, 28], amount_underlying)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `underlyingToken` (0x2495a599) function
        pub fn underlying_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userUnderlying` (0x8f6a6240) function
        pub fn user_underlying(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 106, 98, 64], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userUnderlyingView` (0x553ca5f8) function
        pub fn user_underlying_view(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([85, 60, 165, 248], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xd9caed12) function
        pub fn withdraw(
            &self,
            recipient: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount_shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 202, 237, 18], (recipient, token, amount_shares))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IStrategy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(address,uint256)` and selector `0x47e7ef24`
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
    #[ethcall(name = "deposit", abi = "deposit(address,uint256)")]
    pub struct DepositCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `explanation` function with signature `explanation()` and selector `0xab5921e1`
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
    #[ethcall(name = "explanation", abi = "explanation()")]
    pub struct ExplanationCall;
    ///Container type for all input parameters for the `shares` function with signature `shares(address)` and selector `0xce7c2ac2`
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
    #[ethcall(name = "shares", abi = "shares(address)")]
    pub struct SharesCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sharesToUnderlying` function with signature `sharesToUnderlying(uint256)` and selector `0xf3e73875`
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
    #[ethcall(name = "sharesToUnderlying", abi = "sharesToUnderlying(uint256)")]
    pub struct SharesToUnderlyingCall {
        pub amount_shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sharesToUnderlyingView` function with signature `sharesToUnderlyingView(uint256)` and selector `0x7a8b2637`
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
        name = "sharesToUnderlyingView",
        abi = "sharesToUnderlyingView(uint256)"
    )]
    pub struct SharesToUnderlyingViewCall {
        pub amount_shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalShares` function with signature `totalShares()` and selector `0x3a98ef39`
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
    #[ethcall(name = "totalShares", abi = "totalShares()")]
    pub struct TotalSharesCall;
    ///Container type for all input parameters for the `underlyingToShares` function with signature `underlyingToShares(uint256)` and selector `0x8c871019`
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
    #[ethcall(name = "underlyingToShares", abi = "underlyingToShares(uint256)")]
    pub struct UnderlyingToSharesCall {
        pub amount_underlying: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `underlyingToSharesView` function with signature `underlyingToSharesView(uint256)` and selector `0xe3dae51c`
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
        name = "underlyingToSharesView",
        abi = "underlyingToSharesView(uint256)"
    )]
    pub struct UnderlyingToSharesViewCall {
        pub amount_underlying: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `underlyingToken` function with signature `underlyingToken()` and selector `0x2495a599`
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
    #[ethcall(name = "underlyingToken", abi = "underlyingToken()")]
    pub struct UnderlyingTokenCall;
    ///Container type for all input parameters for the `userUnderlying` function with signature `userUnderlying(address)` and selector `0x8f6a6240`
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
    #[ethcall(name = "userUnderlying", abi = "userUnderlying(address)")]
    pub struct UserUnderlyingCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userUnderlyingView` function with signature `userUnderlyingView(address)` and selector `0x553ca5f8`
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
    #[ethcall(name = "userUnderlyingView", abi = "userUnderlyingView(address)")]
    pub struct UserUnderlyingViewCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(address,address,uint256)` and selector `0xd9caed12`
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
    #[ethcall(name = "withdraw", abi = "withdraw(address,address,uint256)")]
    pub struct WithdrawCall {
        pub recipient: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount_shares: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IStrategyCalls {
        Deposit(DepositCall),
        Explanation(ExplanationCall),
        Shares(SharesCall),
        SharesToUnderlying(SharesToUnderlyingCall),
        SharesToUnderlyingView(SharesToUnderlyingViewCall),
        TotalShares(TotalSharesCall),
        UnderlyingToShares(UnderlyingToSharesCall),
        UnderlyingToSharesView(UnderlyingToSharesViewCall),
        UnderlyingToken(UnderlyingTokenCall),
        UserUnderlying(UserUnderlyingCall),
        UserUnderlyingView(UserUnderlyingViewCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for IStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <ExplanationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Explanation(decoded));
            }
            if let Ok(decoded) = <SharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Shares(decoded));
            }
            if let Ok(decoded) =
                <SharesToUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SharesToUnderlying(decoded));
            }
            if let Ok(decoded) =
                <SharesToUnderlyingViewCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SharesToUnderlyingView(decoded));
            }
            if let Ok(decoded) = <TotalSharesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalShares(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingToSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnderlyingToShares(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingToSharesViewCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnderlyingToSharesView(decoded));
            }
            if let Ok(decoded) =
                <UnderlyingTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnderlyingToken(decoded));
            }
            if let Ok(decoded) =
                <UserUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UserUnderlying(decoded));
            }
            if let Ok(decoded) =
                <UserUnderlyingViewCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UserUnderlyingView(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Explanation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Shares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SharesToUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SharesToUnderlyingView(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnderlyingToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingToSharesView(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnderlyingToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UserUnderlying(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UserUnderlyingView(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Explanation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Shares(element) => ::core::fmt::Display::fmt(element, f),
                Self::SharesToUnderlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::SharesToUnderlyingView(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingToSharesView(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnderlyingToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserUnderlying(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserUnderlyingView(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositCall> for IStrategyCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExplanationCall> for IStrategyCalls {
        fn from(value: ExplanationCall) -> Self {
            Self::Explanation(value)
        }
    }
    impl ::core::convert::From<SharesCall> for IStrategyCalls {
        fn from(value: SharesCall) -> Self {
            Self::Shares(value)
        }
    }
    impl ::core::convert::From<SharesToUnderlyingCall> for IStrategyCalls {
        fn from(value: SharesToUnderlyingCall) -> Self {
            Self::SharesToUnderlying(value)
        }
    }
    impl ::core::convert::From<SharesToUnderlyingViewCall> for IStrategyCalls {
        fn from(value: SharesToUnderlyingViewCall) -> Self {
            Self::SharesToUnderlyingView(value)
        }
    }
    impl ::core::convert::From<TotalSharesCall> for IStrategyCalls {
        fn from(value: TotalSharesCall) -> Self {
            Self::TotalShares(value)
        }
    }
    impl ::core::convert::From<UnderlyingToSharesCall> for IStrategyCalls {
        fn from(value: UnderlyingToSharesCall) -> Self {
            Self::UnderlyingToShares(value)
        }
    }
    impl ::core::convert::From<UnderlyingToSharesViewCall> for IStrategyCalls {
        fn from(value: UnderlyingToSharesViewCall) -> Self {
            Self::UnderlyingToSharesView(value)
        }
    }
    impl ::core::convert::From<UnderlyingTokenCall> for IStrategyCalls {
        fn from(value: UnderlyingTokenCall) -> Self {
            Self::UnderlyingToken(value)
        }
    }
    impl ::core::convert::From<UserUnderlyingCall> for IStrategyCalls {
        fn from(value: UserUnderlyingCall) -> Self {
            Self::UserUnderlying(value)
        }
    }
    impl ::core::convert::From<UserUnderlyingViewCall> for IStrategyCalls {
        fn from(value: UserUnderlyingViewCall) -> Self {
            Self::UserUnderlyingView(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for IStrategyCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `deposit` function with signature `deposit(address,uint256)` and selector `0x47e7ef24`
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
    pub struct DepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `explanation` function with signature `explanation()` and selector `0xab5921e1`
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
    pub struct ExplanationReturn(pub ::std::string::String);
    ///Container type for all return fields from the `shares` function with signature `shares(address)` and selector `0xce7c2ac2`
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
    pub struct SharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sharesToUnderlying` function with signature `sharesToUnderlying(uint256)` and selector `0xf3e73875`
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
    pub struct SharesToUnderlyingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sharesToUnderlyingView` function with signature `sharesToUnderlyingView(uint256)` and selector `0x7a8b2637`
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
    pub struct SharesToUnderlyingViewReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalShares` function with signature `totalShares()` and selector `0x3a98ef39`
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
    pub struct TotalSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToShares` function with signature `underlyingToShares(uint256)` and selector `0x8c871019`
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
    pub struct UnderlyingToSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToSharesView` function with signature `underlyingToSharesView(uint256)` and selector `0xe3dae51c`
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
    pub struct UnderlyingToSharesViewReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `underlyingToken` function with signature `underlyingToken()` and selector `0x2495a599`
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
    pub struct UnderlyingTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `userUnderlying` function with signature `userUnderlying(address)` and selector `0x8f6a6240`
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
    pub struct UserUnderlyingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userUnderlyingView` function with signature `userUnderlyingView(address)` and selector `0x553ca5f8`
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
    pub struct UserUnderlyingViewReturn(pub ::ethers::core::types::U256);
}
