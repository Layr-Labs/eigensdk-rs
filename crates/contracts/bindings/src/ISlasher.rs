pub use i_slasher::*;
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
pub mod i_slasher {
    const _: () = {
        ::core::include_bytes!(
            "/Users/supernovahs/Desktop/eigensdk-rs/crates/contracts/utils/json/ISlasher.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("canSlash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("canSlash"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("toBeSlashed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slashingContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("canWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("canWithdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalStartBlock",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("middlewareTimesIndex",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("contractCanSlashOperatorUntilBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "contractCanSlashOperatorUntilBlock",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serviceContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("freezeOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("freezeOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("toBeFrozen"),
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
                    ::std::borrow::ToOwned::to_owned("getCorrectValueForInsertAfter"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCorrectValueForInsertAfter",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("getMiddlewareTimesIndexServeUntilBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getMiddlewareTimesIndexServeUntilBlock",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getMiddlewareTimesIndexStalestUpdateBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getMiddlewareTimesIndexStalestUpdateBlock",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("isFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isFrozen"),
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
                    ::std::borrow::ToOwned::to_owned("latestUpdateBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestUpdateBlock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serviceContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("middlewareTimesLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("middlewareTimesLength",),
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
                    ::std::borrow::ToOwned::to_owned("operatorToMiddlewareTimes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorToMiddlewareTimes",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("arrayIndex"),
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
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISlasher.MiddlewareTimes",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorWhitelistedContractsLinkedListEntry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "operatorWhitelistedContractsLinkedListEntry",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("node"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorWhitelistedContractsLinkedListSize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "operatorWhitelistedContractsLinkedListSize",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("optIntoSlashing"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("optIntoSlashing"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("contractAddress"),
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
                    ::std::borrow::ToOwned::to_owned("recordFirstStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("recordFirstStakeUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serveUntilBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "recordLastStakeUpdateAndRevokeSlashingAbility",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "recordLastStakeUpdateAndRevokeSlashingAbility",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serveUntilBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("recordStakeUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serveUntilBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("insertAfter"),
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
                    ::std::borrow::ToOwned::to_owned("resetFrozenStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("resetFrozenStatus"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("frozenAddresses"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FrozenStatusReset"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FrozenStatusReset"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("previouslySlashedAddress",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MiddlewareTimesAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MiddlewareTimesAdded",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stalestUpdateBlock",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("latestServeUntilBlock",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorFrozen"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("slashedOperator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("slashingContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptedIntoSlashing"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OptedIntoSlashing"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SlashingAbilityRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SlashingAbilityRevoked",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "contractCanSlashOperatorUntilBlock",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
    pub static ISLASHER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct ISlasher<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ISlasher<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ISlasher<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ISlasher<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ISlasher<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ISlasher))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ISlasher<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISLASHER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `canSlash` (0xd98128c0) function
        pub fn can_slash(
            &self,
            to_be_slashed: ::ethers::core::types::Address,
            slashing_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 129, 40, 192], (to_be_slashed, slashing_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `canWithdraw` (0x8105e043) function
        pub fn can_withdraw(
            &self,
            operator: ::ethers::core::types::Address,
            withdrawal_start_block: u32,
            middleware_times_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [129, 5, 224, 67],
                    (operator, withdrawal_start_block, middleware_times_index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractCanSlashOperatorUntilBlock` (0x6f0c2f74) function
        pub fn contract_can_slash_operator_until_block(
            &self,
            operator: ::ethers::core::types::Address,
            service_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 12, 47, 116], (operator, service_contract))
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
        ///Calls the contract's `freezeOperator` (0x38c8ee64) function
        pub fn freeze_operator(
            &self,
            to_be_frozen: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 200, 238, 100], to_be_frozen)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCorrectValueForInsertAfter` (0x723e59c7) function
        pub fn get_correct_value_for_insert_after(
            &self,
            operator: ::ethers::core::types::Address,
            update_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 62, 89, 199], (operator, update_block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMiddlewareTimesIndexServeUntilBlock` (0x7259a45c) function
        pub fn get_middleware_times_index_serve_until_block(
            &self,
            operator: ::ethers::core::types::Address,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([114, 89, 164, 92], (operator, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMiddlewareTimesIndexStalestUpdateBlock` (0x1874e5ae) function
        pub fn get_middleware_times_index_stalest_update_block(
            &self,
            operator: ::ethers::core::types::Address,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([24, 116, 229, 174], (operator, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isFrozen` (0xe5839836) function
        pub fn is_frozen(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 131, 152, 54], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestUpdateBlock` (0xda16e29b) function
        pub fn latest_update_block(
            &self,
            operator: ::ethers::core::types::Address,
            service_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([218, 22, 226, 155], (operator, service_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `middlewareTimesLength` (0xa49db732) function
        pub fn middleware_times_length(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 157, 183, 50], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorToMiddlewareTimes` (0x282670fc) function
        pub fn operator_to_middleware_times(
            &self,
            operator: ::ethers::core::types::Address,
            array_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, MiddlewareTimes> {
            self.0
                .method_hash([40, 38, 112, 252], (operator, array_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorWhitelistedContractsLinkedListEntry` (0x855fcc4a) function
        pub fn operator_whitelisted_contracts_linked_list_entry(
            &self,
            operator: ::ethers::core::types::Address,
            node: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([133, 95, 204, 74], (operator, node))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorWhitelistedContractsLinkedListSize` (0xe921d4fa) function
        pub fn operator_whitelisted_contracts_linked_list_size(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 33, 212, 250], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `optIntoSlashing` (0xf73b7519) function
        pub fn opt_into_slashing(
            &self,
            contract_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 59, 117, 25], contract_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordFirstStakeUpdate` (0x175d3205) function
        pub fn record_first_stake_update(
            &self,
            operator: ::ethers::core::types::Address,
            serve_until_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 93, 50, 5], (operator, serve_until_block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordLastStakeUpdateAndRevokeSlashingAbility` (0x0ffabbce) function
        pub fn record_last_stake_update_and_revoke_slashing_ability(
            &self,
            operator: ::ethers::core::types::Address,
            serve_until_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 250, 187, 206], (operator, serve_until_block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordStakeUpdate` (0xc747075b) function
        pub fn record_stake_update(
            &self,
            operator: ::ethers::core::types::Address,
            update_block: u32,
            serve_until_block: u32,
            insert_after: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [199, 71, 7, 91],
                    (operator, update_block, serve_until_block, insert_after),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetFrozenStatus` (0x7cf72bba) function
        pub fn reset_frozen_status(
            &self,
            frozen_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 247, 43, 186], frozen_addresses)
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
        ///Gets the contract's `FrozenStatusReset` event
        pub fn frozen_status_reset_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FrozenStatusResetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MiddlewareTimesAdded` event
        pub fn middleware_times_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MiddlewareTimesAddedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorFrozen` event
        pub fn operator_frozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorFrozenFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OptedIntoSlashing` event
        pub fn opted_into_slashing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OptedIntoSlashingFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SlashingAbilityRevoked` event
        pub fn slashing_ability_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SlashingAbilityRevokedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ISlasherEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ISlasher<M> {
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
    #[ethevent(name = "FrozenStatusReset", abi = "FrozenStatusReset(address)")]
    pub struct FrozenStatusResetFilter {
        #[ethevent(indexed)]
        pub previously_slashed_address: ::ethers::core::types::Address,
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
        name = "MiddlewareTimesAdded",
        abi = "MiddlewareTimesAdded(address,uint256,uint32,uint32)"
    )]
    pub struct MiddlewareTimesAddedFilter {
        pub operator: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
        pub stalest_update_block: u32,
        pub latest_serve_until_block: u32,
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
    #[ethevent(name = "OperatorFrozen", abi = "OperatorFrozen(address,address)")]
    pub struct OperatorFrozenFilter {
        #[ethevent(indexed)]
        pub slashed_operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub slashing_contract: ::ethers::core::types::Address,
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
    #[ethevent(name = "OptedIntoSlashing", abi = "OptedIntoSlashing(address,address)")]
    pub struct OptedIntoSlashingFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
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
        name = "SlashingAbilityRevoked",
        abi = "SlashingAbilityRevoked(address,address,uint32)"
    )]
    pub struct SlashingAbilityRevokedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
        pub contract_can_slash_operator_until_block: u32,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISlasherEvents {
        FrozenStatusResetFilter(FrozenStatusResetFilter),
        MiddlewareTimesAddedFilter(MiddlewareTimesAddedFilter),
        OperatorFrozenFilter(OperatorFrozenFilter),
        OptedIntoSlashingFilter(OptedIntoSlashingFilter),
        SlashingAbilityRevokedFilter(SlashingAbilityRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ISlasherEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FrozenStatusResetFilter::decode_log(log) {
                return Ok(ISlasherEvents::FrozenStatusResetFilter(decoded));
            }
            if let Ok(decoded) = MiddlewareTimesAddedFilter::decode_log(log) {
                return Ok(ISlasherEvents::MiddlewareTimesAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorFrozenFilter::decode_log(log) {
                return Ok(ISlasherEvents::OperatorFrozenFilter(decoded));
            }
            if let Ok(decoded) = OptedIntoSlashingFilter::decode_log(log) {
                return Ok(ISlasherEvents::OptedIntoSlashingFilter(decoded));
            }
            if let Ok(decoded) = SlashingAbilityRevokedFilter::decode_log(log) {
                return Ok(ISlasherEvents::SlashingAbilityRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ISlasherEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FrozenStatusResetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MiddlewareTimesAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorFrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OptedIntoSlashingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashingAbilityRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FrozenStatusResetFilter> for ISlasherEvents {
        fn from(value: FrozenStatusResetFilter) -> Self {
            Self::FrozenStatusResetFilter(value)
        }
    }
    impl ::core::convert::From<MiddlewareTimesAddedFilter> for ISlasherEvents {
        fn from(value: MiddlewareTimesAddedFilter) -> Self {
            Self::MiddlewareTimesAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorFrozenFilter> for ISlasherEvents {
        fn from(value: OperatorFrozenFilter) -> Self {
            Self::OperatorFrozenFilter(value)
        }
    }
    impl ::core::convert::From<OptedIntoSlashingFilter> for ISlasherEvents {
        fn from(value: OptedIntoSlashingFilter) -> Self {
            Self::OptedIntoSlashingFilter(value)
        }
    }
    impl ::core::convert::From<SlashingAbilityRevokedFilter> for ISlasherEvents {
        fn from(value: SlashingAbilityRevokedFilter) -> Self {
            Self::SlashingAbilityRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `canSlash` function with signature `canSlash(address,address)` and selector `0xd98128c0`
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
    #[ethcall(name = "canSlash", abi = "canSlash(address,address)")]
    pub struct CanSlashCall {
        pub to_be_slashed: ::ethers::core::types::Address,
        pub slashing_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `canWithdraw` function with signature `canWithdraw(address,uint32,uint256)` and selector `0x8105e043`
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
    #[ethcall(name = "canWithdraw", abi = "canWithdraw(address,uint32,uint256)")]
    pub struct CanWithdrawCall {
        pub operator: ::ethers::core::types::Address,
        pub withdrawal_start_block: u32,
        pub middleware_times_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `contractCanSlashOperatorUntilBlock` function with signature `contractCanSlashOperatorUntilBlock(address,address)` and selector `0x6f0c2f74`
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
        name = "contractCanSlashOperatorUntilBlock",
        abi = "contractCanSlashOperatorUntilBlock(address,address)"
    )]
    pub struct ContractCanSlashOperatorUntilBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub service_contract: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `freezeOperator` function with signature `freezeOperator(address)` and selector `0x38c8ee64`
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
    #[ethcall(name = "freezeOperator", abi = "freezeOperator(address)")]
    pub struct FreezeOperatorCall {
        pub to_be_frozen: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getCorrectValueForInsertAfter` function with signature `getCorrectValueForInsertAfter(address,uint32)` and selector `0x723e59c7`
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
        name = "getCorrectValueForInsertAfter",
        abi = "getCorrectValueForInsertAfter(address,uint32)"
    )]
    pub struct GetCorrectValueForInsertAfterCall {
        pub operator: ::ethers::core::types::Address,
        pub update_block: u32,
    }
    ///Container type for all input parameters for the `getMiddlewareTimesIndexServeUntilBlock` function with signature `getMiddlewareTimesIndexServeUntilBlock(address,uint32)` and selector `0x7259a45c`
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
        name = "getMiddlewareTimesIndexServeUntilBlock",
        abi = "getMiddlewareTimesIndexServeUntilBlock(address,uint32)"
    )]
    pub struct GetMiddlewareTimesIndexServeUntilBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub index: u32,
    }
    ///Container type for all input parameters for the `getMiddlewareTimesIndexStalestUpdateBlock` function with signature `getMiddlewareTimesIndexStalestUpdateBlock(address,uint32)` and selector `0x1874e5ae`
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
        name = "getMiddlewareTimesIndexStalestUpdateBlock",
        abi = "getMiddlewareTimesIndexStalestUpdateBlock(address,uint32)"
    )]
    pub struct GetMiddlewareTimesIndexStalestUpdateBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub index: u32,
    }
    ///Container type for all input parameters for the `isFrozen` function with signature `isFrozen(address)` and selector `0xe5839836`
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
    #[ethcall(name = "isFrozen", abi = "isFrozen(address)")]
    pub struct IsFrozenCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `latestUpdateBlock` function with signature `latestUpdateBlock(address,address)` and selector `0xda16e29b`
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
    #[ethcall(name = "latestUpdateBlock", abi = "latestUpdateBlock(address,address)")]
    pub struct LatestUpdateBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub service_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `middlewareTimesLength` function with signature `middlewareTimesLength(address)` and selector `0xa49db732`
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
    #[ethcall(name = "middlewareTimesLength", abi = "middlewareTimesLength(address)")]
    pub struct MiddlewareTimesLengthCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorToMiddlewareTimes` function with signature `operatorToMiddlewareTimes(address,uint256)` and selector `0x282670fc`
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
        name = "operatorToMiddlewareTimes",
        abi = "operatorToMiddlewareTimes(address,uint256)"
    )]
    pub struct OperatorToMiddlewareTimesCall {
        pub operator: ::ethers::core::types::Address,
        pub array_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operatorWhitelistedContractsLinkedListEntry` function with signature `operatorWhitelistedContractsLinkedListEntry(address,address)` and selector `0x855fcc4a`
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
        name = "operatorWhitelistedContractsLinkedListEntry",
        abi = "operatorWhitelistedContractsLinkedListEntry(address,address)"
    )]
    pub struct OperatorWhitelistedContractsLinkedListEntryCall {
        pub operator: ::ethers::core::types::Address,
        pub node: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorWhitelistedContractsLinkedListSize` function with signature `operatorWhitelistedContractsLinkedListSize(address)` and selector `0xe921d4fa`
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
        name = "operatorWhitelistedContractsLinkedListSize",
        abi = "operatorWhitelistedContractsLinkedListSize(address)"
    )]
    pub struct OperatorWhitelistedContractsLinkedListSizeCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `optIntoSlashing` function with signature `optIntoSlashing(address)` and selector `0xf73b7519`
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
    #[ethcall(name = "optIntoSlashing", abi = "optIntoSlashing(address)")]
    pub struct OptIntoSlashingCall {
        pub contract_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `recordFirstStakeUpdate` function with signature `recordFirstStakeUpdate(address,uint32)` and selector `0x175d3205`
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
        name = "recordFirstStakeUpdate",
        abi = "recordFirstStakeUpdate(address,uint32)"
    )]
    pub struct RecordFirstStakeUpdateCall {
        pub operator: ::ethers::core::types::Address,
        pub serve_until_block: u32,
    }
    ///Container type for all input parameters for the `recordLastStakeUpdateAndRevokeSlashingAbility` function with signature `recordLastStakeUpdateAndRevokeSlashingAbility(address,uint32)` and selector `0x0ffabbce`
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
        name = "recordLastStakeUpdateAndRevokeSlashingAbility",
        abi = "recordLastStakeUpdateAndRevokeSlashingAbility(address,uint32)"
    )]
    pub struct RecordLastStakeUpdateAndRevokeSlashingAbilityCall {
        pub operator: ::ethers::core::types::Address,
        pub serve_until_block: u32,
    }
    ///Container type for all input parameters for the `recordStakeUpdate` function with signature `recordStakeUpdate(address,uint32,uint32,uint256)` and selector `0xc747075b`
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
        name = "recordStakeUpdate",
        abi = "recordStakeUpdate(address,uint32,uint32,uint256)"
    )]
    pub struct RecordStakeUpdateCall {
        pub operator: ::ethers::core::types::Address,
        pub update_block: u32,
        pub serve_until_block: u32,
        pub insert_after: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `resetFrozenStatus` function with signature `resetFrozenStatus(address[])` and selector `0x7cf72bba`
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
    #[ethcall(name = "resetFrozenStatus", abi = "resetFrozenStatus(address[])")]
    pub struct ResetFrozenStatusCall {
        pub frozen_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
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
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ISlasherCalls {
        CanSlash(CanSlashCall),
        CanWithdraw(CanWithdrawCall),
        ContractCanSlashOperatorUntilBlock(ContractCanSlashOperatorUntilBlockCall),
        Delegation(DelegationCall),
        FreezeOperator(FreezeOperatorCall),
        GetCorrectValueForInsertAfter(GetCorrectValueForInsertAfterCall),
        GetMiddlewareTimesIndexServeUntilBlock(GetMiddlewareTimesIndexServeUntilBlockCall),
        GetMiddlewareTimesIndexStalestUpdateBlock(GetMiddlewareTimesIndexStalestUpdateBlockCall),
        IsFrozen(IsFrozenCall),
        LatestUpdateBlock(LatestUpdateBlockCall),
        MiddlewareTimesLength(MiddlewareTimesLengthCall),
        OperatorToMiddlewareTimes(OperatorToMiddlewareTimesCall),
        OperatorWhitelistedContractsLinkedListEntry(
            OperatorWhitelistedContractsLinkedListEntryCall,
        ),
        OperatorWhitelistedContractsLinkedListSize(OperatorWhitelistedContractsLinkedListSizeCall),
        OptIntoSlashing(OptIntoSlashingCall),
        RecordFirstStakeUpdate(RecordFirstStakeUpdateCall),
        RecordLastStakeUpdateAndRevokeSlashingAbility(
            RecordLastStakeUpdateAndRevokeSlashingAbilityCall,
        ),
        RecordStakeUpdate(RecordStakeUpdateCall),
        ResetFrozenStatus(ResetFrozenStatusCall),
        StrategyManager(StrategyManagerCall),
    }
    impl ::ethers::core::abi::AbiDecode for ISlasherCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CanSlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CanSlash(decoded));
            }
            if let Ok(decoded) = <CanWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CanWithdraw(decoded));
            }
            if let Ok(decoded) =
                <ContractCanSlashOperatorUntilBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ContractCanSlashOperatorUntilBlock(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) =
                <FreezeOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FreezeOperator(decoded));
            }
            if let Ok(decoded) =
                <GetCorrectValueForInsertAfterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCorrectValueForInsertAfter(decoded));
            }
            if let Ok(decoded) = <GetMiddlewareTimesIndexServeUntilBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMiddlewareTimesIndexServeUntilBlock(decoded));
            }
            if let Ok(decoded) = <GetMiddlewareTimesIndexStalestUpdateBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMiddlewareTimesIndexStalestUpdateBlock(decoded));
            }
            if let Ok(decoded) = <IsFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsFrozen(decoded));
            }
            if let Ok(decoded) =
                <LatestUpdateBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestUpdateBlock(decoded));
            }
            if let Ok(decoded) =
                <MiddlewareTimesLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MiddlewareTimesLength(decoded));
            }
            if let Ok(decoded) =
                <OperatorToMiddlewareTimesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorToMiddlewareTimes(decoded));
            }
            if let Ok(decoded) = <OperatorWhitelistedContractsLinkedListEntryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorWhitelistedContractsLinkedListEntry(decoded));
            }
            if let Ok(decoded) = <OperatorWhitelistedContractsLinkedListSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorWhitelistedContractsLinkedListSize(decoded));
            }
            if let Ok(decoded) =
                <OptIntoSlashingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OptIntoSlashing(decoded));
            }
            if let Ok(decoded) =
                <RecordFirstStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RecordFirstStakeUpdate(decoded));
            }
            if let Ok(decoded) = <RecordLastStakeUpdateAndRevokeSlashingAbilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordLastStakeUpdateAndRevokeSlashingAbility(decoded));
            }
            if let Ok(decoded) =
                <RecordStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RecordStakeUpdate(decoded));
            }
            if let Ok(decoded) =
                <ResetFrozenStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ResetFrozenStatus(decoded));
            }
            if let Ok(decoded) =
                <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyManager(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ISlasherCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CanSlash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CanWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractCanSlashOperatorUntilBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FreezeOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCorrectValueForInsertAfter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMiddlewareTimesIndexServeUntilBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMiddlewareTimesIndexStalestUpdateBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsFrozen(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestUpdateBlock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MiddlewareTimesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorToMiddlewareTimes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorWhitelistedContractsLinkedListEntry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorWhitelistedContractsLinkedListSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptIntoSlashing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordFirstStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordLastStakeUpdateAndRevokeSlashingAbility(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordStakeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResetFrozenStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategyManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ISlasherCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CanSlash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CanWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractCanSlashOperatorUntilBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCorrectValueForInsertAfter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMiddlewareTimesIndexServeUntilBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMiddlewareTimesIndexStalestUpdateBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestUpdateBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::MiddlewareTimesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorToMiddlewareTimes(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorWhitelistedContractsLinkedListEntry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorWhitelistedContractsLinkedListSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptIntoSlashing(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordFirstStakeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordLastStakeUpdateAndRevokeSlashingAbility(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecordStakeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetFrozenStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CanSlashCall> for ISlasherCalls {
        fn from(value: CanSlashCall) -> Self {
            Self::CanSlash(value)
        }
    }
    impl ::core::convert::From<CanWithdrawCall> for ISlasherCalls {
        fn from(value: CanWithdrawCall) -> Self {
            Self::CanWithdraw(value)
        }
    }
    impl ::core::convert::From<ContractCanSlashOperatorUntilBlockCall> for ISlasherCalls {
        fn from(value: ContractCanSlashOperatorUntilBlockCall) -> Self {
            Self::ContractCanSlashOperatorUntilBlock(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for ISlasherCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<FreezeOperatorCall> for ISlasherCalls {
        fn from(value: FreezeOperatorCall) -> Self {
            Self::FreezeOperator(value)
        }
    }
    impl ::core::convert::From<GetCorrectValueForInsertAfterCall> for ISlasherCalls {
        fn from(value: GetCorrectValueForInsertAfterCall) -> Self {
            Self::GetCorrectValueForInsertAfter(value)
        }
    }
    impl ::core::convert::From<GetMiddlewareTimesIndexServeUntilBlockCall> for ISlasherCalls {
        fn from(value: GetMiddlewareTimesIndexServeUntilBlockCall) -> Self {
            Self::GetMiddlewareTimesIndexServeUntilBlock(value)
        }
    }
    impl ::core::convert::From<GetMiddlewareTimesIndexStalestUpdateBlockCall> for ISlasherCalls {
        fn from(value: GetMiddlewareTimesIndexStalestUpdateBlockCall) -> Self {
            Self::GetMiddlewareTimesIndexStalestUpdateBlock(value)
        }
    }
    impl ::core::convert::From<IsFrozenCall> for ISlasherCalls {
        fn from(value: IsFrozenCall) -> Self {
            Self::IsFrozen(value)
        }
    }
    impl ::core::convert::From<LatestUpdateBlockCall> for ISlasherCalls {
        fn from(value: LatestUpdateBlockCall) -> Self {
            Self::LatestUpdateBlock(value)
        }
    }
    impl ::core::convert::From<MiddlewareTimesLengthCall> for ISlasherCalls {
        fn from(value: MiddlewareTimesLengthCall) -> Self {
            Self::MiddlewareTimesLength(value)
        }
    }
    impl ::core::convert::From<OperatorToMiddlewareTimesCall> for ISlasherCalls {
        fn from(value: OperatorToMiddlewareTimesCall) -> Self {
            Self::OperatorToMiddlewareTimes(value)
        }
    }
    impl ::core::convert::From<OperatorWhitelistedContractsLinkedListEntryCall> for ISlasherCalls {
        fn from(value: OperatorWhitelistedContractsLinkedListEntryCall) -> Self {
            Self::OperatorWhitelistedContractsLinkedListEntry(value)
        }
    }
    impl ::core::convert::From<OperatorWhitelistedContractsLinkedListSizeCall> for ISlasherCalls {
        fn from(value: OperatorWhitelistedContractsLinkedListSizeCall) -> Self {
            Self::OperatorWhitelistedContractsLinkedListSize(value)
        }
    }
    impl ::core::convert::From<OptIntoSlashingCall> for ISlasherCalls {
        fn from(value: OptIntoSlashingCall) -> Self {
            Self::OptIntoSlashing(value)
        }
    }
    impl ::core::convert::From<RecordFirstStakeUpdateCall> for ISlasherCalls {
        fn from(value: RecordFirstStakeUpdateCall) -> Self {
            Self::RecordFirstStakeUpdate(value)
        }
    }
    impl ::core::convert::From<RecordLastStakeUpdateAndRevokeSlashingAbilityCall> for ISlasherCalls {
        fn from(value: RecordLastStakeUpdateAndRevokeSlashingAbilityCall) -> Self {
            Self::RecordLastStakeUpdateAndRevokeSlashingAbility(value)
        }
    }
    impl ::core::convert::From<RecordStakeUpdateCall> for ISlasherCalls {
        fn from(value: RecordStakeUpdateCall) -> Self {
            Self::RecordStakeUpdate(value)
        }
    }
    impl ::core::convert::From<ResetFrozenStatusCall> for ISlasherCalls {
        fn from(value: ResetFrozenStatusCall) -> Self {
            Self::ResetFrozenStatus(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for ISlasherCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    ///Container type for all return fields from the `canSlash` function with signature `canSlash(address,address)` and selector `0xd98128c0`
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
    pub struct CanSlashReturn(pub bool);
    ///Container type for all return fields from the `canWithdraw` function with signature `canWithdraw(address,uint32,uint256)` and selector `0x8105e043`
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
    pub struct CanWithdrawReturn(pub bool);
    ///Container type for all return fields from the `contractCanSlashOperatorUntilBlock` function with signature `contractCanSlashOperatorUntilBlock(address,address)` and selector `0x6f0c2f74`
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
    pub struct ContractCanSlashOperatorUntilBlockReturn(pub u32);
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
    ///Container type for all return fields from the `getCorrectValueForInsertAfter` function with signature `getCorrectValueForInsertAfter(address,uint32)` and selector `0x723e59c7`
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
    pub struct GetCorrectValueForInsertAfterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMiddlewareTimesIndexServeUntilBlock` function with signature `getMiddlewareTimesIndexServeUntilBlock(address,uint32)` and selector `0x7259a45c`
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
    pub struct GetMiddlewareTimesIndexServeUntilBlockReturn(pub u32);
    ///Container type for all return fields from the `getMiddlewareTimesIndexStalestUpdateBlock` function with signature `getMiddlewareTimesIndexStalestUpdateBlock(address,uint32)` and selector `0x1874e5ae`
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
    pub struct GetMiddlewareTimesIndexStalestUpdateBlockReturn(pub u32);
    ///Container type for all return fields from the `isFrozen` function with signature `isFrozen(address)` and selector `0xe5839836`
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
    pub struct IsFrozenReturn(pub bool);
    ///Container type for all return fields from the `latestUpdateBlock` function with signature `latestUpdateBlock(address,address)` and selector `0xda16e29b`
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
    pub struct LatestUpdateBlockReturn(pub u32);
    ///Container type for all return fields from the `middlewareTimesLength` function with signature `middlewareTimesLength(address)` and selector `0xa49db732`
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
    pub struct MiddlewareTimesLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operatorToMiddlewareTimes` function with signature `operatorToMiddlewareTimes(address,uint256)` and selector `0x282670fc`
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
    pub struct OperatorToMiddlewareTimesReturn(pub MiddlewareTimes);
    ///Container type for all return fields from the `operatorWhitelistedContractsLinkedListEntry` function with signature `operatorWhitelistedContractsLinkedListEntry(address,address)` and selector `0x855fcc4a`
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
    pub struct OperatorWhitelistedContractsLinkedListEntryReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `operatorWhitelistedContractsLinkedListSize` function with signature `operatorWhitelistedContractsLinkedListSize(address)` and selector `0xe921d4fa`
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
    pub struct OperatorWhitelistedContractsLinkedListSizeReturn(pub ::ethers::core::types::U256);
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
    ///`MiddlewareTimes(uint32,uint32)`
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
    pub struct MiddlewareTimes {
        pub stalest_update_block: u32,
        pub latest_serve_until_block: u32,
    }
}
