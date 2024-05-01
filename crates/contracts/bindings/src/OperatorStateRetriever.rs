pub use operator_state_retriever::*;
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
pub mod operator_state_retriever {
    const _: () = {
        ::core::include_bytes!(
            "/Users/supernovahs/Desktop/eigensdk-rs/crates/contracts/bindings/utils/json/OperatorStateRetriever.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getCheckSignaturesIndices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCheckSignaturesIndices",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "referenceBlockNumber",
                                    ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerOperatorIds",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.CheckSignaturesIndices",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.Operator[][]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.Operator[][]",
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
                    ::std::borrow::ToOwned::to_owned("getQuorumBitmapsAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapsAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                            outputs: ::std::vec![
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OPERATORSTATERETRIEVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x1A\x05\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c5c\xB0\xD1\x14a\0QW\x80cOs\x9Ft\x14a\0zW\x80c\\\x15Vb\x14a\0\x9AW\x80c\xCE\xFD\xC1\xD4\x14a\0\xBAW[`\0\x80\xFD[a\0da\0_6`\x04a\x11rV[a\0\xDBV[`@Qa\0q\x91\x90a\x12\xCDV[`@Q\x80\x91\x03\x90\xF3[a\0\x8Da\0\x886`\x04a\x132V[a\x05qV[`@Qa\0q\x91\x90a\x145V[a\0\xADa\0\xA86`\x04a\x15\x13V[a\x0C\x9BV[`@Qa\0q\x91\x90a\x15\xC4V[a\0\xCDa\0\xC86`\x04a\x16\x08V[a\x0EcV[`@Qa\0q\x92\x91\x90a\x16JV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01A\x91\x90a\x16kV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xA7\x91\x90a\x16kV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\r\x91\x90a\x16kV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02*Wa\x02*a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02]W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02HW\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x05eW`\0\x88\x82\x81Q\x81\x10a\x02\x80Wa\x02\x80a\x16\x88V[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\t\x91\x90\x81\x01\x90a\x16\x9EV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03$Wa\x03$a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03oW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x03BW\x90P[P\x84\x84\x81Q\x81\x10a\x03\x82Wa\x03\x82a\x16\x88V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x05OW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x03\xC5Wa\x03\xC5a\x16\x88V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04,\x91\x90a\x16kV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x04LWa\x04La\x16\x88V[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x04zWa\x04za\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFA\x91\x90a\x17.V[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x05\x18Wa\x05\x18a\x16\x88V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x051Wa\x051a\x16\x88V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x05G\x90a\x17mV[\x91PPa\x03\x90V[PPP\x80\x80a\x05]\x90a\x17mV[\x91PPa\x02cV[P\x97\x96PPPPPPPV[a\x05\x9C`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\0\x91\x90a\x16kV[\x90Pa\x06-`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x06]\x90\x8B\x90\x89\x90\x89\x90`\x04\x01a\x17\x88V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\xA2\x91\x90\x81\x01\x90a\x17\xD2V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x06\xD4\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x18\x89V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x19\x91\x90\x81\x01\x90a\x17\xD2V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x076Wa\x076a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07iW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07TW\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x0B\xACW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x97Wa\x07\x97a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xC0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x07\xDAWa\x07\xDAa\x16\x88V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\n\xACW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x08\x13Wa\x08\x13a\x16\x88V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x081Wa\x081a\x16\x88V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08n\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xAF\x91\x90a\x18\xB2V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\tlWa\tla\x16\x88V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\n\x99W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\t\xAEWa\t\xAEa\x16\x88V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\t\xCAWa\t\xCAa\x16\x88V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nD\x91\x90a\x18\xDBV[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\n]Wa\n]a\x16\x88V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\nvWa\nva\x16\x88V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\n\x95\x81a\x17mV[\x93PP[P\x80a\n\xA4\x81a\x17mV[\x91PPa\x07\xE8V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xC7Wa\n\xC7a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xF0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0BqW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x0B\x17Wa\x0B\x17a\x16\x88V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x0B0Wa\x0B0a\x16\x88V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0BJWa\x0BJa\x16\x88V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0Bi\x81a\x17mV[\x91PPa\n\xF6V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x0B\x8CWa\x0B\x8Ca\x16\x88V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x0B\xA4\x90a\x18\xF8V[\x91PPa\x07rV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x11\x91\x90a\x16kV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x0CD\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01a\x19\x18V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CaW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\x89\x91\x90\x81\x01\x90a\x17\xD2V[` \x83\x01RP\x98\x97PPPPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xCD\x92\x91\x90a\x19BV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x12\x91\x90\x81\x01\x90a\x17\xD2V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r/Wa\r/a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\rXW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a\x0EYW\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a\r\x88Wa\r\x88a\x16\x88V[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a\r\xA3Wa\r\xA3a\x16\x88V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xE0\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E!\x91\x90a\x18\xB2V[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x0E<Wa\x0E<a\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0EQ\x81a\x17mV[\x91PPa\r^V[P\x95\x94PPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\x0E\x9EWa\x0E\x9Ea\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a\x0E\xDA\x90\x88\x90\x86\x90`\x04\x01a\x19BV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x1F\x91\x90\x81\x01\x90a\x17\xD2V[`\0\x81Q\x81\x10a\x0F1Wa\x0F1a\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC1\x91\x90a\x18\xB2V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a\x0F\xD7\x82a\x0F\xF5V[\x90P\x81a\x0F\xE5\x8A\x83\x8Aa\0\xDBV[\x95P\x95PPPPP\x93P\x93\x91PPV[```\0\x80a\x10\x03\x84a\x10\xC1V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x1EWa\x10\x1Ea\x11\nV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10HW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x10`WPa\x01\0\x81\x10[\x15a\x10\xB7W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x10\xA7W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x10\x89Wa\x10\x89a\x16\x88V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x10\xB0\x81a\x17mV[\x90Pa\x10OV[P\x90\x94\x93PPPPV[`\0\x80[\x82\x15a\x10\xECWa\x10\xD6`\x01\x84a\x19\x96V[\x90\x92\x16\x91\x80a\x10\xE4\x81a\x19\xADV[\x91PPa\x10\xC5V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\x07W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11HWa\x11Ha\x11\nV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\x07W`\0\x80\xFD[\x805a\x11m\x81a\x11PV[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\x87W`\0\x80\xFD[\x835a\x11\x92\x81a\x10\xF2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\xAFW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x11\xC3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\xD5Wa\x11\xD5a\x11\nV[a\x11\xE7`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x11 V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x11\xFDW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x12 `@\x85\x01a\x11bV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x12\xBFW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a\x12\xAAW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01a\x12fV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a\x12HV[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x12\xE0` \x83\x01\x84a\x12)V[\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x12\xF9W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x10W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13+W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x13KW`\0\x80\xFD[\x865a\x13V\x81a\x10\xF2V[\x95P` \x87\x015a\x13f\x81a\x11PV[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\x82W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x13\x96W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x13\xA5W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x13\xB7W`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP``\x89\x015\x91P\x80\x82\x11\x15a\x13\xD5W`\0\x80\xFD[Pa\x13\xE2\x89\x82\x8A\x01a\x12\xE7V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x14*W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x14\x08V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01Ra\x14Q`\xA0\x85\x01\x82a\x13\xF4V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01Ra\x14n\x83\x83a\x13\xF4V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01Ra\x14\x8B\x83\x83a\x13\xF4V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15a\x14\xE2W\x84\x87\x83\x03\x01\x84Ra\x14\xD0\x82\x87Qa\x13\xF4V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01a\x14\xB6V[P\x99\x98PPPPPPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\tWa\x15\ta\x11\nV[P`\x05\x1B` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15(W`\0\x80\xFD[\x835a\x153\x81a\x10\xF2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15OW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x15`W`\0\x80\xFD[\x805a\x15sa\x15n\x82a\x14\xF0V[a\x11 V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x89\x83\x11\x15a\x15\x92W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x15\xB0W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15\x97V[\x80\x96PPPPPPa\x12 `@\x85\x01a\x11bV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x15\xFCW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x15\xE0V[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\x1DW`\0\x80\xFD[\x835a\x16(\x81a\x10\xF2V[\x92P` \x84\x015\x91P`@\x84\x015a\x16?\x81a\x11PV[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0a\x16c`@\x83\x01\x84a\x12)V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x16}W`\0\x80\xFD[\x81Qa\x12\xE0\x81a\x10\xF2V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x16\xB1W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xC7W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x16\xD8W`\0\x80\xFD[\x80Qa\x16\xE6a\x15n\x82a\x14\xF0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x17\x05W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x17#W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x17\nV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x17@W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x12\xE0W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x17\x81Wa\x17\x81a\x17WV[P`\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x17\xB5W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x17\xE5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xFBW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x18\x0CW`\0\x80\xFD[\x80Qa\x18\x1Aa\x15n\x82a\x14\xF0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x189W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x17#W\x83Qa\x18Q\x81a\x11PV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x18>V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x18\xA9`@\x83\x01\x84\x86a\x18`V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x18\xC4W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x12\xE0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x18\xEDW`\0\x80\xFD[\x81Qa\x12\xE0\x81a\x11PV[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x19\x0FWa\x19\x0Fa\x17WV[`\x01\x01\x92\x91PPV[`@\x81R`\0a\x19,`@\x83\x01\x85\x87a\x18`V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15a\x19\x89W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x19mV[P\x90\x97\x96PPPPPPPV[`\0\x82\x82\x10\x15a\x19\xA8Wa\x19\xA8a\x17WV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x19\xC5Wa\x19\xC5a\x17WV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x19nb-\xD4[\xD3a\0\x01v\x11\xC9\x87\x1D\xF0\x9E\xB9\xE7\x1D\x10/\x84\xCA\x0B\x18\x89\xE5\xF36.\xB2dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static OPERATORSTATERETRIEVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c5c\xB0\xD1\x14a\0QW\x80cOs\x9Ft\x14a\0zW\x80c\\\x15Vb\x14a\0\x9AW\x80c\xCE\xFD\xC1\xD4\x14a\0\xBAW[`\0\x80\xFD[a\0da\0_6`\x04a\x11rV[a\0\xDBV[`@Qa\0q\x91\x90a\x12\xCDV[`@Q\x80\x91\x03\x90\xF3[a\0\x8Da\0\x886`\x04a\x132V[a\x05qV[`@Qa\0q\x91\x90a\x145V[a\0\xADa\0\xA86`\x04a\x15\x13V[a\x0C\x9BV[`@Qa\0q\x91\x90a\x15\xC4V[a\0\xCDa\0\xC86`\x04a\x16\x08V[a\x0EcV[`@Qa\0q\x92\x91\x90a\x16JV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01A\x91\x90a\x16kV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xA7\x91\x90a\x16kV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\r\x91\x90a\x16kV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02*Wa\x02*a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02]W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02HW\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x05eW`\0\x88\x82\x81Q\x81\x10a\x02\x80Wa\x02\x80a\x16\x88V[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\t\x91\x90\x81\x01\x90a\x16\x9EV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03$Wa\x03$a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03oW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x03BW\x90P[P\x84\x84\x81Q\x81\x10a\x03\x82Wa\x03\x82a\x16\x88V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x05OW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x03\xC5Wa\x03\xC5a\x16\x88V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xEB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04,\x91\x90a\x16kV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x04LWa\x04La\x16\x88V[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x04zWa\x04za\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFA\x91\x90a\x17.V[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x05\x18Wa\x05\x18a\x16\x88V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x051Wa\x051a\x16\x88V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x05G\x90a\x17mV[\x91PPa\x03\x90V[PPP\x80\x80a\x05]\x90a\x17mV[\x91PPa\x02cV[P\x97\x96PPPPPPPV[a\x05\x9C`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\0\x91\x90a\x16kV[\x90Pa\x06-`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x06]\x90\x8B\x90\x89\x90\x89\x90`\x04\x01a\x17\x88V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\xA2\x91\x90\x81\x01\x90a\x17\xD2V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x06\xD4\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x18\x89V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x19\x91\x90\x81\x01\x90a\x17\xD2V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x076Wa\x076a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07iW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07TW\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x0B\xACW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x97Wa\x07\x97a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xC0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x07\xDAWa\x07\xDAa\x16\x88V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\n\xACW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x08\x13Wa\x08\x13a\x16\x88V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x081Wa\x081a\x16\x88V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08n\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xAF\x91\x90a\x18\xB2V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01`@Q\x80\x91\x03\x90\xFD[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\tlWa\tla\x16\x88V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\n\x99W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\t\xAEWa\t\xAEa\x16\x88V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\t\xCAWa\t\xCAa\x16\x88V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nD\x91\x90a\x18\xDBV[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\n]Wa\n]a\x16\x88V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\nvWa\nva\x16\x88V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\n\x95\x81a\x17mV[\x93PP[P\x80a\n\xA4\x81a\x17mV[\x91PPa\x07\xE8V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xC7Wa\n\xC7a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xF0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x0BqW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x0B\x17Wa\x0B\x17a\x16\x88V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x0B0Wa\x0B0a\x16\x88V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x0BJWa\x0BJa\x16\x88V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x0Bi\x81a\x17mV[\x91PPa\n\xF6V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x0B\x8CWa\x0B\x8Ca\x16\x88V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x0B\xA4\x90a\x18\xF8V[\x91PPa\x07rV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x11\x91\x90a\x16kV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x0CD\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01a\x19\x18V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CaW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\x89\x91\x90\x81\x01\x90a\x17\xD2V[` \x83\x01RP\x98\x97PPPPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xCD\x92\x91\x90a\x19BV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x12\x91\x90\x81\x01\x90a\x17\xD2V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r/Wa\r/a\x11\nV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\rXW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a\x0EYW\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a\r\x88Wa\r\x88a\x16\x88V[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a\r\xA3Wa\r\xA3a\x16\x88V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xE0\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E!\x91\x90a\x18\xB2V[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x0E<Wa\x0E<a\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0EQ\x81a\x17mV[\x91PPa\r^V[P\x95\x94PPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\x0E\x9EWa\x0E\x9Ea\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a\x0E\xDA\x90\x88\x90\x86\x90`\x04\x01a\x19BV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x1F\x91\x90\x81\x01\x90a\x17\xD2V[`\0\x81Q\x81\x10a\x0F1Wa\x0F1a\x16\x88V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC1\x91\x90a\x18\xB2V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a\x0F\xD7\x82a\x0F\xF5V[\x90P\x81a\x0F\xE5\x8A\x83\x8Aa\0\xDBV[\x95P\x95PPPPP\x93P\x93\x91PPV[```\0\x80a\x10\x03\x84a\x10\xC1V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x1EWa\x10\x1Ea\x11\nV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10HW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x10`WPa\x01\0\x81\x10[\x15a\x10\xB7W`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x10\xA7W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x10\x89Wa\x10\x89a\x16\x88V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x10\xB0\x81a\x17mV[\x90Pa\x10OV[P\x90\x94\x93PPPPV[`\0\x80[\x82\x15a\x10\xECWa\x10\xD6`\x01\x84a\x19\x96V[\x90\x92\x16\x91\x80a\x10\xE4\x81a\x19\xADV[\x91PPa\x10\xC5V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\x07W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x11HWa\x11Ha\x11\nV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\x07W`\0\x80\xFD[\x805a\x11m\x81a\x11PV[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\x87W`\0\x80\xFD[\x835a\x11\x92\x81a\x10\xF2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\xAFW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x11\xC3W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\xD5Wa\x11\xD5a\x11\nV[a\x11\xE7`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x11 V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x11\xFDW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x12 `@\x85\x01a\x11bV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x12\xBFW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a\x12\xAAW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01a\x12fV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a\x12HV[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x12\xE0` \x83\x01\x84a\x12)V[\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x12\xF9W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x10W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x13+W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\x13KW`\0\x80\xFD[\x865a\x13V\x81a\x10\xF2V[\x95P` \x87\x015a\x13f\x81a\x11PV[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\x82W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x13\x96W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x13\xA5W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x13\xB7W`\0\x80\xFD[` \x83\x01\x96P\x80\x95PP``\x89\x015\x91P\x80\x82\x11\x15a\x13\xD5W`\0\x80\xFD[Pa\x13\xE2\x89\x82\x8A\x01a\x12\xE7V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x14*W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x14\x08V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01Ra\x14Q`\xA0\x85\x01\x82a\x13\xF4V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01Ra\x14n\x83\x83a\x13\xF4V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01Ra\x14\x8B\x83\x83a\x13\xF4V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15a\x14\xE2W\x84\x87\x83\x03\x01\x84Ra\x14\xD0\x82\x87Qa\x13\xF4V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01a\x14\xB6V[P\x99\x98PPPPPPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\tWa\x15\ta\x11\nV[P`\x05\x1B` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15(W`\0\x80\xFD[\x835a\x153\x81a\x10\xF2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15OW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x15`W`\0\x80\xFD[\x805a\x15sa\x15n\x82a\x14\xF0V[a\x11 V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x89\x83\x11\x15a\x15\x92W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x15\xB0W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15\x97V[\x80\x96PPPPPPa\x12 `@\x85\x01a\x11bV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x15\xFCW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x15\xE0V[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\x1DW`\0\x80\xFD[\x835a\x16(\x81a\x10\xF2V[\x92P` \x84\x015\x91P`@\x84\x015a\x16?\x81a\x11PV[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0a\x16c`@\x83\x01\x84a\x12)V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x16}W`\0\x80\xFD[\x81Qa\x12\xE0\x81a\x10\xF2V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x16\xB1W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xC7W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x16\xD8W`\0\x80\xFD[\x80Qa\x16\xE6a\x15n\x82a\x14\xF0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x17\x05W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x17#W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x17\nV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x17@W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x12\xE0W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x17\x81Wa\x17\x81a\x17WV[P`\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x17\xB5W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x17\xE5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xFBW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x18\x0CW`\0\x80\xFD[\x80Qa\x18\x1Aa\x15n\x82a\x14\xF0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x189W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x17#W\x83Qa\x18Q\x81a\x11PV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x18>V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0a\x18\xA9`@\x83\x01\x84\x86a\x18`V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x18\xC4W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x12\xE0W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x18\xEDW`\0\x80\xFD[\x81Qa\x12\xE0\x81a\x11PV[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x19\x0FWa\x19\x0Fa\x17WV[`\x01\x01\x92\x91PPV[`@\x81R`\0a\x19,`@\x83\x01\x85\x87a\x18`V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15a\x19\x89W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x19mV[P\x90\x97\x96PPPPPPPV[`\0\x82\x82\x10\x15a\x19\xA8Wa\x19\xA8a\x17WV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x19\xC5Wa\x19\xC5a\x17WV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x19nb-\xD4[\xD3a\0\x01v\x11\xC9\x87\x1D\xF0\x9E\xB9\xE7\x1D\x10/\x84\xCA\x0B\x18\x89\xE5\xF36.\xB2dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static OPERATORSTATERETRIEVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct OperatorStateRetriever<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OperatorStateRetriever<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OperatorStateRetriever<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OperatorStateRetriever<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OperatorStateRetriever<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OperatorStateRetriever))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OperatorStateRetriever<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OPERATORSTATERETRIEVER_ABI.clone(),
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
                OPERATORSTATERETRIEVER_ABI.clone(),
                OPERATORSTATERETRIEVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getCheckSignaturesIndices` (0x4f739f74) function
        pub fn get_check_signatures_indices(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            reference_block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
            non_signer_operator_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, CheckSignaturesIndices> {
            self.0
                .method_hash(
                    [79, 115, 159, 116],
                    (
                        registry_coordinator,
                        reference_block_number,
                        quorum_numbers,
                        non_signer_operator_ids,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0x3563b0d1) function
        pub fn get_operator_state(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<Operator>>,
        > {
            self.0
                .method_hash(
                    [53, 99, 176, 209],
                    (registry_coordinator, quorum_numbers, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0xcefdc1d4) function
        pub fn get_operator_state_with_registry_coordinator_and_operator_id(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::std::vec::Vec<::std::vec::Vec<Operator>>),
        > {
            self.0
                .method_hash(
                    [206, 253, 193, 212],
                    (registry_coordinator, operator_id, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapsAtBlockNumber` (0x5c155662) function
        pub fn get_quorum_bitmaps_at_block_number(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operator_ids: ::std::vec::Vec<[u8; 32]>,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [92, 21, 86, 98],
                    (registry_coordinator, operator_ids, block_number),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for OperatorStateRetriever<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getCheckSignaturesIndices` function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`
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
        name = "getCheckSignaturesIndices",
        abi = "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])"
    )]
    pub struct GetCheckSignaturesIndicesCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub reference_block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub non_signer_operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `getOperatorState` function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`
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
    #[ethcall(name = "getOperatorState", abi = "getOperatorState(address,bytes,uint32)")]
    pub struct GetOperatorStateCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getOperatorState` function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`
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
        name = "getOperatorState",
        abi = "getOperatorState(address,bytes32,uint32)"
    )]
    pub struct GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getQuorumBitmapsAtBlockNumber` function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`
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
        name = "getQuorumBitmapsAtBlockNumber",
        abi = "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)"
    )]
    pub struct GetQuorumBitmapsAtBlockNumberCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operator_ids: ::std::vec::Vec<[u8; 32]>,
        pub block_number: u32,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OperatorStateRetrieverCalls {
        GetCheckSignaturesIndices(GetCheckSignaturesIndicesCall),
        GetOperatorState(GetOperatorStateCall),
        GetOperatorStateWithRegistryCoordinatorAndOperatorId(
            GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ),
        GetQuorumBitmapsAtBlockNumber(GetQuorumBitmapsAtBlockNumberCall),
    }
    impl ::ethers::core::abi::AbiDecode for OperatorStateRetrieverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetCheckSignaturesIndicesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCheckSignaturesIndices(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorState(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(decoded),
                );
            }
            if let Ok(decoded) = <GetQuorumBitmapsAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapsAtBlockNumber(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OperatorStateRetrieverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetCheckSignaturesIndices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapsAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OperatorStateRetrieverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetCheckSignaturesIndices(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapsAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetCheckSignaturesIndicesCall>
    for OperatorStateRetrieverCalls {
        fn from(value: GetCheckSignaturesIndicesCall) -> Self {
            Self::GetCheckSignaturesIndices(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateCall> for OperatorStateRetrieverCalls {
        fn from(value: GetOperatorStateCall) -> Self {
            Self::GetOperatorState(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall>
    for OperatorStateRetrieverCalls {
        fn from(
            value: GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ) -> Self {
            Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapsAtBlockNumberCall>
    for OperatorStateRetrieverCalls {
        fn from(value: GetQuorumBitmapsAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapsAtBlockNumber(value)
        }
    }
    ///Container type for all return fields from the `getCheckSignaturesIndices` function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`
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
    pub struct GetCheckSignaturesIndicesReturn(pub CheckSignaturesIndices);
    ///Container type for all return fields from the `getOperatorState` function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`
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
    pub struct GetOperatorStateReturn(pub ::std::vec::Vec<::std::vec::Vec<Operator>>);
    ///Container type for all return fields from the `getOperatorState` function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`
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
    pub struct GetOperatorStateWithRegistryCoordinatorAndOperatorIdReturn(
        pub ::ethers::core::types::U256,
        pub ::std::vec::Vec<::std::vec::Vec<Operator>>,
    );
    ///Container type for all return fields from the `getQuorumBitmapsAtBlockNumber` function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`
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
    pub struct GetQuorumBitmapsAtBlockNumberReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///`CheckSignaturesIndices(uint32[],uint32[],uint32[],uint32[][])`
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
    pub struct CheckSignaturesIndices {
        pub non_signer_quorum_bitmap_indices: ::std::vec::Vec<u32>,
        pub quorum_apk_indices: ::std::vec::Vec<u32>,
        pub total_stake_indices: ::std::vec::Vec<u32>,
        pub non_signer_stake_indices: ::std::vec::Vec<::std::vec::Vec<u32>>,
    }
    ///`Operator(address,bytes32,uint96)`
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
    pub struct Operator {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub stake: u128,
    }
}
