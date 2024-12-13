///Module containing a contract's types and functions.
/**

```solidity
library IEjectionManager {
    struct QuorumEjectionParams { uint32 rateLimitWindow; uint16 ejectableStakePercent; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IEjectionManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct QuorumEjectionParams { uint32 rateLimitWindow; uint16 ejectableStakePercent; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumEjectionParams {
        pub rateLimitWindow: u32,
        pub ejectableStakePercent: u16,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<16>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, u16);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QuorumEjectionParams> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumEjectionParams) -> Self {
                (value.rateLimitWindow, value.ejectableStakePercent)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumEjectionParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    rateLimitWindow: tuple.0,
                    ejectableStakePercent: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumEjectionParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumEjectionParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rateLimitWindow),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.ejectableStakePercent),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for QuorumEjectionParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for QuorumEjectionParams {
            const NAME: &'static str = "QuorumEjectionParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumEjectionParams(uint32 rateLimitWindow,uint16 ejectableStakePercent)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rateLimitWindow,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ejectableStakePercent,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumEjectionParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rateLimitWindow,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ejectableStakePercent,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rateLimitWindow,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    16,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ejectableStakePercent,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IEjectionManager`](self) contract instance.

See the [wrapper's documentation](`IEjectionManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IEjectionManagerInstance<T, P, N> {
        IEjectionManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IEjectionManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IEjectionManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IEjectionManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IEjectionManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IEjectionManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEjectionManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IEjectionManager`](self) contract instance.

See the [wrapper's documentation](`IEjectionManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IEjectionManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IEjectionManagerInstance<T, P, N> {
            IEjectionManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEjectionManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEjectionManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IEjectionManager {
    struct QuorumEjectionParams {
        uint32 rateLimitWindow;
        uint16 ejectableStakePercent;
    }
}

interface EjectionManager {
    event EjectorUpdated(address ejector, bool status);
    event Initialized(uint8 version);
    event OperatorEjected(bytes32 operatorId, uint8 quorumNumber);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event QuorumEjection(uint32 ejectedOperators, bool ratelimitHit);
    event QuorumEjectionParamsSet(uint8 quorumNumber, uint32 rateLimitWindow, uint16 ejectableStakePercent);

    constructor(address _registryCoordinator, address _stakeRegistry);

    function amountEjectableForQuorum(uint8 _quorumNumber) external view returns (uint256);
    function ejectOperators(bytes32[][] memory _operatorIds) external;
    function initialize(address _owner, address[] memory _ejectors, IEjectionManager.QuorumEjectionParams[] memory _quorumEjectionParams) external;
    function isEjector(address) external view returns (bool);
    function owner() external view returns (address);
    function quorumEjectionParams(uint8) external view returns (uint32 rateLimitWindow, uint16 ejectableStakePercent);
    function registryCoordinator() external view returns (address);
    function renounceOwnership() external;
    function setEjector(address _ejector, bool _status) external;
    function setQuorumEjectionParams(uint8 _quorumNumber, IEjectionManager.QuorumEjectionParams memory _quorumEjectionParams) external;
    function stakeEjectedForQuorum(uint8, uint256) external view returns (uint256 timestamp, uint256 stakeEjected);
    function stakeRegistry() external view returns (address);
    function transferOwnership(address newOwner) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_registryCoordinator",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "amountEjectableForQuorum",
    "inputs": [
      {
        "name": "_quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ejectOperators",
    "inputs": [
      {
        "name": "_operatorIds",
        "type": "bytes32[][]",
        "internalType": "bytes32[][]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_ejectors",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "_quorumEjectionParams",
        "type": "tuple[]",
        "internalType": "struct IEjectionManager.QuorumEjectionParams[]",
        "components": [
          {
            "name": "rateLimitWindow",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "ejectableStakePercent",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isEjector",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "quorumEjectionParams",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "rateLimitWindow",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "ejectableStakePercent",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEjector",
    "inputs": [
      {
        "name": "_ejector",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_status",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setQuorumEjectionParams",
    "inputs": [
      {
        "name": "_quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_quorumEjectionParams",
        "type": "tuple",
        "internalType": "struct IEjectionManager.QuorumEjectionParams",
        "components": [
          {
            "name": "rateLimitWindow",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "ejectableStakePercent",
            "type": "uint16",
            "internalType": "uint16"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stakeEjectedForQuorum",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "timestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "stakeEjected",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "EjectorUpdated",
    "inputs": [
      {
        "name": "ejector",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "status",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorEjected",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumEjection",
    "inputs": [
      {
        "name": "ejectedOperators",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "ratelimitHit",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumEjectionParamsSet",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "rateLimitWindow",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "ejectableStakePercent",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod EjectionManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c060405234801561000f575f5ffd5b506040516125af3803806125af83398181016040528101906100319190610217565b8173ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff1660a08173ffffffffffffffffffffffffffffffffffffffff16815250506100a76100ae60201b60201c565b5050610327565b5f60019054906101000a900460ff16156100fd576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100f4906102d5565b60405180910390fd5b60ff80165f5f9054906101000a900460ff1660ff16101561016b5760ff5f5f6101000a81548160ff021916908360ff1602179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860ff604051610162919061030e565b60405180910390a15b565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61019a82610171565b9050919050565b5f6101ab82610190565b9050919050565b6101bb816101a1565b81146101c5575f5ffd5b50565b5f815190506101d6816101b2565b92915050565b5f6101e682610190565b9050919050565b6101f6816101dc565b8114610200575f5ffd5b50565b5f81519050610211816101ed565b92915050565b5f5f6040838503121561022d5761022c61016d565b5b5f61023a858286016101c8565b925050602061024b85828601610203565b9150509250929050565b5f82825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320696e6974695f8201527f616c697a696e6700000000000000000000000000000000000000000000000000602082015250565b5f6102bf602783610255565b91506102ca82610265565b604082019050919050565b5f6020820190508181035f8301526102ec816102b3565b9050919050565b5f60ff82169050919050565b610308816102f3565b82525050565b5f6020820190506103215f8301846102ff565b92915050565b60805160a05161224b6103645f395f81816103d3015281816109310152610bc701525f81816105dc015281816106180152610972015261224b5ff3fe608060405234801561000f575f5ffd5b50600436106100cc575f3560e01c80636d14a9871161008a5780638b88a024116100645780638b88a024146101fc5780638da5cb5b14610218578063b13f450414610236578063f2fde38b14610266576100cc565b80636d14a987146101b8578063715018a6146101d657806377d17586146101e0576100cc565b8062482569146100d05780630a0593d11461010157806310ea4f8a1461011d5780633a0b0ddd14610139578063683048351461016a5780636c08a87914610188575b5f5ffd5b6100ea60048036038101906100e591906111c0565b610282565b6040516100f8929190611225565b60405180910390f35b61011b600480360381019061011691906114ad565b6102be565b005b61013760048036038101906101329190611583565b6108df565b005b610153600480360381019061014e91906115f4565b6108f5565b604051610161929190611641565b60405180910390f35b61017261092f565b60405161017f91906116c3565b60405180910390f35b6101a2600480360381019061019d91906116dc565b610953565b6040516101af9190611716565b60405180910390f35b6101c0610970565b6040516101cd919061174f565b60405180910390f35b6101de610994565b005b6101fa60048036038101906101f5919061180d565b6109a7565b005b610216600480360381019061021191906119cb565b6109bd565b005b610220610b94565b60405161022d9190611a62565b60405180910390f35b610250600480360381019061024b91906111c0565b610bbc565b60405161025d9190611a7b565b60405180910390f35b610280600480360381019061027b91906116dc565b610e0c565b005b6067602052805f5260405f205f91509050805f015f9054906101000a900463ffffffff1690805f0160049054906101000a900461ffff16905082565b60655f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16806103455750610316610b94565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b610384576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161037b90611b14565b60405180910390fd5b5f5f90505b81518110156108db575f8190505f6103a082610bbc565b90505f5f5f5f5f90505b8787815181106103bd576103bc611b32565b5b6020026020010151518160ff1610156107d1575f7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16635401ed278a8a815181106104205761041f611b32565b5b60200260200101518460ff168151811061043d5761043c611b32565b5b6020026020010151896040518363ffffffff1660e01b8152600401610463929190611b7d565b602060405180830381865afa15801561047e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104a29190611be5565b6bffffffffffffffffffffffff16905060655f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16801561053557505f60675f8960ff1660ff1681526020019081526020015f205f015f9054906101000a900463ffffffff1663ffffffff16115b801561054b57508581866105499190611c3d565b115b156105c05760665f8860ff1660ff1681526020019081526020015f20604051806040016040528042815260200187815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f015560208201518160010155505060019250506107d1565b80856105cc9190611c3d565b9450836105d890611c70565b93507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16636e3b17db7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663296bb0648c8c8151811061066557610664611b32565b5b60200260200101518660ff168151811061068257610681611b32565b5b60200260200101516040518263ffffffff1660e01b81526004016106a69190611c9b565b602060405180830381865afa1580156106c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106e59190611cc8565b896040516020016106f69190611d27565b6040516020818303038152906040526040518363ffffffff1660e01b8152600401610722929190611da1565b5f604051808303815f87803b158015610739575f5ffd5b505af115801561074b573d5f5f3e3d5ffd5b505050507f97ddb711c61a9d2d7effcba3e042a33862297f898d555655cca39ec4451f53b489898151811061078357610782611b32565b5b60200260200101518360ff16815181106107a05761079f611b32565b5b6020026020010151886040516107b7929190611b7d565b60405180910390a150806107ca90611dcf565b90506103aa565b5080158015610826575060655f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff165b156108925760665f8660ff1660ff1681526020019081526020015f20604051806040016040528042815260200185815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f01556020820151816001015550505b7f19dd87ae49ed14a795f8c2d5e8055bf2a4a9d01641a00a2f8f0a5a7bf7f7024982826040516108c3929190611df7565b60405180910390a15050505050806001019050610389565b5050565b6108e7610e8e565b6108f18282610f0c565b5050565b6066602052815f5260405f20818154811061090e575f80fd5b905f5260205f2090600202015f9150915050805f0154908060010154905082565b7f000000000000000000000000000000000000000000000000000000000000000081565b6065602052805f5260405f205f915054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b61099c610e8e565b6109a55f610f9d565b565b6109af610e8e565b6109b98282611060565b5050565b5f5f60019054906101000a900460ff161590508080156109ed575060015f5f9054906101000a900460ff1660ff16105b80610a1a57506109fc30611150565b158015610a19575060015f5f9054906101000a900460ff1660ff16145b5b610a59576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a5090611e8e565b60405180910390fd5b60015f5f6101000a81548160ff021916908360ff1602179055508015610a945760015f60016101000a81548160ff0219169083151502179055505b610a9d84610f9d565b5f5f90505b83518160ff161015610ae957610ad6848260ff1681518110610ac757610ac6611b32565b5b60200260200101516001610f0c565b8080610ae190611dcf565b915050610aa2565b505f5f90505b82518160ff161015610b3557610b2281848360ff1681518110610b1557610b14611b32565b5b6020026020010151611060565b8080610b2d90611dcf565b915050610aef565b508015610b8e575f5f60016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024986001604051610b859190611ee5565b60405180910390a15b50505050565b5f60335f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f5f61271061ffff167f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663d5eccc05856040518263ffffffff1660e01b8152600401610c1e9190611efe565b602060405180830381865afa158015610c39573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c5d9190611be5565b6bffffffffffffffffffffffff1660675f8660ff1660ff1681526020019081526020015f205f0160049054906101000a900461ffff1661ffff16610ca19190611f17565b610cab9190611f85565b90505f60665f8560ff1660ff1681526020019081526020015f208054905003610cd75780915050610e07565b5f60675f8560ff1660ff1681526020019081526020015f205f015f9054906101000a900463ffffffff1663ffffffff1642610d129190611fb5565b90505f5f90505f600160665f8860ff1660ff1681526020019081526020015f2080549050610d409190611fb5565b90505b8260665f8860ff1660ff1681526020019081526020015f208281548110610d6d57610d6c611b32565b5b905f5260205f2090600202015f01541115610de15760665f8760ff1660ff1681526020019081526020015f208181548110610dab57610daa611b32565b5b905f5260205f2090600202016001015482610dc69190611c3d565b91505f810315610de15780610dda90611fe8565b9050610d43565b838210610df4575f945050505050610e07565b8184610e009190611fb5565b9450505050505b919050565b610e14610e8e565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610e82576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e799061207f565b60405180910390fd5b610e8b81610f9d565b50565b610e96611172565b73ffffffffffffffffffffffffffffffffffffffff16610eb4610b94565b73ffffffffffffffffffffffffffffffffffffffff1614610f0a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f01906120e7565b60405180910390fd5b565b8060655f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055507f7676686b6d22e112412bd874d70177e011ab06602c26063f19f0386c9a3cee428282604051610f91929190612105565b60405180910390a15050565b5f60335f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160335f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b60c060ff168260ff16106110a9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016110a0906121c2565b60405180910390fd5b8060675f8460ff1660ff1681526020019081526020015f205f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548161ffff021916908361ffff1602179055509050507fe69c2827a1e2fdd32265ebb4eeea5ee564f0551cf5dfed4150f8e116a67209eb82825f01518360200151604051611144939291906121e0565b60405180910390a15050565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f33905090565b5f604051905090565b5f5ffd5b5f5ffd5b5f60ff82169050919050565b61119f8161118a565b81146111a9575f5ffd5b50565b5f813590506111ba81611196565b92915050565b5f602082840312156111d5576111d4611182565b5b5f6111e2848285016111ac565b91505092915050565b5f63ffffffff82169050919050565b611203816111eb565b82525050565b5f61ffff82169050919050565b61121f81611209565b82525050565b5f6040820190506112385f8301856111fa565b6112456020830184611216565b9392505050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61129682611250565b810181811067ffffffffffffffff821117156112b5576112b4611260565b5b80604052505050565b5f6112c7611179565b90506112d3828261128d565b919050565b5f67ffffffffffffffff8211156112f2576112f1611260565b5b602082029050602081019050919050565b5f5ffd5b5f67ffffffffffffffff82111561132157611320611260565b5b602082029050602081019050919050565b5f819050919050565b61134481611332565b811461134e575f5ffd5b50565b5f8135905061135f8161133b565b92915050565b5f61137761137284611307565b6112be565b9050808382526020820190506020840283018581111561139a57611399611303565b5b835b818110156113c357806113af8882611351565b84526020840193505060208101905061139c565b5050509392505050565b5f82601f8301126113e1576113e061124c565b5b81356113f1848260208601611365565b91505092915050565b5f61140c611407846112d8565b6112be565b9050808382526020820190506020840283018581111561142f5761142e611303565b5b835b8181101561147657803567ffffffffffffffff8111156114545761145361124c565b5b80860161146189826113cd565b85526020850194505050602081019050611431565b5050509392505050565b5f82601f8301126114945761149361124c565b5b81356114a48482602086016113fa565b91505092915050565b5f602082840312156114c2576114c1611182565b5b5f82013567ffffffffffffffff8111156114df576114de611186565b5b6114eb84828501611480565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61151d826114f4565b9050919050565b61152d81611513565b8114611537575f5ffd5b50565b5f8135905061154881611524565b92915050565b5f8115159050919050565b6115628161154e565b811461156c575f5ffd5b50565b5f8135905061157d81611559565b92915050565b5f5f6040838503121561159957611598611182565b5b5f6115a68582860161153a565b92505060206115b78582860161156f565b9150509250929050565b5f819050919050565b6115d3816115c1565b81146115dd575f5ffd5b50565b5f813590506115ee816115ca565b92915050565b5f5f6040838503121561160a57611609611182565b5b5f611617858286016111ac565b9250506020611628858286016115e0565b9150509250929050565b61163b816115c1565b82525050565b5f6040820190506116545f830185611632565b6116616020830184611632565b9392505050565b5f819050919050565b5f61168b611686611681846114f4565b611668565b6114f4565b9050919050565b5f61169c82611671565b9050919050565b5f6116ad82611692565b9050919050565b6116bd816116a3565b82525050565b5f6020820190506116d65f8301846116b4565b92915050565b5f602082840312156116f1576116f0611182565b5b5f6116fe8482850161153a565b91505092915050565b6117108161154e565b82525050565b5f6020820190506117295f830184611707565b92915050565b5f61173982611692565b9050919050565b6117498161172f565b82525050565b5f6020820190506117625f830184611740565b92915050565b5f5ffd5b611775816111eb565b811461177f575f5ffd5b50565b5f813590506117908161176c565b92915050565b61179f81611209565b81146117a9575f5ffd5b50565b5f813590506117ba81611796565b92915050565b5f604082840312156117d5576117d4611768565b5b6117df60406112be565b90505f6117ee84828501611782565b5f830152506020611801848285016117ac565b60208301525092915050565b5f5f6060838503121561182357611822611182565b5b5f611830858286016111ac565b9250506020611841858286016117c0565b9150509250929050565b5f67ffffffffffffffff82111561186557611864611260565b5b602082029050602081019050919050565b5f6118886118838461184b565b6112be565b905080838252602082019050602084028301858111156118ab576118aa611303565b5b835b818110156118d457806118c0888261153a565b8452602084019350506020810190506118ad565b5050509392505050565b5f82601f8301126118f2576118f161124c565b5b8135611902848260208601611876565b91505092915050565b5f67ffffffffffffffff82111561192557611924611260565b5b602082029050602081019050919050565b5f6119486119438461190b565b6112be565b9050808382526020820190506040840283018581111561196b5761196a611303565b5b835b81811015611994578061198088826117c0565b84526020840193505060408101905061196d565b5050509392505050565b5f82601f8301126119b2576119b161124c565b5b81356119c2848260208601611936565b91505092915050565b5f5f5f606084860312156119e2576119e1611182565b5b5f6119ef8682870161153a565b935050602084013567ffffffffffffffff811115611a1057611a0f611186565b5b611a1c868287016118de565b925050604084013567ffffffffffffffff811115611a3d57611a3c611186565b5b611a498682870161199e565b9150509250925092565b611a5c81611513565b82525050565b5f602082019050611a755f830184611a53565b92915050565b5f602082019050611a8e5f830184611632565b92915050565b5f82825260208201905092915050565b7f456a656374696f6e4d616e616765722e656a6563744f70657261746f72733a205f8201527f4f6e6c79206f776e6572206f7220656a6563746f722063616e20656a65637400602082015250565b5f611afe603f83611a94565b9150611b0982611aa4565b604082019050919050565b5f6020820190508181035f830152611b2b81611af2565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b611b6881611332565b82525050565b611b778161118a565b82525050565b5f604082019050611b905f830185611b5f565b611b9d6020830184611b6e565b9392505050565b5f6bffffffffffffffffffffffff82169050919050565b611bc481611ba4565b8114611bce575f5ffd5b50565b5f81519050611bdf81611bbb565b92915050565b5f60208284031215611bfa57611bf9611182565b5b5f611c0784828501611bd1565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f611c47826115c1565b9150611c52836115c1565b9250828201905080821115611c6a57611c69611c10565b5b92915050565b5f611c7a826111eb565b915063ffffffff8203611c9057611c8f611c10565b5b600182019050919050565b5f602082019050611cae5f830184611b5f565b92915050565b5f81519050611cc281611524565b92915050565b5f60208284031215611cdd57611cdc611182565b5b5f611cea84828501611cb4565b91505092915050565b5f8160f81b9050919050565b5f611d0982611cf3565b9050919050565b611d21611d1c8261118a565b611cff565b82525050565b5f611d328284611d10565b60018201915081905092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f611d7382611d41565b611d7d8185611d4b565b9350611d8d818560208601611d5b565b611d9681611250565b840191505092915050565b5f604082019050611db45f830185611a53565b8181036020830152611dc68184611d69565b90509392505050565b5f611dd98261118a565b915060ff8203611dec57611deb611c10565b5b600182019050919050565b5f604082019050611e0a5f8301856111fa565b611e176020830184611707565b9392505050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c7265615f8201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b5f611e78602e83611a94565b9150611e8382611e1e565b604082019050919050565b5f6020820190508181035f830152611ea581611e6c565b9050919050565b5f819050919050565b5f611ecf611eca611ec584611eac565b611668565b61118a565b9050919050565b611edf81611eb5565b82525050565b5f602082019050611ef85f830184611ed6565b92915050565b5f602082019050611f115f830184611b6e565b92915050565b5f611f21826115c1565b9150611f2c836115c1565b9250828202611f3a816115c1565b91508282048414831517611f5157611f50611c10565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f611f8f826115c1565b9150611f9a836115c1565b925082611faa57611fa9611f58565b5b828204905092915050565b5f611fbf826115c1565b9150611fca836115c1565b9250828203905081811115611fe257611fe1611c10565b5b92915050565b5f611ff2826115c1565b91505f820361200457612003611c10565b5b600182039050919050565b7f4f776e61626c653a206e6577206f776e657220697320746865207a65726f20615f8201527f6464726573730000000000000000000000000000000000000000000000000000602082015250565b5f612069602683611a94565b91506120748261200f565b604082019050919050565b5f6020820190508181035f8301526120968161205d565b9050919050565b7f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65725f82015250565b5f6120d1602083611a94565b91506120dc8261209d565b602082019050919050565b5f6020820190508181035f8301526120fe816120c5565b9050919050565b5f6040820190506121185f830185611a53565b6121256020830184611707565b9392505050565b7f456a656374696f6e4d616e616765722e5f73657451756f72756d456a656374695f8201527f6f6e506172616d733a2051756f72756d206e756d62657220657863656564732060208201527f4d41585f51554f52554d5f434f554e5400000000000000000000000000000000604082015250565b5f6121ac605083611a94565b91506121b78261212c565b606082019050919050565b5f6020820190508181035f8301526121d9816121a0565b9050919050565b5f6060820190506121f35f830186611b6e565b61220060208301856111fa565b61220d6040830184611216565b94935050505056fea26469706673582212203f1416cc27db2ac09b283ce5560051b9b06291eda425ecd755504ab44c05453464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa%\xAF8\x03\x80a%\xAF\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x02\x17V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\0\xA7a\0\xAE` \x1B` \x1CV[PPa\x03'V[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\0\xFDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xF4\x90a\x02\xD5V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10\x15a\x01kW`\xFF__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\xFF`@Qa\x01b\x91\x90a\x03\x0EV[`@Q\x80\x91\x03\x90\xA1[V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01\x9A\x82a\x01qV[\x90P\x91\x90PV[_a\x01\xAB\x82a\x01\x90V[\x90P\x91\x90PV[a\x01\xBB\x81a\x01\xA1V[\x81\x14a\x01\xC5W__\xFD[PV[_\x81Q\x90Pa\x01\xD6\x81a\x01\xB2V[\x92\x91PPV[_a\x01\xE6\x82a\x01\x90V[\x90P\x91\x90PV[a\x01\xF6\x81a\x01\xDCV[\x81\x14a\x02\0W__\xFD[PV[_\x81Q\x90Pa\x02\x11\x81a\x01\xEDV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x02-Wa\x02,a\x01mV[[_a\x02:\x85\x82\x86\x01a\x01\xC8V[\x92PP` a\x02K\x85\x82\x86\x01a\x02\x03V[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is initi_\x82\x01R\x7Falizing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x02\xBF`'\x83a\x02UV[\x91Pa\x02\xCA\x82a\x02eV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x02\xEC\x81a\x02\xB3V[\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x03\x08\x81a\x02\xF3V[\x82RPPV[_` \x82\x01\x90Pa\x03!_\x83\x01\x84a\x02\xFFV[\x92\x91PPV[`\x80Q`\xA0Qa\"Ka\x03d_9_\x81\x81a\x03\xD3\x01R\x81\x81a\t1\x01Ra\x0B\xC7\x01R_\x81\x81a\x05\xDC\x01R\x81\x81a\x06\x18\x01Ra\tr\x01Ra\"K_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCCW_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\x8AW\x80c\x8B\x88\xA0$\x11a\0dW\x80c\x8B\x88\xA0$\x14a\x01\xFCW\x80c\x8D\xA5\xCB[\x14a\x02\x18W\x80c\xB1?E\x04\x14a\x026W\x80c\xF2\xFD\xE3\x8B\x14a\x02fWa\0\xCCV[\x80cm\x14\xA9\x87\x14a\x01\xB8W\x80cqP\x18\xA6\x14a\x01\xD6W\x80cw\xD1u\x86\x14a\x01\xE0Wa\0\xCCV[\x80bH%i\x14a\0\xD0W\x80c\n\x05\x93\xD1\x14a\x01\x01W\x80c\x10\xEAO\x8A\x14a\x01\x1DW\x80c:\x0B\r\xDD\x14a\x019W\x80ch0H5\x14a\x01jW\x80cl\x08\xA8y\x14a\x01\x88W[__\xFD[a\0\xEA`\x04\x806\x03\x81\x01\x90a\0\xE5\x91\x90a\x11\xC0V[a\x02\x82V[`@Qa\0\xF8\x92\x91\x90a\x12%V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1B`\x04\x806\x03\x81\x01\x90a\x01\x16\x91\x90a\x14\xADV[a\x02\xBEV[\0[a\x017`\x04\x806\x03\x81\x01\x90a\x012\x91\x90a\x15\x83V[a\x08\xDFV[\0[a\x01S`\x04\x806\x03\x81\x01\x90a\x01N\x91\x90a\x15\xF4V[a\x08\xF5V[`@Qa\x01a\x92\x91\x90a\x16AV[`@Q\x80\x91\x03\x90\xF3[a\x01ra\t/V[`@Qa\x01\x7F\x91\x90a\x16\xC3V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA2`\x04\x806\x03\x81\x01\x90a\x01\x9D\x91\x90a\x16\xDCV[a\tSV[`@Qa\x01\xAF\x91\x90a\x17\x16V[`@Q\x80\x91\x03\x90\xF3[a\x01\xC0a\tpV[`@Qa\x01\xCD\x91\x90a\x17OV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDEa\t\x94V[\0[a\x01\xFA`\x04\x806\x03\x81\x01\x90a\x01\xF5\x91\x90a\x18\rV[a\t\xA7V[\0[a\x02\x16`\x04\x806\x03\x81\x01\x90a\x02\x11\x91\x90a\x19\xCBV[a\t\xBDV[\0[a\x02 a\x0B\x94V[`@Qa\x02-\x91\x90a\x1AbV[`@Q\x80\x91\x03\x90\xF3[a\x02P`\x04\x806\x03\x81\x01\x90a\x02K\x91\x90a\x11\xC0V[a\x0B\xBCV[`@Qa\x02]\x91\x90a\x1A{V[`@Q\x80\x91\x03\x90\xF3[a\x02\x80`\x04\x806\x03\x81\x01\x90a\x02{\x91\x90a\x16\xDCV[a\x0E\x0CV[\0[`g` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x04\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x90P\x82V[`e_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x03EWPa\x03\x16a\x0B\x94V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x03\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03{\x90a\x1B\x14V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x81Q\x81\x10\x15a\x08\xDBW_\x81\x90P_a\x03\xA0\x82a\x0B\xBCV[\x90P_____\x90P[\x87\x87\x81Q\x81\x10a\x03\xBDWa\x03\xBCa\x1B2V[[` \x02` \x01\x01QQ\x81`\xFF\x16\x10\x15a\x07\xD1W_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cT\x01\xED'\x8A\x8A\x81Q\x81\x10a\x04 Wa\x04\x1Fa\x1B2V[[` \x02` \x01\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x04=Wa\x04<a\x1B2V[[` \x02` \x01\x01Q\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04c\x92\x91\x90a\x1B}V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xA2\x91\x90a\x1B\xE5V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`e_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80\x15a\x055WP_`g_\x89`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11[\x80\x15a\x05KWP\x85\x81\x86a\x05I\x91\x90a\x1C=V[\x11[\x15a\x05\xC0W`f_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80B\x81R` \x01\x87\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01UPP`\x01\x92PPa\x07\xD1V[\x80\x85a\x05\xCC\x91\x90a\x1C=V[\x94P\x83a\x05\xD8\x90a\x1CpV[\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cn;\x17\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c)k\xB0d\x8C\x8C\x81Q\x81\x10a\x06eWa\x06da\x1B2V[[` \x02` \x01\x01Q\x86`\xFF\x16\x81Q\x81\x10a\x06\x82Wa\x06\x81a\x1B2V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xA6\x91\x90a\x1C\x9BV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE5\x91\x90a\x1C\xC8V[\x89`@Q` \x01a\x06\xF6\x91\x90a\x1D'V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\"\x92\x91\x90a\x1D\xA1V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x079W__\xFD[PZ\xF1\x15\x80\x15a\x07KW=__>=_\xFD[PPPP\x7F\x97\xDD\xB7\x11\xC6\x1A\x9D-~\xFF\xCB\xA3\xE0B\xA38b)\x7F\x89\x8DUVU\xCC\xA3\x9E\xC4E\x1FS\xB4\x89\x89\x81Q\x81\x10a\x07\x83Wa\x07\x82a\x1B2V[[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x07\xA0Wa\x07\x9Fa\x1B2V[[` \x02` \x01\x01Q\x88`@Qa\x07\xB7\x92\x91\x90a\x1B}V[`@Q\x80\x91\x03\x90\xA1P\x80a\x07\xCA\x90a\x1D\xCFV[\x90Pa\x03\xAAV[P\x80\x15\x80\x15a\x08&WP`e_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[\x15a\x08\x92W`f_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80B\x81R` \x01\x85\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01UPP[\x7F\x19\xDD\x87\xAEI\xED\x14\xA7\x95\xF8\xC2\xD5\xE8\x05[\xF2\xA4\xA9\xD0\x16A\xA0\n/\x8F\nZ{\xF7\xF7\x02I\x82\x82`@Qa\x08\xC3\x92\x91\x90a\x1D\xF7V[`@Q\x80\x91\x03\x90\xA1PPPPP\x80`\x01\x01\x90Pa\x03\x89V[PPV[a\x08\xE7a\x0E\x8EV[a\x08\xF1\x82\x82a\x0F\x0CV[PPV[`f` R\x81_R`@_ \x81\x81T\x81\x10a\t\x0EW_\x80\xFD[\x90_R` _ \x90`\x02\x02\x01_\x91P\x91PP\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`e` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\t\x9Ca\x0E\x8EV[a\t\xA5_a\x0F\x9DV[V[a\t\xAFa\x0E\x8EV[a\t\xB9\x82\x82a\x10`V[PPV[__`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x90P\x80\x80\x15a\t\xEDWP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[\x80a\n\x1AWPa\t\xFC0a\x11PV[\x15\x80\x15a\n\x19WP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x14[[a\nYW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nP\x90a\x1E\x8EV[`@Q\x80\x91\x03\x90\xFD[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x15a\n\x94W`\x01_`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[a\n\x9D\x84a\x0F\x9DV[__\x90P[\x83Q\x81`\xFF\x16\x10\x15a\n\xE9Wa\n\xD6\x84\x82`\xFF\x16\x81Q\x81\x10a\n\xC7Wa\n\xC6a\x1B2V[[` \x02` \x01\x01Q`\x01a\x0F\x0CV[\x80\x80a\n\xE1\x90a\x1D\xCFV[\x91PPa\n\xA2V[P__\x90P[\x82Q\x81`\xFF\x16\x10\x15a\x0B5Wa\x0B\"\x81\x84\x83`\xFF\x16\x81Q\x81\x10a\x0B\x15Wa\x0B\x14a\x1B2V[[` \x02` \x01\x01Qa\x10`V[\x80\x80a\x0B-\x90a\x1D\xCFV[\x91PPa\n\xEFV[P\x80\x15a\x0B\x8EW__`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\x01`@Qa\x0B\x85\x91\x90a\x1E\xE5V[`@Q\x80\x91\x03\x90\xA1[PPPPV[_`3_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[__a'\x10a\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5\xEC\xCC\x05\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x1E\x91\x90a\x1E\xFEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C]\x91\x90a\x1B\xE5V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`g_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01`\x04\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16a\xFF\xFF\x16a\x0C\xA1\x91\x90a\x1F\x17V[a\x0C\xAB\x91\x90a\x1F\x85V[\x90P_`f_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x03a\x0C\xD7W\x80\x91PPa\x0E\x07V[_`g_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16Ba\r\x12\x91\x90a\x1F\xB5V[\x90P__\x90P_`\x01`f_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa\r@\x91\x90a\x1F\xB5V[\x90P[\x82`f_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\rmWa\rla\x1B2V[[\x90_R` _ \x90`\x02\x02\x01_\x01T\x11\x15a\r\xE1W`f_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a\r\xABWa\r\xAAa\x1B2V[[\x90_R` _ \x90`\x02\x02\x01`\x01\x01T\x82a\r\xC6\x91\x90a\x1C=V[\x91P_\x81\x03\x15a\r\xE1W\x80a\r\xDA\x90a\x1F\xE8V[\x90Pa\rCV[\x83\x82\x10a\r\xF4W_\x94PPPPPa\x0E\x07V[\x81\x84a\x0E\0\x91\x90a\x1F\xB5V[\x94PPPPP[\x91\x90PV[a\x0E\x14a\x0E\x8EV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0E\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ey\x90a \x7FV[`@Q\x80\x91\x03\x90\xFD[a\x0E\x8B\x81a\x0F\x9DV[PV[a\x0E\x96a\x11rV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0E\xB4a\x0B\x94V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x01\x90a \xE7V[`@Q\x80\x91\x03\x90\xFD[V[\x80`e_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fvvhkm\"\xE1\x12A+\xD8t\xD7\x01w\xE0\x11\xAB\x06`,&\x06?\x19\xF08l\x9A<\xEEB\x82\x82`@Qa\x0F\x91\x92\x91\x90a!\x05V[`@Q\x80\x91\x03\x90\xA1PPV[_`3_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`3_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\xC0`\xFF\x16\x82`\xFF\x16\x10a\x10\xA9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xA0\x90a!\xC2V[`@Q\x80\x91\x03\x90\xFD[\x80`g_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UP\x90PP\x7F\xE6\x9C('\xA1\xE2\xFD\xD3\"e\xEB\xB4\xEE\xEA^\xE5d\xF0U\x1C\xF5\xDF\xEDAP\xF8\xE1\x16\xA6r\t\xEB\x82\x82_\x01Q\x83` \x01Q`@Qa\x11D\x93\x92\x91\x90a!\xE0V[`@Q\x80\x91\x03\x90\xA1PPV[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_3\x90P\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\x11\x9F\x81a\x11\x8AV[\x81\x14a\x11\xA9W__\xFD[PV[_\x815\x90Pa\x11\xBA\x81a\x11\x96V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11\xD5Wa\x11\xD4a\x11\x82V[[_a\x11\xE2\x84\x82\x85\x01a\x11\xACV[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x12\x03\x81a\x11\xEBV[\x82RPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x12\x1F\x81a\x12\tV[\x82RPPV[_`@\x82\x01\x90Pa\x128_\x83\x01\x85a\x11\xFAV[a\x12E` \x83\x01\x84a\x12\x16V[\x93\x92PPPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x12\x96\x82a\x12PV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12\xB5Wa\x12\xB4a\x12`V[[\x80`@RPPPV[_a\x12\xC7a\x11yV[\x90Pa\x12\xD3\x82\x82a\x12\x8DV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12\xF2Wa\x12\xF1a\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13!Wa\x13 a\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x13D\x81a\x132V[\x81\x14a\x13NW__\xFD[PV[_\x815\x90Pa\x13_\x81a\x13;V[\x92\x91PPV[_a\x13wa\x13r\x84a\x13\x07V[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x13\x9AWa\x13\x99a\x13\x03V[[\x83[\x81\x81\x10\x15a\x13\xC3W\x80a\x13\xAF\x88\x82a\x13QV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x13\x9CV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x13\xE1Wa\x13\xE0a\x12LV[[\x815a\x13\xF1\x84\x82` \x86\x01a\x13eV[\x91PP\x92\x91PPV[_a\x14\x0Ca\x14\x07\x84a\x12\xD8V[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x14/Wa\x14.a\x13\x03V[[\x83[\x81\x81\x10\x15a\x14vW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14TWa\x14Sa\x12LV[[\x80\x86\x01a\x14a\x89\x82a\x13\xCDV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x141V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x14\x94Wa\x14\x93a\x12LV[[\x815a\x14\xA4\x84\x82` \x86\x01a\x13\xFAV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xC2Wa\x14\xC1a\x11\x82V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xDFWa\x14\xDEa\x11\x86V[[a\x14\xEB\x84\x82\x85\x01a\x14\x80V[\x91PP\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x15\x1D\x82a\x14\xF4V[\x90P\x91\x90PV[a\x15-\x81a\x15\x13V[\x81\x14a\x157W__\xFD[PV[_\x815\x90Pa\x15H\x81a\x15$V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x15b\x81a\x15NV[\x81\x14a\x15lW__\xFD[PV[_\x815\x90Pa\x15}\x81a\x15YV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x15\x99Wa\x15\x98a\x11\x82V[[_a\x15\xA6\x85\x82\x86\x01a\x15:V[\x92PP` a\x15\xB7\x85\x82\x86\x01a\x15oV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a\x15\xD3\x81a\x15\xC1V[\x81\x14a\x15\xDDW__\xFD[PV[_\x815\x90Pa\x15\xEE\x81a\x15\xCAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x16\nWa\x16\ta\x11\x82V[[_a\x16\x17\x85\x82\x86\x01a\x11\xACV[\x92PP` a\x16(\x85\x82\x86\x01a\x15\xE0V[\x91PP\x92P\x92\x90PV[a\x16;\x81a\x15\xC1V[\x82RPPV[_`@\x82\x01\x90Pa\x16T_\x83\x01\x85a\x162V[a\x16a` \x83\x01\x84a\x162V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x16\x8Ba\x16\x86a\x16\x81\x84a\x14\xF4V[a\x16hV[a\x14\xF4V[\x90P\x91\x90PV[_a\x16\x9C\x82a\x16qV[\x90P\x91\x90PV[_a\x16\xAD\x82a\x16\x92V[\x90P\x91\x90PV[a\x16\xBD\x81a\x16\xA3V[\x82RPPV[_` \x82\x01\x90Pa\x16\xD6_\x83\x01\x84a\x16\xB4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x16\xF1Wa\x16\xF0a\x11\x82V[[_a\x16\xFE\x84\x82\x85\x01a\x15:V[\x91PP\x92\x91PPV[a\x17\x10\x81a\x15NV[\x82RPPV[_` \x82\x01\x90Pa\x17)_\x83\x01\x84a\x17\x07V[\x92\x91PPV[_a\x179\x82a\x16\x92V[\x90P\x91\x90PV[a\x17I\x81a\x17/V[\x82RPPV[_` \x82\x01\x90Pa\x17b_\x83\x01\x84a\x17@V[\x92\x91PPV[__\xFD[a\x17u\x81a\x11\xEBV[\x81\x14a\x17\x7FW__\xFD[PV[_\x815\x90Pa\x17\x90\x81a\x17lV[\x92\x91PPV[a\x17\x9F\x81a\x12\tV[\x81\x14a\x17\xA9W__\xFD[PV[_\x815\x90Pa\x17\xBA\x81a\x17\x96V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x17\xD5Wa\x17\xD4a\x17hV[[a\x17\xDF`@a\x12\xBEV[\x90P_a\x17\xEE\x84\x82\x85\x01a\x17\x82V[_\x83\x01RP` a\x18\x01\x84\x82\x85\x01a\x17\xACV[` \x83\x01RP\x92\x91PPV[__``\x83\x85\x03\x12\x15a\x18#Wa\x18\"a\x11\x82V[[_a\x180\x85\x82\x86\x01a\x11\xACV[\x92PP` a\x18A\x85\x82\x86\x01a\x17\xC0V[\x91PP\x92P\x92\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18eWa\x18da\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a\x18\x88a\x18\x83\x84a\x18KV[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x18\xABWa\x18\xAAa\x13\x03V[[\x83[\x81\x81\x10\x15a\x18\xD4W\x80a\x18\xC0\x88\x82a\x15:V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x18\xADV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x18\xF2Wa\x18\xF1a\x12LV[[\x815a\x19\x02\x84\x82` \x86\x01a\x18vV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19%Wa\x19$a\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a\x19Ha\x19C\x84a\x19\x0BV[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15a\x19kWa\x19ja\x13\x03V[[\x83[\x81\x81\x10\x15a\x19\x94W\x80a\x19\x80\x88\x82a\x17\xC0V[\x84R` \x84\x01\x93PP`@\x81\x01\x90Pa\x19mV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x19\xB2Wa\x19\xB1a\x12LV[[\x815a\x19\xC2\x84\x82` \x86\x01a\x196V[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x19\xE2Wa\x19\xE1a\x11\x82V[[_a\x19\xEF\x86\x82\x87\x01a\x15:V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x10Wa\x1A\x0Fa\x11\x86V[[a\x1A\x1C\x86\x82\x87\x01a\x18\xDEV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A=Wa\x1A<a\x11\x86V[[a\x1AI\x86\x82\x87\x01a\x19\x9EV[\x91PP\x92P\x92P\x92V[a\x1A\\\x81a\x15\x13V[\x82RPPV[_` \x82\x01\x90Pa\x1Au_\x83\x01\x84a\x1ASV[\x92\x91PPV[_` \x82\x01\x90Pa\x1A\x8E_\x83\x01\x84a\x162V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEjectionManager.ejectOperators: _\x82\x01R\x7FOnly owner or ejector can eject\0` \x82\x01RPV[_a\x1A\xFE`?\x83a\x1A\x94V[\x91Pa\x1B\t\x82a\x1A\xA4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1B+\x81a\x1A\xF2V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x1Bh\x81a\x132V[\x82RPPV[a\x1Bw\x81a\x11\x8AV[\x82RPPV[_`@\x82\x01\x90Pa\x1B\x90_\x83\x01\x85a\x1B_V[a\x1B\x9D` \x83\x01\x84a\x1BnV[\x93\x92PPPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x1B\xC4\x81a\x1B\xA4V[\x81\x14a\x1B\xCEW__\xFD[PV[_\x81Q\x90Pa\x1B\xDF\x81a\x1B\xBBV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1B\xFAWa\x1B\xF9a\x11\x82V[[_a\x1C\x07\x84\x82\x85\x01a\x1B\xD1V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x1CG\x82a\x15\xC1V[\x91Pa\x1CR\x83a\x15\xC1V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1CjWa\x1Cia\x1C\x10V[[\x92\x91PPV[_a\x1Cz\x82a\x11\xEBV[\x91Pc\xFF\xFF\xFF\xFF\x82\x03a\x1C\x90Wa\x1C\x8Fa\x1C\x10V[[`\x01\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90Pa\x1C\xAE_\x83\x01\x84a\x1B_V[\x92\x91PPV[_\x81Q\x90Pa\x1C\xC2\x81a\x15$V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1C\xDDWa\x1C\xDCa\x11\x82V[[_a\x1C\xEA\x84\x82\x85\x01a\x1C\xB4V[\x91PP\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x1D\t\x82a\x1C\xF3V[\x90P\x91\x90PV[a\x1D!a\x1D\x1C\x82a\x11\x8AV[a\x1C\xFFV[\x82RPPV[_a\x1D2\x82\x84a\x1D\x10V[`\x01\x82\x01\x91P\x81\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x1Ds\x82a\x1DAV[a\x1D}\x81\x85a\x1DKV[\x93Pa\x1D\x8D\x81\x85` \x86\x01a\x1D[V[a\x1D\x96\x81a\x12PV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\x1D\xB4_\x83\x01\x85a\x1ASV[\x81\x81\x03` \x83\x01Ra\x1D\xC6\x81\x84a\x1DiV[\x90P\x93\x92PPPV[_a\x1D\xD9\x82a\x11\x8AV[\x91P`\xFF\x82\x03a\x1D\xECWa\x1D\xEBa\x1C\x10V[[`\x01\x82\x01\x90P\x91\x90PV[_`@\x82\x01\x90Pa\x1E\n_\x83\x01\x85a\x11\xFAV[a\x1E\x17` \x83\x01\x84a\x17\x07V[\x93\x92PPPV[\x7FInitializable: contract is alrea_\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1Ex`.\x83a\x1A\x94V[\x91Pa\x1E\x83\x82a\x1E\x1EV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1E\xA5\x81a\x1ElV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x1E\xCFa\x1E\xCAa\x1E\xC5\x84a\x1E\xACV[a\x16hV[a\x11\x8AV[\x90P\x91\x90PV[a\x1E\xDF\x81a\x1E\xB5V[\x82RPPV[_` \x82\x01\x90Pa\x1E\xF8_\x83\x01\x84a\x1E\xD6V[\x92\x91PPV[_` \x82\x01\x90Pa\x1F\x11_\x83\x01\x84a\x1BnV[\x92\x91PPV[_a\x1F!\x82a\x15\xC1V[\x91Pa\x1F,\x83a\x15\xC1V[\x92P\x82\x82\x02a\x1F:\x81a\x15\xC1V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1FQWa\x1FPa\x1C\x10V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x1F\x8F\x82a\x15\xC1V[\x91Pa\x1F\x9A\x83a\x15\xC1V[\x92P\x82a\x1F\xAAWa\x1F\xA9a\x1FXV[[\x82\x82\x04\x90P\x92\x91PPV[_a\x1F\xBF\x82a\x15\xC1V[\x91Pa\x1F\xCA\x83a\x15\xC1V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1F\xE2Wa\x1F\xE1a\x1C\x10V[[\x92\x91PPV[_a\x1F\xF2\x82a\x15\xC1V[\x91P_\x82\x03a \x04Wa \x03a\x1C\x10V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FOwnable: new owner is the zero a_\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a i`&\x83a\x1A\x94V[\x91Pa t\x82a \x0FV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra \x96\x81a ]V[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner_\x82\x01RPV[_a \xD1` \x83a\x1A\x94V[\x91Pa \xDC\x82a \x9DV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra \xFE\x81a \xC5V[\x90P\x91\x90PV[_`@\x82\x01\x90Pa!\x18_\x83\x01\x85a\x1ASV[a!%` \x83\x01\x84a\x17\x07V[\x93\x92PPPV[\x7FEjectionManager._setQuorumEjecti_\x82\x01R\x7FonParams: Quorum number exceeds ` \x82\x01R\x7FMAX_QUORUM_COUNT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a!\xAC`P\x83a\x1A\x94V[\x91Pa!\xB7\x82a!,V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra!\xD9\x81a!\xA0V[\x90P\x91\x90PV[_``\x82\x01\x90Pa!\xF3_\x83\x01\x86a\x1BnV[a\"\0` \x83\x01\x85a\x11\xFAV[a\"\r`@\x83\x01\x84a\x12\x16V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 ?\x14\x16\xCC'\xDB*\xC0\x9B(<\xE5V\0Q\xB9\xB0b\x91\xED\xA4%\xEC\xD7UPJ\xB4L\x05E4dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100cc575f3560e01c80636d14a9871161008a5780638b88a024116100645780638b88a024146101fc5780638da5cb5b14610218578063b13f450414610236578063f2fde38b14610266576100cc565b80636d14a987146101b8578063715018a6146101d657806377d17586146101e0576100cc565b8062482569146100d05780630a0593d11461010157806310ea4f8a1461011d5780633a0b0ddd14610139578063683048351461016a5780636c08a87914610188575b5f5ffd5b6100ea60048036038101906100e591906111c0565b610282565b6040516100f8929190611225565b60405180910390f35b61011b600480360381019061011691906114ad565b6102be565b005b61013760048036038101906101329190611583565b6108df565b005b610153600480360381019061014e91906115f4565b6108f5565b604051610161929190611641565b60405180910390f35b61017261092f565b60405161017f91906116c3565b60405180910390f35b6101a2600480360381019061019d91906116dc565b610953565b6040516101af9190611716565b60405180910390f35b6101c0610970565b6040516101cd919061174f565b60405180910390f35b6101de610994565b005b6101fa60048036038101906101f5919061180d565b6109a7565b005b610216600480360381019061021191906119cb565b6109bd565b005b610220610b94565b60405161022d9190611a62565b60405180910390f35b610250600480360381019061024b91906111c0565b610bbc565b60405161025d9190611a7b565b60405180910390f35b610280600480360381019061027b91906116dc565b610e0c565b005b6067602052805f5260405f205f91509050805f015f9054906101000a900463ffffffff1690805f0160049054906101000a900461ffff16905082565b60655f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16806103455750610316610b94565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16145b610384576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161037b90611b14565b60405180910390fd5b5f5f90505b81518110156108db575f8190505f6103a082610bbc565b90505f5f5f5f5f90505b8787815181106103bd576103bc611b32565b5b6020026020010151518160ff1610156107d1575f7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16635401ed278a8a815181106104205761041f611b32565b5b60200260200101518460ff168151811061043d5761043c611b32565b5b6020026020010151896040518363ffffffff1660e01b8152600401610463929190611b7d565b602060405180830381865afa15801561047e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104a29190611be5565b6bffffffffffffffffffffffff16905060655f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16801561053557505f60675f8960ff1660ff1681526020019081526020015f205f015f9054906101000a900463ffffffff1663ffffffff16115b801561054b57508581866105499190611c3d565b115b156105c05760665f8860ff1660ff1681526020019081526020015f20604051806040016040528042815260200187815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f015560208201518160010155505060019250506107d1565b80856105cc9190611c3d565b9450836105d890611c70565b93507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16636e3b17db7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663296bb0648c8c8151811061066557610664611b32565b5b60200260200101518660ff168151811061068257610681611b32565b5b60200260200101516040518263ffffffff1660e01b81526004016106a69190611c9b565b602060405180830381865afa1580156106c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106e59190611cc8565b896040516020016106f69190611d27565b6040516020818303038152906040526040518363ffffffff1660e01b8152600401610722929190611da1565b5f604051808303815f87803b158015610739575f5ffd5b505af115801561074b573d5f5f3e3d5ffd5b505050507f97ddb711c61a9d2d7effcba3e042a33862297f898d555655cca39ec4451f53b489898151811061078357610782611b32565b5b60200260200101518360ff16815181106107a05761079f611b32565b5b6020026020010151886040516107b7929190611b7d565b60405180910390a150806107ca90611dcf565b90506103aa565b5080158015610826575060655f3373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff165b156108925760665f8660ff1660ff1681526020019081526020015f20604051806040016040528042815260200185815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f01556020820151816001015550505b7f19dd87ae49ed14a795f8c2d5e8055bf2a4a9d01641a00a2f8f0a5a7bf7f7024982826040516108c3929190611df7565b60405180910390a15050505050806001019050610389565b5050565b6108e7610e8e565b6108f18282610f0c565b5050565b6066602052815f5260405f20818154811061090e575f80fd5b905f5260205f2090600202015f9150915050805f0154908060010154905082565b7f000000000000000000000000000000000000000000000000000000000000000081565b6065602052805f5260405f205f915054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b61099c610e8e565b6109a55f610f9d565b565b6109af610e8e565b6109b98282611060565b5050565b5f5f60019054906101000a900460ff161590508080156109ed575060015f5f9054906101000a900460ff1660ff16105b80610a1a57506109fc30611150565b158015610a19575060015f5f9054906101000a900460ff1660ff16145b5b610a59576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a5090611e8e565b60405180910390fd5b60015f5f6101000a81548160ff021916908360ff1602179055508015610a945760015f60016101000a81548160ff0219169083151502179055505b610a9d84610f9d565b5f5f90505b83518160ff161015610ae957610ad6848260ff1681518110610ac757610ac6611b32565b5b60200260200101516001610f0c565b8080610ae190611dcf565b915050610aa2565b505f5f90505b82518160ff161015610b3557610b2281848360ff1681518110610b1557610b14611b32565b5b6020026020010151611060565b8080610b2d90611dcf565b915050610aef565b508015610b8e575f5f60016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024986001604051610b859190611ee5565b60405180910390a15b50505050565b5f60335f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b5f5f61271061ffff167f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663d5eccc05856040518263ffffffff1660e01b8152600401610c1e9190611efe565b602060405180830381865afa158015610c39573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c5d9190611be5565b6bffffffffffffffffffffffff1660675f8660ff1660ff1681526020019081526020015f205f0160049054906101000a900461ffff1661ffff16610ca19190611f17565b610cab9190611f85565b90505f60665f8560ff1660ff1681526020019081526020015f208054905003610cd75780915050610e07565b5f60675f8560ff1660ff1681526020019081526020015f205f015f9054906101000a900463ffffffff1663ffffffff1642610d129190611fb5565b90505f5f90505f600160665f8860ff1660ff1681526020019081526020015f2080549050610d409190611fb5565b90505b8260665f8860ff1660ff1681526020019081526020015f208281548110610d6d57610d6c611b32565b5b905f5260205f2090600202015f01541115610de15760665f8760ff1660ff1681526020019081526020015f208181548110610dab57610daa611b32565b5b905f5260205f2090600202016001015482610dc69190611c3d565b91505f810315610de15780610dda90611fe8565b9050610d43565b838210610df4575f945050505050610e07565b8184610e009190611fb5565b9450505050505b919050565b610e14610e8e565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610e82576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e799061207f565b60405180910390fd5b610e8b81610f9d565b50565b610e96611172565b73ffffffffffffffffffffffffffffffffffffffff16610eb4610b94565b73ffffffffffffffffffffffffffffffffffffffff1614610f0a576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610f01906120e7565b60405180910390fd5b565b8060655f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055507f7676686b6d22e112412bd874d70177e011ab06602c26063f19f0386c9a3cee428282604051610f91929190612105565b60405180910390a15050565b5f60335f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690508160335f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a35050565b60c060ff168260ff16106110a9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016110a0906121c2565b60405180910390fd5b8060675f8460ff1660ff1681526020019081526020015f205f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548161ffff021916908361ffff1602179055509050507fe69c2827a1e2fdd32265ebb4eeea5ee564f0551cf5dfed4150f8e116a67209eb82825f01518360200151604051611144939291906121e0565b60405180910390a15050565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f33905090565b5f604051905090565b5f5ffd5b5f5ffd5b5f60ff82169050919050565b61119f8161118a565b81146111a9575f5ffd5b50565b5f813590506111ba81611196565b92915050565b5f602082840312156111d5576111d4611182565b5b5f6111e2848285016111ac565b91505092915050565b5f63ffffffff82169050919050565b611203816111eb565b82525050565b5f61ffff82169050919050565b61121f81611209565b82525050565b5f6040820190506112385f8301856111fa565b6112456020830184611216565b9392505050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61129682611250565b810181811067ffffffffffffffff821117156112b5576112b4611260565b5b80604052505050565b5f6112c7611179565b90506112d3828261128d565b919050565b5f67ffffffffffffffff8211156112f2576112f1611260565b5b602082029050602081019050919050565b5f5ffd5b5f67ffffffffffffffff82111561132157611320611260565b5b602082029050602081019050919050565b5f819050919050565b61134481611332565b811461134e575f5ffd5b50565b5f8135905061135f8161133b565b92915050565b5f61137761137284611307565b6112be565b9050808382526020820190506020840283018581111561139a57611399611303565b5b835b818110156113c357806113af8882611351565b84526020840193505060208101905061139c565b5050509392505050565b5f82601f8301126113e1576113e061124c565b5b81356113f1848260208601611365565b91505092915050565b5f61140c611407846112d8565b6112be565b9050808382526020820190506020840283018581111561142f5761142e611303565b5b835b8181101561147657803567ffffffffffffffff8111156114545761145361124c565b5b80860161146189826113cd565b85526020850194505050602081019050611431565b5050509392505050565b5f82601f8301126114945761149361124c565b5b81356114a48482602086016113fa565b91505092915050565b5f602082840312156114c2576114c1611182565b5b5f82013567ffffffffffffffff8111156114df576114de611186565b5b6114eb84828501611480565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61151d826114f4565b9050919050565b61152d81611513565b8114611537575f5ffd5b50565b5f8135905061154881611524565b92915050565b5f8115159050919050565b6115628161154e565b811461156c575f5ffd5b50565b5f8135905061157d81611559565b92915050565b5f5f6040838503121561159957611598611182565b5b5f6115a68582860161153a565b92505060206115b78582860161156f565b9150509250929050565b5f819050919050565b6115d3816115c1565b81146115dd575f5ffd5b50565b5f813590506115ee816115ca565b92915050565b5f5f6040838503121561160a57611609611182565b5b5f611617858286016111ac565b9250506020611628858286016115e0565b9150509250929050565b61163b816115c1565b82525050565b5f6040820190506116545f830185611632565b6116616020830184611632565b9392505050565b5f819050919050565b5f61168b611686611681846114f4565b611668565b6114f4565b9050919050565b5f61169c82611671565b9050919050565b5f6116ad82611692565b9050919050565b6116bd816116a3565b82525050565b5f6020820190506116d65f8301846116b4565b92915050565b5f602082840312156116f1576116f0611182565b5b5f6116fe8482850161153a565b91505092915050565b6117108161154e565b82525050565b5f6020820190506117295f830184611707565b92915050565b5f61173982611692565b9050919050565b6117498161172f565b82525050565b5f6020820190506117625f830184611740565b92915050565b5f5ffd5b611775816111eb565b811461177f575f5ffd5b50565b5f813590506117908161176c565b92915050565b61179f81611209565b81146117a9575f5ffd5b50565b5f813590506117ba81611796565b92915050565b5f604082840312156117d5576117d4611768565b5b6117df60406112be565b90505f6117ee84828501611782565b5f830152506020611801848285016117ac565b60208301525092915050565b5f5f6060838503121561182357611822611182565b5b5f611830858286016111ac565b9250506020611841858286016117c0565b9150509250929050565b5f67ffffffffffffffff82111561186557611864611260565b5b602082029050602081019050919050565b5f6118886118838461184b565b6112be565b905080838252602082019050602084028301858111156118ab576118aa611303565b5b835b818110156118d457806118c0888261153a565b8452602084019350506020810190506118ad565b5050509392505050565b5f82601f8301126118f2576118f161124c565b5b8135611902848260208601611876565b91505092915050565b5f67ffffffffffffffff82111561192557611924611260565b5b602082029050602081019050919050565b5f6119486119438461190b565b6112be565b9050808382526020820190506040840283018581111561196b5761196a611303565b5b835b81811015611994578061198088826117c0565b84526020840193505060408101905061196d565b5050509392505050565b5f82601f8301126119b2576119b161124c565b5b81356119c2848260208601611936565b91505092915050565b5f5f5f606084860312156119e2576119e1611182565b5b5f6119ef8682870161153a565b935050602084013567ffffffffffffffff811115611a1057611a0f611186565b5b611a1c868287016118de565b925050604084013567ffffffffffffffff811115611a3d57611a3c611186565b5b611a498682870161199e565b9150509250925092565b611a5c81611513565b82525050565b5f602082019050611a755f830184611a53565b92915050565b5f602082019050611a8e5f830184611632565b92915050565b5f82825260208201905092915050565b7f456a656374696f6e4d616e616765722e656a6563744f70657261746f72733a205f8201527f4f6e6c79206f776e6572206f7220656a6563746f722063616e20656a65637400602082015250565b5f611afe603f83611a94565b9150611b0982611aa4565b604082019050919050565b5f6020820190508181035f830152611b2b81611af2565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b611b6881611332565b82525050565b611b778161118a565b82525050565b5f604082019050611b905f830185611b5f565b611b9d6020830184611b6e565b9392505050565b5f6bffffffffffffffffffffffff82169050919050565b611bc481611ba4565b8114611bce575f5ffd5b50565b5f81519050611bdf81611bbb565b92915050565b5f60208284031215611bfa57611bf9611182565b5b5f611c0784828501611bd1565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f611c47826115c1565b9150611c52836115c1565b9250828201905080821115611c6a57611c69611c10565b5b92915050565b5f611c7a826111eb565b915063ffffffff8203611c9057611c8f611c10565b5b600182019050919050565b5f602082019050611cae5f830184611b5f565b92915050565b5f81519050611cc281611524565b92915050565b5f60208284031215611cdd57611cdc611182565b5b5f611cea84828501611cb4565b91505092915050565b5f8160f81b9050919050565b5f611d0982611cf3565b9050919050565b611d21611d1c8261118a565b611cff565b82525050565b5f611d328284611d10565b60018201915081905092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f611d7382611d41565b611d7d8185611d4b565b9350611d8d818560208601611d5b565b611d9681611250565b840191505092915050565b5f604082019050611db45f830185611a53565b8181036020830152611dc68184611d69565b90509392505050565b5f611dd98261118a565b915060ff8203611dec57611deb611c10565b5b600182019050919050565b5f604082019050611e0a5f8301856111fa565b611e176020830184611707565b9392505050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c7265615f8201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b5f611e78602e83611a94565b9150611e8382611e1e565b604082019050919050565b5f6020820190508181035f830152611ea581611e6c565b9050919050565b5f819050919050565b5f611ecf611eca611ec584611eac565b611668565b61118a565b9050919050565b611edf81611eb5565b82525050565b5f602082019050611ef85f830184611ed6565b92915050565b5f602082019050611f115f830184611b6e565b92915050565b5f611f21826115c1565b9150611f2c836115c1565b9250828202611f3a816115c1565b91508282048414831517611f5157611f50611c10565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f611f8f826115c1565b9150611f9a836115c1565b925082611faa57611fa9611f58565b5b828204905092915050565b5f611fbf826115c1565b9150611fca836115c1565b9250828203905081811115611fe257611fe1611c10565b5b92915050565b5f611ff2826115c1565b91505f820361200457612003611c10565b5b600182039050919050565b7f4f776e61626c653a206e6577206f776e657220697320746865207a65726f20615f8201527f6464726573730000000000000000000000000000000000000000000000000000602082015250565b5f612069602683611a94565b91506120748261200f565b604082019050919050565b5f6020820190508181035f8301526120968161205d565b9050919050565b7f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65725f82015250565b5f6120d1602083611a94565b91506120dc8261209d565b602082019050919050565b5f6020820190508181035f8301526120fe816120c5565b9050919050565b5f6040820190506121185f830185611a53565b6121256020830184611707565b9392505050565b7f456a656374696f6e4d616e616765722e5f73657451756f72756d456a656374695f8201527f6f6e506172616d733a2051756f72756d206e756d62657220657863656564732060208201527f4d41585f51554f52554d5f434f554e5400000000000000000000000000000000604082015250565b5f6121ac605083611a94565b91506121b78261212c565b606082019050919050565b5f6020820190508181035f8301526121d9816121a0565b9050919050565b5f6060820190506121f35f830186611b6e565b61220060208301856111fa565b61220d6040830184611216565b94935050505056fea26469706673582212203f1416cc27db2ac09b283ce5560051b9b06291eda425ecd755504ab44c05453464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCCW_5`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\x8AW\x80c\x8B\x88\xA0$\x11a\0dW\x80c\x8B\x88\xA0$\x14a\x01\xFCW\x80c\x8D\xA5\xCB[\x14a\x02\x18W\x80c\xB1?E\x04\x14a\x026W\x80c\xF2\xFD\xE3\x8B\x14a\x02fWa\0\xCCV[\x80cm\x14\xA9\x87\x14a\x01\xB8W\x80cqP\x18\xA6\x14a\x01\xD6W\x80cw\xD1u\x86\x14a\x01\xE0Wa\0\xCCV[\x80bH%i\x14a\0\xD0W\x80c\n\x05\x93\xD1\x14a\x01\x01W\x80c\x10\xEAO\x8A\x14a\x01\x1DW\x80c:\x0B\r\xDD\x14a\x019W\x80ch0H5\x14a\x01jW\x80cl\x08\xA8y\x14a\x01\x88W[__\xFD[a\0\xEA`\x04\x806\x03\x81\x01\x90a\0\xE5\x91\x90a\x11\xC0V[a\x02\x82V[`@Qa\0\xF8\x92\x91\x90a\x12%V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1B`\x04\x806\x03\x81\x01\x90a\x01\x16\x91\x90a\x14\xADV[a\x02\xBEV[\0[a\x017`\x04\x806\x03\x81\x01\x90a\x012\x91\x90a\x15\x83V[a\x08\xDFV[\0[a\x01S`\x04\x806\x03\x81\x01\x90a\x01N\x91\x90a\x15\xF4V[a\x08\xF5V[`@Qa\x01a\x92\x91\x90a\x16AV[`@Q\x80\x91\x03\x90\xF3[a\x01ra\t/V[`@Qa\x01\x7F\x91\x90a\x16\xC3V[`@Q\x80\x91\x03\x90\xF3[a\x01\xA2`\x04\x806\x03\x81\x01\x90a\x01\x9D\x91\x90a\x16\xDCV[a\tSV[`@Qa\x01\xAF\x91\x90a\x17\x16V[`@Q\x80\x91\x03\x90\xF3[a\x01\xC0a\tpV[`@Qa\x01\xCD\x91\x90a\x17OV[`@Q\x80\x91\x03\x90\xF3[a\x01\xDEa\t\x94V[\0[a\x01\xFA`\x04\x806\x03\x81\x01\x90a\x01\xF5\x91\x90a\x18\rV[a\t\xA7V[\0[a\x02\x16`\x04\x806\x03\x81\x01\x90a\x02\x11\x91\x90a\x19\xCBV[a\t\xBDV[\0[a\x02 a\x0B\x94V[`@Qa\x02-\x91\x90a\x1AbV[`@Q\x80\x91\x03\x90\xF3[a\x02P`\x04\x806\x03\x81\x01\x90a\x02K\x91\x90a\x11\xC0V[a\x0B\xBCV[`@Qa\x02]\x91\x90a\x1A{V[`@Q\x80\x91\x03\x90\xF3[a\x02\x80`\x04\x806\x03\x81\x01\x90a\x02{\x91\x90a\x16\xDCV[a\x0E\x0CV[\0[`g` R\x80_R`@_ _\x91P\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x04\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16\x90P\x82V[`e_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x03EWPa\x03\x16a\x0B\x94V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[a\x03\x84W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03{\x90a\x1B\x14V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x81Q\x81\x10\x15a\x08\xDBW_\x81\x90P_a\x03\xA0\x82a\x0B\xBCV[\x90P_____\x90P[\x87\x87\x81Q\x81\x10a\x03\xBDWa\x03\xBCa\x1B2V[[` \x02` \x01\x01QQ\x81`\xFF\x16\x10\x15a\x07\xD1W_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cT\x01\xED'\x8A\x8A\x81Q\x81\x10a\x04 Wa\x04\x1Fa\x1B2V[[` \x02` \x01\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x04=Wa\x04<a\x1B2V[[` \x02` \x01\x01Q\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04c\x92\x91\x90a\x1B}V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xA2\x91\x90a\x1B\xE5V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P`e_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80\x15a\x055WP_`g_\x89`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11[\x80\x15a\x05KWP\x85\x81\x86a\x05I\x91\x90a\x1C=V[\x11[\x15a\x05\xC0W`f_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80B\x81R` \x01\x87\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01UPP`\x01\x92PPa\x07\xD1V[\x80\x85a\x05\xCC\x91\x90a\x1C=V[\x94P\x83a\x05\xD8\x90a\x1CpV[\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cn;\x17\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c)k\xB0d\x8C\x8C\x81Q\x81\x10a\x06eWa\x06da\x1B2V[[` \x02` \x01\x01Q\x86`\xFF\x16\x81Q\x81\x10a\x06\x82Wa\x06\x81a\x1B2V[[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xA6\x91\x90a\x1C\x9BV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE5\x91\x90a\x1C\xC8V[\x89`@Q` \x01a\x06\xF6\x91\x90a\x1D'V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\"\x92\x91\x90a\x1D\xA1V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x079W__\xFD[PZ\xF1\x15\x80\x15a\x07KW=__>=_\xFD[PPPP\x7F\x97\xDD\xB7\x11\xC6\x1A\x9D-~\xFF\xCB\xA3\xE0B\xA38b)\x7F\x89\x8DUVU\xCC\xA3\x9E\xC4E\x1FS\xB4\x89\x89\x81Q\x81\x10a\x07\x83Wa\x07\x82a\x1B2V[[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x07\xA0Wa\x07\x9Fa\x1B2V[[` \x02` \x01\x01Q\x88`@Qa\x07\xB7\x92\x91\x90a\x1B}V[`@Q\x80\x91\x03\x90\xA1P\x80a\x07\xCA\x90a\x1D\xCFV[\x90Pa\x03\xAAV[P\x80\x15\x80\x15a\x08&WP`e_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16[\x15a\x08\x92W`f_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80B\x81R` \x01\x85\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01UPP[\x7F\x19\xDD\x87\xAEI\xED\x14\xA7\x95\xF8\xC2\xD5\xE8\x05[\xF2\xA4\xA9\xD0\x16A\xA0\n/\x8F\nZ{\xF7\xF7\x02I\x82\x82`@Qa\x08\xC3\x92\x91\x90a\x1D\xF7V[`@Q\x80\x91\x03\x90\xA1PPPPP\x80`\x01\x01\x90Pa\x03\x89V[PPV[a\x08\xE7a\x0E\x8EV[a\x08\xF1\x82\x82a\x0F\x0CV[PPV[`f` R\x81_R`@_ \x81\x81T\x81\x10a\t\x0EW_\x80\xFD[\x90_R` _ \x90`\x02\x02\x01_\x91P\x91PP\x80_\x01T\x90\x80`\x01\x01T\x90P\x82V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`e` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\t\x9Ca\x0E\x8EV[a\t\xA5_a\x0F\x9DV[V[a\t\xAFa\x0E\x8EV[a\t\xB9\x82\x82a\x10`V[PPV[__`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x90P\x80\x80\x15a\t\xEDWP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[\x80a\n\x1AWPa\t\xFC0a\x11PV[\x15\x80\x15a\n\x19WP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x14[[a\nYW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nP\x90a\x1E\x8EV[`@Q\x80\x91\x03\x90\xFD[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x15a\n\x94W`\x01_`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[a\n\x9D\x84a\x0F\x9DV[__\x90P[\x83Q\x81`\xFF\x16\x10\x15a\n\xE9Wa\n\xD6\x84\x82`\xFF\x16\x81Q\x81\x10a\n\xC7Wa\n\xC6a\x1B2V[[` \x02` \x01\x01Q`\x01a\x0F\x0CV[\x80\x80a\n\xE1\x90a\x1D\xCFV[\x91PPa\n\xA2V[P__\x90P[\x82Q\x81`\xFF\x16\x10\x15a\x0B5Wa\x0B\"\x81\x84\x83`\xFF\x16\x81Q\x81\x10a\x0B\x15Wa\x0B\x14a\x1B2V[[` \x02` \x01\x01Qa\x10`V[\x80\x80a\x0B-\x90a\x1D\xCFV[\x91PPa\n\xEFV[P\x80\x15a\x0B\x8EW__`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\x01`@Qa\x0B\x85\x91\x90a\x1E\xE5V[`@Q\x80\x91\x03\x90\xA1[PPPPV[_`3_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[__a'\x10a\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5\xEC\xCC\x05\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x1E\x91\x90a\x1E\xFEV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C]\x91\x90a\x1B\xE5V[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`g_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01`\x04\x90T\x90a\x01\0\n\x90\x04a\xFF\xFF\x16a\xFF\xFF\x16a\x0C\xA1\x91\x90a\x1F\x17V[a\x0C\xAB\x91\x90a\x1F\x85V[\x90P_`f_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x03a\x0C\xD7W\x80\x91PPa\x0E\x07V[_`g_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16Ba\r\x12\x91\x90a\x1F\xB5V[\x90P__\x90P_`\x01`f_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa\r@\x91\x90a\x1F\xB5V[\x90P[\x82`f_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\rmWa\rla\x1B2V[[\x90_R` _ \x90`\x02\x02\x01_\x01T\x11\x15a\r\xE1W`f_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a\r\xABWa\r\xAAa\x1B2V[[\x90_R` _ \x90`\x02\x02\x01`\x01\x01T\x82a\r\xC6\x91\x90a\x1C=V[\x91P_\x81\x03\x15a\r\xE1W\x80a\r\xDA\x90a\x1F\xE8V[\x90Pa\rCV[\x83\x82\x10a\r\xF4W_\x94PPPPPa\x0E\x07V[\x81\x84a\x0E\0\x91\x90a\x1F\xB5V[\x94PPPPP[\x91\x90PV[a\x0E\x14a\x0E\x8EV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0E\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0Ey\x90a \x7FV[`@Q\x80\x91\x03\x90\xFD[a\x0E\x8B\x81a\x0F\x9DV[PV[a\x0E\x96a\x11rV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0E\xB4a\x0B\x94V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x01\x90a \xE7V[`@Q\x80\x91\x03\x90\xFD[V[\x80`e_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Fvvhkm\"\xE1\x12A+\xD8t\xD7\x01w\xE0\x11\xAB\x06`,&\x06?\x19\xF08l\x9A<\xEEB\x82\x82`@Qa\x0F\x91\x92\x91\x90a!\x05V[`@Q\x80\x91\x03\x90\xA1PPV[_`3_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`3_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\xC0`\xFF\x16\x82`\xFF\x16\x10a\x10\xA9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xA0\x90a!\xC2V[`@Q\x80\x91\x03\x90\xFD[\x80`g_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UP\x90PP\x7F\xE6\x9C('\xA1\xE2\xFD\xD3\"e\xEB\xB4\xEE\xEA^\xE5d\xF0U\x1C\xF5\xDF\xEDAP\xF8\xE1\x16\xA6r\t\xEB\x82\x82_\x01Q\x83` \x01Q`@Qa\x11D\x93\x92\x91\x90a!\xE0V[`@Q\x80\x91\x03\x90\xA1PPV[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_3\x90P\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a\x11\x9F\x81a\x11\x8AV[\x81\x14a\x11\xA9W__\xFD[PV[_\x815\x90Pa\x11\xBA\x81a\x11\x96V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x11\xD5Wa\x11\xD4a\x11\x82V[[_a\x11\xE2\x84\x82\x85\x01a\x11\xACV[\x91PP\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x12\x03\x81a\x11\xEBV[\x82RPPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x12\x1F\x81a\x12\tV[\x82RPPV[_`@\x82\x01\x90Pa\x128_\x83\x01\x85a\x11\xFAV[a\x12E` \x83\x01\x84a\x12\x16V[\x93\x92PPPV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x12\x96\x82a\x12PV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12\xB5Wa\x12\xB4a\x12`V[[\x80`@RPPPV[_a\x12\xC7a\x11yV[\x90Pa\x12\xD3\x82\x82a\x12\x8DV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12\xF2Wa\x12\xF1a\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13!Wa\x13 a\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x13D\x81a\x132V[\x81\x14a\x13NW__\xFD[PV[_\x815\x90Pa\x13_\x81a\x13;V[\x92\x91PPV[_a\x13wa\x13r\x84a\x13\x07V[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x13\x9AWa\x13\x99a\x13\x03V[[\x83[\x81\x81\x10\x15a\x13\xC3W\x80a\x13\xAF\x88\x82a\x13QV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x13\x9CV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x13\xE1Wa\x13\xE0a\x12LV[[\x815a\x13\xF1\x84\x82` \x86\x01a\x13eV[\x91PP\x92\x91PPV[_a\x14\x0Ca\x14\x07\x84a\x12\xD8V[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x14/Wa\x14.a\x13\x03V[[\x83[\x81\x81\x10\x15a\x14vW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14TWa\x14Sa\x12LV[[\x80\x86\x01a\x14a\x89\x82a\x13\xCDV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\x141V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x14\x94Wa\x14\x93a\x12LV[[\x815a\x14\xA4\x84\x82` \x86\x01a\x13\xFAV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xC2Wa\x14\xC1a\x11\x82V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xDFWa\x14\xDEa\x11\x86V[[a\x14\xEB\x84\x82\x85\x01a\x14\x80V[\x91PP\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x15\x1D\x82a\x14\xF4V[\x90P\x91\x90PV[a\x15-\x81a\x15\x13V[\x81\x14a\x157W__\xFD[PV[_\x815\x90Pa\x15H\x81a\x15$V[\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a\x15b\x81a\x15NV[\x81\x14a\x15lW__\xFD[PV[_\x815\x90Pa\x15}\x81a\x15YV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x15\x99Wa\x15\x98a\x11\x82V[[_a\x15\xA6\x85\x82\x86\x01a\x15:V[\x92PP` a\x15\xB7\x85\x82\x86\x01a\x15oV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a\x15\xD3\x81a\x15\xC1V[\x81\x14a\x15\xDDW__\xFD[PV[_\x815\x90Pa\x15\xEE\x81a\x15\xCAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x16\nWa\x16\ta\x11\x82V[[_a\x16\x17\x85\x82\x86\x01a\x11\xACV[\x92PP` a\x16(\x85\x82\x86\x01a\x15\xE0V[\x91PP\x92P\x92\x90PV[a\x16;\x81a\x15\xC1V[\x82RPPV[_`@\x82\x01\x90Pa\x16T_\x83\x01\x85a\x162V[a\x16a` \x83\x01\x84a\x162V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x16\x8Ba\x16\x86a\x16\x81\x84a\x14\xF4V[a\x16hV[a\x14\xF4V[\x90P\x91\x90PV[_a\x16\x9C\x82a\x16qV[\x90P\x91\x90PV[_a\x16\xAD\x82a\x16\x92V[\x90P\x91\x90PV[a\x16\xBD\x81a\x16\xA3V[\x82RPPV[_` \x82\x01\x90Pa\x16\xD6_\x83\x01\x84a\x16\xB4V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x16\xF1Wa\x16\xF0a\x11\x82V[[_a\x16\xFE\x84\x82\x85\x01a\x15:V[\x91PP\x92\x91PPV[a\x17\x10\x81a\x15NV[\x82RPPV[_` \x82\x01\x90Pa\x17)_\x83\x01\x84a\x17\x07V[\x92\x91PPV[_a\x179\x82a\x16\x92V[\x90P\x91\x90PV[a\x17I\x81a\x17/V[\x82RPPV[_` \x82\x01\x90Pa\x17b_\x83\x01\x84a\x17@V[\x92\x91PPV[__\xFD[a\x17u\x81a\x11\xEBV[\x81\x14a\x17\x7FW__\xFD[PV[_\x815\x90Pa\x17\x90\x81a\x17lV[\x92\x91PPV[a\x17\x9F\x81a\x12\tV[\x81\x14a\x17\xA9W__\xFD[PV[_\x815\x90Pa\x17\xBA\x81a\x17\x96V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x17\xD5Wa\x17\xD4a\x17hV[[a\x17\xDF`@a\x12\xBEV[\x90P_a\x17\xEE\x84\x82\x85\x01a\x17\x82V[_\x83\x01RP` a\x18\x01\x84\x82\x85\x01a\x17\xACV[` \x83\x01RP\x92\x91PPV[__``\x83\x85\x03\x12\x15a\x18#Wa\x18\"a\x11\x82V[[_a\x180\x85\x82\x86\x01a\x11\xACV[\x92PP` a\x18A\x85\x82\x86\x01a\x17\xC0V[\x91PP\x92P\x92\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18eWa\x18da\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a\x18\x88a\x18\x83\x84a\x18KV[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x18\xABWa\x18\xAAa\x13\x03V[[\x83[\x81\x81\x10\x15a\x18\xD4W\x80a\x18\xC0\x88\x82a\x15:V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x18\xADV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x18\xF2Wa\x18\xF1a\x12LV[[\x815a\x19\x02\x84\x82` \x86\x01a\x18vV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19%Wa\x19$a\x12`V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a\x19Ha\x19C\x84a\x19\x0BV[a\x12\xBEV[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15a\x19kWa\x19ja\x13\x03V[[\x83[\x81\x81\x10\x15a\x19\x94W\x80a\x19\x80\x88\x82a\x17\xC0V[\x84R` \x84\x01\x93PP`@\x81\x01\x90Pa\x19mV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x19\xB2Wa\x19\xB1a\x12LV[[\x815a\x19\xC2\x84\x82` \x86\x01a\x196V[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x19\xE2Wa\x19\xE1a\x11\x82V[[_a\x19\xEF\x86\x82\x87\x01a\x15:V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x10Wa\x1A\x0Fa\x11\x86V[[a\x1A\x1C\x86\x82\x87\x01a\x18\xDEV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A=Wa\x1A<a\x11\x86V[[a\x1AI\x86\x82\x87\x01a\x19\x9EV[\x91PP\x92P\x92P\x92V[a\x1A\\\x81a\x15\x13V[\x82RPPV[_` \x82\x01\x90Pa\x1Au_\x83\x01\x84a\x1ASV[\x92\x91PPV[_` \x82\x01\x90Pa\x1A\x8E_\x83\x01\x84a\x162V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FEjectionManager.ejectOperators: _\x82\x01R\x7FOnly owner or ejector can eject\0` \x82\x01RPV[_a\x1A\xFE`?\x83a\x1A\x94V[\x91Pa\x1B\t\x82a\x1A\xA4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1B+\x81a\x1A\xF2V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[a\x1Bh\x81a\x132V[\x82RPPV[a\x1Bw\x81a\x11\x8AV[\x82RPPV[_`@\x82\x01\x90Pa\x1B\x90_\x83\x01\x85a\x1B_V[a\x1B\x9D` \x83\x01\x84a\x1BnV[\x93\x92PPPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x1B\xC4\x81a\x1B\xA4V[\x81\x14a\x1B\xCEW__\xFD[PV[_\x81Q\x90Pa\x1B\xDF\x81a\x1B\xBBV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1B\xFAWa\x1B\xF9a\x11\x82V[[_a\x1C\x07\x84\x82\x85\x01a\x1B\xD1V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x1CG\x82a\x15\xC1V[\x91Pa\x1CR\x83a\x15\xC1V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x1CjWa\x1Cia\x1C\x10V[[\x92\x91PPV[_a\x1Cz\x82a\x11\xEBV[\x91Pc\xFF\xFF\xFF\xFF\x82\x03a\x1C\x90Wa\x1C\x8Fa\x1C\x10V[[`\x01\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90Pa\x1C\xAE_\x83\x01\x84a\x1B_V[\x92\x91PPV[_\x81Q\x90Pa\x1C\xC2\x81a\x15$V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1C\xDDWa\x1C\xDCa\x11\x82V[[_a\x1C\xEA\x84\x82\x85\x01a\x1C\xB4V[\x91PP\x92\x91PPV[_\x81`\xF8\x1B\x90P\x91\x90PV[_a\x1D\t\x82a\x1C\xF3V[\x90P\x91\x90PV[a\x1D!a\x1D\x1C\x82a\x11\x8AV[a\x1C\xFFV[\x82RPPV[_a\x1D2\x82\x84a\x1D\x10V[`\x01\x82\x01\x91P\x81\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x1Ds\x82a\x1DAV[a\x1D}\x81\x85a\x1DKV[\x93Pa\x1D\x8D\x81\x85` \x86\x01a\x1D[V[a\x1D\x96\x81a\x12PV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90Pa\x1D\xB4_\x83\x01\x85a\x1ASV[\x81\x81\x03` \x83\x01Ra\x1D\xC6\x81\x84a\x1DiV[\x90P\x93\x92PPPV[_a\x1D\xD9\x82a\x11\x8AV[\x91P`\xFF\x82\x03a\x1D\xECWa\x1D\xEBa\x1C\x10V[[`\x01\x82\x01\x90P\x91\x90PV[_`@\x82\x01\x90Pa\x1E\n_\x83\x01\x85a\x11\xFAV[a\x1E\x17` \x83\x01\x84a\x17\x07V[\x93\x92PPPV[\x7FInitializable: contract is alrea_\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1Ex`.\x83a\x1A\x94V[\x91Pa\x1E\x83\x82a\x1E\x1EV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1E\xA5\x81a\x1ElV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x1E\xCFa\x1E\xCAa\x1E\xC5\x84a\x1E\xACV[a\x16hV[a\x11\x8AV[\x90P\x91\x90PV[a\x1E\xDF\x81a\x1E\xB5V[\x82RPPV[_` \x82\x01\x90Pa\x1E\xF8_\x83\x01\x84a\x1E\xD6V[\x92\x91PPV[_` \x82\x01\x90Pa\x1F\x11_\x83\x01\x84a\x1BnV[\x92\x91PPV[_a\x1F!\x82a\x15\xC1V[\x91Pa\x1F,\x83a\x15\xC1V[\x92P\x82\x82\x02a\x1F:\x81a\x15\xC1V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a\x1FQWa\x1FPa\x1C\x10V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a\x1F\x8F\x82a\x15\xC1V[\x91Pa\x1F\x9A\x83a\x15\xC1V[\x92P\x82a\x1F\xAAWa\x1F\xA9a\x1FXV[[\x82\x82\x04\x90P\x92\x91PPV[_a\x1F\xBF\x82a\x15\xC1V[\x91Pa\x1F\xCA\x83a\x15\xC1V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1F\xE2Wa\x1F\xE1a\x1C\x10V[[\x92\x91PPV[_a\x1F\xF2\x82a\x15\xC1V[\x91P_\x82\x03a \x04Wa \x03a\x1C\x10V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FOwnable: new owner is the zero a_\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a i`&\x83a\x1A\x94V[\x91Pa t\x82a \x0FV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra \x96\x81a ]V[\x90P\x91\x90PV[\x7FOwnable: caller is not the owner_\x82\x01RPV[_a \xD1` \x83a\x1A\x94V[\x91Pa \xDC\x82a \x9DV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra \xFE\x81a \xC5V[\x90P\x91\x90PV[_`@\x82\x01\x90Pa!\x18_\x83\x01\x85a\x1ASV[a!%` \x83\x01\x84a\x17\x07V[\x93\x92PPPV[\x7FEjectionManager._setQuorumEjecti_\x82\x01R\x7FonParams: Quorum number exceeds ` \x82\x01R\x7FMAX_QUORUM_COUNT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a!\xAC`P\x83a\x1A\x94V[\x91Pa!\xB7\x82a!,V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra!\xD9\x81a!\xA0V[\x90P\x91\x90PV[_``\x82\x01\x90Pa!\xF3_\x83\x01\x86a\x1BnV[a\"\0` \x83\x01\x85a\x11\xFAV[a\"\r`@\x83\x01\x84a\x12\x16V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 ?\x14\x16\xCC'\xDB*\xC0\x9B(<\xE5V\0Q\xB9\xB0b\x91\xED\xA4%\xEC\xD7UPJ\xB4L\x05E4dsolcC\0\x08\x1B\x003",
    );
    /**Event with signature `EjectorUpdated(address,bool)` and selector `0x7676686b6d22e112412bd874d70177e011ab06602c26063f19f0386c9a3cee42`.
```solidity
event EjectorUpdated(address ejector, bool status);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EjectorUpdated {
        #[allow(missing_docs)]
        pub ejector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub status: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EjectorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectorUpdated(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                118u8,
                118u8,
                104u8,
                107u8,
                109u8,
                34u8,
                225u8,
                18u8,
                65u8,
                43u8,
                216u8,
                116u8,
                215u8,
                1u8,
                119u8,
                224u8,
                17u8,
                171u8,
                6u8,
                96u8,
                44u8,
                38u8,
                6u8,
                63u8,
                25u8,
                240u8,
                56u8,
                108u8,
                154u8,
                60u8,
                238u8,
                66u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    ejector: data.0,
                    status: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ejector,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EjectorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EjectorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EjectorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorEjected(bytes32,uint8)` and selector `0x97ddb711c61a9d2d7effcba3e042a33862297f898d555655cca39ec4451f53b4`.
```solidity
event OperatorEjected(bytes32 operatorId, uint8 quorumNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorEjected {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorEjected {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorEjected(bytes32,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                151u8,
                221u8,
                183u8,
                17u8,
                198u8,
                26u8,
                157u8,
                45u8,
                126u8,
                255u8,
                203u8,
                163u8,
                224u8,
                66u8,
                163u8,
                56u8,
                98u8,
                41u8,
                127u8,
                137u8,
                141u8,
                85u8,
                86u8,
                85u8,
                204u8,
                163u8,
                158u8,
                196u8,
                69u8,
                31u8,
                83u8,
                180u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorId: data.0,
                    quorumNumber: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorEjected {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorEjected> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorEjected) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumEjection(uint32,bool)` and selector `0x19dd87ae49ed14a795f8c2d5e8055bf2a4a9d01641a00a2f8f0a5a7bf7f70249`.
```solidity
event QuorumEjection(uint32 ejectedOperators, bool ratelimitHit);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumEjection {
        #[allow(missing_docs)]
        pub ejectedOperators: u32,
        #[allow(missing_docs)]
        pub ratelimitHit: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumEjection {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumEjection(uint32,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                25u8,
                221u8,
                135u8,
                174u8,
                73u8,
                237u8,
                20u8,
                167u8,
                149u8,
                248u8,
                194u8,
                213u8,
                232u8,
                5u8,
                91u8,
                242u8,
                164u8,
                169u8,
                208u8,
                22u8,
                65u8,
                160u8,
                10u8,
                47u8,
                143u8,
                10u8,
                90u8,
                123u8,
                247u8,
                247u8,
                2u8,
                73u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    ejectedOperators: data.0,
                    ratelimitHit: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ejectedOperators),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.ratelimitHit,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumEjection {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumEjection> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumEjection) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumEjectionParamsSet(uint8,uint32,uint16)` and selector `0xe69c2827a1e2fdd32265ebb4eeea5ee564f0551cf5dfed4150f8e116a67209eb`.
```solidity
event QuorumEjectionParamsSet(uint8 quorumNumber, uint32 rateLimitWindow, uint16 ejectableStakePercent);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumEjectionParamsSet {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub rateLimitWindow: u32,
        #[allow(missing_docs)]
        pub ejectableStakePercent: u16,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumEjectionParamsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumEjectionParamsSet(uint8,uint32,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                230u8,
                156u8,
                40u8,
                39u8,
                161u8,
                226u8,
                253u8,
                211u8,
                34u8,
                101u8,
                235u8,
                180u8,
                238u8,
                234u8,
                94u8,
                229u8,
                100u8,
                240u8,
                85u8,
                28u8,
                245u8,
                223u8,
                237u8,
                65u8,
                80u8,
                248u8,
                225u8,
                22u8,
                166u8,
                114u8,
                9u8,
                235u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: data.0,
                    rateLimitWindow: data.1,
                    ejectableStakePercent: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rateLimitWindow),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.ejectableStakePercent),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumEjectionParamsSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumEjectionParamsSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &QuorumEjectionParamsSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _registryCoordinator, address _stakeRegistry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
        pub _stakeRegistry: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._registryCoordinator, value._stakeRegistry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _stakeRegistry: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                )
            }
        }
    };
    /**Function with signature `amountEjectableForQuorum(uint8)` and selector `0xb13f4504`.
```solidity
function amountEjectableForQuorum(uint8 _quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct amountEjectableForQuorumCall {
        pub _quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`amountEjectableForQuorum(uint8)`](amountEjectableForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct amountEjectableForQuorumReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<amountEjectableForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: amountEjectableForQuorumCall) -> Self {
                    (value._quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for amountEjectableForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<amountEjectableForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: amountEjectableForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for amountEjectableForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for amountEjectableForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = amountEjectableForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "amountEjectableForQuorum(uint8)";
            const SELECTOR: [u8; 4] = [177u8, 63u8, 69u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._quorumNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `ejectOperators(bytes32[][])` and selector `0x0a0593d1`.
```solidity
function ejectOperators(bytes32[][] memory _operatorIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorsCall {
        pub _operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
    }
    ///Container type for the return parameters of the [`ejectOperators(bytes32[][])`](ejectOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ejectOperatorsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::FixedBytes<32>,
                    >,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorsCall) -> Self {
                    (value._operatorIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operatorIds: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ejectOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectOperatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ejectOperators(bytes32[][])";
            const SELECTOR: [u8; 4] = [10u8, 5u8, 147u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self._operatorIds),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address[],(uint32,uint16)[])` and selector `0x8b88a024`.
```solidity
function initialize(address _owner, address[] memory _ejectors, IEjectionManager.QuorumEjectionParams[] memory _quorumEjectionParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _owner: alloy::sol_types::private::Address,
        pub _ejectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub _quorumEjectionParams: alloy::sol_types::private::Vec<
            <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initialize(address,address[],(uint32,uint16)[])`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<IEjectionManager::QuorumEjectionParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._owner, value._ejectors, value._quorumEjectionParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _owner: tuple.0,
                        _ejectors: tuple.1,
                        _quorumEjectionParams: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<IEjectionManager::QuorumEjectionParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address[],(uint32,uint16)[])";
            const SELECTOR: [u8; 4] = [139u8, 136u8, 160u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._owner,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._ejectors),
                    <alloy::sol_types::sol_data::Array<
                        IEjectionManager::QuorumEjectionParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._quorumEjectionParams),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isEjector(address)` and selector `0x6c08a879`.
```solidity
function isEjector(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isEjectorCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isEjector(address)`](isEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isEjectorReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isEjectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isEjectorCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isEjectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isEjectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isEjectorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isEjectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isEjectorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isEjectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isEjector(address)";
            const SELECTOR: [u8; 4] = [108u8, 8u8, 168u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumEjectionParams(uint8)` and selector `0x00482569`.
```solidity
function quorumEjectionParams(uint8) external view returns (uint32 rateLimitWindow, uint16 ejectableStakePercent);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumEjectionParamsCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`quorumEjectionParams(uint8)`](quorumEjectionParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumEjectionParamsReturn {
        pub rateLimitWindow: u32,
        pub ejectableStakePercent: u16,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumEjectionParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumEjectionParamsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumEjectionParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u16);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumEjectionParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: quorumEjectionParamsReturn) -> Self {
                    (value.rateLimitWindow, value.ejectableStakePercent)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for quorumEjectionParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rateLimitWindow: tuple.0,
                        ejectableStakePercent: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumEjectionParamsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumEjectionParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorumEjectionParams(uint8)";
            const SELECTOR: [u8; 4] = [0u8, 72u8, 37u8, 105u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
```solidity
function registryCoordinator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registryCoordinator()";
            const SELECTOR: [u8; 4] = [109u8, 20u8, 169u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setEjector(address,bool)` and selector `0x10ea4f8a`.
```solidity
function setEjector(address _ejector, bool _status) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorCall {
        pub _ejector: alloy::sol_types::private::Address,
        pub _status: bool,
    }
    ///Container type for the return parameters of the [`setEjector(address,bool)`](setEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEjectorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEjectorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorCall) -> Self {
                    (value._ejector, value._status)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ejector: tuple.0,
                        _status: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setEjectorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setEjectorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEjectorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEjectorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEjector(address,bool)";
            const SELECTOR: [u8; 4] = [16u8, 234u8, 79u8, 138u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ejector,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._status,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setQuorumEjectionParams(uint8,(uint32,uint16))` and selector `0x77d17586`.
```solidity
function setQuorumEjectionParams(uint8 _quorumNumber, IEjectionManager.QuorumEjectionParams memory _quorumEjectionParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setQuorumEjectionParamsCall {
        pub _quorumNumber: u8,
        pub _quorumEjectionParams: <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setQuorumEjectionParams(uint8,(uint32,uint16))`](setQuorumEjectionParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setQuorumEjectionParamsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                IEjectionManager::QuorumEjectionParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setQuorumEjectionParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setQuorumEjectionParamsCall) -> Self {
                    (value._quorumNumber, value._quorumEjectionParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setQuorumEjectionParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _quorumNumber: tuple.0,
                        _quorumEjectionParams: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setQuorumEjectionParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setQuorumEjectionParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setQuorumEjectionParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setQuorumEjectionParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                IEjectionManager::QuorumEjectionParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setQuorumEjectionParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setQuorumEjectionParams(uint8,(uint32,uint16))";
            const SELECTOR: [u8; 4] = [119u8, 209u8, 117u8, 134u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._quorumNumber),
                    <IEjectionManager::QuorumEjectionParams as alloy_sol_types::SolType>::tokenize(
                        &self._quorumEjectionParams,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `stakeEjectedForQuorum(uint8,uint256)` and selector `0x3a0b0ddd`.
```solidity
function stakeEjectedForQuorum(uint8, uint256) external view returns (uint256 timestamp, uint256 stakeEjected);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeEjectedForQuorumCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`stakeEjectedForQuorum(uint8,uint256)`](stakeEjectedForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeEjectedForQuorumReturn {
        pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub stakeEjected: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeEjectedForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeEjectedForQuorumCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeEjectedForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeEjectedForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeEjectedForQuorumReturn) -> Self {
                    (value.timestamp, value.stakeEjected)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeEjectedForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        timestamp: tuple.0,
                        stakeEjected: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeEjectedForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeEjectedForQuorumReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeEjectedForQuorum(uint8,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 11u8, 13u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
```solidity
function stakeRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`EjectionManager`](self) function calls.
    pub enum EjectionManagerCalls {
        amountEjectableForQuorum(amountEjectableForQuorumCall),
        ejectOperators(ejectOperatorsCall),
        initialize(initializeCall),
        isEjector(isEjectorCall),
        owner(ownerCall),
        quorumEjectionParams(quorumEjectionParamsCall),
        registryCoordinator(registryCoordinatorCall),
        renounceOwnership(renounceOwnershipCall),
        setEjector(setEjectorCall),
        setQuorumEjectionParams(setQuorumEjectionParamsCall),
        stakeEjectedForQuorum(stakeEjectedForQuorumCall),
        stakeRegistry(stakeRegistryCall),
        transferOwnership(transferOwnershipCall),
    }
    #[automatically_derived]
    impl EjectionManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 72u8, 37u8, 105u8],
            [10u8, 5u8, 147u8, 209u8],
            [16u8, 234u8, 79u8, 138u8],
            [58u8, 11u8, 13u8, 221u8],
            [104u8, 48u8, 72u8, 53u8],
            [108u8, 8u8, 168u8, 121u8],
            [109u8, 20u8, 169u8, 135u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 209u8, 117u8, 134u8],
            [139u8, 136u8, 160u8, 36u8],
            [141u8, 165u8, 203u8, 91u8],
            [177u8, 63u8, 69u8, 4u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EjectionManagerCalls {
        const NAME: &'static str = "EjectionManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::amountEjectableForQuorum(_) => {
                    <amountEjectableForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ejectOperators(_) => {
                    <ejectOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isEjector(_) => {
                    <isEjectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorumEjectionParams(_) => {
                    <quorumEjectionParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEjector(_) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setQuorumEjectionParams(_) => {
                    <setQuorumEjectionParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeEjectedForQuorum(_) => {
                    <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<EjectionManagerCalls>] = &[
                {
                    fn quorumEjectionParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <quorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::quorumEjectionParams)
                    }
                    quorumEjectionParams
                },
                {
                    fn ejectOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <ejectOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::ejectOperators)
                    }
                    ejectOperators
                },
                {
                    fn setEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <setEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::setEjector)
                    }
                    setEjector
                },
                {
                    fn stakeEjectedForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::stakeEjectedForQuorum)
                    }
                    stakeEjectedForQuorum
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn isEjector(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <isEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::isEjector)
                    }
                    isEjector
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn setQuorumEjectionParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <setQuorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::setQuorumEjectionParams)
                    }
                    setQuorumEjectionParams
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn amountEjectableForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <amountEjectableForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::amountEjectableForQuorum)
                    }
                    amountEjectableForQuorum
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EjectionManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::amountEjectableForQuorum(inner) => {
                    <amountEjectableForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ejectOperators(inner) => {
                    <ejectOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isEjector(inner) => {
                    <isEjectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorumEjectionParams(inner) => {
                    <quorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setQuorumEjectionParams(inner) => {
                    <setQuorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeEjectedForQuorum(inner) => {
                    <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::amountEjectableForQuorum(inner) => {
                    <amountEjectableForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ejectOperators(inner) => {
                    <ejectOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isEjector(inner) => {
                    <isEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorumEjectionParams(inner) => {
                    <quorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setQuorumEjectionParams(inner) => {
                    <setQuorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeEjectedForQuorum(inner) => {
                    <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EjectionManager`](self) events.
    pub enum EjectionManagerEvents {
        EjectorUpdated(EjectorUpdated),
        Initialized(Initialized),
        OperatorEjected(OperatorEjected),
        OwnershipTransferred(OwnershipTransferred),
        QuorumEjection(QuorumEjection),
        QuorumEjectionParamsSet(QuorumEjectionParamsSet),
    }
    #[automatically_derived]
    impl EjectionManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                25u8,
                221u8,
                135u8,
                174u8,
                73u8,
                237u8,
                20u8,
                167u8,
                149u8,
                248u8,
                194u8,
                213u8,
                232u8,
                5u8,
                91u8,
                242u8,
                164u8,
                169u8,
                208u8,
                22u8,
                65u8,
                160u8,
                10u8,
                47u8,
                143u8,
                10u8,
                90u8,
                123u8,
                247u8,
                247u8,
                2u8,
                73u8,
            ],
            [
                118u8,
                118u8,
                104u8,
                107u8,
                109u8,
                34u8,
                225u8,
                18u8,
                65u8,
                43u8,
                216u8,
                116u8,
                215u8,
                1u8,
                119u8,
                224u8,
                17u8,
                171u8,
                6u8,
                96u8,
                44u8,
                38u8,
                6u8,
                63u8,
                25u8,
                240u8,
                56u8,
                108u8,
                154u8,
                60u8,
                238u8,
                66u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                151u8,
                221u8,
                183u8,
                17u8,
                198u8,
                26u8,
                157u8,
                45u8,
                126u8,
                255u8,
                203u8,
                163u8,
                224u8,
                66u8,
                163u8,
                56u8,
                98u8,
                41u8,
                127u8,
                137u8,
                141u8,
                85u8,
                86u8,
                85u8,
                204u8,
                163u8,
                158u8,
                196u8,
                69u8,
                31u8,
                83u8,
                180u8,
            ],
            [
                230u8,
                156u8,
                40u8,
                39u8,
                161u8,
                226u8,
                253u8,
                211u8,
                34u8,
                101u8,
                235u8,
                180u8,
                238u8,
                234u8,
                94u8,
                229u8,
                100u8,
                240u8,
                85u8,
                28u8,
                245u8,
                223u8,
                237u8,
                65u8,
                80u8,
                248u8,
                225u8,
                22u8,
                166u8,
                114u8,
                9u8,
                235u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EjectionManagerEvents {
        const NAME: &'static str = "EjectionManagerEvents";
        const COUNT: usize = 6usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<EjectorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EjectorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EjectorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(<OperatorEjected as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorEjected as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorEjected)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<QuorumEjection as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumEjection as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumEjection)
                }
                Some(
                    <QuorumEjectionParamsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <QuorumEjectionParamsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumEjectionParamsSet)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for EjectionManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorEjected(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumEjection(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumEjectionParamsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EjectorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorEjected(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumEjection(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumEjectionParamsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EjectionManager`](self) contract instance.

See the [wrapper's documentation](`EjectionManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EjectionManagerInstance<T, P, N> {
        EjectionManagerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _registryCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EjectionManagerInstance<T, P, N>>,
    > {
        EjectionManagerInstance::<
            T,
            P,
            N,
        >::deploy(provider, _registryCoordinator, _stakeRegistry)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _registryCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EjectionManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _registryCoordinator, _stakeRegistry)
    }
    /**A [`EjectionManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EjectionManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EjectionManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EjectionManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EjectionManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EjectionManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EjectionManager`](self) contract instance.

See the [wrapper's documentation](`EjectionManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _registryCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<EjectionManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _stakeRegistry,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _registryCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
                            _stakeRegistry,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> EjectionManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EjectionManagerInstance<T, P, N> {
            EjectionManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EjectionManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`amountEjectableForQuorum`] function.
        pub fn amountEjectableForQuorum(
            &self,
            _quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, amountEjectableForQuorumCall, N> {
            self.call_builder(
                &amountEjectableForQuorumCall {
                    _quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`ejectOperators`] function.
        pub fn ejectOperators(
            &self,
            _operatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, ejectOperatorsCall, N> {
            self.call_builder(&ejectOperatorsCall { _operatorIds })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _owner: alloy::sol_types::private::Address,
            _ejectors: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            _quorumEjectionParams: alloy::sol_types::private::Vec<
                <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _owner,
                    _ejectors,
                    _quorumEjectionParams,
                },
            )
        }
        ///Creates a new call builder for the [`isEjector`] function.
        pub fn isEjector(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isEjectorCall, N> {
            self.call_builder(&isEjectorCall { _0 })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`quorumEjectionParams`] function.
        pub fn quorumEjectionParams(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, quorumEjectionParamsCall, N> {
            self.call_builder(&quorumEjectionParamsCall { _0 })
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setEjector`] function.
        pub fn setEjector(
            &self,
            _ejector: alloy::sol_types::private::Address,
            _status: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setEjectorCall, N> {
            self.call_builder(
                &setEjectorCall {
                    _ejector,
                    _status,
                },
            )
        }
        ///Creates a new call builder for the [`setQuorumEjectionParams`] function.
        pub fn setQuorumEjectionParams(
            &self,
            _quorumNumber: u8,
            _quorumEjectionParams: <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setQuorumEjectionParamsCall, N> {
            self.call_builder(
                &setQuorumEjectionParamsCall {
                    _quorumNumber,
                    _quorumEjectionParams,
                },
            )
        }
        ///Creates a new call builder for the [`stakeEjectedForQuorum`] function.
        pub fn stakeEjectedForQuorum(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeEjectedForQuorumCall, N> {
            self.call_builder(
                &stakeEjectedForQuorumCall {
                    _0,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EjectionManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EjectorUpdated`] event.
        pub fn EjectorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EjectorUpdated, N> {
            self.event_filter::<EjectorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorEjected`] event.
        pub fn OperatorEjected_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorEjected, N> {
            self.event_filter::<OperatorEjected>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumEjection`] event.
        pub fn QuorumEjection_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumEjection, N> {
            self.event_filter::<QuorumEjection>()
        }
        ///Creates a new event filter for the [`QuorumEjectionParamsSet`] event.
        pub fn QuorumEjectionParamsSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumEjectionParamsSet, N> {
            self.event_filter::<QuorumEjectionParamsSet>()
        }
    }
}
