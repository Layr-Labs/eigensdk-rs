pub use stake_registry::*;
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
pub mod stake_registry {
    const _: () = {
        ::core::include_bytes!(
            "/Users/supernovahs/Desktop/eigensdk-rs/crates/contracts/utils/json/StakeRegistry.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_registryCoordinator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IRegistryCoordinator",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_delegationManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IDelegationManager",),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_WEIGHING_FUNCTION_LENGTH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_WEIGHING_FUNCTION_LENGTH",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WEIGHTING_DIVISOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("WEIGHTING_DIVISOR"),
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
                    ::std::borrow::ToOwned::to_owned("addStrategies"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addStrategies"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_strategyParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IStakeRegistry.StrategyParams[]",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegation"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IDelegationManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCurrentStake"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentTotalStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCurrentTotalStake",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLatestStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLatestStakeUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.StakeUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumber",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumberAndIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumberAndIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeHistory"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeHistory"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.StakeUpdate[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeHistoryLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeHistoryLength",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("getStakeUpdateAtIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeUpdateAtIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.StakeUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeUpdateIndexAtBlockNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStakeUpdateIndexAtBlockNumber",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeAtBlockNumberFromIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getTotalStakeAtBlockNumberFromIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeHistoryLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalStakeHistoryLength",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalStakeIndicesAtBlockNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalStakeIndicesAtBlockNumber",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeUpdateAtIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalStakeUpdateAtIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.StakeUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_strategyParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IStakeRegistry.StrategyParams[]",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minimumStakeForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minimumStakeForQuorum",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("modifyStrategyParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("modifyStrategyParams",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategyIndices"),
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
                                name: ::std::borrow::ToOwned::to_owned("newMultipliers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96[]"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registryCoordinator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registryCoordinator",),
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
                    ::std::borrow::ToOwned::to_owned("removeStrategies"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeStrategies"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("indicesToRemove"),
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
                    ::std::borrow::ToOwned::to_owned("setMinimumStakeForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setMinimumStakeForQuorum",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategiesPerQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategiesPerQuorum",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("strategyParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyParams"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("multiplier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint96"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyParamsByIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyParamsByIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.StrategyParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyParamsLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyParamsLength",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("updateOperatorStake"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateOperatorStake",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(192usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint192"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightOfOperatorForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("weightOfOperatorForQuorum",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MinimumStakeForQuorumUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinimumStakeForQuorumUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorStakeUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QuorumCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("QuorumCreated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyAddedToQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StrategyAddedToQuorum",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyMultiplierUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StrategyMultiplierUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("multiplier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyRemovedFromQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StrategyRemovedFromQuorum",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static STAKEREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\09X8\x03\x80b\09X\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0eV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\x80Rb\0\0\xA4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0bW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0yW`\0\x80\xFD[\x82Qb\0\0\x86\x81b\0\0LV[` \x84\x01Q\x90\x92Pb\0\0\x99\x81b\0\0LV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa8Ob\0\x01\t`\09`\0\x81\x81a\x03z\x01R\x81\x81a\x062\x01R\x81\x81a\to\x01R\x81\x81a\x0C\xE6\x01R\x81\x81a\x11\x11\x01R\x81\x81a\x17\x03\x01R\x81\x81a\x18\x05\x01R\x81\x81a\x19)\x01Ra\x1C\xE7\x01R`\0\x81\x81a\x05)\x01Ra\x1E\xA8\x01Ra8O`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80c\x9F<\xCFe\x11a\x01\x0FW\x80c\xC8)LV\x11a\0\xA2W\x80c\xF2\xBE\x94\xAE\x11a\0qW\x80c\xF2\xBE\x94\xAE\x14a\x05KW\x80c\xF8Q\xE1\x98\x14a\x05^W\x80c\xFA(\xC6'\x14a\x05qW\x80c\xFFiJw\x14a\x05\x84W`\0\x80\xFD[\x80c\xC8)LV\x14a\x04\xD6W\x80c\xD5\xEC\xCC\x05\x14a\x04\xE9W\x80c\xDD\x98F\xB9\x14a\x04\xFCW\x80c\xDF\\\xF7#\x14a\x05$W`\0\x80\xFD[\x80c\xBC\x9A@\xC3\x11a\0\xDEW\x80c\xBC\x9A@\xC3\x14a\x04tW\x80c\xBD)\xB8\xCD\x14a\x04\x87W\x80c\xC4gx\xA5\x14a\x04\x9AW\x80c\xC6\x01R}\x14a\x04\xC3W`\0\x80\xFD[\x80c\x9F<\xCFe\x14a\x03\xEEW\x80c\xACk\xFB\x03\x14a\x04\x01W\x80c\xAD\xC8\x04\xDA\x14a\x04!W\x80c\xB6\x90Kx\x14a\x04aW`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\x87W\x80cf\xAC\xFE\xFE\x11a\x01VW\x80cf\xAC\xFE\xFE\x14a\x03JW\x80cm\x14\xA9\x87\x14a\x03uW\x80c|\x17#G\x14a\x03\xB4W\x80c\x81\xC0u\x02\x14a\x03\xCEW`\0\x80\xFD[\x80cK\xD2n\t\x14a\x02\xE5W\x80cT\x01\xED'\x14a\x03\x15W\x80c^Zgu\x14a\x03(W\x80c_\x1F-w\x14a\x037W`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xC3W\x80c \xB6b\x98\x14a\x02lW\x80c%PGw\x14a\x02\x81W\x80c,\xD9Y@\x14a\x02\xA2W\x80c<\xA5\xA5\xF5\x14a\x02\xC2W`\0\x80\xFD[\x80c\x04\x91\xB4\x1C\x14a\x01\xEAW\x80c\x08s$a\x14a\x02 W\x80c\x1F\x9Bt\xE0\x14a\x02AW[`\0\x80\xFD[a\x02\ra\x01\xF86`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a,BV[a\x05\x97V[`@Qa\x02\x17\x92\x91\x90a,lV[a\x02Ta\x02O6`\x04a,\xA6V[a\x05\xE0V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x02\x7Fa\x02z6`\x04a-!V[a\x060V[\0[a\x02\x94a\x02\x8F6`\x04a-\xE2V[a\taV[`@Qa\x02\x17\x92\x91\x90a.\x81V[a\x02\xB5a\x02\xB06`\x04a.\xA6V[a\x0C,V[`@Qa\x02\x17\x91\x90a.\xD2V[a\x02\ra\x02\xD06`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02\ra\x02\xF36`\x04a.\xA6V[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02Ta\x03#6`\x04a.\xA6V[a\x0C\xCBV[a\x02\rg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x7Fa\x03E6`\x04a/\xDBV[a\x0C\xE4V[a\x03]a\x03X6`\x04a-\xE2V[a\x11\x04V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xBC` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xE1a\x03\xDC6`\x04a0\x97V[a\x12^V[`@Qa\x02\x17\x91\x90a0\xE9V[a\x03\x9Ca\x03\xFC6`\x04a,BV[a\x15(V[a\x04\x14a\x04\x0F6`\x04a1'V[a\x15`V[`@Qa\x02\x17\x91\x90a1ZV[a\x044a\x04/6`\x04a,BV[a\x15\xF8V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x17V[a\x04\x14a\x04o6`\x04a,BV[a\x16rV[a\x02\x7Fa\x04\x826`\x04a1\xA6V[a\x17\x01V[a\x02\x7Fa\x04\x956`\x04a1\xD0V[a\x17\xFAV[a\x02Ta\x04\xA86`\x04a,'V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\x7Fa\x04\xD16`\x04a2\x9CV[a\x19'V[a\x02Ta\x04\xE46`\x04a2\xE9V[a\x1A\x1BV[a\x02Ta\x04\xF76`\x04a,'V[a\x1A\x99V[a\x05\x0Fa\x05\n6`\x04a3%V[a\x1A\xECV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02Ta\x05Y6`\x04a3aV[a\x1B\x01V[a\x04\x14a\x05l6`\x04a.\xA6V[a\x1B\x96V[a\x02Ta\x05\x7F6`\x04a3%V[a\x1C{V[a\x02\x7Fa\x05\x926`\x04a3\xA3V[a\x1C\xDCV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x05\xB3W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06&\x85\x85a\x1EGV[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x84a\x06\xFE\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x83\x80a\x07\x90W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06\x11V[\x82\x81\x14a\x08\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\tVW\x85\x85\x82\x81\x81\x10a\x083Wa\x083a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x08H\x91\x90a5\0V[\x82\x89\x89\x84\x81\x81\x10a\x08[Wa\x08[a4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08rWa\x08ra4\xEAV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08\xDBWa\x08\xDBa4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08\xF2Wa\x08\xF2a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\x19Wa\t\x19a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\t.\x91\x90a5\0V[`@Qa\t<\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a\tN\x81a51V[\x91PPa\x08\x19V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xC6Wa\t\xC6a/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xEFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x0CWa\n\x0Ca/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\x1EW`\0\x87\x87\x83\x81\x81\x10a\nWWa\nWa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\n\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\n\xEC\x83\x8Da\x1EGV[\x91P\x91P\x80a\x0B\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0a\x0B\x96\x8C\x85\x85a EV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0B\xABWa\x0B\xABa4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0B\xD5\x84\x82a\"\xC5V[\x86\x86\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x16\x90a51V[\x91PPa\n;V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x0C\xBEW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0CeV[PPPP\x90P[\x92\x91PPV[`\0\x80a\x0C\xD8\x84\x84a\x1B\x96V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rf\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\r\xB2\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\r\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x81Q\x80a\x0ECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x84\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x10\xFBW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0E\xA2Wa\x0E\xA2a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0E\xBAWa\x0E\xBAa4\xEAV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x0F\x18Wa\x0F\x18a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F0Wa\x0F0a4\xEAV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\x0Fp\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x0F\x80Wa\x0F\x80a4\xEAV[\x90`\0R` `\0 \x01\x83\x87\x83\x81Q\x81\x10a\x0F\x9DWa\x0F\x9Da4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\xB5Wa\x0F\xB5a4\xEAV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x10\x08Wa\x10\x08a5\xD5V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x100\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x10@Wa\x10@a4\xEAV[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x10qWa\x10qa4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x89Wa\x10\x89a4\xEAV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x10\xC7Wa\x10\xC7a5\xD5V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U\x80a\x10\xF3\x81a51V[\x91PPa\x0EbV[PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x80[\x83\x81\x10\x15a\x06&W`\0\x85\x85\x83\x81\x81\x10a\x11nWa\x11na4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x11\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\x12\x0B\x83\x8Ba\x1EGV[\x91P\x91P\x80a\x12-W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x12:\x8A\x85\x85a EV[\x90Pa\x12F\x84\x82a\"\xC5V[PPPPP\x80\x80a\x12V\x90a51V[\x91PPa\x11RV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12zWa\x12za/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\x1DW`\0\x85\x85\x83\x81\x81\x10a\x12\xC5Wa\x12\xC5a4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x13dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum does not`d\x82\x01Re\x08\x19^\x1A\\\xDD`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x13\x8DWa\x13\x8Da4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x149W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x15\x07W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x14}\x84\x86a5\xBEV[a\x14\x87\x91\x90a5\xBEV[\x81T\x81\x10a\x14\x97Wa\x14\x97a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x14\xF5W`\x01a\x14\xBA\x82\x84a5\xBEV[a\x14\xC4\x91\x90a5\xBEV[\x85\x85\x81Q\x81\x10a\x14\xD6Wa\x14\xD6a4\xEAV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x15\x07V[\x80a\x14\xFF\x81a51V[\x91PPa\x14NV[PPP\x80\x80a\x15\x15\x90a51V[\x91PPa\x12\xA9V[P\x90P[\x93\x92PPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15DW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\xA5Wa\x15\xA5a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x160Wa\x160a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x16\xAFWa\x16\xAFa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x83\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x17\xCF\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x17\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$?V[PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0[\x81\x81\x10\x15a\x19!W`\0\x83\x83\x83\x81\x81\x10a\x18aWa\x18aa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x18\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0a\x18\xFF\x86\x83`\0a EV[\x90Pa\x19\x0B\x82\x82a\"\xC5V[PPP\x80\x80a\x19\x19\x90a51V[\x91PPa\x18EV[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA9\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x19\xF5\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x1A\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$\xA8V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1ABWa\x1ABa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x0C\xD8\x81\x85a(\xEBV[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1A\xBA\x91a5\xBEV[\x81T\x81\x10a\x1A\xCAWa\x1A\xCAa4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x1A\xF9\x84\x84\x84a*eV[\x94\x93PPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B2Wa\x1B2a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1B\x89\x81\x86a(\xEBV[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1B\xEFW\x91Pa\x0C\xC5\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1C\x16`\x01\x84a5\xBEV[\x81T\x81\x10a\x1C&Wa\x1C&a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x0C\xC5\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1C\xA2\x85\x85\x85a*eV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C\xB8Wa\x1C\xB8a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1D\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06\x11V[a\x1D\xAC\x83\x82a$\xA8V[a\x1D\xB6\x83\x83a$?V[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\0\x80`\0\x80a\x1Ef\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\xFF\x87\x16`\0\x90\x81R`\x04` \x81\x90R`@\x80\x83 \x90Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1E\xDB\x92\x8C\x92\x01a5\xEBV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F \x91\x90\x81\x01\x90a6JV[\x90P`\0[\x83\x81\x10\x15a \x11W`\xFF\x89\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1FQWa\x1FQa4\xEAV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1F\x9FWa\x1F\x9Fa4\xEAV[` \x02` \x01\x01Q\x11\x15a\x1F\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1F\xD6Wa\x1F\xD6a4\xEAV[` \x02` \x01\x01Qa\x1F\xE8\x91\x90a6\xDAV[a\x1F\xF2\x91\x90a6\xF9V[a\x1F\xFC\x90\x86a7\x1BV[\x94P[\x80a \t\x81a51V[\x91PPa\x1F%V[PPP`\xFF\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x93PP`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a!\tW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"kV[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a!0`\x01\x84a5\xBEV[\x81T\x81\x10a!@Wa!@a4\xEAV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a!yW`\0\x93PPPPa\x15!V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a!\xB3W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"iV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\xBB\x82\x85a+\xCBV[\x96\x95PPPPPPV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\xE9\x90\x84a5\xBEV[\x81T\x81\x10a\"\xF9Wa\"\xF9a4\xEAV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a#(WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\x0C\xC5\x90PV[\x80T`\0\x90a#G\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a+\xE3V[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a#\x84W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua$6V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a%\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a%0\x83\x83a7FV[\x11\x15a%\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0[\x82\x81\x10\x15a(\xE4W`\0[a%\xB8\x82\x84a7FV[\x81\x10\x15a&\x99W\x84\x82\x81Q\x81\x10a%\xD1Wa%\xD1a4\xEAV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a&\x10Wa&\x10a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a&\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80a&\x91\x81a51V[\x91PPa%\xAEV[P`\0\x84\x82\x81Q\x81\x10a&\xAEWa&\xAEa4\xEAV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a'3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'YWa'Ya4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\xBEWa'\xBEa4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a(5Wa(5a4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a(\x92Wa(\x92a4\xEAV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a(\xB0Wa(\xB0a4\xEAV[` \x02` \x01\x01Q` \x01Q`@Qa(\xCA\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a(\xDC\x81a51V[\x91PPa%\xA3V[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a)\xB6WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x11V[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+\x06W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a*\xB9`\x01\x84a5\xBEV[\x81T\x81\x10a*\xC9Wa*\xC9a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a*\xF4Wa*\xEB`\x01\x82a5\xBEV[\x92PPPa\x15!V[\x80a*\xFE\x81a7^V[\x91PPa*\x84V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06\x11V[`\0a\x15!`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a7uV[`\0\x80\x82\x12\x15a,\x07Wa+\xF6\x82a7\xB4V[a,\0\x90\x84a7\xD1V[\x90Pa\x0C\xC5V[a,\0\x82\x84a7\x1BV[\x805`\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,9W`\0\x80\xFD[a\x15!\x82a,\x11V[`\0\x80`@\x83\x85\x03\x12\x15a,UW`\0\x80\xFD[a,^\x83a,\x11V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a,\xA3W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xB9W`\0\x80\xFD[a,\xC2\x83a,\x11V[\x91P` \x83\x015a,\xD2\x81a,\x8EV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,\xEFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x06W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a-9W`\0\x80\xFD[a-B\x86a,\x11V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-^W`\0\x80\xFD[a-j\x89\x83\x8A\x01a,\xDDV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a-\x83W`\0\x80\xFD[Pa-\x90\x88\x82\x89\x01a,\xDDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a-\xB3W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a-\xF8W`\0\x80\xFD[\x845a.\x03\x81a,\x8EV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.%W`\0\x80\xFD[a.1\x87\x82\x88\x01a-\xA1V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a.vW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.QV[P\x94\x95\x94PPPPPV[`@\x81R`\0a.\x94`@\x83\x01\x85a.=V[\x82\x81\x03` \x84\x01Ra$6\x81\x85a.=V[`\0\x80`@\x83\x85\x03\x12\x15a.\xB9W`\0\x80\xFD[\x825\x91Pa.\xC9` \x84\x01a,\x11V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>Wa/+\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.\xEEV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\x82Wa/\x82a/JV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\xB0Wa/\xB0a/JV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xD1Wa/\xD1a/JV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a/\xEEW`\0\x80\xFD[a/\xF7\x83a,\x11V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x13W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a0$W`\0\x80\xFD[\x805a07a02\x82a/\xB8V[a/\x88V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a0VW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a0tW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a0[V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xACW`\0\x80\xFD[a0\xB5\x84a0\x83V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[a0\xDC\x86\x82\x87\x01a-\xA1V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a1\x05V[`\0\x80`\0``\x84\x86\x03\x12\x15a1<W`\0\x80\xFD[a1E\x84a,\x11V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x0C\xC5V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xB9W`\0\x80\xFD[a1\xC2\x83a,\x11V[\x91Pa.\xC9` \x84\x01a1\x8FV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xE5W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a2\x13W`\0\x80\xFD[\x815` a2#a02\x83a/\xB8V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a2BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a2\x91W`@\x81\x89\x03\x12\x15a2_W`\0\x80\x81\xFD[a2ga/`V[\x815a2r\x81a,\x8EV[\x81Ra2\x7F\x82\x86\x01a1\x8FV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a2FV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xAFW`\0\x80\xFD[a2\xB8\x83a,\x11V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xD3W`\0\x80\xFD[a2\xDF\x85\x82\x86\x01a2\x02V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xFEW`\0\x80\xFD[a3\x07\x84a,\x11V[\x92Pa3\x15` \x85\x01a0\x83V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a3:W`\0\x80\xFD[\x835\x92Pa3J` \x85\x01a,\x11V[\x91Pa3X`@\x85\x01a0\x83V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3wW`\0\x80\xFD[a3\x80\x85a,\x11V[\x93Pa3\x8E` \x86\x01a0\x83V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xB8W`\0\x80\xFD[a3\xC1\x84a,\x11V[\x92Pa3\xCF` \x85\x01a1\x8FV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xEAW`\0\x80\xFD[a3\xF6\x86\x82\x87\x01a2\x02V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a4cW`\0\x80\xFD[\x81Qa\x15!\x81a,\x8EV[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\x12W`\0\x80\xFD[a\x15!\x82a1\x8FV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a5EWa5Ea5\x1BV[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a5\xD0Wa5\xD0a5\x1BV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0`@\x82\x01`\x01\x80`\xA0\x1B\x03\x80\x86\x16\x84R` `@\x81\x86\x01R\x82\x86T\x80\x85R``\x87\x01\x91P\x87`\0R\x82`\0 \x94P`\0[\x81\x81\x10\x15a6<W\x85T\x85\x16\x83R`\x01\x95\x86\x01\x95\x92\x84\x01\x92\x01a6\x1EV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a6]W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6sW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a6\x84W`\0\x80\xFD[\x80Qa6\x92a02\x82a/\xB8V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a6\xB1W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a6\xCFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a6\xB6V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a6\xF4Wa6\xF4a5\x1BV[P\x02\x90V[`\0\x82a7\x16WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a7=Wa7=a5\x1BV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15a7YWa7Ya5\x1BV[P\x01\x90V[`\0\x81a7mWa7ma5\x1BV[P`\0\x19\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a7\x93Wa7\x93a5\x1BV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a7\xAEWa7\xAEa5\x1BV[PP\x03\x90V[`\0`\x01`\xFF\x1B\x82\x14\x15a7\xCAWa7\xCAa5\x1BV[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a7\xF1Wa7\xF1a5\x1BV[\x03\x93\x92PPPV\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \xCC\xC9+s7lx\xCB\xEB\x80/\xDC\xA1\xC5\xBD~\xBEJ\x96\xF8\xCE4\x92V\x93\xF8\x123\xE2L\xCA\x98dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STAKEREGISTRY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80c\x9F<\xCFe\x11a\x01\x0FW\x80c\xC8)LV\x11a\0\xA2W\x80c\xF2\xBE\x94\xAE\x11a\0qW\x80c\xF2\xBE\x94\xAE\x14a\x05KW\x80c\xF8Q\xE1\x98\x14a\x05^W\x80c\xFA(\xC6'\x14a\x05qW\x80c\xFFiJw\x14a\x05\x84W`\0\x80\xFD[\x80c\xC8)LV\x14a\x04\xD6W\x80c\xD5\xEC\xCC\x05\x14a\x04\xE9W\x80c\xDD\x98F\xB9\x14a\x04\xFCW\x80c\xDF\\\xF7#\x14a\x05$W`\0\x80\xFD[\x80c\xBC\x9A@\xC3\x11a\0\xDEW\x80c\xBC\x9A@\xC3\x14a\x04tW\x80c\xBD)\xB8\xCD\x14a\x04\x87W\x80c\xC4gx\xA5\x14a\x04\x9AW\x80c\xC6\x01R}\x14a\x04\xC3W`\0\x80\xFD[\x80c\x9F<\xCFe\x14a\x03\xEEW\x80c\xACk\xFB\x03\x14a\x04\x01W\x80c\xAD\xC8\x04\xDA\x14a\x04!W\x80c\xB6\x90Kx\x14a\x04aW`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\x87W\x80cf\xAC\xFE\xFE\x11a\x01VW\x80cf\xAC\xFE\xFE\x14a\x03JW\x80cm\x14\xA9\x87\x14a\x03uW\x80c|\x17#G\x14a\x03\xB4W\x80c\x81\xC0u\x02\x14a\x03\xCEW`\0\x80\xFD[\x80cK\xD2n\t\x14a\x02\xE5W\x80cT\x01\xED'\x14a\x03\x15W\x80c^Zgu\x14a\x03(W\x80c_\x1F-w\x14a\x037W`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xC3W\x80c \xB6b\x98\x14a\x02lW\x80c%PGw\x14a\x02\x81W\x80c,\xD9Y@\x14a\x02\xA2W\x80c<\xA5\xA5\xF5\x14a\x02\xC2W`\0\x80\xFD[\x80c\x04\x91\xB4\x1C\x14a\x01\xEAW\x80c\x08s$a\x14a\x02 W\x80c\x1F\x9Bt\xE0\x14a\x02AW[`\0\x80\xFD[a\x02\ra\x01\xF86`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a,BV[a\x05\x97V[`@Qa\x02\x17\x92\x91\x90a,lV[a\x02Ta\x02O6`\x04a,\xA6V[a\x05\xE0V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x02\x7Fa\x02z6`\x04a-!V[a\x060V[\0[a\x02\x94a\x02\x8F6`\x04a-\xE2V[a\taV[`@Qa\x02\x17\x92\x91\x90a.\x81V[a\x02\xB5a\x02\xB06`\x04a.\xA6V[a\x0C,V[`@Qa\x02\x17\x91\x90a.\xD2V[a\x02\ra\x02\xD06`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02\ra\x02\xF36`\x04a.\xA6V[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02Ta\x03#6`\x04a.\xA6V[a\x0C\xCBV[a\x02\rg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x7Fa\x03E6`\x04a/\xDBV[a\x0C\xE4V[a\x03]a\x03X6`\x04a-\xE2V[a\x11\x04V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xBC` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xE1a\x03\xDC6`\x04a0\x97V[a\x12^V[`@Qa\x02\x17\x91\x90a0\xE9V[a\x03\x9Ca\x03\xFC6`\x04a,BV[a\x15(V[a\x04\x14a\x04\x0F6`\x04a1'V[a\x15`V[`@Qa\x02\x17\x91\x90a1ZV[a\x044a\x04/6`\x04a,BV[a\x15\xF8V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x17V[a\x04\x14a\x04o6`\x04a,BV[a\x16rV[a\x02\x7Fa\x04\x826`\x04a1\xA6V[a\x17\x01V[a\x02\x7Fa\x04\x956`\x04a1\xD0V[a\x17\xFAV[a\x02Ta\x04\xA86`\x04a,'V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\x7Fa\x04\xD16`\x04a2\x9CV[a\x19'V[a\x02Ta\x04\xE46`\x04a2\xE9V[a\x1A\x1BV[a\x02Ta\x04\xF76`\x04a,'V[a\x1A\x99V[a\x05\x0Fa\x05\n6`\x04a3%V[a\x1A\xECV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02Ta\x05Y6`\x04a3aV[a\x1B\x01V[a\x04\x14a\x05l6`\x04a.\xA6V[a\x1B\x96V[a\x02Ta\x05\x7F6`\x04a3%V[a\x1C{V[a\x02\x7Fa\x05\x926`\x04a3\xA3V[a\x1C\xDCV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x05\xB3W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06&\x85\x85a\x1EGV[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x84a\x06\xFE\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x83\x80a\x07\x90W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06\x11V[\x82\x81\x14a\x08\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\tVW\x85\x85\x82\x81\x81\x10a\x083Wa\x083a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x08H\x91\x90a5\0V[\x82\x89\x89\x84\x81\x81\x10a\x08[Wa\x08[a4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08rWa\x08ra4\xEAV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08\xDBWa\x08\xDBa4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08\xF2Wa\x08\xF2a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\x19Wa\t\x19a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\t.\x91\x90a5\0V[`@Qa\t<\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a\tN\x81a51V[\x91PPa\x08\x19V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xC6Wa\t\xC6a/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xEFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x0CWa\n\x0Ca/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\x1EW`\0\x87\x87\x83\x81\x81\x10a\nWWa\nWa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\n\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\n\xEC\x83\x8Da\x1EGV[\x91P\x91P\x80a\x0B\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0a\x0B\x96\x8C\x85\x85a EV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0B\xABWa\x0B\xABa4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0B\xD5\x84\x82a\"\xC5V[\x86\x86\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x16\x90a51V[\x91PPa\n;V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x0C\xBEW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0CeV[PPPP\x90P[\x92\x91PPV[`\0\x80a\x0C\xD8\x84\x84a\x1B\x96V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rf\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\r\xB2\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\r\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x81Q\x80a\x0ECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x84\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x10\xFBW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0E\xA2Wa\x0E\xA2a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0E\xBAWa\x0E\xBAa4\xEAV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x0F\x18Wa\x0F\x18a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F0Wa\x0F0a4\xEAV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\x0Fp\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x0F\x80Wa\x0F\x80a4\xEAV[\x90`\0R` `\0 \x01\x83\x87\x83\x81Q\x81\x10a\x0F\x9DWa\x0F\x9Da4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\xB5Wa\x0F\xB5a4\xEAV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x10\x08Wa\x10\x08a5\xD5V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x100\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x10@Wa\x10@a4\xEAV[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x10qWa\x10qa4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x89Wa\x10\x89a4\xEAV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x10\xC7Wa\x10\xC7a5\xD5V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U\x80a\x10\xF3\x81a51V[\x91PPa\x0EbV[PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x80[\x83\x81\x10\x15a\x06&W`\0\x85\x85\x83\x81\x81\x10a\x11nWa\x11na4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x11\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\x12\x0B\x83\x8Ba\x1EGV[\x91P\x91P\x80a\x12-W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x12:\x8A\x85\x85a EV[\x90Pa\x12F\x84\x82a\"\xC5V[PPPPP\x80\x80a\x12V\x90a51V[\x91PPa\x11RV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12zWa\x12za/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\x1DW`\0\x85\x85\x83\x81\x81\x10a\x12\xC5Wa\x12\xC5a4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x13dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum does not`d\x82\x01Re\x08\x19^\x1A\\\xDD`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x13\x8DWa\x13\x8Da4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x149W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x15\x07W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x14}\x84\x86a5\xBEV[a\x14\x87\x91\x90a5\xBEV[\x81T\x81\x10a\x14\x97Wa\x14\x97a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x14\xF5W`\x01a\x14\xBA\x82\x84a5\xBEV[a\x14\xC4\x91\x90a5\xBEV[\x85\x85\x81Q\x81\x10a\x14\xD6Wa\x14\xD6a4\xEAV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x15\x07V[\x80a\x14\xFF\x81a51V[\x91PPa\x14NV[PPP\x80\x80a\x15\x15\x90a51V[\x91PPa\x12\xA9V[P\x90P[\x93\x92PPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15DW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\xA5Wa\x15\xA5a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x160Wa\x160a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x16\xAFWa\x16\xAFa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x83\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x17\xCF\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x17\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$?V[PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0[\x81\x81\x10\x15a\x19!W`\0\x83\x83\x83\x81\x81\x10a\x18aWa\x18aa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x18\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0a\x18\xFF\x86\x83`\0a EV[\x90Pa\x19\x0B\x82\x82a\"\xC5V[PPP\x80\x80a\x19\x19\x90a51V[\x91PPa\x18EV[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA9\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x19\xF5\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x1A\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$\xA8V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1ABWa\x1ABa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x0C\xD8\x81\x85a(\xEBV[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1A\xBA\x91a5\xBEV[\x81T\x81\x10a\x1A\xCAWa\x1A\xCAa4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x1A\xF9\x84\x84\x84a*eV[\x94\x93PPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B2Wa\x1B2a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1B\x89\x81\x86a(\xEBV[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1B\xEFW\x91Pa\x0C\xC5\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1C\x16`\x01\x84a5\xBEV[\x81T\x81\x10a\x1C&Wa\x1C&a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x0C\xC5\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1C\xA2\x85\x85\x85a*eV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C\xB8Wa\x1C\xB8a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1D\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06\x11V[a\x1D\xAC\x83\x82a$\xA8V[a\x1D\xB6\x83\x83a$?V[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\0\x80`\0\x80a\x1Ef\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\xFF\x87\x16`\0\x90\x81R`\x04` \x81\x90R`@\x80\x83 \x90Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1E\xDB\x92\x8C\x92\x01a5\xEBV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F \x91\x90\x81\x01\x90a6JV[\x90P`\0[\x83\x81\x10\x15a \x11W`\xFF\x89\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1FQWa\x1FQa4\xEAV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1F\x9FWa\x1F\x9Fa4\xEAV[` \x02` \x01\x01Q\x11\x15a\x1F\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1F\xD6Wa\x1F\xD6a4\xEAV[` \x02` \x01\x01Qa\x1F\xE8\x91\x90a6\xDAV[a\x1F\xF2\x91\x90a6\xF9V[a\x1F\xFC\x90\x86a7\x1BV[\x94P[\x80a \t\x81a51V[\x91PPa\x1F%V[PPP`\xFF\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x93PP`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a!\tW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"kV[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a!0`\x01\x84a5\xBEV[\x81T\x81\x10a!@Wa!@a4\xEAV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a!yW`\0\x93PPPPa\x15!V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a!\xB3W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"iV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\xBB\x82\x85a+\xCBV[\x96\x95PPPPPPV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\xE9\x90\x84a5\xBEV[\x81T\x81\x10a\"\xF9Wa\"\xF9a4\xEAV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a#(WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\x0C\xC5\x90PV[\x80T`\0\x90a#G\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a+\xE3V[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a#\x84W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua$6V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a%\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a%0\x83\x83a7FV[\x11\x15a%\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0[\x82\x81\x10\x15a(\xE4W`\0[a%\xB8\x82\x84a7FV[\x81\x10\x15a&\x99W\x84\x82\x81Q\x81\x10a%\xD1Wa%\xD1a4\xEAV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a&\x10Wa&\x10a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a&\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80a&\x91\x81a51V[\x91PPa%\xAEV[P`\0\x84\x82\x81Q\x81\x10a&\xAEWa&\xAEa4\xEAV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a'3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'YWa'Ya4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\xBEWa'\xBEa4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a(5Wa(5a4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a(\x92Wa(\x92a4\xEAV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a(\xB0Wa(\xB0a4\xEAV[` \x02` \x01\x01Q` \x01Q`@Qa(\xCA\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a(\xDC\x81a51V[\x91PPa%\xA3V[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a)\xB6WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x11V[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+\x06W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a*\xB9`\x01\x84a5\xBEV[\x81T\x81\x10a*\xC9Wa*\xC9a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a*\xF4Wa*\xEB`\x01\x82a5\xBEV[\x92PPPa\x15!V[\x80a*\xFE\x81a7^V[\x91PPa*\x84V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06\x11V[`\0a\x15!`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a7uV[`\0\x80\x82\x12\x15a,\x07Wa+\xF6\x82a7\xB4V[a,\0\x90\x84a7\xD1V[\x90Pa\x0C\xC5V[a,\0\x82\x84a7\x1BV[\x805`\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,9W`\0\x80\xFD[a\x15!\x82a,\x11V[`\0\x80`@\x83\x85\x03\x12\x15a,UW`\0\x80\xFD[a,^\x83a,\x11V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a,\xA3W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xB9W`\0\x80\xFD[a,\xC2\x83a,\x11V[\x91P` \x83\x015a,\xD2\x81a,\x8EV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,\xEFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x06W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a-9W`\0\x80\xFD[a-B\x86a,\x11V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-^W`\0\x80\xFD[a-j\x89\x83\x8A\x01a,\xDDV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a-\x83W`\0\x80\xFD[Pa-\x90\x88\x82\x89\x01a,\xDDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a-\xB3W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a-\xF8W`\0\x80\xFD[\x845a.\x03\x81a,\x8EV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.%W`\0\x80\xFD[a.1\x87\x82\x88\x01a-\xA1V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a.vW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.QV[P\x94\x95\x94PPPPPV[`@\x81R`\0a.\x94`@\x83\x01\x85a.=V[\x82\x81\x03` \x84\x01Ra$6\x81\x85a.=V[`\0\x80`@\x83\x85\x03\x12\x15a.\xB9W`\0\x80\xFD[\x825\x91Pa.\xC9` \x84\x01a,\x11V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>Wa/+\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.\xEEV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\x82Wa/\x82a/JV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\xB0Wa/\xB0a/JV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xD1Wa/\xD1a/JV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a/\xEEW`\0\x80\xFD[a/\xF7\x83a,\x11V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x13W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a0$W`\0\x80\xFD[\x805a07a02\x82a/\xB8V[a/\x88V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a0VW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a0tW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a0[V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xACW`\0\x80\xFD[a0\xB5\x84a0\x83V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[a0\xDC\x86\x82\x87\x01a-\xA1V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a1\x05V[`\0\x80`\0``\x84\x86\x03\x12\x15a1<W`\0\x80\xFD[a1E\x84a,\x11V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x0C\xC5V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xB9W`\0\x80\xFD[a1\xC2\x83a,\x11V[\x91Pa.\xC9` \x84\x01a1\x8FV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xE5W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a2\x13W`\0\x80\xFD[\x815` a2#a02\x83a/\xB8V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a2BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a2\x91W`@\x81\x89\x03\x12\x15a2_W`\0\x80\x81\xFD[a2ga/`V[\x815a2r\x81a,\x8EV[\x81Ra2\x7F\x82\x86\x01a1\x8FV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a2FV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xAFW`\0\x80\xFD[a2\xB8\x83a,\x11V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xD3W`\0\x80\xFD[a2\xDF\x85\x82\x86\x01a2\x02V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xFEW`\0\x80\xFD[a3\x07\x84a,\x11V[\x92Pa3\x15` \x85\x01a0\x83V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a3:W`\0\x80\xFD[\x835\x92Pa3J` \x85\x01a,\x11V[\x91Pa3X`@\x85\x01a0\x83V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3wW`\0\x80\xFD[a3\x80\x85a,\x11V[\x93Pa3\x8E` \x86\x01a0\x83V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xB8W`\0\x80\xFD[a3\xC1\x84a,\x11V[\x92Pa3\xCF` \x85\x01a1\x8FV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xEAW`\0\x80\xFD[a3\xF6\x86\x82\x87\x01a2\x02V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a4cW`\0\x80\xFD[\x81Qa\x15!\x81a,\x8EV[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\x12W`\0\x80\xFD[a\x15!\x82a1\x8FV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a5EWa5Ea5\x1BV[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a5\xD0Wa5\xD0a5\x1BV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0`@\x82\x01`\x01\x80`\xA0\x1B\x03\x80\x86\x16\x84R` `@\x81\x86\x01R\x82\x86T\x80\x85R``\x87\x01\x91P\x87`\0R\x82`\0 \x94P`\0[\x81\x81\x10\x15a6<W\x85T\x85\x16\x83R`\x01\x95\x86\x01\x95\x92\x84\x01\x92\x01a6\x1EV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a6]W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6sW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a6\x84W`\0\x80\xFD[\x80Qa6\x92a02\x82a/\xB8V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a6\xB1W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a6\xCFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a6\xB6V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a6\xF4Wa6\xF4a5\x1BV[P\x02\x90V[`\0\x82a7\x16WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a7=Wa7=a5\x1BV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15a7YWa7Ya5\x1BV[P\x01\x90V[`\0\x81a7mWa7ma5\x1BV[P`\0\x19\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a7\x93Wa7\x93a5\x1BV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a7\xAEWa7\xAEa5\x1BV[PP\x03\x90V[`\0`\x01`\xFF\x1B\x82\x14\x15a7\xCAWa7\xCAa5\x1BV[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a7\xF1Wa7\xF1a5\x1BV[\x03\x93\x92PPPV\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \xCC\xC9+s7lx\xCB\xEB\x80/\xDC\xA1\xC5\xBD~\xBEJ\x96\xF8\xCE4\x92V\x93\xF8\x123\xE2L\xCA\x98dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STAKEREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct StakeRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                STAKEREGISTRY_ABI.clone(),
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
                STAKEREGISTRY_ABI.clone(),
                STAKEREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_WEIGHING_FUNCTION_LENGTH` (0x7c172347) function
        pub fn max_weighing_function_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([124, 23, 35, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WEIGHTING_DIVISOR` (0x5e5a6775) function
        pub fn weighting_divisor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 90, 103, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStrategies` (0xc601527d) function
        pub fn add_strategies(
            &self,
            quorum_number: u8,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 1, 82, 125], (quorum_number, strategy_params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperator` (0xbd29b8cd) function
        pub fn deregister_operator(
            &self,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 41, 184, 205], (operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentStake` (0x5401ed27) function
        pub fn get_current_stake(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([84, 1, 237, 39], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentTotalStake` (0xd5eccc05) function
        pub fn get_current_total_stake(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([213, 236, 204, 5], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestStakeUpdate` (0xf851e198) function
        pub fn get_latest_stake_update(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([248, 81, 225, 152], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAtBlockNumber` (0xfa28c627) function
        pub fn get_stake_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [250, 40, 198, 39],
                    (operator_id, quorum_number, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAtBlockNumberAndIndex` (0xf2be94ae) function
        pub fn get_stake_at_block_number_and_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [242, 190, 148, 174],
                    (quorum_number, block_number, operator_id, index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeHistory` (0x2cd95940) function
        pub fn get_stake_history(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<StakeUpdate>> {
            self.0
                .method_hash([44, 217, 89, 64], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeHistoryLength` (0x4bd26e09) function
        pub fn get_stake_history_length(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([75, 210, 110, 9], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateAtIndex` (0xac6bfb03) function
        pub fn get_stake_update_at_index(
            &self,
            quorum_number: u8,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([172, 107, 251, 3], (quorum_number, operator_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateIndexAtBlockNumber` (0xdd9846b9) function
        pub fn get_stake_update_index_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash(
                    [221, 152, 70, 185],
                    (operator_id, quorum_number, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeAtBlockNumberFromIndex` (0xc8294c56) function
        pub fn get_total_stake_at_block_number_from_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([200, 41, 76, 86], (quorum_number, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeHistoryLength` (0x0491b41c) function
        pub fn get_total_stake_history_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 145, 180, 28], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeIndicesAtBlockNumber` (0x81c07502) function
        pub fn get_total_stake_indices_at_block_number(
            &self,
            block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([129, 192, 117, 2], (block_number, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeUpdateAtIndex` (0xb6904b78) function
        pub fn get_total_stake_update_at_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([182, 144, 75, 120], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeQuorum` (0xff694a77) function
        pub fn initialize_quorum(
            &self,
            quorum_number: u8,
            minimum_stake: u128,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [255, 105, 74, 119],
                    (quorum_number, minimum_stake, strategy_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumStakeForQuorum` (0xc46778a5) function
        pub fn minimum_stake_for_quorum(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([196, 103, 120, 165], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyStrategyParams` (0x20b66298) function
        pub fn modify_strategy_params(
            &self,
            quorum_number: u8,
            strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
            new_multipliers: ::std::vec::Vec<u128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [32, 182, 98, 152],
                    (quorum_number, strategy_indices, new_multipliers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x25504777) function
        pub fn register_operator(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<u128>, ::std::vec::Vec<u128>),
        > {
            self.0
                .method_hash([37, 80, 71, 119], (operator, operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([109, 20, 169, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStrategies` (0x5f1f2d77) function
        pub fn remove_strategies(
            &self,
            quorum_number: u8,
            indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 31, 45, 119], (quorum_number, indices_to_remove))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinimumStakeForQuorum` (0xbc9a40c3) function
        pub fn set_minimum_stake_for_quorum(
            &self,
            quorum_number: u8,
            minimum_stake: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 154, 64, 195], (quorum_number, minimum_stake))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategiesPerQuorum` (0x9f3ccf65) function
        pub fn strategies_per_quorum(
            &self,
            p0: u8,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 60, 207, 101], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParams` (0x08732461) function
        pub fn strategy_params(
            &self,
            p0: u8,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::Address, u128)>
        {
            self.0
                .method_hash([8, 115, 36, 97], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParamsByIndex` (0xadc804da) function
        pub fn strategy_params_by_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StrategyParams> {
            self.0
                .method_hash([173, 200, 4, 218], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParamsLength` (0x3ca5a5f5) function
        pub fn strategy_params_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 165, 165, 245], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorStake` (0x66acfefe) function
        pub fn update_operator_stake(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [102, 172, 254, 254],
                    (operator, operator_id, quorum_numbers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weightOfOperatorForQuorum` (0x1f9b74e0) function
        pub fn weight_of_operator_for_quorum(
            &self,
            quorum_number: u8,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([31, 155, 116, 224], (quorum_number, operator))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MinimumStakeForQuorumUpdated` event
        pub fn minimum_stake_for_quorum_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinimumStakeForQuorumUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorStakeUpdate` event
        pub fn operator_stake_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorStakeUpdateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `QuorumCreated` event
        pub fn quorum_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, QuorumCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StrategyAddedToQuorum` event
        pub fn strategy_added_to_quorum_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StrategyAddedToQuorumFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `StrategyMultiplierUpdated` event
        pub fn strategy_multiplier_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyMultiplierUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyRemovedFromQuorum` event
        pub fn strategy_removed_from_quorum_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyRemovedFromQuorumFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeRegistryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for StakeRegistry<M>
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
    #[ethevent(
        name = "MinimumStakeForQuorumUpdated",
        abi = "MinimumStakeForQuorumUpdated(uint8,uint96)"
    )]
    pub struct MinimumStakeForQuorumUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub minimum_stake: u128,
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
        name = "OperatorStakeUpdate",
        abi = "OperatorStakeUpdate(bytes32,uint8,uint96)"
    )]
    pub struct OperatorStakeUpdateFilter {
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub stake: u128,
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
    #[ethevent(name = "QuorumCreated", abi = "QuorumCreated(uint8)")]
    pub struct QuorumCreatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
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
        name = "StrategyAddedToQuorum",
        abi = "StrategyAddedToQuorum(uint8,address)"
    )]
    pub struct StrategyAddedToQuorumFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
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
        Hash,
    )]
    #[ethevent(
        name = "StrategyMultiplierUpdated",
        abi = "StrategyMultiplierUpdated(uint8,address,uint256)"
    )]
    pub struct StrategyMultiplierUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: ::ethers::core::types::U256,
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
        name = "StrategyRemovedFromQuorum",
        abi = "StrategyRemovedFromQuorum(uint8,address)"
    )]
    pub struct StrategyRemovedFromQuorumFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StakeRegistryEvents {
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        OperatorStakeUpdateFilter(OperatorStakeUpdateFilter),
        QuorumCreatedFilter(QuorumCreatedFilter),
        StrategyAddedToQuorumFilter(StrategyAddedToQuorumFilter),
        StrategyMultiplierUpdatedFilter(StrategyMultiplierUpdatedFilter),
        StrategyRemovedFromQuorumFilter(StrategyRemovedFromQuorumFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::MinimumStakeForQuorumUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorStakeUpdateFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::OperatorStakeUpdateFilter(decoded));
            }
            if let Ok(decoded) = QuorumCreatedFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::QuorumCreatedFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToQuorumFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StrategyAddedToQuorumFilter(decoded));
            }
            if let Ok(decoded) = StrategyMultiplierUpdatedFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StrategyMultiplierUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategyRemovedFromQuorumFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StrategyRemovedFromQuorumFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinimumStakeForQuorumUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorStakeUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyAddedToQuorumFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyMultiplierUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyRemovedFromQuorumFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumUpdatedFilter> for StakeRegistryEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorStakeUpdateFilter> for StakeRegistryEvents {
        fn from(value: OperatorStakeUpdateFilter) -> Self {
            Self::OperatorStakeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<QuorumCreatedFilter> for StakeRegistryEvents {
        fn from(value: QuorumCreatedFilter) -> Self {
            Self::QuorumCreatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToQuorumFilter> for StakeRegistryEvents {
        fn from(value: StrategyAddedToQuorumFilter) -> Self {
            Self::StrategyAddedToQuorumFilter(value)
        }
    }
    impl ::core::convert::From<StrategyMultiplierUpdatedFilter> for StakeRegistryEvents {
        fn from(value: StrategyMultiplierUpdatedFilter) -> Self {
            Self::StrategyMultiplierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromQuorumFilter> for StakeRegistryEvents {
        fn from(value: StrategyRemovedFromQuorumFilter) -> Self {
            Self::StrategyRemovedFromQuorumFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_WEIGHING_FUNCTION_LENGTH` function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`
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
        name = "MAX_WEIGHING_FUNCTION_LENGTH",
        abi = "MAX_WEIGHING_FUNCTION_LENGTH()"
    )]
    pub struct MaxWeighingFunctionLengthCall;
    ///Container type for all input parameters for the `WEIGHTING_DIVISOR` function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`
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
    #[ethcall(name = "WEIGHTING_DIVISOR", abi = "WEIGHTING_DIVISOR()")]
    pub struct WeightingDivisorCall;
    ///Container type for all input parameters for the `addStrategies` function with signature `addStrategies(uint8,(address,uint96)[])` and selector `0xc601527d`
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
        name = "addStrategies",
        abi = "addStrategies(uint8,(address,uint96)[])"
    )]
    pub struct AddStrategiesCall {
        pub quorum_number: u8,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
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
        Hash,
    )]
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
    ///Container type for all input parameters for the `deregisterOperator` function with signature `deregisterOperator(bytes32,bytes)` and selector `0xbd29b8cd`
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
    #[ethcall(name = "deregisterOperator", abi = "deregisterOperator(bytes32,bytes)")]
    pub struct DeregisterOperatorCall {
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getCurrentStake` function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`
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
    #[ethcall(name = "getCurrentStake", abi = "getCurrentStake(bytes32,uint8)")]
    pub struct GetCurrentStakeCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getCurrentTotalStake` function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`
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
    #[ethcall(name = "getCurrentTotalStake", abi = "getCurrentTotalStake(uint8)")]
    pub struct GetCurrentTotalStakeCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getLatestStakeUpdate` function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`
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
        name = "getLatestStakeUpdate",
        abi = "getLatestStakeUpdate(bytes32,uint8)"
    )]
    pub struct GetLatestStakeUpdateCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeAtBlockNumber` function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`
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
        name = "getStakeAtBlockNumber",
        abi = "getStakeAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getStakeAtBlockNumberAndIndex` function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`
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
        name = "getStakeAtBlockNumberAndIndex",
        abi = "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)"
    )]
    pub struct GetStakeAtBlockNumberAndIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeHistory` function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`
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
    #[ethcall(name = "getStakeHistory", abi = "getStakeHistory(bytes32,uint8)")]
    pub struct GetStakeHistoryCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeHistoryLength` function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`
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
        name = "getStakeHistoryLength",
        abi = "getStakeHistoryLength(bytes32,uint8)"
    )]
    pub struct GetStakeHistoryLengthCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeUpdateAtIndex` function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`
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
        name = "getStakeUpdateAtIndex",
        abi = "getStakeUpdateAtIndex(uint8,bytes32,uint256)"
    )]
    pub struct GetStakeUpdateAtIndexCall {
        pub quorum_number: u8,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeUpdateIndexAtBlockNumber` function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`
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
        name = "getStakeUpdateIndexAtBlockNumber",
        abi = "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeUpdateIndexAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getTotalStakeAtBlockNumberFromIndex` function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`
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
        name = "getTotalStakeAtBlockNumberFromIndex",
        abi = "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)"
    )]
    pub struct GetTotalStakeAtBlockNumberFromIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalStakeHistoryLength` function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`
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
        name = "getTotalStakeHistoryLength",
        abi = "getTotalStakeHistoryLength(uint8)"
    )]
    pub struct GetTotalStakeHistoryLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getTotalStakeIndicesAtBlockNumber` function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`
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
        name = "getTotalStakeIndicesAtBlockNumber",
        abi = "getTotalStakeIndicesAtBlockNumber(uint32,bytes)"
    )]
    pub struct GetTotalStakeIndicesAtBlockNumberCall {
        pub block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getTotalStakeUpdateAtIndex` function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`
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
        name = "getTotalStakeUpdateAtIndex",
        abi = "getTotalStakeUpdateAtIndex(uint8,uint256)"
    )]
    pub struct GetTotalStakeUpdateAtIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initializeQuorum` function with signature `initializeQuorum(uint8,uint96,(address,uint96)[])` and selector `0xff694a77`
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
        name = "initializeQuorum",
        abi = "initializeQuorum(uint8,uint96,(address,uint96)[])"
    )]
    pub struct InitializeQuorumCall {
        pub quorum_number: u8,
        pub minimum_stake: u128,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
    }
    ///Container type for all input parameters for the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`
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
    #[ethcall(name = "minimumStakeForQuorum", abi = "minimumStakeForQuorum(uint8)")]
    pub struct MinimumStakeForQuorumCall(pub u8);
    ///Container type for all input parameters for the `modifyStrategyParams` function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`
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
        name = "modifyStrategyParams",
        abi = "modifyStrategyParams(uint8,uint256[],uint96[])"
    )]
    pub struct ModifyStrategyParamsCall {
        pub quorum_number: u8,
        pub strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub new_multipliers: ::std::vec::Vec<u128>,
    }
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`
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
        name = "registerOperator",
        abi = "registerOperator(address,bytes32,bytes)"
    )]
    pub struct RegisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `registryCoordinator` function with signature `registryCoordinator()` and selector `0x6d14a987`
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
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
    ///Container type for all input parameters for the `removeStrategies` function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`
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
    #[ethcall(name = "removeStrategies", abi = "removeStrategies(uint8,uint256[])")]
    pub struct RemoveStrategiesCall {
        pub quorum_number: u8,
        pub indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `setMinimumStakeForQuorum` function with signature `setMinimumStakeForQuorum(uint8,uint96)` and selector `0xbc9a40c3`
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
        name = "setMinimumStakeForQuorum",
        abi = "setMinimumStakeForQuorum(uint8,uint96)"
    )]
    pub struct SetMinimumStakeForQuorumCall {
        pub quorum_number: u8,
        pub minimum_stake: u128,
    }
    ///Container type for all input parameters for the `strategiesPerQuorum` function with signature `strategiesPerQuorum(uint8,uint256)` and selector `0x9f3ccf65`
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
        name = "strategiesPerQuorum",
        abi = "strategiesPerQuorum(uint8,uint256)"
    )]
    pub struct StrategiesPerQuorumCall(pub u8, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `strategyParams` function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`
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
    #[ethcall(name = "strategyParams", abi = "strategyParams(uint8,uint256)")]
    pub struct StrategyParamsCall(pub u8, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `strategyParamsByIndex` function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`
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
        name = "strategyParamsByIndex",
        abi = "strategyParamsByIndex(uint8,uint256)"
    )]
    pub struct StrategyParamsByIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategyParamsLength` function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`
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
    #[ethcall(name = "strategyParamsLength", abi = "strategyParamsLength(uint8)")]
    pub struct StrategyParamsLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`
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
        name = "updateOperatorStake",
        abi = "updateOperatorStake(address,bytes32,bytes)"
    )]
    pub struct UpdateOperatorStakeCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `weightOfOperatorForQuorum` function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`
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
        name = "weightOfOperatorForQuorum",
        abi = "weightOfOperatorForQuorum(uint8,address)"
    )]
    pub struct WeightOfOperatorForQuorumCall {
        pub quorum_number: u8,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StakeRegistryCalls {
        MaxWeighingFunctionLength(MaxWeighingFunctionLengthCall),
        WeightingDivisor(WeightingDivisorCall),
        AddStrategies(AddStrategiesCall),
        Delegation(DelegationCall),
        DeregisterOperator(DeregisterOperatorCall),
        GetCurrentStake(GetCurrentStakeCall),
        GetCurrentTotalStake(GetCurrentTotalStakeCall),
        GetLatestStakeUpdate(GetLatestStakeUpdateCall),
        GetStakeAtBlockNumber(GetStakeAtBlockNumberCall),
        GetStakeAtBlockNumberAndIndex(GetStakeAtBlockNumberAndIndexCall),
        GetStakeHistory(GetStakeHistoryCall),
        GetStakeHistoryLength(GetStakeHistoryLengthCall),
        GetStakeUpdateAtIndex(GetStakeUpdateAtIndexCall),
        GetStakeUpdateIndexAtBlockNumber(GetStakeUpdateIndexAtBlockNumberCall),
        GetTotalStakeAtBlockNumberFromIndex(GetTotalStakeAtBlockNumberFromIndexCall),
        GetTotalStakeHistoryLength(GetTotalStakeHistoryLengthCall),
        GetTotalStakeIndicesAtBlockNumber(GetTotalStakeIndicesAtBlockNumberCall),
        GetTotalStakeUpdateAtIndex(GetTotalStakeUpdateAtIndexCall),
        InitializeQuorum(InitializeQuorumCall),
        MinimumStakeForQuorum(MinimumStakeForQuorumCall),
        ModifyStrategyParams(ModifyStrategyParamsCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RemoveStrategies(RemoveStrategiesCall),
        SetMinimumStakeForQuorum(SetMinimumStakeForQuorumCall),
        StrategiesPerQuorum(StrategiesPerQuorumCall),
        StrategyParams(StrategyParamsCall),
        StrategyParamsByIndex(StrategyParamsByIndexCall),
        StrategyParamsLength(StrategyParamsLengthCall),
        UpdateOperatorStake(UpdateOperatorStakeCall),
        WeightOfOperatorForQuorum(WeightOfOperatorForQuorumCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <MaxWeighingFunctionLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxWeighingFunctionLength(decoded));
            }
            if let Ok(decoded) =
                <WeightingDivisorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WeightingDivisor(decoded));
            }
            if let Ok(decoded) = <AddStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddStrategies(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCurrentStake(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentTotalStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCurrentTotalStake(decoded));
            }
            if let Ok(decoded) =
                <GetLatestStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLatestStakeUpdate(decoded));
            }
            if let Ok(decoded) =
                <GetStakeAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeAtBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetStakeAtBlockNumberAndIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeAtBlockNumberAndIndex(decoded));
            }
            if let Ok(decoded) =
                <GetStakeHistoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeHistory(decoded));
            }
            if let Ok(decoded) =
                <GetStakeHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeHistoryLength(decoded));
            }
            if let Ok(decoded) =
                <GetStakeUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStakeUpdateAtIndex(decoded));
            }
            if let Ok(decoded) =
                <GetStakeUpdateIndexAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetStakeUpdateIndexAtBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetTotalStakeAtBlockNumberFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetTotalStakeAtBlockNumberFromIndex(decoded));
            }
            if let Ok(decoded) =
                <GetTotalStakeHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTotalStakeHistoryLength(decoded));
            }
            if let Ok(decoded) =
                <GetTotalStakeIndicesAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetTotalStakeIndicesAtBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetTotalStakeUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTotalStakeUpdateAtIndex(decoded));
            }
            if let Ok(decoded) =
                <InitializeQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitializeQuorum(decoded));
            }
            if let Ok(decoded) =
                <MinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) =
                <ModifyStrategyParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ModifyStrategyParams(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) =
                <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) =
                <RemoveStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveStrategies(decoded));
            }
            if let Ok(decoded) =
                <SetMinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetMinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) =
                <StrategiesPerQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategiesPerQuorum(decoded));
            }
            if let Ok(decoded) =
                <StrategyParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyParams(decoded));
            }
            if let Ok(decoded) =
                <StrategyParamsByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyParamsByIndex(decoded));
            }
            if let Ok(decoded) =
                <StrategyParamsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyParamsLength(decoded));
            }
            if let Ok(decoded) =
                <UpdateOperatorStakeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateOperatorStake(decoded));
            }
            if let Ok(decoded) =
                <WeightOfOperatorForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WeightOfOperatorForQuorum(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxWeighingFunctionLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightingDivisor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddStrategies(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentStake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentTotalStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAtBlockNumberAndIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeHistory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStakeHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateIndexAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeIndicesAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeQuorum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyStrategyParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStrategies(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategiesPerQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategyParamsByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParamsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightOfOperatorForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakeRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxWeighingFunctionLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeightingDivisor(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentTotalStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLatestStakeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAtBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeAtBlockNumberAndIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeHistoryLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeUpdateAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeUpdateIndexAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeHistoryLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalStakeIndicesAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeUpdateAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumStakeForQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyStrategyParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinimumStakeForQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategiesPerQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyParamsByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyParamsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperatorStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeightOfOperatorForQuorum(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxWeighingFunctionLengthCall> for StakeRegistryCalls {
        fn from(value: MaxWeighingFunctionLengthCall) -> Self {
            Self::MaxWeighingFunctionLength(value)
        }
    }
    impl ::core::convert::From<WeightingDivisorCall> for StakeRegistryCalls {
        fn from(value: WeightingDivisorCall) -> Self {
            Self::WeightingDivisor(value)
        }
    }
    impl ::core::convert::From<AddStrategiesCall> for StakeRegistryCalls {
        fn from(value: AddStrategiesCall) -> Self {
            Self::AddStrategies(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for StakeRegistryCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for StakeRegistryCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetCurrentStakeCall> for StakeRegistryCalls {
        fn from(value: GetCurrentStakeCall) -> Self {
            Self::GetCurrentStake(value)
        }
    }
    impl ::core::convert::From<GetCurrentTotalStakeCall> for StakeRegistryCalls {
        fn from(value: GetCurrentTotalStakeCall) -> Self {
            Self::GetCurrentTotalStake(value)
        }
    }
    impl ::core::convert::From<GetLatestStakeUpdateCall> for StakeRegistryCalls {
        fn from(value: GetLatestStakeUpdateCall) -> Self {
            Self::GetLatestStakeUpdate(value)
        }
    }
    impl ::core::convert::From<GetStakeAtBlockNumberCall> for StakeRegistryCalls {
        fn from(value: GetStakeAtBlockNumberCall) -> Self {
            Self::GetStakeAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetStakeAtBlockNumberAndIndexCall> for StakeRegistryCalls {
        fn from(value: GetStakeAtBlockNumberAndIndexCall) -> Self {
            Self::GetStakeAtBlockNumberAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeHistoryCall> for StakeRegistryCalls {
        fn from(value: GetStakeHistoryCall) -> Self {
            Self::GetStakeHistory(value)
        }
    }
    impl ::core::convert::From<GetStakeHistoryLengthCall> for StakeRegistryCalls {
        fn from(value: GetStakeHistoryLengthCall) -> Self {
            Self::GetStakeHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateAtIndexCall> for StakeRegistryCalls {
        fn from(value: GetStakeUpdateAtIndexCall) -> Self {
            Self::GetStakeUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateIndexAtBlockNumberCall> for StakeRegistryCalls {
        fn from(value: GetStakeUpdateIndexAtBlockNumberCall) -> Self {
            Self::GetStakeUpdateIndexAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeAtBlockNumberFromIndexCall> for StakeRegistryCalls {
        fn from(value: GetTotalStakeAtBlockNumberFromIndexCall) -> Self {
            Self::GetTotalStakeAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeHistoryLengthCall> for StakeRegistryCalls {
        fn from(value: GetTotalStakeHistoryLengthCall) -> Self {
            Self::GetTotalStakeHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeIndicesAtBlockNumberCall> for StakeRegistryCalls {
        fn from(value: GetTotalStakeIndicesAtBlockNumberCall) -> Self {
            Self::GetTotalStakeIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeUpdateAtIndexCall> for StakeRegistryCalls {
        fn from(value: GetTotalStakeUpdateAtIndexCall) -> Self {
            Self::GetTotalStakeUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<InitializeQuorumCall> for StakeRegistryCalls {
        fn from(value: InitializeQuorumCall) -> Self {
            Self::InitializeQuorum(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumCall> for StakeRegistryCalls {
        fn from(value: MinimumStakeForQuorumCall) -> Self {
            Self::MinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<ModifyStrategyParamsCall> for StakeRegistryCalls {
        fn from(value: ModifyStrategyParamsCall) -> Self {
            Self::ModifyStrategyParams(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for StakeRegistryCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for StakeRegistryCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesCall> for StakeRegistryCalls {
        fn from(value: RemoveStrategiesCall) -> Self {
            Self::RemoveStrategies(value)
        }
    }
    impl ::core::convert::From<SetMinimumStakeForQuorumCall> for StakeRegistryCalls {
        fn from(value: SetMinimumStakeForQuorumCall) -> Self {
            Self::SetMinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<StrategiesPerQuorumCall> for StakeRegistryCalls {
        fn from(value: StrategiesPerQuorumCall) -> Self {
            Self::StrategiesPerQuorum(value)
        }
    }
    impl ::core::convert::From<StrategyParamsCall> for StakeRegistryCalls {
        fn from(value: StrategyParamsCall) -> Self {
            Self::StrategyParams(value)
        }
    }
    impl ::core::convert::From<StrategyParamsByIndexCall> for StakeRegistryCalls {
        fn from(value: StrategyParamsByIndexCall) -> Self {
            Self::StrategyParamsByIndex(value)
        }
    }
    impl ::core::convert::From<StrategyParamsLengthCall> for StakeRegistryCalls {
        fn from(value: StrategyParamsLengthCall) -> Self {
            Self::StrategyParamsLength(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorStakeCall> for StakeRegistryCalls {
        fn from(value: UpdateOperatorStakeCall) -> Self {
            Self::UpdateOperatorStake(value)
        }
    }
    impl ::core::convert::From<WeightOfOperatorForQuorumCall> for StakeRegistryCalls {
        fn from(value: WeightOfOperatorForQuorumCall) -> Self {
            Self::WeightOfOperatorForQuorum(value)
        }
    }
    ///Container type for all return fields from the `MAX_WEIGHING_FUNCTION_LENGTH` function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`
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
    pub struct MaxWeighingFunctionLengthReturn(pub u8);
    ///Container type for all return fields from the `WEIGHTING_DIVISOR` function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`
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
    pub struct WeightingDivisorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getCurrentStake` function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`
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
    pub struct GetCurrentStakeReturn(pub u128);
    ///Container type for all return fields from the `getCurrentTotalStake` function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`
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
    pub struct GetCurrentTotalStakeReturn(pub u128);
    ///Container type for all return fields from the `getLatestStakeUpdate` function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`
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
    pub struct GetLatestStakeUpdateReturn(pub StakeUpdate);
    ///Container type for all return fields from the `getStakeAtBlockNumber` function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`
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
    pub struct GetStakeAtBlockNumberReturn(pub u128);
    ///Container type for all return fields from the `getStakeAtBlockNumberAndIndex` function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`
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
    pub struct GetStakeAtBlockNumberAndIndexReturn(pub u128);
    ///Container type for all return fields from the `getStakeHistory` function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`
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
    pub struct GetStakeHistoryReturn(pub ::std::vec::Vec<StakeUpdate>);
    ///Container type for all return fields from the `getStakeHistoryLength` function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`
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
    pub struct GetStakeHistoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStakeUpdateAtIndex` function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`
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
    pub struct GetStakeUpdateAtIndexReturn(pub StakeUpdate);
    ///Container type for all return fields from the `getStakeUpdateIndexAtBlockNumber` function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`
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
    pub struct GetStakeUpdateIndexAtBlockNumberReturn(pub u32);
    ///Container type for all return fields from the `getTotalStakeAtBlockNumberFromIndex` function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`
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
    pub struct GetTotalStakeAtBlockNumberFromIndexReturn(pub u128);
    ///Container type for all return fields from the `getTotalStakeHistoryLength` function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`
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
    pub struct GetTotalStakeHistoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalStakeIndicesAtBlockNumber` function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`
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
    pub struct GetTotalStakeIndicesAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getTotalStakeUpdateAtIndex` function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`
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
    pub struct GetTotalStakeUpdateAtIndexReturn(pub StakeUpdate);
    ///Container type for all return fields from the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`
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
    pub struct MinimumStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `registerOperator` function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`
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
    pub struct RegisterOperatorReturn(pub ::std::vec::Vec<u128>, pub ::std::vec::Vec<u128>);
    ///Container type for all return fields from the `registryCoordinator` function with signature `registryCoordinator()` and selector `0x6d14a987`
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
    pub struct RegistryCoordinatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `strategiesPerQuorum` function with signature `strategiesPerQuorum(uint8,uint256)` and selector `0x9f3ccf65`
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
    pub struct StrategiesPerQuorumReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `strategyParams` function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`
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
    pub struct StrategyParamsReturn {
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: u128,
    }
    ///Container type for all return fields from the `strategyParamsByIndex` function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`
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
    pub struct StrategyParamsByIndexReturn(pub StrategyParams);
    ///Container type for all return fields from the `strategyParamsLength` function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`
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
    pub struct StrategyParamsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`
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
    pub struct UpdateOperatorStakeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weightOfOperatorForQuorum` function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`
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
    pub struct WeightOfOperatorForQuorumReturn(pub u128);
    ///`StakeUpdate(uint32,uint32,uint96)`
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
    pub struct StakeUpdate {
        pub update_block_number: u32,
        pub next_update_block_number: u32,
        pub stake: u128,
    }
    ///`StrategyParams(address,uint96)`
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
    pub struct StrategyParams {
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: u128,
    }
}
