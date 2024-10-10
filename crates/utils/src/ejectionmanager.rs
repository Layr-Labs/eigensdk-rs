///Module containing a contract's types and functions.
/**

```solidity
library IEjectionManager {
    struct QuorumEjectionParams { uint32 rateLimitWindow; uint16 ejectableStakePercent; }
}
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod IEjectionManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct QuorumEjectionParams { uint32 rateLimitWindow; uint16 ejectableStakePercent; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct QuorumEjectionParams {
        pub rateLimitWindow: u32,
        pub ejectableStakePercent: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.rateLimitWindow,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.ejectableStakePercent,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for QuorumEjectionParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
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
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("IEjectionManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IEjectionManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IEjectionManager`](self) contract instance.

        See the [wrapper's documentation](`IEjectionManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
        > IEjectionManagerInstance<T, P, N>
    {
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
        > IEjectionManagerInstance<T, P, N>
    {
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
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod EjectionManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c06040523480156200001157600080fd5b50604051620015a8380380620015a8833981016040819052620000349162000134565b6001600160a01b03808316608052811660a0526200005162000059565b505062000173565b600054610100900460ff1615620000c65760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000119576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200013157600080fd5b50565b600080604083850312156200014857600080fd5b825162000155816200011b565b602084015190925062000168816200011b565b809150509250929050565b60805160a0516113f3620001b5600039600081816101800152818161035a0152610a0a0152600081816101f2015281816104ef015261051e01526113f36000f3fe608060405234801561001057600080fd5b50600436106100ce5760003560e01c80636d14a9871161008c5780638b88a024116100665780638b88a0241461022f5780638da5cb5b14610242578063b13f450414610253578063f2fde38b1461027457600080fd5b80636d14a987146101ed578063715018a61461021457806377d175861461021c57600080fd5b8062482569146100d35780630a0593d11461012b57806310ea4f8a146101405780633a0b0ddd14610153578063683048351461017b5780636c08a879146101ba575b600080fd5b6101076100e1366004610de1565b60676020526000908152604090205463ffffffff811690640100000000900461ffff1682565b6040805163ffffffff909316835261ffff9091166020830152015b60405180910390f35b61013e610139366004610e6e565b610287565b005b61013e61014e366004610f8f565b6107a9565b610166610161366004610fcd565b6107bb565b60408051928352602083019190915201610122565b6101a27f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610122565b6101dd6101c8366004610ff7565b60656020526000908152604090205460ff1681565b6040519015158152602001610122565b6101a27f000000000000000000000000000000000000000000000000000000000000000081565b61013e6107f7565b61013e61022a366004611087565b61080b565b61013e61023d36600461112a565b61081d565b6033546001600160a01b03166101a2565b610266610261366004610de1565b6109c3565b604051908152602001610122565b61013e610282366004610ff7565b610bc5565b3360009081526065602052604090205460ff16806102af57506033546001600160a01b031633145b6103115760405162461bcd60e51b815260206004820152602860248201527f456a6563746f723a204f6e6c79206f776e6572206f7220656a6563746f722063604482015267185b88195a9958dd60c21b60648201526084015b60405180910390fd5b60005b81518110156107a557806000610329826109c3565b905060008080805b87878151811061034357610343611200565b6020026020010151518160ff1610156106e35760007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316635401ed278a8a8151811061039957610399611200565b60200260200101518460ff16815181106103b5576103b5611200565b6020026020010151896040518363ffffffff1660e01b81526004016103e792919091825260ff16602082015260400190565b602060405180830381865afa158015610404573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104289190611216565b336000908152606560205260409020546001600160601b0391909116915060ff16801561046c575060ff871660009081526067602052604090205463ffffffff1615155b801561048057508561047e8287611255565b115b156104d6575060ff861660009081526066602090815260408083208151808301909252428252818301888152815460018181018455928652939094209151600290930290910191825591519082015591506106e3565b6104e08186611255565b94506104eb8461126d565b93507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636e3b17db7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663296bb0648c8c8151811061055d5761055d611200565b60200260200101518660ff168151811061057957610579611200565b60200260200101516040518263ffffffff1660e01b815260040161059f91815260200190565b602060405180830381865afa1580156105bc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e09190611291565b6040516001600160f81b031960f88c901b1660208201526021016040516020818303038152906040526040518363ffffffff1660e01b81526004016106269291906112ae565b600060405180830381600087803b15801561064057600080fd5b505af1158015610654573d6000803e3d6000fd5b505050507f97ddb711c61a9d2d7effcba3e042a33862297f898d555655cca39ec4451f53b489898151811061068b5761068b611200565b60200260200101518360ff16815181106106a7576106a7611200565b6020026020010151886040516106ca92919091825260ff16602082015260400190565b60405180910390a1506106dc81611313565b9050610331565b508015801561070157503360009081526065602052604090205460ff165b1561074f5760ff851660009081526066602090815260408083208151808301909252428252818301878152815460018181018455928652939094209151600290930290910191825591519101555b6040805163ffffffff8416815282151560208201527f19dd87ae49ed14a795f8c2d5e8055bf2a4a9d01641a00a2f8f0a5a7bf7f70249910160405180910390a150505050508061079e90611333565b9050610314565b5050565b6107b1610c3e565b6107a58282610c98565b606660205281600052604060002081815481106107d757600080fd5b600091825260209091206002909102018054600190910154909250905082565b6107ff610c3e565b6108096000610cfc565b565b610813610c3e565b6107a58282610d4e565b600054610100900460ff161580801561083d5750600054600160ff909116105b806108575750303b158015610857575060005460ff166001145b6108ba5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610308565b6000805460ff1916600117905580156108dd576000805461ff0019166101001790555b6108e684610cfc565b60005b83518160ff16101561092e5761091c848260ff168151811061090d5761090d611200565b60200260200101516001610c98565b8061092681611313565b9150506108e9565b5060005b82518160ff1610156109765761096481848360ff168151811061095757610957611200565b6020026020010151610d4e565b8061096e81611313565b915050610932565b5080156109bd576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60ff811660009081526067602052604081205481906109e89063ffffffff164261134e565b60405163d5eccc0560e01b815260ff85166004820152909150600090612710907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d5eccc0590602401602060405180830381865afa158015610a59573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a7d9190611216565b60ff8616600090815260676020526040902054610aaf916001600160601b031690640100000000900461ffff16611365565b610ab99190611384565b60ff8516600090815260666020526040812054919250908190610ae0575090949350505050565b60ff8616600090815260666020526040902054610aff9060019061134e565b90505b60ff86166000908152606660205260409020805485919083908110610b2957610b29611200565b9060005260206000209060020201600001541115610b9e5760ff86166000908152606660205260409020805482908110610b6557610b65611200565b90600052602060002090600202016001015482610b829190611255565b915080610b8e57610b9e565b610b97816113a6565b9050610b02565b828210610bb15750600095945050505050565b610bbb828461134e565b9695505050505050565b610bcd610c3e565b6001600160a01b038116610c325760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610308565b610c3b81610cfc565b50565b6033546001600160a01b031633146108095760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610308565b6001600160a01b038216600081815260656020908152604091829020805460ff19168515159081179091558251938452908301527f7676686b6d22e112412bd874d70177e011ab06602c26063f19f0386c9a3cee4291015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60ff8216600081815260676020908152604091829020845181548684015163ffffffff90921665ffffffffffff19909116811764010000000061ffff90931692830217909255835194855291840152908201527fe69c2827a1e2fdd32265ebb4eeea5ee564f0551cf5dfed4150f8e116a67209eb90606001610cf0565b803560ff81168114610ddc57600080fd5b919050565b600060208284031215610df357600080fd5b610dfc82610dcb565b9392505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715610e4257610e42610e03565b604052919050565b600067ffffffffffffffff821115610e6457610e64610e03565b5060051b60200190565b60006020808385031215610e8157600080fd5b823567ffffffffffffffff80821115610e9957600080fd5b818501915085601f830112610ead57600080fd5b8135610ec0610ebb82610e4a565b610e19565b818152600591821b8401850191858201919089841115610edf57600080fd5b8686015b84811015610f6b57803586811115610efb5760008081fd5b8701603f81018c13610f0d5760008081fd5b888101356040610f1f610ebb83610e4a565b82815291851b83018101918b8101908f841115610f3c5760008081fd5b938201935b83851015610f5a5784358252938c0193908c0190610f41565b885250505093880193508701610ee3565b50909998505050505050505050565b6001600160a01b0381168114610c3b57600080fd5b60008060408385031215610fa257600080fd5b8235610fad81610f7a565b915060208301358015158114610fc257600080fd5b809150509250929050565b60008060408385031215610fe057600080fd5b610fe983610dcb565b946020939093013593505050565b60006020828403121561100957600080fd5b8135610dfc81610f7a565b60006040828403121561102657600080fd5b6040516040810181811067ffffffffffffffff8211171561104957611049610e03565b604052905080823563ffffffff8116811461106357600080fd5b8152602083013561ffff8116811461107a57600080fd5b6020919091015292915050565b6000806060838503121561109a57600080fd5b6110a383610dcb565b91506110b28460208501611014565b90509250929050565b600082601f8301126110cc57600080fd5b813560206110dc610ebb83610e4a565b82815260069290921b840181019181810190868411156110fb57600080fd5b8286015b8481101561111f576111118882611014565b8352918301916040016110ff565b509695505050505050565b60008060006060848603121561113f57600080fd5b833561114a81610f7a565b925060208481013567ffffffffffffffff8082111561116857600080fd5b818701915087601f83011261117c57600080fd5b813561118a610ebb82610e4a565b81815260059190911b8301840190848101908a8311156111a957600080fd5b938501935b828510156111d05784356111c181610f7a565b825293850193908501906111ae565b9650505060408701359250808311156111e857600080fd5b50506111f6868287016110bb565b9150509250925092565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561122857600080fd5b81516001600160601b0381168114610dfc57600080fd5b634e487b7160e01b600052601160045260246000fd5b600082198211156112685761126861123f565b500190565b600063ffffffff808316818114156112875761128761123f565b6001019392505050565b6000602082840312156112a357600080fd5b8151610dfc81610f7a565b60018060a01b038316815260006020604081840152835180604085015260005b818110156112ea578581018301518582016060015282016112ce565b818111156112fc576000606083870101525b50601f01601f191692909201606001949350505050565b600060ff821660ff81141561132a5761132a61123f565b60010192915050565b60006000198214156113475761134761123f565b5060010190565b6000828210156113605761136061123f565b500390565b600081600019048311821515161561137f5761137f61123f565b500290565b6000826113a157634e487b7160e01b600052601260045260246000fd5b500490565b6000816113b5576113b561123f565b50600019019056fea26469706673582212200491d577a31ceab4c00062e65e01bccefbfae29de7f1cac7f038acd3ec33f92c64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x15\xA88\x03\x80b\0\x15\xA8\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x014V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R\x81\x16`\xA0Rb\0\0Qb\0\0YV[PPb\0\x01sV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x19W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x011W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01HW`\0\x80\xFD[\x82Qb\0\x01U\x81b\0\x01\x1BV[` \x84\x01Q\x90\x92Pb\0\x01h\x81b\0\x01\x1BV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x13\xF3b\0\x01\xB5`\09`\0\x81\x81a\x01\x80\x01R\x81\x81a\x03Z\x01Ra\n\n\x01R`\0\x81\x81a\x01\xF2\x01R\x81\x81a\x04\xEF\x01Ra\x05\x1E\x01Ra\x13\xF3`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\x8CW\x80c\x8B\x88\xA0$\x11a\0fW\x80c\x8B\x88\xA0$\x14a\x02/W\x80c\x8D\xA5\xCB[\x14a\x02BW\x80c\xB1?E\x04\x14a\x02SW\x80c\xF2\xFD\xE3\x8B\x14a\x02tW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x01\xEDW\x80cqP\x18\xA6\x14a\x02\x14W\x80cw\xD1u\x86\x14a\x02\x1CW`\0\x80\xFD[\x80bH%i\x14a\0\xD3W\x80c\n\x05\x93\xD1\x14a\x01+W\x80c\x10\xEAO\x8A\x14a\x01@W\x80c:\x0B\r\xDD\x14a\x01SW\x80ch0H5\x14a\x01{W\x80cl\x08\xA8y\x14a\x01\xBAW[`\0\x80\xFD[a\x01\x07a\0\xE16`\x04a\r\xE1V[`g` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x81\x16\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16\x82V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x93\x16\x83Ra\xFF\xFF\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01>a\x0196`\x04a\x0EnV[a\x02\x87V[\0[a\x01>a\x01N6`\x04a\x0F\x8FV[a\x07\xA9V[a\x01fa\x01a6`\x04a\x0F\xCDV[a\x07\xBBV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\"V[a\x01\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\"V[a\x01\xDDa\x01\xC86`\x04a\x0F\xF7V[`e` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\"V[a\x01\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01>a\x07\xF7V[a\x01>a\x02*6`\x04a\x10\x87V[a\x08\x0BV[a\x01>a\x02=6`\x04a\x11*V[a\x08\x1DV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\xA2V[a\x02fa\x02a6`\x04a\r\xE1V[a\t\xC3V[`@Q\x90\x81R` \x01a\x01\"V[a\x01>a\x02\x826`\x04a\x0F\xF7V[a\x0B\xC5V[3`\0\x90\x81R`e` R`@\x90 T`\xFF\x16\x80a\x02\xAFWP`3T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x03\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FEjector: Only owner or ejector c`D\x82\x01Rg\x18[\x88\x19Z\x99X\xDD`\xC2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x07\xA5W\x80`\0a\x03)\x82a\t\xC3V[\x90P`\0\x80\x80\x80[\x87\x87\x81Q\x81\x10a\x03CWa\x03Ca\x12\0V[` \x02` \x01\x01QQ\x81`\xFF\x16\x10\x15a\x06\xE3W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cT\x01\xED'\x8A\x8A\x81Q\x81\x10a\x03\x99Wa\x03\x99a\x12\0V[` \x02` \x01\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x03\xB5Wa\x03\xB5a\x12\0V[` \x02` \x01\x01Q\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE7\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04(\x91\x90a\x12\x16V[3`\0\x90\x81R`e` R`@\x90 T`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x91P`\xFF\x16\x80\x15a\x04lWP`\xFF\x87\x16`\0\x90\x81R`g` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x15\x15[\x80\x15a\x04\x80WP\x85a\x04~\x82\x87a\x12UV[\x11[\x15a\x04\xD6WP`\xFF\x86\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92RB\x82R\x81\x83\x01\x88\x81R\x81T`\x01\x81\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q`\x02\x90\x93\x02\x90\x91\x01\x91\x82U\x91Q\x90\x82\x01U\x91Pa\x06\xE3V[a\x04\xE0\x81\x86a\x12UV[\x94Pa\x04\xEB\x84a\x12mV[\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cn;\x17\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c)k\xB0d\x8C\x8C\x81Q\x81\x10a\x05]Wa\x05]a\x12\0V[` \x02` \x01\x01Q\x86`\xFF\x16\x81Q\x81\x10a\x05yWa\x05ya\x12\0V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x9F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE0\x91\x90a\x12\x91V[`@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x8C\x90\x1B\x16` \x82\x01R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06&\x92\x91\x90a\x12\xAEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06TW=`\0\x80>=`\0\xFD[PPPP\x7F\x97\xDD\xB7\x11\xC6\x1A\x9D-~\xFF\xCB\xA3\xE0B\xA38b)\x7F\x89\x8DUVU\xCC\xA3\x9E\xC4E\x1FS\xB4\x89\x89\x81Q\x81\x10a\x06\x8BWa\x06\x8Ba\x12\0V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x06\xA7Wa\x06\xA7a\x12\0V[` \x02` \x01\x01Q\x88`@Qa\x06\xCA\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1Pa\x06\xDC\x81a\x13\x13V[\x90Pa\x031V[P\x80\x15\x80\x15a\x07\x01WP3`\0\x90\x81R`e` R`@\x90 T`\xFF\x16[\x15a\x07OW`\xFF\x85\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92RB\x82R\x81\x83\x01\x87\x81R\x81T`\x01\x81\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q`\x02\x90\x93\x02\x90\x91\x01\x91\x82U\x91Q\x91\x01U[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R\x82\x15\x15` \x82\x01R\x7F\x19\xDD\x87\xAEI\xED\x14\xA7\x95\xF8\xC2\xD5\xE8\x05[\xF2\xA4\xA9\xD0\x16A\xA0\n/\x8F\nZ{\xF7\xF7\x02I\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPP\x80a\x07\x9E\x90a\x133V[\x90Pa\x03\x14V[PPV[a\x07\xB1a\x0C>V[a\x07\xA5\x82\x82a\x0C\x98V[`f` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x07\xD7W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T\x90\x92P\x90P\x82V[a\x07\xFFa\x0C>V[a\x08\t`\0a\x0C\xFCV[V[a\x08\x13a\x0C>V[a\x07\xA5\x82\x82a\rNV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08=WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08WWP0;\x15\x80\x15a\x08WWP`\0T`\xFF\x16`\x01\x14[a\x08\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x03\x08V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xDDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\xE6\x84a\x0C\xFCV[`\0[\x83Q\x81`\xFF\x16\x10\x15a\t.Wa\t\x1C\x84\x82`\xFF\x16\x81Q\x81\x10a\t\rWa\t\ra\x12\0V[` \x02` \x01\x01Q`\x01a\x0C\x98V[\x80a\t&\x81a\x13\x13V[\x91PPa\x08\xE9V[P`\0[\x82Q\x81`\xFF\x16\x10\x15a\tvWa\td\x81\x84\x83`\xFF\x16\x81Q\x81\x10a\tWWa\tWa\x12\0V[` \x02` \x01\x01Qa\rNV[\x80a\tn\x81a\x13\x13V[\x91PPa\t2V[P\x80\x15a\t\xBDW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`\xFF\x81\x16`\0\x90\x81R`g` R`@\x81 T\x81\x90a\t\xE8\x90c\xFF\xFF\xFF\xFF\x16Ba\x13NV[`@Qc\xD5\xEC\xCC\x05`\xE0\x1B\x81R`\xFF\x85\x16`\x04\x82\x01R\x90\x91P`\0\x90a'\x10\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD5\xEC\xCC\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nYW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n}\x91\x90a\x12\x16V[`\xFF\x86\x16`\0\x90\x81R`g` R`@\x90 Ta\n\xAF\x91`\x01`\x01``\x1B\x03\x16\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16a\x13eV[a\n\xB9\x91\x90a\x13\x84V[`\xFF\x85\x16`\0\x90\x81R`f` R`@\x81 T\x91\x92P\x90\x81\x90a\n\xE0WP\x90\x94\x93PPPPV[`\xFF\x86\x16`\0\x90\x81R`f` R`@\x90 Ta\n\xFF\x90`\x01\x90a\x13NV[\x90P[`\xFF\x86\x16`\0\x90\x81R`f` R`@\x90 \x80T\x85\x91\x90\x83\x90\x81\x10a\x0B)Wa\x0B)a\x12\0V[\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x01T\x11\x15a\x0B\x9EW`\xFF\x86\x16`\0\x90\x81R`f` R`@\x90 \x80T\x82\x90\x81\x10a\x0BeWa\x0Bea\x12\0V[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01T\x82a\x0B\x82\x91\x90a\x12UV[\x91P\x80a\x0B\x8EWa\x0B\x9EV[a\x0B\x97\x81a\x13\xA6V[\x90Pa\x0B\x02V[\x82\x82\x10a\x0B\xB1WP`\0\x95\x94PPPPPV[a\x0B\xBB\x82\x84a\x13NV[\x96\x95PPPPPPV[a\x0B\xCDa\x0C>V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x03\x08V[a\x0C;\x81a\x0C\xFCV[PV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x08V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7Fvvhkm\"\xE1\x12A+\xD8t\xD7\x01w\xE0\x11\xAB\x06`,&\x06?\x19\xF08l\x9A<\xEEB\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\xFF\x82\x16`\0\x81\x81R`g` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x92\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81\x17d\x01\0\0\0\0a\xFF\xFF\x90\x93\x16\x92\x83\x02\x17\x90\x92U\x83Q\x94\x85R\x91\x84\x01R\x90\x82\x01R\x7F\xE6\x9C('\xA1\xE2\xFD\xD3\"e\xEB\xB4\xEE\xEA^\xE5d\xF0U\x1C\xF5\xDF\xEDAP\xF8\xE1\x16\xA6r\t\xEB\x90``\x01a\x0C\xF0V[\x805`\xFF\x81\x16\x81\x14a\r\xDCW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xF3W`\0\x80\xFD[a\r\xFC\x82a\r\xCBV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EBWa\x0EBa\x0E\x03V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EdWa\x0Eda\x0E\x03V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15a\x0E\x81W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x99W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xADW`\0\x80\xFD[\x815a\x0E\xC0a\x0E\xBB\x82a\x0EJV[a\x0E\x19V[\x81\x81R`\x05\x91\x82\x1B\x84\x01\x85\x01\x91\x85\x82\x01\x91\x90\x89\x84\x11\x15a\x0E\xDFW`\0\x80\xFD[\x86\x86\x01[\x84\x81\x10\x15a\x0FkW\x805\x86\x81\x11\x15a\x0E\xFBW`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x8C\x13a\x0F\rW`\0\x80\x81\xFD[\x88\x81\x015`@a\x0F\x1Fa\x0E\xBB\x83a\x0EJV[\x82\x81R\x91\x85\x1B\x83\x01\x81\x01\x91\x8B\x81\x01\x90\x8F\x84\x11\x15a\x0F<W`\0\x80\x81\xFD[\x93\x82\x01\x93[\x83\x85\x10\x15a\x0FZW\x845\x82R\x93\x8C\x01\x93\x90\x8C\x01\x90a\x0FAV[\x88RPPP\x93\x88\x01\x93P\x87\x01a\x0E\xE3V[P\x90\x99\x98PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C;W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xA2W`\0\x80\xFD[\x825a\x0F\xAD\x81a\x0FzV[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x0F\xC2W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xE0W`\0\x80\xFD[a\x0F\xE9\x83a\r\xCBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x10\tW`\0\x80\xFD[\x815a\r\xFC\x81a\x0FzV[`\0`@\x82\x84\x03\x12\x15a\x10&W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10IWa\x10Ia\x0E\x03V[`@R\x90P\x80\x825c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10cW`\0\x80\xFD[\x81R` \x83\x015a\xFF\xFF\x81\x16\x81\x14a\x10zW`\0\x80\xFD[` \x91\x90\x91\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x10\x9AW`\0\x80\xFD[a\x10\xA3\x83a\r\xCBV[\x91Pa\x10\xB2\x84` \x85\x01a\x10\x14V[\x90P\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12a\x10\xCCW`\0\x80\xFD[\x815` a\x10\xDCa\x0E\xBB\x83a\x0EJV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\xFBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x11\x1FWa\x11\x11\x88\x82a\x10\x14V[\x83R\x91\x83\x01\x91`@\x01a\x10\xFFV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11?W`\0\x80\xFD[\x835a\x11J\x81a\x0FzV[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11hW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x11|W`\0\x80\xFD[\x815a\x11\x8Aa\x0E\xBB\x82a\x0EJV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a\x11\xA9W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x11\xD0W\x845a\x11\xC1\x81a\x0FzV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x11\xAEV[\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a\x11\xE8W`\0\x80\xFD[PPa\x11\xF6\x86\x82\x87\x01a\x10\xBBV[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x12(W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\r\xFCW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x12hWa\x12ha\x12?V[P\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x12\x87Wa\x12\x87a\x12?V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\xA3W`\0\x80\xFD[\x81Qa\r\xFC\x81a\x0FzV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x12\xEAW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x12\xCEV[\x81\x81\x11\x15a\x12\xFCW`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x13*Wa\x13*a\x12?V[`\x01\x01\x92\x91PPV[`\0`\0\x19\x82\x14\x15a\x13GWa\x13Ga\x12?V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x13`Wa\x13`a\x12?V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x13\x7FWa\x13\x7Fa\x12?V[P\x02\x90V[`\0\x82a\x13\xA1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81a\x13\xB5Wa\x13\xB5a\x12?V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \x04\x91\xD5w\xA3\x1C\xEA\xB4\xC0\0b\xE6^\x01\xBC\xCE\xFB\xFA\xE2\x9D\xE7\xF1\xCA\xC7\xF08\xAC\xD3\xEC3\xF9,dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106100ce5760003560e01c80636d14a9871161008c5780638b88a024116100665780638b88a0241461022f5780638da5cb5b14610242578063b13f450414610253578063f2fde38b1461027457600080fd5b80636d14a987146101ed578063715018a61461021457806377d175861461021c57600080fd5b8062482569146100d35780630a0593d11461012b57806310ea4f8a146101405780633a0b0ddd14610153578063683048351461017b5780636c08a879146101ba575b600080fd5b6101076100e1366004610de1565b60676020526000908152604090205463ffffffff811690640100000000900461ffff1682565b6040805163ffffffff909316835261ffff9091166020830152015b60405180910390f35b61013e610139366004610e6e565b610287565b005b61013e61014e366004610f8f565b6107a9565b610166610161366004610fcd565b6107bb565b60408051928352602083019190915201610122565b6101a27f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610122565b6101dd6101c8366004610ff7565b60656020526000908152604090205460ff1681565b6040519015158152602001610122565b6101a27f000000000000000000000000000000000000000000000000000000000000000081565b61013e6107f7565b61013e61022a366004611087565b61080b565b61013e61023d36600461112a565b61081d565b6033546001600160a01b03166101a2565b610266610261366004610de1565b6109c3565b604051908152602001610122565b61013e610282366004610ff7565b610bc5565b3360009081526065602052604090205460ff16806102af57506033546001600160a01b031633145b6103115760405162461bcd60e51b815260206004820152602860248201527f456a6563746f723a204f6e6c79206f776e6572206f7220656a6563746f722063604482015267185b88195a9958dd60c21b60648201526084015b60405180910390fd5b60005b81518110156107a557806000610329826109c3565b905060008080805b87878151811061034357610343611200565b6020026020010151518160ff1610156106e35760007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316635401ed278a8a8151811061039957610399611200565b60200260200101518460ff16815181106103b5576103b5611200565b6020026020010151896040518363ffffffff1660e01b81526004016103e792919091825260ff16602082015260400190565b602060405180830381865afa158015610404573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104289190611216565b336000908152606560205260409020546001600160601b0391909116915060ff16801561046c575060ff871660009081526067602052604090205463ffffffff1615155b801561048057508561047e8287611255565b115b156104d6575060ff861660009081526066602090815260408083208151808301909252428252818301888152815460018181018455928652939094209151600290930290910191825591519082015591506106e3565b6104e08186611255565b94506104eb8461126d565b93507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636e3b17db7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663296bb0648c8c8151811061055d5761055d611200565b60200260200101518660ff168151811061057957610579611200565b60200260200101516040518263ffffffff1660e01b815260040161059f91815260200190565b602060405180830381865afa1580156105bc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105e09190611291565b6040516001600160f81b031960f88c901b1660208201526021016040516020818303038152906040526040518363ffffffff1660e01b81526004016106269291906112ae565b600060405180830381600087803b15801561064057600080fd5b505af1158015610654573d6000803e3d6000fd5b505050507f97ddb711c61a9d2d7effcba3e042a33862297f898d555655cca39ec4451f53b489898151811061068b5761068b611200565b60200260200101518360ff16815181106106a7576106a7611200565b6020026020010151886040516106ca92919091825260ff16602082015260400190565b60405180910390a1506106dc81611313565b9050610331565b508015801561070157503360009081526065602052604090205460ff165b1561074f5760ff851660009081526066602090815260408083208151808301909252428252818301878152815460018181018455928652939094209151600290930290910191825591519101555b6040805163ffffffff8416815282151560208201527f19dd87ae49ed14a795f8c2d5e8055bf2a4a9d01641a00a2f8f0a5a7bf7f70249910160405180910390a150505050508061079e90611333565b9050610314565b5050565b6107b1610c3e565b6107a58282610c98565b606660205281600052604060002081815481106107d757600080fd5b600091825260209091206002909102018054600190910154909250905082565b6107ff610c3e565b6108096000610cfc565b565b610813610c3e565b6107a58282610d4e565b600054610100900460ff161580801561083d5750600054600160ff909116105b806108575750303b158015610857575060005460ff166001145b6108ba5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610308565b6000805460ff1916600117905580156108dd576000805461ff0019166101001790555b6108e684610cfc565b60005b83518160ff16101561092e5761091c848260ff168151811061090d5761090d611200565b60200260200101516001610c98565b8061092681611313565b9150506108e9565b5060005b82518160ff1610156109765761096481848360ff168151811061095757610957611200565b6020026020010151610d4e565b8061096e81611313565b915050610932565b5080156109bd576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60ff811660009081526067602052604081205481906109e89063ffffffff164261134e565b60405163d5eccc0560e01b815260ff85166004820152909150600090612710907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d5eccc0590602401602060405180830381865afa158015610a59573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a7d9190611216565b60ff8616600090815260676020526040902054610aaf916001600160601b031690640100000000900461ffff16611365565b610ab99190611384565b60ff8516600090815260666020526040812054919250908190610ae0575090949350505050565b60ff8616600090815260666020526040902054610aff9060019061134e565b90505b60ff86166000908152606660205260409020805485919083908110610b2957610b29611200565b9060005260206000209060020201600001541115610b9e5760ff86166000908152606660205260409020805482908110610b6557610b65611200565b90600052602060002090600202016001015482610b829190611255565b915080610b8e57610b9e565b610b97816113a6565b9050610b02565b828210610bb15750600095945050505050565b610bbb828461134e565b9695505050505050565b610bcd610c3e565b6001600160a01b038116610c325760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610308565b610c3b81610cfc565b50565b6033546001600160a01b031633146108095760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610308565b6001600160a01b038216600081815260656020908152604091829020805460ff19168515159081179091558251938452908301527f7676686b6d22e112412bd874d70177e011ab06602c26063f19f0386c9a3cee4291015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60ff8216600081815260676020908152604091829020845181548684015163ffffffff90921665ffffffffffff19909116811764010000000061ffff90931692830217909255835194855291840152908201527fe69c2827a1e2fdd32265ebb4eeea5ee564f0551cf5dfed4150f8e116a67209eb90606001610cf0565b803560ff81168114610ddc57600080fd5b919050565b600060208284031215610df357600080fd5b610dfc82610dcb565b9392505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715610e4257610e42610e03565b604052919050565b600067ffffffffffffffff821115610e6457610e64610e03565b5060051b60200190565b60006020808385031215610e8157600080fd5b823567ffffffffffffffff80821115610e9957600080fd5b818501915085601f830112610ead57600080fd5b8135610ec0610ebb82610e4a565b610e19565b818152600591821b8401850191858201919089841115610edf57600080fd5b8686015b84811015610f6b57803586811115610efb5760008081fd5b8701603f81018c13610f0d5760008081fd5b888101356040610f1f610ebb83610e4a565b82815291851b83018101918b8101908f841115610f3c5760008081fd5b938201935b83851015610f5a5784358252938c0193908c0190610f41565b885250505093880193508701610ee3565b50909998505050505050505050565b6001600160a01b0381168114610c3b57600080fd5b60008060408385031215610fa257600080fd5b8235610fad81610f7a565b915060208301358015158114610fc257600080fd5b809150509250929050565b60008060408385031215610fe057600080fd5b610fe983610dcb565b946020939093013593505050565b60006020828403121561100957600080fd5b8135610dfc81610f7a565b60006040828403121561102657600080fd5b6040516040810181811067ffffffffffffffff8211171561104957611049610e03565b604052905080823563ffffffff8116811461106357600080fd5b8152602083013561ffff8116811461107a57600080fd5b6020919091015292915050565b6000806060838503121561109a57600080fd5b6110a383610dcb565b91506110b28460208501611014565b90509250929050565b600082601f8301126110cc57600080fd5b813560206110dc610ebb83610e4a565b82815260069290921b840181019181810190868411156110fb57600080fd5b8286015b8481101561111f576111118882611014565b8352918301916040016110ff565b509695505050505050565b60008060006060848603121561113f57600080fd5b833561114a81610f7a565b925060208481013567ffffffffffffffff8082111561116857600080fd5b818701915087601f83011261117c57600080fd5b813561118a610ebb82610e4a565b81815260059190911b8301840190848101908a8311156111a957600080fd5b938501935b828510156111d05784356111c181610f7a565b825293850193908501906111ae565b9650505060408701359250808311156111e857600080fd5b50506111f6868287016110bb565b9150509250925092565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561122857600080fd5b81516001600160601b0381168114610dfc57600080fd5b634e487b7160e01b600052601160045260246000fd5b600082198211156112685761126861123f565b500190565b600063ffffffff808316818114156112875761128761123f565b6001019392505050565b6000602082840312156112a357600080fd5b8151610dfc81610f7a565b60018060a01b038316815260006020604081840152835180604085015260005b818110156112ea578581018301518582016060015282016112ce565b818111156112fc576000606083870101525b50601f01601f191692909201606001949350505050565b600060ff821660ff81141561132a5761132a61123f565b60010192915050565b60006000198214156113475761134761123f565b5060010190565b6000828210156113605761136061123f565b500390565b600081600019048311821515161561137f5761137f61123f565b500290565b6000826113a157634e487b7160e01b600052601260045260246000fd5b500490565b6000816113b5576113b561123f565b50600019019056fea26469706673582212200491d577a31ceab4c00062e65e01bccefbfae29de7f1cac7f038acd3ec33f92c64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCEW`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\x8CW\x80c\x8B\x88\xA0$\x11a\0fW\x80c\x8B\x88\xA0$\x14a\x02/W\x80c\x8D\xA5\xCB[\x14a\x02BW\x80c\xB1?E\x04\x14a\x02SW\x80c\xF2\xFD\xE3\x8B\x14a\x02tW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x01\xEDW\x80cqP\x18\xA6\x14a\x02\x14W\x80cw\xD1u\x86\x14a\x02\x1CW`\0\x80\xFD[\x80bH%i\x14a\0\xD3W\x80c\n\x05\x93\xD1\x14a\x01+W\x80c\x10\xEAO\x8A\x14a\x01@W\x80c:\x0B\r\xDD\x14a\x01SW\x80ch0H5\x14a\x01{W\x80cl\x08\xA8y\x14a\x01\xBAW[`\0\x80\xFD[a\x01\x07a\0\xE16`\x04a\r\xE1V[`g` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x81\x16\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16\x82V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x93\x16\x83Ra\xFF\xFF\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01>a\x0196`\x04a\x0EnV[a\x02\x87V[\0[a\x01>a\x01N6`\x04a\x0F\x8FV[a\x07\xA9V[a\x01fa\x01a6`\x04a\x0F\xCDV[a\x07\xBBV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\"V[a\x01\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\"V[a\x01\xDDa\x01\xC86`\x04a\x0F\xF7V[`e` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\"V[a\x01\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01>a\x07\xF7V[a\x01>a\x02*6`\x04a\x10\x87V[a\x08\x0BV[a\x01>a\x02=6`\x04a\x11*V[a\x08\x1DV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\xA2V[a\x02fa\x02a6`\x04a\r\xE1V[a\t\xC3V[`@Q\x90\x81R` \x01a\x01\"V[a\x01>a\x02\x826`\x04a\x0F\xF7V[a\x0B\xC5V[3`\0\x90\x81R`e` R`@\x90 T`\xFF\x16\x80a\x02\xAFWP`3T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x03\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FEjector: Only owner or ejector c`D\x82\x01Rg\x18[\x88\x19Z\x99X\xDD`\xC2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0[\x81Q\x81\x10\x15a\x07\xA5W\x80`\0a\x03)\x82a\t\xC3V[\x90P`\0\x80\x80\x80[\x87\x87\x81Q\x81\x10a\x03CWa\x03Ca\x12\0V[` \x02` \x01\x01QQ\x81`\xFF\x16\x10\x15a\x06\xE3W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cT\x01\xED'\x8A\x8A\x81Q\x81\x10a\x03\x99Wa\x03\x99a\x12\0V[` \x02` \x01\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x03\xB5Wa\x03\xB5a\x12\0V[` \x02` \x01\x01Q\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xE7\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04(\x91\x90a\x12\x16V[3`\0\x90\x81R`e` R`@\x90 T`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x91P`\xFF\x16\x80\x15a\x04lWP`\xFF\x87\x16`\0\x90\x81R`g` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x15\x15[\x80\x15a\x04\x80WP\x85a\x04~\x82\x87a\x12UV[\x11[\x15a\x04\xD6WP`\xFF\x86\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92RB\x82R\x81\x83\x01\x88\x81R\x81T`\x01\x81\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q`\x02\x90\x93\x02\x90\x91\x01\x91\x82U\x91Q\x90\x82\x01U\x91Pa\x06\xE3V[a\x04\xE0\x81\x86a\x12UV[\x94Pa\x04\xEB\x84a\x12mV[\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cn;\x17\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c)k\xB0d\x8C\x8C\x81Q\x81\x10a\x05]Wa\x05]a\x12\0V[` \x02` \x01\x01Q\x86`\xFF\x16\x81Q\x81\x10a\x05yWa\x05ya\x12\0V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x9F\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE0\x91\x90a\x12\x91V[`@Q`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x8C\x90\x1B\x16` \x82\x01R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06&\x92\x91\x90a\x12\xAEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06TW=`\0\x80>=`\0\xFD[PPPP\x7F\x97\xDD\xB7\x11\xC6\x1A\x9D-~\xFF\xCB\xA3\xE0B\xA38b)\x7F\x89\x8DUVU\xCC\xA3\x9E\xC4E\x1FS\xB4\x89\x89\x81Q\x81\x10a\x06\x8BWa\x06\x8Ba\x12\0V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x06\xA7Wa\x06\xA7a\x12\0V[` \x02` \x01\x01Q\x88`@Qa\x06\xCA\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1Pa\x06\xDC\x81a\x13\x13V[\x90Pa\x031V[P\x80\x15\x80\x15a\x07\x01WP3`\0\x90\x81R`e` R`@\x90 T`\xFF\x16[\x15a\x07OW`\xFF\x85\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92RB\x82R\x81\x83\x01\x87\x81R\x81T`\x01\x81\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q`\x02\x90\x93\x02\x90\x91\x01\x91\x82U\x91Q\x91\x01U[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R\x82\x15\x15` \x82\x01R\x7F\x19\xDD\x87\xAEI\xED\x14\xA7\x95\xF8\xC2\xD5\xE8\x05[\xF2\xA4\xA9\xD0\x16A\xA0\n/\x8F\nZ{\xF7\xF7\x02I\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPP\x80a\x07\x9E\x90a\x133V[\x90Pa\x03\x14V[PPV[a\x07\xB1a\x0C>V[a\x07\xA5\x82\x82a\x0C\x98V[`f` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x07\xD7W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x02\x90\x91\x02\x01\x80T`\x01\x90\x91\x01T\x90\x92P\x90P\x82V[a\x07\xFFa\x0C>V[a\x08\t`\0a\x0C\xFCV[V[a\x08\x13a\x0C>V[a\x07\xA5\x82\x82a\rNV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08=WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08WWP0;\x15\x80\x15a\x08WWP`\0T`\xFF\x16`\x01\x14[a\x08\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x03\x08V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xDDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\xE6\x84a\x0C\xFCV[`\0[\x83Q\x81`\xFF\x16\x10\x15a\t.Wa\t\x1C\x84\x82`\xFF\x16\x81Q\x81\x10a\t\rWa\t\ra\x12\0V[` \x02` \x01\x01Q`\x01a\x0C\x98V[\x80a\t&\x81a\x13\x13V[\x91PPa\x08\xE9V[P`\0[\x82Q\x81`\xFF\x16\x10\x15a\tvWa\td\x81\x84\x83`\xFF\x16\x81Q\x81\x10a\tWWa\tWa\x12\0V[` \x02` \x01\x01Qa\rNV[\x80a\tn\x81a\x13\x13V[\x91PPa\t2V[P\x80\x15a\t\xBDW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`\xFF\x81\x16`\0\x90\x81R`g` R`@\x81 T\x81\x90a\t\xE8\x90c\xFF\xFF\xFF\xFF\x16Ba\x13NV[`@Qc\xD5\xEC\xCC\x05`\xE0\x1B\x81R`\xFF\x85\x16`\x04\x82\x01R\x90\x91P`\0\x90a'\x10\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD5\xEC\xCC\x05\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nYW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n}\x91\x90a\x12\x16V[`\xFF\x86\x16`\0\x90\x81R`g` R`@\x90 Ta\n\xAF\x91`\x01`\x01``\x1B\x03\x16\x90d\x01\0\0\0\0\x90\x04a\xFF\xFF\x16a\x13eV[a\n\xB9\x91\x90a\x13\x84V[`\xFF\x85\x16`\0\x90\x81R`f` R`@\x81 T\x91\x92P\x90\x81\x90a\n\xE0WP\x90\x94\x93PPPPV[`\xFF\x86\x16`\0\x90\x81R`f` R`@\x90 Ta\n\xFF\x90`\x01\x90a\x13NV[\x90P[`\xFF\x86\x16`\0\x90\x81R`f` R`@\x90 \x80T\x85\x91\x90\x83\x90\x81\x10a\x0B)Wa\x0B)a\x12\0V[\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x01T\x11\x15a\x0B\x9EW`\xFF\x86\x16`\0\x90\x81R`f` R`@\x90 \x80T\x82\x90\x81\x10a\x0BeWa\x0Bea\x12\0V[\x90`\0R` `\0 \x90`\x02\x02\x01`\x01\x01T\x82a\x0B\x82\x91\x90a\x12UV[\x91P\x80a\x0B\x8EWa\x0B\x9EV[a\x0B\x97\x81a\x13\xA6V[\x90Pa\x0B\x02V[\x82\x82\x10a\x0B\xB1WP`\0\x95\x94PPPPPV[a\x0B\xBB\x82\x84a\x13NV[\x96\x95PPPPPPV[a\x0B\xCDa\x0C>V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0C2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x03\x08V[a\x0C;\x81a\x0C\xFCV[PV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x08V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7Fvvhkm\"\xE1\x12A+\xD8t\xD7\x01w\xE0\x11\xAB\x06`,&\x06?\x19\xF08l\x9A<\xEEB\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\xFF\x82\x16`\0\x81\x81R`g` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x92\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81\x17d\x01\0\0\0\0a\xFF\xFF\x90\x93\x16\x92\x83\x02\x17\x90\x92U\x83Q\x94\x85R\x91\x84\x01R\x90\x82\x01R\x7F\xE6\x9C('\xA1\xE2\xFD\xD3\"e\xEB\xB4\xEE\xEA^\xE5d\xF0U\x1C\xF5\xDF\xEDAP\xF8\xE1\x16\xA6r\t\xEB\x90``\x01a\x0C\xF0V[\x805`\xFF\x81\x16\x81\x14a\r\xDCW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xF3W`\0\x80\xFD[a\r\xFC\x82a\r\xCBV[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EBWa\x0EBa\x0E\x03V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0EdWa\x0Eda\x0E\x03V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15a\x0E\x81W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x99W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\xADW`\0\x80\xFD[\x815a\x0E\xC0a\x0E\xBB\x82a\x0EJV[a\x0E\x19V[\x81\x81R`\x05\x91\x82\x1B\x84\x01\x85\x01\x91\x85\x82\x01\x91\x90\x89\x84\x11\x15a\x0E\xDFW`\0\x80\xFD[\x86\x86\x01[\x84\x81\x10\x15a\x0FkW\x805\x86\x81\x11\x15a\x0E\xFBW`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x8C\x13a\x0F\rW`\0\x80\x81\xFD[\x88\x81\x015`@a\x0F\x1Fa\x0E\xBB\x83a\x0EJV[\x82\x81R\x91\x85\x1B\x83\x01\x81\x01\x91\x8B\x81\x01\x90\x8F\x84\x11\x15a\x0F<W`\0\x80\x81\xFD[\x93\x82\x01\x93[\x83\x85\x10\x15a\x0FZW\x845\x82R\x93\x8C\x01\x93\x90\x8C\x01\x90a\x0FAV[\x88RPPP\x93\x88\x01\x93P\x87\x01a\x0E\xE3V[P\x90\x99\x98PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C;W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xA2W`\0\x80\xFD[\x825a\x0F\xAD\x81a\x0FzV[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x0F\xC2W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xE0W`\0\x80\xFD[a\x0F\xE9\x83a\r\xCBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x10\tW`\0\x80\xFD[\x815a\r\xFC\x81a\x0FzV[`\0`@\x82\x84\x03\x12\x15a\x10&W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10IWa\x10Ia\x0E\x03V[`@R\x90P\x80\x825c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10cW`\0\x80\xFD[\x81R` \x83\x015a\xFF\xFF\x81\x16\x81\x14a\x10zW`\0\x80\xFD[` \x91\x90\x91\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x10\x9AW`\0\x80\xFD[a\x10\xA3\x83a\r\xCBV[\x91Pa\x10\xB2\x84` \x85\x01a\x10\x14V[\x90P\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12a\x10\xCCW`\0\x80\xFD[\x815` a\x10\xDCa\x0E\xBB\x83a\x0EJV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\xFBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x11\x1FWa\x11\x11\x88\x82a\x10\x14V[\x83R\x91\x83\x01\x91`@\x01a\x10\xFFV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11?W`\0\x80\xFD[\x835a\x11J\x81a\x0FzV[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11hW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x11|W`\0\x80\xFD[\x815a\x11\x8Aa\x0E\xBB\x82a\x0EJV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8A\x83\x11\x15a\x11\xA9W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x11\xD0W\x845a\x11\xC1\x81a\x0FzV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x11\xAEV[\x96PPP`@\x87\x015\x92P\x80\x83\x11\x15a\x11\xE8W`\0\x80\xFD[PPa\x11\xF6\x86\x82\x87\x01a\x10\xBBV[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x12(W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\r\xFCW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x12hWa\x12ha\x12?V[P\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x12\x87Wa\x12\x87a\x12?V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x12\xA3W`\0\x80\xFD[\x81Qa\r\xFC\x81a\x0FzV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x12\xEAW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x12\xCEV[\x81\x81\x11\x15a\x12\xFCW`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x13*Wa\x13*a\x12?V[`\x01\x01\x92\x91PPV[`\0`\0\x19\x82\x14\x15a\x13GWa\x13Ga\x12?V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x13`Wa\x13`a\x12?V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x13\x7FWa\x13\x7Fa\x12?V[P\x02\x90V[`\0\x82a\x13\xA1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x81a\x13\xB5Wa\x13\xB5a\x12?V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \x04\x91\xD5w\xA3\x1C\xEA\xB4\xC0\0b\xE6^\x01\xBC\xCE\xFB\xFA\xE2\x9D\xE7\xF1\xCA\xC7\xF08\xAC\xD3\xEC3\xF9,dsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `EjectorUpdated(address,bool)` and selector `0x7676686b6d22e112412bd874d70177e011ab06602c26063f19f0386c9a3cee42`.
    ```solidity
    event EjectorUpdated(address ejector, bool status);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct EjectorUpdated {
        #[allow(missing_docs)]
        pub ejector: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub status: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EjectorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EjectorUpdated(address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    118u8, 118u8, 104u8, 107u8, 109u8, 34u8, 225u8, 18u8, 65u8, 43u8, 216u8, 116u8,
                    215u8, 1u8, 119u8, 224u8, 17u8, 171u8, 6u8, 96u8, 44u8, 38u8, 6u8, 63u8, 25u8,
                    240u8, 56u8, 108u8, 154u8, 60u8, 238u8, 66u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorEjected {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorEjected {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorEjected(bytes32,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    151u8, 221u8, 183u8, 17u8, 198u8, 26u8, 157u8, 45u8, 126u8, 255u8, 203u8,
                    163u8, 224u8, 66u8, 163u8, 56u8, 98u8, 41u8, 127u8, 137u8, 141u8, 85u8, 86u8,
                    85u8, 204u8, 163u8, 158u8, 196u8, 69u8, 31u8, 83u8, 180u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct QuorumEjection {
        #[allow(missing_docs)]
        pub ejectedOperators: u32,
        #[allow(missing_docs)]
        pub ratelimitHit: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumEjection {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumEjection(uint32,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    25u8, 221u8, 135u8, 174u8, 73u8, 237u8, 20u8, 167u8, 149u8, 248u8, 194u8,
                    213u8, 232u8, 5u8, 91u8, 242u8, 164u8, 169u8, 208u8, 22u8, 65u8, 160u8, 10u8,
                    47u8, 143u8, 10u8, 90u8, 123u8, 247u8, 247u8, 2u8, 73u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.ejectedOperators,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct QuorumEjectionParamsSet {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub rateLimitWindow: u32,
        #[allow(missing_docs)]
        pub ejectableStakePercent: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for QuorumEjectionParamsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumEjectionParamsSet(uint8,uint32,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    230u8, 156u8, 40u8, 39u8, 161u8, 226u8, 253u8, 211u8, 34u8, 101u8, 235u8,
                    180u8, 238u8, 234u8, 94u8, 229u8, 100u8, 240u8, 85u8, 28u8, 245u8, 223u8,
                    237u8, 65u8, 80u8, 248u8, 225u8, 22u8, 166u8, 114u8, 9u8, 235u8,
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
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.rateLimitWindow,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.ejectableStakePercent,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &QuorumEjectionParamsSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _registryCoordinator, address _stakeRegistry);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct amountEjectableForQuorumCall {
        pub _quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`amountEjectableForQuorum(uint8)`](amountEjectableForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct amountEjectableForQuorumReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<amountEjectableForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: amountEjectableForQuorumCall) -> Self {
                    (value._quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for amountEjectableForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<amountEjectableForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: amountEjectableForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for amountEjectableForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for amountEjectableForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = amountEjectableForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._quorumNumber,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ejectOperators(bytes32[][])` and selector `0x0a0593d1`.
    ```solidity
    function ejectOperators(bytes32[][] memory _operatorIds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ejectOperatorsCall {
        pub _operatorIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
    }
    ///Container type for the return parameters of the [`ejectOperators(bytes32[][])`](ejectOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ejectOperatorsReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        _operatorIds: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ejectOperatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ejectOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ejectOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ejectOperatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ejectOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._operatorIds
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address[],(uint32,uint16)[])` and selector `0x8b88a024`.
    ```solidity
    function initialize(address _owner, address[] memory _ejectors, IEjectionManager.QuorumEjectionParams[] memory _quorumEjectionParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _owner: alloy::sol_types::private::Address,
        pub _ejectors: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub _quorumEjectionParams: alloy::sol_types::private::Vec<
            <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initialize(address,address[],(uint32,uint16)[])`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isEjector(address)` and selector `0x6c08a879`.
    ```solidity
    function isEjector(address) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isEjectorCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isEjector(address)`](isEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isEjectorReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isEjectorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `quorumEjectionParams(uint8)` and selector `0x00482569`.
    ```solidity
    function quorumEjectionParams(uint8) external view returns (uint32 rateLimitWindow, uint16 ejectableStakePercent);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct quorumEjectionParamsCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`quorumEjectionParams(uint8)`](quorumEjectionParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct quorumEjectionParamsReturn {
        pub rateLimitWindow: u32,
        pub ejectableStakePercent: u16,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumEjectionParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumEjectionParamsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumEjectionParamsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<quorumEjectionParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumEjectionParamsReturn) -> Self {
                    (value.rateLimitWindow, value.ejectableStakePercent)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumEjectionParamsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumEjectionParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
    ```solidity
    function registryCoordinator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registryCoordinatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setEjector(address,bool)` and selector `0x10ea4f8a`.
    ```solidity
    function setEjector(address _ejector, bool _status) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setEjectorCall {
        pub _ejector: alloy::sol_types::private::Address,
        pub _status: bool,
    }
    ///Container type for the return parameters of the [`setEjector(address,bool)`](setEjectorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setEjectorReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEjectorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setQuorumEjectionParams(uint8,(uint32,uint16))` and selector `0x77d17586`.
    ```solidity
    function setQuorumEjectionParams(uint8 _quorumNumber, IEjectionManager.QuorumEjectionParams memory _quorumEjectionParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setQuorumEjectionParamsCall {
        pub _quorumNumber: u8,
        pub _quorumEjectionParams:
            <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setQuorumEjectionParams(uint8,(uint32,uint16))`](setQuorumEjectionParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setQuorumEjectionParamsReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setQuorumEjectionParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: setQuorumEjectionParamsCall) -> Self {
                    (value._quorumNumber, value._quorumEjectionParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setQuorumEjectionParamsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setQuorumEjectionParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setQuorumEjectionParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setQuorumEjectionParamsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setQuorumEjectionParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._quorumNumber,
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stakeEjectedForQuorum(uint8,uint256)` and selector `0x3a0b0ddd`.
    ```solidity
    function stakeEjectedForQuorum(uint8, uint256) external view returns (uint256 timestamp, uint256 stakeEjected);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct stakeEjectedForQuorumCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`stakeEjectedForQuorum(uint8,uint256)`](stakeEjectedForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct stakeEjectedForQuorumReturn {
        pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub stakeEjected: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeEjectedForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeEjectedForQuorumCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeEjectedForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeEjectedForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeEjectedForQuorumReturn) -> Self {
                    (value.timestamp, value.stakeEjected)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeEjectedForQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeEjectedForQuorumReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isEjector(_) => <isEjectorCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::setEjector(_) => <setEjectorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setQuorumEjectionParams(_) => {
                    <setQuorumEjectionParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeEjectedForQuorum(_) => {
                    <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<EjectionManagerCalls>] = &[
                {
                    fn quorumEjectionParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <quorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                        <setEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                        <isEjectorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(EjectionManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EjectionManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(EjectionManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
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
                    <ejectOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <quorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::amountEjectableForQuorum(inner) => {
                    <amountEjectableForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::ejectOperators(inner) => {
                    <ejectOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isEjector(inner) => {
                    <isEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorumEjectionParams(inner) => {
                    <quorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setEjector(inner) => {
                    <setEjectorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setQuorumEjectionParams(inner) => {
                    <setQuorumEjectionParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stakeEjectedForQuorum(inner) => {
                    <stakeEjectedForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                25u8, 221u8, 135u8, 174u8, 73u8, 237u8, 20u8, 167u8, 149u8, 248u8, 194u8, 213u8,
                232u8, 5u8, 91u8, 242u8, 164u8, 169u8, 208u8, 22u8, 65u8, 160u8, 10u8, 47u8, 143u8,
                10u8, 90u8, 123u8, 247u8, 247u8, 2u8, 73u8,
            ],
            [
                118u8, 118u8, 104u8, 107u8, 109u8, 34u8, 225u8, 18u8, 65u8, 43u8, 216u8, 116u8,
                215u8, 1u8, 119u8, 224u8, 17u8, 171u8, 6u8, 96u8, 44u8, 38u8, 6u8, 63u8, 25u8,
                240u8, 56u8, 108u8, 154u8, 60u8, 238u8, 66u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                151u8, 221u8, 183u8, 17u8, 198u8, 26u8, 157u8, 45u8, 126u8, 255u8, 203u8, 163u8,
                224u8, 66u8, 163u8, 56u8, 98u8, 41u8, 127u8, 137u8, 141u8, 85u8, 86u8, 85u8, 204u8,
                163u8, 158u8, 196u8, 69u8, 31u8, 83u8, 180u8,
            ],
            [
                230u8, 156u8, 40u8, 39u8, 161u8, 226u8, 253u8, 211u8, 34u8, 101u8, 235u8, 180u8,
                238u8, 234u8, 94u8, 229u8, 100u8, 240u8, 85u8, 28u8, 245u8, 223u8, 237u8, 65u8,
                80u8, 248u8, 225u8, 22u8, 166u8, 114u8, 9u8, 235u8,
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
                        topics, data, validate,
                    )
                    .map(Self::EjectorUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<OperatorEjected as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorEjected as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorEjected)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<QuorumEjection as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumEjection as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumEjection)
                }
                Some(<QuorumEjectionParamsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumEjectionParamsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumEjectionParamsSet)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<EjectionManagerInstance<T, P, N>>>
    {
        EjectionManagerInstance::<T, P, N>::deploy(provider, _registryCoordinator, _stakeRegistry)
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
        EjectionManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _registryCoordinator,
            _stakeRegistry,
        )
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
            f.debug_tuple("EjectionManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > EjectionManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`EjectionManager`](self) contract instance.

        See the [wrapper's documentation](`EjectionManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
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
            let call_builder = Self::deploy_builder(provider, _registryCoordinator, _stakeRegistry);
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
                        _stakeRegistry,
                    })[..],
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
        > EjectionManagerInstance<T, P, N>
    {
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
            self.call_builder(&amountEjectableForQuorumCall { _quorumNumber })
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
            _ejectors: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            _quorumEjectionParams: alloy::sol_types::private::Vec<
                <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _owner,
                _ejectors,
                _quorumEjectionParams,
            })
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
            self.call_builder(&setEjectorCall { _ejector, _status })
        }
        ///Creates a new call builder for the [`setQuorumEjectionParams`] function.
        pub fn setQuorumEjectionParams(
            &self,
            _quorumNumber: u8,
            _quorumEjectionParams: <IEjectionManager::QuorumEjectionParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setQuorumEjectionParamsCall, N> {
            self.call_builder(&setQuorumEjectionParamsCall {
                _quorumNumber,
                _quorumEjectionParams,
            })
        }
        ///Creates a new call builder for the [`stakeEjectedForQuorum`] function.
        pub fn stakeEjectedForQuorum(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeEjectedForQuorumCall, N> {
            self.call_builder(&stakeEjectedForQuorumCall { _0, _1 })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
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
        > EjectionManagerInstance<T, P, N>
    {
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
        pub fn EjectorUpdated_filter(&self) -> alloy_contract::Event<T, &P, EjectorUpdated, N> {
            self.event_filter::<EjectorUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorEjected`] event.
        pub fn OperatorEjected_filter(&self) -> alloy_contract::Event<T, &P, OperatorEjected, N> {
            self.event_filter::<OperatorEjected>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumEjection`] event.
        pub fn QuorumEjection_filter(&self) -> alloy_contract::Event<T, &P, QuorumEjection, N> {
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
