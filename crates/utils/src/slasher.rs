///Module containing a contract's types and functions.
/**

```solidity
library IOperatorSetManager {
    struct OperatorSet { address avs; bytes4 id; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IOperatorSetManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct OperatorSet { address avs; bytes4 id; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        pub avs: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::FixedBytes<4>,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::FixedBytes<4>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<4>,
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
        impl ::core::convert::From<OperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSet) -> Self {
                (value.avs, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { avs: tuple.0, id: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSet {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSet {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
        impl alloy_sol_types::SolType for OperatorSet {
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
        impl alloy_sol_types::SolStruct for OperatorSet {
            const NAME: &'static str = "OperatorSet";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSet(address avs,bytes4 id)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSet {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    4,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
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
    /**Creates a new wrapper around an on-chain [`IOperatorSetManager`](self) contract instance.

See the [wrapper's documentation](`IOperatorSetManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IOperatorSetManagerInstance<T, P, N> {
        IOperatorSetManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IOperatorSetManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IOperatorSetManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IOperatorSetManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IOperatorSetManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IOperatorSetManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IOperatorSetManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IOperatorSetManager`](self) contract instance.

See the [wrapper's documentation](`IOperatorSetManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IOperatorSetManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IOperatorSetManagerInstance<T, P, N> {
            IOperatorSetManagerInstance {
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
    > IOperatorSetManagerInstance<T, P, N> {
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
    > IOperatorSetManagerInstance<T, P, N> {
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
library IOperatorSetManager {
    struct OperatorSet {
        address avs;
        bytes4 id;
    }
}

interface Slasher {
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event RequestedBipsToSlashModified(uint32 epoch, address operator, IOperatorSetManager.OperatorSet operatorSet, address[] strategies, int32 bipsToModify);
    event SlashingExecuted(uint32 epoch, address operator, address strategy, uint64 slashingRate);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _strategyManager, address _delegationManager, address _operatorSetManager);

    function canWithdraw(address operator, address strategy, uint32 epoch) external view returns (bool);
    function delegation() external view returns (address);
    function executeSlashing(address operator, address[] memory strategies, uint32 epoch) external;
    function getPendingSlashingRate(address operator, address strategy, IOperatorSetManager.OperatorSet memory operatorSet, uint32 epoch) external view returns (uint32);
    function getRequestedSlashingRate(address operator, address strategy, IOperatorSetManager.OperatorSet memory operatorSet, uint32 epoch) external view returns (uint32);
    function getTotalPendingSlashingRate(address operator, address strategy, uint32 epoch) external view returns (uint32);
    function getWithdrawabilityAndScalingFactorAtEpoch(address operator, address strategy, uint32 epoch) external view returns (bool, uint64);
    function increaseRequestedBipsToSlash(address operator, bytes4 operatorSetID, address[] memory strategies, uint32 bipsToIncrease) external;
    function lastSlashed(address operator, address strategy) external view returns (uint32);
    function operatorSetManager() external view returns (address);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pendingShareScalingFactor(address operator, address strategy) external view returns (uint64);
    function reduceRequestedBipsToSlash(address operator, bytes4 operatorSetID, address[] memory strategies, uint32 epoch, uint32 bipsToReduce) external;
    function renounceOwnership() external;
    function requestedSlashedBips(address, address, uint32, bytes32) external view returns (uint32);
    function setPauserRegistry(address newPauserRegistry) external;
    function shareScalingFactor(address operator, address strategy) external view returns (uint64);
    function shareScalingFactorAtEpoch(address operator, address strategy, uint32 epoch) external view returns (uint64);
    function slashedEpochHistory(address, address, uint256) external view returns (uint32);
    function slashingRequestIds(address, address) external view returns (uint32 lastCreatedSlashingRequestId, uint32 lastExecutedSlashingRequestId);
    function slashingRequests(address, address, uint32) external view returns (uint32 id, uint64 slashingRate, uint64 scalingFactor);
    function strategyManager() external view returns (address);
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_strategyManager",
        "type": "address",
        "internalType": "contract IStrategyManager"
      },
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      },
      {
        "name": "_operatorSetManager",
        "type": "address",
        "internalType": "contract IOperatorSetManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "canWithdraw",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "executeSlashing",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getPendingSlashingRate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct IOperatorSetManager.OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRequestedSlashingRate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct IOperatorSetManager.OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalPendingSlashingRate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getWithdrawabilityAndScalingFactorAtEpoch",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "increaseRequestedBipsToSlash",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetID",
        "type": "bytes4",
        "internalType": "bytes4"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "bipsToIncrease",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastSlashed",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorSetManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IOperatorSetManager"
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
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "paused",
    "inputs": [],
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
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pendingShareScalingFactor",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "reduceRequestedBipsToSlash",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetID",
        "type": "bytes4",
        "internalType": "bytes4"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "bipsToReduce",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "requestedSlashedBips",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setPauserRegistry",
    "inputs": [
      {
        "name": "newPauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "shareScalingFactor",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "shareScalingFactorAtEpoch",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "epoch",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slashedEpochHistory",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slashingRequestIds",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "lastCreatedSlashingRequestId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "lastExecutedSlashingRequestId",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slashingRequests",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "id",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "slashingRate",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "scalingFactor",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategyManager"
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
    "type": "function",
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PauserRegistrySet",
    "inputs": [
      {
        "name": "pauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "newPauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RequestedBipsToSlashModified",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IOperatorSetManager.OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "bytes4",
            "internalType": "bytes4"
          }
        ]
      },
      {
        "name": "strategies",
        "type": "address[]",
        "indexed": false,
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "bipsToModify",
        "type": "int32",
        "indexed": false,
        "internalType": "int32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashingExecuted",
    "inputs": [
      {
        "name": "epoch",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "slashingRate",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
pub mod Slasher {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e06040523480156200001157600080fd5b5060405162002b1738038062002b1783398101604081905262000034916200006b565b6001600160a01b0392831660805290821660a0521660c052620000bf565b6001600160a01b03811681146200006857600080fd5b50565b6000806000606084860312156200008157600080fd5b83516200008e8162000052565b6020850151909350620000a18162000052565b6040850151909250620000b48162000052565b809150509250925092565b60805160a05160c051612a1a620000fd600039600081816104c50152818161079c0152611a65015260006104ec0152600061026c0152612a1a6000f3fe608060405234801561001057600080fd5b50600436106101cf5760003560e01c80635c975abb1161010457806390e7cde1116100a2578063e49a1e8411610071578063e49a1e841461050e578063ec65b53d14610521578063f2fde38b14610561578063fabc1cbc1461057457600080fd5b806390e7cde11461049a5780639d086ecb146104ad578063c78d4bcd146104c0578063df5cf723146104e757600080fd5b806379c415ec116100de57806379c415ec1461040a5780637ef639a61461041d578063886f1195146104765780638da5cb5b1461048957600080fd5b80635c975abb146103de5780636c0d75d0146103ef578063715018a61461040257600080fd5b80633dd9e7c5116101715780634dcaafb81161014b5780634dcaafb81461037d578063595c6a67146103905780635ab112d6146103985780635ac86ab7146103ab57600080fd5b80633dd9e7c5146102d85780633f2201bb146102eb5780634d54dc3c1461036a57600080fd5b8063287a96da116101ad578063287a96da14610229578063334f00d61461023c57806339b70e38146102675780633be2073b146102a657600080fd5b806310d67a2f146101d4578063136439dd146101e95780632421a64c146101fc575b600080fd5b6101e76101e2366004612045565b610587565b005b6101e76101f7366004612062565b610643565b61020f61020a366004612094565b610782565b60405163ffffffff90911681526020015b60405180910390f35b6101e76102373660046121c9565b610897565b61024f61024a36600461222e565b6109d3565b6040516001600160401b039091168152602001610220565b61028e7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610220565b6102b96102b4366004612267565b610a23565b6040805192151583526001600160401b03909116602083015201610220565b61024f6102e636600461222e565b610aab565b61033e6102f9366004612267565b609860209081526000938452604080852082529284528284209052825290205463ffffffff8116906001600160401b03600160201b8204811691600160601b90041683565b6040805163ffffffff90941684526001600160401b039283166020850152911690820152606001610220565b61020f610378366004612094565b610b36565b6101e761038b3660046122ae565b610b67565b6101e7610fa6565b61020f6103a636600461222e565b61106d565b6103ce6103b9366004612304565b606654600160ff9092169190911b9081161490565b6040519015158152602001610220565b606654604051908152602001610220565b61020f6103fd366004612327565b611112565b6101e7611168565b6103ce610418366004612267565b61117c565b61045961042b36600461222e565b609760209081526000928352604080842090915290825290205463ffffffff80821691600160201b90041682565b6040805163ffffffff938416815292909116602083015201610220565b60655461028e906001600160a01b031681565b6033546001600160a01b031661028e565b61020f6104a8366004612267565b6111b0565b6101e76104bb366004612368565b61120f565b61028e7f000000000000000000000000000000000000000000000000000000000000000081565b61028e7f000000000000000000000000000000000000000000000000000000000000000081565b61024f61051c366004612267565b611411565b61020f61052f3660046123e9565b609b60209081526000948552604080862082529385528385208152918452828420909152825290205463ffffffff1681565b6101e761056f366004612045565b611481565b6101e7610582366004612062565b6114f7565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105da573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105fe9190612438565b6001600160a01b0316336001600160a01b0316146106375760405162461bcd60e51b815260040161062e90612455565b60405180910390fd5b61064081611653565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561068b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106af919061249f565b6106cb5760405162461bcd60e51b815260040161062e906124c1565b606654818116146107445760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161062e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b604051633f76c6c760e01b81526000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633f76c6c7906107d7908890879089908890600401612509565b602060405180830381865afa1580156107f4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108189190612567565b6001600160a01b038087166000908152609b60209081526040808320938916835292815282822063ffffffff87168352905290812061ffff92909216919061086d6108683688900388018861258b565b61174a565b815260208101919091526040016000205461088e919063ffffffff166125fd565b95945050505050565b60008163ffffffff16116109215760405162461bcd60e51b815260206004820152604560248201527f536c61736865722e696e63726561736552657175657374656442697073546f5360448201527f6c6173683a2062697073546f496e637265617365206d75737420626520706f73606482015264697469766560d81b608482015260a40161062e565b6127108163ffffffff16106109b95760405162461bcd60e51b815260206004820152605260248201527f536c61736865722e696e63726561736552657175657374656442697073546f5360448201527f6c6173683a2062697073546f496e637265617365206d757374206265206c657360648201527139903a3430b7102124a829afa320a1aa27a960711b608482015260a40161062e565b6109cd8484846109c76117db565b856117eb565b50505050565b6001600160a01b0380831660009081526099602090815260408083209385168352929052908120546001600160401b031680610a1a57670de0b6b3a7640000915050610a1d565b90505b92915050565b6000806001670de0b6b3a76400008280610a3e898989611c67565b915091508015610a9d57610a53898984611d32565b6001600160a01b038a81166000908152609860209081526040808320938d16835292815282822063ffffffff8716835290522054909450600160601b90046001600160401b031692505b509197909650945050505050565b6001600160a01b03808316600081815260996020908152604080832094861680845294825280832054938352609882528083209483529390529182208291610b2e916001600160401b039091169083610b026117db565b63ffffffff168152602081019190915260400160002054600160201b90046001600160401b0316611d92565b949350505050565b600080610b4586868686610782565b90506305f5e10063ffffffff82161061088e57506305f5e10095945050505050565b610b7081611edf565b63ffffffff16610b7e6117db565b63ffffffff1611610c1d5760405162461bcd60e51b815260206004820152605760248201527f536c61736865722e65786563757465536c617368696e673a2063757272656e7460448201527f2065706f6368206d7573742062652067726561746572207468616e207468652060648201527f6d696e696d756d20657865637574696f6e2065706f6368000000000000000000608482015260a40161062e565b60005b82518110156109cd576000838281518110610c3d57610c3d612629565b6020908102919091018101516001600160a01b03808816600081815260988552604080822093851680835293865280822063ffffffff808b168452908752818320825160608101845290548083168083526001600160401b03600160201b8084048216858d0152600160601b909304168386015295855260978952838520968552959097529120549395509092610cd892900416600161263f565b63ffffffff1614610d515760405162461bcd60e51b815260206004820152603860248201527f536c61736865722e65786563757465536c617368696e673a206d75737420657860448201527f656375746520736c617368696e677320696e206f726465720000000000000000606482015260840161062e565b80516001600160a01b0380881660009081526097602090815260408083209387168352928152919020805463ffffffff909316600160201b0267ffffffff0000000019909316929092179091558101516305f5e1006001600160401b03919091161115610dc7576305f5e1006020820152610de0565b60208101516001600160401b0316610de0575050610f96565b6000610dec87846109d3565b90506000610dfe828460200151611d92565b9050609a6000896001600160a01b03166001600160a01b031681526020019081526020016000206000856001600160a01b03166001600160a01b031681526020019081526020016000208690806001815401808255809150506001900390600052602060002090600891828204019190066004029091909190916101000a81548163ffffffff021916908363ffffffff16021790555080609960008a6001600160a01b03166001600160a01b031681526020019081526020016000206000866001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a8154816001600160401b0302191690836001600160401b031602179055508083604001906001600160401b031690816001600160401b0316815250507f2f679597a08f229c142b2f79a954c91a30bbda82795ef8dee2775b84db9699248689868660200151604051610f89949392919063ffffffff9490941684526001600160a01b039283166020850152911660408301526001600160401b0316606082015260800190565b60405180910390a1505050505b610f9f81612667565b9050610c20565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610fee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611012919061249f565b61102e5760405162461bcd60e51b815260040161062e906124c1565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6001600160a01b038083166000908152609a60209081526040808320938516835292905290812054806110a4576000915050610a1d565b6001600160a01b038085166000908152609a602090815260408083209387168352929052206110d4600183612682565b815481106110e4576110e4612629565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16915050610a1d565b609a602052826000526040600020602052816000526040600020818154811061113a57600080fd5b906000526020600020906008918282040191900660040292509250509054906101000a900463ffffffff1681565b611170611eec565b61117a6000611f46565b565b60006001818061118d878787611c67565b9150915080156111a5576111a2878784611d32565b92505b509095945050505050565b6001600160a01b038381166000908152609860209081526040808320938616835292815282822063ffffffff85168352905290812054600160201b90046001600160401b03166305f5e1008110610b2e57506305f5e100949350505050565b60006112196117db565b90508063ffffffff168363ffffffff16148061124a575063ffffffff811661124284600161263f565b63ffffffff16145b6112d65760405162461bcd60e51b815260206004820152605160248201527f536c61736865722e72656475636552657175657374656442697073546f536c6160448201527f73683a2063616e206f6e6c792072656475636520666f722063757272656e74206064820152700dee440e0e4caecd2deeae640cae0dec6d607b1b608482015260a40161062e565b60008263ffffffff161161135c5760405162461bcd60e51b815260206004820152604160248201527f536c61736865722e72656475636552657175657374656442697073546f536c6160448201527f73683a2062697073546f526564756365206d757374206265206e6567617469766064820152606560f81b608482015260a40161062e565b63800000008263ffffffff16106113f45760405162461bcd60e51b815260206004820152605060248201527f536c61736865722e72656475636552657175657374656442697073546f536c6160448201527f73683a2062697073546f526564756365206d757374206265206c65737320746860648201526f30b71036b4b734b6bab69034b73a199960811b608482015260a40161062e565b6114098686868661140487612699565b6117eb565b505050505050565b6000670de0b6b3a76400008180611429878787611c67565b9150915080156111a557506001600160a01b03958616600090815260986020908152604080832097909816825295865286812063ffffffff92909216815294525050502054600160601b90046001600160401b031690565b611489611eec565b6001600160a01b0381166114ee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161062e565b61064081611f46565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561154a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061156e9190612438565b6001600160a01b0316336001600160a01b03161461159e5760405162461bcd60e51b815260040161062e90612455565b60665419811960665419161461161c5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161062e565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610777565b6001600160a01b0381166116e15760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161062e565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b604080516001808252818301909252600091829190816020015b604080518082019091526000808252602082015281526020019060019003908161176457905050905082816000815181106117a1576117a1612629565b6020026020010181905250806040516020016117bd91906126df565b60405160208183030381529060405280519060200120915050919050565b60006117e642611f98565b905090565b8060030b60001415611867576040805162461bcd60e51b81526020600482015260248101919091527f536c61736865722e5f6d6f6469667952657175657374656442697073546f536c60448201527f6173683a2063616e6e6f74206d6f6469667920736c617368696e672062792030606482015260840161062e565b604080518082019091523381526001600160e01b031985166020820152600061188f8261174a565b905060005b8551811015611c1e5760008682815181106118b1576118b1612629565b6020908102919091018101516001600160a01b03808c166000908152609b84526040808220928416825291845281812063ffffffff808c1683529085528282208883529094529081205491935091169061190b878361272c565b905060008160030b12156119295761192282612699565b9650600090505b6001600160a01b038b81166000818152609b6020908152604080832094881680845294825280832063ffffffff8e81168086529184528285208c86528452828520805463ffffffff1916898316179055948452609883528184209584529482528083209483529381529083902083516060810185529054928316808252600160201b84046001600160401b0390811693830193909352600160601b90930490911692810192909252611a4e576001600160a01b03808d166000908152609760209081526040808320938816835292905290812054611a0e9063ffffffff16600161263f565b6001600160a01b03808f166000908152609760209081526040808320938a16835292905220805463ffffffff90921663ffffffff19909216821790558252505b604051633f76c6c760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633f76c6c790611aa0908f908b9089908f90600401612775565b602060405180830381865afa158015611abd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ae19190612567565b61ffff168860030b611af391906127b2565b8160200151611b029190612849565b81602001906001600160401b031690816001600160401b03168152505080609860008e6001600160a01b03166001600160a01b031681526020019081526020016000206000866001600160a01b03166001600160a01b0316815260200190815260200160002060008b63ffffffff1663ffffffff16815260200190815260200160002060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a8154816001600160401b0302191690836001600160401b03160217905550604082015181600001600c6101000a8154816001600160401b0302191690836001600160401b031602179055509050505050505080611c1790612667565b9050611894565b507f51b15dc60a707d9c43660fdd6af7cf86060e2778638d04ef462faa56241ea6bf8488848887604051611c56959493929190612891565b60405180910390a150505050505050565b6001600160a01b038084166000908152609a602090815260408083209386168352929052908120548190819081905b8015611d25576001600160a01b038089166000908152609a60209081526040808320938b16835292905220611ccc600183612682565b81548110611cdc57611cdc612629565b6000918252602090912060088204015460079091166004026101000a900463ffffffff908116935086168311611d155760019150611d25565b611d1e81612910565b9050611c96565b5090969095509350505050565b6001600160a01b03928316600081815260976020908152604080832095909616808352948152858220549282526098815285822094825293845284812063ffffffff93841682529093529290912054600160201b90920481169116111590565b60006001600160401b038216611de05760405162461bcd60e51b815260206004820152601360248201527263616e6e6f7420736c61736820666f7220302560681b604482015260640161062e565b6305f5e1006001600160401b0383161115611e495760405162461bcd60e51b815260206004820152602360248201527f63616e6e6f7420736c617368206d6f7265207468616e2031303025206174206f6044820152626e636560e81b606482015260840161062e565b60006001600160401b0383166305f5e1001480611ea657506001600160401b03808416908516611e8d670de0b6b3a76400006bffffffffffffffffffffffff612927565b611e999060001961295c565b611ea3919061295c565b10155b15611eb957506001600160401b03610a1a565b611ec7836305f5e100612970565b611ed56305f5e10086612998565b610b2e91906129be565b6000610a1d82600261263f565b6033546001600160a01b0316331461117a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161062e565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6000635fc630408210156120145760405162461bcd60e51b815260206004820152603d60248201527f45706f63685574696c732e67657445706f636846726f6d54696d657374616d7060448201527f3a2074696d657374616d70206973206265666f72652067656e65736973000000606482015260840161062e565b62093a80612026635fc6304084612682565b610a1d919061295c565b6001600160a01b038116811461064057600080fd5b60006020828403121561205757600080fd5b8135610a1a81612030565b60006020828403121561207457600080fd5b5035919050565b803563ffffffff8116811461208f57600080fd5b919050565b60008060008084860360a08112156120ab57600080fd5b85356120b681612030565b945060208601356120c681612030565b93506040603f19820112156120da57600080fd5b506040850191506120ed6080860161207b565b905092959194509250565b80356001600160e01b03198116811461208f57600080fd5b634e487b7160e01b600052604160045260246000fd5b600082601f83011261213757600080fd5b813560206001600160401b038083111561215357612153612110565b8260051b604051601f19603f8301168101818110848211171561217857612178612110565b60405293845285810183019383810192508785111561219657600080fd5b83870191505b848210156121be5781356121af81612030565b8352918301919083019061219c565b979650505050505050565b600080600080608085870312156121df57600080fd5b84356121ea81612030565b93506121f8602086016120f8565b925060408501356001600160401b0381111561221357600080fd5b61221f87828801612126565b9250506120ed6060860161207b565b6000806040838503121561224157600080fd5b823561224c81612030565b9150602083013561225c81612030565b809150509250929050565b60008060006060848603121561227c57600080fd5b833561228781612030565b9250602084013561229781612030565b91506122a56040850161207b565b90509250925092565b6000806000606084860312156122c357600080fd5b83356122ce81612030565b925060208401356001600160401b038111156122e957600080fd5b6122f586828701612126565b9250506122a56040850161207b565b60006020828403121561231657600080fd5b813560ff81168114610a1a57600080fd5b60008060006060848603121561233c57600080fd5b833561234781612030565b9250602084013561235781612030565b929592945050506040919091013590565b600080600080600060a0868803121561238057600080fd5b853561238b81612030565b9450612399602087016120f8565b935060408601356001600160401b038111156123b457600080fd5b6123c088828901612126565b9350506123cf6060870161207b565b91506123dd6080870161207b565b90509295509295909350565b600080600080608085870312156123ff57600080fd5b843561240a81612030565b9350602085013561241a81612030565b92506124286040860161207b565b9396929550929360600135925050565b60006020828403121561244a57600080fd5b8151610a1a81612030565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156124b157600080fd5b81518015158114610a1a57600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b6001600160a01b03858116825260a0820190853561252681612030565b81811660208501525063ffffffff60e01b612543602088016120f8565b16604084015280851660608401525063ffffffff8316608083015295945050505050565b60006020828403121561257957600080fd5b815161ffff81168114610a1a57600080fd5b60006040828403121561259d57600080fd5b604051604081018181106001600160401b03821117156125bf576125bf612110565b60405282356125cd81612030565b81526125db602084016120f8565b60208201529392505050565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff80831681851681830481118215151615612620576126206125e7565b02949350505050565b634e487b7160e01b600052603260045260246000fd5b600063ffffffff80831681851680830382111561265e5761265e6125e7565b01949350505050565b600060001982141561267b5761267b6125e7565b5060010190565b600082821015612694576126946125e7565b500390565b60008160030b637fffffff198114156126b4576126b46125e7565b60000392915050565b80516001600160a01b031682526020908101516001600160e01b031916910152565b602080825282518282018190526000919060409081850190868401855b8281101561271f5761270f8483516126bd565b92840192908501906001016126fc565b5091979650505050505050565b60008160030b8360030b6000821282637fffffff03821381151615612753576127536125e7565b82637fffffff1903821281161561276c5761276c6125e7565b50019392505050565b6001600160a01b03858116825260a082019061279460208401876126bd565b80851660608401525063ffffffff8316608083015295945050505050565b60008160070b8360070b677fffffffffffffff6000821360008413838304851182821616156127e3576127e36125e7565b677fffffffffffffff196000851282811687830587121615612807576128076125e7565b60008712925085820587128484161615612823576128236125e7565b85850587128184161615612839576128396125e7565b5050509290910295945050505050565b60008160070b8360070b6000821282677fffffffffffffff03821381151615612874576128746125e7565b82677fffffffffffffff1903821281161561276c5761276c6125e7565b600060c0820163ffffffff88168352602060018060a01b03808916828601526128bd60408601896126bd565b60c060808601528651928390528187019260e086019060005b818110156128f45785518416835294840194918401916001016128d6565b5050809450505050508260030b60a08301529695505050505050565b60008161291f5761291f6125e7565b506000190190565b6000816000190483118215151615612941576129416125e7565b500290565b634e487b7160e01b600052601260045260246000fd5b60008261296b5761296b612946565b500490565b60006001600160401b0383811690831681811015612990576129906125e7565b039392505050565b60006001600160401b0380831681851681830481118215151615612620576126206125e7565b60006001600160401b03808416806129d8576129d8612946565b9216919091049291505056fea2646970667358221220cefc068490f4fd35869e596174bfab1f011fb4a666619d26f06a01d7f6d26ddf64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0+\x178\x03\x80b\0+\x17\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0kV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x16`\xC0Rb\0\0\xBFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0hW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\0\x81W`\0\x80\xFD[\x83Qb\0\0\x8E\x81b\0\0RV[` \x85\x01Q\x90\x93Pb\0\0\xA1\x81b\0\0RV[`@\x85\x01Q\x90\x92Pb\0\0\xB4\x81b\0\0RV[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa*\x1Ab\0\0\xFD`\09`\0\x81\x81a\x04\xC5\x01R\x81\x81a\x07\x9C\x01Ra\x1Ae\x01R`\0a\x04\xEC\x01R`\0a\x02l\x01Ra*\x1A`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xCFW`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\x01\x04W\x80c\x90\xE7\xCD\xE1\x11a\0\xA2W\x80c\xE4\x9A\x1E\x84\x11a\0qW\x80c\xE4\x9A\x1E\x84\x14a\x05\x0EW\x80c\xECe\xB5=\x14a\x05!W\x80c\xF2\xFD\xE3\x8B\x14a\x05aW\x80c\xFA\xBC\x1C\xBC\x14a\x05tW`\0\x80\xFD[\x80c\x90\xE7\xCD\xE1\x14a\x04\x9AW\x80c\x9D\x08n\xCB\x14a\x04\xADW\x80c\xC7\x8DK\xCD\x14a\x04\xC0W\x80c\xDF\\\xF7#\x14a\x04\xE7W`\0\x80\xFD[\x80cy\xC4\x15\xEC\x11a\0\xDEW\x80cy\xC4\x15\xEC\x14a\x04\nW\x80c~\xF69\xA6\x14a\x04\x1DW\x80c\x88o\x11\x95\x14a\x04vW\x80c\x8D\xA5\xCB[\x14a\x04\x89W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xDEW\x80cl\ru\xD0\x14a\x03\xEFW\x80cqP\x18\xA6\x14a\x04\x02W`\0\x80\xFD[\x80c=\xD9\xE7\xC5\x11a\x01qW\x80cM\xCA\xAF\xB8\x11a\x01KW\x80cM\xCA\xAF\xB8\x14a\x03}W\x80cY\\jg\x14a\x03\x90W\x80cZ\xB1\x12\xD6\x14a\x03\x98W\x80cZ\xC8j\xB7\x14a\x03\xABW`\0\x80\xFD[\x80c=\xD9\xE7\xC5\x14a\x02\xD8W\x80c?\"\x01\xBB\x14a\x02\xEBW\x80cMT\xDC<\x14a\x03jW`\0\x80\xFD[\x80c(z\x96\xDA\x11a\x01\xADW\x80c(z\x96\xDA\x14a\x02)W\x80c3O\0\xD6\x14a\x02<W\x80c9\xB7\x0E8\x14a\x02gW\x80c;\xE2\x07;\x14a\x02\xA6W`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x01\xD4W\x80c\x13d9\xDD\x14a\x01\xE9W\x80c$!\xA6L\x14a\x01\xFCW[`\0\x80\xFD[a\x01\xE7a\x01\xE26`\x04a EV[a\x05\x87V[\0[a\x01\xE7a\x01\xF76`\x04a bV[a\x06CV[a\x02\x0Fa\x02\n6`\x04a \x94V[a\x07\x82V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE7a\x0276`\x04a!\xC9V[a\x08\x97V[a\x02Oa\x02J6`\x04a\".V[a\t\xD3V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02 V[a\x02\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02 V[a\x02\xB9a\x02\xB46`\x04a\"gV[a\n#V[`@\x80Q\x92\x15\x15\x83R`\x01`\x01`@\x1B\x03\x90\x91\x16` \x83\x01R\x01a\x02 V[a\x02Oa\x02\xE66`\x04a\".V[a\n\xABV[a\x03>a\x02\xF96`\x04a\"gV[`\x98` \x90\x81R`\0\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 Tc\xFF\xFF\xFF\xFF\x81\x16\x90`\x01`\x01`@\x1B\x03`\x01` \x1B\x82\x04\x81\x16\x91`\x01``\x1B\x90\x04\x16\x83V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x94\x16\x84R`\x01`\x01`@\x1B\x03\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x02 V[a\x02\x0Fa\x03x6`\x04a \x94V[a\x0B6V[a\x01\xE7a\x03\x8B6`\x04a\"\xAEV[a\x0BgV[a\x01\xE7a\x0F\xA6V[a\x02\x0Fa\x03\xA66`\x04a\".V[a\x10mV[a\x03\xCEa\x03\xB96`\x04a#\x04V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02 V[`fT`@Q\x90\x81R` \x01a\x02 V[a\x02\x0Fa\x03\xFD6`\x04a#'V[a\x11\x12V[a\x01\xE7a\x11hV[a\x03\xCEa\x04\x186`\x04a\"gV[a\x11|V[a\x04Ya\x04+6`\x04a\".V[`\x97` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x91`\x01` \x1B\x90\x04\x16\x82V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x02 V[`eTa\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x8EV[a\x02\x0Fa\x04\xA86`\x04a\"gV[a\x11\xB0V[a\x01\xE7a\x04\xBB6`\x04a#hV[a\x12\x0FV[a\x02\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02Oa\x05\x1C6`\x04a\"gV[a\x14\x11V[a\x02\x0Fa\x05/6`\x04a#\xE9V[`\x9B` \x90\x81R`\0\x94\x85R`@\x80\x86 \x82R\x93\x85R\x83\x85 \x81R\x91\x84R\x82\x84 \x90\x91R\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xE7a\x05o6`\x04a EV[a\x14\x81V[a\x01\xE7a\x05\x826`\x04a bV[a\x14\xF7V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFE\x91\x90a$8V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x067W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$UV[`@Q\x80\x91\x03\x90\xFD[a\x06@\x81a\x16SV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xAF\x91\x90a$\x9FV[a\x06\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$\xC1V[`fT\x81\x81\x16\x14a\x07DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06.V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`@Qc?v\xC6\xC7`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?v\xC6\xC7\x90a\x07\xD7\x90\x88\x90\x87\x90\x89\x90\x88\x90`\x04\x01a%\tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x18\x91\x90a%gV[`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x81R\x82\x82 c\xFF\xFF\xFF\xFF\x87\x16\x83R\x90R\x90\x81 a\xFF\xFF\x92\x90\x92\x16\x91\x90a\x08ma\x08h6\x88\x90\x03\x88\x01\x88a%\x8BV[a\x17JV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta\x08\x8E\x91\x90c\xFF\xFF\xFF\xFF\x16a%\xFDV[\x95\x94PPPPPV[`\0\x81c\xFF\xFF\xFF\xFF\x16\x11a\t!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FSlasher.increaseRequestedBipsToS`D\x82\x01R\x7Flash: bipsToIncrease must be pos`d\x82\x01Rditive`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[a'\x10\x81c\xFF\xFF\xFF\xFF\x16\x10a\t\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FSlasher.increaseRequestedBipsToS`D\x82\x01R\x7Flash: bipsToIncrease must be les`d\x82\x01Rq9\x90:40\xB7\x10!$\xA8)\xAF\xA3 \xA1\xAA'\xA9`q\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[a\t\xCD\x84\x84\x84a\t\xC7a\x17\xDBV[\x85a\x17\xEBV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 T`\x01`\x01`@\x1B\x03\x16\x80a\n\x1AWg\r\xE0\xB6\xB3\xA7d\0\0\x91PPa\n\x1DV[\x90P[\x92\x91PPV[`\0\x80`\x01g\r\xE0\xB6\xB3\xA7d\0\0\x82\x80a\n>\x89\x89\x89a\x1CgV[\x91P\x91P\x80\x15a\n\x9DWa\nS\x89\x89\x84a\x1D2V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x8D\x16\x83R\x92\x81R\x82\x82 c\xFF\xFF\xFF\xFF\x87\x16\x83R\x90R T\x90\x94P`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x92P[P\x91\x97\x90\x96P\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x99` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\x98\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 \x82\x91a\x0B.\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90\x83a\x0B\x02a\x17\xDBV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1D\x92V[\x94\x93PPPPV[`\0\x80a\x0BE\x86\x86\x86\x86a\x07\x82V[\x90Pc\x05\xF5\xE1\0c\xFF\xFF\xFF\xFF\x82\x16\x10a\x08\x8EWPc\x05\xF5\xE1\0\x95\x94PPPPPV[a\x0Bp\x81a\x1E\xDFV[c\xFF\xFF\xFF\xFF\x16a\x0B~a\x17\xDBV[c\xFF\xFF\xFF\xFF\x16\x11a\x0C\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FSlasher.executeSlashing: current`D\x82\x01R\x7F epoch must be greater than the `d\x82\x01R\x7Fminimum execution epoch\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06.V[`\0[\x82Q\x81\x10\x15a\t\xCDW`\0\x83\x82\x81Q\x81\x10a\x0C=Wa\x0C=a&)V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x81\x81R`\x98\x85R`@\x80\x82 \x93\x85\x16\x80\x83R\x93\x86R\x80\x82 c\xFF\xFF\xFF\xFF\x80\x8B\x16\x84R\x90\x87R\x81\x83 \x82Q``\x81\x01\x84R\x90T\x80\x83\x16\x80\x83R`\x01`\x01`@\x1B\x03`\x01` \x1B\x80\x84\x04\x82\x16\x85\x8D\x01R`\x01``\x1B\x90\x93\x04\x16\x83\x86\x01R\x95\x85R`\x97\x89R\x83\x85 \x96\x85R\x95\x90\x97R\x91 T\x93\x95P\x90\x92a\x0C\xD8\x92\x90\x04\x16`\x01a&?V[c\xFF\xFF\xFF\xFF\x16\x14a\rQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FSlasher.executeSlashing: must ex`D\x82\x01R\x7Fecute slashings in order\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06.V[\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x81R\x91\x90 \x80Tc\xFF\xFF\xFF\xFF\x90\x93\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x81\x01Qc\x05\xF5\xE1\0`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x11\x15a\r\xC7Wc\x05\xF5\xE1\0` \x82\x01Ra\r\xE0V[` \x81\x01Q`\x01`\x01`@\x1B\x03\x16a\r\xE0WPPa\x0F\x96V[`\0a\r\xEC\x87\x84a\t\xD3V[\x90P`\0a\r\xFE\x82\x84` \x01Qa\x1D\x92V[\x90P`\x9A`\0\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x86\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x99`\0\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80\x83`@\x01\x90`\x01`\x01`@\x1B\x03\x16\x90\x81`\x01`\x01`@\x1B\x03\x16\x81RPP\x7F/g\x95\x97\xA0\x8F\"\x9C\x14+/y\xA9T\xC9\x1A0\xBB\xDA\x82y^\xF8\xDE\xE2w[\x84\xDB\x96\x99$\x86\x89\x86\x86` \x01Q`@Qa\x0F\x89\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x85\x01R\x91\x16`@\x83\x01R`\x01`\x01`@\x1B\x03\x16``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPP[a\x0F\x9F\x81a&gV[\x90Pa\x0C V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x12\x91\x90a$\x9FV[a\x10.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$\xC1V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 T\x80a\x10\xA4W`\0\x91PPa\n\x1DV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R a\x10\xD4`\x01\x83a&\x82V[\x81T\x81\x10a\x10\xE4Wa\x10\xE4a&)V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91PPa\n\x1DV[`\x9A` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x11:W`\0\x80\xFD[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x92P\x92PP\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x11pa\x1E\xECV[a\x11z`\0a\x1FFV[V[`\0`\x01\x81\x80a\x11\x8D\x87\x87\x87a\x1CgV[\x91P\x91P\x80\x15a\x11\xA5Wa\x11\xA2\x87\x87\x84a\x1D2V[\x92P[P\x90\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x81R\x82\x82 c\xFF\xFF\xFF\xFF\x85\x16\x83R\x90R\x90\x81 T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16c\x05\xF5\xE1\0\x81\x10a\x0B.WPc\x05\xF5\xE1\0\x94\x93PPPPV[`\0a\x12\x19a\x17\xDBV[\x90P\x80c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x14\x80a\x12JWPc\xFF\xFF\xFF\xFF\x81\x16a\x12B\x84`\x01a&?V[c\xFF\xFF\xFF\xFF\x16\x14[a\x12\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FSlasher.reduceRequestedBipsToSla`D\x82\x01R\x7Fsh: can only reduce for current `d\x82\x01Rp\r\xEED\x0E\x0EL\xAE\xCD-\xEE\xAEd\x0C\xAE\r\xECm`{\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[`\0\x82c\xFF\xFF\xFF\xFF\x16\x11a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FSlasher.reduceRequestedBipsToSla`D\x82\x01R\x7Fsh: bipsToReduce must be negativ`d\x82\x01R`e`\xF8\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[c\x80\0\0\0\x82c\xFF\xFF\xFF\xFF\x16\x10a\x13\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FSlasher.reduceRequestedBipsToSla`D\x82\x01R\x7Fsh: bipsToReduce must be less th`d\x82\x01Ro0\xB7\x106\xB4\xB74\xB6\xBA\xB6\x904\xB7:\x19\x99`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[a\x14\t\x86\x86\x86\x86a\x14\x04\x87a&\x99V[a\x17\xEBV[PPPPPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81\x80a\x14)\x87\x87\x87a\x1CgV[\x91P\x91P\x80\x15a\x11\xA5WP`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x97\x90\x98\x16\x82R\x95\x86R\x86\x81 c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x81R\x94RPPP T`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90V[a\x14\x89a\x1E\xECV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06.V[a\x06@\x81a\x1FFV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15n\x91\x90a$8V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$UV[`fT\x19\x81\x19`fT\x19\x16\x14a\x16\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06.V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07wV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x82\x91\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17dW\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x17\xA1Wa\x17\xA1a&)V[` \x02` \x01\x01\x81\x90RP\x80`@Q` \x01a\x17\xBD\x91\x90a&\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[`\0a\x17\xE6Ba\x1F\x98V[\x90P\x90V[\x80`\x03\x0B`\0\x14\x15a\x18gW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FSlasher._modifyRequestedBipsToSl`D\x82\x01R\x7Fash: cannot modify slashing by 0`d\x82\x01R`\x84\x01a\x06.V[`@\x80Q\x80\x82\x01\x90\x91R3\x81R`\x01`\x01`\xE0\x1B\x03\x19\x85\x16` \x82\x01R`\0a\x18\x8F\x82a\x17JV[\x90P`\0[\x85Q\x81\x10\x15a\x1C\x1EW`\0\x86\x82\x81Q\x81\x10a\x18\xB1Wa\x18\xB1a&)V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16`\0\x90\x81R`\x9B\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x84R\x81\x81 c\xFF\xFF\xFF\xFF\x80\x8C\x16\x83R\x90\x85R\x82\x82 \x88\x83R\x90\x94R\x90\x81 T\x91\x93P\x91\x16\x90a\x19\x0B\x87\x83a',V[\x90P`\0\x81`\x03\x0B\x12\x15a\x19)Wa\x19\"\x82a&\x99V[\x96P`\0\x90P[`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x80\x83 \x94\x88\x16\x80\x84R\x94\x82R\x80\x83 c\xFF\xFF\xFF\xFF\x8E\x81\x16\x80\x86R\x91\x84R\x82\x85 \x8C\x86R\x84R\x82\x85 \x80Tc\xFF\xFF\xFF\xFF\x19\x16\x89\x83\x16\x17\x90U\x94\x84R`\x98\x83R\x81\x84 \x95\x84R\x94\x82R\x80\x83 \x94\x83R\x93\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x80\x82R`\x01` \x1B\x84\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01``\x1B\x90\x93\x04\x90\x91\x16\x92\x81\x01\x92\x90\x92Ra\x1ANW`\x01`\x01`\xA0\x1B\x03\x80\x8D\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 Ta\x1A\x0E\x90c\xFF\xFF\xFF\xFF\x16`\x01a&?V[`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16c\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x82\x17\x90U\x82RP[`@Qc?v\xC6\xC7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?v\xC6\xC7\x90a\x1A\xA0\x90\x8F\x90\x8B\x90\x89\x90\x8F\x90`\x04\x01a'uV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xE1\x91\x90a%gV[a\xFF\xFF\x16\x88`\x03\x0Ba\x1A\xF3\x91\x90a'\xB2V[\x81` \x01Qa\x1B\x02\x91\x90a(IV[\x81` \x01\x90`\x01`\x01`@\x1B\x03\x16\x90\x81`\x01`\x01`@\x1B\x03\x16\x81RPP\x80`\x98`\0\x8E`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\0\x01`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90PPPPPP\x80a\x1C\x17\x90a&gV[\x90Pa\x18\x94V[P\x7FQ\xB1]\xC6\np}\x9CCf\x0F\xDDj\xF7\xCF\x86\x06\x0E'xc\x8D\x04\xEFF/\xAAV$\x1E\xA6\xBF\x84\x88\x84\x88\x87`@Qa\x1CV\x95\x94\x93\x92\x91\x90a(\x91V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T\x81\x90\x81\x90\x81\x90[\x80\x15a\x1D%W`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a\x1C\xCC`\x01\x83a&\x82V[\x81T\x81\x10a\x1C\xDCWa\x1C\xDCa&)V[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93P\x86\x16\x83\x11a\x1D\x15W`\x01\x91Pa\x1D%V[a\x1D\x1E\x81a)\x10V[\x90Pa\x1C\x96V[P\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 \x95\x90\x96\x16\x80\x83R\x94\x81R\x85\x82 T\x92\x82R`\x98\x81R\x85\x82 \x94\x82R\x93\x84R\x84\x81 c\xFF\xFF\xFF\xFF\x93\x84\x16\x82R\x90\x93R\x92\x90\x91 T`\x01` \x1B\x90\x92\x04\x81\x16\x91\x16\x11\x15\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x16a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rrcannot slash for 0%`h\x1B`D\x82\x01R`d\x01a\x06.V[c\x05\xF5\xE1\0`\x01`\x01`@\x1B\x03\x83\x16\x11\x15a\x1EIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fcannot slash more than 100% at o`D\x82\x01Rbnce`\xE8\x1B`d\x82\x01R`\x84\x01a\x06.V[`\0`\x01`\x01`@\x1B\x03\x83\x16c\x05\xF5\xE1\0\x14\x80a\x1E\xA6WP`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x85\x16a\x1E\x8Dg\r\xE0\xB6\xB3\xA7d\0\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa)'V[a\x1E\x99\x90`\0\x19a)\\V[a\x1E\xA3\x91\x90a)\\V[\x10\x15[\x15a\x1E\xB9WP`\x01`\x01`@\x1B\x03a\n\x1AV[a\x1E\xC7\x83c\x05\xF5\xE1\0a)pV[a\x1E\xD5c\x05\xF5\xE1\0\x86a)\x98V[a\x0B.\x91\x90a)\xBEV[`\0a\n\x1D\x82`\x02a&?V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06.V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0c_\xC60@\x82\x10\x15a \x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEpochUtils.getEpochFromTimestamp`D\x82\x01R\x7F: timestamp is before genesis\0\0\0`d\x82\x01R`\x84\x01a\x06.V[b\t:\x80a &c_\xC60@\x84a&\x82V[a\n\x1D\x91\x90a)\\V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06@W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a WW`\0\x80\xFD[\x815a\n\x1A\x81a 0V[`\0` \x82\x84\x03\x12\x15a tW`\0\x80\xFD[P5\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a \x8FW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80\x84\x86\x03`\xA0\x81\x12\x15a \xABW`\0\x80\xFD[\x855a \xB6\x81a 0V[\x94P` \x86\x015a \xC6\x81a 0V[\x93P`@`?\x19\x82\x01\x12\x15a \xDAW`\0\x80\xFD[P`@\x85\x01\x91Pa \xED`\x80\x86\x01a {V[\x90P\x92\x95\x91\x94P\x92PV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a \x8FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a!7W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15a!SWa!Sa!\x10V[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15a!xWa!xa!\x10V[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15a!\x96W`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15a!\xBEW\x815a!\xAF\x81a 0V[\x83R\x91\x83\x01\x91\x90\x83\x01\x90a!\x9CV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a!\xDFW`\0\x80\xFD[\x845a!\xEA\x81a 0V[\x93Pa!\xF8` \x86\x01a \xF8V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x13W`\0\x80\xFD[a\"\x1F\x87\x82\x88\x01a!&V[\x92PPa \xED``\x86\x01a {V[`\0\x80`@\x83\x85\x03\x12\x15a\"AW`\0\x80\xFD[\x825a\"L\x81a 0V[\x91P` \x83\x015a\"\\\x81a 0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"|W`\0\x80\xFD[\x835a\"\x87\x81a 0V[\x92P` \x84\x015a\"\x97\x81a 0V[\x91Pa\"\xA5`@\x85\x01a {V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\xC3W`\0\x80\xFD[\x835a\"\xCE\x81a 0V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xE9W`\0\x80\xFD[a\"\xF5\x86\x82\x87\x01a!&V[\x92PPa\"\xA5`@\x85\x01a {V[`\0` \x82\x84\x03\x12\x15a#\x16W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\n\x1AW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a#<W`\0\x80\xFD[\x835a#G\x81a 0V[\x92P` \x84\x015a#W\x81a 0V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a#\x80W`\0\x80\xFD[\x855a#\x8B\x81a 0V[\x94Pa#\x99` \x87\x01a \xF8V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xB4W`\0\x80\xFD[a#\xC0\x88\x82\x89\x01a!&V[\x93PPa#\xCF``\x87\x01a {V[\x91Pa#\xDD`\x80\x87\x01a {V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a#\xFFW`\0\x80\xFD[\x845a$\n\x81a 0V[\x93P` \x85\x015a$\x1A\x81a 0V[\x92Pa$(`@\x86\x01a {V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0` \x82\x84\x03\x12\x15a$JW`\0\x80\xFD[\x81Qa\n\x1A\x81a 0V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a$\xB1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\x1AW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\xA0\x82\x01\x90\x855a%&\x81a 0V[\x81\x81\x16` \x85\x01RPc\xFF\xFF\xFF\xFF`\xE0\x1Ba%C` \x88\x01a \xF8V[\x16`@\x84\x01R\x80\x85\x16``\x84\x01RPc\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a%yW`\0\x80\xFD[\x81Qa\xFF\xFF\x81\x16\x81\x14a\n\x1AW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a%\x9DW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a%\xBFWa%\xBFa!\x10V[`@R\x825a%\xCD\x81a 0V[\x81Ra%\xDB` \x84\x01a \xF8V[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a& Wa& a%\xE7V[\x02\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a&^Wa&^a%\xE7V[\x01\x94\x93PPPPV[`\0`\0\x19\x82\x14\x15a&{Wa&{a%\xE7V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a&\x94Wa&\x94a%\xE7V[P\x03\x90V[`\0\x81`\x03\x0Bc\x7F\xFF\xFF\xFF\x19\x81\x14\x15a&\xB4Wa&\xB4a%\xE7V[`\0\x03\x92\x91PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a'\x1FWa'\x0F\x84\x83Qa&\xBDV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a&\xFCV[P\x91\x97\x96PPPPPPPV[`\0\x81`\x03\x0B\x83`\x03\x0B`\0\x82\x12\x82c\x7F\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a'SWa'Sa%\xE7V[\x82c\x7F\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a'lWa'la%\xE7V[P\x01\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\xA0\x82\x01\x90a'\x94` \x84\x01\x87a&\xBDV[\x80\x85\x16``\x84\x01RPc\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[`\0\x81`\x07\x0B\x83`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a'\xE3Wa'\xE3a%\xE7V[g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a(\x07Wa(\x07a%\xE7V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a(#Wa(#a%\xE7V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a(9Wa(9a%\xE7V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x81`\x07\x0B\x83`\x07\x0B`\0\x82\x12\x82g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a(tWa(ta%\xE7V[\x82g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a'lWa'la%\xE7V[`\0`\xC0\x82\x01c\xFF\xFF\xFF\xFF\x88\x16\x83R` `\x01\x80`\xA0\x1B\x03\x80\x89\x16\x82\x86\x01Ra(\xBD`@\x86\x01\x89a&\xBDV[`\xC0`\x80\x86\x01R\x86Q\x92\x83\x90R\x81\x87\x01\x92`\xE0\x86\x01\x90`\0[\x81\x81\x10\x15a(\xF4W\x85Q\x84\x16\x83R\x94\x84\x01\x94\x91\x84\x01\x91`\x01\x01a(\xD6V[PP\x80\x94PPPPP\x82`\x03\x0B`\xA0\x83\x01R\x96\x95PPPPPPV[`\0\x81a)\x1FWa)\x1Fa%\xE7V[P`\0\x19\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a)AWa)Aa%\xE7V[P\x02\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)kWa)ka)FV[P\x04\x90V[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a)\x90Wa)\x90a%\xE7V[\x03\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a& Wa& a%\xE7V[`\0`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a)\xD8Wa)\xD8a)FV[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xCE\xFC\x06\x84\x90\xF4\xFD5\x86\x9EYat\xBF\xAB\x1F\x01\x1F\xB4\xA6fa\x9D&\xF0j\x01\xD7\xF6\xD2m\xDFdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101cf5760003560e01c80635c975abb1161010457806390e7cde1116100a2578063e49a1e8411610071578063e49a1e841461050e578063ec65b53d14610521578063f2fde38b14610561578063fabc1cbc1461057457600080fd5b806390e7cde11461049a5780639d086ecb146104ad578063c78d4bcd146104c0578063df5cf723146104e757600080fd5b806379c415ec116100de57806379c415ec1461040a5780637ef639a61461041d578063886f1195146104765780638da5cb5b1461048957600080fd5b80635c975abb146103de5780636c0d75d0146103ef578063715018a61461040257600080fd5b80633dd9e7c5116101715780634dcaafb81161014b5780634dcaafb81461037d578063595c6a67146103905780635ab112d6146103985780635ac86ab7146103ab57600080fd5b80633dd9e7c5146102d85780633f2201bb146102eb5780634d54dc3c1461036a57600080fd5b8063287a96da116101ad578063287a96da14610229578063334f00d61461023c57806339b70e38146102675780633be2073b146102a657600080fd5b806310d67a2f146101d4578063136439dd146101e95780632421a64c146101fc575b600080fd5b6101e76101e2366004612045565b610587565b005b6101e76101f7366004612062565b610643565b61020f61020a366004612094565b610782565b60405163ffffffff90911681526020015b60405180910390f35b6101e76102373660046121c9565b610897565b61024f61024a36600461222e565b6109d3565b6040516001600160401b039091168152602001610220565b61028e7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610220565b6102b96102b4366004612267565b610a23565b6040805192151583526001600160401b03909116602083015201610220565b61024f6102e636600461222e565b610aab565b61033e6102f9366004612267565b609860209081526000938452604080852082529284528284209052825290205463ffffffff8116906001600160401b03600160201b8204811691600160601b90041683565b6040805163ffffffff90941684526001600160401b039283166020850152911690820152606001610220565b61020f610378366004612094565b610b36565b6101e761038b3660046122ae565b610b67565b6101e7610fa6565b61020f6103a636600461222e565b61106d565b6103ce6103b9366004612304565b606654600160ff9092169190911b9081161490565b6040519015158152602001610220565b606654604051908152602001610220565b61020f6103fd366004612327565b611112565b6101e7611168565b6103ce610418366004612267565b61117c565b61045961042b36600461222e565b609760209081526000928352604080842090915290825290205463ffffffff80821691600160201b90041682565b6040805163ffffffff938416815292909116602083015201610220565b60655461028e906001600160a01b031681565b6033546001600160a01b031661028e565b61020f6104a8366004612267565b6111b0565b6101e76104bb366004612368565b61120f565b61028e7f000000000000000000000000000000000000000000000000000000000000000081565b61028e7f000000000000000000000000000000000000000000000000000000000000000081565b61024f61051c366004612267565b611411565b61020f61052f3660046123e9565b609b60209081526000948552604080862082529385528385208152918452828420909152825290205463ffffffff1681565b6101e761056f366004612045565b611481565b6101e7610582366004612062565b6114f7565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156105da573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105fe9190612438565b6001600160a01b0316336001600160a01b0316146106375760405162461bcd60e51b815260040161062e90612455565b60405180910390fd5b61064081611653565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561068b573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106af919061249f565b6106cb5760405162461bcd60e51b815260040161062e906124c1565b606654818116146107445760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c6974790000000000000000606482015260840161062e565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b604051633f76c6c760e01b81526000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633f76c6c7906107d7908890879089908890600401612509565b602060405180830381865afa1580156107f4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108189190612567565b6001600160a01b038087166000908152609b60209081526040808320938916835292815282822063ffffffff87168352905290812061ffff92909216919061086d6108683688900388018861258b565b61174a565b815260208101919091526040016000205461088e919063ffffffff166125fd565b95945050505050565b60008163ffffffff16116109215760405162461bcd60e51b815260206004820152604560248201527f536c61736865722e696e63726561736552657175657374656442697073546f5360448201527f6c6173683a2062697073546f496e637265617365206d75737420626520706f73606482015264697469766560d81b608482015260a40161062e565b6127108163ffffffff16106109b95760405162461bcd60e51b815260206004820152605260248201527f536c61736865722e696e63726561736552657175657374656442697073546f5360448201527f6c6173683a2062697073546f496e637265617365206d757374206265206c657360648201527139903a3430b7102124a829afa320a1aa27a960711b608482015260a40161062e565b6109cd8484846109c76117db565b856117eb565b50505050565b6001600160a01b0380831660009081526099602090815260408083209385168352929052908120546001600160401b031680610a1a57670de0b6b3a7640000915050610a1d565b90505b92915050565b6000806001670de0b6b3a76400008280610a3e898989611c67565b915091508015610a9d57610a53898984611d32565b6001600160a01b038a81166000908152609860209081526040808320938d16835292815282822063ffffffff8716835290522054909450600160601b90046001600160401b031692505b509197909650945050505050565b6001600160a01b03808316600081815260996020908152604080832094861680845294825280832054938352609882528083209483529390529182208291610b2e916001600160401b039091169083610b026117db565b63ffffffff168152602081019190915260400160002054600160201b90046001600160401b0316611d92565b949350505050565b600080610b4586868686610782565b90506305f5e10063ffffffff82161061088e57506305f5e10095945050505050565b610b7081611edf565b63ffffffff16610b7e6117db565b63ffffffff1611610c1d5760405162461bcd60e51b815260206004820152605760248201527f536c61736865722e65786563757465536c617368696e673a2063757272656e7460448201527f2065706f6368206d7573742062652067726561746572207468616e207468652060648201527f6d696e696d756d20657865637574696f6e2065706f6368000000000000000000608482015260a40161062e565b60005b82518110156109cd576000838281518110610c3d57610c3d612629565b6020908102919091018101516001600160a01b03808816600081815260988552604080822093851680835293865280822063ffffffff808b168452908752818320825160608101845290548083168083526001600160401b03600160201b8084048216858d0152600160601b909304168386015295855260978952838520968552959097529120549395509092610cd892900416600161263f565b63ffffffff1614610d515760405162461bcd60e51b815260206004820152603860248201527f536c61736865722e65786563757465536c617368696e673a206d75737420657860448201527f656375746520736c617368696e677320696e206f726465720000000000000000606482015260840161062e565b80516001600160a01b0380881660009081526097602090815260408083209387168352928152919020805463ffffffff909316600160201b0267ffffffff0000000019909316929092179091558101516305f5e1006001600160401b03919091161115610dc7576305f5e1006020820152610de0565b60208101516001600160401b0316610de0575050610f96565b6000610dec87846109d3565b90506000610dfe828460200151611d92565b9050609a6000896001600160a01b03166001600160a01b031681526020019081526020016000206000856001600160a01b03166001600160a01b031681526020019081526020016000208690806001815401808255809150506001900390600052602060002090600891828204019190066004029091909190916101000a81548163ffffffff021916908363ffffffff16021790555080609960008a6001600160a01b03166001600160a01b031681526020019081526020016000206000866001600160a01b03166001600160a01b0316815260200190815260200160002060006101000a8154816001600160401b0302191690836001600160401b031602179055508083604001906001600160401b031690816001600160401b0316815250507f2f679597a08f229c142b2f79a954c91a30bbda82795ef8dee2775b84db9699248689868660200151604051610f89949392919063ffffffff9490941684526001600160a01b039283166020850152911660408301526001600160401b0316606082015260800190565b60405180910390a1505050505b610f9f81612667565b9050610c20565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610fee573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611012919061249f565b61102e5760405162461bcd60e51b815260040161062e906124c1565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6001600160a01b038083166000908152609a60209081526040808320938516835292905290812054806110a4576000915050610a1d565b6001600160a01b038085166000908152609a602090815260408083209387168352929052206110d4600183612682565b815481106110e4576110e4612629565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16915050610a1d565b609a602052826000526040600020602052816000526040600020818154811061113a57600080fd5b906000526020600020906008918282040191900660040292509250509054906101000a900463ffffffff1681565b611170611eec565b61117a6000611f46565b565b60006001818061118d878787611c67565b9150915080156111a5576111a2878784611d32565b92505b509095945050505050565b6001600160a01b038381166000908152609860209081526040808320938616835292815282822063ffffffff85168352905290812054600160201b90046001600160401b03166305f5e1008110610b2e57506305f5e100949350505050565b60006112196117db565b90508063ffffffff168363ffffffff16148061124a575063ffffffff811661124284600161263f565b63ffffffff16145b6112d65760405162461bcd60e51b815260206004820152605160248201527f536c61736865722e72656475636552657175657374656442697073546f536c6160448201527f73683a2063616e206f6e6c792072656475636520666f722063757272656e74206064820152700dee440e0e4caecd2deeae640cae0dec6d607b1b608482015260a40161062e565b60008263ffffffff161161135c5760405162461bcd60e51b815260206004820152604160248201527f536c61736865722e72656475636552657175657374656442697073546f536c6160448201527f73683a2062697073546f526564756365206d757374206265206e6567617469766064820152606560f81b608482015260a40161062e565b63800000008263ffffffff16106113f45760405162461bcd60e51b815260206004820152605060248201527f536c61736865722e72656475636552657175657374656442697073546f536c6160448201527f73683a2062697073546f526564756365206d757374206265206c65737320746860648201526f30b71036b4b734b6bab69034b73a199960811b608482015260a40161062e565b6114098686868661140487612699565b6117eb565b505050505050565b6000670de0b6b3a76400008180611429878787611c67565b9150915080156111a557506001600160a01b03958616600090815260986020908152604080832097909816825295865286812063ffffffff92909216815294525050502054600160601b90046001600160401b031690565b611489611eec565b6001600160a01b0381166114ee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840161062e565b61064081611f46565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561154a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061156e9190612438565b6001600160a01b0316336001600160a01b03161461159e5760405162461bcd60e51b815260040161062e90612455565b60665419811960665419161461161c5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c6974790000000000000000606482015260840161062e565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610777565b6001600160a01b0381166116e15760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a40161062e565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b604080516001808252818301909252600091829190816020015b604080518082019091526000808252602082015281526020019060019003908161176457905050905082816000815181106117a1576117a1612629565b6020026020010181905250806040516020016117bd91906126df565b60405160208183030381529060405280519060200120915050919050565b60006117e642611f98565b905090565b8060030b60001415611867576040805162461bcd60e51b81526020600482015260248101919091527f536c61736865722e5f6d6f6469667952657175657374656442697073546f536c60448201527f6173683a2063616e6e6f74206d6f6469667920736c617368696e672062792030606482015260840161062e565b604080518082019091523381526001600160e01b031985166020820152600061188f8261174a565b905060005b8551811015611c1e5760008682815181106118b1576118b1612629565b6020908102919091018101516001600160a01b03808c166000908152609b84526040808220928416825291845281812063ffffffff808c1683529085528282208883529094529081205491935091169061190b878361272c565b905060008160030b12156119295761192282612699565b9650600090505b6001600160a01b038b81166000818152609b6020908152604080832094881680845294825280832063ffffffff8e81168086529184528285208c86528452828520805463ffffffff1916898316179055948452609883528184209584529482528083209483529381529083902083516060810185529054928316808252600160201b84046001600160401b0390811693830193909352600160601b90930490911692810192909252611a4e576001600160a01b03808d166000908152609760209081526040808320938816835292905290812054611a0e9063ffffffff16600161263f565b6001600160a01b03808f166000908152609760209081526040808320938a16835292905220805463ffffffff90921663ffffffff19909216821790558252505b604051633f76c6c760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690633f76c6c790611aa0908f908b9089908f90600401612775565b602060405180830381865afa158015611abd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ae19190612567565b61ffff168860030b611af391906127b2565b8160200151611b029190612849565b81602001906001600160401b031690816001600160401b03168152505080609860008e6001600160a01b03166001600160a01b031681526020019081526020016000206000866001600160a01b03166001600160a01b0316815260200190815260200160002060008b63ffffffff1663ffffffff16815260200190815260200160002060008201518160000160006101000a81548163ffffffff021916908363ffffffff16021790555060208201518160000160046101000a8154816001600160401b0302191690836001600160401b03160217905550604082015181600001600c6101000a8154816001600160401b0302191690836001600160401b031602179055509050505050505080611c1790612667565b9050611894565b507f51b15dc60a707d9c43660fdd6af7cf86060e2778638d04ef462faa56241ea6bf8488848887604051611c56959493929190612891565b60405180910390a150505050505050565b6001600160a01b038084166000908152609a602090815260408083209386168352929052908120548190819081905b8015611d25576001600160a01b038089166000908152609a60209081526040808320938b16835292905220611ccc600183612682565b81548110611cdc57611cdc612629565b6000918252602090912060088204015460079091166004026101000a900463ffffffff908116935086168311611d155760019150611d25565b611d1e81612910565b9050611c96565b5090969095509350505050565b6001600160a01b03928316600081815260976020908152604080832095909616808352948152858220549282526098815285822094825293845284812063ffffffff93841682529093529290912054600160201b90920481169116111590565b60006001600160401b038216611de05760405162461bcd60e51b815260206004820152601360248201527263616e6e6f7420736c61736820666f7220302560681b604482015260640161062e565b6305f5e1006001600160401b0383161115611e495760405162461bcd60e51b815260206004820152602360248201527f63616e6e6f7420736c617368206d6f7265207468616e2031303025206174206f6044820152626e636560e81b606482015260840161062e565b60006001600160401b0383166305f5e1001480611ea657506001600160401b03808416908516611e8d670de0b6b3a76400006bffffffffffffffffffffffff612927565b611e999060001961295c565b611ea3919061295c565b10155b15611eb957506001600160401b03610a1a565b611ec7836305f5e100612970565b611ed56305f5e10086612998565b610b2e91906129be565b6000610a1d82600261263f565b6033546001600160a01b0316331461117a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161062e565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6000635fc630408210156120145760405162461bcd60e51b815260206004820152603d60248201527f45706f63685574696c732e67657445706f636846726f6d54696d657374616d7060448201527f3a2074696d657374616d70206973206265666f72652067656e65736973000000606482015260840161062e565b62093a80612026635fc6304084612682565b610a1d919061295c565b6001600160a01b038116811461064057600080fd5b60006020828403121561205757600080fd5b8135610a1a81612030565b60006020828403121561207457600080fd5b5035919050565b803563ffffffff8116811461208f57600080fd5b919050565b60008060008084860360a08112156120ab57600080fd5b85356120b681612030565b945060208601356120c681612030565b93506040603f19820112156120da57600080fd5b506040850191506120ed6080860161207b565b905092959194509250565b80356001600160e01b03198116811461208f57600080fd5b634e487b7160e01b600052604160045260246000fd5b600082601f83011261213757600080fd5b813560206001600160401b038083111561215357612153612110565b8260051b604051601f19603f8301168101818110848211171561217857612178612110565b60405293845285810183019383810192508785111561219657600080fd5b83870191505b848210156121be5781356121af81612030565b8352918301919083019061219c565b979650505050505050565b600080600080608085870312156121df57600080fd5b84356121ea81612030565b93506121f8602086016120f8565b925060408501356001600160401b0381111561221357600080fd5b61221f87828801612126565b9250506120ed6060860161207b565b6000806040838503121561224157600080fd5b823561224c81612030565b9150602083013561225c81612030565b809150509250929050565b60008060006060848603121561227c57600080fd5b833561228781612030565b9250602084013561229781612030565b91506122a56040850161207b565b90509250925092565b6000806000606084860312156122c357600080fd5b83356122ce81612030565b925060208401356001600160401b038111156122e957600080fd5b6122f586828701612126565b9250506122a56040850161207b565b60006020828403121561231657600080fd5b813560ff81168114610a1a57600080fd5b60008060006060848603121561233c57600080fd5b833561234781612030565b9250602084013561235781612030565b929592945050506040919091013590565b600080600080600060a0868803121561238057600080fd5b853561238b81612030565b9450612399602087016120f8565b935060408601356001600160401b038111156123b457600080fd5b6123c088828901612126565b9350506123cf6060870161207b565b91506123dd6080870161207b565b90509295509295909350565b600080600080608085870312156123ff57600080fd5b843561240a81612030565b9350602085013561241a81612030565b92506124286040860161207b565b9396929550929360600135925050565b60006020828403121561244a57600080fd5b8151610a1a81612030565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b6000602082840312156124b157600080fd5b81518015158114610a1a57600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b6001600160a01b03858116825260a0820190853561252681612030565b81811660208501525063ffffffff60e01b612543602088016120f8565b16604084015280851660608401525063ffffffff8316608083015295945050505050565b60006020828403121561257957600080fd5b815161ffff81168114610a1a57600080fd5b60006040828403121561259d57600080fd5b604051604081018181106001600160401b03821117156125bf576125bf612110565b60405282356125cd81612030565b81526125db602084016120f8565b60208201529392505050565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff80831681851681830481118215151615612620576126206125e7565b02949350505050565b634e487b7160e01b600052603260045260246000fd5b600063ffffffff80831681851680830382111561265e5761265e6125e7565b01949350505050565b600060001982141561267b5761267b6125e7565b5060010190565b600082821015612694576126946125e7565b500390565b60008160030b637fffffff198114156126b4576126b46125e7565b60000392915050565b80516001600160a01b031682526020908101516001600160e01b031916910152565b602080825282518282018190526000919060409081850190868401855b8281101561271f5761270f8483516126bd565b92840192908501906001016126fc565b5091979650505050505050565b60008160030b8360030b6000821282637fffffff03821381151615612753576127536125e7565b82637fffffff1903821281161561276c5761276c6125e7565b50019392505050565b6001600160a01b03858116825260a082019061279460208401876126bd565b80851660608401525063ffffffff8316608083015295945050505050565b60008160070b8360070b677fffffffffffffff6000821360008413838304851182821616156127e3576127e36125e7565b677fffffffffffffff196000851282811687830587121615612807576128076125e7565b60008712925085820587128484161615612823576128236125e7565b85850587128184161615612839576128396125e7565b5050509290910295945050505050565b60008160070b8360070b6000821282677fffffffffffffff03821381151615612874576128746125e7565b82677fffffffffffffff1903821281161561276c5761276c6125e7565b600060c0820163ffffffff88168352602060018060a01b03808916828601526128bd60408601896126bd565b60c060808601528651928390528187019260e086019060005b818110156128f45785518416835294840194918401916001016128d6565b5050809450505050508260030b60a08301529695505050505050565b60008161291f5761291f6125e7565b506000190190565b6000816000190483118215151615612941576129416125e7565b500290565b634e487b7160e01b600052601260045260246000fd5b60008261296b5761296b612946565b500490565b60006001600160401b0383811690831681811015612990576129906125e7565b039392505050565b60006001600160401b0380831681851681830481118215151615612620576126206125e7565b60006001600160401b03808416806129d8576129d8612946565b9216919091049291505056fea2646970667358221220cefc068490f4fd35869e596174bfab1f011fb4a666619d26f06a01d7f6d26ddf64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xCFW`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\x01\x04W\x80c\x90\xE7\xCD\xE1\x11a\0\xA2W\x80c\xE4\x9A\x1E\x84\x11a\0qW\x80c\xE4\x9A\x1E\x84\x14a\x05\x0EW\x80c\xECe\xB5=\x14a\x05!W\x80c\xF2\xFD\xE3\x8B\x14a\x05aW\x80c\xFA\xBC\x1C\xBC\x14a\x05tW`\0\x80\xFD[\x80c\x90\xE7\xCD\xE1\x14a\x04\x9AW\x80c\x9D\x08n\xCB\x14a\x04\xADW\x80c\xC7\x8DK\xCD\x14a\x04\xC0W\x80c\xDF\\\xF7#\x14a\x04\xE7W`\0\x80\xFD[\x80cy\xC4\x15\xEC\x11a\0\xDEW\x80cy\xC4\x15\xEC\x14a\x04\nW\x80c~\xF69\xA6\x14a\x04\x1DW\x80c\x88o\x11\x95\x14a\x04vW\x80c\x8D\xA5\xCB[\x14a\x04\x89W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xDEW\x80cl\ru\xD0\x14a\x03\xEFW\x80cqP\x18\xA6\x14a\x04\x02W`\0\x80\xFD[\x80c=\xD9\xE7\xC5\x11a\x01qW\x80cM\xCA\xAF\xB8\x11a\x01KW\x80cM\xCA\xAF\xB8\x14a\x03}W\x80cY\\jg\x14a\x03\x90W\x80cZ\xB1\x12\xD6\x14a\x03\x98W\x80cZ\xC8j\xB7\x14a\x03\xABW`\0\x80\xFD[\x80c=\xD9\xE7\xC5\x14a\x02\xD8W\x80c?\"\x01\xBB\x14a\x02\xEBW\x80cMT\xDC<\x14a\x03jW`\0\x80\xFD[\x80c(z\x96\xDA\x11a\x01\xADW\x80c(z\x96\xDA\x14a\x02)W\x80c3O\0\xD6\x14a\x02<W\x80c9\xB7\x0E8\x14a\x02gW\x80c;\xE2\x07;\x14a\x02\xA6W`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x01\xD4W\x80c\x13d9\xDD\x14a\x01\xE9W\x80c$!\xA6L\x14a\x01\xFCW[`\0\x80\xFD[a\x01\xE7a\x01\xE26`\x04a EV[a\x05\x87V[\0[a\x01\xE7a\x01\xF76`\x04a bV[a\x06CV[a\x02\x0Fa\x02\n6`\x04a \x94V[a\x07\x82V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xE7a\x0276`\x04a!\xC9V[a\x08\x97V[a\x02Oa\x02J6`\x04a\".V[a\t\xD3V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02 V[a\x02\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02 V[a\x02\xB9a\x02\xB46`\x04a\"gV[a\n#V[`@\x80Q\x92\x15\x15\x83R`\x01`\x01`@\x1B\x03\x90\x91\x16` \x83\x01R\x01a\x02 V[a\x02Oa\x02\xE66`\x04a\".V[a\n\xABV[a\x03>a\x02\xF96`\x04a\"gV[`\x98` \x90\x81R`\0\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 Tc\xFF\xFF\xFF\xFF\x81\x16\x90`\x01`\x01`@\x1B\x03`\x01` \x1B\x82\x04\x81\x16\x91`\x01``\x1B\x90\x04\x16\x83V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x94\x16\x84R`\x01`\x01`@\x1B\x03\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x02 V[a\x02\x0Fa\x03x6`\x04a \x94V[a\x0B6V[a\x01\xE7a\x03\x8B6`\x04a\"\xAEV[a\x0BgV[a\x01\xE7a\x0F\xA6V[a\x02\x0Fa\x03\xA66`\x04a\".V[a\x10mV[a\x03\xCEa\x03\xB96`\x04a#\x04V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02 V[`fT`@Q\x90\x81R` \x01a\x02 V[a\x02\x0Fa\x03\xFD6`\x04a#'V[a\x11\x12V[a\x01\xE7a\x11hV[a\x03\xCEa\x04\x186`\x04a\"gV[a\x11|V[a\x04Ya\x04+6`\x04a\".V[`\x97` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x91`\x01` \x1B\x90\x04\x16\x82V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x02 V[`eTa\x02\x8E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x8EV[a\x02\x0Fa\x04\xA86`\x04a\"gV[a\x11\xB0V[a\x01\xE7a\x04\xBB6`\x04a#hV[a\x12\x0FV[a\x02\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02Oa\x05\x1C6`\x04a\"gV[a\x14\x11V[a\x02\x0Fa\x05/6`\x04a#\xE9V[`\x9B` \x90\x81R`\0\x94\x85R`@\x80\x86 \x82R\x93\x85R\x83\x85 \x81R\x91\x84R\x82\x84 \x90\x91R\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xE7a\x05o6`\x04a EV[a\x14\x81V[a\x01\xE7a\x05\x826`\x04a bV[a\x14\xF7V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFE\x91\x90a$8V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x067W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$UV[`@Q\x80\x91\x03\x90\xFD[a\x06@\x81a\x16SV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xAF\x91\x90a$\x9FV[a\x06\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$\xC1V[`fT\x81\x81\x16\x14a\x07DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06.V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`@Qc?v\xC6\xC7`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?v\xC6\xC7\x90a\x07\xD7\x90\x88\x90\x87\x90\x89\x90\x88\x90`\x04\x01a%\tV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x18\x91\x90a%gV[`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x81R\x82\x82 c\xFF\xFF\xFF\xFF\x87\x16\x83R\x90R\x90\x81 a\xFF\xFF\x92\x90\x92\x16\x91\x90a\x08ma\x08h6\x88\x90\x03\x88\x01\x88a%\x8BV[a\x17JV[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta\x08\x8E\x91\x90c\xFF\xFF\xFF\xFF\x16a%\xFDV[\x95\x94PPPPPV[`\0\x81c\xFF\xFF\xFF\xFF\x16\x11a\t!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FSlasher.increaseRequestedBipsToS`D\x82\x01R\x7Flash: bipsToIncrease must be pos`d\x82\x01Rditive`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[a'\x10\x81c\xFF\xFF\xFF\xFF\x16\x10a\t\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FSlasher.increaseRequestedBipsToS`D\x82\x01R\x7Flash: bipsToIncrease must be les`d\x82\x01Rq9\x90:40\xB7\x10!$\xA8)\xAF\xA3 \xA1\xAA'\xA9`q\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[a\t\xCD\x84\x84\x84a\t\xC7a\x17\xDBV[\x85a\x17\xEBV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 T`\x01`\x01`@\x1B\x03\x16\x80a\n\x1AWg\r\xE0\xB6\xB3\xA7d\0\0\x91PPa\n\x1DV[\x90P[\x92\x91PPV[`\0\x80`\x01g\r\xE0\xB6\xB3\xA7d\0\0\x82\x80a\n>\x89\x89\x89a\x1CgV[\x91P\x91P\x80\x15a\n\x9DWa\nS\x89\x89\x84a\x1D2V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x8D\x16\x83R\x92\x81R\x82\x82 c\xFF\xFF\xFF\xFF\x87\x16\x83R\x90R T\x90\x94P`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x92P[P\x91\x97\x90\x96P\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x99` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\x98\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 \x82\x91a\x0B.\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90\x83a\x0B\x02a\x17\xDBV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1D\x92V[\x94\x93PPPPV[`\0\x80a\x0BE\x86\x86\x86\x86a\x07\x82V[\x90Pc\x05\xF5\xE1\0c\xFF\xFF\xFF\xFF\x82\x16\x10a\x08\x8EWPc\x05\xF5\xE1\0\x95\x94PPPPPV[a\x0Bp\x81a\x1E\xDFV[c\xFF\xFF\xFF\xFF\x16a\x0B~a\x17\xDBV[c\xFF\xFF\xFF\xFF\x16\x11a\x0C\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FSlasher.executeSlashing: current`D\x82\x01R\x7F epoch must be greater than the `d\x82\x01R\x7Fminimum execution epoch\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06.V[`\0[\x82Q\x81\x10\x15a\t\xCDW`\0\x83\x82\x81Q\x81\x10a\x0C=Wa\x0C=a&)V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x81\x81R`\x98\x85R`@\x80\x82 \x93\x85\x16\x80\x83R\x93\x86R\x80\x82 c\xFF\xFF\xFF\xFF\x80\x8B\x16\x84R\x90\x87R\x81\x83 \x82Q``\x81\x01\x84R\x90T\x80\x83\x16\x80\x83R`\x01`\x01`@\x1B\x03`\x01` \x1B\x80\x84\x04\x82\x16\x85\x8D\x01R`\x01``\x1B\x90\x93\x04\x16\x83\x86\x01R\x95\x85R`\x97\x89R\x83\x85 \x96\x85R\x95\x90\x97R\x91 T\x93\x95P\x90\x92a\x0C\xD8\x92\x90\x04\x16`\x01a&?V[c\xFF\xFF\xFF\xFF\x16\x14a\rQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FSlasher.executeSlashing: must ex`D\x82\x01R\x7Fecute slashings in order\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06.V[\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x81R\x91\x90 \x80Tc\xFF\xFF\xFF\xFF\x90\x93\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x81\x01Qc\x05\xF5\xE1\0`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x11\x15a\r\xC7Wc\x05\xF5\xE1\0` \x82\x01Ra\r\xE0V[` \x81\x01Q`\x01`\x01`@\x1B\x03\x16a\r\xE0WPPa\x0F\x96V[`\0a\r\xEC\x87\x84a\t\xD3V[\x90P`\0a\r\xFE\x82\x84` \x01Qa\x1D\x92V[\x90P`\x9A`\0\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x86\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x99`\0\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x80\x83`@\x01\x90`\x01`\x01`@\x1B\x03\x16\x90\x81`\x01`\x01`@\x1B\x03\x16\x81RPP\x7F/g\x95\x97\xA0\x8F\"\x9C\x14+/y\xA9T\xC9\x1A0\xBB\xDA\x82y^\xF8\xDE\xE2w[\x84\xDB\x96\x99$\x86\x89\x86\x86` \x01Q`@Qa\x0F\x89\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x85\x01R\x91\x16`@\x83\x01R`\x01`\x01`@\x1B\x03\x16``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPP[a\x0F\x9F\x81a&gV[\x90Pa\x0C V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x12\x91\x90a$\x9FV[a\x10.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$\xC1V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 T\x80a\x10\xA4W`\0\x91PPa\n\x1DV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R a\x10\xD4`\x01\x83a&\x82V[\x81T\x81\x10a\x10\xE4Wa\x10\xE4a&)V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91PPa\n\x1DV[`\x9A` R\x82`\0R`@`\0 ` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x11:W`\0\x80\xFD[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x92P\x92PP\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x11pa\x1E\xECV[a\x11z`\0a\x1FFV[V[`\0`\x01\x81\x80a\x11\x8D\x87\x87\x87a\x1CgV[\x91P\x91P\x80\x15a\x11\xA5Wa\x11\xA2\x87\x87\x84a\x1D2V[\x92P[P\x90\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x81R\x82\x82 c\xFF\xFF\xFF\xFF\x85\x16\x83R\x90R\x90\x81 T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16c\x05\xF5\xE1\0\x81\x10a\x0B.WPc\x05\xF5\xE1\0\x94\x93PPPPV[`\0a\x12\x19a\x17\xDBV[\x90P\x80c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x14\x80a\x12JWPc\xFF\xFF\xFF\xFF\x81\x16a\x12B\x84`\x01a&?V[c\xFF\xFF\xFF\xFF\x16\x14[a\x12\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FSlasher.reduceRequestedBipsToSla`D\x82\x01R\x7Fsh: can only reduce for current `d\x82\x01Rp\r\xEED\x0E\x0EL\xAE\xCD-\xEE\xAEd\x0C\xAE\r\xECm`{\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[`\0\x82c\xFF\xFF\xFF\xFF\x16\x11a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FSlasher.reduceRequestedBipsToSla`D\x82\x01R\x7Fsh: bipsToReduce must be negativ`d\x82\x01R`e`\xF8\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[c\x80\0\0\0\x82c\xFF\xFF\xFF\xFF\x16\x10a\x13\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FSlasher.reduceRequestedBipsToSla`D\x82\x01R\x7Fsh: bipsToReduce must be less th`d\x82\x01Ro0\xB7\x106\xB4\xB74\xB6\xBA\xB6\x904\xB7:\x19\x99`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[a\x14\t\x86\x86\x86\x86a\x14\x04\x87a&\x99V[a\x17\xEBV[PPPPPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81\x80a\x14)\x87\x87\x87a\x1CgV[\x91P\x91P\x80\x15a\x11\xA5WP`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x97\x90\x98\x16\x82R\x95\x86R\x86\x81 c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x81R\x94RPPP T`\x01``\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x90V[a\x14\x89a\x1E\xECV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06.V[a\x06@\x81a\x1FFV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15n\x91\x90a$8V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06.\x90a$UV[`fT\x19\x81\x19`fT\x19\x16\x14a\x16\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06.V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07wV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x16\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06.V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x82\x91\x90\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17dW\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x17\xA1Wa\x17\xA1a&)V[` \x02` \x01\x01\x81\x90RP\x80`@Q` \x01a\x17\xBD\x91\x90a&\xDFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x91\x90PV[`\0a\x17\xE6Ba\x1F\x98V[\x90P\x90V[\x80`\x03\x0B`\0\x14\x15a\x18gW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FSlasher._modifyRequestedBipsToSl`D\x82\x01R\x7Fash: cannot modify slashing by 0`d\x82\x01R`\x84\x01a\x06.V[`@\x80Q\x80\x82\x01\x90\x91R3\x81R`\x01`\x01`\xE0\x1B\x03\x19\x85\x16` \x82\x01R`\0a\x18\x8F\x82a\x17JV[\x90P`\0[\x85Q\x81\x10\x15a\x1C\x1EW`\0\x86\x82\x81Q\x81\x10a\x18\xB1Wa\x18\xB1a&)V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x8C\x16`\0\x90\x81R`\x9B\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x84R\x81\x81 c\xFF\xFF\xFF\xFF\x80\x8C\x16\x83R\x90\x85R\x82\x82 \x88\x83R\x90\x94R\x90\x81 T\x91\x93P\x91\x16\x90a\x19\x0B\x87\x83a',V[\x90P`\0\x81`\x03\x0B\x12\x15a\x19)Wa\x19\"\x82a&\x99V[\x96P`\0\x90P[`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x80\x83 \x94\x88\x16\x80\x84R\x94\x82R\x80\x83 c\xFF\xFF\xFF\xFF\x8E\x81\x16\x80\x86R\x91\x84R\x82\x85 \x8C\x86R\x84R\x82\x85 \x80Tc\xFF\xFF\xFF\xFF\x19\x16\x89\x83\x16\x17\x90U\x94\x84R`\x98\x83R\x81\x84 \x95\x84R\x94\x82R\x80\x83 \x94\x83R\x93\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x80\x82R`\x01` \x1B\x84\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x93\x83\x01\x93\x90\x93R`\x01``\x1B\x90\x93\x04\x90\x91\x16\x92\x81\x01\x92\x90\x92Ra\x1ANW`\x01`\x01`\xA0\x1B\x03\x80\x8D\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 Ta\x1A\x0E\x90c\xFF\xFF\xFF\xFF\x16`\x01a&?V[`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16c\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x82\x17\x90U\x82RP[`@Qc?v\xC6\xC7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?v\xC6\xC7\x90a\x1A\xA0\x90\x8F\x90\x8B\x90\x89\x90\x8F\x90`\x04\x01a'uV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xE1\x91\x90a%gV[a\xFF\xFF\x16\x88`\x03\x0Ba\x1A\xF3\x91\x90a'\xB2V[\x81` \x01Qa\x1B\x02\x91\x90a(IV[\x81` \x01\x90`\x01`\x01`@\x1B\x03\x16\x90\x81`\x01`\x01`@\x1B\x03\x16\x81RPP\x80`\x98`\0\x8E`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x86`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8Bc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\0\x01`\x0Ca\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90PPPPPP\x80a\x1C\x17\x90a&gV[\x90Pa\x18\x94V[P\x7FQ\xB1]\xC6\np}\x9CCf\x0F\xDDj\xF7\xCF\x86\x06\x0E'xc\x8D\x04\xEFF/\xAAV$\x1E\xA6\xBF\x84\x88\x84\x88\x87`@Qa\x1CV\x95\x94\x93\x92\x91\x90a(\x91V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T\x81\x90\x81\x90\x81\x90[\x80\x15a\x1D%W`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a\x1C\xCC`\x01\x83a&\x82V[\x81T\x81\x10a\x1C\xDCWa\x1C\xDCa&)V[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x93P\x86\x16\x83\x11a\x1D\x15W`\x01\x91Pa\x1D%V[a\x1D\x1E\x81a)\x10V[\x90Pa\x1C\x96V[P\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 \x95\x90\x96\x16\x80\x83R\x94\x81R\x85\x82 T\x92\x82R`\x98\x81R\x85\x82 \x94\x82R\x93\x84R\x84\x81 c\xFF\xFF\xFF\xFF\x93\x84\x16\x82R\x90\x93R\x92\x90\x91 T`\x01` \x1B\x90\x92\x04\x81\x16\x91\x16\x11\x15\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x16a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rrcannot slash for 0%`h\x1B`D\x82\x01R`d\x01a\x06.V[c\x05\xF5\xE1\0`\x01`\x01`@\x1B\x03\x83\x16\x11\x15a\x1EIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fcannot slash more than 100% at o`D\x82\x01Rbnce`\xE8\x1B`d\x82\x01R`\x84\x01a\x06.V[`\0`\x01`\x01`@\x1B\x03\x83\x16c\x05\xF5\xE1\0\x14\x80a\x1E\xA6WP`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x85\x16a\x1E\x8Dg\r\xE0\xB6\xB3\xA7d\0\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa)'V[a\x1E\x99\x90`\0\x19a)\\V[a\x1E\xA3\x91\x90a)\\V[\x10\x15[\x15a\x1E\xB9WP`\x01`\x01`@\x1B\x03a\n\x1AV[a\x1E\xC7\x83c\x05\xF5\xE1\0a)pV[a\x1E\xD5c\x05\xF5\xE1\0\x86a)\x98V[a\x0B.\x91\x90a)\xBEV[`\0a\n\x1D\x82`\x02a&?V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06.V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0c_\xC60@\x82\x10\x15a \x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEpochUtils.getEpochFromTimestamp`D\x82\x01R\x7F: timestamp is before genesis\0\0\0`d\x82\x01R`\x84\x01a\x06.V[b\t:\x80a &c_\xC60@\x84a&\x82V[a\n\x1D\x91\x90a)\\V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06@W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a WW`\0\x80\xFD[\x815a\n\x1A\x81a 0V[`\0` \x82\x84\x03\x12\x15a tW`\0\x80\xFD[P5\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a \x8FW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80\x84\x86\x03`\xA0\x81\x12\x15a \xABW`\0\x80\xFD[\x855a \xB6\x81a 0V[\x94P` \x86\x015a \xC6\x81a 0V[\x93P`@`?\x19\x82\x01\x12\x15a \xDAW`\0\x80\xFD[P`@\x85\x01\x91Pa \xED`\x80\x86\x01a {V[\x90P\x92\x95\x91\x94P\x92PV[\x805`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a \x8FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a!7W`\0\x80\xFD[\x815` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15a!SWa!Sa!\x10V[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15a!xWa!xa!\x10V[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15a!\x96W`\0\x80\xFD[\x83\x87\x01\x91P[\x84\x82\x10\x15a!\xBEW\x815a!\xAF\x81a 0V[\x83R\x91\x83\x01\x91\x90\x83\x01\x90a!\x9CV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a!\xDFW`\0\x80\xFD[\x845a!\xEA\x81a 0V[\x93Pa!\xF8` \x86\x01a \xF8V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x13W`\0\x80\xFD[a\"\x1F\x87\x82\x88\x01a!&V[\x92PPa \xED``\x86\x01a {V[`\0\x80`@\x83\x85\x03\x12\x15a\"AW`\0\x80\xFD[\x825a\"L\x81a 0V[\x91P` \x83\x015a\"\\\x81a 0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"|W`\0\x80\xFD[\x835a\"\x87\x81a 0V[\x92P` \x84\x015a\"\x97\x81a 0V[\x91Pa\"\xA5`@\x85\x01a {V[\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\xC3W`\0\x80\xFD[\x835a\"\xCE\x81a 0V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xE9W`\0\x80\xFD[a\"\xF5\x86\x82\x87\x01a!&V[\x92PPa\"\xA5`@\x85\x01a {V[`\0` \x82\x84\x03\x12\x15a#\x16W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\n\x1AW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a#<W`\0\x80\xFD[\x835a#G\x81a 0V[\x92P` \x84\x015a#W\x81a 0V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a#\x80W`\0\x80\xFD[\x855a#\x8B\x81a 0V[\x94Pa#\x99` \x87\x01a \xF8V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xB4W`\0\x80\xFD[a#\xC0\x88\x82\x89\x01a!&V[\x93PPa#\xCF``\x87\x01a {V[\x91Pa#\xDD`\x80\x87\x01a {V[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a#\xFFW`\0\x80\xFD[\x845a$\n\x81a 0V[\x93P` \x85\x015a$\x1A\x81a 0V[\x92Pa$(`@\x86\x01a {V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0` \x82\x84\x03\x12\x15a$JW`\0\x80\xFD[\x81Qa\n\x1A\x81a 0V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a$\xB1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\n\x1AW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\xA0\x82\x01\x90\x855a%&\x81a 0V[\x81\x81\x16` \x85\x01RPc\xFF\xFF\xFF\xFF`\xE0\x1Ba%C` \x88\x01a \xF8V[\x16`@\x84\x01R\x80\x85\x16``\x84\x01RPc\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a%yW`\0\x80\xFD[\x81Qa\xFF\xFF\x81\x16\x81\x14a\n\x1AW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a%\x9DW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a%\xBFWa%\xBFa!\x10V[`@R\x825a%\xCD\x81a 0V[\x81Ra%\xDB` \x84\x01a \xF8V[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a& Wa& a%\xE7V[\x02\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a&^Wa&^a%\xE7V[\x01\x94\x93PPPPV[`\0`\0\x19\x82\x14\x15a&{Wa&{a%\xE7V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a&\x94Wa&\x94a%\xE7V[P\x03\x90V[`\0\x81`\x03\x0Bc\x7F\xFF\xFF\xFF\x19\x81\x14\x15a&\xB4Wa&\xB4a%\xE7V[`\0\x03\x92\x91PPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a'\x1FWa'\x0F\x84\x83Qa&\xBDV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a&\xFCV[P\x91\x97\x96PPPPPPPV[`\0\x81`\x03\x0B\x83`\x03\x0B`\0\x82\x12\x82c\x7F\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a'SWa'Sa%\xE7V[\x82c\x7F\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a'lWa'la%\xE7V[P\x01\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\xA0\x82\x01\x90a'\x94` \x84\x01\x87a&\xBDV[\x80\x85\x16``\x84\x01RPc\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x01R\x95\x94PPPPPV[`\0\x81`\x07\x0B\x83`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a'\xE3Wa'\xE3a%\xE7V[g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a(\x07Wa(\x07a%\xE7V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a(#Wa(#a%\xE7V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a(9Wa(9a%\xE7V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x81`\x07\x0B\x83`\x07\x0B`\0\x82\x12\x82g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a(tWa(ta%\xE7V[\x82g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a'lWa'la%\xE7V[`\0`\xC0\x82\x01c\xFF\xFF\xFF\xFF\x88\x16\x83R` `\x01\x80`\xA0\x1B\x03\x80\x89\x16\x82\x86\x01Ra(\xBD`@\x86\x01\x89a&\xBDV[`\xC0`\x80\x86\x01R\x86Q\x92\x83\x90R\x81\x87\x01\x92`\xE0\x86\x01\x90`\0[\x81\x81\x10\x15a(\xF4W\x85Q\x84\x16\x83R\x94\x84\x01\x94\x91\x84\x01\x91`\x01\x01a(\xD6V[PP\x80\x94PPPPP\x82`\x03\x0B`\xA0\x83\x01R\x96\x95PPPPPPV[`\0\x81a)\x1FWa)\x1Fa%\xE7V[P`\0\x19\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a)AWa)Aa%\xE7V[P\x02\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)kWa)ka)FV[P\x04\x90V[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a)\x90Wa)\x90a%\xE7V[\x03\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a& Wa& a%\xE7V[`\0`\x01`\x01`@\x1B\x03\x80\x84\x16\x80a)\xD8Wa)\xD8a)FV[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xCE\xFC\x06\x84\x90\xF4\xFD5\x86\x9EYat\xBF\xAB\x1F\x01\x1F\xB4\xA6fa\x9D&\xF0j\x01\xD7\xF6\xD2m\xDFdsolcC\0\x08\x0C\x003",
    );
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
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
```solidity
event Paused(address indexed account, uint256 newPausedStatus);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                171u8,
                64u8,
                163u8,
                116u8,
                188u8,
                81u8,
                222u8,
                55u8,
                34u8,
                0u8,
                168u8,
                188u8,
                152u8,
                26u8,
                248u8,
                201u8,
                236u8,
                220u8,
                8u8,
                223u8,
                218u8,
                239u8,
                11u8,
                182u8,
                224u8,
                159u8,
                136u8,
                243u8,
                198u8,
                22u8,
                239u8,
                61u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PauserRegistrySet(address,address)` and selector `0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6`.
```solidity
event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PauserRegistrySet {
        #[allow(missing_docs)]
        pub pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPauserRegistry: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PauserRegistrySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauserRegistrySet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pauserRegistry: data.0,
                    newPauserRegistry: data.1,
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
                        &self.pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newPauserRegistry,
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
        impl alloy_sol_types::private::IntoLogData for PauserRegistrySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PauserRegistrySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PauserRegistrySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RequestedBipsToSlashModified(uint32,address,(address,bytes4),address[],int32)` and selector `0x51b15dc60a707d9c43660fdd6af7cf86060e2778638d04ef462faa56241ea6bf`.
```solidity
event RequestedBipsToSlashModified(uint32 epoch, address operator, IOperatorSetManager.OperatorSet operatorSet, address[] strategies, int32 bipsToModify);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RequestedBipsToSlashModified {
        #[allow(missing_docs)]
        pub epoch: u32,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub bipsToModify: i32,
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
        impl alloy_sol_types::SolEvent for RequestedBipsToSlashModified {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Int<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RequestedBipsToSlashModified(uint32,address,(address,bytes4),address[],int32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                81u8,
                177u8,
                93u8,
                198u8,
                10u8,
                112u8,
                125u8,
                156u8,
                67u8,
                102u8,
                15u8,
                221u8,
                106u8,
                247u8,
                207u8,
                134u8,
                6u8,
                14u8,
                39u8,
                120u8,
                99u8,
                141u8,
                4u8,
                239u8,
                70u8,
                47u8,
                170u8,
                86u8,
                36u8,
                30u8,
                166u8,
                191u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    epoch: data.0,
                    operator: data.1,
                    operatorSet: data.2,
                    strategies: data.3,
                    bipsToModify: data.4,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <IOperatorSetManager::OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Int<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.bipsToModify),
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
        impl alloy_sol_types::private::IntoLogData for RequestedBipsToSlashModified {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RequestedBipsToSlashModified> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RequestedBipsToSlashModified,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashingExecuted(uint32,address,address,uint64)` and selector `0x2f679597a08f229c142b2f79a954c91a30bbda82795ef8dee2775b84db969924`.
```solidity
event SlashingExecuted(uint32 epoch, address operator, address strategy, uint64 slashingRate);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingExecuted {
        #[allow(missing_docs)]
        pub epoch: u32,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub slashingRate: u64,
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
        impl alloy_sol_types::SolEvent for SlashingExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "SlashingExecuted(uint32,address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                103u8,
                149u8,
                151u8,
                160u8,
                143u8,
                34u8,
                156u8,
                20u8,
                43u8,
                47u8,
                121u8,
                169u8,
                84u8,
                201u8,
                26u8,
                48u8,
                187u8,
                218u8,
                130u8,
                121u8,
                94u8,
                248u8,
                222u8,
                226u8,
                119u8,
                91u8,
                132u8,
                219u8,
                150u8,
                153u8,
                36u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    epoch: data.0,
                    operator: data.1,
                    strategy: data.2,
                    slashingRate: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.slashingRate),
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
        impl alloy_sol_types::private::IntoLogData for SlashingExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlashingExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
```solidity
event Unpaused(address indexed account, uint256 newPausedStatus);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                53u8,
                130u8,
                209u8,
                130u8,
                142u8,
                38u8,
                191u8,
                86u8,
                189u8,
                128u8,
                21u8,
                2u8,
                188u8,
                2u8,
                26u8,
                192u8,
                188u8,
                138u8,
                251u8,
                87u8,
                200u8,
                38u8,
                228u8,
                152u8,
                107u8,
                69u8,
                89u8,
                60u8,
                143u8,
                173u8,
                56u8,
                156u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _strategyManager, address _delegationManager, address _operatorSetManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _strategyManager: alloy::sol_types::private::Address,
        pub _delegationManager: alloy::sol_types::private::Address,
        pub _operatorSetManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (
                        value._strategyManager,
                        value._delegationManager,
                        value._operatorSetManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _strategyManager: tuple.0,
                        _delegationManager: tuple.1,
                        _operatorSetManager: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                        &self._strategyManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._operatorSetManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `canWithdraw(address,address,uint32)` and selector `0x79c415ec`.
```solidity
function canWithdraw(address operator, address strategy, uint32 epoch) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canWithdrawCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`canWithdraw(address,address,uint32)`](canWithdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canWithdrawReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<canWithdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: canWithdrawCall) -> Self {
                    (value.operator, value.strategy, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for canWithdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        epoch: tuple.2,
                    }
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
            impl ::core::convert::From<canWithdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: canWithdrawReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for canWithdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for canWithdrawCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = canWithdrawReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "canWithdraw(address,address,uint32)";
            const SELECTOR: [u8; 4] = [121u8, 196u8, 21u8, 236u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
```solidity
function delegation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
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
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
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
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
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
    /**Function with signature `executeSlashing(address,address[],uint32)` and selector `0x4dcaafb8`.
```solidity
function executeSlashing(address operator, address[] memory strategies, uint32 epoch) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSlashingCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`executeSlashing(address,address[],uint32)`](executeSlashingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeSlashingReturn {}
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u32,
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
            impl ::core::convert::From<executeSlashingCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeSlashingCall) -> Self {
                    (value.operator, value.strategies, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeSlashingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                        epoch: tuple.2,
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
            impl ::core::convert::From<executeSlashingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: executeSlashingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for executeSlashingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeSlashingCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeSlashingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "executeSlashing(address,address[],uint32)";
            const SELECTOR: [u8; 4] = [77u8, 202u8, 175u8, 184u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `getPendingSlashingRate(address,address,(address,bytes4),uint32)` and selector `0x4d54dc3c`.
```solidity
function getPendingSlashingRate(address operator, address strategy, IOperatorSetManager.OperatorSet memory operatorSet, uint32 epoch) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPendingSlashingRateCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`getPendingSlashingRate(address,address,(address,bytes4),uint32)`](getPendingSlashingRateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPendingSlashingRateReturn {
        pub _0: u32,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<getPendingSlashingRateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPendingSlashingRateCall) -> Self {
                    (value.operator, value.strategy, value.operatorSet, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPendingSlashingRateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        operatorSet: tuple.2,
                        epoch: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<getPendingSlashingRateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPendingSlashingRateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPendingSlashingRateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPendingSlashingRateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPendingSlashingRateReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPendingSlashingRate(address,address,(address,bytes4),uint32)";
            const SELECTOR: [u8; 4] = [77u8, 84u8, 220u8, 60u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <IOperatorSetManager::OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `getRequestedSlashingRate(address,address,(address,bytes4),uint32)` and selector `0x2421a64c`.
```solidity
function getRequestedSlashingRate(address operator, address strategy, IOperatorSetManager.OperatorSet memory operatorSet, uint32 epoch) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequestedSlashingRateCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`getRequestedSlashingRate(address,address,(address,bytes4),uint32)`](getRequestedSlashingRateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequestedSlashingRateReturn {
        pub _0: u32,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<getRequestedSlashingRateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRequestedSlashingRateCall) -> Self {
                    (value.operator, value.strategy, value.operatorSet, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRequestedSlashingRateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        operatorSet: tuple.2,
                        epoch: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<getRequestedSlashingRateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRequestedSlashingRateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRequestedSlashingRateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRequestedSlashingRateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IOperatorSetManager::OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRequestedSlashingRateReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRequestedSlashingRate(address,address,(address,bytes4),uint32)";
            const SELECTOR: [u8; 4] = [36u8, 33u8, 166u8, 76u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <IOperatorSetManager::OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `getTotalPendingSlashingRate(address,address,uint32)` and selector `0x90e7cde1`.
```solidity
function getTotalPendingSlashingRate(address operator, address strategy, uint32 epoch) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalPendingSlashingRateCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`getTotalPendingSlashingRate(address,address,uint32)`](getTotalPendingSlashingRateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalPendingSlashingRateReturn {
        pub _0: u32,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<getTotalPendingSlashingRateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalPendingSlashingRateCall) -> Self {
                    (value.operator, value.strategy, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalPendingSlashingRateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        epoch: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<getTotalPendingSlashingRateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalPendingSlashingRateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalPendingSlashingRateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalPendingSlashingRateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalPendingSlashingRateReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalPendingSlashingRate(address,address,uint32)";
            const SELECTOR: [u8; 4] = [144u8, 231u8, 205u8, 225u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `getWithdrawabilityAndScalingFactorAtEpoch(address,address,uint32)` and selector `0x3be2073b`.
```solidity
function getWithdrawabilityAndScalingFactorAtEpoch(address operator, address strategy, uint32 epoch) external view returns (bool, uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawabilityAndScalingFactorAtEpochCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`getWithdrawabilityAndScalingFactorAtEpoch(address,address,uint32)`](getWithdrawabilityAndScalingFactorAtEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawabilityAndScalingFactorAtEpochReturn {
        pub _0: bool,
        pub _1: u64,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<getWithdrawabilityAndScalingFactorAtEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawabilityAndScalingFactorAtEpochCall) -> Self {
                    (value.operator, value.strategy, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawabilityAndScalingFactorAtEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        epoch: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u64);
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
            impl ::core::convert::From<getWithdrawabilityAndScalingFactorAtEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawabilityAndScalingFactorAtEpochReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawabilityAndScalingFactorAtEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawabilityAndScalingFactorAtEpochCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawabilityAndScalingFactorAtEpochReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawabilityAndScalingFactorAtEpoch(address,address,uint32)";
            const SELECTOR: [u8; 4] = [59u8, 226u8, 7u8, 59u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `increaseRequestedBipsToSlash(address,bytes4,address[],uint32)` and selector `0x287a96da`.
```solidity
function increaseRequestedBipsToSlash(address operator, bytes4 operatorSetID, address[] memory strategies, uint32 bipsToIncrease) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseRequestedBipsToSlashCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetID: alloy::sol_types::private::FixedBytes<4>,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub bipsToIncrease: u32,
    }
    ///Container type for the return parameters of the [`increaseRequestedBipsToSlash(address,bytes4,address[],uint32)`](increaseRequestedBipsToSlashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseRequestedBipsToSlashReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u32,
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
            impl ::core::convert::From<increaseRequestedBipsToSlashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseRequestedBipsToSlashCall) -> Self {
                    (
                        value.operator,
                        value.operatorSetID,
                        value.strategies,
                        value.bipsToIncrease,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseRequestedBipsToSlashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetID: tuple.1,
                        strategies: tuple.2,
                        bipsToIncrease: tuple.3,
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
            impl ::core::convert::From<increaseRequestedBipsToSlashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: increaseRequestedBipsToSlashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for increaseRequestedBipsToSlashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for increaseRequestedBipsToSlashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = increaseRequestedBipsToSlashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increaseRequestedBipsToSlash(address,bytes4,address[],uint32)";
            const SELECTOR: [u8; 4] = [40u8, 122u8, 150u8, 218u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetID),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.bipsToIncrease),
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
    /**Function with signature `lastSlashed(address,address)` and selector `0x5ab112d6`.
```solidity
function lastSlashed(address operator, address strategy) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastSlashedCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`lastSlashed(address,address)`](lastSlashedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastSlashedReturn {
        pub _0: u32,
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
            impl ::core::convert::From<lastSlashedCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastSlashedCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastSlashedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<lastSlashedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastSlashedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastSlashedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastSlashedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastSlashedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastSlashed(address,address)";
            const SELECTOR: [u8; 4] = [90u8, 177u8, 18u8, 214u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
    /**Function with signature `operatorSetManager()` and selector `0xc78d4bcd`.
```solidity
function operatorSetManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetManagerCall {}
    ///Container type for the return parameters of the [`operatorSetManager()`](operatorSetManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetManagerReturn {
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
            impl ::core::convert::From<operatorSetManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetManagerCall {
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
            impl ::core::convert::From<operatorSetManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSetManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSetManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSetManager()";
            const SELECTOR: [u8; 4] = [199u8, 141u8, 75u8, 205u8];
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
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
```solidity
function pause(uint256 newPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
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
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
```solidity
function pauseAll() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall {}
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
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
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
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
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
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
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
```solidity
function paused(uint8 index) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
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
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call {}
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
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
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
```solidity
function pauserRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall {}
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
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
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
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
            impl ::core::convert::From<pauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
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
    /**Function with signature `pendingShareScalingFactor(address,address)` and selector `0x3dd9e7c5`.
```solidity
function pendingShareScalingFactor(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingShareScalingFactorCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`pendingShareScalingFactor(address,address)`](pendingShareScalingFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingShareScalingFactorReturn {
        pub _0: u64,
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
            impl ::core::convert::From<pendingShareScalingFactorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingShareScalingFactorCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingShareScalingFactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<pendingShareScalingFactorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pendingShareScalingFactorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pendingShareScalingFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingShareScalingFactorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingShareScalingFactorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingShareScalingFactor(address,address)";
            const SELECTOR: [u8; 4] = [61u8, 217u8, 231u8, 197u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
    /**Function with signature `reduceRequestedBipsToSlash(address,bytes4,address[],uint32,uint32)` and selector `0x9d086ecb`.
```solidity
function reduceRequestedBipsToSlash(address operator, bytes4 operatorSetID, address[] memory strategies, uint32 epoch, uint32 bipsToReduce) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reduceRequestedBipsToSlashCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetID: alloy::sol_types::private::FixedBytes<4>,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub epoch: u32,
        pub bipsToReduce: u32,
    }
    ///Container type for the return parameters of the [`reduceRequestedBipsToSlash(address,bytes4,address[],uint32,uint32)`](reduceRequestedBipsToSlashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reduceRequestedBipsToSlashReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                u32,
                u32,
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
            impl ::core::convert::From<reduceRequestedBipsToSlashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: reduceRequestedBipsToSlashCall) -> Self {
                    (
                        value.operator,
                        value.operatorSetID,
                        value.strategies,
                        value.epoch,
                        value.bipsToReduce,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reduceRequestedBipsToSlashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetID: tuple.1,
                        strategies: tuple.2,
                        epoch: tuple.3,
                        bipsToReduce: tuple.4,
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
            impl ::core::convert::From<reduceRequestedBipsToSlashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: reduceRequestedBipsToSlashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reduceRequestedBipsToSlashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reduceRequestedBipsToSlashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reduceRequestedBipsToSlashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reduceRequestedBipsToSlash(address,bytes4,address[],uint32,uint32)";
            const SELECTOR: [u8; 4] = [157u8, 8u8, 110u8, 203u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetID),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.bipsToReduce),
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
    /**Function with signature `requestedSlashedBips(address,address,uint32,bytes32)` and selector `0xec65b53d`.
```solidity
function requestedSlashedBips(address, address, uint32, bytes32) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestedSlashedBipsCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
        pub _2: u32,
        pub _3: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`requestedSlashedBips(address,address,uint32,bytes32)`](requestedSlashedBipsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestedSlashedBipsReturn {
        pub _0: u32,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<requestedSlashedBipsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestedSlashedBipsCall) -> Self {
                    (value._0, value._1, value._2, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestedSlashedBipsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<requestedSlashedBipsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestedSlashedBipsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestedSlashedBipsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestedSlashedBipsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestedSlashedBipsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestedSlashedBips(address,address,uint32,bytes32)";
            const SELECTOR: [u8; 4] = [236u8, 101u8, 181u8, 61u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._3),
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
    /**Function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`.
```solidity
function setPauserRegistry(address newPauserRegistry) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryCall {
        pub newPauserRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPauserRegistry(address)`](setPauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryReturn {}
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
            impl ::core::convert::From<setPauserRegistryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryCall) -> Self {
                    (value.newPauserRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPauserRegistry: tuple.0 }
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
            impl ::core::convert::From<setPauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPauserRegistryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPauserRegistryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPauserRegistry(address)";
            const SELECTOR: [u8; 4] = [16u8, 214u8, 122u8, 47u8];
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
                        &self.newPauserRegistry,
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
    /**Function with signature `shareScalingFactor(address,address)` and selector `0x334f00d6`.
```solidity
function shareScalingFactor(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareScalingFactorCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`shareScalingFactor(address,address)`](shareScalingFactorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareScalingFactorReturn {
        pub _0: u64,
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
            impl ::core::convert::From<shareScalingFactorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: shareScalingFactorCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for shareScalingFactorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<shareScalingFactorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: shareScalingFactorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for shareScalingFactorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for shareScalingFactorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = shareScalingFactorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "shareScalingFactor(address,address)";
            const SELECTOR: [u8; 4] = [51u8, 79u8, 0u8, 214u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
    /**Function with signature `shareScalingFactorAtEpoch(address,address,uint32)` and selector `0xe49a1e84`.
```solidity
function shareScalingFactorAtEpoch(address operator, address strategy, uint32 epoch) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareScalingFactorAtEpochCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub epoch: u32,
    }
    ///Container type for the return parameters of the [`shareScalingFactorAtEpoch(address,address,uint32)`](shareScalingFactorAtEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct shareScalingFactorAtEpochReturn {
        pub _0: u64,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<shareScalingFactorAtEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: shareScalingFactorAtEpochCall) -> Self {
                    (value.operator, value.strategy, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for shareScalingFactorAtEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        epoch: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<shareScalingFactorAtEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: shareScalingFactorAtEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for shareScalingFactorAtEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for shareScalingFactorAtEpochCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = shareScalingFactorAtEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "shareScalingFactorAtEpoch(address,address,uint32)";
            const SELECTOR: [u8; 4] = [228u8, 154u8, 30u8, 132u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Function with signature `slashedEpochHistory(address,address,uint256)` and selector `0x6c0d75d0`.
```solidity
function slashedEpochHistory(address, address, uint256) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashedEpochHistoryCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
        pub _2: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`slashedEpochHistory(address,address,uint256)`](slashedEpochHistoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashedEpochHistoryReturn {
        pub _0: u32,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<slashedEpochHistoryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashedEpochHistoryCall) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashedEpochHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<slashedEpochHistoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashedEpochHistoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashedEpochHistoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashedEpochHistoryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashedEpochHistoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashedEpochHistory(address,address,uint256)";
            const SELECTOR: [u8; 4] = [108u8, 13u8, 117u8, 208u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
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
    /**Function with signature `slashingRequestIds(address,address)` and selector `0x7ef639a6`.
```solidity
function slashingRequestIds(address, address) external view returns (uint32 lastCreatedSlashingRequestId, uint32 lastExecutedSlashingRequestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRequestIdsCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`slashingRequestIds(address,address)`](slashingRequestIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRequestIdsReturn {
        pub lastCreatedSlashingRequestId: u32,
        pub lastExecutedSlashingRequestId: u32,
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
            impl ::core::convert::From<slashingRequestIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRequestIdsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRequestIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u32);
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
            impl ::core::convert::From<slashingRequestIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRequestIdsReturn) -> Self {
                    (
                        value.lastCreatedSlashingRequestId,
                        value.lastExecutedSlashingRequestId,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRequestIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        lastCreatedSlashingRequestId: tuple.0,
                        lastExecutedSlashingRequestId: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashingRequestIdsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashingRequestIdsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashingRequestIds(address,address)";
            const SELECTOR: [u8; 4] = [126u8, 246u8, 57u8, 166u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `slashingRequests(address,address,uint32)` and selector `0x3f2201bb`.
```solidity
function slashingRequests(address, address, uint32) external view returns (uint32 id, uint64 slashingRate, uint64 scalingFactor);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRequestsCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
        pub _2: u32,
    }
    ///Container type for the return parameters of the [`slashingRequests(address,address,uint32)`](slashingRequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRequestsReturn {
        pub id: u32,
        pub slashingRate: u64,
        pub scalingFactor: u64,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
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
            impl ::core::convert::From<slashingRequestsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRequestsCall) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRequestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, u64, u64);
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
            impl ::core::convert::From<slashingRequestsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRequestsReturn) -> Self {
                    (value.id, value.slashingRate, value.scalingFactor)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        slashingRate: tuple.1,
                        scalingFactor: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashingRequestsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashingRequestsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashingRequests(address,address,uint32)";
            const SELECTOR: [u8; 4] = [63u8, 34u8, 1u8, 187u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
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
    /**Function with signature `strategyManager()` and selector `0x39b70e38`.
```solidity
function strategyManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerCall {}
    ///Container type for the return parameters of the [`strategyManager()`](strategyManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerReturn {
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
            impl ::core::convert::From<strategyManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyManagerCall {
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
            impl ::core::convert::From<strategyManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyManager()";
            const SELECTOR: [u8; 4] = [57u8, 183u8, 14u8, 56u8];
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
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
```solidity
function unpause(uint256 newPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPausedStatus: tuple.0 }
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
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newPausedStatus),
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
    ///Container for all the [`Slasher`](self) function calls.
    pub enum SlasherCalls {
        canWithdraw(canWithdrawCall),
        delegation(delegationCall),
        executeSlashing(executeSlashingCall),
        getPendingSlashingRate(getPendingSlashingRateCall),
        getRequestedSlashingRate(getRequestedSlashingRateCall),
        getTotalPendingSlashingRate(getTotalPendingSlashingRateCall),
        getWithdrawabilityAndScalingFactorAtEpoch(
            getWithdrawabilityAndScalingFactorAtEpochCall,
        ),
        increaseRequestedBipsToSlash(increaseRequestedBipsToSlashCall),
        lastSlashed(lastSlashedCall),
        operatorSetManager(operatorSetManagerCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        pendingShareScalingFactor(pendingShareScalingFactorCall),
        reduceRequestedBipsToSlash(reduceRequestedBipsToSlashCall),
        renounceOwnership(renounceOwnershipCall),
        requestedSlashedBips(requestedSlashedBipsCall),
        setPauserRegistry(setPauserRegistryCall),
        shareScalingFactor(shareScalingFactorCall),
        shareScalingFactorAtEpoch(shareScalingFactorAtEpochCall),
        slashedEpochHistory(slashedEpochHistoryCall),
        slashingRequestIds(slashingRequestIdsCall),
        slashingRequests(slashingRequestsCall),
        strategyManager(strategyManagerCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl SlasherCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 100u8, 57u8, 221u8],
            [36u8, 33u8, 166u8, 76u8],
            [40u8, 122u8, 150u8, 218u8],
            [51u8, 79u8, 0u8, 214u8],
            [57u8, 183u8, 14u8, 56u8],
            [59u8, 226u8, 7u8, 59u8],
            [61u8, 217u8, 231u8, 197u8],
            [63u8, 34u8, 1u8, 187u8],
            [77u8, 84u8, 220u8, 60u8],
            [77u8, 202u8, 175u8, 184u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 177u8, 18u8, 214u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [108u8, 13u8, 117u8, 208u8],
            [113u8, 80u8, 24u8, 166u8],
            [121u8, 196u8, 21u8, 236u8],
            [126u8, 246u8, 57u8, 166u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 231u8, 205u8, 225u8],
            [157u8, 8u8, 110u8, 203u8],
            [199u8, 141u8, 75u8, 205u8],
            [223u8, 92u8, 247u8, 35u8],
            [228u8, 154u8, 30u8, 132u8],
            [236u8, 101u8, 181u8, 61u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SlasherCalls {
        const NAME: &'static str = "SlasherCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::canWithdraw(_) => {
                    <canWithdrawCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::executeSlashing(_) => {
                    <executeSlashingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPendingSlashingRate(_) => {
                    <getPendingSlashingRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRequestedSlashingRate(_) => {
                    <getRequestedSlashingRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalPendingSlashingRate(_) => {
                    <getTotalPendingSlashingRateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawabilityAndScalingFactorAtEpoch(_) => {
                    <getWithdrawabilityAndScalingFactorAtEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::increaseRequestedBipsToSlash(_) => {
                    <increaseRequestedBipsToSlashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastSlashed(_) => {
                    <lastSlashedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSetManager(_) => {
                    <operatorSetManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingShareScalingFactor(_) => {
                    <pendingShareScalingFactorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reduceRequestedBipsToSlash(_) => {
                    <reduceRequestedBipsToSlashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestedSlashedBips(_) => {
                    <requestedSlashedBipsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::shareScalingFactor(_) => {
                    <shareScalingFactorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::shareScalingFactorAtEpoch(_) => {
                    <shareScalingFactorAtEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashedEpochHistory(_) => {
                    <slashedEpochHistoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashingRequestIds(_) => {
                    <slashingRequestIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashingRequests(_) => {
                    <slashingRequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManager(_) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<SlasherCalls>] = &[
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::pause)
                    }
                    pause
                },
                {
                    fn getRequestedSlashingRate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <getRequestedSlashingRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::getRequestedSlashingRate)
                    }
                    getRequestedSlashingRate
                },
                {
                    fn increaseRequestedBipsToSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <increaseRequestedBipsToSlashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::increaseRequestedBipsToSlash)
                    }
                    increaseRequestedBipsToSlash
                },
                {
                    fn shareScalingFactor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <shareScalingFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::shareScalingFactor)
                    }
                    shareScalingFactor
                },
                {
                    fn strategyManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <strategyManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::strategyManager)
                    }
                    strategyManager
                },
                {
                    fn getWithdrawabilityAndScalingFactorAtEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <getWithdrawabilityAndScalingFactorAtEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::getWithdrawabilityAndScalingFactorAtEpoch)
                    }
                    getWithdrawabilityAndScalingFactorAtEpoch
                },
                {
                    fn pendingShareScalingFactor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <pendingShareScalingFactorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::pendingShareScalingFactor)
                    }
                    pendingShareScalingFactor
                },
                {
                    fn slashingRequests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <slashingRequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::slashingRequests)
                    }
                    slashingRequests
                },
                {
                    fn getPendingSlashingRate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <getPendingSlashingRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::getPendingSlashingRate)
                    }
                    getPendingSlashingRate
                },
                {
                    fn executeSlashing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <executeSlashingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::executeSlashing)
                    }
                    executeSlashing
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn lastSlashed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <lastSlashedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::lastSlashed)
                    }
                    lastSlashed
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn slashedEpochHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <slashedEpochHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::slashedEpochHistory)
                    }
                    slashedEpochHistory
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn canWithdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <canWithdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::canWithdraw)
                    }
                    canWithdraw
                },
                {
                    fn slashingRequestIds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <slashingRequestIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::slashingRequestIds)
                    }
                    slashingRequestIds
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::owner)
                    }
                    owner
                },
                {
                    fn getTotalPendingSlashingRate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <getTotalPendingSlashingRateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::getTotalPendingSlashingRate)
                    }
                    getTotalPendingSlashingRate
                },
                {
                    fn reduceRequestedBipsToSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <reduceRequestedBipsToSlashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::reduceRequestedBipsToSlash)
                    }
                    reduceRequestedBipsToSlash
                },
                {
                    fn operatorSetManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <operatorSetManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::operatorSetManager)
                    }
                    operatorSetManager
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::delegation)
                    }
                    delegation
                },
                {
                    fn shareScalingFactorAtEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <shareScalingFactorAtEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::shareScalingFactorAtEpoch)
                    }
                    shareScalingFactorAtEpoch
                },
                {
                    fn requestedSlashedBips(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <requestedSlashedBipsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::requestedSlashedBips)
                    }
                    requestedSlashedBips
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SlasherCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SlasherCalls::unpause)
                    }
                    unpause
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
                Self::canWithdraw(inner) => {
                    <canWithdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::executeSlashing(inner) => {
                    <executeSlashingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPendingSlashingRate(inner) => {
                    <getPendingSlashingRateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRequestedSlashingRate(inner) => {
                    <getRequestedSlashingRateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalPendingSlashingRate(inner) => {
                    <getTotalPendingSlashingRateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawabilityAndScalingFactorAtEpoch(inner) => {
                    <getWithdrawabilityAndScalingFactorAtEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::increaseRequestedBipsToSlash(inner) => {
                    <increaseRequestedBipsToSlashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastSlashed(inner) => {
                    <lastSlashedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSetManager(inner) => {
                    <operatorSetManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pendingShareScalingFactor(inner) => {
                    <pendingShareScalingFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reduceRequestedBipsToSlash(inner) => {
                    <reduceRequestedBipsToSlashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestedSlashedBips(inner) => {
                    <requestedSlashedBipsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::shareScalingFactor(inner) => {
                    <shareScalingFactorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::shareScalingFactorAtEpoch(inner) => {
                    <shareScalingFactorAtEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashedEpochHistory(inner) => {
                    <slashedEpochHistoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashingRequestIds(inner) => {
                    <slashingRequestIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashingRequests(inner) => {
                    <slashingRequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::canWithdraw(inner) => {
                    <canWithdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::executeSlashing(inner) => {
                    <executeSlashingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPendingSlashingRate(inner) => {
                    <getPendingSlashingRateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRequestedSlashingRate(inner) => {
                    <getRequestedSlashingRateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalPendingSlashingRate(inner) => {
                    <getTotalPendingSlashingRateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawabilityAndScalingFactorAtEpoch(inner) => {
                    <getWithdrawabilityAndScalingFactorAtEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::increaseRequestedBipsToSlash(inner) => {
                    <increaseRequestedBipsToSlashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastSlashed(inner) => {
                    <lastSlashedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSetManager(inner) => {
                    <operatorSetManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pendingShareScalingFactor(inner) => {
                    <pendingShareScalingFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reduceRequestedBipsToSlash(inner) => {
                    <reduceRequestedBipsToSlashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::requestedSlashedBips(inner) => {
                    <requestedSlashedBipsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::shareScalingFactor(inner) => {
                    <shareScalingFactorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::shareScalingFactorAtEpoch(inner) => {
                    <shareScalingFactorAtEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashedEpochHistory(inner) => {
                    <slashedEpochHistoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashingRequestIds(inner) => {
                    <slashingRequestIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashingRequests(inner) => {
                    <slashingRequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`Slasher`](self) events.
    pub enum SlasherEvents {
        Initialized(Initialized),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
        RequestedBipsToSlashModified(RequestedBipsToSlashModified),
        SlashingExecuted(SlashingExecuted),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl SlasherEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                47u8,
                103u8,
                149u8,
                151u8,
                160u8,
                143u8,
                34u8,
                156u8,
                20u8,
                43u8,
                47u8,
                121u8,
                169u8,
                84u8,
                201u8,
                26u8,
                48u8,
                187u8,
                218u8,
                130u8,
                121u8,
                94u8,
                248u8,
                222u8,
                226u8,
                119u8,
                91u8,
                132u8,
                219u8,
                150u8,
                153u8,
                36u8,
            ],
            [
                53u8,
                130u8,
                209u8,
                130u8,
                142u8,
                38u8,
                191u8,
                86u8,
                189u8,
                128u8,
                21u8,
                2u8,
                188u8,
                2u8,
                26u8,
                192u8,
                188u8,
                138u8,
                251u8,
                87u8,
                200u8,
                38u8,
                228u8,
                152u8,
                107u8,
                69u8,
                89u8,
                60u8,
                143u8,
                173u8,
                56u8,
                156u8,
            ],
            [
                81u8,
                177u8,
                93u8,
                198u8,
                10u8,
                112u8,
                125u8,
                156u8,
                67u8,
                102u8,
                15u8,
                221u8,
                106u8,
                247u8,
                207u8,
                134u8,
                6u8,
                14u8,
                39u8,
                120u8,
                99u8,
                141u8,
                4u8,
                239u8,
                70u8,
                47u8,
                170u8,
                86u8,
                36u8,
                30u8,
                166u8,
                191u8,
            ],
            [
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
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
                171u8,
                64u8,
                163u8,
                116u8,
                188u8,
                81u8,
                222u8,
                55u8,
                34u8,
                0u8,
                168u8,
                188u8,
                152u8,
                26u8,
                248u8,
                201u8,
                236u8,
                220u8,
                8u8,
                223u8,
                218u8,
                239u8,
                11u8,
                182u8,
                224u8,
                159u8,
                136u8,
                243u8,
                198u8,
                22u8,
                239u8,
                61u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SlasherEvents {
        const NAME: &'static str = "SlasherEvents";
        const COUNT: usize = 7usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
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
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Paused)
                }
                Some(
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PauserRegistrySet)
                }
                Some(
                    <RequestedBipsToSlashModified as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RequestedBipsToSlashModified as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RequestedBipsToSlashModified)
                }
                Some(<SlashingExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SlashingExecuted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingExecuted)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for SlasherEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RequestedBipsToSlashModified(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RequestedBipsToSlashModified(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Slasher`](self) contract instance.

See the [wrapper's documentation](`SlasherInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SlasherInstance<T, P, N> {
        SlasherInstance::<T, P, N>::new(address, provider)
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
        _strategyManager: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _operatorSetManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SlasherInstance<T, P, N>>,
    > {
        SlasherInstance::<
            T,
            P,
            N,
        >::deploy(provider, _strategyManager, _delegationManager, _operatorSetManager)
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
        _strategyManager: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _operatorSetManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SlasherInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _strategyManager,
            _delegationManager,
            _operatorSetManager,
        )
    }
    /**A [`Slasher`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Slasher`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SlasherInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SlasherInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SlasherInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SlasherInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Slasher`](self) contract instance.

See the [wrapper's documentation](`SlasherInstance`) for more details.*/
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
            _strategyManager: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _operatorSetManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SlasherInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _strategyManager,
                _delegationManager,
                _operatorSetManager,
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
            _strategyManager: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _operatorSetManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _strategyManager,
                            _delegationManager,
                            _operatorSetManager,
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
    impl<T, P: ::core::clone::Clone, N> SlasherInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SlasherInstance<T, P, N> {
            SlasherInstance {
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
    > SlasherInstance<T, P, N> {
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
        ///Creates a new call builder for the [`canWithdraw`] function.
        pub fn canWithdraw(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, canWithdrawCall, N> {
            self.call_builder(
                &canWithdrawCall {
                    operator,
                    strategy,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`executeSlashing`] function.
        pub fn executeSlashing(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeSlashingCall, N> {
            self.call_builder(
                &executeSlashingCall {
                    operator,
                    strategies,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`getPendingSlashingRate`] function.
        pub fn getPendingSlashingRate(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPendingSlashingRateCall, N> {
            self.call_builder(
                &getPendingSlashingRateCall {
                    operator,
                    strategy,
                    operatorSet,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`getRequestedSlashingRate`] function.
        pub fn getRequestedSlashingRate(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            operatorSet: <IOperatorSetManager::OperatorSet as alloy::sol_types::SolType>::RustType,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRequestedSlashingRateCall, N> {
            self.call_builder(
                &getRequestedSlashingRateCall {
                    operator,
                    strategy,
                    operatorSet,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalPendingSlashingRate`] function.
        pub fn getTotalPendingSlashingRate(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalPendingSlashingRateCall, N> {
            self.call_builder(
                &getTotalPendingSlashingRateCall {
                    operator,
                    strategy,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`getWithdrawabilityAndScalingFactorAtEpoch`] function.
        pub fn getWithdrawabilityAndScalingFactorAtEpoch(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getWithdrawabilityAndScalingFactorAtEpochCall,
            N,
        > {
            self.call_builder(
                &getWithdrawabilityAndScalingFactorAtEpochCall {
                    operator,
                    strategy,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`increaseRequestedBipsToSlash`] function.
        pub fn increaseRequestedBipsToSlash(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetID: alloy::sol_types::private::FixedBytes<4>,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            bipsToIncrease: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseRequestedBipsToSlashCall, N> {
            self.call_builder(
                &increaseRequestedBipsToSlashCall {
                    operator,
                    operatorSetID,
                    strategies,
                    bipsToIncrease,
                },
            )
        }
        ///Creates a new call builder for the [`lastSlashed`] function.
        pub fn lastSlashed(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastSlashedCall, N> {
            self.call_builder(
                &lastSlashedCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`operatorSetManager`] function.
        pub fn operatorSetManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSetManagerCall, N> {
            self.call_builder(&operatorSetManagerCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall {})
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(
            &self,
            index: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`pendingShareScalingFactor`] function.
        pub fn pendingShareScalingFactor(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingShareScalingFactorCall, N> {
            self.call_builder(
                &pendingShareScalingFactorCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`reduceRequestedBipsToSlash`] function.
        pub fn reduceRequestedBipsToSlash(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetID: alloy::sol_types::private::FixedBytes<4>,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            epoch: u32,
            bipsToReduce: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, reduceRequestedBipsToSlashCall, N> {
            self.call_builder(
                &reduceRequestedBipsToSlashCall {
                    operator,
                    operatorSetID,
                    strategies,
                    epoch,
                    bipsToReduce,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`requestedSlashedBips`] function.
        pub fn requestedSlashedBips(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: u32,
            _3: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, requestedSlashedBipsCall, N> {
            self.call_builder(
                &requestedSlashedBipsCall {
                    _0,
                    _1,
                    _2,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`setPauserRegistry`] function.
        pub fn setPauserRegistry(
            &self,
            newPauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPauserRegistryCall, N> {
            self.call_builder(
                &setPauserRegistryCall {
                    newPauserRegistry,
                },
            )
        }
        ///Creates a new call builder for the [`shareScalingFactor`] function.
        pub fn shareScalingFactor(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, shareScalingFactorCall, N> {
            self.call_builder(
                &shareScalingFactorCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`shareScalingFactorAtEpoch`] function.
        pub fn shareScalingFactorAtEpoch(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            epoch: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, shareScalingFactorAtEpochCall, N> {
            self.call_builder(
                &shareScalingFactorAtEpochCall {
                    operator,
                    strategy,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`slashedEpochHistory`] function.
        pub fn slashedEpochHistory(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashedEpochHistoryCall, N> {
            self.call_builder(
                &slashedEpochHistoryCall {
                    _0,
                    _1,
                    _2,
                },
            )
        }
        ///Creates a new call builder for the [`slashingRequestIds`] function.
        pub fn slashingRequestIds(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashingRequestIdsCall, N> {
            self.call_builder(&slashingRequestIdsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`slashingRequests`] function.
        pub fn slashingRequests(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashingRequestsCall, N> {
            self.call_builder(&slashingRequestsCall { _0, _1, _2 })
        }
        ///Creates a new call builder for the [`strategyManager`] function.
        pub fn strategyManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyManagerCall, N> {
            self.call_builder(&strategyManagerCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SlasherInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`PauserRegistrySet`] event.
        pub fn PauserRegistrySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PauserRegistrySet, N> {
            self.event_filter::<PauserRegistrySet>()
        }
        ///Creates a new event filter for the [`RequestedBipsToSlashModified`] event.
        pub fn RequestedBipsToSlashModified_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RequestedBipsToSlashModified, N> {
            self.event_filter::<RequestedBipsToSlashModified>()
        }
        ///Creates a new event filter for the [`SlashingExecuted`] event.
        pub fn SlashingExecuted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingExecuted, N> {
            self.event_filter::<SlashingExecuted>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
