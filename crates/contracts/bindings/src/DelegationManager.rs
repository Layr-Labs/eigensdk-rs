pub use delegation_manager::*;
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
pub mod delegation_manager {
    const _: () = {
        ::core::include_bytes!(
            "/Users/supernovahs/Desktop/eigensdk-rs/crates/contracts/bindings/json/DelegationManager.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategyManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStrategyManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_slasher"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_eigenPodManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IEigenPodManager"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DELEGATION_APPROVAL_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DELEGATION_APPROVAL_TYPEHASH",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("MAX_STAKER_OPT_OUT_WINDOW_BLOCKS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_STAKER_OPT_OUT_WINDOW_BLOCKS",),
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
                    ::std::borrow::ToOwned::to_owned("MAX_WITHDRAWAL_DELAY_BLOCKS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_WITHDRAWAL_DELAY_BLOCKS",),
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
                    ::std::borrow::ToOwned::to_owned("STAKER_DELEGATION_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("STAKER_DELEGATION_TYPEHASH",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("beaconChainETHStrategy"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("beaconChainETHStrategy",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateCurrentStakerDelegationDigestHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "calculateCurrentStakerDelegationDigestHash",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateDelegationApprovalDigestHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "calculateDelegationApprovalDigestHash",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_delegationApprover",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approverSalt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateStakerDelegationDigestHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "calculateStakerDelegationDigestHash",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_stakerNonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IDelegationManager.Withdrawal",
                                ),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawal",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IDelegationManager.Withdrawal",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("middlewareTimesIndex",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiveAsTokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawals",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        256usize
                                                    ),
                                                ),
                                            ),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IDelegationManager.Withdrawal[]",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Address,
                                            ),
                                        ),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IERC20[][]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("middlewareTimesIndexes",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiveAsTokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cumulativeWithdrawalsQueued"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cumulativeWithdrawalsQueued",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("decreaseDelegatedShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decreaseDelegatedShares",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
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
                (
                    ::std::borrow::ToOwned::to_owned("delegateTo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegateTo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "approverSignatureAndExpiry",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ISignatureUtils.SignatureWithExpiry",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approverSalt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegateToBySignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegateToBySignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("stakerSignatureAndExpiry",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ISignatureUtils.SignatureWithExpiry",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "approverSignatureAndExpiry",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ISignatureUtils.SignatureWithExpiry",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approverSalt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegatedTo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegatedTo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("delegationApprover"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegationApprover"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("delegationApproverSaltIsSpent"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegationApproverSaltIsSpent",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("earningsReceiver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("earningsReceiver"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IEigenPodManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDelegatableShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDelegatableShares",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("staker"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorShares"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategies"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getWithdrawalDelay"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getWithdrawalDelay"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("strategies"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
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
                    ::std::borrow::ToOwned::to_owned("increaseDelegatedShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("increaseDelegatedShares",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
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
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialPausedStatus",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_minWithdrawalDelayBlocks",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_strategies"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawalDelayBlocks",),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDelegated"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDelegated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("staker"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawals",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("withdrawalsToMigrate",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Address,
                                            ),
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal[]",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minWithdrawalDelayBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minWithdrawalDelayBlocks",),
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
                    ::std::borrow::ToOwned::to_owned("modifyOperatorDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("modifyOperatorDetails",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOperatorDetails",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IDelegationManager.OperatorDetails",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorDetails"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IDelegationManager.OperatorDetails",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorShares"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorShares"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pause"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingWithdrawals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingWithdrawals"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queueWithdrawals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("queueWithdrawals"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("queuedWithdrawalParams",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Address,
                                            ),
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IDelegationManager.QueuedWithdrawalParams[]",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerAsOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerAsOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "registeringOperatorDetails",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IDelegationManager.OperatorDetails",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("metadataURI"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinWithdrawalDelayBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinWithdrawalDelayBlocks",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newMinWithdrawalDelayBlocks",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setStrategyWithdrawalDelayBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setStrategyWithdrawalDelayBlocks",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategies"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalDelayBlocks",),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("slasher"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakerNonce"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("stakerOptOutWindowBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakerOptOutWindowBlocks",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("strategyManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategyManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyWithdrawalDelayBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyWithdrawalDelayBlocks",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("undelegate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("undelegate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("staker"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("withdrawalRoots"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unpause"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateOperatorMetadataURI"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateOperatorMetadataURI",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("metadataURI"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinWithdrawalDelayBlocksSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinWithdrawalDelayBlocksSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousValue"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newValue"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorDetailsModified"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorDetailsModified",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOperatorDetails",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorMetadataURIUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorMetadataURIUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("metadataURI"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorDetails"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSharesDecreased"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorSharesDecreased",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSharesIncreased"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorSharesIncreased",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Paused"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerDelegated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakerDelegated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerForceUndelegated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakerForceUndelegated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerUndelegated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakerUndelegated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("staker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyWithdrawalDelayBlocksSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StrategyWithdrawalDelayBlocksSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousValue"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newValue"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalCompleted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawalCompleted",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalMigrated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawalMigrated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldWithdrawalRoot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newWithdrawalRoot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalQueued"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawalQueued"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DELEGATIONMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0a\x818\x03\x80b\0a\x81\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01@V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x80\x82\x16`\xC0R\x82\x16`\xA0Rb\0\0Xb\0\0eV[PPF`\xE0RPb\0\x01\x94V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01%W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01VW`\0\x80\xFD[\x83Qb\0\x01c\x81b\0\x01'V[` \x85\x01Q\x90\x93Pb\0\x01v\x81b\0\x01'V[`@\x85\x01Q\x90\x92Pb\0\x01\x89\x81b\0\x01'V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa_Qb\0\x020`\09`\0a'y\x01R`\0\x81\x81a\x05\xE7\x01R\x81\x81a\x10\xAE\x01R\x81\x81a\x14*\x01R\x81\x81a\x1E\r\x01R\x81\x81a*\xD2\x01R\x81\x81aA9\x01RaF%\x01R`\0a\x07\xD7\x01R`\0\x81\x81a\x05/\x01R\x81\x81a\x10|\x01R\x81\x81a\x13\xF8\x01R\x81\x81a\x17 \x01R\x81\x81a\x1E\xA1\x01R\x81\x81a+\x9F\x01R\x81\x81a-\"\x01R\x81\x81aB_\x01RaF\xCB\x01Ra_Q`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03xW`\x005`\xE0\x1C\x80c`\xD7\xFA\xED\x11a\x01\xD3W\x80c\xB7\xF0n\xBE\x11a\x01\x04W\x80c\xCF\x80\x87>\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t}W\x80c\xF2\xFD\xE3\x8B\x14a\t\x90W\x80c\xF6\x98\xDA%\x14a\t\xA3W\x80c\xFA\xBC\x1C\xBC\x14a\t\xABW`\0\x80\xFD[\x80c\xCF\x80\x87>\x14a\t6W\x80c\xDA\x8B\xE8d\x14a\tWW\x80c\xEE\xA9\x06K\x14a\tjW`\0\x80\xFD[\x80c\xC4\x887Z\x11a\0\xDEW\x80c\xC4\x887Z\x14a\x08SW\x80c\xC5\xE4\x80\xDB\x14a\x08sW\x80c\xC9KQ\x11\x14a\t\x19W\x80c\xCAf\x1C\x04\x14a\t,W`\0\x80\xFD[\x80c\xB7\xF0n\xBE\x14a\x07\xF9W\x80c\xBBE\xFE\xF2\x14a\x08\x1CW\x80c\xC4H\xFE\xB8\x14a\x08JW`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01qW\x80c\x91\x04\xC3\x19\x11a\x01KW\x80c\x91\x04\xC3\x19\x14a\x07\x84W\x80c\x99\xBE\x81\xC8\x14a\x07\x9FW\x80c\xA1x\x84\x84\x14a\x07\xB2W\x80c\xB14Bq\x14a\x07\xD2W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x07@W\x80c\x8D\xA5\xCB[\x14a\x07SW\x80c\x90\x04\x13G\x14a\x07dW`\0\x80\xFD[\x80cmp\xF7\xAE\x11a\x01\xADW\x80cmp\xF7\xAE\x14a\x06\xE7W\x80cqP\x18\xA6\x14a\x06\xFAW\x80cw\x8EU\xF3\x14a\x07\x02W\x80c\x7FT\x80q\x14a\x07-W`\0\x80\xFD[\x80c`\xD7\xFA\xED\x14a\x06\x98W\x80cc[\xBD\x10\x14a\x06\xABW\x80ce\xDA\x12d\x14a\x06\xBEW`\0\x80\xFD[\x80c)\xC7}O\x11a\x02\xADW\x80cO\xC4\x0Ba\x11a\x02KW\x80cZ\xC8j\xB7\x11a\x02%W\x80cZ\xC8j\xB7\x14a\x06.W\x80c\\\x97Z\xBB\x14a\x06QW\x80c\\\xFE\x8D,\x14a\x06YW\x80c_\x96o\x14\x14a\x06lW`\0\x80\xFD[\x80cO\xC4\x0Ba\x14a\x06\tW\x80cY\\jg\x14a\x06\x13W\x80cY{6\xDA\x14a\x06\x1BW`\0\x80\xFD[\x80c<\xDE\xB5\xE0\x11a\x02\x87W\x80c<\xDE\xB5\xE0\x14a\x05iW\x80c>(9\x1D\x14a\x05\x98W\x80cC7s\x82\x14a\x05\xBBW\x80cFe\xBC\xDA\x14a\x05\xE2W`\0\x80\xFD[\x80c)\xC7}O\x14a\x04\xF7W\x80c3@C\x96\x14a\x05\x17W\x80c9\xB7\x0E8\x14a\x05*W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x03\x1AW\x80c\x1B\xBC\xE0\x91\x11a\x02\xF4W\x80c\x1B\xBC\xE0\x91\x14a\x04\x97W\x80c `kp\x14a\x04\xAAW\x80c\"\xBF@\xE4\x14a\x04\xD1W\x80c(\xA5s\xAE\x14a\x04\xE4W`\0\x80\xFD[\x80c\x13d9\xDD\x14a\x048W\x80c\x15\"\xBF\x02\x14a\x04KW\x80c\x16\x92\x83e\x14a\x04^W`\0\x80\xFD[\x80c\r\xD8\xDD\x02\x11a\x03VW\x80c\r\xD8\xDD\x02\x14a\x03\xDDW\x80c\x0FX\x9EY\x14a\x03\xFDW\x80c\x10\xD6z/\x14a\x04\x12W\x80c\x13-Ig\x14a\x04%W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x03}W\x80c\x04\xA4\xF9y\x14a\x03\xA3W\x80c\x0B\x9FHz\x14a\x03\xCAW[`\0\x80\xFD[a\x03\x90a\x03\x8B6`\x04aJ\xDBV[a\t\xBEV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\x90\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[a\x03\x90a\x03\xD86`\x04aKAV[a\nCV[a\x03\xF0a\x03\xEB6`\x04aJ\xDBV[a\x0B\x05V[`@Qa\x03\x9A\x91\x90aK\x9CV[a\x04\x10a\x04\x0B6`\x04aL9V[a\x0EnV[\0[a\x04\x10a\x04 6`\x04aL\x8CV[a\x0F\xBEV[a\x04\x10a\x0436`\x04aL\xB0V[a\x10qV[a\x04\x10a\x04F6`\x04aL\xF1V[a\x11(V[a\x04\x10a\x04Y6`\x04aM\nV[a\x12gV[a\x03\x90a\x04l6`\x04aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03\x90a\x04\xA56`\x04aL\xB0V[a\x12{V[a\x03\x90\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x04\x10a\x04\xDF6`\x04aMuV[a\x12\xA9V[a\x04\x10a\x04\xF26`\x04aL\xB0V[a\x13\xEDV[a\x03\x90a\x05\x056`\x04aL\x8CV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x04\x10a\x05%6`\x04aN\x1CV[a\x14\x9DV[a\x05Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x9AV[a\x05Qa\x05w6`\x04aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05\xABa\x05\xA66`\x04aL\x8CV[a\x15\xDAV[`@Q\x90\x15\x15\x81R` \x01a\x03\x9AV[a\x03\x90\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x05Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x90b\x13\xC6\x80\x81V[a\x04\x10a\x15\xFAV[a\x03\x90a\x06)6`\x04aQ;V[a\x16\xC1V[a\x05\xABa\x06<6`\x04aQwV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03\x90V[a\x04\x10a\x06g6`\x04aQ\xEDV[a\x16\xF1V[a\x05Qa\x06z6`\x04aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x04\x10a\x06\xA66`\x04aSMV[a\x19\x9CV[a\x04\x10a\x06\xB96`\x04aL\xF1V[a\x1A7V[a\x05Qa\x06\xCC6`\x04aL\x8CV[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05\xABa\x06\xF56`\x04aL\x8CV[a\x1AHV[a\x04\x10a\x1AhV[a\x03\x90a\x07\x106`\x04aS\xDCV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x04\x10a\x07;6`\x04aT\xBDV[a\x1A|V[`eTa\x05Q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05QV[a\x07wa\x07r6`\x04aUMV[a\x1B\x81V[`@Qa\x03\x9A\x91\x90aU\xD7V[a\x05Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x04\x10a\x07\xAD6`\x04aU\xEAV[a\x1C[V[a\x03\x90a\x07\xC06`\x04aL\x8CV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x05Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xABa\x08\x076`\x04aL\xF1V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05\xABa\x08*6`\x04aV\x1FV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x03\x90`\x9DT\x81V[a\x03\x90a\x08a6`\x04aL\x8CV[`\xA1` R`\0\x90\x81R`@\x90 T\x81V[a\x08\xE3a\x08\x816`\x04aL\x8CV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03\x9AV[a\x03\x90a\t'6`\x04aVKV[a\x1D-V[a\x03\x90b\x03K\xC0\x81V[a\tIa\tD6`\x04aL\x8CV[a\x1D\xE6V[`@Qa\x03\x9A\x92\x91\x90aV\xCCV[a\x03\xF0a\te6`\x04aL\x8CV[a!\x9EV[a\x04\x10a\tx6`\x04aV\xF1V[a&bV[a\x04\x10a\t\x8B6`\x04aWIV[a&nV[a\x04\x10a\t\x9E6`\x04aL\x8CV[a&\xFFV[a\x03\x90a'uV[a\x04\x10a\t\xB96`\x04aL\xF1V[a'\xB3V[`\x9DT`\0\x90\x81[\x83\x81\x10\x15a\n;W`\0`\xA1`\0\x87\x87\x85\x81\x81\x10a\t\xE6Wa\t\xE6aWeV[\x90P` \x02\x01` \x81\x01\x90a\t\xFB\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x82\x81\x11\x15a\n*W\x80\x92P[Pa\n4\x81aW\x91V[\x90Pa\t\xC6V[P\x93\x92PPPV[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\n\xC1a'uV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0B:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BTWa\x0BTaN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3`\0\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0EcW\x86\x86\x82\x81\x81\x10a\x0B\xB8Wa\x0B\xB8aWeV[\x90P` \x02\x81\x01\x90a\x0B\xCA\x91\x90aW\xE3V[a\x0B\xD8\x90` \x81\x01\x90aX\x03V[\x90P\x87\x87\x83\x81\x81\x10a\x0B\xECWa\x0B\xECaWeV[\x90P` \x02\x81\x01\x90a\x0B\xFE\x91\x90aW\xE3V[a\x0C\x08\x90\x80aX\x03V[\x90P\x14a\x0C}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[3\x87\x87\x83\x81\x81\x10a\x0C\x90Wa\x0C\x90aWeV[\x90P` \x02\x81\x01\x90a\x0C\xA2\x91\x90aW\xE3V[a\x0C\xB3\x90``\x81\x01\x90`@\x01aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\r/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: withdrawer must be staker\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[a\x0E43\x83\x89\x89\x85\x81\x81\x10a\rFWa\rFaWeV[\x90P` \x02\x81\x01\x90a\rX\x91\x90aW\xE3V[a\ri\x90``\x81\x01\x90`@\x01aL\x8CV[\x8A\x8A\x86\x81\x81\x10a\r{Wa\r{aWeV[\x90P` \x02\x81\x01\x90a\r\x8D\x91\x90aW\xE3V[a\r\x97\x90\x80aX\x03V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\r\xDDWa\r\xDDaWeV[\x90P` \x02\x81\x01\x90a\r\xEF\x91\x90aW\xE3V[a\r\xFD\x90` \x81\x01\x90aX\x03V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\x0F\x92PPPV[\x83\x82\x81Q\x81\x10a\x0EFWa\x0EFaWeV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0E[\x81aW\x91V[\x91PPa\x0B\x9EV[P\x90\x95\x94PPPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a\x0F\x123\x84a.\xCFV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0F43\x80\x83`\0a1kV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0Fm\x91\x90aXLV[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F\xB0\x92\x91\x90aX\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x105\x91\x90aX\xCDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aX\xEAV[a\x10n\x81a5\x10V[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10\xD0WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY4V[a\x10\xF5\x83a\x15\xDAV[\x15a\x11#W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11!\x81\x85\x85\x85a6\x07V[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x94\x91\x90aY\x91V[a\x11\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY\xAEV[`fT\x81\x81\x16\x14a\x12)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x12oa6\x82V[a\x11!\x84\x84\x84\x84a6\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\xA0\x85\x82\x86\x86a\x1D-V[\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\xC9WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12\xE3WP0;\x15\x80\x15a\x12\xE3WP`\0T`\xFF\x16`\x01\x14[a\x13FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B1V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x13iW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x13s\x88\x88a9\x02V[a\x13{a9\xE8V[`\x97Ua\x13\x87\x89a:\x7FV[a\x13\x90\x86a:\xD1V[a\x13\x9C\x85\x85\x85\x85a6\xDCV[\x80\x15a\x13\xE2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x14LWP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x14hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY4V[a\x14q\x83a\x15\xDAV[\x15a\x11#W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11!\x81\x85\x85\x85a;\xCBV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x14\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[`\x02`\xC9T\x14\x15a\x15\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B1V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x15\xC9Wa\x15\xB9\x8A\x8A\x83\x81\x81\x10a\x15>Wa\x15>aWeV[\x90P` \x02\x81\x01\x90a\x15P\x91\x90aY\xF6V[\x89\x89\x84\x81\x81\x10a\x15bWa\x15baWeV[\x90P` \x02\x81\x01\x90a\x15t\x91\x90aX\x03V[\x89\x89\x86\x81\x81\x10a\x15\x86Wa\x15\x86aWeV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x15\x9FWa\x15\x9FaWeV[\x90P` \x02\x01` \x81\x01\x90a\x15\xB4\x91\x90aZ\x0CV[a<FV[a\x15\xC2\x81aW\x91V[\x90Pa\x15!V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16f\x91\x90aY\x91V[a\x16\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY\xAEV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16\xD4\x91\x90aZ\x9DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x19\x98W`\0\x82\x82\x81Q\x81\x10a\x17\x11Wa\x17\x11aWeV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17j\x91\x90aZ\xB0V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xAC\x91\x90a[\\V[\x91P\x91P\x81\x15a\x19\x8AW`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x17\xE3\x83aW\x91V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x18c\x82a\x16\xC1V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x18\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x19D\x90\x83\x90\x85\x90a[\x8AV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x16\xF4V[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x19\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[`\x02`\xC9T\x14\x15a\x1A\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B1V[`\x02`\xC9Ua\x1A*\x86\x86\x86\x86\x86a<FV[PP`\x01`\xC9UPPPPV[a\x1A?a6\x82V[a\x10n\x81a:\xD1V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x1Apa6\x82V[a\x1Az`\0a:\x7FV[V[B\x83` \x01Q\x10\x15a\x1B\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x1B<\x87\x83\x88\x88` \x01Qa\x1D-V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1Bl\x90\x88\x90\x83\x90aD0V[a\x1Bx\x87\x87\x86\x86a1kV[PPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x9EWa\x1B\x9EaN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\xC7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\n;W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1C\x05Wa\x1C\x05aWeV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1C@Wa\x1C@aWeV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x1CT\x81aW\x91V[\x90Pa\x1B\xCDV[a\x1Cd3a\x1AHV[a\x1C\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1D!\x92\x91\x90aX\x9EV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1D\xA3a'uV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ez\x91\x90a[\xA3V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x12\x91\x90\x81\x01\x90a\\\x17V[\x91P\x91P`\0\x83\x13a\x1F)W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1F\xE3W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1F\x9EWa\x1F\x9EaWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aWeV[` \x02` \x01\x01\x81\x81RPPa!\x91V[\x83Qa\x1F\xF0\x90`\x01a\\\xD1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a \x07Wa \x07aN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a 0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a LWa LaN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a uW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a!\x0FW\x84\x81\x81Q\x81\x10a \x96Wa \x96aWeV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a \xB0Wa \xB0aWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a \xE2Wa \xE2aWeV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a \xFCWa \xFCaWeV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a {V[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa!4\x91\x90a\\\xE9V[\x81Q\x81\x10a!DWa!DaWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa!t\x91\x90a\\\xE9V[\x81Q\x81\x10a!\x84Wa!\x84aWeV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a!\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[a!\xD3\x83a\x15\xDAV[a\"SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a\"\\\x83a\x1AHV[\x15a\"\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x83\x16a#KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a#~WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a#\xA5WP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a$\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\0\x80a$#\x86a\x1D\xE6V[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a$yW\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa$\xFBW`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa&YV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x14Wa%\x14aN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%=W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a&WW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a%\xA3Wa%\xA3aWeV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a%\xBEWa%\xBEaWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a%\xF0Wa%\xF0aWeV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a&\x0BWa&\x0BaWeV[` \x02` \x01\x01\x81\x81RPPa&$\x89\x87\x8B\x85\x85a)\x0FV[\x88\x84\x81Q\x81\x10a&6Wa&6aWeV[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a&O\x90aW\x91V[\x91PPa%CV[P[PPPP\x91\x90PV[a\x11#3\x84\x84\x84a1kV[a&w3a\x1AHV[a&\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a\x10n3\x82a.\xCFV[a'\x07a6\x82V[`\x01`\x01`\xA0\x1B\x03\x81\x16a'lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B1V[a\x10n\x81a:\x7FV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a'\xA6WP`\x97T\x90V[a'\xAEa9\xE8V[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(*\x91\x90aX\xCDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aX\xEAV[`fT\x19\x81\x19`fT\x19\x16\x14a(\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x12\\V[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a)\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x82Qa*0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0[\x83Q\x81\x10\x15a-\xDDW`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a*\x89Wa*\x89\x86\x88\x86\x84\x81Q\x81\x10a*bWa*baWeV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a*|Wa*|aWeV[` \x02` \x01\x01Qa6\x07V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a*\xB9Wa*\xB9aWeV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a+\x82W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a+\x12Wa+\x12aWeV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+K\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+eW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+yW=`\0\x80>=`\0\xFD[PPPPa-\xD5V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a,TWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9BM\xA0=\x85\x83\x81Q\x81\x10a+\xDEWa+\xDEaWeV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x11\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,R\x91\x90aY\x91V[\x15[a- W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x84`$\x82\x01\x81\x90R\x7FDelegationManager._removeSharesA`D\x83\x01R\x7FndQueueWithdrawal: withdrawer mu`d\x83\x01R\x7Fst be same address as staker if \x90\x82\x01R\x7FthirdPartyTransfersForbidden are`\xA4\x82\x01Rc\x08\x1C\xD9]`\xE2\x1B`\xC4\x82\x01R`\xE4\x01a\x0B1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a-bWa-baWeV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a-|Wa-|aWeV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\xA2\x93\x92\x91\x90a]\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\xBCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\xD0W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a*3V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a.\x05\x83aW\x91V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a.m\x82a\x16\xC1V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a.\xBB\x90\x83\x90\x85\x90a[\x8AV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a.\xDE` \x83\x01\x83aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a/xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[b\x13\xC6\x80a/\x8C``\x83\x01`@\x84\x01a]$V[c\xFF\xFF\xFF\xFF\x16\x11\x15a0AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a0}\x90``\x84\x01\x90\x84\x01a]$V[c\xFF\xFF\xFF\xFF\x16\x10\x15a1\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a17\x82\x82a]aV[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1D!\x90\x84\x90aXLV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a1\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[a1\x9D\x85a\x15\xDAV[\x15a2\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a2#\x84a\x1AHV[a2\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a2\xD9WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a2\xEEWP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a4[WB\x84` \x01Q\x10\x15a3mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a4\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa4H\x90\x88\x90\x88\x90\x85\x90\x88\x90a\nCV[\x90Pa4Y\x82\x82\x87`\0\x01QaD0V[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a4\xBA\x88a\x1D\xE6V[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x13\xE2Wa5\x08\x88\x8A\x85\x84\x81Q\x81\x10a4\xE1Wa4\xE1aWeV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a4\xFBWa4\xFBaWeV[` \x02` \x01\x01Qa;\xCBV[`\x01\x01a4\xC1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a5\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a6>\x90\x84\x90a\\\xE9V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F\xB0\x93\x92\x91\x90a]\0V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B1V[\x82\x81\x14a7dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: input lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x82`\0[\x81\x81\x10\x15a8\xFAW`\0\x86\x86\x83\x81\x81\x10a7\x84Wa7\x84aWeV[\x90P` \x02\x01` \x81\x01\x90a7\x99\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xA1` R`@\x81 T\x91\x92P\x86\x86\x85\x81\x81\x10a7\xC7Wa7\xC7aWeV[\x90P` \x02\x015\x90Pb\x03K\xC0\x81\x11\x15a8\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`s`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: _withdrawal`d\x82\x01R\x7FDelayBlocks cannot be > MAX_WITH`\x84\x82\x01RrDRAWAL_DELAY_BLOCKS`h\x1B`\xA4\x82\x01R`\xC4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\xA1` \x90\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x84\x90R\x81\x01\x82\x90R\x7F\x0E~\xFAs\x8E\x8B\x0C\xE67j\x0C\x1A\xF4qeU@\xD2\xE9\xA8\x16G\xD7\xB0\x9E\xD8#\x01\x84&Wm\x90``\x01`@Q\x80\x91\x03\x90\xA1PPP\x80a8\xF3\x90aW\x91V[\x90Pa7hV[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a9#WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x19\x98\x82a5\x10V[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\x03K\xC0\x81\x11\x15a;\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`q`$\x82\x01R\x7FDelegationManager._setMinWithdra`D\x82\x01R\x7FwalDelayBlocks: _minWithdrawalDe`d\x82\x01R\x7FlayBlocks cannot be > MAX_WITHDR`\x84\x82\x01RpAWAL_DELAY_BLOCKS`x\x1B`\xA4\x82\x01R`\xC4\x01a\x0B1V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xAF\xA0\x03\xCDv\xF8\x7F\xF9\xD6+5\xBE\xEA\x88\x99 \xF3<\x0CB\xB8\xD4[t\x95Ma\xD5\x0FKki\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a<\x02\x90\x84\x90a\\\xD1V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F\xB0\x93\x92\x91\x90a]\0V[`\0a<Ta\x06)\x87a]\xC4V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a<\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: action is not in qu`d\x82\x01Rbeue`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x9DTC\x90a<\xEA`\xA0\x89\x01`\x80\x8A\x01a]$V[c\xFF\xFF\xFF\xFF\x16a<\xFA\x91\x90a\\\xD1V[\x11\x15a=\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`_`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: minWithdrawalDelayB`d\x82\x01R\x7Flocks period has not yet passed\0`\x84\x82\x01R`\xA4\x01a\x0B1V[a=\x92``\x87\x01`@\x88\x01aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a>\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: only withdrawer can`d\x82\x01Ro\x101\xB7\xB6\xB862\xBA2\x900\xB1\xBA4\xB7\xB7`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x81\x15a>\xA1Wa>2`\xA0\x87\x01\x87aX\x03V[\x85\x14\x90Pa>\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: input length mismat`d\x82\x01Ra\x0Cm`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a@\x06W`\0[a>\xCD`\xA0\x88\x01\x88aX\x03V[\x90P\x81\x10\x15a@\0WC`\xA1`\0a>\xE8`\xA0\x8B\x01\x8BaX\x03V[\x85\x81\x81\x10a>\xF8Wa>\xF8aWeV[\x90P` \x02\x01` \x81\x01\x90a?\r\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta?7`\xA0\x8A\x01`\x80\x8B\x01a]$V[c\xFF\xFF\xFF\xFF\x16a?G\x91\x90a\\\xD1V[\x11\x15a?eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90a]\xD6V[a?\xF8a?u` \x89\x01\x89aL\x8CV[3a?\x83`\xA0\x8B\x01\x8BaX\x03V[\x85\x81\x81\x10a?\x93Wa?\x93aWeV[\x90P` \x02\x01` \x81\x01\x90a?\xA8\x91\x90aL\x8CV[a?\xB5`\xC0\x8C\x01\x8CaX\x03V[\x86\x81\x81\x10a?\xC5Wa?\xC5aWeV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a?\xDEWa?\xDEaWeV[\x90P` \x02\x01` \x81\x01\x90a?\xF3\x91\x90aL\x8CV[aE\xEAV[`\x01\x01a>\xC0V[PaC\xF5V[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a@.`\xA0\x89\x01\x89aX\x03V[\x90P\x81\x10\x15aC\xF2WC`\xA1`\0a@I`\xA0\x8C\x01\x8CaX\x03V[\x85\x81\x81\x10a@YWa@YaWeV[\x90P` \x02\x01` \x81\x01\x90a@n\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta@\x98`\xA0\x8B\x01`\x80\x8C\x01a]$V[c\xFF\xFF\xFF\xFF\x16a@\xA8\x91\x90a\\\xD1V[\x11\x15a@\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90a]\xD6V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a@\xE8`\xA0\x8A\x01\x8AaX\x03V[\x83\x81\x81\x10a@\xF8Wa@\xF8aWeV[\x90P` \x02\x01` \x81\x01\x90aA\r\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15aB]W`\0aA+` \x8A\x01\x8AaL\x8CV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83aAl`\xC0\x8E\x01\x8EaX\x03V[\x87\x81\x81\x10aA|WaA|aWeV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aA\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xF4\x91\x90a[\xA3V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15aBUWaBU\x81\x84aB*`\xA0\x8F\x01\x8FaX\x03V[\x88\x81\x81\x10aB:WaB:aWeV[\x90P` \x02\x01` \x81\x01\x90aBO\x91\x90aL\x8CV[\x85a;\xCBV[PPPaC\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA13\x89\x89\x85\x81\x81\x10aB\x9FWaB\x9FaWeV[\x90P` \x02\x01` \x81\x01\x90aB\xB4\x91\x90aL\x8CV[aB\xC1`\xA0\x8D\x01\x8DaX\x03V[\x86\x81\x81\x10aB\xD1WaB\xD1aWeV[\x90P` \x02\x01` \x81\x01\x90aB\xE6\x91\x90aL\x8CV[aB\xF3`\xC0\x8E\x01\x8EaX\x03V[\x87\x81\x81\x10aC\x03WaC\x03aWeV[`@Q`\xE0\x88\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x04\x82\x01R\x94\x86\x16`$\x86\x01R\x92\x90\x94\x16`D\x84\x01R` \x90\x91\x02\x015`d\x82\x01R`\x84\x01\x90P`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aCcW`\0\x80\xFD[PZ\xF1\x15\x80\x15aCwW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aC\xEAWaC\xEA\x823aC\x9C`\xA0\x8C\x01\x8CaX\x03V[\x85\x81\x81\x10aC\xACWaC\xACaWeV[\x90P` \x02\x01` \x81\x01\x90aC\xC1\x91\x90aL\x8CV[aC\xCE`\xC0\x8D\x01\x8DaX\x03V[\x86\x81\x81\x10aC\xDEWaC\xDEaWeV[\x90P` \x02\x015a;\xCBV[`\x01\x01a@!V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aEJW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aDp\x90\x86\x90\x86\x90`\x04\x01a^^V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xB1\x91\x90a^\xBBV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x82`\x01`\x01`\xA0\x1B\x03\x16aE^\x83\x83aG*V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aF\x95W`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aF^\x90\x88\x90\x88\x90\x87\x90`\x04\x01a]\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aFxW`\0\x80\xFD[PZ\xF1\x15\x80\x15aF\x8CW=`\0\x80>=`\0\xFD[PPPPaG#V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aG\x0FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xE2W=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80`\0aG9\x85\x85aGFV[\x91P\x91Pa\n;\x81aG\xB6V[`\0\x80\x82Q`A\x14\x15aG}W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaGq\x87\x82\x85\x85aIqV[\x94P\x94PPPPaG\xAFV[\x82Q`@\x14\x15aG\xA7W` \x83\x01Q`@\x84\x01QaG\x9C\x86\x83\x83aJ^V[\x93P\x93PPPaG\xAFV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aG\xCAWaG\xCAa^\xE5V[\x14\x15aG\xD3WPV[`\x01\x81`\x04\x81\x11\x15aG\xE7WaG\xE7a^\xE5V[\x14\x15aH5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B1V[`\x02\x81`\x04\x81\x11\x15aHIWaHIa^\xE5V[\x14\x15aH\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0B1V[`\x03\x81`\x04\x81\x11\x15aH\xABWaH\xABa^\xE5V[\x14\x15aI\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B1V[`\x04\x81`\x04\x81\x11\x15aI\x18WaI\x18a^\xE5V[\x14\x15a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B1V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aI\xA8WP`\0\x90P`\x03aJUV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aI\xC0WP\x84`\xFF\x16`\x1C\x14\x15[\x15aI\xD1WP`\0\x90P`\x04aJUV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aJ%W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJNW`\0`\x01\x92P\x92PPaJUV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aJ{`\xFF\x86\x90\x1C`\x1Ba\\\xD1V[\x90PaJ\x89\x87\x82\x88\x85aIqV[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aJ\xA9W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xC0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aG\xAFW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ\xEEW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x04W`\0\x80\xFD[aK\x10\x85\x82\x86\x01aJ\x97V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10nW`\0\x80\xFD[\x805aK<\x81aK\x1CV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aKYW`\0\x80\xFD[\x855aKd\x81aK\x1CV[\x94P` \x86\x015aKt\x81aK\x1CV[\x93P`@\x86\x015aK\x84\x81aK\x1CV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aK\xD4W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aK\xB8V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aK\xF2W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aL\nW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\xAFW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aLNW`\0\x80\xFD[aLX\x85\x85aK\xE0V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aLsW`\0\x80\xFD[aL\x7F\x86\x82\x87\x01aK\xF8V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aL\x9EW`\0\x80\xFD[\x815aL\xA9\x81aK\x1CV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aL\xC5W`\0\x80\xFD[\x835aL\xD0\x81aK\x1CV[\x92P` \x84\x015aL\xE0\x81aK\x1CV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aM\x03W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aM W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM7W`\0\x80\xFD[aMC\x88\x83\x89\x01aJ\x97V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aM\\W`\0\x80\xFD[PaMi\x87\x82\x88\x01aJ\x97V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aM\x91W`\0\x80\xFD[\x885aM\x9C\x81aK\x1CV[\x97P` \x89\x015aM\xAC\x81aK\x1CV[\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xD6W`\0\x80\xFD[aM\xE2\x8C\x83\x8D\x01aJ\x97V[\x90\x96P\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15aM\xFBW`\0\x80\xFD[PaN\x08\x8B\x82\x8C\x01aJ\x97V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aN8W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aNOW`\0\x80\xFD[aN[\x8C\x83\x8D\x01aJ\x97V[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aNtW`\0\x80\xFD[aN\x80\x8C\x83\x8D\x01aJ\x97V[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aN\x99W`\0\x80\xFD[aN\xA5\x8C\x83\x8D\x01aJ\x97V[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aM\xFBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xF6WaN\xF6aN\xBEV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xF6WaN\xF6aN\xBEV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xF6WaN\xF6aN\xBEV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aOhWaOhaN\xBEV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10nW`\0\x80\xFD[\x805aK<\x81aOpV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aO\xA6WaO\xA6aN\xBEV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aO\xC1W`\0\x80\xFD[\x815` aO\xD6aO\xD1\x83aO\x8DV[aO@V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aO\xF5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aP\x19W\x805aP\x0C\x81aK\x1CV[\x83R\x91\x83\x01\x91\x83\x01aO\xF9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aP5W`\0\x80\xFD[\x815` aPEaO\xD1\x83aO\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aPdW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aP\x19W\x805\x83R\x91\x83\x01\x91\x83\x01aPhV[`\0`\xE0\x82\x84\x03\x12\x15aP\x91W`\0\x80\xFD[aP\x99aN\xD4V[\x90PaP\xA4\x82aK1V[\x81RaP\xB2` \x83\x01aK1V[` \x82\x01RaP\xC3`@\x83\x01aK1V[`@\x82\x01R``\x82\x015``\x82\x01RaP\xDE`\x80\x83\x01aO\x82V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xFDW`\0\x80\xFD[aQ\t\x85\x83\x86\x01aO\xB0V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aQ\"W`\0\x80\xFD[PaQ/\x84\x82\x85\x01aP$V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aQMW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aQcW`\0\x80\xFD[aQo\x84\x82\x85\x01aP\x7FV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aQ\x89W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aL\xA9W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aQ\xACW`\0\x80\xFD[aQ\xB4aN\xFCV[\x90P\x815aQ\xC1\x81aK\x1CV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aQ\xE2W`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aR\0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aR\x17W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aR+W`\0\x80\xFD[\x815aR9aO\xD1\x82aO\x8DV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aRXW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aS2W\x805\x85\x81\x11\x15aRtW`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aR\x8CW`\0\x80\x81\xFD[aR\x94aO\x1EV[\x89\x83\x015\x88\x81\x11\x15aR\xA6W`\0\x80\x81\xFD[aR\xB4\x8E\x8C\x83\x87\x01\x01aO\xB0V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aR\xCBW`\0\x80\x81\xFD[aR\xD9\x8F\x8D\x83\x88\x01\x01aP$V[\x8C\x84\x01RP``aR\xEB\x81\x86\x01aK1V[\x82\x84\x01R`\x80\x91PaR\xFF\x8F\x83\x87\x01aQ\x9AV[\x90\x83\x01RaS\x0F`\xC0\x85\x01aO\x82V[\x90\x82\x01RaS\x1E\x83\x83\x01aK1V[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aR\\V[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x10nW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aSeW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aS|W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aS\x90W`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aS\xA6W`\0\x80\xFD[PaS\xB3\x88\x82\x89\x01aJ\x97V[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aS\xCE\x81aS?V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aS\xEFW`\0\x80\xFD[\x825aS\xFA\x81aK\x1CV[\x91P` \x83\x015aT\n\x81aK\x1CV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aT'W`\0\x80\xFD[aT/aN\xFCV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aTHW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aT\\W`\0\x80\xFD[\x815` \x82\x82\x11\x15aTpWaTpaN\xBEV[aT\x82`\x1F\x83\x01`\x1F\x19\x16\x82\x01aO@V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aT\x98W`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aT\xD5W`\0\x80\xFD[\x855aT\xE0\x81aK\x1CV[\x94P` \x86\x015aT\xF0\x81aK\x1CV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aU\x0CW`\0\x80\xFD[aU\x18\x89\x83\x8A\x01aT\x15V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aU.W`\0\x80\xFD[PaU;\x88\x82\x89\x01aT\x15V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aU`W`\0\x80\xFD[\x825aUk\x81aK\x1CV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\x86W`\0\x80\xFD[aU\x92\x85\x82\x86\x01aO\xB0V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xCCW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\xB0V[P\x94\x95\x94PPPPPV[` \x81R`\0aL\xA9` \x83\x01\x84aU\x9CV[`\0\x80` \x83\x85\x03\x12\x15aU\xFDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x13W`\0\x80\xFD[aK\x10\x85\x82\x86\x01aK\xF8V[`\0\x80`@\x83\x85\x03\x12\x15aV2W`\0\x80\xFD[\x825aV=\x81aK\x1CV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aVaW`\0\x80\xFD[\x845aVl\x81aK\x1CV[\x93P` \x85\x015\x92P`@\x85\x015aV\x83\x81aK\x1CV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xCCW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aV\xA7V[`@\x81R`\0aV\xDF`@\x83\x01\x85aV\x93V[\x82\x81\x03` \x84\x01Ra\x12\xA0\x81\x85aU\x9CV[`\0\x80`\0``\x84\x86\x03\x12\x15aW\x06W`\0\x80\xFD[\x835aW\x11\x81aK\x1CV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aW,W`\0\x80\xFD[aW8\x86\x82\x87\x01aT\x15V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aW[W`\0\x80\xFD[aL\xA9\x83\x83aK\xE0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aW\xA5WaW\xA5aW{V[P`\x01\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aW\xF9W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aX\x1AW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX4W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aG\xAFW`\0\x80\xFD[``\x81\x01\x825aX[\x81aK\x1CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aXw\x82aK\x1CV[\x16` \x83\x01R`@\x83\x015aX\x8B\x81aOpV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aX\xDFW`\0\x80\xFD[\x81QaL\xA9\x81aK\x1CV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aY\xA3W`\0\x80\xFD[\x81QaL\xA9\x81aS?V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aW\xF9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aZ\x1EW`\0\x80\xFD[\x815aL\xA9\x81aS?V[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaZ\x84`\xE0\x85\x01\x82aV\x93V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\xA0\x82\x82aU\x9CV[` \x81R`\0aL\xA9` \x83\x01\x84aZ)V[` \x81R`\0\x82Q`\xE0` \x84\x01RaZ\xCDa\x01\0\x84\x01\x82aV\x93V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaZ\xEA\x82\x82aU\x9CV[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01Qa[B`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01Ra\n;V[`\0\x80`@\x83\x85\x03\x12\x15a[oW`\0\x80\xFD[\x82Qa[z\x81aS?V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aQo`@\x83\x01\x84aZ)V[`\0` \x82\x84\x03\x12\x15a[\xB5W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a[\xCDW`\0\x80\xFD[\x81Q` a[\xDDaO\xD1\x83aO\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a[\xFCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aP\x19W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\\\0V[`\0\x80`@\x83\x85\x03\x12\x15a\\*W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\\AW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\\UW`\0\x80\xFD[\x81Q` a\\eaO\xD1\x83aO\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\\\x84W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\\\xABW\x85Qa\\\x9C\x81aK\x1CV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\\\x89V[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a\\\xC4W`\0\x80\xFD[PaU\x92\x85\x82\x86\x01a[\xBCV[`\0\x82\x19\x82\x11\x15a\\\xE4Wa\\\xE4aW{V[P\x01\x90V[`\0\x82\x82\x10\x15a\\\xFBWa\\\xFBaW{V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a]6W`\0\x80\xFD[\x815aL\xA9\x81aOpV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a]l\x81aK\x1CV[a]v\x81\x83a]AV[P`\x01\x81\x01` \x83\x015a]\x89\x81aK\x1CV[a]\x93\x81\x83a]AV[P`@\x83\x015a]\xA2\x81aOpV[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a]\xD06\x83aP\x7FV[\x92\x91PPV[` \x80\x82R`n\x90\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`@\x82\x01R\x7FdWithdrawal: withdrawalDelayBloc``\x82\x01R\x7Fks period has not yet passed for`\x80\x82\x01Rm this strategy`\x90\x1B`\xA0\x82\x01R`\xC0\x01\x90V[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a^\x92W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a^vV[\x81\x81\x11\x15a^\xA4W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a^\xCDW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aL\xA9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager._completeQueue\xA2dipfsX\"\x12 \x1F;\x96vQ[)\x03\xB8\xEA\x11\x8C?va\0\x14\xA4\x97.\xEB\xA1\xC64\x0F\xFB\xB0\xC5\xA3\x17\xF7\xF8dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static DELEGATIONMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03xW`\x005`\xE0\x1C\x80c`\xD7\xFA\xED\x11a\x01\xD3W\x80c\xB7\xF0n\xBE\x11a\x01\x04W\x80c\xCF\x80\x87>\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t}W\x80c\xF2\xFD\xE3\x8B\x14a\t\x90W\x80c\xF6\x98\xDA%\x14a\t\xA3W\x80c\xFA\xBC\x1C\xBC\x14a\t\xABW`\0\x80\xFD[\x80c\xCF\x80\x87>\x14a\t6W\x80c\xDA\x8B\xE8d\x14a\tWW\x80c\xEE\xA9\x06K\x14a\tjW`\0\x80\xFD[\x80c\xC4\x887Z\x11a\0\xDEW\x80c\xC4\x887Z\x14a\x08SW\x80c\xC5\xE4\x80\xDB\x14a\x08sW\x80c\xC9KQ\x11\x14a\t\x19W\x80c\xCAf\x1C\x04\x14a\t,W`\0\x80\xFD[\x80c\xB7\xF0n\xBE\x14a\x07\xF9W\x80c\xBBE\xFE\xF2\x14a\x08\x1CW\x80c\xC4H\xFE\xB8\x14a\x08JW`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01qW\x80c\x91\x04\xC3\x19\x11a\x01KW\x80c\x91\x04\xC3\x19\x14a\x07\x84W\x80c\x99\xBE\x81\xC8\x14a\x07\x9FW\x80c\xA1x\x84\x84\x14a\x07\xB2W\x80c\xB14Bq\x14a\x07\xD2W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x07@W\x80c\x8D\xA5\xCB[\x14a\x07SW\x80c\x90\x04\x13G\x14a\x07dW`\0\x80\xFD[\x80cmp\xF7\xAE\x11a\x01\xADW\x80cmp\xF7\xAE\x14a\x06\xE7W\x80cqP\x18\xA6\x14a\x06\xFAW\x80cw\x8EU\xF3\x14a\x07\x02W\x80c\x7FT\x80q\x14a\x07-W`\0\x80\xFD[\x80c`\xD7\xFA\xED\x14a\x06\x98W\x80cc[\xBD\x10\x14a\x06\xABW\x80ce\xDA\x12d\x14a\x06\xBEW`\0\x80\xFD[\x80c)\xC7}O\x11a\x02\xADW\x80cO\xC4\x0Ba\x11a\x02KW\x80cZ\xC8j\xB7\x11a\x02%W\x80cZ\xC8j\xB7\x14a\x06.W\x80c\\\x97Z\xBB\x14a\x06QW\x80c\\\xFE\x8D,\x14a\x06YW\x80c_\x96o\x14\x14a\x06lW`\0\x80\xFD[\x80cO\xC4\x0Ba\x14a\x06\tW\x80cY\\jg\x14a\x06\x13W\x80cY{6\xDA\x14a\x06\x1BW`\0\x80\xFD[\x80c<\xDE\xB5\xE0\x11a\x02\x87W\x80c<\xDE\xB5\xE0\x14a\x05iW\x80c>(9\x1D\x14a\x05\x98W\x80cC7s\x82\x14a\x05\xBBW\x80cFe\xBC\xDA\x14a\x05\xE2W`\0\x80\xFD[\x80c)\xC7}O\x14a\x04\xF7W\x80c3@C\x96\x14a\x05\x17W\x80c9\xB7\x0E8\x14a\x05*W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x03\x1AW\x80c\x1B\xBC\xE0\x91\x11a\x02\xF4W\x80c\x1B\xBC\xE0\x91\x14a\x04\x97W\x80c `kp\x14a\x04\xAAW\x80c\"\xBF@\xE4\x14a\x04\xD1W\x80c(\xA5s\xAE\x14a\x04\xE4W`\0\x80\xFD[\x80c\x13d9\xDD\x14a\x048W\x80c\x15\"\xBF\x02\x14a\x04KW\x80c\x16\x92\x83e\x14a\x04^W`\0\x80\xFD[\x80c\r\xD8\xDD\x02\x11a\x03VW\x80c\r\xD8\xDD\x02\x14a\x03\xDDW\x80c\x0FX\x9EY\x14a\x03\xFDW\x80c\x10\xD6z/\x14a\x04\x12W\x80c\x13-Ig\x14a\x04%W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x03}W\x80c\x04\xA4\xF9y\x14a\x03\xA3W\x80c\x0B\x9FHz\x14a\x03\xCAW[`\0\x80\xFD[a\x03\x90a\x03\x8B6`\x04aJ\xDBV[a\t\xBEV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\x90\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[a\x03\x90a\x03\xD86`\x04aKAV[a\nCV[a\x03\xF0a\x03\xEB6`\x04aJ\xDBV[a\x0B\x05V[`@Qa\x03\x9A\x91\x90aK\x9CV[a\x04\x10a\x04\x0B6`\x04aL9V[a\x0EnV[\0[a\x04\x10a\x04 6`\x04aL\x8CV[a\x0F\xBEV[a\x04\x10a\x0436`\x04aL\xB0V[a\x10qV[a\x04\x10a\x04F6`\x04aL\xF1V[a\x11(V[a\x04\x10a\x04Y6`\x04aM\nV[a\x12gV[a\x03\x90a\x04l6`\x04aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03\x90a\x04\xA56`\x04aL\xB0V[a\x12{V[a\x03\x90\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x04\x10a\x04\xDF6`\x04aMuV[a\x12\xA9V[a\x04\x10a\x04\xF26`\x04aL\xB0V[a\x13\xEDV[a\x03\x90a\x05\x056`\x04aL\x8CV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x04\x10a\x05%6`\x04aN\x1CV[a\x14\x9DV[a\x05Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x9AV[a\x05Qa\x05w6`\x04aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05\xABa\x05\xA66`\x04aL\x8CV[a\x15\xDAV[`@Q\x90\x15\x15\x81R` \x01a\x03\x9AV[a\x03\x90\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x05Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x90b\x13\xC6\x80\x81V[a\x04\x10a\x15\xFAV[a\x03\x90a\x06)6`\x04aQ;V[a\x16\xC1V[a\x05\xABa\x06<6`\x04aQwV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03\x90V[a\x04\x10a\x06g6`\x04aQ\xEDV[a\x16\xF1V[a\x05Qa\x06z6`\x04aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x04\x10a\x06\xA66`\x04aSMV[a\x19\x9CV[a\x04\x10a\x06\xB96`\x04aL\xF1V[a\x1A7V[a\x05Qa\x06\xCC6`\x04aL\x8CV[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05\xABa\x06\xF56`\x04aL\x8CV[a\x1AHV[a\x04\x10a\x1AhV[a\x03\x90a\x07\x106`\x04aS\xDCV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x04\x10a\x07;6`\x04aT\xBDV[a\x1A|V[`eTa\x05Q\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05QV[a\x07wa\x07r6`\x04aUMV[a\x1B\x81V[`@Qa\x03\x9A\x91\x90aU\xD7V[a\x05Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x04\x10a\x07\xAD6`\x04aU\xEAV[a\x1C[V[a\x03\x90a\x07\xC06`\x04aL\x8CV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x05Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xABa\x08\x076`\x04aL\xF1V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05\xABa\x08*6`\x04aV\x1FV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x03\x90`\x9DT\x81V[a\x03\x90a\x08a6`\x04aL\x8CV[`\xA1` R`\0\x90\x81R`@\x90 T\x81V[a\x08\xE3a\x08\x816`\x04aL\x8CV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03\x9AV[a\x03\x90a\t'6`\x04aVKV[a\x1D-V[a\x03\x90b\x03K\xC0\x81V[a\tIa\tD6`\x04aL\x8CV[a\x1D\xE6V[`@Qa\x03\x9A\x92\x91\x90aV\xCCV[a\x03\xF0a\te6`\x04aL\x8CV[a!\x9EV[a\x04\x10a\tx6`\x04aV\xF1V[a&bV[a\x04\x10a\t\x8B6`\x04aWIV[a&nV[a\x04\x10a\t\x9E6`\x04aL\x8CV[a&\xFFV[a\x03\x90a'uV[a\x04\x10a\t\xB96`\x04aL\xF1V[a'\xB3V[`\x9DT`\0\x90\x81[\x83\x81\x10\x15a\n;W`\0`\xA1`\0\x87\x87\x85\x81\x81\x10a\t\xE6Wa\t\xE6aWeV[\x90P` \x02\x01` \x81\x01\x90a\t\xFB\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x82\x81\x11\x15a\n*W\x80\x92P[Pa\n4\x81aW\x91V[\x90Pa\t\xC6V[P\x93\x92PPPV[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\n\xC1a'uV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0B:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BTWa\x0BTaN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3`\0\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0EcW\x86\x86\x82\x81\x81\x10a\x0B\xB8Wa\x0B\xB8aWeV[\x90P` \x02\x81\x01\x90a\x0B\xCA\x91\x90aW\xE3V[a\x0B\xD8\x90` \x81\x01\x90aX\x03V[\x90P\x87\x87\x83\x81\x81\x10a\x0B\xECWa\x0B\xECaWeV[\x90P` \x02\x81\x01\x90a\x0B\xFE\x91\x90aW\xE3V[a\x0C\x08\x90\x80aX\x03V[\x90P\x14a\x0C}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[3\x87\x87\x83\x81\x81\x10a\x0C\x90Wa\x0C\x90aWeV[\x90P` \x02\x81\x01\x90a\x0C\xA2\x91\x90aW\xE3V[a\x0C\xB3\x90``\x81\x01\x90`@\x01aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\r/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: withdrawer must be staker\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[a\x0E43\x83\x89\x89\x85\x81\x81\x10a\rFWa\rFaWeV[\x90P` \x02\x81\x01\x90a\rX\x91\x90aW\xE3V[a\ri\x90``\x81\x01\x90`@\x01aL\x8CV[\x8A\x8A\x86\x81\x81\x10a\r{Wa\r{aWeV[\x90P` \x02\x81\x01\x90a\r\x8D\x91\x90aW\xE3V[a\r\x97\x90\x80aX\x03V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\r\xDDWa\r\xDDaWeV[\x90P` \x02\x81\x01\x90a\r\xEF\x91\x90aW\xE3V[a\r\xFD\x90` \x81\x01\x90aX\x03V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\x0F\x92PPPV[\x83\x82\x81Q\x81\x10a\x0EFWa\x0EFaWeV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0E[\x81aW\x91V[\x91PPa\x0B\x9EV[P\x90\x95\x94PPPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a\x0F\x123\x84a.\xCFV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0F43\x80\x83`\0a1kV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0Fm\x91\x90aXLV[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F\xB0\x92\x91\x90aX\x9EV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x105\x91\x90aX\xCDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aX\xEAV[a\x10n\x81a5\x10V[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10\xD0WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY4V[a\x10\xF5\x83a\x15\xDAV[\x15a\x11#W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11!\x81\x85\x85\x85a6\x07V[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x94\x91\x90aY\x91V[a\x11\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY\xAEV[`fT\x81\x81\x16\x14a\x12)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x12oa6\x82V[a\x11!\x84\x84\x84\x84a6\xDCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\xA0\x85\x82\x86\x86a\x1D-V[\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\xC9WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12\xE3WP0;\x15\x80\x15a\x12\xE3WP`\0T`\xFF\x16`\x01\x14[a\x13FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B1V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x13iW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x13s\x88\x88a9\x02V[a\x13{a9\xE8V[`\x97Ua\x13\x87\x89a:\x7FV[a\x13\x90\x86a:\xD1V[a\x13\x9C\x85\x85\x85\x85a6\xDCV[\x80\x15a\x13\xE2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x14LWP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x14hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY4V[a\x14q\x83a\x15\xDAV[\x15a\x11#W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11!\x81\x85\x85\x85a;\xCBV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x14\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[`\x02`\xC9T\x14\x15a\x15\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B1V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x15\xC9Wa\x15\xB9\x8A\x8A\x83\x81\x81\x10a\x15>Wa\x15>aWeV[\x90P` \x02\x81\x01\x90a\x15P\x91\x90aY\xF6V[\x89\x89\x84\x81\x81\x10a\x15bWa\x15baWeV[\x90P` \x02\x81\x01\x90a\x15t\x91\x90aX\x03V[\x89\x89\x86\x81\x81\x10a\x15\x86Wa\x15\x86aWeV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x15\x9FWa\x15\x9FaWeV[\x90P` \x02\x01` \x81\x01\x90a\x15\xB4\x91\x90aZ\x0CV[a<FV[a\x15\xC2\x81aW\x91V[\x90Pa\x15!V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16f\x91\x90aY\x91V[a\x16\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aY\xAEV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16\xD4\x91\x90aZ\x9DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x19\x98W`\0\x82\x82\x81Q\x81\x10a\x17\x11Wa\x17\x11aWeV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17j\x91\x90aZ\xB0V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xAC\x91\x90a[\\V[\x91P\x91P\x81\x15a\x19\x8AW`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x17\xE3\x83aW\x91V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x18c\x82a\x16\xC1V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x18\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x19D\x90\x83\x90\x85\x90a[\x8AV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x16\xF4V[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x19\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[`\x02`\xC9T\x14\x15a\x1A\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B1V[`\x02`\xC9Ua\x1A*\x86\x86\x86\x86\x86a<FV[PP`\x01`\xC9UPPPPV[a\x1A?a6\x82V[a\x10n\x81a:\xD1V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x1Apa6\x82V[a\x1Az`\0a:\x7FV[V[B\x83` \x01Q\x10\x15a\x1B\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x1B<\x87\x83\x88\x88` \x01Qa\x1D-V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1Bl\x90\x88\x90\x83\x90aD0V[a\x1Bx\x87\x87\x86\x86a1kV[PPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x9EWa\x1B\x9EaN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\xC7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\n;W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1C\x05Wa\x1C\x05aWeV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1C@Wa\x1C@aWeV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x1CT\x81aW\x91V[\x90Pa\x1B\xCDV[a\x1Cd3a\x1AHV[a\x1C\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1D!\x92\x91\x90aX\x9EV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1D\xA3a'uV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ez\x91\x90a[\xA3V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x12\x91\x90\x81\x01\x90a\\\x17V[\x91P\x91P`\0\x83\x13a\x1F)W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1F\xE3W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1F\x9EWa\x1F\x9EaWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1F\xD2Wa\x1F\xD2aWeV[` \x02` \x01\x01\x81\x81RPPa!\x91V[\x83Qa\x1F\xF0\x90`\x01a\\\xD1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a \x07Wa \x07aN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a 0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a LWa LaN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a uW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a!\x0FW\x84\x81\x81Q\x81\x10a \x96Wa \x96aWeV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a \xB0Wa \xB0aWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a \xE2Wa \xE2aWeV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a \xFCWa \xFCaWeV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a {V[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa!4\x91\x90a\\\xE9V[\x81Q\x81\x10a!DWa!DaWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa!t\x91\x90a\\\xE9V[\x81Q\x81\x10a!\x84Wa!\x84aWeV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a!\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[a!\xD3\x83a\x15\xDAV[a\"SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a\"\\\x83a\x1AHV[\x15a\"\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x83\x16a#KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a#~WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a#\xA5WP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a$\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\0\x80a$#\x86a\x1D\xE6V[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a$yW\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa$\xFBW`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa&YV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x14Wa%\x14aN\xBEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%=W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a&WW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a%\xA3Wa%\xA3aWeV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a%\xBEWa%\xBEaWeV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a%\xF0Wa%\xF0aWeV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a&\x0BWa&\x0BaWeV[` \x02` \x01\x01\x81\x81RPPa&$\x89\x87\x8B\x85\x85a)\x0FV[\x88\x84\x81Q\x81\x10a&6Wa&6aWeV[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a&O\x90aW\x91V[\x91PPa%CV[P[PPPP\x91\x90PV[a\x11#3\x84\x84\x84a1kV[a&w3a\x1AHV[a&\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a\x10n3\x82a.\xCFV[a'\x07a6\x82V[`\x01`\x01`\xA0\x1B\x03\x81\x16a'lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B1V[a\x10n\x81a:\x7FV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a'\xA6WP`\x97T\x90V[a'\xAEa9\xE8V[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(*\x91\x90aX\xCDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(ZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aX\xEAV[`fT\x19\x81\x19`fT\x19\x16\x14a(\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x12\\V[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a)\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x82Qa*0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0[\x83Q\x81\x10\x15a-\xDDW`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a*\x89Wa*\x89\x86\x88\x86\x84\x81Q\x81\x10a*bWa*baWeV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a*|Wa*|aWeV[` \x02` \x01\x01Qa6\x07V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a*\xB9Wa*\xB9aWeV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a+\x82W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a+\x12Wa+\x12aWeV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+K\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+eW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+yW=`\0\x80>=`\0\xFD[PPPPa-\xD5V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a,TWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9BM\xA0=\x85\x83\x81Q\x81\x10a+\xDEWa+\xDEaWeV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\x11\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,R\x91\x90aY\x91V[\x15[a- W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x84`$\x82\x01\x81\x90R\x7FDelegationManager._removeSharesA`D\x83\x01R\x7FndQueueWithdrawal: withdrawer mu`d\x83\x01R\x7Fst be same address as staker if \x90\x82\x01R\x7FthirdPartyTransfersForbidden are`\xA4\x82\x01Rc\x08\x1C\xD9]`\xE2\x1B`\xC4\x82\x01R`\xE4\x01a\x0B1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a-bWa-baWeV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a-|Wa-|aWeV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-\xA2\x93\x92\x91\x90a]\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\xBCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\xD0W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a*3V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a.\x05\x83aW\x91V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a.m\x82a\x16\xC1V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a.\xBB\x90\x83\x90\x85\x90a[\x8AV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a.\xDE` \x83\x01\x83aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a/xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[b\x13\xC6\x80a/\x8C``\x83\x01`@\x84\x01a]$V[c\xFF\xFF\xFF\xFF\x16\x11\x15a0AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a0}\x90``\x84\x01\x90\x84\x01a]$V[c\xFF\xFF\xFF\xFF\x16\x10\x15a1\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a17\x82\x82a]aV[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1D!\x90\x84\x90aXLV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a1\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90aW\xACV[a1\x9D\x85a\x15\xDAV[\x15a2\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[a2#\x84a\x1AHV[a2\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a2\xD9WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a2\xEEWP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a4[WB\x84` \x01Q\x10\x15a3mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a4\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa4H\x90\x88\x90\x88\x90\x85\x90\x88\x90a\nCV[\x90Pa4Y\x82\x82\x87`\0\x01QaD0V[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a4\xBA\x88a\x1D\xE6V[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x13\xE2Wa5\x08\x88\x8A\x85\x84\x81Q\x81\x10a4\xE1Wa4\xE1aWeV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a4\xFBWa4\xFBaWeV[` \x02` \x01\x01Qa;\xCBV[`\x01\x01a4\xC1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a5\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a6>\x90\x84\x90a\\\xE9V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F\xB0\x93\x92\x91\x90a]\0V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B1V[\x82\x81\x14a7dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: input lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x82`\0[\x81\x81\x10\x15a8\xFAW`\0\x86\x86\x83\x81\x81\x10a7\x84Wa7\x84aWeV[\x90P` \x02\x01` \x81\x01\x90a7\x99\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xA1` R`@\x81 T\x91\x92P\x86\x86\x85\x81\x81\x10a7\xC7Wa7\xC7aWeV[\x90P` \x02\x015\x90Pb\x03K\xC0\x81\x11\x15a8\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`s`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: _withdrawal`d\x82\x01R\x7FDelayBlocks cannot be > MAX_WITH`\x84\x82\x01RrDRAWAL_DELAY_BLOCKS`h\x1B`\xA4\x82\x01R`\xC4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\xA1` \x90\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x84\x90R\x81\x01\x82\x90R\x7F\x0E~\xFAs\x8E\x8B\x0C\xE67j\x0C\x1A\xF4qeU@\xD2\xE9\xA8\x16G\xD7\xB0\x9E\xD8#\x01\x84&Wm\x90``\x01`@Q\x80\x91\x03\x90\xA1PPP\x80a8\xF3\x90aW\x91V[\x90Pa7hV[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a9#WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x19\x98\x82a5\x10V[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\x03K\xC0\x81\x11\x15a;\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`q`$\x82\x01R\x7FDelegationManager._setMinWithdra`D\x82\x01R\x7FwalDelayBlocks: _minWithdrawalDe`d\x82\x01R\x7FlayBlocks cannot be > MAX_WITHDR`\x84\x82\x01RpAWAL_DELAY_BLOCKS`x\x1B`\xA4\x82\x01R`\xC4\x01a\x0B1V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xAF\xA0\x03\xCDv\xF8\x7F\xF9\xD6+5\xBE\xEA\x88\x99 \xF3<\x0CB\xB8\xD4[t\x95Ma\xD5\x0FKki\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a<\x02\x90\x84\x90a\\\xD1V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F\xB0\x93\x92\x91\x90a]\0V[`\0a<Ta\x06)\x87a]\xC4V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a<\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: action is not in qu`d\x82\x01Rbeue`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x9DTC\x90a<\xEA`\xA0\x89\x01`\x80\x8A\x01a]$V[c\xFF\xFF\xFF\xFF\x16a<\xFA\x91\x90a\\\xD1V[\x11\x15a=\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`_`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: minWithdrawalDelayB`d\x82\x01R\x7Flocks period has not yet passed\0`\x84\x82\x01R`\xA4\x01a\x0B1V[a=\x92``\x87\x01`@\x88\x01aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a>\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: only withdrawer can`d\x82\x01Ro\x101\xB7\xB6\xB862\xBA2\x900\xB1\xBA4\xB7\xB7`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x81\x15a>\xA1Wa>2`\xA0\x87\x01\x87aX\x03V[\x85\x14\x90Pa>\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: input length mismat`d\x82\x01Ra\x0Cm`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a@\x06W`\0[a>\xCD`\xA0\x88\x01\x88aX\x03V[\x90P\x81\x10\x15a@\0WC`\xA1`\0a>\xE8`\xA0\x8B\x01\x8BaX\x03V[\x85\x81\x81\x10a>\xF8Wa>\xF8aWeV[\x90P` \x02\x01` \x81\x01\x90a?\r\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta?7`\xA0\x8A\x01`\x80\x8B\x01a]$V[c\xFF\xFF\xFF\xFF\x16a?G\x91\x90a\\\xD1V[\x11\x15a?eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90a]\xD6V[a?\xF8a?u` \x89\x01\x89aL\x8CV[3a?\x83`\xA0\x8B\x01\x8BaX\x03V[\x85\x81\x81\x10a?\x93Wa?\x93aWeV[\x90P` \x02\x01` \x81\x01\x90a?\xA8\x91\x90aL\x8CV[a?\xB5`\xC0\x8C\x01\x8CaX\x03V[\x86\x81\x81\x10a?\xC5Wa?\xC5aWeV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a?\xDEWa?\xDEaWeV[\x90P` \x02\x01` \x81\x01\x90a?\xF3\x91\x90aL\x8CV[aE\xEAV[`\x01\x01a>\xC0V[PaC\xF5V[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a@.`\xA0\x89\x01\x89aX\x03V[\x90P\x81\x10\x15aC\xF2WC`\xA1`\0a@I`\xA0\x8C\x01\x8CaX\x03V[\x85\x81\x81\x10a@YWa@YaWeV[\x90P` \x02\x01` \x81\x01\x90a@n\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta@\x98`\xA0\x8B\x01`\x80\x8C\x01a]$V[c\xFF\xFF\xFF\xFF\x16a@\xA8\x91\x90a\\\xD1V[\x11\x15a@\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B1\x90a]\xD6V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a@\xE8`\xA0\x8A\x01\x8AaX\x03V[\x83\x81\x81\x10a@\xF8Wa@\xF8aWeV[\x90P` \x02\x01` \x81\x01\x90aA\r\x91\x90aL\x8CV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15aB]W`\0aA+` \x8A\x01\x8AaL\x8CV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83aAl`\xC0\x8E\x01\x8EaX\x03V[\x87\x81\x81\x10aA|WaA|aWeV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aA\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xF4\x91\x90a[\xA3V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15aBUWaBU\x81\x84aB*`\xA0\x8F\x01\x8FaX\x03V[\x88\x81\x81\x10aB:WaB:aWeV[\x90P` \x02\x01` \x81\x01\x90aBO\x91\x90aL\x8CV[\x85a;\xCBV[PPPaC\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA13\x89\x89\x85\x81\x81\x10aB\x9FWaB\x9FaWeV[\x90P` \x02\x01` \x81\x01\x90aB\xB4\x91\x90aL\x8CV[aB\xC1`\xA0\x8D\x01\x8DaX\x03V[\x86\x81\x81\x10aB\xD1WaB\xD1aWeV[\x90P` \x02\x01` \x81\x01\x90aB\xE6\x91\x90aL\x8CV[aB\xF3`\xC0\x8E\x01\x8EaX\x03V[\x87\x81\x81\x10aC\x03WaC\x03aWeV[`@Q`\xE0\x88\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x04\x82\x01R\x94\x86\x16`$\x86\x01R\x92\x90\x94\x16`D\x84\x01R` \x90\x91\x02\x015`d\x82\x01R`\x84\x01\x90P`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aCcW`\0\x80\xFD[PZ\xF1\x15\x80\x15aCwW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aC\xEAWaC\xEA\x823aC\x9C`\xA0\x8C\x01\x8CaX\x03V[\x85\x81\x81\x10aC\xACWaC\xACaWeV[\x90P` \x02\x01` \x81\x01\x90aC\xC1\x91\x90aL\x8CV[aC\xCE`\xC0\x8D\x01\x8DaX\x03V[\x86\x81\x81\x10aC\xDEWaC\xDEaWeV[\x90P` \x02\x015a;\xCBV[`\x01\x01a@!V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aEJW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aDp\x90\x86\x90\x86\x90`\x04\x01a^^V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xB1\x91\x90a^\xBBV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[\x82`\x01`\x01`\xA0\x1B\x03\x16aE^\x83\x83aG*V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B1V[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aF\x95W`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aF^\x90\x88\x90\x88\x90\x87\x90`\x04\x01a]\0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aFxW`\0\x80\xFD[PZ\xF1\x15\x80\x15aF\x8CW=`\0\x80>=`\0\xFD[PPPPaG#V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aG\x0FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xE2W=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80`\0aG9\x85\x85aGFV[\x91P\x91Pa\n;\x81aG\xB6V[`\0\x80\x82Q`A\x14\x15aG}W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaGq\x87\x82\x85\x85aIqV[\x94P\x94PPPPaG\xAFV[\x82Q`@\x14\x15aG\xA7W` \x83\x01Q`@\x84\x01QaG\x9C\x86\x83\x83aJ^V[\x93P\x93PPPaG\xAFV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aG\xCAWaG\xCAa^\xE5V[\x14\x15aG\xD3WPV[`\x01\x81`\x04\x81\x11\x15aG\xE7WaG\xE7a^\xE5V[\x14\x15aH5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B1V[`\x02\x81`\x04\x81\x11\x15aHIWaHIa^\xE5V[\x14\x15aH\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0B1V[`\x03\x81`\x04\x81\x11\x15aH\xABWaH\xABa^\xE5V[\x14\x15aI\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B1V[`\x04\x81`\x04\x81\x11\x15aI\x18WaI\x18a^\xE5V[\x14\x15a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B1V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aI\xA8WP`\0\x90P`\x03aJUV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aI\xC0WP\x84`\xFF\x16`\x1C\x14\x15[\x15aI\xD1WP`\0\x90P`\x04aJUV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aJ%W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJNW`\0`\x01\x92P\x92PPaJUV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aJ{`\xFF\x86\x90\x1C`\x1Ba\\\xD1V[\x90PaJ\x89\x87\x82\x88\x85aIqV[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aJ\xA9W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xC0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aG\xAFW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ\xEEW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x04W`\0\x80\xFD[aK\x10\x85\x82\x86\x01aJ\x97V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10nW`\0\x80\xFD[\x805aK<\x81aK\x1CV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aKYW`\0\x80\xFD[\x855aKd\x81aK\x1CV[\x94P` \x86\x015aKt\x81aK\x1CV[\x93P`@\x86\x015aK\x84\x81aK\x1CV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aK\xD4W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aK\xB8V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aK\xF2W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aL\nW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL!W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\xAFW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aLNW`\0\x80\xFD[aLX\x85\x85aK\xE0V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aLsW`\0\x80\xFD[aL\x7F\x86\x82\x87\x01aK\xF8V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aL\x9EW`\0\x80\xFD[\x815aL\xA9\x81aK\x1CV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aL\xC5W`\0\x80\xFD[\x835aL\xD0\x81aK\x1CV[\x92P` \x84\x015aL\xE0\x81aK\x1CV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aM\x03W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aM W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM7W`\0\x80\xFD[aMC\x88\x83\x89\x01aJ\x97V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aM\\W`\0\x80\xFD[PaMi\x87\x82\x88\x01aJ\x97V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aM\x91W`\0\x80\xFD[\x885aM\x9C\x81aK\x1CV[\x97P` \x89\x015aM\xAC\x81aK\x1CV[\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xD6W`\0\x80\xFD[aM\xE2\x8C\x83\x8D\x01aJ\x97V[\x90\x96P\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15aM\xFBW`\0\x80\xFD[PaN\x08\x8B\x82\x8C\x01aJ\x97V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aN8W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aNOW`\0\x80\xFD[aN[\x8C\x83\x8D\x01aJ\x97V[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aNtW`\0\x80\xFD[aN\x80\x8C\x83\x8D\x01aJ\x97V[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aN\x99W`\0\x80\xFD[aN\xA5\x8C\x83\x8D\x01aJ\x97V[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aM\xFBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xF6WaN\xF6aN\xBEV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xF6WaN\xF6aN\xBEV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xF6WaN\xF6aN\xBEV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aOhWaOhaN\xBEV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10nW`\0\x80\xFD[\x805aK<\x81aOpV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aO\xA6WaO\xA6aN\xBEV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aO\xC1W`\0\x80\xFD[\x815` aO\xD6aO\xD1\x83aO\x8DV[aO@V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aO\xF5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aP\x19W\x805aP\x0C\x81aK\x1CV[\x83R\x91\x83\x01\x91\x83\x01aO\xF9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aP5W`\0\x80\xFD[\x815` aPEaO\xD1\x83aO\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aPdW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aP\x19W\x805\x83R\x91\x83\x01\x91\x83\x01aPhV[`\0`\xE0\x82\x84\x03\x12\x15aP\x91W`\0\x80\xFD[aP\x99aN\xD4V[\x90PaP\xA4\x82aK1V[\x81RaP\xB2` \x83\x01aK1V[` \x82\x01RaP\xC3`@\x83\x01aK1V[`@\x82\x01R``\x82\x015``\x82\x01RaP\xDE`\x80\x83\x01aO\x82V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xFDW`\0\x80\xFD[aQ\t\x85\x83\x86\x01aO\xB0V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aQ\"W`\0\x80\xFD[PaQ/\x84\x82\x85\x01aP$V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aQMW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aQcW`\0\x80\xFD[aQo\x84\x82\x85\x01aP\x7FV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aQ\x89W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aL\xA9W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aQ\xACW`\0\x80\xFD[aQ\xB4aN\xFCV[\x90P\x815aQ\xC1\x81aK\x1CV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aQ\xE2W`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aR\0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aR\x17W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aR+W`\0\x80\xFD[\x815aR9aO\xD1\x82aO\x8DV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aRXW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aS2W\x805\x85\x81\x11\x15aRtW`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aR\x8CW`\0\x80\x81\xFD[aR\x94aO\x1EV[\x89\x83\x015\x88\x81\x11\x15aR\xA6W`\0\x80\x81\xFD[aR\xB4\x8E\x8C\x83\x87\x01\x01aO\xB0V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aR\xCBW`\0\x80\x81\xFD[aR\xD9\x8F\x8D\x83\x88\x01\x01aP$V[\x8C\x84\x01RP``aR\xEB\x81\x86\x01aK1V[\x82\x84\x01R`\x80\x91PaR\xFF\x8F\x83\x87\x01aQ\x9AV[\x90\x83\x01RaS\x0F`\xC0\x85\x01aO\x82V[\x90\x82\x01RaS\x1E\x83\x83\x01aK1V[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aR\\V[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x10nW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aSeW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aS|W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aS\x90W`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aS\xA6W`\0\x80\xFD[PaS\xB3\x88\x82\x89\x01aJ\x97V[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aS\xCE\x81aS?V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aS\xEFW`\0\x80\xFD[\x825aS\xFA\x81aK\x1CV[\x91P` \x83\x015aT\n\x81aK\x1CV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aT'W`\0\x80\xFD[aT/aN\xFCV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aTHW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aT\\W`\0\x80\xFD[\x815` \x82\x82\x11\x15aTpWaTpaN\xBEV[aT\x82`\x1F\x83\x01`\x1F\x19\x16\x82\x01aO@V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aT\x98W`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aT\xD5W`\0\x80\xFD[\x855aT\xE0\x81aK\x1CV[\x94P` \x86\x015aT\xF0\x81aK\x1CV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aU\x0CW`\0\x80\xFD[aU\x18\x89\x83\x8A\x01aT\x15V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aU.W`\0\x80\xFD[PaU;\x88\x82\x89\x01aT\x15V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aU`W`\0\x80\xFD[\x825aUk\x81aK\x1CV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\x86W`\0\x80\xFD[aU\x92\x85\x82\x86\x01aO\xB0V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xCCW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\xB0V[P\x94\x95\x94PPPPPV[` \x81R`\0aL\xA9` \x83\x01\x84aU\x9CV[`\0\x80` \x83\x85\x03\x12\x15aU\xFDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x13W`\0\x80\xFD[aK\x10\x85\x82\x86\x01aK\xF8V[`\0\x80`@\x83\x85\x03\x12\x15aV2W`\0\x80\xFD[\x825aV=\x81aK\x1CV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aVaW`\0\x80\xFD[\x845aVl\x81aK\x1CV[\x93P` \x85\x015\x92P`@\x85\x015aV\x83\x81aK\x1CV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xCCW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aV\xA7V[`@\x81R`\0aV\xDF`@\x83\x01\x85aV\x93V[\x82\x81\x03` \x84\x01Ra\x12\xA0\x81\x85aU\x9CV[`\0\x80`\0``\x84\x86\x03\x12\x15aW\x06W`\0\x80\xFD[\x835aW\x11\x81aK\x1CV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aW,W`\0\x80\xFD[aW8\x86\x82\x87\x01aT\x15V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aW[W`\0\x80\xFD[aL\xA9\x83\x83aK\xE0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aW\xA5WaW\xA5aW{V[P`\x01\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aW\xF9W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aX\x1AW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX4W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aG\xAFW`\0\x80\xFD[``\x81\x01\x825aX[\x81aK\x1CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aXw\x82aK\x1CV[\x16` \x83\x01R`@\x83\x015aX\x8B\x81aOpV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aX\xDFW`\0\x80\xFD[\x81QaL\xA9\x81aK\x1CV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aY\xA3W`\0\x80\xFD[\x81QaL\xA9\x81aS?V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aW\xF9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aZ\x1EW`\0\x80\xFD[\x815aL\xA9\x81aS?V[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaZ\x84`\xE0\x85\x01\x82aV\x93V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\xA0\x82\x82aU\x9CV[` \x81R`\0aL\xA9` \x83\x01\x84aZ)V[` \x81R`\0\x82Q`\xE0` \x84\x01RaZ\xCDa\x01\0\x84\x01\x82aV\x93V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaZ\xEA\x82\x82aU\x9CV[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01Qa[B`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01Ra\n;V[`\0\x80`@\x83\x85\x03\x12\x15a[oW`\0\x80\xFD[\x82Qa[z\x81aS?V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aQo`@\x83\x01\x84aZ)V[`\0` \x82\x84\x03\x12\x15a[\xB5W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a[\xCDW`\0\x80\xFD[\x81Q` a[\xDDaO\xD1\x83aO\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a[\xFCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aP\x19W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\\\0V[`\0\x80`@\x83\x85\x03\x12\x15a\\*W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\\AW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\\UW`\0\x80\xFD[\x81Q` a\\eaO\xD1\x83aO\x8DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\\\x84W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\\\xABW\x85Qa\\\x9C\x81aK\x1CV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\\\x89V[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a\\\xC4W`\0\x80\xFD[PaU\x92\x85\x82\x86\x01a[\xBCV[`\0\x82\x19\x82\x11\x15a\\\xE4Wa\\\xE4aW{V[P\x01\x90V[`\0\x82\x82\x10\x15a\\\xFBWa\\\xFBaW{V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a]6W`\0\x80\xFD[\x815aL\xA9\x81aOpV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a]l\x81aK\x1CV[a]v\x81\x83a]AV[P`\x01\x81\x01` \x83\x015a]\x89\x81aK\x1CV[a]\x93\x81\x83a]AV[P`@\x83\x015a]\xA2\x81aOpV[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a]\xD06\x83aP\x7FV[\x92\x91PPV[` \x80\x82R`n\x90\x82\x01R`\0\x80Q` a^\xFC\x839\x81Q\x91R`@\x82\x01R\x7FdWithdrawal: withdrawalDelayBloc``\x82\x01R\x7Fks period has not yet passed for`\x80\x82\x01Rm this strategy`\x90\x1B`\xA0\x82\x01R`\xC0\x01\x90V[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a^\x92W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a^vV[\x81\x81\x11\x15a^\xA4W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a^\xCDW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aL\xA9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager._completeQueue\xA2dipfsX\"\x12 \x1F;\x96vQ[)\x03\xB8\xEA\x11\x8C?va\0\x14\xA4\x97.\xEB\xA1\xC64\x0F\xFB\xB0\xC5\xA3\x17\xF7\xF8dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static DELEGATIONMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DelegationManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DelegationManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DelegationManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DelegationManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DelegationManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DelegationManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DelegationManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DELEGATIONMANAGER_ABI.clone(),
                client,
            ))
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
                DELEGATIONMANAGER_ABI.clone(),
                DELEGATIONMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DELEGATION_APPROVAL_TYPEHASH` (0x04a4f979) function
        pub fn delegation_approval_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([4, 164, 249, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function
        pub fn domain_typehash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS` (0x4fc40b61) function
        pub fn max_staker_opt_out_window_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 196, 11, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_WITHDRAWAL_DELAY_BLOCKS` (0xca661c04) function
        pub fn max_withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 102, 28, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKER_DELEGATION_TYPEHASH` (0x43377382) function
        pub fn staker_delegation_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 55, 115, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beaconChainETHStrategy` (0x9104c319) function
        pub fn beacon_chain_eth_strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([145, 4, 195, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateCurrentStakerDelegationDigestHash` (0x1bbce091) function
        pub fn calculate_current_staker_delegation_digest_hash(
            &self,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 188, 224, 145], (staker, operator, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDelegationApprovalDigestHash` (0x0b9f487a) function
        pub fn calculate_delegation_approval_digest_hash(
            &self,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            delegation_approver: ::ethers::core::types::Address,
            approver_salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [11, 159, 72, 122],
                    (staker, operator, delegation_approver, approver_salt, expiry),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateStakerDelegationDigestHash` (0xc94b5111) function
        pub fn calculate_staker_delegation_digest_hash(
            &self,
            staker: ::ethers::core::types::Address,
            staker_nonce: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([201, 75, 81, 17], (staker, staker_nonce, operator, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateWithdrawalRoot` (0x597b36da) function
        pub fn calculate_withdrawal_root(
            &self,
            withdrawal: Withdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([89, 123, 54, 218], (withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeQueuedWithdrawal` (0x60d7faed) function
        pub fn complete_queued_withdrawal(
            &self,
            withdrawal: Withdrawal,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            middleware_times_index: ::ethers::core::types::U256,
            receive_as_tokens: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [96, 215, 250, 237],
                    (
                        withdrawal,
                        tokens,
                        middleware_times_index,
                        receive_as_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeQueuedWithdrawals` (0x33404396) function
        pub fn complete_queued_withdrawals(
            &self,
            withdrawals: ::std::vec::Vec<Withdrawal>,
            tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
            middleware_times_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
            receive_as_tokens: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [51, 64, 67, 150],
                    (
                        withdrawals,
                        tokens,
                        middleware_times_indexes,
                        receive_as_tokens,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeWithdrawalsQueued` (0xa1788484) function
        pub fn cumulative_withdrawals_queued(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 120, 132, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseDelegatedShares` (0x132d4967) function
        pub fn decrease_delegated_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 45, 73, 103], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateTo` (0xeea9064b) function
        pub fn delegate_to(
            &self,
            operator: ::ethers::core::types::Address,
            approver_signature_and_expiry: SignatureWithExpiry,
            approver_salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [238, 169, 6, 75],
                    (operator, approver_signature_and_expiry, approver_salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateToBySignature` (0x7f548071) function
        pub fn delegate_to_by_signature(
            &self,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            staker_signature_and_expiry: SignatureWithExpiry,
            approver_signature_and_expiry: SignatureWithExpiry,
            approver_salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [127, 84, 128, 113],
                    (
                        staker,
                        operator,
                        staker_signature_and_expiry,
                        approver_signature_and_expiry,
                        approver_salt,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegatedTo` (0x65da1264) function
        pub fn delegated_to(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([101, 218, 18, 100], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationApprover` (0x3cdeb5e0) function
        pub fn delegation_approver(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([60, 222, 181, 224], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationApproverSaltIsSpent` (0xbb45fef2) function
        pub fn delegation_approver_salt_is_spent(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([187, 69, 254, 242], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earningsReceiver` (0x5f966f14) function
        pub fn earnings_receiver(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([95, 150, 111, 20], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eigenPodManager` (0x4665bcda) function
        pub fn eigen_pod_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([70, 101, 188, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDelegatableShares` (0xcf80873e) function
        pub fn get_delegatable_shares(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([207, 128, 135, 62], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorShares` (0x90041347) function
        pub fn get_operator_shares(
            &self,
            operator: ::ethers::core::types::Address,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([144, 4, 19, 71], (operator, strategies))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWithdrawalDelay` (0x0449ca39) function
        pub fn get_withdrawal_delay(
            &self,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 73, 202, 57], strategies)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseDelegatedShares` (0x28a573ae) function
        pub fn increase_delegated_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 165, 115, 174], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x22bf40e4) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
            min_withdrawal_delay_blocks: ::ethers::core::types::U256,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
            withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 191, 64, 228],
                    (
                        initial_owner,
                        pauser_registry,
                        initial_paused_status,
                        min_withdrawal_delay_blocks,
                        strategies,
                        withdrawal_delay_blocks,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDelegated` (0x3e28391d) function
        pub fn is_delegated(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 40, 57, 29], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperator` (0x6d70f7ae) function
        pub fn is_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 112, 247, 174], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateQueuedWithdrawals` (0x5cfe8d2c) function
        pub fn migrate_queued_withdrawals(
            &self,
            withdrawals_to_migrate: ::std::vec::Vec<DeprecatedStructQueuedWithdrawal>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 254, 141, 44], withdrawals_to_migrate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minWithdrawalDelayBlocks` (0xc448feb8) function
        pub fn min_withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 72, 254, 184], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyOperatorDetails` (0xf16172b0) function
        pub fn modify_operator_details(
            &self,
            new_operator_details: OperatorDetails,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 97, 114, 176], (new_operator_details,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorDetails` (0xc5e480db) function
        pub fn operator_details(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorDetails> {
            self.0
                .method_hash([197, 228, 128, 219], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorShares` (0x778e55f3) function
        pub fn operator_shares(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 142, 85, 243], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x136439dd) function
        pub fn pause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 100, 57, 221], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseAll` (0x595c6a67) function
        pub fn pause_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 92, 106, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5ac86ab7) function
        pub fn paused_with_index(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 200, 106, 183], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauserRegistry` (0x886f1195) function
        pub fn pauser_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingWithdrawals` (0xb7f06ebe) function
        pub fn pending_withdrawals(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 240, 110, 190], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueWithdrawals` (0x0dd8dd02) function
        pub fn queue_withdrawals(
            &self,
            queued_withdrawal_params: ::std::vec::Vec<QueuedWithdrawalParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([13, 216, 221, 2], queued_withdrawal_params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerAsOperator` (0x0f589e59) function
        pub fn register_as_operator(
            &self,
            registering_operator_details: OperatorDetails,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [15, 88, 158, 89],
                    (registering_operator_details, metadata_uri),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinWithdrawalDelayBlocks` (0x635bbd10) function
        pub fn set_min_withdrawal_delay_blocks(
            &self,
            new_min_withdrawal_delay_blocks: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 91, 189, 16], new_min_withdrawal_delay_blocks)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauserRegistry` (0x10d67a2f) function
        pub fn set_pauser_registry(
            &self,
            new_pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 122, 47], new_pauser_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrategyWithdrawalDelayBlocks` (0x1522bf02) function
        pub fn set_strategy_withdrawal_delay_blocks(
            &self,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
            withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 34, 191, 2], (strategies, withdrawal_delay_blocks))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([177, 52, 66, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerNonce` (0x29c77d4f) function
        pub fn staker_nonce(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([41, 199, 125, 79], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerOptOutWindowBlocks` (0x16928365) function
        pub fn staker_opt_out_window_blocks(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 146, 131, 101], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyManager` (0x39b70e38) function
        pub fn strategy_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([57, 183, 14, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyWithdrawalDelayBlocks` (0xc488375a) function
        pub fn strategy_withdrawal_delay_blocks(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 136, 55, 90], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `undelegate` (0xda8be864) function
        pub fn undelegate(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([218, 139, 232, 100], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorMetadataURI` (0x99be81c8) function
        pub fn update_operator_metadata_uri(
            &self,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 190, 129, 200], metadata_uri)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MinWithdrawalDelayBlocksSet` event
        pub fn min_withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinWithdrawalDelayBlocksSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorDetailsModified` event
        pub fn operator_details_modified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorDetailsModifiedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorMetadataURIUpdated` event
        pub fn operator_metadata_uri_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorMetadataURIUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorRegistered` event
        pub fn operator_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorRegisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorSharesDecreased` event
        pub fn operator_shares_decreased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSharesDecreasedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSharesIncreased` event
        pub fn operator_shares_increased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSharesIncreasedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PauserRegistrySet` event
        pub fn pauser_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauserRegistrySetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakerDelegated` event
        pub fn staker_delegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakerDelegatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakerForceUndelegated` event
        pub fn staker_force_undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakerForceUndelegatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StakerUndelegated` event
        pub fn staker_undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakerUndelegatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StrategyWithdrawalDelayBlocksSet` event
        pub fn strategy_withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyWithdrawalDelayBlocksSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalCompleted` event
        pub fn withdrawal_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalCompletedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalMigrated` event
        pub fn withdrawal_migrated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalMigratedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalQueued` event
        pub fn withdrawal_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalQueuedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DelegationManagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for DelegationManager<M>
    {
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
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        name = "MinWithdrawalDelayBlocksSet",
        abi = "MinWithdrawalDelayBlocksSet(uint256,uint256)"
    )]
    pub struct MinWithdrawalDelayBlocksSetFilter {
        pub previous_value: ::ethers::core::types::U256,
        pub new_value: ::ethers::core::types::U256,
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
        name = "OperatorDetailsModified",
        abi = "OperatorDetailsModified(address,(address,address,uint32))"
    )]
    pub struct OperatorDetailsModifiedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub new_operator_details: OperatorDetails,
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
        name = "OperatorMetadataURIUpdated",
        abi = "OperatorMetadataURIUpdated(address,string)"
    )]
    pub struct OperatorMetadataURIUpdatedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub metadata_uri: ::std::string::String,
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
        name = "OperatorRegistered",
        abi = "OperatorRegistered(address,(address,address,uint32))"
    )]
    pub struct OperatorRegisteredFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub operator_details: OperatorDetails,
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
        name = "OperatorSharesDecreased",
        abi = "OperatorSharesDecreased(address,address,address,uint256)"
    )]
    pub struct OperatorSharesDecreasedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
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
        name = "OperatorSharesIncreased",
        abi = "OperatorSharesIncreased(address,address,address,uint256)"
    )]
    pub struct OperatorSharesIncreasedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "Paused", abi = "Paused(address,uint256)")]
    pub struct PausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
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
    #[ethevent(name = "PauserRegistrySet", abi = "PauserRegistrySet(address,address)")]
    pub struct PauserRegistrySetFilter {
        pub pauser_registry: ::ethers::core::types::Address,
        pub new_pauser_registry: ::ethers::core::types::Address,
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
    #[ethevent(name = "StakerDelegated", abi = "StakerDelegated(address,address)")]
    pub struct StakerDelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StakerForceUndelegated",
        abi = "StakerForceUndelegated(address,address)"
    )]
    pub struct StakerForceUndelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "StakerUndelegated", abi = "StakerUndelegated(address,address)")]
    pub struct StakerUndelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StrategyWithdrawalDelayBlocksSet",
        abi = "StrategyWithdrawalDelayBlocksSet(address,uint256,uint256)"
    )]
    pub struct StrategyWithdrawalDelayBlocksSetFilter {
        pub strategy: ::ethers::core::types::Address,
        pub previous_value: ::ethers::core::types::U256,
        pub new_value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address,uint256)")]
    pub struct UnpausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
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
    #[ethevent(name = "WithdrawalCompleted", abi = "WithdrawalCompleted(bytes32)")]
    pub struct WithdrawalCompletedFilter {
        pub withdrawal_root: [u8; 32],
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
        name = "WithdrawalMigrated",
        abi = "WithdrawalMigrated(bytes32,bytes32)"
    )]
    pub struct WithdrawalMigratedFilter {
        pub old_withdrawal_root: [u8; 32],
        pub new_withdrawal_root: [u8; 32],
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
        name = "WithdrawalQueued",
        abi = "WithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))"
    )]
    pub struct WithdrawalQueuedFilter {
        pub withdrawal_root: [u8; 32],
        pub withdrawal: Withdrawal,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DelegationManagerEvents {
        InitializedFilter(InitializedFilter),
        MinWithdrawalDelayBlocksSetFilter(MinWithdrawalDelayBlocksSetFilter),
        OperatorDetailsModifiedFilter(OperatorDetailsModifiedFilter),
        OperatorMetadataURIUpdatedFilter(OperatorMetadataURIUpdatedFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSharesDecreasedFilter(OperatorSharesDecreasedFilter),
        OperatorSharesIncreasedFilter(OperatorSharesIncreasedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        StakerDelegatedFilter(StakerDelegatedFilter),
        StakerForceUndelegatedFilter(StakerForceUndelegatedFilter),
        StakerUndelegatedFilter(StakerUndelegatedFilter),
        StrategyWithdrawalDelayBlocksSetFilter(StrategyWithdrawalDelayBlocksSetFilter),
        UnpausedFilter(UnpausedFilter),
        WithdrawalCompletedFilter(WithdrawalCompletedFilter),
        WithdrawalMigratedFilter(WithdrawalMigratedFilter),
        WithdrawalQueuedFilter(WithdrawalQueuedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DelegationManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MinWithdrawalDelayBlocksSetFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::MinWithdrawalDelayBlocksSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorDetailsModifiedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OperatorDetailsModifiedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorMetadataURIUpdatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OperatorMetadataURIUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OperatorRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OperatorSharesDecreasedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OperatorSharesDecreasedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorSharesIncreasedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OperatorSharesIncreasedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = StakerDelegatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::StakerDelegatedFilter(decoded));
            }
            if let Ok(decoded) = StakerForceUndelegatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::StakerForceUndelegatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakerUndelegatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::StakerUndelegatedFilter(decoded));
            }
            if let Ok(decoded) = StrategyWithdrawalDelayBlocksSetFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::StrategyWithdrawalDelayBlocksSetFilter(decoded),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalCompletedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalCompletedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalMigratedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalMigratedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalQueuedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalQueuedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DelegationManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinWithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDetailsModifiedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorMetadataURIUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSharesDecreasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSharesIncreasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerDelegatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerForceUndelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerUndelegatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyWithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalMigratedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalQueuedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for DelegationManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MinWithdrawalDelayBlocksSetFilter> for DelegationManagerEvents {
        fn from(value: MinWithdrawalDelayBlocksSetFilter) -> Self {
            Self::MinWithdrawalDelayBlocksSetFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDetailsModifiedFilter> for DelegationManagerEvents {
        fn from(value: OperatorDetailsModifiedFilter) -> Self {
            Self::OperatorDetailsModifiedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorMetadataURIUpdatedFilter> for DelegationManagerEvents {
        fn from(value: OperatorMetadataURIUpdatedFilter) -> Self {
            Self::OperatorMetadataURIUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter> for DelegationManagerEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSharesDecreasedFilter> for DelegationManagerEvents {
        fn from(value: OperatorSharesDecreasedFilter) -> Self {
            Self::OperatorSharesDecreasedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSharesIncreasedFilter> for DelegationManagerEvents {
        fn from(value: OperatorSharesIncreasedFilter) -> Self {
            Self::OperatorSharesIncreasedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DelegationManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for DelegationManagerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for DelegationManagerEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<StakerDelegatedFilter> for DelegationManagerEvents {
        fn from(value: StakerDelegatedFilter) -> Self {
            Self::StakerDelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StakerForceUndelegatedFilter> for DelegationManagerEvents {
        fn from(value: StakerForceUndelegatedFilter) -> Self {
            Self::StakerForceUndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StakerUndelegatedFilter> for DelegationManagerEvents {
        fn from(value: StakerUndelegatedFilter) -> Self {
            Self::StakerUndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyWithdrawalDelayBlocksSetFilter> for DelegationManagerEvents {
        fn from(value: StrategyWithdrawalDelayBlocksSetFilter) -> Self {
            Self::StrategyWithdrawalDelayBlocksSetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for DelegationManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalCompletedFilter> for DelegationManagerEvents {
        fn from(value: WithdrawalCompletedFilter) -> Self {
            Self::WithdrawalCompletedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalMigratedFilter> for DelegationManagerEvents {
        fn from(value: WithdrawalMigratedFilter) -> Self {
            Self::WithdrawalMigratedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalQueuedFilter> for DelegationManagerEvents {
        fn from(value: WithdrawalQueuedFilter) -> Self {
            Self::WithdrawalQueuedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DELEGATION_APPROVAL_TYPEHASH` function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`
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
        name = "DELEGATION_APPROVAL_TYPEHASH",
        abi = "DELEGATION_APPROVAL_TYPEHASH()"
    )]
    pub struct DelegationApprovalTypehashCall;
    ///Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    ///Container type for all input parameters for the `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS` function with signature `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()` and selector `0x4fc40b61`
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
        name = "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS",
        abi = "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()"
    )]
    pub struct MaxStakerOptOutWindowBlocksCall;
    ///Container type for all input parameters for the `MAX_WITHDRAWAL_DELAY_BLOCKS` function with signature `MAX_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xca661c04`
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
        name = "MAX_WITHDRAWAL_DELAY_BLOCKS",
        abi = "MAX_WITHDRAWAL_DELAY_BLOCKS()"
    )]
    pub struct MaxWithdrawalDelayBlocksCall;
    ///Container type for all input parameters for the `STAKER_DELEGATION_TYPEHASH` function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`
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
        name = "STAKER_DELEGATION_TYPEHASH",
        abi = "STAKER_DELEGATION_TYPEHASH()"
    )]
    pub struct StakerDelegationTypehashCall;
    ///Container type for all input parameters for the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
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
    #[ethcall(name = "beaconChainETHStrategy", abi = "beaconChainETHStrategy()")]
    pub struct BeaconChainETHStrategyCall;
    ///Container type for all input parameters for the `calculateCurrentStakerDelegationDigestHash` function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`
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
        name = "calculateCurrentStakerDelegationDigestHash",
        abi = "calculateCurrentStakerDelegationDigestHash(address,address,uint256)"
    )]
    pub struct CalculateCurrentStakerDelegationDigestHashCall {
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDelegationApprovalDigestHash` function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`
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
        name = "calculateDelegationApprovalDigestHash",
        abi = "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)"
    )]
    pub struct CalculateDelegationApprovalDigestHashCall {
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub delegation_approver: ::ethers::core::types::Address,
        pub approver_salt: [u8; 32],
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateStakerDelegationDigestHash` function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`
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
        name = "calculateStakerDelegationDigestHash",
        abi = "calculateStakerDelegationDigestHash(address,uint256,address,uint256)"
    )]
    pub struct CalculateStakerDelegationDigestHashCall {
        pub staker: ::ethers::core::types::Address,
        pub staker_nonce: ::ethers::core::types::U256,
        pub operator: ::ethers::core::types::Address,
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`
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
        name = "calculateWithdrawalRoot",
        abi = "calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))"
    )]
    pub struct CalculateWithdrawalRootCall {
        pub withdrawal: Withdrawal,
    }
    ///Container type for all input parameters for the `completeQueuedWithdrawal` function with signature `completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)` and selector `0x60d7faed`
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
        name = "completeQueuedWithdrawal",
        abi = "completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)"
    )]
    pub struct CompleteQueuedWithdrawalCall {
        pub withdrawal: Withdrawal,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub middleware_times_index: ::ethers::core::types::U256,
        pub receive_as_tokens: bool,
    }
    ///Container type for all input parameters for the `completeQueuedWithdrawals` function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])` and selector `0x33404396`
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
        name = "completeQueuedWithdrawals",
        abi = "completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])"
    )]
    pub struct CompleteQueuedWithdrawalsCall {
        pub withdrawals: ::std::vec::Vec<Withdrawal>,
        pub tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        pub middleware_times_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
        pub receive_as_tokens: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
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
        name = "cumulativeWithdrawalsQueued",
        abi = "cumulativeWithdrawalsQueued(address)"
    )]
    pub struct CumulativeWithdrawalsQueuedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `decreaseDelegatedShares` function with signature `decreaseDelegatedShares(address,address,uint256)` and selector `0x132d4967`
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
        name = "decreaseDelegatedShares",
        abi = "decreaseDelegatedShares(address,address,uint256)"
    )]
    pub struct DecreaseDelegatedSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `delegateTo` function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`
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
        name = "delegateTo",
        abi = "delegateTo(address,(bytes,uint256),bytes32)"
    )]
    pub struct DelegateToCall {
        pub operator: ::ethers::core::types::Address,
        pub approver_signature_and_expiry: SignatureWithExpiry,
        pub approver_salt: [u8; 32],
    }
    ///Container type for all input parameters for the `delegateToBySignature` function with signature `delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)` and selector `0x7f548071`
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
        name = "delegateToBySignature",
        abi = "delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)"
    )]
    pub struct DelegateToBySignatureCall {
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub staker_signature_and_expiry: SignatureWithExpiry,
        pub approver_signature_and_expiry: SignatureWithExpiry,
        pub approver_salt: [u8; 32],
    }
    ///Container type for all input parameters for the `delegatedTo` function with signature `delegatedTo(address)` and selector `0x65da1264`
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
    #[ethcall(name = "delegatedTo", abi = "delegatedTo(address)")]
    pub struct DelegatedToCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `delegationApprover` function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`
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
    #[ethcall(name = "delegationApprover", abi = "delegationApprover(address)")]
    pub struct DelegationApproverCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `delegationApproverSaltIsSpent` function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`
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
        name = "delegationApproverSaltIsSpent",
        abi = "delegationApproverSaltIsSpent(address,bytes32)"
    )]
    pub struct DelegationApproverSaltIsSpentCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `earningsReceiver` function with signature `earningsReceiver(address)` and selector `0x5f966f14`
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
    #[ethcall(name = "earningsReceiver", abi = "earningsReceiver(address)")]
    pub struct EarningsReceiverCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    #[ethcall(name = "eigenPodManager", abi = "eigenPodManager()")]
    pub struct EigenPodManagerCall;
    ///Container type for all input parameters for the `getDelegatableShares` function with signature `getDelegatableShares(address)` and selector `0xcf80873e`
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
    #[ethcall(name = "getDelegatableShares", abi = "getDelegatableShares(address)")]
    pub struct GetDelegatableSharesCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOperatorShares` function with signature `getOperatorShares(address,address[])` and selector `0x90041347`
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
        name = "getOperatorShares",
        abi = "getOperatorShares(address,address[])"
    )]
    pub struct GetOperatorSharesCall {
        pub operator: ::ethers::core::types::Address,
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getWithdrawalDelay` function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`
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
    #[ethcall(name = "getWithdrawalDelay", abi = "getWithdrawalDelay(address[])")]
    pub struct GetWithdrawalDelayCall {
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `increaseDelegatedShares` function with signature `increaseDelegatedShares(address,address,uint256)` and selector `0x28a573ae`
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
        name = "increaseDelegatedShares",
        abi = "increaseDelegatedShares(address,address,uint256)"
    )]
    pub struct IncreaseDelegatedSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,uint256,address[],uint256[])` and selector `0x22bf40e4`
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
        name = "initialize",
        abi = "initialize(address,address,uint256,uint256,address[],uint256[])"
    )]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
        pub min_withdrawal_delay_blocks: ::ethers::core::types::U256,
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `isDelegated` function with signature `isDelegated(address)` and selector `0x3e28391d`
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
    #[ethcall(name = "isDelegated", abi = "isDelegated(address)")]
    pub struct IsDelegatedCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOperator` function with signature `isOperator(address)` and selector `0x6d70f7ae`
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
    #[ethcall(name = "isOperator", abi = "isOperator(address)")]
    pub struct IsOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `migrateQueuedWithdrawals` function with signature `migrateQueuedWithdrawals((address[],uint256[],address,(address,uint96),uint32,address)[])` and selector `0x5cfe8d2c`
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
        name = "migrateQueuedWithdrawals",
        abi = "migrateQueuedWithdrawals((address[],uint256[],address,(address,uint96),uint32,address)[])"
    )]
    pub struct MigrateQueuedWithdrawalsCall {
        pub withdrawals_to_migrate: ::std::vec::Vec<DeprecatedStructQueuedWithdrawal>,
    }
    ///Container type for all input parameters for the `minWithdrawalDelayBlocks` function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`
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
    #[ethcall(name = "minWithdrawalDelayBlocks", abi = "minWithdrawalDelayBlocks()")]
    pub struct MinWithdrawalDelayBlocksCall;
    ///Container type for all input parameters for the `modifyOperatorDetails` function with signature `modifyOperatorDetails((address,address,uint32))` and selector `0xf16172b0`
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
        name = "modifyOperatorDetails",
        abi = "modifyOperatorDetails((address,address,uint32))"
    )]
    pub struct ModifyOperatorDetailsCall {
        pub new_operator_details: OperatorDetails,
    }
    ///Container type for all input parameters for the `operatorDetails` function with signature `operatorDetails(address)` and selector `0xc5e480db`
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
    #[ethcall(name = "operatorDetails", abi = "operatorDetails(address)")]
    pub struct OperatorDetailsCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorShares` function with signature `operatorShares(address,address)` and selector `0x778e55f3`
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
    #[ethcall(name = "operatorShares", abi = "operatorShares(address,address)")]
    pub struct OperatorSharesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause(uint256)` and selector `0x136439dd`
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
    #[ethcall(name = "pause", abi = "pause(uint256)")]
    pub struct PauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pauseAll` function with signature `pauseAll()` and selector `0x595c6a67`
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
    #[ethcall(name = "pauseAll", abi = "pauseAll()")]
    pub struct PauseAllCall;
    ///Container type for all input parameters for the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
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
    #[ethcall(name = "paused", abi = "paused(uint8)")]
    pub struct PausedWithIndexCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
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
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `pendingWithdrawals` function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`
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
    #[ethcall(name = "pendingWithdrawals", abi = "pendingWithdrawals(bytes32)")]
    pub struct PendingWithdrawalsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `queueWithdrawals` function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`
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
        name = "queueWithdrawals",
        abi = "queueWithdrawals((address[],uint256[],address)[])"
    )]
    pub struct QueueWithdrawalsCall {
        pub queued_withdrawal_params: ::std::vec::Vec<QueuedWithdrawalParams>,
    }
    ///Container type for all input parameters for the `registerAsOperator` function with signature `registerAsOperator((address,address,uint32),string)` and selector `0x0f589e59`
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
        name = "registerAsOperator",
        abi = "registerAsOperator((address,address,uint32),string)"
    )]
    pub struct RegisterAsOperatorCall {
        pub registering_operator_details: OperatorDetails,
        pub metadata_uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setMinWithdrawalDelayBlocks` function with signature `setMinWithdrawalDelayBlocks(uint256)` and selector `0x635bbd10`
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
        name = "setMinWithdrawalDelayBlocks",
        abi = "setMinWithdrawalDelayBlocks(uint256)"
    )]
    pub struct SetMinWithdrawalDelayBlocksCall {
        pub new_min_withdrawal_delay_blocks: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setPauserRegistry` function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`
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
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStrategyWithdrawalDelayBlocks` function with signature `setStrategyWithdrawalDelayBlocks(address[],uint256[])` and selector `0x1522bf02`
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
        name = "setStrategyWithdrawalDelayBlocks",
        abi = "setStrategyWithdrawalDelayBlocks(address[],uint256[])"
    )]
    pub struct SetStrategyWithdrawalDelayBlocksCall {
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
    ///Container type for all input parameters for the `stakerNonce` function with signature `stakerNonce(address)` and selector `0x29c77d4f`
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
    #[ethcall(name = "stakerNonce", abi = "stakerNonce(address)")]
    pub struct StakerNonceCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `stakerOptOutWindowBlocks` function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`
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
        name = "stakerOptOutWindowBlocks",
        abi = "stakerOptOutWindowBlocks(address)"
    )]
    pub struct StakerOptOutWindowBlocksCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
    #[ethcall(name = "strategyManager", abi = "strategyManager()")]
    pub struct StrategyManagerCall;
    ///Container type for all input parameters for the `strategyWithdrawalDelayBlocks` function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`
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
        name = "strategyWithdrawalDelayBlocks",
        abi = "strategyWithdrawalDelayBlocks(address)"
    )]
    pub struct StrategyWithdrawalDelayBlocksCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `undelegate` function with signature `undelegate(address)` and selector `0xda8be864`
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
    #[ethcall(name = "undelegate", abi = "undelegate(address)")]
    pub struct UndelegateCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause(uint256)` and selector `0xfabc1cbc`
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
    #[ethcall(name = "unpause", abi = "unpause(uint256)")]
    pub struct UnpauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateOperatorMetadataURI` function with signature `updateOperatorMetadataURI(string)` and selector `0x99be81c8`
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
        name = "updateOperatorMetadataURI",
        abi = "updateOperatorMetadataURI(string)"
    )]
    pub struct UpdateOperatorMetadataURICall {
        pub metadata_uri: ::std::string::String,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DelegationManagerCalls {
        DelegationApprovalTypehash(DelegationApprovalTypehashCall),
        DomainTypehash(DomainTypehashCall),
        MaxStakerOptOutWindowBlocks(MaxStakerOptOutWindowBlocksCall),
        MaxWithdrawalDelayBlocks(MaxWithdrawalDelayBlocksCall),
        StakerDelegationTypehash(StakerDelegationTypehashCall),
        BeaconChainETHStrategy(BeaconChainETHStrategyCall),
        CalculateCurrentStakerDelegationDigestHash(CalculateCurrentStakerDelegationDigestHashCall),
        CalculateDelegationApprovalDigestHash(CalculateDelegationApprovalDigestHashCall),
        CalculateStakerDelegationDigestHash(CalculateStakerDelegationDigestHashCall),
        CalculateWithdrawalRoot(CalculateWithdrawalRootCall),
        CompleteQueuedWithdrawal(CompleteQueuedWithdrawalCall),
        CompleteQueuedWithdrawals(CompleteQueuedWithdrawalsCall),
        CumulativeWithdrawalsQueued(CumulativeWithdrawalsQueuedCall),
        DecreaseDelegatedShares(DecreaseDelegatedSharesCall),
        DelegateTo(DelegateToCall),
        DelegateToBySignature(DelegateToBySignatureCall),
        DelegatedTo(DelegatedToCall),
        DelegationApprover(DelegationApproverCall),
        DelegationApproverSaltIsSpent(DelegationApproverSaltIsSpentCall),
        DomainSeparator(DomainSeparatorCall),
        EarningsReceiver(EarningsReceiverCall),
        EigenPodManager(EigenPodManagerCall),
        GetDelegatableShares(GetDelegatableSharesCall),
        GetOperatorShares(GetOperatorSharesCall),
        GetWithdrawalDelay(GetWithdrawalDelayCall),
        IncreaseDelegatedShares(IncreaseDelegatedSharesCall),
        Initialize(InitializeCall),
        IsDelegated(IsDelegatedCall),
        IsOperator(IsOperatorCall),
        MigrateQueuedWithdrawals(MigrateQueuedWithdrawalsCall),
        MinWithdrawalDelayBlocks(MinWithdrawalDelayBlocksCall),
        ModifyOperatorDetails(ModifyOperatorDetailsCall),
        OperatorDetails(OperatorDetailsCall),
        OperatorShares(OperatorSharesCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        PendingWithdrawals(PendingWithdrawalsCall),
        QueueWithdrawals(QueueWithdrawalsCall),
        RegisterAsOperator(RegisterAsOperatorCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetMinWithdrawalDelayBlocks(SetMinWithdrawalDelayBlocksCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetStrategyWithdrawalDelayBlocks(SetStrategyWithdrawalDelayBlocksCall),
        Slasher(SlasherCall),
        StakerNonce(StakerNonceCall),
        StakerOptOutWindowBlocks(StakerOptOutWindowBlocksCall),
        StrategyManager(StrategyManagerCall),
        StrategyWithdrawalDelayBlocks(StrategyWithdrawalDelayBlocksCall),
        TransferOwnership(TransferOwnershipCall),
        Undelegate(UndelegateCall),
        Unpause(UnpauseCall),
        UpdateOperatorMetadataURI(UpdateOperatorMetadataURICall),
    }
    impl ::ethers::core::abi::AbiDecode for DelegationManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DelegationApprovalTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegationApprovalTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <MaxStakerOptOutWindowBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxStakerOptOutWindowBlocks(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) =
                <StakerDelegationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerDelegationTypehash(decoded));
            }
            if let Ok(decoded) =
                <BeaconChainETHStrategyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BeaconChainETHStrategy(decoded));
            }
            if let Ok(decoded) = <CalculateCurrentStakerDelegationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateCurrentStakerDelegationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateDelegationApprovalDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDelegationApprovalDigestHash(decoded));
            }
            if let Ok(decoded) =
                <CalculateStakerDelegationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CalculateStakerDelegationDigestHash(decoded));
            }
            if let Ok(decoded) =
                <CalculateWithdrawalRootCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CalculateWithdrawalRoot(decoded));
            }
            if let Ok(decoded) =
                <CompleteQueuedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompleteQueuedWithdrawal(decoded));
            }
            if let Ok(decoded) =
                <CompleteQueuedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompleteQueuedWithdrawals(decoded));
            }
            if let Ok(decoded) =
                <CumulativeWithdrawalsQueuedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeWithdrawalsQueued(decoded));
            }
            if let Ok(decoded) =
                <DecreaseDelegatedSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DecreaseDelegatedShares(decoded));
            }
            if let Ok(decoded) = <DelegateToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelegateTo(decoded));
            }
            if let Ok(decoded) =
                <DelegateToBySignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegateToBySignature(decoded));
            }
            if let Ok(decoded) = <DelegatedToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelegatedTo(decoded));
            }
            if let Ok(decoded) =
                <DelegationApproverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegationApprover(decoded));
            }
            if let Ok(decoded) =
                <DelegationApproverSaltIsSpentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegationApproverSaltIsSpent(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <EarningsReceiverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EarningsReceiver(decoded));
            }
            if let Ok(decoded) =
                <EigenPodManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EigenPodManager(decoded));
            }
            if let Ok(decoded) =
                <GetDelegatableSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDelegatableShares(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorShares(decoded));
            }
            if let Ok(decoded) =
                <GetWithdrawalDelayCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetWithdrawalDelay(decoded));
            }
            if let Ok(decoded) =
                <IncreaseDelegatedSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncreaseDelegatedShares(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsDelegatedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDelegated(decoded));
            }
            if let Ok(decoded) = <IsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOperator(decoded));
            }
            if let Ok(decoded) =
                <MigrateQueuedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MigrateQueuedWithdrawals(decoded));
            }
            if let Ok(decoded) =
                <MinWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) =
                <ModifyOperatorDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ModifyOperatorDetails(decoded));
            }
            if let Ok(decoded) =
                <OperatorDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorDetails(decoded));
            }
            if let Ok(decoded) =
                <OperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorShares(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) =
                <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <PendingWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingWithdrawals(decoded));
            }
            if let Ok(decoded) =
                <QueueWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QueueWithdrawals(decoded));
            }
            if let Ok(decoded) =
                <RegisterAsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterAsOperator(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetMinWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetMinWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <SetStrategyWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SetStrategyWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakerNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StakerNonce(decoded));
            }
            if let Ok(decoded) =
                <StakerOptOutWindowBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakerOptOutWindowBlocks(decoded));
            }
            if let Ok(decoded) =
                <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyManager(decoded));
            }
            if let Ok(decoded) =
                <StrategyWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UndelegateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Undelegate(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateOperatorMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateOperatorMetadataURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DelegationManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DelegationApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainTypehash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxStakerOptOutWindowBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerDelegationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeaconChainETHStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateStakerDelegationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteQueuedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteQueuedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CumulativeWithdrawalsQueued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseDelegatedShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelegateToBySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegatedTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelegationApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationApproverSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EarningsReceiver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EigenPodManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDelegatableShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetWithdrawalDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseDelegatedShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDelegated(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MigrateQueuedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorDetails(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueueWithdrawals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterAsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMinWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetStrategyWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerOptOutWindowBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategyWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Undelegate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateOperatorMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DelegationManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelegationApprovalTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxStakerOptOutWindowBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithdrawalDelayBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerDelegationTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeaconChainETHStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateWithdrawalRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompleteQueuedWithdrawal(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompleteQueuedWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeWithdrawalsQueued(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseDelegatedShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateToBySignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegatedTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationApproverSaltIsSpent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarningsReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::EigenPodManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDelegatableShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWithdrawalDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseDelegatedShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDelegated(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateQueuedWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinWithdrawalDelayBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyOperatorDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueueWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterAsOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinWithdrawalDelayBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrategyWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerOptOutWindowBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperatorMetadataURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DelegationApprovalTypehashCall> for DelegationManagerCalls {
        fn from(value: DelegationApprovalTypehashCall) -> Self {
            Self::DelegationApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<DomainTypehashCall> for DelegationManagerCalls {
        fn from(value: DomainTypehashCall) -> Self {
            Self::DomainTypehash(value)
        }
    }
    impl ::core::convert::From<MaxStakerOptOutWindowBlocksCall> for DelegationManagerCalls {
        fn from(value: MaxStakerOptOutWindowBlocksCall) -> Self {
            Self::MaxStakerOptOutWindowBlocks(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: MaxWithdrawalDelayBlocksCall) -> Self {
            Self::MaxWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<StakerDelegationTypehashCall> for DelegationManagerCalls {
        fn from(value: StakerDelegationTypehashCall) -> Self {
            Self::StakerDelegationTypehash(value)
        }
    }
    impl ::core::convert::From<BeaconChainETHStrategyCall> for DelegationManagerCalls {
        fn from(value: BeaconChainETHStrategyCall) -> Self {
            Self::BeaconChainETHStrategy(value)
        }
    }
    impl ::core::convert::From<CalculateCurrentStakerDelegationDigestHashCall>
        for DelegationManagerCalls
    {
        fn from(value: CalculateCurrentStakerDelegationDigestHashCall) -> Self {
            Self::CalculateCurrentStakerDelegationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateDelegationApprovalDigestHashCall> for DelegationManagerCalls {
        fn from(value: CalculateDelegationApprovalDigestHashCall) -> Self {
            Self::CalculateDelegationApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateStakerDelegationDigestHashCall> for DelegationManagerCalls {
        fn from(value: CalculateStakerDelegationDigestHashCall) -> Self {
            Self::CalculateStakerDelegationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateWithdrawalRootCall> for DelegationManagerCalls {
        fn from(value: CalculateWithdrawalRootCall) -> Self {
            Self::CalculateWithdrawalRoot(value)
        }
    }
    impl ::core::convert::From<CompleteQueuedWithdrawalCall> for DelegationManagerCalls {
        fn from(value: CompleteQueuedWithdrawalCall) -> Self {
            Self::CompleteQueuedWithdrawal(value)
        }
    }
    impl ::core::convert::From<CompleteQueuedWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: CompleteQueuedWithdrawalsCall) -> Self {
            Self::CompleteQueuedWithdrawals(value)
        }
    }
    impl ::core::convert::From<CumulativeWithdrawalsQueuedCall> for DelegationManagerCalls {
        fn from(value: CumulativeWithdrawalsQueuedCall) -> Self {
            Self::CumulativeWithdrawalsQueued(value)
        }
    }
    impl ::core::convert::From<DecreaseDelegatedSharesCall> for DelegationManagerCalls {
        fn from(value: DecreaseDelegatedSharesCall) -> Self {
            Self::DecreaseDelegatedShares(value)
        }
    }
    impl ::core::convert::From<DelegateToCall> for DelegationManagerCalls {
        fn from(value: DelegateToCall) -> Self {
            Self::DelegateTo(value)
        }
    }
    impl ::core::convert::From<DelegateToBySignatureCall> for DelegationManagerCalls {
        fn from(value: DelegateToBySignatureCall) -> Self {
            Self::DelegateToBySignature(value)
        }
    }
    impl ::core::convert::From<DelegatedToCall> for DelegationManagerCalls {
        fn from(value: DelegatedToCall) -> Self {
            Self::DelegatedTo(value)
        }
    }
    impl ::core::convert::From<DelegationApproverCall> for DelegationManagerCalls {
        fn from(value: DelegationApproverCall) -> Self {
            Self::DelegationApprover(value)
        }
    }
    impl ::core::convert::From<DelegationApproverSaltIsSpentCall> for DelegationManagerCalls {
        fn from(value: DelegationApproverSaltIsSpentCall) -> Self {
            Self::DelegationApproverSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for DelegationManagerCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EarningsReceiverCall> for DelegationManagerCalls {
        fn from(value: EarningsReceiverCall) -> Self {
            Self::EarningsReceiver(value)
        }
    }
    impl ::core::convert::From<EigenPodManagerCall> for DelegationManagerCalls {
        fn from(value: EigenPodManagerCall) -> Self {
            Self::EigenPodManager(value)
        }
    }
    impl ::core::convert::From<GetDelegatableSharesCall> for DelegationManagerCalls {
        fn from(value: GetDelegatableSharesCall) -> Self {
            Self::GetDelegatableShares(value)
        }
    }
    impl ::core::convert::From<GetOperatorSharesCall> for DelegationManagerCalls {
        fn from(value: GetOperatorSharesCall) -> Self {
            Self::GetOperatorShares(value)
        }
    }
    impl ::core::convert::From<GetWithdrawalDelayCall> for DelegationManagerCalls {
        fn from(value: GetWithdrawalDelayCall) -> Self {
            Self::GetWithdrawalDelay(value)
        }
    }
    impl ::core::convert::From<IncreaseDelegatedSharesCall> for DelegationManagerCalls {
        fn from(value: IncreaseDelegatedSharesCall) -> Self {
            Self::IncreaseDelegatedShares(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for DelegationManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsDelegatedCall> for DelegationManagerCalls {
        fn from(value: IsDelegatedCall) -> Self {
            Self::IsDelegated(value)
        }
    }
    impl ::core::convert::From<IsOperatorCall> for DelegationManagerCalls {
        fn from(value: IsOperatorCall) -> Self {
            Self::IsOperator(value)
        }
    }
    impl ::core::convert::From<MigrateQueuedWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: MigrateQueuedWithdrawalsCall) -> Self {
            Self::MigrateQueuedWithdrawals(value)
        }
    }
    impl ::core::convert::From<MinWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: MinWithdrawalDelayBlocksCall) -> Self {
            Self::MinWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<ModifyOperatorDetailsCall> for DelegationManagerCalls {
        fn from(value: ModifyOperatorDetailsCall) -> Self {
            Self::ModifyOperatorDetails(value)
        }
    }
    impl ::core::convert::From<OperatorDetailsCall> for DelegationManagerCalls {
        fn from(value: OperatorDetailsCall) -> Self {
            Self::OperatorDetails(value)
        }
    }
    impl ::core::convert::From<OperatorSharesCall> for DelegationManagerCalls {
        fn from(value: OperatorSharesCall) -> Self {
            Self::OperatorShares(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DelegationManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for DelegationManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for DelegationManagerCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for DelegationManagerCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for DelegationManagerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for DelegationManagerCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<PendingWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: PendingWithdrawalsCall) -> Self {
            Self::PendingWithdrawals(value)
        }
    }
    impl ::core::convert::From<QueueWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: QueueWithdrawalsCall) -> Self {
            Self::QueueWithdrawals(value)
        }
    }
    impl ::core::convert::From<RegisterAsOperatorCall> for DelegationManagerCalls {
        fn from(value: RegisterAsOperatorCall) -> Self {
            Self::RegisterAsOperator(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DelegationManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetMinWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: SetMinWithdrawalDelayBlocksCall) -> Self {
            Self::SetMinWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for DelegationManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetStrategyWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: SetStrategyWithdrawalDelayBlocksCall) -> Self {
            Self::SetStrategyWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for DelegationManagerCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakerNonceCall> for DelegationManagerCalls {
        fn from(value: StakerNonceCall) -> Self {
            Self::StakerNonce(value)
        }
    }
    impl ::core::convert::From<StakerOptOutWindowBlocksCall> for DelegationManagerCalls {
        fn from(value: StakerOptOutWindowBlocksCall) -> Self {
            Self::StakerOptOutWindowBlocks(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for DelegationManagerCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<StrategyWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: StrategyWithdrawalDelayBlocksCall) -> Self {
            Self::StrategyWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DelegationManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UndelegateCall> for DelegationManagerCalls {
        fn from(value: UndelegateCall) -> Self {
            Self::Undelegate(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for DelegationManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorMetadataURICall> for DelegationManagerCalls {
        fn from(value: UpdateOperatorMetadataURICall) -> Self {
            Self::UpdateOperatorMetadataURI(value)
        }
    }
    ///Container type for all return fields from the `DELEGATION_APPROVAL_TYPEHASH` function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`
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
    pub struct DelegationApprovalTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    pub struct DomainTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS` function with signature `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()` and selector `0x4fc40b61`
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
    pub struct MaxStakerOptOutWindowBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_WITHDRAWAL_DELAY_BLOCKS` function with signature `MAX_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xca661c04`
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
    pub struct MaxWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `STAKER_DELEGATION_TYPEHASH` function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`
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
    pub struct StakerDelegationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
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
    pub struct BeaconChainETHStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `calculateCurrentStakerDelegationDigestHash` function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`
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
    pub struct CalculateCurrentStakerDelegationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateDelegationApprovalDigestHash` function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`
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
    pub struct CalculateDelegationApprovalDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateStakerDelegationDigestHash` function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`
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
    pub struct CalculateStakerDelegationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`
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
    pub struct CalculateWithdrawalRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
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
    pub struct CumulativeWithdrawalsQueuedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegatedTo` function with signature `delegatedTo(address)` and selector `0x65da1264`
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
    pub struct DelegatedToReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationApprover` function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`
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
    pub struct DelegationApproverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationApproverSaltIsSpent` function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`
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
    pub struct DelegationApproverSaltIsSpentReturn(pub bool);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `earningsReceiver` function with signature `earningsReceiver(address)` and selector `0x5f966f14`
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
    pub struct EarningsReceiverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    pub struct EigenPodManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDelegatableShares` function with signature `getDelegatableShares(address)` and selector `0xcf80873e`
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
    pub struct GetDelegatableSharesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getOperatorShares` function with signature `getOperatorShares(address,address[])` and selector `0x90041347`
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
    pub struct GetOperatorSharesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getWithdrawalDelay` function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`
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
    pub struct GetWithdrawalDelayReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isDelegated` function with signature `isDelegated(address)` and selector `0x3e28391d`
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
    pub struct IsDelegatedReturn(pub bool);
    ///Container type for all return fields from the `isOperator` function with signature `isOperator(address)` and selector `0x6d70f7ae`
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
    pub struct IsOperatorReturn(pub bool);
    ///Container type for all return fields from the `minWithdrawalDelayBlocks` function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`
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
    pub struct MinWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operatorDetails` function with signature `operatorDetails(address)` and selector `0xc5e480db`
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
    pub struct OperatorDetailsReturn(pub OperatorDetails);
    ///Container type for all return fields from the `operatorShares` function with signature `operatorShares(address,address)` and selector `0x778e55f3`
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
    pub struct OperatorSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
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
    pub struct PausedWithIndexReturn(pub bool);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
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
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingWithdrawals` function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`
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
    pub struct PendingWithdrawalsReturn(pub bool);
    ///Container type for all return fields from the `queueWithdrawals` function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`
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
    pub struct QueueWithdrawalsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakerNonce` function with signature `stakerNonce(address)` and selector `0x29c77d4f`
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
    pub struct StakerNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerOptOutWindowBlocks` function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`
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
    pub struct StakerOptOutWindowBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
    pub struct StrategyManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `strategyWithdrawalDelayBlocks` function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`
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
    pub struct StrategyWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `undelegate` function with signature `undelegate(address)` and selector `0xda8be864`
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
    pub struct UndelegateReturn {
        pub withdrawal_roots: ::std::vec::Vec<[u8; 32]>,
    }
    ///`OperatorDetails(address,address,uint32)`
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
    pub struct OperatorDetails {
        pub earnings_receiver: ::ethers::core::types::Address,
        pub delegation_approver: ::ethers::core::types::Address,
        pub staker_opt_out_window_blocks: u32,
    }
    ///`QueuedWithdrawalParams(address[],uint256[],address)`
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
    pub struct QueuedWithdrawalParams {
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
        pub withdrawer: ::ethers::core::types::Address,
    }
    ///`Withdrawal(address,address,address,uint256,uint32,address[],uint256[])`
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
    pub struct Withdrawal {
        pub staker: ::ethers::core::types::Address,
        pub delegated_to: ::ethers::core::types::Address,
        pub withdrawer: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub start_block: u32,
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///`SignatureWithExpiry(bytes,uint256)`
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
    pub struct SignatureWithExpiry {
        pub signature: ::ethers::core::types::Bytes,
        pub expiry: ::ethers::core::types::U256,
    }
    ///`DeprecatedStructQueuedWithdrawal(address[],uint256[],address,(address,uint96),uint32,address)`
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
    pub struct DeprecatedStructQueuedWithdrawal {
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub shares: ::std::vec::Vec<::ethers::core::types::U256>,
        pub staker: ::ethers::core::types::Address,
        pub withdrawer_and_nonce: DeprecatedStructWithdrawerAndNonce,
        pub withdrawal_start_block: u32,
        pub delegated_address: ::ethers::core::types::Address,
    }
    ///`DeprecatedStructWithdrawerAndNonce(address,uint96)`
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
    pub struct DeprecatedStructWithdrawerAndNonce {
        pub withdrawer: ::ethers::core::types::Address,
        pub nonce: u128,
    }
}
