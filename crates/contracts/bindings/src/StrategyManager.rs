pub use strategy_manager::*;
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
pub mod strategy_manager {
    const _: () = {
        ::core::include_bytes!(
            "/Users/supernovahs/Desktop/eigensdk-rs/crates/contracts/bindings/json/StrategyManager.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_delegation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IDelegationManager",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_eigenPodManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IEigenPodManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_slasher"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEPOSIT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEPOSIT_TYPEHASH"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("addShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
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
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("addStrategiesToDepositWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addStrategiesToDepositWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "strategiesToWhitelist",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "thirdPartyTransfersForbiddenValues",
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateWithdrawalRoot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("queuedWithdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IDelegationManager",
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
                    ::std::borrow::ToOwned::to_owned("depositIntoStrategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositIntoStrategy",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositIntoStrategyWithSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositIntoStrategyWithSignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IEigenPodManager",
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
                    ::std::borrow::ToOwned::to_owned("getDeposits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDeposits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initialStrategyWhitelister",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initialPausedStatus",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "migrateQueuedWithdrawal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("queuedWithdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeShares"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "removeStrategiesFromDepositWhitelist",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeStrategiesFromDepositWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "strategiesToRemoveFromWhitelist",
                                    ),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("setStrategyWhitelister"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStrategyWhitelister",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newStrategyWhitelister",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setThirdPartyTransfersForbidden"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setThirdPartyTransfersForbidden",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slasher"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerStrategyList"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakerStrategyList"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerStrategyListLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "stakerStrategyListLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("stakerStrategyShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "stakerStrategyShares",
                            ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("strategyIsWhitelistedForDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyIsWhitelistedForDeposit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyWhitelister"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyWhitelister",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("thirdPartyTransfersForbidden"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "thirdPartyTransfersForbidden",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawSharesAsTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawSharesAsTokens",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawalRootPending"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawalRootPending",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyAddedToDepositWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyAddedToDepositWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "StrategyRemovedFromDepositWhitelist",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyRemovedFromDepositWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyWhitelisterChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyWhitelisterChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "UpdatedThirdPartyTransfersForbidden",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UpdatedThirdPartyTransfersForbidden",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STRATEGYMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\08)8\x03\x80b\08)\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01@V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x80\x83\x16`\xA0R\x81\x16`\xC0Rb\0\0Xb\0\0eV[PPF`\xE0RPb\0\x01\x94V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01%W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01VW`\0\x80\xFD[\x83Qb\0\x01c\x81b\0\x01'V[` \x85\x01Q\x90\x93Pb\0\x01v\x81b\0\x01'V[`@\x85\x01Q\x90\x92Pb\0\x01\x89\x81b\0\x01'V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa68b\0\x01\xF1`\09`\0a\x16\x1D\x01R`\0a\x04\x8F\x01R`\0a\x02\xA6\x01R`\0\x81\x81a\x05\x9B\x01R\x81\x81a\x0C\x05\x01R\x81\x81a\x0F\x9F\x01R\x81\x81a\x0F\xF3\x01R\x81\x81a\x10\xF7\x01Ra\x1B\xD3\x01Ra68`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02'W`\x005`\xE0\x1C\x80c\x94\xF6I\xDD\x11a\x010W\x80c\xC6eg\x02\x11a\0\xB8W\x80c\xDF\\\xF7#\x11a\0|W\x80c\xDF\\\xF7#\x14a\x05\x96W\x80c\xE7\xA0P\xAA\x14a\x05\xBDW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xD0W\x80c\xF6\x98\xDA%\x14a\x05\xE3W\x80c\xFA\xBC\x1C\xBC\x14a\x05\xEBW`\0\x80\xFD[\x80c\xC6eg\x02\x14a\x05 W\x80c\xCB\xC2\xBDb\x14a\x053W\x80c\xCD)?o\x14a\x05FW\x80c\xCFuo\xDF\x14a\x05pW\x80c\xDF[5G\x14a\x05\x83W`\0\x80\xFD[\x80c\xB4;QK\x11a\0\xFFW\x80c\xB4;QK\x14a\x04\xB1W\x80c\xB5\xD8\xB5\xB8\x14a\x04\xC4W\x80c\xC3\xC6\xB3\xA9\x14a\x04\xD7W\x80c\xC4b>\xA1\x14a\x04\xFAW\x80c\xC6\x08\xC7\xF3\x14a\x05\rW`\0\x80\xFD[\x80c\x94\xF6I\xDD\x14a\x043W\x80c\x96\x7F\xC0\xD2\x14a\x04TW\x80c\x9BM\xA0=\x14a\x04gW\x80c\xB14Bq\x14a\x04\x8AW`\0\x80\xFD[\x80c\\\x97Z\xBB\x11a\x01\xB3W\x80c~\xCE\xBE\0\x11a\x01\x82W\x80c~\xCE\xBE\0\x14a\x03\xB3W\x80c\x88o\x11\x95\x14a\x03\xD3W\x80c\x8B\x8A\xAC<\x14a\x03\xE6W\x80c\x8C\x80\xD4\xE5\x14a\x04\x0FW\x80c\x8D\xA5\xCB[\x14a\x04\"W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03UW\x80cf<\x1D\xE4\x14a\x03]W\x80cqP\x18\xA6\x14a\x03\x80W\x80cz~\r\x92\x14a\x03\x88W`\0\x80\xFD[\x80cFe\xBC\xDA\x11a\x01\xFAW\x80cFe\xBC\xDA\x14a\x02\xA1W\x80cH\x82^\x94\x14a\x02\xE0W\x80cNZBc\x14a\x03\x07W\x80cY\\jg\x14a\x03\x1AW\x80cZ\xC8j\xB7\x14a\x03\"W`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x02,W\x80c\x13d9\xDD\x14a\x02AW\x80c `kp\x14a\x02TW\x80c2\xE8\x9A\xCE\x14a\x02\x8EW[`\0\x80\xFD[a\x02?a\x02:6`\x04a+JV[a\x05\xFEV[\0[a\x02?a\x02O6`\x04a+gV[a\x06\xBAV[a\x02{\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02{a\x02\x9C6`\x04a,\0V[a\x07\xF9V[a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x85V[a\x02{\x7FC7\xF8-\x14.A\xF2\xA8\xC1\x05G\xCD\x8C\x85\x9B\xDD\xB9\"b\xA6\x10X\xE7xB\xE2M\x9D\xEA\x92$\x81V[a\x02?a\x03\x156`\x04a,\xEDV[a\n\xE7V[a\x02?a\x0B\x1FV[a\x03Ea\x0306`\x04a-&V[`\x98T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x85V[`\x98Ta\x02{V[a\x03Ea\x03k6`\x04a+JV[`\xD1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02?a\x0B\xE6V[a\x02{a\x03\x966`\x04a-IV[`\xCD` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02{a\x03\xC16`\x04a+JV[`\xCA` R`\0\x90\x81R`@\x90 T\x81V[`\x97Ta\x02\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02{a\x03\xF46`\x04a+JV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xCE` R`@\x90 T\x90V[a\x02?a\x04\x1D6`\x04a-wV[a\x0B\xFAV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC8V[a\x04Fa\x04A6`\x04a+JV[a\x0CSV[`@Qa\x02\x85\x92\x91\x90a.,V[`\xCBTa\x02\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Ea\x04u6`\x04a+JV[`\xD3` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02{a\x04\xBF6`\x04a/\xD5V[a\r\xD3V[a\x02?a\x04\xD26`\x04a0\xF0V[a\x0E V[a\x03Ea\x04\xE56`\x04a+gV[`\xCF` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02?a\x05\x086`\x04a12V[a\x0F\x94V[a\x02?a\x05\x1B6`\x04a1\x83V[a\x0F\xE8V[a\x02?a\x05.6`\x04a+JV[a\x10\xA0V[a\x02\xC8a\x05A6`\x04a1\xD6V[a\x10\xB1V[a\x05Ya\x05T6`\x04a/\xD5V[a\x10\xE9V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x85V[a\x02?a\x05~6`\x04a12V[a\x11}V[a\x02?a\x05\x916`\x04a2\x02V[a\x12\xB1V[a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02{a\x05\xCB6`\x04a-wV[a\x14\xDAV[a\x02?a\x05\xDE6`\x04a+JV[a\x15\xA3V[a\x02{a\x16\x19V[a\x02?a\x05\xF96`\x04a+gV[a\x16WV[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06u\x91\x90a2nV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\x8BV[`@Q\x80\x91\x03\x90\xFD[a\x06\xB7\x81a\x17\xB3V[PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07&\x91\x90a2\xD5V[a\x07BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\xF2V[`\x98T\x81\x81\x16\x14a\x07\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x98T`\0\x90\x81\x90`\x01\x90\x81\x16\x14\x15a\x08PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eT\x14\x15a\x08\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eU`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\xD3` R`@\x90 T`\xFF\x16\x15a\tJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FStrategyManager.depositIntoStrat`D\x82\x01R\x7FegyWithSignature: third transfer`d\x82\x01Ri\x1C\xC8\x19\x1A\\\xD8X\x9B\x19Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[B\x84\x10\x15a\t\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FStrategyManager.depositIntoStrat`D\x82\x01R\x7FegyWithSignature: signature expi`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x81\x81R`\xCA` \x90\x81R`@\x80\x83 T\x81Q\x7FC7\xF8-\x14.A\xF2\xA8\xC1\x05G\xCD\x8C\x85\x9B\xDD\xB9\"b\xA6\x10X\xE7xB\xE2M\x9D\xEA\x92$\x93\x81\x01\x93\x90\x93R\x90\x82\x01\x93\x90\x93R\x8B\x84\x16``\x82\x01R\x92\x8A\x16`\x80\x84\x01R`\xA0\x83\x01\x89\x90R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x87\x90R\x90\x91a\x01\0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\xCA\x90\x93R\x90\x82 `\x01\x85\x01\x90U\x91Pa\n\x84a\x16\x19V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\n\xC7\x88\x82\x88a\x18\xAAV[a\n\xD3\x88\x8C\x8C\x8Ca\x1AiV[`\x01`eU\x9B\x9APPPPPPPPPPPV[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3:V[a\x0B\x1B\x82\x82a\x1C8V[PPV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x8B\x91\x90a2\xD5V[a\x0B\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\xF2V[`\0\x19`\x98\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0B\xEEa\x1C\xA6V[a\x0B\xF8`\0a\x1D\0V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0CBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[a\x0CM\x83\x83\x83a\x1DRV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCE` R`@\x81 T``\x91\x82\x91\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x8BWa\x0C\x8Ba+\x90V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xB4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\rEW`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 `\xCE\x90\x92R\x82 \x80T\x91\x92\x91\x84\x90\x81\x10a\x0C\xF9Wa\x0C\xF9a4\x02V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x90 T\x82Q\x83\x90\x83\x90\x81\x10a\r2Wa\r2a4\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0C\xBAV[P`\xCE`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x81\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\r\xC1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\r\xA3W[PPPPP\x91P\x93P\x93PPP\x91P\x91V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a\x0E\x03\x97\x90\x96\x95\x91\x01a4\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EJW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3:V[\x80`\0[\x81\x81\x10\x15a\x0CMW`\xD1`\0\x85\x85\x84\x81\x81\x10a\x0ElWa\x0Ela4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0E\x81\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x0F\x8CW`\0`\xD1`\0\x86\x86\x85\x81\x81\x10a\x0E\xBBWa\x0E\xBBa4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xD0\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U\x7F@tA;KD>NX\x01\x9F(U\xA8vQ\x135\x8C|r\xE3\x95\t\xC6\xAFE\xFC\x0F[\xA00\x84\x84\x83\x81\x81\x10a\x0F+Wa\x0F+a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0F@\x91\x90a+JV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1a\x0F\x8C\x84\x84\x83\x81\x81\x10a\x0FpWa\x0Fpa4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0F\x85\x91\x90a+JV[`\0a\x1C8V[`\x01\x01a\x0ENV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0F\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[a\x0CM\x84\x84\x84\x84a\x1E\xAEV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x100W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[`@Qcl\xE5v\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x84\x16\x90c\xD9\xCA\xED\x12\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x96W=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x10\xA8a\x1C\xA6V[a\x06\xB7\x81a!;V[`\xCE` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\xCDW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`\0\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x114W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[`\0a\x11?\x84a\r\xD3V[`\0\x81\x81R`\xCF` R`@\x81 T\x91\x92P\x90`\xFF\x16\x15a\x11tWP`\0\x81\x81R`\xCF` R`@\x90 \x80T`\xFF\x19\x16\x90U`\x01[\x92P\x90P\x91P\x91V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11\x9DWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\xB7WP0;\x15\x80\x15a\x11\xB7WP`\0T`\xFF\x16`\x01\x14[a\x12\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12=W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12Ea!\xA4V[`\xC9Ua\x12R\x83\x83a\";V[a\x12[\x85a\x1D\0V[a\x12d\x84a!;V[\x80\x15a\x12\xAAW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3:V[\x82\x81\x14a\x13dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FStrategyManager.addStrategiesToD`D\x82\x01R\x7FepositWhitelist: array lengths d`d\x82\x01Rj\r\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xAB\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[\x82`\0[\x81\x81\x10\x15a\x14\xD2W`\xD1`\0\x87\x87\x84\x81\x81\x10a\x13\x86Wa\x13\x86a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x13\x9B\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16a\x14\xCAW`\x01`\xD1`\0\x88\x88\x85\x81\x81\x10a\x13\xD4Wa\x13\xD4a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x13\xE9\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U\x7F\x0C5\xB1}\x91\xC9n\xB2u\x1C\xD4V\xE1%/B\xA3\x86\xE5$\xEF\x9F\xF2n\xCC\x99P\x85\x9F\xDC\x04\xFE\x86\x86\x83\x81\x81\x10a\x14DWa\x14Da4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x14Y\x91\x90a+JV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1a\x14\xCA\x86\x86\x83\x81\x81\x10a\x14\x89Wa\x14\x89a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x14\x9E\x91\x90a+JV[\x85\x85\x84\x81\x81\x10a\x14\xB0Wa\x14\xB0a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x14\xC5\x91\x90a4\x91V[a\x1C8V[`\x01\x01a\x13hV[PPPPPPV[`\x98T`\0\x90\x81\x90`\x01\x90\x81\x16\x14\x15a\x151W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eT\x14\x15a\x15\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eUa\x15\x953\x86\x86\x86a\x1AiV[`\x01`eU\x95\x94PPPPPV[a\x15\xABa\x1C\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[a\x06\xB7\x81a\x1D\0V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\x16JWP`\xC9T\x90V[a\x16Ra!\xA4V[\x90P\x90V[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xCE\x91\x90a2nV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\x8BV[`\x98T\x19\x81\x19`\x98T\x19\x16\x14a\x17|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\xEEV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x19\xC9W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a\x18\xEA\x90\x86\x90\x86\x90`\x04\x01a5\x06V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19+\x91\x90a5\x1FV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[PPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x19\xDD\x83\x83a#!V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xD1` R`@\x81 T\x84\x90`\xFF\x16a\x1B\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FStrategyManager.onlyStrategiesWh`D\x82\x01R\x7FitelistedForDeposit: strategy no`d\x82\x01Rl\x1D\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[a\x1B$`\x01`\x01`\xA0\x1B\x03\x85\x163\x87\x86a#EV[`@Qc\x11\xF9\xFB\xC9`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x86\x16\x90cG\xE7\xEF$\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1BsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x97\x91\x90a5IV[\x91Pa\x1B\xA5\x86\x85\x87\x85a\x1E\xAEV[`@Qc\x14R\xB9\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c(\xA5s\xAE\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\x17W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C+W=`\0\x80>=`\0\xFD[PPPPP\x94\x93PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82\x15\x15` \x82\x01R\x7Fw\xD90\xDFI7y4s\xA9P$\xD8z\x98\xFD,\xCB\x9E\x92\xD3\xC2F;=\xAC\xD6]>jW\x86\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\xD3` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xA5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x81a\x1D\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStrategyManager._removeShares: s`D\x82\x01R\x7FhareAmount should not be zero!\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R T\x80\x83\x11\x15a\x1EYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FStrategyManager._removeShares: s`D\x82\x01Rr\r\x0C.L\xA8-\xAD\xEE\xAD\xCE\x84\x0E\x8D\xED\xE4\r\r,\xED`k\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x83\x82\x03\x90\x81\x90U\x90\x83\x14\x15a\x1E\xA1Wa\x1E\x97\x85\x85a#\x9FV[`\x01\x91PPa\x1E\xA7V[`\0\x91PP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1F*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStrategyManager._addShares: stak`D\x82\x01R\x7Fer cannot be zero address\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[\x80a\x1F\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FStrategyManager._addShares: shar`D\x82\x01Rues should not be zero!`P\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R Ta \xA7W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCE` \x90\x81R`@\x90\x91 T\x10a hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FStrategyManager._addShares: depo`D\x82\x01R\x7Fsit would exceed MAX_STAKER_STRA`d\x82\x01Ro\n\x88\xA8\xEB+\xE9\x89*j\x8B\xE9\x88\xA9\xC8\xEA\x89`\x83\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\xCE` \x90\x81R`@\x82 \x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a \xDE\x90\x84\x90a5xV[\x90\x91UPP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R``\x81\x01\x83\x90R\x90Q\x7F|\xFF\xF9\x08\xA4\xB5\x83\xF3d0\xB2]u\x96LE\x8D\x8E\xDE\x8A\x99\xBDa\xBEu\x0E\x97\xEE\x1B/:\x96\x91\x81\x90\x03`\x80\x01\x90\xA1PPPPV[`\xCBT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7FBd'^Y9U\xFF\x9DaF\xA5\x1AE%\xF6\xDD\xAC\xE2\xE8\x1D\xB99\x1A\xBC\xC9\xD1\xCAH\x04})\x91\x01`@Q\x80\x91\x03\x90\xA1`\xCB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`\x97T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\"\\WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\"\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x0B\x1B\x82a\x17\xB3V[`\0\x80`\0a#0\x85\x85a%\x91V[\x91P\x91Pa#=\x81a&\x01V[P\x93\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x0CM\x90\x85\x90a'\xBCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCE` R`@\x81 T\x90[\x81\x81\x10\x15a$\xBAW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\xCE` R`@\x90 \x80T\x91\x85\x16\x91\x83\x90\x81\x10a#\xF1Wa#\xF1a4\x02V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a$\xB2W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCE` R`@\x90 \x80Ta$2\x90`\x01\x90a5\x90V[\x81T\x81\x10a$BWa$Ba4\x02V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x84R`\xCE\x90\x92R`@\x90\x92 \x80T\x91\x90\x92\x16\x91\x90\x83\x90\x81\x10a$\x7FWa$\x7Fa4\x02V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPa$\xBAV[`\x01\x01a#\xBAV[\x81\x81\x14\x15a%BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FStrategyManager._removeStrategyF`D\x82\x01R\x7FromStakerStrategyList: strategy `d\x82\x01Rh\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`\xBA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCE` R`@\x90 \x80T\x80a%iWa%ia5\xA7V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPPV[`\0\x80\x82Q`A\x14\x15a%\xC8W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa%\xBC\x87\x82\x85\x85a(\x8EV[\x94P\x94PPPPa%\xFAV[\x82Q`@\x14\x15a%\xF2W` \x83\x01Q`@\x84\x01Qa%\xE7\x86\x83\x83a){V[\x93P\x93PPPa%\xFAV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a&\x15Wa&\x15a5\xBDV[\x14\x15a&\x1EWPV[`\x01\x81`\x04\x81\x11\x15a&2Wa&2a5\xBDV[\x14\x15a&\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xA5V[`\x02\x81`\x04\x81\x11\x15a&\x94Wa&\x94a5\xBDV[\x14\x15a&\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x06\xA5V[`\x03\x81`\x04\x81\x11\x15a&\xF6Wa&\xF6a5\xBDV[\x14\x15a'OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x04\x81`\x04\x81\x11\x15a'cWa'ca5\xBDV[\x14\x15a\x06\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\0a(\x11\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a)\xB4\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x19\xC4W\x80\x80` \x01\x90Q\x81\x01\x90a(/\x91\x90a2\xD5V[a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a(\xC5WP`\0\x90P`\x03a)rV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a(\xDDWP\x84`\xFF\x16`\x1C\x14\x15[\x15a(\xEEWP`\0\x90P`\x04a)rV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a)BW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a)kW`\0`\x01\x92P\x92PPa)rV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a)\x98`\xFF\x86\x90\x1C`\x1Ba5xV[\x90Pa)\xA6\x87\x82\x88\x85a(\x8EV[\x93P\x93PPP\x93P\x93\x91PPV[``a)\xC3\x84\x84`\0\x85a)\xCBV[\x94\x93PPPPV[``\x82G\x10\x15a*,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a*\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x06\xA5V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa*\x9F\x91\x90a5\xD3V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a*\xDCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a*\xE1V[``\x91P[P\x91P\x91Pa*\xF1\x82\x82\x86a*\xFCV[\x97\x96PPPPPPPV[``\x83\x15a+\x0BWP\x81a\x1E\xA7V[\x82Q\x15a+\x1BW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x91\x90a5\xEFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xB7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a+\\W`\0\x80\xFD[\x815a\x1E\xA7\x81a+5V[`\0` \x82\x84\x03\x12\x15a+yW`\0\x80\xFD[P5\x91\x90PV[\x805a+\x8B\x81a+5V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a+\xC9Wa+\xC9a+\x90V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a+\xF8Wa+\xF8a+\x90V[`@R\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a,\x19W`\0\x80\xFD[\x865a,$\x81a+5V[\x95P` \x87\x81\x015a,5\x81a+5V[\x95P`@\x88\x015\x94P``\x88\x015a,L\x81a+5V[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a,pW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a,\x84W`\0\x80\xFD[\x815\x81\x81\x11\x15a,\x96Wa,\x96a+\x90V[a,\xA8`\x1F\x82\x01`\x1F\x19\x16\x85\x01a+\xCFV[\x91P\x80\x82R\x8B\x84\x82\x85\x01\x01\x11\x15a,\xBEW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[\x80\x15\x15\x81\x14a\x06\xB7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a-\0W`\0\x80\xFD[\x825a-\x0B\x81a+5V[\x91P` \x83\x015a-\x1B\x81a,\xDFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a-8W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x1E\xA7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a-\\W`\0\x80\xFD[\x825a-g\x81a+5V[\x91P` \x83\x015a-\x1B\x81a+5V[`\0\x80`\0``\x84\x86\x03\x12\x15a-\x8CW`\0\x80\xFD[\x835a-\x97\x81a+5V[\x92P` \x84\x015a-\xA7\x81a+5V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a-\xF1W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a-\xCCV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a-\xF1W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.\x10V[`@\x81R`\0a.?`@\x83\x01\x85a-\xB8V[\x82\x81\x03` \x84\x01Ra.Q\x81\x85a-\xFCV[\x95\x94PPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a.tWa.ta+\x90V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a.\x8FW`\0\x80\xFD[\x815` a.\xA4a.\x9F\x83a.ZV[a+\xCFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a.\xC3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a.\xE7W\x805a.\xDA\x81a+5V[\x83R\x91\x83\x01\x91\x83\x01a.\xC7V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a/\x03W`\0\x80\xFD[\x815` a/\x13a.\x9F\x83a.ZV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a.\xE7W\x805\x83R\x91\x83\x01\x91\x83\x01a/6V[`\0`@\x82\x84\x03\x12\x15a/_W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/\x82Wa/\x82a+\x90V[`@R\x90P\x80\x825a/\x93\x81a+5V[\x81R` \x83\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\xB4W`\0\x80\xFD[` \x91\x90\x91\x01R\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a+\x8BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a/\xE7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a/\xFFW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a0\x13W`\0\x80\xFD[a0\x1Ba+\xA6V[\x825\x82\x81\x11\x15a0*W`\0\x80\xFD[a06\x87\x82\x86\x01a.~V[\x82RP` \x83\x015\x82\x81\x11\x15a0KW`\0\x80\xFD[a0W\x87\x82\x86\x01a.\xF2V[` \x83\x01RPa0i`@\x84\x01a+\x80V[`@\x82\x01Ra0{\x86``\x85\x01a/MV[``\x82\x01Ra0\x8C`\xA0\x84\x01a/\xC1V[`\x80\x82\x01Ra0\x9D`\xC0\x84\x01a+\x80V[`\xA0\x82\x01R\x95\x94PPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a0\xBDW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xD5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a%\xFAW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a1\x03W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x1AW`\0\x80\xFD[a1&\x85\x82\x86\x01a0\xABV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1HW`\0\x80\xFD[\x845a1S\x81a+5V[\x93P` \x85\x015a1c\x81a+5V[\x92P`@\x85\x015a1s\x81a+5V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1\x99W`\0\x80\xFD[\x845a1\xA4\x81a+5V[\x93P` \x85\x015a1\xB4\x81a+5V[\x92P`@\x85\x015\x91P``\x85\x015a1\xCB\x81a+5V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xE9W`\0\x80\xFD[\x825a1\xF4\x81a+5V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a2\x18W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a20W`\0\x80\xFD[a2<\x88\x83\x89\x01a0\xABV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a2UW`\0\x80\xFD[Pa2b\x87\x82\x88\x01a0\xABV[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a2\x80W`\0\x80\xFD[\x81Qa\x1E\xA7\x81a+5V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a2\xE7W`\0\x80\xFD[\x81Qa\x1E\xA7\x81a,\xDFV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`D\x90\x82\x01R\x7FStrategyManager.onlyStrategyWhit`@\x82\x01R\x7Felister: not the strategyWhiteli``\x82\x01Rc9\xBA2\xB9`\xE1\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`@\x90\x82\x01\x81\x90R\x7FStrategyManager.onlyDelegationMa\x90\x82\x01R\x7Fnager: not the DelegationManager``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\xE0\x81R`\0a4+`\xE0\x83\x01\x89a-\xB8V[\x82\x81\x03` \x84\x01Ra4=\x81\x89a-\xFCV[`\x01`\x01`\xA0\x1B\x03\x97\x88\x16`@\x85\x01R\x86Q\x88\x16``\x85\x01R` \x90\x96\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x84\x01RPPc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\xA0\x83\x01R\x90\x92\x16`\xC0\x90\x92\x01\x91\x90\x91R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a4\xA3W`\0\x80\xFD[\x815a\x1E\xA7\x81a,\xDFV[`\0[\x83\x81\x10\x15a4\xC9W\x81\x81\x01Q\x83\x82\x01R` \x01a4\xB1V[\x83\x81\x11\x15a\x0CMWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra4\xF2\x81` \x86\x01` \x86\x01a4\xAEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a)\xC3`@\x83\x01\x84a4\xDAV[`\0` \x82\x84\x03\x12\x15a51W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1E\xA7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a5[W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a5\x8BWa5\x8Ba5bV[P\x01\x90V[`\0\x82\x82\x10\x15a5\xA2Wa5\xA2a5bV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82Qa5\xE5\x81\x84` \x87\x01a4\xAEV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x1E\xA7` \x83\x01\x84a4\xDAV\xFE\xA2dipfsX\"\x12 V\xDC\xEAK\xB1\x93{IH)\xE1\x84\x1C\x9F[\xE7\xE2\xE9\x0B\xC0\x0F\x87\x1B\x96:\xC1\x98b\x80JgfdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STRATEGYMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02'W`\x005`\xE0\x1C\x80c\x94\xF6I\xDD\x11a\x010W\x80c\xC6eg\x02\x11a\0\xB8W\x80c\xDF\\\xF7#\x11a\0|W\x80c\xDF\\\xF7#\x14a\x05\x96W\x80c\xE7\xA0P\xAA\x14a\x05\xBDW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xD0W\x80c\xF6\x98\xDA%\x14a\x05\xE3W\x80c\xFA\xBC\x1C\xBC\x14a\x05\xEBW`\0\x80\xFD[\x80c\xC6eg\x02\x14a\x05 W\x80c\xCB\xC2\xBDb\x14a\x053W\x80c\xCD)?o\x14a\x05FW\x80c\xCFuo\xDF\x14a\x05pW\x80c\xDF[5G\x14a\x05\x83W`\0\x80\xFD[\x80c\xB4;QK\x11a\0\xFFW\x80c\xB4;QK\x14a\x04\xB1W\x80c\xB5\xD8\xB5\xB8\x14a\x04\xC4W\x80c\xC3\xC6\xB3\xA9\x14a\x04\xD7W\x80c\xC4b>\xA1\x14a\x04\xFAW\x80c\xC6\x08\xC7\xF3\x14a\x05\rW`\0\x80\xFD[\x80c\x94\xF6I\xDD\x14a\x043W\x80c\x96\x7F\xC0\xD2\x14a\x04TW\x80c\x9BM\xA0=\x14a\x04gW\x80c\xB14Bq\x14a\x04\x8AW`\0\x80\xFD[\x80c\\\x97Z\xBB\x11a\x01\xB3W\x80c~\xCE\xBE\0\x11a\x01\x82W\x80c~\xCE\xBE\0\x14a\x03\xB3W\x80c\x88o\x11\x95\x14a\x03\xD3W\x80c\x8B\x8A\xAC<\x14a\x03\xE6W\x80c\x8C\x80\xD4\xE5\x14a\x04\x0FW\x80c\x8D\xA5\xCB[\x14a\x04\"W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03UW\x80cf<\x1D\xE4\x14a\x03]W\x80cqP\x18\xA6\x14a\x03\x80W\x80cz~\r\x92\x14a\x03\x88W`\0\x80\xFD[\x80cFe\xBC\xDA\x11a\x01\xFAW\x80cFe\xBC\xDA\x14a\x02\xA1W\x80cH\x82^\x94\x14a\x02\xE0W\x80cNZBc\x14a\x03\x07W\x80cY\\jg\x14a\x03\x1AW\x80cZ\xC8j\xB7\x14a\x03\"W`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x02,W\x80c\x13d9\xDD\x14a\x02AW\x80c `kp\x14a\x02TW\x80c2\xE8\x9A\xCE\x14a\x02\x8EW[`\0\x80\xFD[a\x02?a\x02:6`\x04a+JV[a\x05\xFEV[\0[a\x02?a\x02O6`\x04a+gV[a\x06\xBAV[a\x02{\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02{a\x02\x9C6`\x04a,\0V[a\x07\xF9V[a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x85V[a\x02{\x7FC7\xF8-\x14.A\xF2\xA8\xC1\x05G\xCD\x8C\x85\x9B\xDD\xB9\"b\xA6\x10X\xE7xB\xE2M\x9D\xEA\x92$\x81V[a\x02?a\x03\x156`\x04a,\xEDV[a\n\xE7V[a\x02?a\x0B\x1FV[a\x03Ea\x0306`\x04a-&V[`\x98T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x85V[`\x98Ta\x02{V[a\x03Ea\x03k6`\x04a+JV[`\xD1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02?a\x0B\xE6V[a\x02{a\x03\x966`\x04a-IV[`\xCD` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02{a\x03\xC16`\x04a+JV[`\xCA` R`\0\x90\x81R`@\x90 T\x81V[`\x97Ta\x02\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02{a\x03\xF46`\x04a+JV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xCE` R`@\x90 T\x90V[a\x02?a\x04\x1D6`\x04a-wV[a\x0B\xFAV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC8V[a\x04Fa\x04A6`\x04a+JV[a\x0CSV[`@Qa\x02\x85\x92\x91\x90a.,V[`\xCBTa\x02\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03Ea\x04u6`\x04a+JV[`\xD3` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02{a\x04\xBF6`\x04a/\xD5V[a\r\xD3V[a\x02?a\x04\xD26`\x04a0\xF0V[a\x0E V[a\x03Ea\x04\xE56`\x04a+gV[`\xCF` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02?a\x05\x086`\x04a12V[a\x0F\x94V[a\x02?a\x05\x1B6`\x04a1\x83V[a\x0F\xE8V[a\x02?a\x05.6`\x04a+JV[a\x10\xA0V[a\x02\xC8a\x05A6`\x04a1\xD6V[a\x10\xB1V[a\x05Ya\x05T6`\x04a/\xD5V[a\x10\xE9V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x85V[a\x02?a\x05~6`\x04a12V[a\x11}V[a\x02?a\x05\x916`\x04a2\x02V[a\x12\xB1V[a\x02\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02{a\x05\xCB6`\x04a-wV[a\x14\xDAV[a\x02?a\x05\xDE6`\x04a+JV[a\x15\xA3V[a\x02{a\x16\x19V[a\x02?a\x05\xF96`\x04a+gV[a\x16WV[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06u\x91\x90a2nV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\x8BV[`@Q\x80\x91\x03\x90\xFD[a\x06\xB7\x81a\x17\xB3V[PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07&\x91\x90a2\xD5V[a\x07BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\xF2V[`\x98T\x81\x81\x16\x14a\x07\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x98T`\0\x90\x81\x90`\x01\x90\x81\x16\x14\x15a\x08PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eT\x14\x15a\x08\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eU`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\xD3` R`@\x90 T`\xFF\x16\x15a\tJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FStrategyManager.depositIntoStrat`D\x82\x01R\x7FegyWithSignature: third transfer`d\x82\x01Ri\x1C\xC8\x19\x1A\\\xD8X\x9B\x19Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[B\x84\x10\x15a\t\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FStrategyManager.depositIntoStrat`D\x82\x01R\x7FegyWithSignature: signature expi`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x81\x81R`\xCA` \x90\x81R`@\x80\x83 T\x81Q\x7FC7\xF8-\x14.A\xF2\xA8\xC1\x05G\xCD\x8C\x85\x9B\xDD\xB9\"b\xA6\x10X\xE7xB\xE2M\x9D\xEA\x92$\x93\x81\x01\x93\x90\x93R\x90\x82\x01\x93\x90\x93R\x8B\x84\x16``\x82\x01R\x92\x8A\x16`\x80\x84\x01R`\xA0\x83\x01\x89\x90R`\xC0\x83\x01\x82\x90R`\xE0\x83\x01\x87\x90R\x90\x91a\x01\0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\xCA\x90\x93R\x90\x82 `\x01\x85\x01\x90U\x91Pa\n\x84a\x16\x19V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\n\xC7\x88\x82\x88a\x18\xAAV[a\n\xD3\x88\x8C\x8C\x8Ca\x1AiV[`\x01`eU\x9B\x9APPPPPPPPPPPV[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3:V[a\x0B\x1B\x82\x82a\x1C8V[PPV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x8B\x91\x90a2\xD5V[a\x0B\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\xF2V[`\0\x19`\x98\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0B\xEEa\x1C\xA6V[a\x0B\xF8`\0a\x1D\0V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0CBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[a\x0CM\x83\x83\x83a\x1DRV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCE` R`@\x81 T``\x91\x82\x91\x90\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x8BWa\x0C\x8Ba+\x90V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xB4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\rEW`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 `\xCE\x90\x92R\x82 \x80T\x91\x92\x91\x84\x90\x81\x10a\x0C\xF9Wa\x0C\xF9a4\x02V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83R\x82\x01\x92\x90\x92R`@\x01\x90 T\x82Q\x83\x90\x83\x90\x81\x10a\r2Wa\r2a4\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0C\xBAV[P`\xCE`\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x81\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\r\xC1W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\r\xA3W[PPPPP\x91P\x93P\x93PPP\x91P\x91V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a\x0E\x03\x97\x90\x96\x95\x91\x01a4\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EJW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3:V[\x80`\0[\x81\x81\x10\x15a\x0CMW`\xD1`\0\x85\x85\x84\x81\x81\x10a\x0ElWa\x0Ela4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0E\x81\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x0F\x8CW`\0`\xD1`\0\x86\x86\x85\x81\x81\x10a\x0E\xBBWa\x0E\xBBa4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xD0\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U\x7F@tA;KD>NX\x01\x9F(U\xA8vQ\x135\x8C|r\xE3\x95\t\xC6\xAFE\xFC\x0F[\xA00\x84\x84\x83\x81\x81\x10a\x0F+Wa\x0F+a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0F@\x91\x90a+JV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1a\x0F\x8C\x84\x84\x83\x81\x81\x10a\x0FpWa\x0Fpa4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x0F\x85\x91\x90a+JV[`\0a\x1C8V[`\x01\x01a\x0ENV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0F\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[a\x0CM\x84\x84\x84\x84a\x1E\xAEV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x100W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[`@Qcl\xE5v\x89`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x84\x16\x90c\xD9\xCA\xED\x12\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\x96W=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x10\xA8a\x1C\xA6V[a\x06\xB7\x81a!;V[`\xCE` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x10\xCDW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`\0\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x114W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3\xA4V[`\0a\x11?\x84a\r\xD3V[`\0\x81\x81R`\xCF` R`@\x81 T\x91\x92P\x90`\xFF\x16\x15a\x11tWP`\0\x81\x81R`\xCF` R`@\x90 \x80T`\xFF\x19\x16\x90U`\x01[\x92P\x90P\x91P\x91V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11\x9DWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\xB7WP0;\x15\x80\x15a\x11\xB7WP`\0T`\xFF\x16`\x01\x14[a\x12\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12=W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12Ea!\xA4V[`\xC9Ua\x12R\x83\x83a\";V[a\x12[\x85a\x1D\0V[a\x12d\x84a!;V[\x80\x15a\x12\xAAW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a3:V[\x82\x81\x14a\x13dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FStrategyManager.addStrategiesToD`D\x82\x01R\x7FepositWhitelist: array lengths d`d\x82\x01Rj\r\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xAB\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[\x82`\0[\x81\x81\x10\x15a\x14\xD2W`\xD1`\0\x87\x87\x84\x81\x81\x10a\x13\x86Wa\x13\x86a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x13\x9B\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16a\x14\xCAW`\x01`\xD1`\0\x88\x88\x85\x81\x81\x10a\x13\xD4Wa\x13\xD4a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x13\xE9\x91\x90a+JV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U\x7F\x0C5\xB1}\x91\xC9n\xB2u\x1C\xD4V\xE1%/B\xA3\x86\xE5$\xEF\x9F\xF2n\xCC\x99P\x85\x9F\xDC\x04\xFE\x86\x86\x83\x81\x81\x10a\x14DWa\x14Da4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x14Y\x91\x90a+JV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1a\x14\xCA\x86\x86\x83\x81\x81\x10a\x14\x89Wa\x14\x89a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x14\x9E\x91\x90a+JV[\x85\x85\x84\x81\x81\x10a\x14\xB0Wa\x14\xB0a4\x02V[\x90P` \x02\x01` \x81\x01\x90a\x14\xC5\x91\x90a4\x91V[a\x1C8V[`\x01\x01a\x13hV[PPPPPPV[`\x98T`\0\x90\x81\x90`\x01\x90\x81\x16\x14\x15a\x151W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x14\x18]\\\xD8X\x9B\x19N\x88\x1A[\x99\x19^\x08\x1A\\\xC8\x1C\x18]\\\xD9Y`:\x1B`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eT\x14\x15a\x15\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x06\xA5V[`\x02`eUa\x15\x953\x86\x86\x86a\x1AiV[`\x01`eU\x95\x94PPPPPV[a\x15\xABa\x1C\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[a\x06\xB7\x81a\x1D\0V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\x16JWP`\xC9T\x90V[a\x16Ra!\xA4V[\x90P\x90V[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xCE\x91\x90a2nV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x90a2\x8BV[`\x98T\x19\x81\x19`\x98T\x19\x16\x14a\x17|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\xEEV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x18AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x19\xC9W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a\x18\xEA\x90\x86\x90\x86\x90`\x04\x01a5\x06V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19+\x91\x90a5\x1FV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[PPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x19\xDD\x83\x83a#!V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xD1` R`@\x81 T\x84\x90`\xFF\x16a\x1B\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FStrategyManager.onlyStrategiesWh`D\x82\x01R\x7FitelistedForDeposit: strategy no`d\x82\x01Rl\x1D\x08\x1D\xDA\x1A]\x19[\x1A\\\xDD\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[a\x1B$`\x01`\x01`\xA0\x1B\x03\x85\x163\x87\x86a#EV[`@Qc\x11\xF9\xFB\xC9`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x86\x16\x90cG\xE7\xEF$\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1BsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x97\x91\x90a5IV[\x91Pa\x1B\xA5\x86\x85\x87\x85a\x1E\xAEV[`@Qc\x14R\xB9\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x86\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c(\xA5s\xAE\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\x17W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1C+W=`\0\x80>=`\0\xFD[PPPPP\x94\x93PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R\x82\x15\x15` \x82\x01R\x7Fw\xD90\xDFI7y4s\xA9P$\xD8z\x98\xFD,\xCB\x9E\x92\xD3\xC2F;=\xAC\xD6]>jW\x86\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\xD3` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xA5V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x81a\x1D\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStrategyManager._removeShares: s`D\x82\x01R\x7FhareAmount should not be zero!\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R T\x80\x83\x11\x15a\x1EYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FStrategyManager._removeShares: s`D\x82\x01Rr\r\x0C.L\xA8-\xAD\xEE\xAD\xCE\x84\x0E\x8D\xED\xE4\r\r,\xED`k\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x83\x82\x03\x90\x81\x90U\x90\x83\x14\x15a\x1E\xA1Wa\x1E\x97\x85\x85a#\x9FV[`\x01\x91PPa\x1E\xA7V[`\0\x91PP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1F*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStrategyManager._addShares: stak`D\x82\x01R\x7Fer cannot be zero address\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xA5V[\x80a\x1F\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FStrategyManager._addShares: shar`D\x82\x01Rues should not be zero!`P\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R Ta \xA7W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCE` \x90\x81R`@\x90\x91 T\x10a hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FStrategyManager._addShares: depo`D\x82\x01R\x7Fsit would exceed MAX_STAKER_STRA`d\x82\x01Ro\n\x88\xA8\xEB+\xE9\x89*j\x8B\xE9\x88\xA9\xC8\xEA\x89`\x83\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\xCE` \x90\x81R`@\x82 \x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a \xDE\x90\x84\x90a5xV[\x90\x91UPP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R``\x81\x01\x83\x90R\x90Q\x7F|\xFF\xF9\x08\xA4\xB5\x83\xF3d0\xB2]u\x96LE\x8D\x8E\xDE\x8A\x99\xBDa\xBEu\x0E\x97\xEE\x1B/:\x96\x91\x81\x90\x03`\x80\x01\x90\xA1PPPPV[`\xCBT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7FBd'^Y9U\xFF\x9DaF\xA5\x1AE%\xF6\xDD\xAC\xE2\xE8\x1D\xB99\x1A\xBC\xC9\xD1\xCAH\x04})\x91\x01`@Q\x80\x91\x03\x90\xA1`\xCB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`\x97T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\"\\WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\"\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x0B\x1B\x82a\x17\xB3V[`\0\x80`\0a#0\x85\x85a%\x91V[\x91P\x91Pa#=\x81a&\x01V[P\x93\x92PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x0CM\x90\x85\x90a'\xBCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCE` R`@\x81 T\x90[\x81\x81\x10\x15a$\xBAW`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\xCE` R`@\x90 \x80T\x91\x85\x16\x91\x83\x90\x81\x10a#\xF1Wa#\xF1a4\x02V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a$\xB2W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCE` R`@\x90 \x80Ta$2\x90`\x01\x90a5\x90V[\x81T\x81\x10a$BWa$Ba4\x02V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x84R`\xCE\x90\x92R`@\x90\x92 \x80T\x91\x90\x92\x16\x91\x90\x83\x90\x81\x10a$\x7FWa$\x7Fa4\x02V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPa$\xBAV[`\x01\x01a#\xBAV[\x81\x81\x14\x15a%BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FStrategyManager._removeStrategyF`D\x82\x01R\x7FromStakerStrategyList: strategy `d\x82\x01Rh\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`\xBA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCE` R`@\x90 \x80T\x80a%iWa%ia5\xA7V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPPV[`\0\x80\x82Q`A\x14\x15a%\xC8W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa%\xBC\x87\x82\x85\x85a(\x8EV[\x94P\x94PPPPa%\xFAV[\x82Q`@\x14\x15a%\xF2W` \x83\x01Q`@\x84\x01Qa%\xE7\x86\x83\x83a){V[\x93P\x93PPPa%\xFAV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a&\x15Wa&\x15a5\xBDV[\x14\x15a&\x1EWPV[`\x01\x81`\x04\x81\x11\x15a&2Wa&2a5\xBDV[\x14\x15a&\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xA5V[`\x02\x81`\x04\x81\x11\x15a&\x94Wa&\x94a5\xBDV[\x14\x15a&\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x06\xA5V[`\x03\x81`\x04\x81\x11\x15a&\xF6Wa&\xF6a5\xBDV[\x14\x15a'OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x04\x81`\x04\x81\x11\x15a'cWa'ca5\xBDV[\x14\x15a\x06\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\0a(\x11\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a)\xB4\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x19\xC4W\x80\x80` \x01\x90Q\x81\x01\x90a(/\x91\x90a2\xD5V[a\x19\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a(\xC5WP`\0\x90P`\x03a)rV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a(\xDDWP\x84`\xFF\x16`\x1C\x14\x15[\x15a(\xEEWP`\0\x90P`\x04a)rV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a)BW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a)kW`\0`\x01\x92P\x92PPa)rV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a)\x98`\xFF\x86\x90\x1C`\x1Ba5xV[\x90Pa)\xA6\x87\x82\x88\x85a(\x8EV[\x93P\x93PPP\x93P\x93\x91PPV[``a)\xC3\x84\x84`\0\x85a)\xCBV[\x94\x93PPPPV[``\x82G\x10\x15a*,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x06\xA5V[`\x01`\x01`\xA0\x1B\x03\x85\x16;a*\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x06\xA5V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa*\x9F\x91\x90a5\xD3V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a*\xDCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a*\xE1V[``\x91P[P\x91P\x91Pa*\xF1\x82\x82\x86a*\xFCV[\x97\x96PPPPPPPV[``\x83\x15a+\x0BWP\x81a\x1E\xA7V[\x82Q\x15a+\x1BW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xA5\x91\x90a5\xEFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xB7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a+\\W`\0\x80\xFD[\x815a\x1E\xA7\x81a+5V[`\0` \x82\x84\x03\x12\x15a+yW`\0\x80\xFD[P5\x91\x90PV[\x805a+\x8B\x81a+5V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a+\xC9Wa+\xC9a+\x90V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a+\xF8Wa+\xF8a+\x90V[`@R\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a,\x19W`\0\x80\xFD[\x865a,$\x81a+5V[\x95P` \x87\x81\x015a,5\x81a+5V[\x95P`@\x88\x015\x94P``\x88\x015a,L\x81a+5V[\x93P`\x80\x88\x015\x92P`\xA0\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a,pW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a,\x84W`\0\x80\xFD[\x815\x81\x81\x11\x15a,\x96Wa,\x96a+\x90V[a,\xA8`\x1F\x82\x01`\x1F\x19\x16\x85\x01a+\xCFV[\x91P\x80\x82R\x8B\x84\x82\x85\x01\x01\x11\x15a,\xBEW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[\x80\x15\x15\x81\x14a\x06\xB7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a-\0W`\0\x80\xFD[\x825a-\x0B\x81a+5V[\x91P` \x83\x015a-\x1B\x81a,\xDFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a-8W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x1E\xA7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a-\\W`\0\x80\xFD[\x825a-g\x81a+5V[\x91P` \x83\x015a-\x1B\x81a+5V[`\0\x80`\0``\x84\x86\x03\x12\x15a-\x8CW`\0\x80\xFD[\x835a-\x97\x81a+5V[\x92P` \x84\x015a-\xA7\x81a+5V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a-\xF1W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a-\xCCV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a-\xF1W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.\x10V[`@\x81R`\0a.?`@\x83\x01\x85a-\xB8V[\x82\x81\x03` \x84\x01Ra.Q\x81\x85a-\xFCV[\x95\x94PPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a.tWa.ta+\x90V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a.\x8FW`\0\x80\xFD[\x815` a.\xA4a.\x9F\x83a.ZV[a+\xCFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a.\xC3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a.\xE7W\x805a.\xDA\x81a+5V[\x83R\x91\x83\x01\x91\x83\x01a.\xC7V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a/\x03W`\0\x80\xFD[\x815` a/\x13a.\x9F\x83a.ZV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a.\xE7W\x805\x83R\x91\x83\x01\x91\x83\x01a/6V[`\0`@\x82\x84\x03\x12\x15a/_W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/\x82Wa/\x82a+\x90V[`@R\x90P\x80\x825a/\x93\x81a+5V[\x81R` \x83\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\xB4W`\0\x80\xFD[` \x91\x90\x91\x01R\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a+\x8BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a/\xE7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a/\xFFW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a0\x13W`\0\x80\xFD[a0\x1Ba+\xA6V[\x825\x82\x81\x11\x15a0*W`\0\x80\xFD[a06\x87\x82\x86\x01a.~V[\x82RP` \x83\x015\x82\x81\x11\x15a0KW`\0\x80\xFD[a0W\x87\x82\x86\x01a.\xF2V[` \x83\x01RPa0i`@\x84\x01a+\x80V[`@\x82\x01Ra0{\x86``\x85\x01a/MV[``\x82\x01Ra0\x8C`\xA0\x84\x01a/\xC1V[`\x80\x82\x01Ra0\x9D`\xC0\x84\x01a+\x80V[`\xA0\x82\x01R\x95\x94PPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a0\xBDW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0\xD5W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a%\xFAW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a1\x03W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x1AW`\0\x80\xFD[a1&\x85\x82\x86\x01a0\xABV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1HW`\0\x80\xFD[\x845a1S\x81a+5V[\x93P` \x85\x015a1c\x81a+5V[\x92P`@\x85\x015a1s\x81a+5V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a1\x99W`\0\x80\xFD[\x845a1\xA4\x81a+5V[\x93P` \x85\x015a1\xB4\x81a+5V[\x92P`@\x85\x015\x91P``\x85\x015a1\xCB\x81a+5V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15a1\xE9W`\0\x80\xFD[\x825a1\xF4\x81a+5V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a2\x18W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a20W`\0\x80\xFD[a2<\x88\x83\x89\x01a0\xABV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a2UW`\0\x80\xFD[Pa2b\x87\x82\x88\x01a0\xABV[\x95\x98\x94\x97P\x95PPPPV[`\0` \x82\x84\x03\x12\x15a2\x80W`\0\x80\xFD[\x81Qa\x1E\xA7\x81a+5V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a2\xE7W`\0\x80\xFD[\x81Qa\x1E\xA7\x81a,\xDFV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`D\x90\x82\x01R\x7FStrategyManager.onlyStrategyWhit`@\x82\x01R\x7Felister: not the strategyWhiteli``\x82\x01Rc9\xBA2\xB9`\xE1\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`@\x90\x82\x01\x81\x90R\x7FStrategyManager.onlyDelegationMa\x90\x82\x01R\x7Fnager: not the DelegationManager``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\xE0\x81R`\0a4+`\xE0\x83\x01\x89a-\xB8V[\x82\x81\x03` \x84\x01Ra4=\x81\x89a-\xFCV[`\x01`\x01`\xA0\x1B\x03\x97\x88\x16`@\x85\x01R\x86Q\x88\x16``\x85\x01R` \x90\x96\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x84\x01RPPc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\xA0\x83\x01R\x90\x92\x16`\xC0\x90\x92\x01\x91\x90\x91R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a4\xA3W`\0\x80\xFD[\x815a\x1E\xA7\x81a,\xDFV[`\0[\x83\x81\x10\x15a4\xC9W\x81\x81\x01Q\x83\x82\x01R` \x01a4\xB1V[\x83\x81\x11\x15a\x0CMWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra4\xF2\x81` \x86\x01` \x86\x01a4\xAEV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a)\xC3`@\x83\x01\x84a4\xDAV[`\0` \x82\x84\x03\x12\x15a51W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1E\xA7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a5[W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a5\x8BWa5\x8Ba5bV[P\x01\x90V[`\0\x82\x82\x10\x15a5\xA2Wa5\xA2a5bV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x82Qa5\xE5\x81\x84` \x87\x01a4\xAEV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x1E\xA7` \x83\x01\x84a4\xDAV\xFE\xA2dipfsX\"\x12 V\xDC\xEAK\xB1\x93{IH)\xE1\x84\x1C\x9F[\xE7\xE2\xE9\x0B\xC0\x0F\x87\x1B\x96:\xC1\x98b\x80JgfdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STRATEGYMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StrategyManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StrategyManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StrategyManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StrategyManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StrategyManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StrategyManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StrategyManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STRATEGYMANAGER_ABI.clone(),
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
                STRATEGYMANAGER_ABI.clone(),
                STRATEGYMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEPOSIT_TYPEHASH` (0x48825e94) function
        pub fn deposit_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([72, 130, 94, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function
        pub fn domain_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addShares` (0xc4623ea1) function
        pub fn add_shares(
            &self,
            staker: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 98, 62, 161], (staker, token, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStrategiesToDepositWhitelist` (0xdf5b3547) function
        pub fn add_strategies_to_deposit_whitelist(
            &self,
            strategies_to_whitelist: ::std::vec::Vec<::ethers::core::types::Address>,
            third_party_transfers_forbidden_values: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [223, 91, 53, 71],
                    (strategies_to_whitelist, third_party_transfers_forbidden_values),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateWithdrawalRoot` (0xb43b514b) function
        pub fn calculate_withdrawal_root(
            &self,
            queued_withdrawal: DeprecatedStructQueuedWithdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([180, 59, 81, 75], (queued_withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositIntoStrategy` (0xe7a050aa) function
        pub fn deposit_into_strategy(
            &self,
            strategy: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 160, 80, 170], (strategy, token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositIntoStrategyWithSignature` (0x32e89ace) function
        pub fn deposit_into_strategy_with_signature(
            &self,
            strategy: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            staker: ::ethers::core::types::Address,
            expiry: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [50, 232, 154, 206],
                    (strategy, token, amount, staker, expiry, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eigenPodManager` (0x4665bcda) function
        pub fn eigen_pod_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 101, 188, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeposits` (0x94f649dd) function
        pub fn get_deposits(
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
                .method_hash([148, 246, 73, 221], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xcf756fdf) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            initial_strategy_whitelister: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [207, 117, 111, 223],
                    (
                        initial_owner,
                        initial_strategy_whitelister,
                        pauser_registry,
                        initial_paused_status,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateQueuedWithdrawal` (0xcd293f6f) function
        pub fn migrate_queued_withdrawal(
            &self,
            queued_withdrawal: DeprecatedStructQueuedWithdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, [u8; 32])> {
            self.0
                .method_hash([205, 41, 63, 111], (queued_withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeShares` (0x8c80d4e5) function
        pub fn remove_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 128, 212, 229], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStrategiesFromDepositWhitelist` (0xb5d8b5b8) function
        pub fn remove_strategies_from_deposit_whitelist(
            &self,
            strategies_to_remove_from_whitelist: ::std::vec::Vec<
                ::ethers::core::types::Address,
            >,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 216, 181, 184], strategies_to_remove_from_whitelist)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `setStrategyWhitelister` (0xc6656702) function
        pub fn set_strategy_whitelister(
            &self,
            new_strategy_whitelister: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 101, 103, 2], new_strategy_whitelister)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setThirdPartyTransfersForbidden` (0x4e5a4263) function
        pub fn set_third_party_transfers_forbidden(
            &self,
            strategy: ::ethers::core::types::Address,
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 90, 66, 99], (strategy, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 52, 66, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrategyList` (0xcbc2bd62) function
        pub fn staker_strategy_list(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([203, 194, 189, 98], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrategyListLength` (0x8b8aac3c) function
        pub fn staker_strategy_list_length(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([139, 138, 172, 60], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerStrategyShares` (0x7a7e0d92) function
        pub fn staker_strategy_shares(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([122, 126, 13, 146], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyIsWhitelistedForDeposit` (0x663c1de4) function
        pub fn strategy_is_whitelisted_for_deposit(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 60, 29, 228], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyWhitelister` (0x967fc0d2) function
        pub fn strategy_whitelister(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([150, 127, 192, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `thirdPartyTransfersForbidden` (0x9b4da03d) function
        pub fn third_party_transfers_forbidden(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([155, 77, 160, 61], p0)
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
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawSharesAsTokens` (0xc608c7f3) function
        pub fn withdraw_shares_as_tokens(
            &self,
            recipient: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 8, 199, 243], (recipient, strategy, shares, token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawalRootPending` (0xc3c6b3a9) function
        pub fn withdrawal_root_pending(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 198, 179, 169], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRegistrySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyAddedToDepositWhitelist` event
        pub fn strategy_added_to_deposit_whitelist_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyAddedToDepositWhitelistFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyRemovedFromDepositWhitelist` event
        pub fn strategy_removed_from_deposit_whitelist_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyRemovedFromDepositWhitelistFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyWhitelisterChanged` event
        pub fn strategy_whitelister_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyWhitelisterChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UpdatedThirdPartyTransfersForbidden` event
        pub fn updated_third_party_transfers_forbidden_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdatedThirdPartyTransfersForbiddenFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StrategyManager<M> {
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
        Hash
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,address,uint256)")]
    pub struct DepositFilter {
        pub staker: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethevent(
        name = "StrategyAddedToDepositWhitelist",
        abi = "StrategyAddedToDepositWhitelist(address)"
    )]
    pub struct StrategyAddedToDepositWhitelistFilter {
        pub strategy: ::ethers::core::types::Address,
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
        name = "StrategyRemovedFromDepositWhitelist",
        abi = "StrategyRemovedFromDepositWhitelist(address)"
    )]
    pub struct StrategyRemovedFromDepositWhitelistFilter {
        pub strategy: ::ethers::core::types::Address,
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
        name = "StrategyWhitelisterChanged",
        abi = "StrategyWhitelisterChanged(address,address)"
    )]
    pub struct StrategyWhitelisterChangedFilter {
        pub previous_address: ::ethers::core::types::Address,
        pub new_address: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethevent(
        name = "UpdatedThirdPartyTransfersForbidden",
        abi = "UpdatedThirdPartyTransfersForbidden(address,bool)"
    )]
    pub struct UpdatedThirdPartyTransfersForbiddenFilter {
        pub strategy: ::ethers::core::types::Address,
        pub value: bool,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StrategyManagerEvents {
        DepositFilter(DepositFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        StrategyAddedToDepositWhitelistFilter(StrategyAddedToDepositWhitelistFilter),
        StrategyRemovedFromDepositWhitelistFilter(
            StrategyRemovedFromDepositWhitelistFilter,
        ),
        StrategyWhitelisterChangedFilter(StrategyWhitelisterChangedFilter),
        UnpausedFilter(UnpausedFilter),
        UpdatedThirdPartyTransfersForbiddenFilter(
            UpdatedThirdPartyTransfersForbiddenFilter,
        ),
    }
    impl ::ethers::contract::EthLogDecode for StrategyManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(StrategyManagerEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StrategyManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StrategyManagerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(StrategyManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(StrategyManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToDepositWhitelistFilter::decode_log(log) {
                return Ok(
                    StrategyManagerEvents::StrategyAddedToDepositWhitelistFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyRemovedFromDepositWhitelistFilter::decode_log(
                log,
            ) {
                return Ok(
                    StrategyManagerEvents::StrategyRemovedFromDepositWhitelistFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = StrategyWhitelisterChangedFilter::decode_log(log) {
                return Ok(
                    StrategyManagerEvents::StrategyWhitelisterChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(StrategyManagerEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = UpdatedThirdPartyTransfersForbiddenFilter::decode_log(
                log,
            ) {
                return Ok(
                    StrategyManagerEvents::UpdatedThirdPartyTransfersForbiddenFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StrategyManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyAddedToDepositWhitelistFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyRemovedFromDepositWhitelistFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyWhitelisterChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatedThirdPartyTransfersForbiddenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for StrategyManagerEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for StrategyManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for StrategyManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for StrategyManagerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for StrategyManagerEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToDepositWhitelistFilter>
    for StrategyManagerEvents {
        fn from(value: StrategyAddedToDepositWhitelistFilter) -> Self {
            Self::StrategyAddedToDepositWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromDepositWhitelistFilter>
    for StrategyManagerEvents {
        fn from(value: StrategyRemovedFromDepositWhitelistFilter) -> Self {
            Self::StrategyRemovedFromDepositWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<StrategyWhitelisterChangedFilter>
    for StrategyManagerEvents {
        fn from(value: StrategyWhitelisterChangedFilter) -> Self {
            Self::StrategyWhitelisterChangedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for StrategyManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    impl ::core::convert::From<UpdatedThirdPartyTransfersForbiddenFilter>
    for StrategyManagerEvents {
        fn from(value: UpdatedThirdPartyTransfersForbiddenFilter) -> Self {
            Self::UpdatedThirdPartyTransfersForbiddenFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEPOSIT_TYPEHASH` function with signature `DEPOSIT_TYPEHASH()` and selector `0x48825e94`
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
    #[ethcall(name = "DEPOSIT_TYPEHASH", abi = "DEPOSIT_TYPEHASH()")]
    pub struct DepositTypehashCall;
    ///Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    ///Container type for all input parameters for the `addShares` function with signature `addShares(address,address,address,uint256)` and selector `0xc4623ea1`
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
    #[ethcall(name = "addShares", abi = "addShares(address,address,address,uint256)")]
    pub struct AddSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addStrategiesToDepositWhitelist` function with signature `addStrategiesToDepositWhitelist(address[],bool[])` and selector `0xdf5b3547`
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
        name = "addStrategiesToDepositWhitelist",
        abi = "addStrategiesToDepositWhitelist(address[],bool[])"
    )]
    pub struct AddStrategiesToDepositWhitelistCall {
        pub strategies_to_whitelist: ::std::vec::Vec<::ethers::core::types::Address>,
        pub third_party_transfers_forbidden_values: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xb43b514b`
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
        name = "calculateWithdrawalRoot",
        abi = "calculateWithdrawalRoot((address[],uint256[],address,(address,uint96),uint32,address))"
    )]
    pub struct CalculateWithdrawalRootCall {
        pub queued_withdrawal: DeprecatedStructQueuedWithdrawal,
    }
    ///Container type for all input parameters for the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
    ///Container type for all input parameters for the `depositIntoStrategy` function with signature `depositIntoStrategy(address,address,uint256)` and selector `0xe7a050aa`
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
        name = "depositIntoStrategy",
        abi = "depositIntoStrategy(address,address,uint256)"
    )]
    pub struct DepositIntoStrategyCall {
        pub strategy: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `depositIntoStrategyWithSignature` function with signature `depositIntoStrategyWithSignature(address,address,uint256,address,uint256,bytes)` and selector `0x32e89ace`
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
        name = "depositIntoStrategyWithSignature",
        abi = "depositIntoStrategyWithSignature(address,address,uint256,address,uint256,bytes)"
    )]
    pub struct DepositIntoStrategyWithSignatureCall {
        pub strategy: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub staker: ::ethers::core::types::Address,
        pub expiry: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    #[ethcall(name = "eigenPodManager", abi = "eigenPodManager()")]
    pub struct EigenPodManagerCall;
    ///Container type for all input parameters for the `getDeposits` function with signature `getDeposits(address)` and selector `0x94f649dd`
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
    #[ethcall(name = "getDeposits", abi = "getDeposits(address)")]
    pub struct GetDepositsCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,uint256)` and selector `0xcf756fdf`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address,uint256)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub initial_strategy_whitelister: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `migrateQueuedWithdrawal` function with signature `migrateQueuedWithdrawal((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xcd293f6f`
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
        name = "migrateQueuedWithdrawal",
        abi = "migrateQueuedWithdrawal((address[],uint256[],address,(address,uint96),uint32,address))"
    )]
    pub struct MigrateQueuedWithdrawalCall {
        pub queued_withdrawal: DeprecatedStructQueuedWithdrawal,
    }
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `removeShares` function with signature `removeShares(address,address,uint256)` and selector `0x8c80d4e5`
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
    #[ethcall(name = "removeShares", abi = "removeShares(address,address,uint256)")]
    pub struct RemoveSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeStrategiesFromDepositWhitelist` function with signature `removeStrategiesFromDepositWhitelist(address[])` and selector `0xb5d8b5b8`
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
        name = "removeStrategiesFromDepositWhitelist",
        abi = "removeStrategiesFromDepositWhitelist(address[])"
    )]
    pub struct RemoveStrategiesFromDepositWhitelistCall {
        pub strategies_to_remove_from_whitelist: ::std::vec::Vec<
            ::ethers::core::types::Address,
        >,
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setPauserRegistry` function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`
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
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStrategyWhitelister` function with signature `setStrategyWhitelister(address)` and selector `0xc6656702`
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
    #[ethcall(name = "setStrategyWhitelister", abi = "setStrategyWhitelister(address)")]
    pub struct SetStrategyWhitelisterCall {
        pub new_strategy_whitelister: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setThirdPartyTransfersForbidden` function with signature `setThirdPartyTransfersForbidden(address,bool)` and selector `0x4e5a4263`
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
        name = "setThirdPartyTransfersForbidden",
        abi = "setThirdPartyTransfersForbidden(address,bool)"
    )]
    pub struct SetThirdPartyTransfersForbiddenCall {
        pub strategy: ::ethers::core::types::Address,
        pub value: bool,
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
        Hash
    )]
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
    ///Container type for all input parameters for the `stakerStrategyList` function with signature `stakerStrategyList(address,uint256)` and selector `0xcbc2bd62`
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
    #[ethcall(name = "stakerStrategyList", abi = "stakerStrategyList(address,uint256)")]
    pub struct StakerStrategyListCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `stakerStrategyListLength` function with signature `stakerStrategyListLength(address)` and selector `0x8b8aac3c`
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
        name = "stakerStrategyListLength",
        abi = "stakerStrategyListLength(address)"
    )]
    pub struct StakerStrategyListLengthCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stakerStrategyShares` function with signature `stakerStrategyShares(address,address)` and selector `0x7a7e0d92`
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
        name = "stakerStrategyShares",
        abi = "stakerStrategyShares(address,address)"
    )]
    pub struct StakerStrategySharesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `strategyIsWhitelistedForDeposit` function with signature `strategyIsWhitelistedForDeposit(address)` and selector `0x663c1de4`
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
        name = "strategyIsWhitelistedForDeposit",
        abi = "strategyIsWhitelistedForDeposit(address)"
    )]
    pub struct StrategyIsWhitelistedForDepositCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `strategyWhitelister` function with signature `strategyWhitelister()` and selector `0x967fc0d2`
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
    #[ethcall(name = "strategyWhitelister", abi = "strategyWhitelister()")]
    pub struct StrategyWhitelisterCall;
    ///Container type for all input parameters for the `thirdPartyTransfersForbidden` function with signature `thirdPartyTransfersForbidden(address)` and selector `0x9b4da03d`
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
        name = "thirdPartyTransfersForbidden",
        abi = "thirdPartyTransfersForbidden(address)"
    )]
    pub struct ThirdPartyTransfersForbiddenCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(name = "unpause", abi = "unpause(uint256)")]
    pub struct UnpauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawSharesAsTokens` function with signature `withdrawSharesAsTokens(address,address,uint256,address)` and selector `0xc608c7f3`
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
        name = "withdrawSharesAsTokens",
        abi = "withdrawSharesAsTokens(address,address,uint256,address)"
    )]
    pub struct WithdrawSharesAsTokensCall {
        pub recipient: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawalRootPending` function with signature `withdrawalRootPending(bytes32)` and selector `0xc3c6b3a9`
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
    #[ethcall(name = "withdrawalRootPending", abi = "withdrawalRootPending(bytes32)")]
    pub struct WithdrawalRootPendingCall(pub [u8; 32]);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StrategyManagerCalls {
        DepositTypehash(DepositTypehashCall),
        DomainTypehash(DomainTypehashCall),
        AddShares(AddSharesCall),
        AddStrategiesToDepositWhitelist(AddStrategiesToDepositWhitelistCall),
        CalculateWithdrawalRoot(CalculateWithdrawalRootCall),
        Delegation(DelegationCall),
        DepositIntoStrategy(DepositIntoStrategyCall),
        DepositIntoStrategyWithSignature(DepositIntoStrategyWithSignatureCall),
        DomainSeparator(DomainSeparatorCall),
        EigenPodManager(EigenPodManagerCall),
        GetDeposits(GetDepositsCall),
        Initialize(InitializeCall),
        MigrateQueuedWithdrawal(MigrateQueuedWithdrawalCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RemoveShares(RemoveSharesCall),
        RemoveStrategiesFromDepositWhitelist(RemoveStrategiesFromDepositWhitelistCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetStrategyWhitelister(SetStrategyWhitelisterCall),
        SetThirdPartyTransfersForbidden(SetThirdPartyTransfersForbiddenCall),
        Slasher(SlasherCall),
        StakerStrategyList(StakerStrategyListCall),
        StakerStrategyListLength(StakerStrategyListLengthCall),
        StakerStrategyShares(StakerStrategySharesCall),
        StrategyIsWhitelistedForDeposit(StrategyIsWhitelistedForDepositCall),
        StrategyWhitelister(StrategyWhitelisterCall),
        ThirdPartyTransfersForbidden(ThirdPartyTransfersForbiddenCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        WithdrawSharesAsTokens(WithdrawSharesAsTokensCall),
        WithdrawalRootPending(WithdrawalRootPendingCall),
    }
    impl ::ethers::core::abi::AbiDecode for StrategyManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DepositTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositTypehash(decoded));
            }
            if let Ok(decoded) = <DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainTypehash(decoded));
            }
            if let Ok(decoded) = <AddSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddShares(decoded));
            }
            if let Ok(decoded) = <AddStrategiesToDepositWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddStrategiesToDepositWhitelist(decoded));
            }
            if let Ok(decoded) = <CalculateWithdrawalRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateWithdrawalRoot(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) = <DepositIntoStrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositIntoStrategy(decoded));
            }
            if let Ok(decoded) = <DepositIntoStrategyWithSignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositIntoStrategyWithSignature(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <EigenPodManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EigenPodManager(decoded));
            }
            if let Ok(decoded) = <GetDepositsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeposits(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MigrateQueuedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MigrateQueuedWithdrawal(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) = <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) = <RemoveSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveShares(decoded));
            }
            if let Ok(decoded) = <RemoveStrategiesFromDepositWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveStrategiesFromDepositWhitelist(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetStrategyWhitelisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrategyWhitelister(decoded));
            }
            if let Ok(decoded) = <SetThirdPartyTransfersForbiddenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetThirdPartyTransfersForbidden(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakerStrategyListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerStrategyList(decoded));
            }
            if let Ok(decoded) = <StakerStrategyListLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerStrategyListLength(decoded));
            }
            if let Ok(decoded) = <StakerStrategySharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerStrategyShares(decoded));
            }
            if let Ok(decoded) = <StrategyIsWhitelistedForDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyIsWhitelistedForDeposit(decoded));
            }
            if let Ok(decoded) = <StrategyWhitelisterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyWhitelister(decoded));
            }
            if let Ok(decoded) = <ThirdPartyTransfersForbiddenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ThirdPartyTransfersForbidden(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <WithdrawSharesAsTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawSharesAsTokens(decoded));
            }
            if let Ok(decoded) = <WithdrawalRootPendingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawalRootPending(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StrategyManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStrategiesToDepositWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositIntoStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositIntoStrategyWithSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EigenPodManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MigrateQueuedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PausedWithIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStrategiesFromDepositWhitelist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrategyWhitelister(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetThirdPartyTransfersForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerStrategyList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerStrategyListLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerStrategyShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyIsWhitelistedForDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyWhitelister(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ThirdPartyTransfersForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawSharesAsTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalRootPending(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StrategyManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStrategiesToDepositWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositIntoStrategy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositIntoStrategyWithSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EigenPodManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateQueuedWithdrawal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStrategiesFromDepositWhitelist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrategyWhitelister(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetThirdPartyTransfersForbidden(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerStrategyList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerStrategyListLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerStrategyShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyIsWhitelistedForDeposit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyWhitelister(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ThirdPartyTransfersForbidden(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawSharesAsTokens(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalRootPending(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DepositTypehashCall> for StrategyManagerCalls {
        fn from(value: DepositTypehashCall) -> Self {
            Self::DepositTypehash(value)
        }
    }
    impl ::core::convert::From<DomainTypehashCall> for StrategyManagerCalls {
        fn from(value: DomainTypehashCall) -> Self {
            Self::DomainTypehash(value)
        }
    }
    impl ::core::convert::From<AddSharesCall> for StrategyManagerCalls {
        fn from(value: AddSharesCall) -> Self {
            Self::AddShares(value)
        }
    }
    impl ::core::convert::From<AddStrategiesToDepositWhitelistCall>
    for StrategyManagerCalls {
        fn from(value: AddStrategiesToDepositWhitelistCall) -> Self {
            Self::AddStrategiesToDepositWhitelist(value)
        }
    }
    impl ::core::convert::From<CalculateWithdrawalRootCall> for StrategyManagerCalls {
        fn from(value: CalculateWithdrawalRootCall) -> Self {
            Self::CalculateWithdrawalRoot(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for StrategyManagerCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DepositIntoStrategyCall> for StrategyManagerCalls {
        fn from(value: DepositIntoStrategyCall) -> Self {
            Self::DepositIntoStrategy(value)
        }
    }
    impl ::core::convert::From<DepositIntoStrategyWithSignatureCall>
    for StrategyManagerCalls {
        fn from(value: DepositIntoStrategyWithSignatureCall) -> Self {
            Self::DepositIntoStrategyWithSignature(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for StrategyManagerCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EigenPodManagerCall> for StrategyManagerCalls {
        fn from(value: EigenPodManagerCall) -> Self {
            Self::EigenPodManager(value)
        }
    }
    impl ::core::convert::From<GetDepositsCall> for StrategyManagerCalls {
        fn from(value: GetDepositsCall) -> Self {
            Self::GetDeposits(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StrategyManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MigrateQueuedWithdrawalCall> for StrategyManagerCalls {
        fn from(value: MigrateQueuedWithdrawalCall) -> Self {
            Self::MigrateQueuedWithdrawal(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for StrategyManagerCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StrategyManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for StrategyManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for StrategyManagerCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for StrategyManagerCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for StrategyManagerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for StrategyManagerCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RemoveSharesCall> for StrategyManagerCalls {
        fn from(value: RemoveSharesCall) -> Self {
            Self::RemoveShares(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesFromDepositWhitelistCall>
    for StrategyManagerCalls {
        fn from(value: RemoveStrategiesFromDepositWhitelistCall) -> Self {
            Self::RemoveStrategiesFromDepositWhitelist(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for StrategyManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for StrategyManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetStrategyWhitelisterCall> for StrategyManagerCalls {
        fn from(value: SetStrategyWhitelisterCall) -> Self {
            Self::SetStrategyWhitelister(value)
        }
    }
    impl ::core::convert::From<SetThirdPartyTransfersForbiddenCall>
    for StrategyManagerCalls {
        fn from(value: SetThirdPartyTransfersForbiddenCall) -> Self {
            Self::SetThirdPartyTransfersForbidden(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for StrategyManagerCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakerStrategyListCall> for StrategyManagerCalls {
        fn from(value: StakerStrategyListCall) -> Self {
            Self::StakerStrategyList(value)
        }
    }
    impl ::core::convert::From<StakerStrategyListLengthCall> for StrategyManagerCalls {
        fn from(value: StakerStrategyListLengthCall) -> Self {
            Self::StakerStrategyListLength(value)
        }
    }
    impl ::core::convert::From<StakerStrategySharesCall> for StrategyManagerCalls {
        fn from(value: StakerStrategySharesCall) -> Self {
            Self::StakerStrategyShares(value)
        }
    }
    impl ::core::convert::From<StrategyIsWhitelistedForDepositCall>
    for StrategyManagerCalls {
        fn from(value: StrategyIsWhitelistedForDepositCall) -> Self {
            Self::StrategyIsWhitelistedForDeposit(value)
        }
    }
    impl ::core::convert::From<StrategyWhitelisterCall> for StrategyManagerCalls {
        fn from(value: StrategyWhitelisterCall) -> Self {
            Self::StrategyWhitelister(value)
        }
    }
    impl ::core::convert::From<ThirdPartyTransfersForbiddenCall>
    for StrategyManagerCalls {
        fn from(value: ThirdPartyTransfersForbiddenCall) -> Self {
            Self::ThirdPartyTransfersForbidden(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StrategyManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for StrategyManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<WithdrawSharesAsTokensCall> for StrategyManagerCalls {
        fn from(value: WithdrawSharesAsTokensCall) -> Self {
            Self::WithdrawSharesAsTokens(value)
        }
    }
    impl ::core::convert::From<WithdrawalRootPendingCall> for StrategyManagerCalls {
        fn from(value: WithdrawalRootPendingCall) -> Self {
            Self::WithdrawalRootPending(value)
        }
    }
    ///Container type for all return fields from the `DEPOSIT_TYPEHASH` function with signature `DEPOSIT_TYPEHASH()` and selector `0x48825e94`
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
    pub struct DepositTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    pub struct DomainTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xb43b514b`
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
    pub struct CalculateWithdrawalRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `depositIntoStrategy` function with signature `depositIntoStrategy(address,address,uint256)` and selector `0xe7a050aa`
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
    pub struct DepositIntoStrategyReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `depositIntoStrategyWithSignature` function with signature `depositIntoStrategyWithSignature(address,address,uint256,address,uint256,bytes)` and selector `0x32e89ace`
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
    pub struct DepositIntoStrategyWithSignatureReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    pub struct EigenPodManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDeposits` function with signature `getDeposits(address)` and selector `0x94f649dd`
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
    pub struct GetDepositsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `migrateQueuedWithdrawal` function with signature `migrateQueuedWithdrawal((address[],uint256[],address,(address,uint96),uint32,address))` and selector `0xcd293f6f`
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
    pub struct MigrateQueuedWithdrawalReturn(pub bool, pub [u8; 32]);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakerStrategyList` function with signature `stakerStrategyList(address,uint256)` and selector `0xcbc2bd62`
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
    pub struct StakerStrategyListReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakerStrategyListLength` function with signature `stakerStrategyListLength(address)` and selector `0x8b8aac3c`
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
    pub struct StakerStrategyListLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerStrategyShares` function with signature `stakerStrategyShares(address,address)` and selector `0x7a7e0d92`
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
    pub struct StakerStrategySharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `strategyIsWhitelistedForDeposit` function with signature `strategyIsWhitelistedForDeposit(address)` and selector `0x663c1de4`
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
    pub struct StrategyIsWhitelistedForDepositReturn(pub bool);
    ///Container type for all return fields from the `strategyWhitelister` function with signature `strategyWhitelister()` and selector `0x967fc0d2`
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
    pub struct StrategyWhitelisterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `thirdPartyTransfersForbidden` function with signature `thirdPartyTransfersForbidden(address)` and selector `0x9b4da03d`
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
    pub struct ThirdPartyTransfersForbiddenReturn(pub bool);
    ///Container type for all return fields from the `withdrawalRootPending` function with signature `withdrawalRootPending(bytes32)` and selector `0xc3c6b3a9`
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
    pub struct WithdrawalRootPendingReturn(pub bool);
    ///`DeprecatedStructQueuedWithdrawal(address[],uint256[],address,(address,uint96),uint32,address)`
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
        Hash
    )]
    pub struct DeprecatedStructWithdrawerAndNonce {
        pub withdrawer: ::ethers::core::types::Address,
        pub nonce: u128,
    }
}
