///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        pub addr: alloy::sol_types::private::Address,
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
        >,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<T, P, N> {
        StdInvariantInstance::<T, P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StdInvariantInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> StdInvariantInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<T, P, N> {
            StdInvariantInstance {
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
    > StdInvariantInstance<T, P, N> {
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
    > StdInvariantInstance<T, P, N> {
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
library StdInvariant {
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface ExistingDeploymentParser {
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    function EIGEN() external view returns (address);
    function EIGENImpl() external view returns (address);
    function IS_SCRIPT() external view returns (bool);
    function IS_TEST() external view returns (bool);
    function allEigenPods(uint256) external view returns (address);
    function allocationManager() external view returns (address);
    function allocationManagerImplementation() external view returns (address);
    function avsDirectory() external view returns (address);
    function avsDirectoryImplementation() external view returns (address);
    function bEIGEN() external view returns (address);
    function bEIGENImpl() external view returns (address);
    function baseStrategyImplementation() external view returns (address);
    function delegationManager() external view returns (address);
    function delegationManagerImplementation() external view returns (address);
    function deployedStrategyArray(uint256) external view returns (address);
    function eigenLayerPauserReg() external view returns (address);
    function eigenLayerProxyAdmin() external view returns (address);
    function eigenPodBeacon() external view returns (address);
    function eigenPodImplementation() external view returns (address);
    function eigenPodManager() external view returns (address);
    function eigenPodManagerImplementation() external view returns (address);
    function eigenStrategy() external view returns (address);
    function eigenStrategyImpl() external view returns (address);
    function emptyContract() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external returns (bool);
    function inActivePods(uint256) external view returns (address);
    function logAndOutputContractAddresses(string memory outputPath) external;
    function logInitialDeploymentParams() external;
    function multiValidatorPods(uint256) external view returns (address);
    function rewardsCoordinator() external view returns (address);
    function rewardsCoordinatorImplementation() external view returns (address);
    function singleValidatorPods(uint256) external view returns (address);
    function strategiesToDeploy(uint256) external view returns (address tokenAddress, string memory tokenName, string memory tokenSymbol);
    function strategyBeacon() external view returns (address);
    function strategyFactory() external view returns (address);
    function strategyFactoryBeaconImplementation() external view returns (address);
    function strategyFactoryImplementation() external view returns (address);
    function strategyManager() external view returns (address);
    function strategyManagerImplementation() external view returns (address);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function tokenProxyAdmin() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "EIGEN",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigen"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "EIGENImpl",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigen"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "IS_SCRIPT",
    "inputs": [],
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
    "name": "IS_TEST",
    "inputs": [],
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
    "name": "allEigenPods",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract AllocationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "allocationManagerImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract AllocationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract AVSDirectory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "avsDirectoryImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract AVSDirectory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bEIGEN",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBackingEigen"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bEIGENImpl",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBackingEigen"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "baseStrategyImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StrategyBase"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract DelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationManagerImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract DelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "deployedStrategyArray",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StrategyBase"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenLayerPauserReg",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract PauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenLayerProxyAdmin",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ProxyAdmin"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodBeacon",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract UpgradeableBeacon"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EigenPod"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodManagerImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenStrategy",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EigenStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenStrategyImpl",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EigenStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "emptyContract",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract EmptyContract"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "inActivePods",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "logAndOutputContractAddresses",
    "inputs": [
      {
        "name": "outputPath",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "logInitialDeploymentParams",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "multiValidatorPods",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "rewardsCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RewardsCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "rewardsCoordinatorImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract RewardsCoordinator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "singleValidatorPods",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "strategiesToDeploy",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "tokenAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenName",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "tokenSymbol",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyBeacon",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract UpgradeableBeacon"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyFactory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StrategyFactory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyFactoryBeaconImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StrategyBase"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyFactoryImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StrategyFactory"
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
        "internalType": "contract StrategyManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyManagerImplementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract StrategyManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tokenProxyAdmin",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ProxyAdmin"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
pub mod ExistingDeploymentParser {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405260008054600160ff19918216811790925560048054821683179055601b80549091169091179055348015603657600080fd5b50613866806100466000396000f3fe608060405234801561001057600080fd5b506004361061029f5760003560e01c806385226c8111610167578063db4df761116100ce578063f2ebb0b611610087578063f2ebb0b6146105be578063f39e9160146105d1578063f7e76e36146105e4578063f8ccbf47146105f7578063fa7626d414610604578063fdc371ce1461061157600080fd5b8063db4df76114610557578063e20c9f711461056a578063e3a8b34514610572578063e7ac55fc14610585578063ea4d3c9b14610598578063f0062d9a146105ab57600080fd5b8063ba414fa611610120578063ba414fa6146104db578063ba8c65d8146104f3578063be5bb5f614610506578063c1daca8014610519578063ca8aa7c71461052c578063d0af26e11461053f57600080fd5b806385226c811461047d5780638a2fc4e314610492578063916a17c6146104a557806399c1ef2b146104ad5780639ef35710146104c0578063b5508aa9146104d357600080fd5b80633f4da4c61161020b57806352315640116101c457806352315640146104145780635da8b4ce1461042757806366d9a9a01461042f5780636b3aa72e146104445780636d42c7501461045757806371c56c321461046a57600080fd5b80633f4da4c61461039c5780633f7286f4146103af5780634665bcda146103b757806346e4e1bf146103ca57806347c94dda146103ec578063516e2828146103ff57600080fd5b8063292b7b2b1161025d578063292b7b2b1461033557806332c085851461034857806339b70e381461035b5780633e2bee3b1461036e5780633e5e3c23146103815780633f483ffa1461038957600080fd5b8062919afe146102a45780630492f4bc146102d45780631e2d334b146102e75780631ed7831c146102fa57806321cb3e371461030f5780632689636314610322575b600080fd5b602f546102b7906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6032546102b7906001600160a01b031681565b602b546102b7906001600160a01b031681565b610302610624565b6040516102cb9190612835565b6036546102b7906001600160a01b031681565b6034546102b7906001600160a01b031681565b6027546102b7906001600160a01b031681565b602d546102b7906001600160a01b031681565b6021546102b7906001600160a01b031681565b601e546102b7906001600160a01b031681565b610302610686565b6102b7610397366004612881565b6106e6565b6033546102b7906001600160a01b031681565b610302610710565b6025546102b7906001600160a01b031681565b6103dd6103d8366004612881565b610770565b6040516102cb939291906128ea565b6102b76103fa366004612881565b6108c0565b61041261040d366004612999565b6108d0565b005b6102b7610422366004612881565b611abe565b610412611ace565b610437612321565b6040516102cb9190612a19565b601d546102b7906001600160a01b031681565b601c546102b7906001600160a01b031681565b6024546102b7906001600160a01b031681565b610485612410565b6040516102cb9190612ad3565b6023546102b7906001600160a01b031681565b6104376124e0565b6029546102b7906001600160a01b031681565b602a546102b7906001600160a01b031681565b6104856125c6565b6104e3612696565b60405190151581526020016102cb565b6102b7610501366004612881565b6127b5565b6020546102b7906001600160a01b031681565b6022546102b7906001600160a01b031681565b602c546102b7906001600160a01b031681565b601b546102b79061010090046001600160a01b031681565b6035546102b7906001600160a01b031681565b6103026127c5565b603b546102b7906001600160a01b031681565b6102b7610593366004612881565b612825565b601f546102b7906001600160a01b031681565b602e546102b7906001600160a01b031681565b6030546102b7906001600160a01b031681565b6026546102b7906001600160a01b031681565b6028546102b7906001600160a01b031681565b601b546104e39060ff1681565b6000546104e39060ff1681565b6031546102b7906001600160a01b031681565b6060600d80548060200260200160405190810160405280929190818152602001828054801561067c57602002820191906000526020600020905b81546001600160a01b0316815260019091019060200180831161065e575b5050505050905090565b6060600f80548060200260200160405190810160405280929190818152602001828054801561067c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161065e575050505050905090565b603881815481106106f657600080fd5b6000918252602090912001546001600160a01b0316905081565b6060600e80548060200260200160405190810160405280929190818152602001828054801561067c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161065e575050505050905090565b6044818154811061078057600080fd5b6000918252602090912060039091020180546001820180546001600160a01b039092169350906107af90612b2c565b80601f01602080910402602001604051908101604052809291908181526020018280546107db90612b2c565b80156108285780601f106107fd57610100808354040283529160200191610828565b820191906000526020600020905b81548152906001019060200180831161080b57829003601f168201915b50505050509080600201805461083d90612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461086990612b2c565b80156108b65780601f1061088b576101008083540402835291602001916108b6565b820191906000526020600020905b81548152906001019060200180831161089957829003601f168201915b5050505050905083565b603981815481106106f657600080fd5b604080518082018252600d81526c1c185c995b9d081bd89a9958dd609a1b6020808301919091528251808401909352600a8352697374726174656769657360b01b908301529060005b604354811015610a15577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b031663972c6062836044848154811061096957610969612b66565b90600052602060002090600302016002016042858154811061098d5761098d612b66565b6000918252602090912001546040516001600160e01b031960e086901b1681526109c59392916001600160a01b031690600401612b7c565b6000604051808303816000875af11580156109e4573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610a0c9190810190612c34565b50600101610919565b506000604354600014610b2c577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b031663972c60628360446001604354610a669190612cab565b81548110610a7657610a76612b66565b906000526020600020906003020160020160426001604354610a989190612cab565b81548110610aa857610aa8612b66565b6000918252602090912001546040516001600160e01b031960e086901b168152610ae09392916001600160a01b031690600401612b7c565b6000604051808303816000875af1158015610aff573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610b279190810190612c34565b610b3d565b604051806020016040528060008152505b604080518082018252600981526861646472657373657360b81b6020820152601b549151634b96303160e11b8152929350916000805160206137d18339815191529163972c606291610ba39185916101009091046001600160a01b031690600401612cd2565b6000604051808303816000875af1158015610bc2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610bea9190810190612c34565b50601c54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610c2b9185916001600160a01b0390911690600401612d2a565b6000604051808303816000875af1158015610c4a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c729190810190612c34565b50601d54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610cb39185916001600160a01b0390911690600401612d81565b6000604051808303816000875af1158015610cd2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610cfa9190810190612c34565b50601e54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610d3b9185916001600160a01b0390911690600401612dd1565b6000604051808303816000875af1158015610d5a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d829190810190612c34565b50601f54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610dc39185916001600160a01b0390911690600401612e32565b6000604051808303816000875af1158015610de2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e0a9190810190612c34565b50602054604051634b96303160e11b81526000805160206137d18339815191529163972c606291610e4b9185916001600160a01b0390911690600401612e87565b6000604051808303816000875af1158015610e6a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e929190810190612c34565b50602154604051634b96303160e11b81526000805160206137d18339815191529163972c606291610ed39185916001600160a01b0390911690600401612ee8565b6000604051808303816000875af1158015610ef2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610f1a9190810190612c34565b50602254604051634b96303160e11b81526000805160206137d18339815191529163972c606291610f5b9185916001600160a01b0390911690600401612f3b565b6000604051808303816000875af1158015610f7a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610fa29190810190612c34565b50602354604051634b96303160e11b81526000805160206137d18339815191529163972c606291610fe39185916001600160a01b0390911690600401612f9c565b6000604051808303816000875af1158015611002573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261102a9190810190612c34565b50602454604051634b96303160e11b81526000805160206137d18339815191529163972c60629161106b9185916001600160a01b0390911690600401612ff2565b6000604051808303816000875af115801561108a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110b29190810190612c34565b50602554604051634b96303160e11b81526000805160206137d18339815191529163972c6062916110f39185916001600160a01b0390911690600401613052565b6000604051808303816000875af1158015611112573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261113a9190810190612c34565b50602654604051634b96303160e11b81526000805160206137d18339815191529163972c60629161117b9185916001600160a01b03909116906004016130a5565b6000604051808303816000875af115801561119a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111c29190810190612c34565b50602754604051634b96303160e11b81526000805160206137d18339815191529163972c6062916112039185916001600160a01b0390911690600401613106565b6000604051808303816000875af1158015611222573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261124a9190810190612c34565b50602854604051634b96303160e11b81526000805160206137d18339815191529163972c60629161128b9185916001600160a01b0390911690600401613158565b6000604051808303816000875af11580156112aa573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526112d29190810190612c34565b50602954604051634b96303160e11b81526000805160206137d18339815191529163972c6062916113139185916001600160a01b03909116906004016131b2565b6000604051808303816000875af1158015611332573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261135a9190810190612c34565b50603b54604051634b96303160e11b81526000805160206137d18339815191529163972c60629161139b9185916001600160a01b0390911690600401613213565b6000604051808303816000875af11580156113ba573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526113e29190810190612c34565b506040516388da6d3560e01b81526000906000805160206137d1833981519152906388da6d35906114199085908790600401613264565b6000604051808303816000875af1158015611438573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526114609190810190612c34565b604080518082018252600a815269706172616d657465727360b01b6020820152603c549151634b96303160e11b8152929350916000805160206137d18339815191529163972c6062916114c39185916001600160a01b03909116906004016132ae565b6000604051808303816000875af11580156114e2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261150a9190810190612c34565b50603d54604051634b96303160e11b81526000805160206137d18339815191529163972c60629161154b9185916001600160a01b0390911690600401613308565b6000604051808303816000875af115801561156a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526115929190810190612c34565b50603e54604051634b96303160e11b81526000805160206137d18339815191529163972c6062916115d39185916001600160a01b039091169060040161334c565b6000604051808303816000875af11580156115f2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261161a9190810190612c34565b50603f54604051634b96303160e11b81526000805160206137d18339815191529163972c60629161165b9185916001600160a01b039091169060040161338f565b6000604051808303816000875af115801561167a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526116a29190810190612c34565b50604080549051634b96303160e11b81526000805160206137d18339815191529163972c6062916116e39185916001600160a01b03909116906004016133cf565b6000604051808303816000875af1158015611702573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261172a9190810190612c34565b50603d54604051634b96303160e11b81526000916000805160206137d18339815191529163972c60629161176c9186916001600160a01b031690600401613308565b6000604051808303816000875af115801561178b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117b39190810190612c34565b6040805180820182526009815268636861696e496e666f60b81b6020820152905163094f480160e11b8152919250906000805160206137d18339815191529063129e900290611808908490439060040161341b565b6000604051808303816000875af1158015611827573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261184f9190810190612c34565b5060405163094f480160e11b81526000906000805160206137d18339815191529063129e9002906118869085904690600401613466565b6000604051808303816000875af11580156118a5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526118cd9190810190612c34565b6040516388da6d3560e01b81529091506000805160206137d1833981519152906388da6d3590611905908c908a908a906004016134a9565b6000604051808303816000875af1158015611924573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261194c9190810190612c34565b506040516388da6d3560e01b81526000805160206137d1833981519152906388da6d3590611982908c90869086906004016134a9565b6000604051808303816000875af11580156119a1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119c99190810190612c34565b506040516388da6d3560e01b81526000906000805160206137d1833981519152906388da6d3590611a02908d90899089906004016134a9565b6000604051808303816000875af1158015611a21573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611a499190810190612c34565b60405163e23cd19f60e01b81529091506000805160206137d18339815191529063e23cd19f90611a7f9084908f906004016134e2565b600060405180830381600087803b158015611a9957600080fd5b505af1158015611aad573d6000803e3d6000fd5b505050505050505050505050505050565b603a81815481106106f657600080fd5b7f0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b604051611b539060208082526038908201527f3d3d3d3d2050617273656420496e6974696c697a6520506172616d7320666f7260408201527f20496e697469616c204465706c6f796d656e74203d3d3d3d0000000000000000606082015260800190565b60405180910390a1603c546040516000805160206137f183398151915291611b86916001600160a01b0390911690613507565b60405180910390a1603d546040516000805160206137f183398151915291611bb9916001600160a01b0390911690613551565b60405180910390a1603e546040516000805160206137f183398151915291611bec916001600160a01b0390911690613583565b60405180910390a1603f546040516000805160206137f183398151915291611c1f916001600160a01b03909116906135b4565b60405180910390a1600080516020613811833981519152604554604051611c8c919060408082526023908201527f53545241544547595f4d414e414745525f494e49545f5041555345445f53544160608201526254555360e81b6080820152602081019190915260a00190565b60405180910390a160465460408051818152601c818301527f53545241544547595f4d414e414745525f57484954454c49535445520000000060608201526001600160a01b039092166020830152516000805160206137f18339815191529181900360800190a1600080516020613811833981519152604854604051611d6391906040808252602e908201527f44454c45474154494f4e5f4d414e414745525f4d494e5f57495448445241574160608201526d4c5f44454c41595f424c4f434b5360901b6080820152602081019190915260a00190565b60405180910390a1600080516020613811833981519152604754604051611dd2919060408082526025908201527f44454c45474154494f4e5f4d414e414745525f494e49545f5041555345445f53606082015264544154555360d81b6080820152602081019190915260a00190565b60405180910390a1604a546040805181815260208183018190527f4156535f4449524543544f52595f494e49545f5041555345445f5354415455536060830152810192909252516000805160206138118339815191529181900360800190a1600080516020613811833981519152604b54604051611e99919060408082526026908201527f524557415244535f434f4f5244494e41544f525f494e49545f5041555345445f60608201526553544154555360d01b6080820152602081019190915260a00190565b60405180910390a1600080516020613811833981519152604f54604051611f06919060408082526023908201527f454947454e504f445f4d414e414745525f494e49545f5041555345445f53544160608201526254555360e81b6080820152602081019190915260a00190565b60405180910390a16051546040805181815260158183015274454947454e504f445f47454e455349535f54494d4560581b60608201526801000000000000000090920467ffffffffffffffff16602083015251600080516020613811833981519152916080908290030190a16052546040805181815260148183015273455448504f534465706f7369744164647265737360601b60608201526001600160a01b039092166020830152516000805160206137f18339815191529181900360800190a17f0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b604051612027906020808252601e908201527f3d3d3d3d205374726174656769657320746f204465706c6f79203d3d3d3d0000604082015260600190565b60405180910390a160005b60435481101561231e5760006044828154811061205157612051612b66565b6000918252602091829020604080516060810190915260039092020180546001600160a01b03168252600181018054929391929184019161209190612b2c565b80601f01602080910402602001604051908101604052809291908181526020018280546120bd90612b2c565b801561210a5780601f106120df5761010080835404028352916020019161210a565b820191906000526020600020905b8154815290600101906020018083116120ed57829003601f168201915b5050505050815260200160028201805461212390612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461214f90612b2c565b801561219c5780601f106121715761010080835404028352916020019161219c565b820191906000526020600020905b81548152906001019060200180831161217f57829003601f168201915b50505091909252505060448054600181018255600091909152825160039091027f9b22d3d61959b4d3528b1d8ba932c96fbe302b36a1aad1d95cab54f9e0a135ea810180546001600160a01b039093166001600160a01b0319909316929092178255602084015193945084939192507f9b22d3d61959b4d3528b1d8ba932c96fbe302b36a1aad1d95cab54f9e0a135eb01906122389082613631565b506040820151600282019061224d9082613631565b5050815160408051818152600d818301526c544f4b454e204144445245535360981b60608201526001600160a01b039092166020830152516000805160206137f183398151915292509081900360800190a17f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358381602001516040516122d291906136f0565b60405180910390a17f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583816040015160405161230d919061372b565b60405180910390a150600101612032565b50565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156124075760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156123ef57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116123b15790505b50505050508152505081526020019060010190612345565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020016000905b8282101561240757838290600052602060002001805461245390612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461247f90612b2c565b80156124cc5780601f106124a1576101008083540402835291602001916124cc565b820191906000526020600020905b8154815290600101906020018083116124af57829003601f168201915b505050505081526020019060010190612434565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156124075760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156125ae57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116125705790505b50505050508152505081526020019060010190612504565b60606010805480602002602001604051908101604052809291908181526020016000905b8282101561240757838290600052602060002001805461260990612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461263590612b2c565b80156126825780601f1061265757610100808354040283529160200191612682565b820191906000526020600020905b81548152906001019060200180831161266557829003601f168201915b5050505050815260200190600101906125ea565b60008054610100900460ff16156126b65750600054610100900460ff1690565b60006000805160206137d18339815191523b156127b057604080516000805160206137d1833981519152602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091612738917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001613761565b60408051601f198184030181529082905261275291613792565b6000604051808303816000865af19150503d806000811461278f576040519150601f19603f3d011682016040523d82523d6000602084013e612794565b606091505b50915050808060200190518101906127ac91906137ae565b9150505b919050565b603781815481106106f657600080fd5b6060600c80548060200260200160405190810160405280929190818152602001828054801561067c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161065e575050505050905090565b604281815481106106f657600080fd5b602080825282518282018190526000918401906040840190835b818110156128765783516001600160a01b031683526020938401939092019160010161284f565b509095945050505050565b60006020828403121561289357600080fd5b5035919050565b60005b838110156128b557818101518382015260200161289d565b50506000910152565b600081518084526128d681602086016020860161289a565b601f01601f19169290920160200192915050565b6001600160a01b038416815260606020820181905260009061290e908301856128be565b828103604084015261292081856128be565b9695505050505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156129695761296961292a565b604052919050565b600067ffffffffffffffff82111561298b5761298b61292a565b50601f01601f191660200190565b6000602082840312156129ab57600080fd5b813567ffffffffffffffff8111156129c257600080fd5b8201601f810184136129d357600080fd5b80356129e66129e182612971565b612940565b8181528560208385010111156129fb57600080fd5b81602084016020830137600091810160200191909152949350505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612ac757868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b80831015612aaf5783516001600160e01b03191682526020938401936001939093019290910190612a83565b50965050506020938401939190910190600101612a41565b50929695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612ac757603f19878603018452612b178583516128be565b94506020938401939190910190600101612afb565b600181811c90821680612b4057607f821691505b602082108103612b6057634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052603260045260246000fd5b606081526000612b8f60608301866128be565b828103602084015260008554612ba481612b2c565b808452600182168015612bbe5760018114612bda57612c11565b60ff1983166020860152602082151560051b8601019350612c11565b88600052602060002060005b83811015612c0857815460208289010152600182019150602081019050612be6565b86016020019450505b5050506001600160a01b03851660408501529150612c2c9050565b949350505050565b600060208284031215612c4657600080fd5b815167ffffffffffffffff811115612c5d57600080fd5b8201601f81018413612c6e57600080fd5b8051612c7c6129e182612971565b818152856020838501011115612c9157600080fd5b612ca282602083016020860161289a565b95945050505050565b81810381811115612ccc57634e487b7160e01b600052601160045260246000fd5b92915050565b606081526000612ce560608301856128be565b828103602080850191909152601482527332b4b3b2b72630bcb2b9283937bc3ca0b236b4b760611b908201526001600160a01b03939093166040928301525001919050565b606081526000612d3d60608301856128be565b8281036020808501919091526013825272656967656e4c6179657250617573657252656760681b908201526001600160a01b03939093166040928301525001919050565b606081526000612d9460608301856128be565b828103602080850191909152600c82526b6176734469726563746f727960a01b908201526001600160a01b03939093166040928301525001919050565b606081526000612de460608301856128be565b828103602080850191909152601a82527f6176734469726563746f7279496d706c656d656e746174696f6e000000000000908201526001600160a01b03939093166040928301525001919050565b606081526000612e4560608301856128be565b82810360208085019190915260118252703232b632b3b0ba34b7b726b0b730b3b2b960791b908201526001600160a01b03939093166040928301525001919050565b606081526000612e9a60608301856128be565b828103602080850191909152601f82527f64656c65676174696f6e4d616e61676572496d706c656d656e746174696f6e00908201526001600160a01b03939093166040928301525001919050565b606081526000612efb60608301856128be565b828103602080850191909152600f82526e39ba3930ba32b3bca6b0b730b3b2b960891b908201526001600160a01b03939093166040928301525001919050565b606081526000612f4e60608301856128be565b828103602080850191909152601d82527f73747261746567794d616e61676572496d706c656d656e746174696f6e000000908201526001600160a01b03939093166040928301525001919050565b606081526000612faf60608301856128be565b82810360208085019190915260128252713932bbb0b93239a1b7b7b93234b730ba37b960711b908201526001600160a01b03939093166040928301525001919050565b60608152600061300560608301856128be565b8281036020808501919091528082527f72657761726473436f6f7264696e61746f72496d706c656d656e746174696f6e908201526001600160a01b03939093166040928301525001919050565b60608152600061306560608301856128be565b828103602080850191909152600f82526e32b4b3b2b72837b226b0b730b3b2b960891b908201526001600160a01b03939093166040928301525001919050565b6060815260006130b860608301856128be565b828103602080850191909152601d82527f656967656e506f644d616e61676572496d706c656d656e746174696f6e000000908201526001600160a01b03939093166040928301525001919050565b60608152600061311960608301856128be565b828103602080850191909152600e82526d32b4b3b2b72837b22132b0b1b7b760911b908201526001600160a01b03939093166040928301525001919050565b60608152600061316b60608301856128be565b828103602080850191909152601682527532b4b3b2b72837b224b6b83632b6b2b73a30ba34b7b760511b908201526001600160a01b03939093166040928301525001919050565b6060815260006131c560608301856128be565b828103602080850191909152601a82527f626173655374726174656779496d706c656d656e746174696f6e000000000000908201526001600160a01b03939093166040928301525001919050565b60608152600061322660608301856128be565b828103602080850191909152600d82526c195b5c1d1e50dbdb9d1c9858dd609a1b908201526001600160a01b03939093166040928301525001919050565b60608152600061327760608301856128be565b828103806020850152600a8252697374726174656769657360b01b602083015260408101604085015250612ca260408201856128be565b6060815260006132c160608301856128be565b82810360208401526132f081601081526f6578656375746f724d756c746973696760801b602082015260400190565b91505060018060a01b03831660408301529392505050565b60608152600061331b60608301856128be565b82810360208401526132f08160128152716f7065726174696f6e734d756c746973696760701b602082015260400190565b60608152600061335f60608301856128be565b82810360208401526132f0816011815270636f6d6d756e6974794d756c746973696760781b602082015260400190565b6060815260006133a260608301856128be565b82810360208401526132f081600e81526d7061757365724d756c746973696760901b602082015260400190565b6060815260006133e260608301856128be565b828103602080850191909152600882526774696d656c6f636b60c01b908201526001600160a01b03939093166040928301525001919050565b60608152600061342e60608301856128be565b8281036020840152600f81526e6465706c6f796d656e74426c6f636b60881b6020820152604081019150508260408301529392505050565b60608152600061347960608301856128be565b8281036020840152600781526618da185a5b925960ca1b6020820152604081019150508260408301529392505050565b6060815260006134bc60608301866128be565b82810360208401526134ce81866128be565b9050828103604084015261292081856128be565b6040815260006134f560408301856128be565b8281036020840152612ca281856128be565b60408152600061353760408301601081526f6578656375746f724d756c746973696760801b602082015260400190565b6001600160a01b0393909316602092909201919091525090565b6040815260006135376040830160128152716f7065726174696f6e734d756c746973696760701b602082015260400190565b604081526000613537604083016011815270636f6d6d756e6974794d756c746973696760781b602082015260400190565b60408152600061353760408301600e81526d7061757365724d756c746973696760901b602082015260400190565b601f82111561362c57806000526020600020601f840160051c810160208510156136095750805b601f840160051c820191505b818110156136295760008155600101613615565b50505b505050565b815167ffffffffffffffff81111561364b5761364b61292a565b61365f816136598454612b2c565b846135e2565b6020601f821160018114613693576000831561367b5750848201515b600019600385901b1c1916600184901b178455613629565b600084815260208120601f198516915b828110156136c357878501518255602094850194600190920191016136a3565b50848210156136e15786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b60408152600a604082015269544f4b454e204e414d4560b01b606082015260806020820152600061372460808301846128be565b9392505050565b60408152600c60408201526b1513d2d1538814d6535093d360a21b606082015260806020820152600061372460808301846128be565b6001600160e01b031983168152815160009061378481600485016020870161289a565b919091016004019392505050565b600082516137a481846020870161289a565b9190910192915050565b6000602082840312156137c057600080fd5b8151801515811461372457600080fdfe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8a26469706673582212205f7a045e7408c65246a4ec2466cfd3b740cf9258bb8a6be8d980128c7d81a02364736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x82\x16\x83\x17\x90U`\x1B\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15`6W`\0\x80\xFD[Pa8f\x80a\0F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x9FW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\x01gW\x80c\xDBM\xF7a\x11a\0\xCEW\x80c\xF2\xEB\xB0\xB6\x11a\0\x87W\x80c\xF2\xEB\xB0\xB6\x14a\x05\xBEW\x80c\xF3\x9E\x91`\x14a\x05\xD1W\x80c\xF7\xE7n6\x14a\x05\xE4W\x80c\xF8\xCC\xBFG\x14a\x05\xF7W\x80c\xFAv&\xD4\x14a\x06\x04W\x80c\xFD\xC3q\xCE\x14a\x06\x11W`\0\x80\xFD[\x80c\xDBM\xF7a\x14a\x05WW\x80c\xE2\x0C\x9Fq\x14a\x05jW\x80c\xE3\xA8\xB3E\x14a\x05rW\x80c\xE7\xACU\xFC\x14a\x05\x85W\x80c\xEAM<\x9B\x14a\x05\x98W\x80c\xF0\x06-\x9A\x14a\x05\xABW`\0\x80\xFD[\x80c\xBAAO\xA6\x11a\x01 W\x80c\xBAAO\xA6\x14a\x04\xDBW\x80c\xBA\x8Ce\xD8\x14a\x04\xF3W\x80c\xBE[\xB5\xF6\x14a\x05\x06W\x80c\xC1\xDA\xCA\x80\x14a\x05\x19W\x80c\xCA\x8A\xA7\xC7\x14a\x05,W\x80c\xD0\xAF&\xE1\x14a\x05?W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x04}W\x80c\x8A/\xC4\xE3\x14a\x04\x92W\x80c\x91j\x17\xC6\x14a\x04\xA5W\x80c\x99\xC1\xEF+\x14a\x04\xADW\x80c\x9E\xF3W\x10\x14a\x04\xC0W\x80c\xB5P\x8A\xA9\x14a\x04\xD3W`\0\x80\xFD[\x80c?M\xA4\xC6\x11a\x02\x0BW\x80cR1V@\x11a\x01\xC4W\x80cR1V@\x14a\x04\x14W\x80c]\xA8\xB4\xCE\x14a\x04'W\x80cf\xD9\xA9\xA0\x14a\x04/W\x80ck:\xA7.\x14a\x04DW\x80cmB\xC7P\x14a\x04WW\x80cq\xC5l2\x14a\x04jW`\0\x80\xFD[\x80c?M\xA4\xC6\x14a\x03\x9CW\x80c?r\x86\xF4\x14a\x03\xAFW\x80cFe\xBC\xDA\x14a\x03\xB7W\x80cF\xE4\xE1\xBF\x14a\x03\xCAW\x80cG\xC9M\xDA\x14a\x03\xECW\x80cQn((\x14a\x03\xFFW`\0\x80\xFD[\x80c)+{+\x11a\x02]W\x80c)+{+\x14a\x035W\x80c2\xC0\x85\x85\x14a\x03HW\x80c9\xB7\x0E8\x14a\x03[W\x80c>+\xEE;\x14a\x03nW\x80c>^<#\x14a\x03\x81W\x80c?H?\xFA\x14a\x03\x89W`\0\x80\xFD[\x80b\x91\x9A\xFE\x14a\x02\xA4W\x80c\x04\x92\xF4\xBC\x14a\x02\xD4W\x80c\x1E-3K\x14a\x02\xE7W\x80c\x1E\xD7\x83\x1C\x14a\x02\xFAW\x80c!\xCB>7\x14a\x03\x0FW\x80c&\x89cc\x14a\x03\"W[`\0\x80\xFD[`/Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`2Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`+Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a\x06$V[`@Qa\x02\xCB\x91\x90a(5V[`6Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`4Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`'Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`-Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`!Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1ETa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a\x06\x86V[a\x02\xB7a\x03\x976`\x04a(\x81V[a\x06\xE6V[`3Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a\x07\x10V[`%Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xDDa\x03\xD86`\x04a(\x81V[a\x07pV[`@Qa\x02\xCB\x93\x92\x91\x90a(\xEAV[a\x02\xB7a\x03\xFA6`\x04a(\x81V[a\x08\xC0V[a\x04\x12a\x04\r6`\x04a)\x99V[a\x08\xD0V[\0[a\x02\xB7a\x04\"6`\x04a(\x81V[a\x1A\xBEV[a\x04\x12a\x1A\xCEV[a\x047a#!V[`@Qa\x02\xCB\x91\x90a*\x19V[`\x1DTa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1CTa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`$Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x85a$\x10V[`@Qa\x02\xCB\x91\x90a*\xD3V[`#Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x047a$\xE0V[`)Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`*Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x85a%\xC6V[a\x04\xE3a&\x96V[`@Q\x90\x15\x15\x81R` \x01a\x02\xCBV[a\x02\xB7a\x05\x016`\x04a(\x81V[a'\xB5V[` Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\"Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`,Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1BTa\x02\xB7\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`5Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a'\xC5V[`;Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xB7a\x05\x936`\x04a(\x81V[a(%V[`\x1FTa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`.Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`0Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`&Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`(Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1BTa\x04\xE3\x90`\xFF\x16\x81V[`\0Ta\x04\xE3\x90`\xFF\x16\x81V[`1Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^WPPPPP\x90P\x90V[`8\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^WPPPPP\x90P\x90V[`D\x81\x81T\x81\x10a\x07\x80W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90a\x07\xAF\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xDB\x90a+,V[\x80\x15a\x08(W\x80`\x1F\x10a\x07\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x08=\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08i\x90a+,V[\x80\x15a\x08\xB6W\x80`\x1F\x10a\x08\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xB6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`9\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[`@\x80Q\x80\x82\x01\x82R`\r\x81Rl\x1C\x18\\\x99[\x9D\x08\x1B\xD8\x9A\x99X\xDD`\x9A\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\n\x83Ristrategies`\xB0\x1B\x90\x83\x01R\x90`\0[`CT\x81\x10\x15a\n\x15W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x97,`b\x83`D\x84\x81T\x81\x10a\tiWa\tia+fV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x02\x01`B\x85\x81T\x81\x10a\t\x8DWa\t\x8Da+fV[`\0\x91\x82R` \x90\x91 \x01T`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra\t\xC5\x93\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a+|V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x0C\x91\x90\x81\x01\x90a,4V[P`\x01\x01a\t\x19V[P`\0`CT`\0\x14a\x0B,W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x97,`b\x83`D`\x01`CTa\nf\x91\x90a,\xABV[\x81T\x81\x10a\nvWa\nva+fV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x02\x01`B`\x01`CTa\n\x98\x91\x90a,\xABV[\x81T\x81\x10a\n\xA8Wa\n\xA8a+fV[`\0\x91\x82R` \x90\x91 \x01T`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra\n\xE0\x93\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a+|V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B'\x91\x90\x81\x01\x90a,4V[a\x0B=V[`@Q\x80` \x01`@R\x80`\0\x81RP[`@\x80Q\x80\x82\x01\x82R`\t\x81Rhaddresses`\xB8\x1B` \x82\x01R`\x1BT\x91QcK\x9601`\xE1\x1B\x81R\x92\x93P\x91`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0B\xA3\x91\x85\x91a\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a,\xD2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\xEA\x91\x90\x81\x01\x90a,4V[P`\x1CT`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0C+\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a-*V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0CJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Cr\x91\x90\x81\x01\x90a,4V[P`\x1DT`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0C\xB3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a-\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xFA\x91\x90\x81\x01\x90a,4V[P`\x1ET`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\r;\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a-\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\rZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x82\x91\x90\x81\x01\x90a,4V[P`\x1FT`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\r\xC3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a.2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\n\x91\x90\x81\x01\x90a,4V[P` T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0EK\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a.\x87V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0EjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x92\x91\x90\x81\x01\x90a,4V[P`!T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0E\xD3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a.\xE8V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x1A\x91\x90\x81\x01\x90a,4V[P`\"T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0F[\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a/;V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0FzW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\xA2\x91\x90\x81\x01\x90a,4V[P`#T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0F\xE3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a/\x9CV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10*\x91\x90\x81\x01\x90a,4V[P`$T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x10k\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a/\xF2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\xB2\x91\x90\x81\x01\x90a,4V[P`%T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x10\xF3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a0RV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11:\x91\x90\x81\x01\x90a,4V[P`&T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x11{\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a0\xA5V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xC2\x91\x90\x81\x01\x90a,4V[P`'T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x12\x03\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a1\x06V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12J\x91\x90\x81\x01\x90a,4V[P`(T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x12\x8B\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a1XV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xD2\x91\x90\x81\x01\x90a,4V[P`)T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x13\x13\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a1\xB2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x132W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13Z\x91\x90\x81\x01\x90a,4V[P`;T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x13\x9B\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a2\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xE2\x91\x90\x81\x01\x90a,4V[P`@Qc\x88\xDAm5`\xE0\x1B\x81R`\0\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x14\x19\x90\x85\x90\x87\x90`\x04\x01a2dV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x148W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14`\x91\x90\x81\x01\x90a,4V[`@\x80Q\x80\x82\x01\x82R`\n\x81Riparameters`\xB0\x1B` \x82\x01R`<T\x91QcK\x9601`\xE1\x1B\x81R\x92\x93P\x91`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x14\xC3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a2\xAEV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\n\x91\x90\x81\x01\x90a,4V[P`=T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x15K\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\x92\x91\x90\x81\x01\x90a,4V[P`>T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x15\xD3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3LV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\x1A\x91\x90\x81\x01\x90a,4V[P`?T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x16[\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\xA2\x91\x90\x81\x01\x90a,4V[P`@\x80T\x90QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x16\xE3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17*\x91\x90\x81\x01\x90a,4V[P`=T`@QcK\x9601`\xE1\x1B\x81R`\0\x91`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x17l\x91\x86\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a3\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xB3\x91\x90\x81\x01\x90a,4V[`@\x80Q\x80\x82\x01\x82R`\t\x81RhchainInfo`\xB8\x1B` \x82\x01R\x90Qc\tOH\x01`\xE1\x1B\x81R\x91\x92P\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x12\x9E\x90\x02\x90a\x18\x08\x90\x84\x90C\x90`\x04\x01a4\x1BV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x18'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18O\x91\x90\x81\x01\x90a,4V[P`@Qc\tOH\x01`\xE1\x1B\x81R`\0\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x12\x9E\x90\x02\x90a\x18\x86\x90\x85\x90F\x90`\x04\x01a4fV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x18\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\xCD\x91\x90\x81\x01\x90a,4V[`@Qc\x88\xDAm5`\xE0\x1B\x81R\x90\x91P`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x19\x05\x90\x8C\x90\x8A\x90\x8A\x90`\x04\x01a4\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19L\x91\x90\x81\x01\x90a,4V[P`@Qc\x88\xDAm5`\xE0\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x19\x82\x90\x8C\x90\x86\x90\x86\x90`\x04\x01a4\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\xC9\x91\x90\x81\x01\x90a,4V[P`@Qc\x88\xDAm5`\xE0\x1B\x81R`\0\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x1A\x02\x90\x8D\x90\x89\x90\x89\x90`\x04\x01a4\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1A!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1AI\x91\x90\x81\x01\x90a,4V[`@Qc\xE2<\xD1\x9F`\xE0\x1B\x81R\x90\x91P`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\xE2<\xD1\x9F\x90a\x1A\x7F\x90\x84\x90\x8F\x90`\x04\x01a4\xE2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xADW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPV[`:\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[\x7F\x0B.\x13\xFF \xAC{GA\x98eU\x83\xED\xF7\r\xED\xD2\xC1\xDC\x98\x0E2\x9CO\xBB/\xC0t\x8Byk`@Qa\x1BS\x90` \x80\x82R`8\x90\x82\x01R\x7F==== Parsed Initilize Params for`@\x82\x01R\x7F Initial Deployment ====\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`<T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1B\x86\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5\x07V[`@Q\x80\x91\x03\x90\xA1`=T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1B\xB9\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5QV[`@Q\x80\x91\x03\x90\xA1`>T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1B\xEC\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5\x83V[`@Q\x80\x91\x03\x90\xA1`?T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1C\x1F\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5\xB4V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`ET`@Qa\x1C\x8C\x91\x90`@\x80\x82R`#\x90\x82\x01R\x7FSTRATEGY_MANAGER_INIT_PAUSED_STA``\x82\x01RbTUS`\xE8\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`FT`@\x80Q\x81\x81R`\x1C\x81\x83\x01R\x7FSTRATEGY_MANAGER_WHITELISTER\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`HT`@Qa\x1Dc\x91\x90`@\x80\x82R`.\x90\x82\x01R\x7FDELEGATION_MANAGER_MIN_WITHDRAWA``\x82\x01RmL_DELAY_BLOCKS`\x90\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`GT`@Qa\x1D\xD2\x91\x90`@\x80\x82R`%\x90\x82\x01R\x7FDELEGATION_MANAGER_INIT_PAUSED_S``\x82\x01RdTATUS`\xD8\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`JT`@\x80Q\x81\x81R` \x81\x83\x01\x81\x90R\x7FAVS_DIRECTORY_INIT_PAUSED_STATUS``\x83\x01R\x81\x01\x92\x90\x92RQ`\0\x80Q` a8\x11\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`KT`@Qa\x1E\x99\x91\x90`@\x80\x82R`&\x90\x82\x01R\x7FREWARDS_COORDINATOR_INIT_PAUSED_``\x82\x01ReSTATUS`\xD0\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`OT`@Qa\x1F\x06\x91\x90`@\x80\x82R`#\x90\x82\x01R\x7FEIGENPOD_MANAGER_INIT_PAUSED_STA``\x82\x01RbTUS`\xE8\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`QT`@\x80Q\x81\x81R`\x15\x81\x83\x01RtEIGENPOD_GENESIS_TIME`X\x1B``\x82\x01Rh\x01\0\0\0\0\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01RQ`\0\x80Q` a8\x11\x839\x81Q\x91R\x91`\x80\x90\x82\x90\x03\x01\x90\xA1`RT`@\x80Q\x81\x81R`\x14\x81\x83\x01RsETHPOSDepositAddress``\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1\x7F\x0B.\x13\xFF \xAC{GA\x98eU\x83\xED\xF7\r\xED\xD2\xC1\xDC\x98\x0E2\x9CO\xBB/\xC0t\x8Byk`@Qa '\x90` \x80\x82R`\x1E\x90\x82\x01R\x7F==== Strategies to Deploy ====\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0[`CT\x81\x10\x15a#\x1EW`\0`D\x82\x81T\x81\x10a QWa Qa+fV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x90\x91R`\x03\x90\x92\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a \x91\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \xBD\x90a+,V[\x80\x15a!\nW\x80`\x1F\x10a \xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta!#\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!O\x90a+,V[\x80\x15a!\x9CW\x80`\x1F\x10a!qWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`D\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x82Q`\x03\x90\x91\x02\x7F\x9B\"\xD3\xD6\x19Y\xB4\xD3R\x8B\x1D\x8B\xA92\xC9o\xBE0+6\xA1\xAA\xD1\xD9\\\xABT\xF9\xE0\xA15\xEA\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x82U` \x84\x01Q\x93\x94P\x84\x93\x91\x92P\x7F\x9B\"\xD3\xD6\x19Y\xB4\xD3R\x8B\x1D\x8B\xA92\xC9o\xBE0+6\xA1\xAA\xD1\xD9\\\xABT\xF9\xE0\xA15\xEB\x01\x90a\"8\x90\x82a61V[P`@\x82\x01Q`\x02\x82\x01\x90a\"M\x90\x82a61V[PP\x81Q`@\x80Q\x81\x81R`\r\x81\x83\x01RlTOKEN ADDRESS`\x98\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ`\0\x80Q` a7\xF1\x839\x81Q\x91R\x92P\x90\x81\x90\x03`\x80\x01\x90\xA1\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x81` \x01Q`@Qa\"\xD2\x91\x90a6\xF0V[`@Q\x80\x91\x03\x90\xA1\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x81`@\x01Q`@Qa#\r\x91\x90a7+V[`@Q\x80\x91\x03\x90\xA1P`\x01\x01a 2V[PV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a#\xEFW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a#\xB1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a#EV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W\x83\x82\x90`\0R` `\0 \x01\x80Ta$S\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$\x7F\x90a+,V[\x80\x15a$\xCCW\x80`\x1F\x10a$\xA1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xCCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\xAFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a$4V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a%\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a%pW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\x04V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W\x83\x82\x90`\0R` `\0 \x01\x80Ta&\t\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&5\x90a+,V[\x80\x15a&\x82W\x80`\x1F\x10a&WWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&\x82V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&eW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a%\xEAV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a&\xB6WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a7\xD1\x839\x81Q\x91R;\x15a'\xB0W`@\x80Q`\0\x80Q` a7\xD1\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a'8\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a7aV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'R\x91a7\x92V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a'\x8FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\x94V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a'\xAC\x91\x90a7\xAEV[\x91PP[\x91\x90PV[`7\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^WPPPPP\x90P\x90V[`B\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(vW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(OV[P\x90\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a(\x93W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a(\xB5W\x81\x81\x01Q\x83\x82\x01R` \x01a(\x9DV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra(\xD6\x81` \x86\x01` \x86\x01a(\x9AV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a)\x0E\x90\x83\x01\x85a(\xBEV[\x82\x81\x03`@\x84\x01Ra) \x81\x85a(\xBEV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)iWa)ia)*V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\x8BWa)\x8Ba)*V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a)\xABW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xC2W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\xD3W`\0\x80\xFD[\x805a)\xE6a)\xE1\x82a)qV[a)@V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\xFBW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*\xC7W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15a*\xAFW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a*\x83V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*AV[P\x92\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*\xC7W`?\x19\x87\x86\x03\x01\x84Ra+\x17\x85\x83Qa(\xBEV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*\xFBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a+@W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+`WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[``\x81R`\0a+\x8F``\x83\x01\x86a(\xBEV[\x82\x81\x03` \x84\x01R`\0\x85Ta+\xA4\x81a+,V[\x80\x84R`\x01\x82\x16\x80\x15a+\xBEW`\x01\x81\x14a+\xDAWa,\x11V[`\xFF\x19\x83\x16` \x86\x01R` \x82\x15\x15`\x05\x1B\x86\x01\x01\x93Pa,\x11V[\x88`\0R` `\0 `\0[\x83\x81\x10\x15a,\x08W\x81T` \x82\x89\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa+\xE6V[\x86\x01` \x01\x94PP[PPP`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x85\x01R\x91Pa,,\x90PV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a,FW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,]W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a,nW`\0\x80\xFD[\x80Qa,|a)\xE1\x82a)qV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a,\x91W`\0\x80\xFD[a,\xA2\x82` \x83\x01` \x86\x01a(\x9AV[\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a,\xCCWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[``\x81R`\0a,\xE5``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x14\x82Rs2\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9(97\xBC<\xA0\xB26\xB4\xB7`a\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a-=``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x13\x82RreigenLayerPauserReg`h\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a-\x94``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0C\x82RkavsDirectory`\xA0\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a-\xE4``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1A\x82R\x7FavsDirectoryImplementation\0\0\0\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a.E``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x11\x82Rp22\xB62\xB3\xB0\xBA4\xB7\xB7&\xB0\xB70\xB3\xB2\xB9`y\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a.\x9A``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1F\x82R\x7FdelegationManagerImplementation\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a.\xFB``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0F\x82Rn9\xBA90\xBA2\xB3\xBC\xA6\xB0\xB70\xB3\xB2\xB9`\x89\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a/N``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1D\x82R\x7FstrategyManagerImplementation\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a/\xAF``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x12\x82Rq92\xBB\xB0\xB929\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a0\x05``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R\x80\x82R\x7FrewardsCoordinatorImplementation\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a0e``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0F\x82Rn2\xB4\xB3\xB2\xB7(7\xB2&\xB0\xB70\xB3\xB2\xB9`\x89\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a0\xB8``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1D\x82R\x7FeigenPodManagerImplementation\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a1\x19``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0E\x82Rm2\xB4\xB3\xB2\xB7(7\xB2!2\xB0\xB1\xB7\xB7`\x91\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a1k``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x16\x82Ru2\xB4\xB3\xB2\xB7(7\xB2$\xB6\xB862\xB6\xB2\xB7:0\xBA4\xB7\xB7`Q\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a1\xC5``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1A\x82R\x7FbaseStrategyImplementation\0\0\0\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a2&``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\r\x82Rl\x19[\\\x1D\x1EP\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a2w``\x83\x01\x85a(\xBEV[\x82\x81\x03\x80` \x85\x01R`\n\x82Ristrategies`\xB0\x1B` \x83\x01R`@\x81\x01`@\x85\x01RPa,\xA2`@\x82\x01\x85a(\xBEV[``\x81R`\0a2\xC1``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x10\x81RoexecutorMultisig`\x80\x1B` \x82\x01R`@\x01\x90V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x93\x92PPPV[``\x81R`\0a3\x1B``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x12\x81RqoperationsMultisig`p\x1B` \x82\x01R`@\x01\x90V[``\x81R`\0a3_``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x11\x81RpcommunityMultisig`x\x1B` \x82\x01R`@\x01\x90V[``\x81R`\0a3\xA2``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x0E\x81RmpauserMultisig`\x90\x1B` \x82\x01R`@\x01\x90V[``\x81R`\0a3\xE2``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x08\x82Rgtimelock`\xC0\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a4.``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01R`\x0F\x81RndeploymentBlock`\x88\x1B` \x82\x01R`@\x81\x01\x91PP\x82`@\x83\x01R\x93\x92PPPV[``\x81R`\0a4y``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01R`\x07\x81Rf\x18\xDA\x18Z[\x92Y`\xCA\x1B` \x82\x01R`@\x81\x01\x91PP\x82`@\x83\x01R\x93\x92PPPV[``\x81R`\0a4\xBC``\x83\x01\x86a(\xBEV[\x82\x81\x03` \x84\x01Ra4\xCE\x81\x86a(\xBEV[\x90P\x82\x81\x03`@\x84\x01Ra) \x81\x85a(\xBEV[`@\x81R`\0a4\xF5`@\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra,\xA2\x81\x85a(\xBEV[`@\x81R`\0a57`@\x83\x01`\x10\x81RoexecutorMultisig`\x80\x1B` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16` \x92\x90\x92\x01\x91\x90\x91RP\x90V[`@\x81R`\0a57`@\x83\x01`\x12\x81RqoperationsMultisig`p\x1B` \x82\x01R`@\x01\x90V[`@\x81R`\0a57`@\x83\x01`\x11\x81RpcommunityMultisig`x\x1B` \x82\x01R`@\x01\x90V[`@\x81R`\0a57`@\x83\x01`\x0E\x81RmpauserMultisig`\x90\x1B` \x82\x01R`@\x01\x90V[`\x1F\x82\x11\x15a6,W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a6\tWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a6)W`\0\x81U`\x01\x01a6\x15V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6KWa6Ka)*V[a6_\x81a6Y\x84Ta+,V[\x84a5\xE2V[` `\x1F\x82\x11`\x01\x81\x14a6\x93W`\0\x83\x15a6{WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua6)V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a6\xC3W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a6\xA3V[P\x84\x82\x10\x15a6\xE1W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\n`@\x82\x01RiTOKEN NAME`\xB0\x1B``\x82\x01R`\x80` \x82\x01R`\0a7$`\x80\x83\x01\x84a(\xBEV[\x93\x92PPPV[`@\x81R`\x0C`@\x82\x01Rk\x15\x13\xD2\xD1S\x88\x14\xD6SP\x93\xD3`\xA2\x1B``\x82\x01R`\x80` \x82\x01R`\0a7$`\x80\x83\x01\x84a(\xBEV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a7\x84\x81`\x04\x85\x01` \x87\x01a(\x9AV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa7\xA4\x81\x84` \x87\x01a(\x9AV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a7\xC0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a7$W`\0\x80\xFD\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\xA2dipfsX\"\x12 _z\x04^t\x08\xC6RF\xA4\xEC$f\xCF\xD3\xB7@\xCF\x92X\xBB\x8Ak\xE8\xD9\x80\x12\x8C}\x81\xA0#dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061029f5760003560e01c806385226c8111610167578063db4df761116100ce578063f2ebb0b611610087578063f2ebb0b6146105be578063f39e9160146105d1578063f7e76e36146105e4578063f8ccbf47146105f7578063fa7626d414610604578063fdc371ce1461061157600080fd5b8063db4df76114610557578063e20c9f711461056a578063e3a8b34514610572578063e7ac55fc14610585578063ea4d3c9b14610598578063f0062d9a146105ab57600080fd5b8063ba414fa611610120578063ba414fa6146104db578063ba8c65d8146104f3578063be5bb5f614610506578063c1daca8014610519578063ca8aa7c71461052c578063d0af26e11461053f57600080fd5b806385226c811461047d5780638a2fc4e314610492578063916a17c6146104a557806399c1ef2b146104ad5780639ef35710146104c0578063b5508aa9146104d357600080fd5b80633f4da4c61161020b57806352315640116101c457806352315640146104145780635da8b4ce1461042757806366d9a9a01461042f5780636b3aa72e146104445780636d42c7501461045757806371c56c321461046a57600080fd5b80633f4da4c61461039c5780633f7286f4146103af5780634665bcda146103b757806346e4e1bf146103ca57806347c94dda146103ec578063516e2828146103ff57600080fd5b8063292b7b2b1161025d578063292b7b2b1461033557806332c085851461034857806339b70e381461035b5780633e2bee3b1461036e5780633e5e3c23146103815780633f483ffa1461038957600080fd5b8062919afe146102a45780630492f4bc146102d45780631e2d334b146102e75780631ed7831c146102fa57806321cb3e371461030f5780632689636314610322575b600080fd5b602f546102b7906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6032546102b7906001600160a01b031681565b602b546102b7906001600160a01b031681565b610302610624565b6040516102cb9190612835565b6036546102b7906001600160a01b031681565b6034546102b7906001600160a01b031681565b6027546102b7906001600160a01b031681565b602d546102b7906001600160a01b031681565b6021546102b7906001600160a01b031681565b601e546102b7906001600160a01b031681565b610302610686565b6102b7610397366004612881565b6106e6565b6033546102b7906001600160a01b031681565b610302610710565b6025546102b7906001600160a01b031681565b6103dd6103d8366004612881565b610770565b6040516102cb939291906128ea565b6102b76103fa366004612881565b6108c0565b61041261040d366004612999565b6108d0565b005b6102b7610422366004612881565b611abe565b610412611ace565b610437612321565b6040516102cb9190612a19565b601d546102b7906001600160a01b031681565b601c546102b7906001600160a01b031681565b6024546102b7906001600160a01b031681565b610485612410565b6040516102cb9190612ad3565b6023546102b7906001600160a01b031681565b6104376124e0565b6029546102b7906001600160a01b031681565b602a546102b7906001600160a01b031681565b6104856125c6565b6104e3612696565b60405190151581526020016102cb565b6102b7610501366004612881565b6127b5565b6020546102b7906001600160a01b031681565b6022546102b7906001600160a01b031681565b602c546102b7906001600160a01b031681565b601b546102b79061010090046001600160a01b031681565b6035546102b7906001600160a01b031681565b6103026127c5565b603b546102b7906001600160a01b031681565b6102b7610593366004612881565b612825565b601f546102b7906001600160a01b031681565b602e546102b7906001600160a01b031681565b6030546102b7906001600160a01b031681565b6026546102b7906001600160a01b031681565b6028546102b7906001600160a01b031681565b601b546104e39060ff1681565b6000546104e39060ff1681565b6031546102b7906001600160a01b031681565b6060600d80548060200260200160405190810160405280929190818152602001828054801561067c57602002820191906000526020600020905b81546001600160a01b0316815260019091019060200180831161065e575b5050505050905090565b6060600f80548060200260200160405190810160405280929190818152602001828054801561067c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161065e575050505050905090565b603881815481106106f657600080fd5b6000918252602090912001546001600160a01b0316905081565b6060600e80548060200260200160405190810160405280929190818152602001828054801561067c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161065e575050505050905090565b6044818154811061078057600080fd5b6000918252602090912060039091020180546001820180546001600160a01b039092169350906107af90612b2c565b80601f01602080910402602001604051908101604052809291908181526020018280546107db90612b2c565b80156108285780601f106107fd57610100808354040283529160200191610828565b820191906000526020600020905b81548152906001019060200180831161080b57829003601f168201915b50505050509080600201805461083d90612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461086990612b2c565b80156108b65780601f1061088b576101008083540402835291602001916108b6565b820191906000526020600020905b81548152906001019060200180831161089957829003601f168201915b5050505050905083565b603981815481106106f657600080fd5b604080518082018252600d81526c1c185c995b9d081bd89a9958dd609a1b6020808301919091528251808401909352600a8352697374726174656769657360b01b908301529060005b604354811015610a15577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b031663972c6062836044848154811061096957610969612b66565b90600052602060002090600302016002016042858154811061098d5761098d612b66565b6000918252602090912001546040516001600160e01b031960e086901b1681526109c59392916001600160a01b031690600401612b7c565b6000604051808303816000875af11580156109e4573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610a0c9190810190612c34565b50600101610919565b506000604354600014610b2c577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b031663972c60628360446001604354610a669190612cab565b81548110610a7657610a76612b66565b906000526020600020906003020160020160426001604354610a989190612cab565b81548110610aa857610aa8612b66565b6000918252602090912001546040516001600160e01b031960e086901b168152610ae09392916001600160a01b031690600401612b7c565b6000604051808303816000875af1158015610aff573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610b279190810190612c34565b610b3d565b604051806020016040528060008152505b604080518082018252600981526861646472657373657360b81b6020820152601b549151634b96303160e11b8152929350916000805160206137d18339815191529163972c606291610ba39185916101009091046001600160a01b031690600401612cd2565b6000604051808303816000875af1158015610bc2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610bea9190810190612c34565b50601c54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610c2b9185916001600160a01b0390911690600401612d2a565b6000604051808303816000875af1158015610c4a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c729190810190612c34565b50601d54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610cb39185916001600160a01b0390911690600401612d81565b6000604051808303816000875af1158015610cd2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610cfa9190810190612c34565b50601e54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610d3b9185916001600160a01b0390911690600401612dd1565b6000604051808303816000875af1158015610d5a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d829190810190612c34565b50601f54604051634b96303160e11b81526000805160206137d18339815191529163972c606291610dc39185916001600160a01b0390911690600401612e32565b6000604051808303816000875af1158015610de2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e0a9190810190612c34565b50602054604051634b96303160e11b81526000805160206137d18339815191529163972c606291610e4b9185916001600160a01b0390911690600401612e87565b6000604051808303816000875af1158015610e6a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610e929190810190612c34565b50602154604051634b96303160e11b81526000805160206137d18339815191529163972c606291610ed39185916001600160a01b0390911690600401612ee8565b6000604051808303816000875af1158015610ef2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610f1a9190810190612c34565b50602254604051634b96303160e11b81526000805160206137d18339815191529163972c606291610f5b9185916001600160a01b0390911690600401612f3b565b6000604051808303816000875af1158015610f7a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610fa29190810190612c34565b50602354604051634b96303160e11b81526000805160206137d18339815191529163972c606291610fe39185916001600160a01b0390911690600401612f9c565b6000604051808303816000875af1158015611002573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261102a9190810190612c34565b50602454604051634b96303160e11b81526000805160206137d18339815191529163972c60629161106b9185916001600160a01b0390911690600401612ff2565b6000604051808303816000875af115801561108a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110b29190810190612c34565b50602554604051634b96303160e11b81526000805160206137d18339815191529163972c6062916110f39185916001600160a01b0390911690600401613052565b6000604051808303816000875af1158015611112573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261113a9190810190612c34565b50602654604051634b96303160e11b81526000805160206137d18339815191529163972c60629161117b9185916001600160a01b03909116906004016130a5565b6000604051808303816000875af115801561119a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111c29190810190612c34565b50602754604051634b96303160e11b81526000805160206137d18339815191529163972c6062916112039185916001600160a01b0390911690600401613106565b6000604051808303816000875af1158015611222573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261124a9190810190612c34565b50602854604051634b96303160e11b81526000805160206137d18339815191529163972c60629161128b9185916001600160a01b0390911690600401613158565b6000604051808303816000875af11580156112aa573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526112d29190810190612c34565b50602954604051634b96303160e11b81526000805160206137d18339815191529163972c6062916113139185916001600160a01b03909116906004016131b2565b6000604051808303816000875af1158015611332573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261135a9190810190612c34565b50603b54604051634b96303160e11b81526000805160206137d18339815191529163972c60629161139b9185916001600160a01b0390911690600401613213565b6000604051808303816000875af11580156113ba573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526113e29190810190612c34565b506040516388da6d3560e01b81526000906000805160206137d1833981519152906388da6d35906114199085908790600401613264565b6000604051808303816000875af1158015611438573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526114609190810190612c34565b604080518082018252600a815269706172616d657465727360b01b6020820152603c549151634b96303160e11b8152929350916000805160206137d18339815191529163972c6062916114c39185916001600160a01b03909116906004016132ae565b6000604051808303816000875af11580156114e2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261150a9190810190612c34565b50603d54604051634b96303160e11b81526000805160206137d18339815191529163972c60629161154b9185916001600160a01b0390911690600401613308565b6000604051808303816000875af115801561156a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526115929190810190612c34565b50603e54604051634b96303160e11b81526000805160206137d18339815191529163972c6062916115d39185916001600160a01b039091169060040161334c565b6000604051808303816000875af11580156115f2573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261161a9190810190612c34565b50603f54604051634b96303160e11b81526000805160206137d18339815191529163972c60629161165b9185916001600160a01b039091169060040161338f565b6000604051808303816000875af115801561167a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526116a29190810190612c34565b50604080549051634b96303160e11b81526000805160206137d18339815191529163972c6062916116e39185916001600160a01b03909116906004016133cf565b6000604051808303816000875af1158015611702573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261172a9190810190612c34565b50603d54604051634b96303160e11b81526000916000805160206137d18339815191529163972c60629161176c9186916001600160a01b031690600401613308565b6000604051808303816000875af115801561178b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117b39190810190612c34565b6040805180820182526009815268636861696e496e666f60b81b6020820152905163094f480160e11b8152919250906000805160206137d18339815191529063129e900290611808908490439060040161341b565b6000604051808303816000875af1158015611827573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261184f9190810190612c34565b5060405163094f480160e11b81526000906000805160206137d18339815191529063129e9002906118869085904690600401613466565b6000604051808303816000875af11580156118a5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526118cd9190810190612c34565b6040516388da6d3560e01b81529091506000805160206137d1833981519152906388da6d3590611905908c908a908a906004016134a9565b6000604051808303816000875af1158015611924573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261194c9190810190612c34565b506040516388da6d3560e01b81526000805160206137d1833981519152906388da6d3590611982908c90869086906004016134a9565b6000604051808303816000875af11580156119a1573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119c99190810190612c34565b506040516388da6d3560e01b81526000906000805160206137d1833981519152906388da6d3590611a02908d90899089906004016134a9565b6000604051808303816000875af1158015611a21573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611a499190810190612c34565b60405163e23cd19f60e01b81529091506000805160206137d18339815191529063e23cd19f90611a7f9084908f906004016134e2565b600060405180830381600087803b158015611a9957600080fd5b505af1158015611aad573d6000803e3d6000fd5b505050505050505050505050505050565b603a81815481106106f657600080fd5b7f0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b604051611b539060208082526038908201527f3d3d3d3d2050617273656420496e6974696c697a6520506172616d7320666f7260408201527f20496e697469616c204465706c6f796d656e74203d3d3d3d0000000000000000606082015260800190565b60405180910390a1603c546040516000805160206137f183398151915291611b86916001600160a01b0390911690613507565b60405180910390a1603d546040516000805160206137f183398151915291611bb9916001600160a01b0390911690613551565b60405180910390a1603e546040516000805160206137f183398151915291611bec916001600160a01b0390911690613583565b60405180910390a1603f546040516000805160206137f183398151915291611c1f916001600160a01b03909116906135b4565b60405180910390a1600080516020613811833981519152604554604051611c8c919060408082526023908201527f53545241544547595f4d414e414745525f494e49545f5041555345445f53544160608201526254555360e81b6080820152602081019190915260a00190565b60405180910390a160465460408051818152601c818301527f53545241544547595f4d414e414745525f57484954454c49535445520000000060608201526001600160a01b039092166020830152516000805160206137f18339815191529181900360800190a1600080516020613811833981519152604854604051611d6391906040808252602e908201527f44454c45474154494f4e5f4d414e414745525f4d494e5f57495448445241574160608201526d4c5f44454c41595f424c4f434b5360901b6080820152602081019190915260a00190565b60405180910390a1600080516020613811833981519152604754604051611dd2919060408082526025908201527f44454c45474154494f4e5f4d414e414745525f494e49545f5041555345445f53606082015264544154555360d81b6080820152602081019190915260a00190565b60405180910390a1604a546040805181815260208183018190527f4156535f4449524543544f52595f494e49545f5041555345445f5354415455536060830152810192909252516000805160206138118339815191529181900360800190a1600080516020613811833981519152604b54604051611e99919060408082526026908201527f524557415244535f434f4f5244494e41544f525f494e49545f5041555345445f60608201526553544154555360d01b6080820152602081019190915260a00190565b60405180910390a1600080516020613811833981519152604f54604051611f06919060408082526023908201527f454947454e504f445f4d414e414745525f494e49545f5041555345445f53544160608201526254555360e81b6080820152602081019190915260a00190565b60405180910390a16051546040805181815260158183015274454947454e504f445f47454e455349535f54494d4560581b60608201526801000000000000000090920467ffffffffffffffff16602083015251600080516020613811833981519152916080908290030190a16052546040805181815260148183015273455448504f534465706f7369744164647265737360601b60608201526001600160a01b039092166020830152516000805160206137f18339815191529181900360800190a17f0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b604051612027906020808252601e908201527f3d3d3d3d205374726174656769657320746f204465706c6f79203d3d3d3d0000604082015260600190565b60405180910390a160005b60435481101561231e5760006044828154811061205157612051612b66565b6000918252602091829020604080516060810190915260039092020180546001600160a01b03168252600181018054929391929184019161209190612b2c565b80601f01602080910402602001604051908101604052809291908181526020018280546120bd90612b2c565b801561210a5780601f106120df5761010080835404028352916020019161210a565b820191906000526020600020905b8154815290600101906020018083116120ed57829003601f168201915b5050505050815260200160028201805461212390612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461214f90612b2c565b801561219c5780601f106121715761010080835404028352916020019161219c565b820191906000526020600020905b81548152906001019060200180831161217f57829003601f168201915b50505091909252505060448054600181018255600091909152825160039091027f9b22d3d61959b4d3528b1d8ba932c96fbe302b36a1aad1d95cab54f9e0a135ea810180546001600160a01b039093166001600160a01b0319909316929092178255602084015193945084939192507f9b22d3d61959b4d3528b1d8ba932c96fbe302b36a1aad1d95cab54f9e0a135eb01906122389082613631565b506040820151600282019061224d9082613631565b5050815160408051818152600d818301526c544f4b454e204144445245535360981b60608201526001600160a01b039092166020830152516000805160206137f183398151915292509081900360800190a17f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358381602001516040516122d291906136f0565b60405180910390a17f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583816040015160405161230d919061372b565b60405180910390a150600101612032565b50565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156124075760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156123ef57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116123b15790505b50505050508152505081526020019060010190612345565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020016000905b8282101561240757838290600052602060002001805461245390612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461247f90612b2c565b80156124cc5780601f106124a1576101008083540402835291602001916124cc565b820191906000526020600020905b8154815290600101906020018083116124af57829003601f168201915b505050505081526020019060010190612434565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156124075760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156125ae57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116125705790505b50505050508152505081526020019060010190612504565b60606010805480602002602001604051908101604052809291908181526020016000905b8282101561240757838290600052602060002001805461260990612b2c565b80601f016020809104026020016040519081016040528092919081815260200182805461263590612b2c565b80156126825780601f1061265757610100808354040283529160200191612682565b820191906000526020600020905b81548152906001019060200180831161266557829003601f168201915b5050505050815260200190600101906125ea565b60008054610100900460ff16156126b65750600054610100900460ff1690565b60006000805160206137d18339815191523b156127b057604080516000805160206137d1833981519152602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091612738917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001613761565b60408051601f198184030181529082905261275291613792565b6000604051808303816000865af19150503d806000811461278f576040519150601f19603f3d011682016040523d82523d6000602084013e612794565b606091505b50915050808060200190518101906127ac91906137ae565b9150505b919050565b603781815481106106f657600080fd5b6060600c80548060200260200160405190810160405280929190818152602001828054801561067c576020028201919060005260206000209081546001600160a01b0316815260019091019060200180831161065e575050505050905090565b604281815481106106f657600080fd5b602080825282518282018190526000918401906040840190835b818110156128765783516001600160a01b031683526020938401939092019160010161284f565b509095945050505050565b60006020828403121561289357600080fd5b5035919050565b60005b838110156128b557818101518382015260200161289d565b50506000910152565b600081518084526128d681602086016020860161289a565b601f01601f19169290920160200192915050565b6001600160a01b038416815260606020820181905260009061290e908301856128be565b828103604084015261292081856128be565b9695505050505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff811182821017156129695761296961292a565b604052919050565b600067ffffffffffffffff82111561298b5761298b61292a565b50601f01601f191660200190565b6000602082840312156129ab57600080fd5b813567ffffffffffffffff8111156129c257600080fd5b8201601f810184136129d357600080fd5b80356129e66129e182612971565b612940565b8181528560208385010111156129fb57600080fd5b81602084016020830137600091810160200191909152949350505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612ac757868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b80831015612aaf5783516001600160e01b03191682526020938401936001939093019290910190612a83565b50965050506020938401939190910190600101612a41565b50929695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612ac757603f19878603018452612b178583516128be565b94506020938401939190910190600101612afb565b600181811c90821680612b4057607f821691505b602082108103612b6057634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052603260045260246000fd5b606081526000612b8f60608301866128be565b828103602084015260008554612ba481612b2c565b808452600182168015612bbe5760018114612bda57612c11565b60ff1983166020860152602082151560051b8601019350612c11565b88600052602060002060005b83811015612c0857815460208289010152600182019150602081019050612be6565b86016020019450505b5050506001600160a01b03851660408501529150612c2c9050565b949350505050565b600060208284031215612c4657600080fd5b815167ffffffffffffffff811115612c5d57600080fd5b8201601f81018413612c6e57600080fd5b8051612c7c6129e182612971565b818152856020838501011115612c9157600080fd5b612ca282602083016020860161289a565b95945050505050565b81810381811115612ccc57634e487b7160e01b600052601160045260246000fd5b92915050565b606081526000612ce560608301856128be565b828103602080850191909152601482527332b4b3b2b72630bcb2b9283937bc3ca0b236b4b760611b908201526001600160a01b03939093166040928301525001919050565b606081526000612d3d60608301856128be565b8281036020808501919091526013825272656967656e4c6179657250617573657252656760681b908201526001600160a01b03939093166040928301525001919050565b606081526000612d9460608301856128be565b828103602080850191909152600c82526b6176734469726563746f727960a01b908201526001600160a01b03939093166040928301525001919050565b606081526000612de460608301856128be565b828103602080850191909152601a82527f6176734469726563746f7279496d706c656d656e746174696f6e000000000000908201526001600160a01b03939093166040928301525001919050565b606081526000612e4560608301856128be565b82810360208085019190915260118252703232b632b3b0ba34b7b726b0b730b3b2b960791b908201526001600160a01b03939093166040928301525001919050565b606081526000612e9a60608301856128be565b828103602080850191909152601f82527f64656c65676174696f6e4d616e61676572496d706c656d656e746174696f6e00908201526001600160a01b03939093166040928301525001919050565b606081526000612efb60608301856128be565b828103602080850191909152600f82526e39ba3930ba32b3bca6b0b730b3b2b960891b908201526001600160a01b03939093166040928301525001919050565b606081526000612f4e60608301856128be565b828103602080850191909152601d82527f73747261746567794d616e61676572496d706c656d656e746174696f6e000000908201526001600160a01b03939093166040928301525001919050565b606081526000612faf60608301856128be565b82810360208085019190915260128252713932bbb0b93239a1b7b7b93234b730ba37b960711b908201526001600160a01b03939093166040928301525001919050565b60608152600061300560608301856128be565b8281036020808501919091528082527f72657761726473436f6f7264696e61746f72496d706c656d656e746174696f6e908201526001600160a01b03939093166040928301525001919050565b60608152600061306560608301856128be565b828103602080850191909152600f82526e32b4b3b2b72837b226b0b730b3b2b960891b908201526001600160a01b03939093166040928301525001919050565b6060815260006130b860608301856128be565b828103602080850191909152601d82527f656967656e506f644d616e61676572496d706c656d656e746174696f6e000000908201526001600160a01b03939093166040928301525001919050565b60608152600061311960608301856128be565b828103602080850191909152600e82526d32b4b3b2b72837b22132b0b1b7b760911b908201526001600160a01b03939093166040928301525001919050565b60608152600061316b60608301856128be565b828103602080850191909152601682527532b4b3b2b72837b224b6b83632b6b2b73a30ba34b7b760511b908201526001600160a01b03939093166040928301525001919050565b6060815260006131c560608301856128be565b828103602080850191909152601a82527f626173655374726174656779496d706c656d656e746174696f6e000000000000908201526001600160a01b03939093166040928301525001919050565b60608152600061322660608301856128be565b828103602080850191909152600d82526c195b5c1d1e50dbdb9d1c9858dd609a1b908201526001600160a01b03939093166040928301525001919050565b60608152600061327760608301856128be565b828103806020850152600a8252697374726174656769657360b01b602083015260408101604085015250612ca260408201856128be565b6060815260006132c160608301856128be565b82810360208401526132f081601081526f6578656375746f724d756c746973696760801b602082015260400190565b91505060018060a01b03831660408301529392505050565b60608152600061331b60608301856128be565b82810360208401526132f08160128152716f7065726174696f6e734d756c746973696760701b602082015260400190565b60608152600061335f60608301856128be565b82810360208401526132f0816011815270636f6d6d756e6974794d756c746973696760781b602082015260400190565b6060815260006133a260608301856128be565b82810360208401526132f081600e81526d7061757365724d756c746973696760901b602082015260400190565b6060815260006133e260608301856128be565b828103602080850191909152600882526774696d656c6f636b60c01b908201526001600160a01b03939093166040928301525001919050565b60608152600061342e60608301856128be565b8281036020840152600f81526e6465706c6f796d656e74426c6f636b60881b6020820152604081019150508260408301529392505050565b60608152600061347960608301856128be565b8281036020840152600781526618da185a5b925960ca1b6020820152604081019150508260408301529392505050565b6060815260006134bc60608301866128be565b82810360208401526134ce81866128be565b9050828103604084015261292081856128be565b6040815260006134f560408301856128be565b8281036020840152612ca281856128be565b60408152600061353760408301601081526f6578656375746f724d756c746973696760801b602082015260400190565b6001600160a01b0393909316602092909201919091525090565b6040815260006135376040830160128152716f7065726174696f6e734d756c746973696760701b602082015260400190565b604081526000613537604083016011815270636f6d6d756e6974794d756c746973696760781b602082015260400190565b60408152600061353760408301600e81526d7061757365724d756c746973696760901b602082015260400190565b601f82111561362c57806000526020600020601f840160051c810160208510156136095750805b601f840160051c820191505b818110156136295760008155600101613615565b50505b505050565b815167ffffffffffffffff81111561364b5761364b61292a565b61365f816136598454612b2c565b846135e2565b6020601f821160018114613693576000831561367b5750848201515b600019600385901b1c1916600184901b178455613629565b600084815260208120601f198516915b828110156136c357878501518255602094850194600190920191016136a3565b50848210156136e15786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b60408152600a604082015269544f4b454e204e414d4560b01b606082015260806020820152600061372460808301846128be565b9392505050565b60408152600c60408201526b1513d2d1538814d6535093d360a21b606082015260806020820152600061372460808301846128be565b6001600160e01b031983168152815160009061378481600485016020870161289a565b919091016004019392505050565b600082516137a481846020870161289a565b9190910192915050565b6000602082840312156137c057600080fd5b8151801515811461372457600080fdfe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8a26469706673582212205f7a045e7408c65246a4ec2466cfd3b740cf9258bb8a6be8d980128c7d81a02364736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x9FW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\x01gW\x80c\xDBM\xF7a\x11a\0\xCEW\x80c\xF2\xEB\xB0\xB6\x11a\0\x87W\x80c\xF2\xEB\xB0\xB6\x14a\x05\xBEW\x80c\xF3\x9E\x91`\x14a\x05\xD1W\x80c\xF7\xE7n6\x14a\x05\xE4W\x80c\xF8\xCC\xBFG\x14a\x05\xF7W\x80c\xFAv&\xD4\x14a\x06\x04W\x80c\xFD\xC3q\xCE\x14a\x06\x11W`\0\x80\xFD[\x80c\xDBM\xF7a\x14a\x05WW\x80c\xE2\x0C\x9Fq\x14a\x05jW\x80c\xE3\xA8\xB3E\x14a\x05rW\x80c\xE7\xACU\xFC\x14a\x05\x85W\x80c\xEAM<\x9B\x14a\x05\x98W\x80c\xF0\x06-\x9A\x14a\x05\xABW`\0\x80\xFD[\x80c\xBAAO\xA6\x11a\x01 W\x80c\xBAAO\xA6\x14a\x04\xDBW\x80c\xBA\x8Ce\xD8\x14a\x04\xF3W\x80c\xBE[\xB5\xF6\x14a\x05\x06W\x80c\xC1\xDA\xCA\x80\x14a\x05\x19W\x80c\xCA\x8A\xA7\xC7\x14a\x05,W\x80c\xD0\xAF&\xE1\x14a\x05?W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x04}W\x80c\x8A/\xC4\xE3\x14a\x04\x92W\x80c\x91j\x17\xC6\x14a\x04\xA5W\x80c\x99\xC1\xEF+\x14a\x04\xADW\x80c\x9E\xF3W\x10\x14a\x04\xC0W\x80c\xB5P\x8A\xA9\x14a\x04\xD3W`\0\x80\xFD[\x80c?M\xA4\xC6\x11a\x02\x0BW\x80cR1V@\x11a\x01\xC4W\x80cR1V@\x14a\x04\x14W\x80c]\xA8\xB4\xCE\x14a\x04'W\x80cf\xD9\xA9\xA0\x14a\x04/W\x80ck:\xA7.\x14a\x04DW\x80cmB\xC7P\x14a\x04WW\x80cq\xC5l2\x14a\x04jW`\0\x80\xFD[\x80c?M\xA4\xC6\x14a\x03\x9CW\x80c?r\x86\xF4\x14a\x03\xAFW\x80cFe\xBC\xDA\x14a\x03\xB7W\x80cF\xE4\xE1\xBF\x14a\x03\xCAW\x80cG\xC9M\xDA\x14a\x03\xECW\x80cQn((\x14a\x03\xFFW`\0\x80\xFD[\x80c)+{+\x11a\x02]W\x80c)+{+\x14a\x035W\x80c2\xC0\x85\x85\x14a\x03HW\x80c9\xB7\x0E8\x14a\x03[W\x80c>+\xEE;\x14a\x03nW\x80c>^<#\x14a\x03\x81W\x80c?H?\xFA\x14a\x03\x89W`\0\x80\xFD[\x80b\x91\x9A\xFE\x14a\x02\xA4W\x80c\x04\x92\xF4\xBC\x14a\x02\xD4W\x80c\x1E-3K\x14a\x02\xE7W\x80c\x1E\xD7\x83\x1C\x14a\x02\xFAW\x80c!\xCB>7\x14a\x03\x0FW\x80c&\x89cc\x14a\x03\"W[`\0\x80\xFD[`/Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`2Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`+Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a\x06$V[`@Qa\x02\xCB\x91\x90a(5V[`6Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`4Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`'Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`-Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`!Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1ETa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a\x06\x86V[a\x02\xB7a\x03\x976`\x04a(\x81V[a\x06\xE6V[`3Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a\x07\x10V[`%Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xDDa\x03\xD86`\x04a(\x81V[a\x07pV[`@Qa\x02\xCB\x93\x92\x91\x90a(\xEAV[a\x02\xB7a\x03\xFA6`\x04a(\x81V[a\x08\xC0V[a\x04\x12a\x04\r6`\x04a)\x99V[a\x08\xD0V[\0[a\x02\xB7a\x04\"6`\x04a(\x81V[a\x1A\xBEV[a\x04\x12a\x1A\xCEV[a\x047a#!V[`@Qa\x02\xCB\x91\x90a*\x19V[`\x1DTa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1CTa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`$Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x85a$\x10V[`@Qa\x02\xCB\x91\x90a*\xD3V[`#Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x047a$\xE0V[`)Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`*Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x85a%\xC6V[a\x04\xE3a&\x96V[`@Q\x90\x15\x15\x81R` \x01a\x02\xCBV[a\x02\xB7a\x05\x016`\x04a(\x81V[a'\xB5V[` Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\"Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`,Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1BTa\x02\xB7\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`5Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x02a'\xC5V[`;Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xB7a\x05\x936`\x04a(\x81V[a(%V[`\x1FTa\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`.Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`0Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`&Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`(Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1BTa\x04\xE3\x90`\xFF\x16\x81V[`\0Ta\x04\xE3\x90`\xFF\x16\x81V[`1Ta\x02\xB7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^WPPPPP\x90P\x90V[`8\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^WPPPPP\x90P\x90V[`D\x81\x81T\x81\x10a\x07\x80W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x03\x90\x91\x02\x01\x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90a\x07\xAF\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xDB\x90a+,V[\x80\x15a\x08(W\x80`\x1F\x10a\x07\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x08=\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08i\x90a+,V[\x80\x15a\x08\xB6W\x80`\x1F\x10a\x08\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xB6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\x99W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[`9\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[`@\x80Q\x80\x82\x01\x82R`\r\x81Rl\x1C\x18\\\x99[\x9D\x08\x1B\xD8\x9A\x99X\xDD`\x9A\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\n\x83Ristrategies`\xB0\x1B\x90\x83\x01R\x90`\0[`CT\x81\x10\x15a\n\x15W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x97,`b\x83`D\x84\x81T\x81\x10a\tiWa\tia+fV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x02\x01`B\x85\x81T\x81\x10a\t\x8DWa\t\x8Da+fV[`\0\x91\x82R` \x90\x91 \x01T`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra\t\xC5\x93\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a+|V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x0C\x91\x90\x81\x01\x90a,4V[P`\x01\x01a\t\x19V[P`\0`CT`\0\x14a\x0B,W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x97,`b\x83`D`\x01`CTa\nf\x91\x90a,\xABV[\x81T\x81\x10a\nvWa\nva+fV[\x90`\0R` `\0 \x90`\x03\x02\x01`\x02\x01`B`\x01`CTa\n\x98\x91\x90a,\xABV[\x81T\x81\x10a\n\xA8Wa\n\xA8a+fV[`\0\x91\x82R` \x90\x91 \x01T`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra\n\xE0\x93\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a+|V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B'\x91\x90\x81\x01\x90a,4V[a\x0B=V[`@Q\x80` \x01`@R\x80`\0\x81RP[`@\x80Q\x80\x82\x01\x82R`\t\x81Rhaddresses`\xB8\x1B` \x82\x01R`\x1BT\x91QcK\x9601`\xE1\x1B\x81R\x92\x93P\x91`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0B\xA3\x91\x85\x91a\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a,\xD2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\xEA\x91\x90\x81\x01\x90a,4V[P`\x1CT`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0C+\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a-*V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0CJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Cr\x91\x90\x81\x01\x90a,4V[P`\x1DT`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0C\xB3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a-\x81V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xFA\x91\x90\x81\x01\x90a,4V[P`\x1ET`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\r;\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a-\xD1V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\rZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x82\x91\x90\x81\x01\x90a,4V[P`\x1FT`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\r\xC3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a.2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\n\x91\x90\x81\x01\x90a,4V[P` T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0EK\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a.\x87V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0EjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x92\x91\x90\x81\x01\x90a,4V[P`!T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0E\xD3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a.\xE8V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x1A\x91\x90\x81\x01\x90a,4V[P`\"T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0F[\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a/;V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0FzW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\xA2\x91\x90\x81\x01\x90a,4V[P`#T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x0F\xE3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a/\x9CV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10*\x91\x90\x81\x01\x90a,4V[P`$T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x10k\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a/\xF2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x10\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\xB2\x91\x90\x81\x01\x90a,4V[P`%T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x10\xF3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a0RV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11:\x91\x90\x81\x01\x90a,4V[P`&T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x11{\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a0\xA5V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\xC2\x91\x90\x81\x01\x90a,4V[P`'T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x12\x03\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a1\x06V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12J\x91\x90\x81\x01\x90a,4V[P`(T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x12\x8B\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a1XV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xD2\x91\x90\x81\x01\x90a,4V[P`)T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x13\x13\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a1\xB2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x132W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13Z\x91\x90\x81\x01\x90a,4V[P`;T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x13\x9B\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a2\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xE2\x91\x90\x81\x01\x90a,4V[P`@Qc\x88\xDAm5`\xE0\x1B\x81R`\0\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x14\x19\x90\x85\x90\x87\x90`\x04\x01a2dV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x148W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14`\x91\x90\x81\x01\x90a,4V[`@\x80Q\x80\x82\x01\x82R`\n\x81Riparameters`\xB0\x1B` \x82\x01R`<T\x91QcK\x9601`\xE1\x1B\x81R\x92\x93P\x91`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x14\xC3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a2\xAEV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\n\x91\x90\x81\x01\x90a,4V[P`=T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x15K\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x15\x92\x91\x90\x81\x01\x90a,4V[P`>T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x15\xD3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3LV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\x1A\x91\x90\x81\x01\x90a,4V[P`?T`@QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x16[\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16zW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\xA2\x91\x90\x81\x01\x90a,4V[P`@\x80T\x90QcK\x9601`\xE1\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x16\xE3\x91\x85\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90`\x04\x01a3\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17*\x91\x90\x81\x01\x90a,4V[P`=T`@QcK\x9601`\xE1\x1B\x81R`\0\x91`\0\x80Q` a7\xD1\x839\x81Q\x91R\x91c\x97,`b\x91a\x17l\x91\x86\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x04\x01a3\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xB3\x91\x90\x81\x01\x90a,4V[`@\x80Q\x80\x82\x01\x82R`\t\x81RhchainInfo`\xB8\x1B` \x82\x01R\x90Qc\tOH\x01`\xE1\x1B\x81R\x91\x92P\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x12\x9E\x90\x02\x90a\x18\x08\x90\x84\x90C\x90`\x04\x01a4\x1BV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x18'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18O\x91\x90\x81\x01\x90a,4V[P`@Qc\tOH\x01`\xE1\x1B\x81R`\0\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x12\x9E\x90\x02\x90a\x18\x86\x90\x85\x90F\x90`\x04\x01a4fV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x18\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\xCD\x91\x90\x81\x01\x90a,4V[`@Qc\x88\xDAm5`\xE0\x1B\x81R\x90\x91P`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x19\x05\x90\x8C\x90\x8A\x90\x8A\x90`\x04\x01a4\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19L\x91\x90\x81\x01\x90a,4V[P`@Qc\x88\xDAm5`\xE0\x1B\x81R`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x19\x82\x90\x8C\x90\x86\x90\x86\x90`\x04\x01a4\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\xC9\x91\x90\x81\x01\x90a,4V[P`@Qc\x88\xDAm5`\xE0\x1B\x81R`\0\x90`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\x88\xDAm5\x90a\x1A\x02\x90\x8D\x90\x89\x90\x89\x90`\x04\x01a4\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1A!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1AI\x91\x90\x81\x01\x90a,4V[`@Qc\xE2<\xD1\x9F`\xE0\x1B\x81R\x90\x91P`\0\x80Q` a7\xD1\x839\x81Q\x91R\x90c\xE2<\xD1\x9F\x90a\x1A\x7F\x90\x84\x90\x8F\x90`\x04\x01a4\xE2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xADW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPPV[`:\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[\x7F\x0B.\x13\xFF \xAC{GA\x98eU\x83\xED\xF7\r\xED\xD2\xC1\xDC\x98\x0E2\x9CO\xBB/\xC0t\x8Byk`@Qa\x1BS\x90` \x80\x82R`8\x90\x82\x01R\x7F==== Parsed Initilize Params for`@\x82\x01R\x7F Initial Deployment ====\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`<T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1B\x86\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5\x07V[`@Q\x80\x91\x03\x90\xA1`=T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1B\xB9\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5QV[`@Q\x80\x91\x03\x90\xA1`>T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1B\xEC\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5\x83V[`@Q\x80\x91\x03\x90\xA1`?T`@Q`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91a\x1C\x1F\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a5\xB4V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`ET`@Qa\x1C\x8C\x91\x90`@\x80\x82R`#\x90\x82\x01R\x7FSTRATEGY_MANAGER_INIT_PAUSED_STA``\x82\x01RbTUS`\xE8\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`FT`@\x80Q\x81\x81R`\x1C\x81\x83\x01R\x7FSTRATEGY_MANAGER_WHITELISTER\0\0\0\0``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`HT`@Qa\x1Dc\x91\x90`@\x80\x82R`.\x90\x82\x01R\x7FDELEGATION_MANAGER_MIN_WITHDRAWA``\x82\x01RmL_DELAY_BLOCKS`\x90\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`GT`@Qa\x1D\xD2\x91\x90`@\x80\x82R`%\x90\x82\x01R\x7FDELEGATION_MANAGER_INIT_PAUSED_S``\x82\x01RdTATUS`\xD8\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`JT`@\x80Q\x81\x81R` \x81\x83\x01\x81\x90R\x7FAVS_DIRECTORY_INIT_PAUSED_STATUS``\x83\x01R\x81\x01\x92\x90\x92RQ`\0\x80Q` a8\x11\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`KT`@Qa\x1E\x99\x91\x90`@\x80\x82R`&\x90\x82\x01R\x7FREWARDS_COORDINATOR_INIT_PAUSED_``\x82\x01ReSTATUS`\xD0\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a8\x11\x839\x81Q\x91R`OT`@Qa\x1F\x06\x91\x90`@\x80\x82R`#\x90\x82\x01R\x7FEIGENPOD_MANAGER_INIT_PAUSED_STA``\x82\x01RbTUS`\xE8\x1B`\x80\x82\x01R` \x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA1`QT`@\x80Q\x81\x81R`\x15\x81\x83\x01RtEIGENPOD_GENESIS_TIME`X\x1B``\x82\x01Rh\x01\0\0\0\0\0\0\0\0\x90\x92\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01RQ`\0\x80Q` a8\x11\x839\x81Q\x91R\x91`\x80\x90\x82\x90\x03\x01\x90\xA1`RT`@\x80Q\x81\x81R`\x14\x81\x83\x01RsETHPOSDepositAddress``\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ`\0\x80Q` a7\xF1\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1\x7F\x0B.\x13\xFF \xAC{GA\x98eU\x83\xED\xF7\r\xED\xD2\xC1\xDC\x98\x0E2\x9CO\xBB/\xC0t\x8Byk`@Qa '\x90` \x80\x82R`\x1E\x90\x82\x01R\x7F==== Strategies to Deploy ====\0\0`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0[`CT\x81\x10\x15a#\x1EW`\0`D\x82\x81T\x81\x10a QWa Qa+fV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x90\x91R`\x03\x90\x92\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a \x91\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \xBD\x90a+,V[\x80\x15a!\nW\x80`\x1F\x10a \xDFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\nV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \xEDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta!#\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!O\x90a+,V[\x80\x15a!\x9CW\x80`\x1F\x10a!qWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`D\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R\x82Q`\x03\x90\x91\x02\x7F\x9B\"\xD3\xD6\x19Y\xB4\xD3R\x8B\x1D\x8B\xA92\xC9o\xBE0+6\xA1\xAA\xD1\xD9\\\xABT\xF9\xE0\xA15\xEA\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x82U` \x84\x01Q\x93\x94P\x84\x93\x91\x92P\x7F\x9B\"\xD3\xD6\x19Y\xB4\xD3R\x8B\x1D\x8B\xA92\xC9o\xBE0+6\xA1\xAA\xD1\xD9\\\xABT\xF9\xE0\xA15\xEB\x01\x90a\"8\x90\x82a61V[P`@\x82\x01Q`\x02\x82\x01\x90a\"M\x90\x82a61V[PP\x81Q`@\x80Q\x81\x81R`\r\x81\x83\x01RlTOKEN ADDRESS`\x98\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ`\0\x80Q` a7\xF1\x839\x81Q\x91R\x92P\x90\x81\x90\x03`\x80\x01\x90\xA1\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x81` \x01Q`@Qa\"\xD2\x91\x90a6\xF0V[`@Q\x80\x91\x03\x90\xA1\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x81`@\x01Q`@Qa#\r\x91\x90a7+V[`@Q\x80\x91\x03\x90\xA1P`\x01\x01a 2V[PV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a#\xEFW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a#\xB1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a#EV[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W\x83\x82\x90`\0R` `\0 \x01\x80Ta$S\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$\x7F\x90a+,V[\x80\x15a$\xCCW\x80`\x1F\x10a$\xA1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xCCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\xAFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a$4V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a%\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a%pW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\x04V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a$\x07W\x83\x82\x90`\0R` `\0 \x01\x80Ta&\t\x90a+,V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&5\x90a+,V[\x80\x15a&\x82W\x80`\x1F\x10a&WWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&\x82V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&eW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a%\xEAV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a&\xB6WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a7\xD1\x839\x81Q\x91R;\x15a'\xB0W`@\x80Q`\0\x80Q` a7\xD1\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a'8\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a7aV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'R\x91a7\x92V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a'\x8FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'\x94V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a'\xAC\x91\x90a7\xAEV[\x91PP[\x91\x90PV[`7\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06|W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06^WPPPPP\x90P\x90V[`B\x81\x81T\x81\x10a\x06\xF6W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(vW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(OV[P\x90\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a(\x93W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a(\xB5W\x81\x81\x01Q\x83\x82\x01R` \x01a(\x9DV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra(\xD6\x81` \x86\x01` \x86\x01a(\x9AV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a)\x0E\x90\x83\x01\x85a(\xBEV[\x82\x81\x03`@\x84\x01Ra) \x81\x85a(\xBEV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)iWa)ia)*V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\x8BWa)\x8Ba)*V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a)\xABW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xC2W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\xD3W`\0\x80\xFD[\x805a)\xE6a)\xE1\x82a)qV[a)@V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\xFBW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*\xC7W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15a*\xAFW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a*\x83V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*AV[P\x92\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*\xC7W`?\x19\x87\x86\x03\x01\x84Ra+\x17\x85\x83Qa(\xBEV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*\xFBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a+@W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+`WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[``\x81R`\0a+\x8F``\x83\x01\x86a(\xBEV[\x82\x81\x03` \x84\x01R`\0\x85Ta+\xA4\x81a+,V[\x80\x84R`\x01\x82\x16\x80\x15a+\xBEW`\x01\x81\x14a+\xDAWa,\x11V[`\xFF\x19\x83\x16` \x86\x01R` \x82\x15\x15`\x05\x1B\x86\x01\x01\x93Pa,\x11V[\x88`\0R` `\0 `\0[\x83\x81\x10\x15a,\x08W\x81T` \x82\x89\x01\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa+\xE6V[\x86\x01` \x01\x94PP[PPP`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x85\x01R\x91Pa,,\x90PV[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a,FW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,]W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a,nW`\0\x80\xFD[\x80Qa,|a)\xE1\x82a)qV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a,\x91W`\0\x80\xFD[a,\xA2\x82` \x83\x01` \x86\x01a(\x9AV[\x95\x94PPPPPV[\x81\x81\x03\x81\x81\x11\x15a,\xCCWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[``\x81R`\0a,\xE5``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x14\x82Rs2\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9(97\xBC<\xA0\xB26\xB4\xB7`a\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a-=``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x13\x82RreigenLayerPauserReg`h\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a-\x94``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0C\x82RkavsDirectory`\xA0\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a-\xE4``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1A\x82R\x7FavsDirectoryImplementation\0\0\0\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a.E``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x11\x82Rp22\xB62\xB3\xB0\xBA4\xB7\xB7&\xB0\xB70\xB3\xB2\xB9`y\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a.\x9A``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1F\x82R\x7FdelegationManagerImplementation\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a.\xFB``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0F\x82Rn9\xBA90\xBA2\xB3\xBC\xA6\xB0\xB70\xB3\xB2\xB9`\x89\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a/N``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1D\x82R\x7FstrategyManagerImplementation\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a/\xAF``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x12\x82Rq92\xBB\xB0\xB929\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a0\x05``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R\x80\x82R\x7FrewardsCoordinatorImplementation\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a0e``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0F\x82Rn2\xB4\xB3\xB2\xB7(7\xB2&\xB0\xB70\xB3\xB2\xB9`\x89\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a0\xB8``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1D\x82R\x7FeigenPodManagerImplementation\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a1\x19``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x0E\x82Rm2\xB4\xB3\xB2\xB7(7\xB2!2\xB0\xB1\xB7\xB7`\x91\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a1k``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x16\x82Ru2\xB4\xB3\xB2\xB7(7\xB2$\xB6\xB862\xB6\xB2\xB7:0\xBA4\xB7\xB7`Q\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a1\xC5``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x1A\x82R\x7FbaseStrategyImplementation\0\0\0\0\0\0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a2&``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\r\x82Rl\x19[\\\x1D\x1EP\xDB\xDB\x9D\x1C\x98X\xDD`\x9A\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a2w``\x83\x01\x85a(\xBEV[\x82\x81\x03\x80` \x85\x01R`\n\x82Ristrategies`\xB0\x1B` \x83\x01R`@\x81\x01`@\x85\x01RPa,\xA2`@\x82\x01\x85a(\xBEV[``\x81R`\0a2\xC1``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x10\x81RoexecutorMultisig`\x80\x1B` \x82\x01R`@\x01\x90V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x93\x92PPPV[``\x81R`\0a3\x1B``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x12\x81RqoperationsMultisig`p\x1B` \x82\x01R`@\x01\x90V[``\x81R`\0a3_``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x11\x81RpcommunityMultisig`x\x1B` \x82\x01R`@\x01\x90V[``\x81R`\0a3\xA2``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra2\xF0\x81`\x0E\x81RmpauserMultisig`\x90\x1B` \x82\x01R`@\x01\x90V[``\x81R`\0a3\xE2``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x80\x85\x01\x91\x90\x91R`\x08\x82Rgtimelock`\xC0\x1B\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`@\x92\x83\x01RP\x01\x91\x90PV[``\x81R`\0a4.``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01R`\x0F\x81RndeploymentBlock`\x88\x1B` \x82\x01R`@\x81\x01\x91PP\x82`@\x83\x01R\x93\x92PPPV[``\x81R`\0a4y``\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01R`\x07\x81Rf\x18\xDA\x18Z[\x92Y`\xCA\x1B` \x82\x01R`@\x81\x01\x91PP\x82`@\x83\x01R\x93\x92PPPV[``\x81R`\0a4\xBC``\x83\x01\x86a(\xBEV[\x82\x81\x03` \x84\x01Ra4\xCE\x81\x86a(\xBEV[\x90P\x82\x81\x03`@\x84\x01Ra) \x81\x85a(\xBEV[`@\x81R`\0a4\xF5`@\x83\x01\x85a(\xBEV[\x82\x81\x03` \x84\x01Ra,\xA2\x81\x85a(\xBEV[`@\x81R`\0a57`@\x83\x01`\x10\x81RoexecutorMultisig`\x80\x1B` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16` \x92\x90\x92\x01\x91\x90\x91RP\x90V[`@\x81R`\0a57`@\x83\x01`\x12\x81RqoperationsMultisig`p\x1B` \x82\x01R`@\x01\x90V[`@\x81R`\0a57`@\x83\x01`\x11\x81RpcommunityMultisig`x\x1B` \x82\x01R`@\x01\x90V[`@\x81R`\0a57`@\x83\x01`\x0E\x81RmpauserMultisig`\x90\x1B` \x82\x01R`@\x01\x90V[`\x1F\x82\x11\x15a6,W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a6\tWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a6)W`\0\x81U`\x01\x01a6\x15V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6KWa6Ka)*V[a6_\x81a6Y\x84Ta+,V[\x84a5\xE2V[` `\x1F\x82\x11`\x01\x81\x14a6\x93W`\0\x83\x15a6{WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua6)V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a6\xC3W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a6\xA3V[P\x84\x82\x10\x15a6\xE1W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`@\x81R`\n`@\x82\x01RiTOKEN NAME`\xB0\x1B``\x82\x01R`\x80` \x82\x01R`\0a7$`\x80\x83\x01\x84a(\xBEV[\x93\x92PPPV[`@\x81R`\x0C`@\x82\x01Rk\x15\x13\xD2\xD1S\x88\x14\xD6SP\x93\xD3`\xA2\x1B``\x82\x01R`\x80` \x82\x01R`\0a7$`\x80\x83\x01\x84a(\xBEV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a7\x84\x81`\x04\x85\x01` \x87\x01a(\x9AV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa7\xA4\x81\x84` \x87\x01a(\x9AV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a7\xC0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a7$W`\0\x80\xFD\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\xA2dipfsX\"\x12 _z\x04^t\x08\xC6RF\xA4\xEC$f\xCF\xD3\xB7@\xCF\x92X\xBB\x8Ak\xE8\xD9\x80\x12\x8C}\x81\xA0#dsolcC\0\x08\x1B\x003",
    );
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
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
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
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
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `EIGEN()` and selector `0xfdc371ce`.
```solidity
function EIGEN() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIGENCall {}
    ///Container type for the return parameters of the [`EIGEN()`](EIGENCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIGENReturn {
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
            impl ::core::convert::From<EIGENCall> for UnderlyingRustTuple<'_> {
                fn from(value: EIGENCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIGENCall {
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
            impl ::core::convert::From<EIGENReturn> for UnderlyingRustTuple<'_> {
                fn from(value: EIGENReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIGENReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EIGENCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EIGENReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EIGEN()";
            const SELECTOR: [u8; 4] = [253u8, 195u8, 113u8, 206u8];
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
    /**Function with signature `EIGENImpl()` and selector `0x0492f4bc`.
```solidity
function EIGENImpl() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIGENImplCall {}
    ///Container type for the return parameters of the [`EIGENImpl()`](EIGENImplCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIGENImplReturn {
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
            impl ::core::convert::From<EIGENImplCall> for UnderlyingRustTuple<'_> {
                fn from(value: EIGENImplCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIGENImplCall {
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
            impl ::core::convert::From<EIGENImplReturn> for UnderlyingRustTuple<'_> {
                fn from(value: EIGENImplReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIGENImplReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EIGENImplCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EIGENImplReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EIGENImpl()";
            const SELECTOR: [u8; 4] = [4u8, 146u8, 244u8, 188u8];
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
    /**Function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`.
```solidity
function IS_SCRIPT() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_SCRIPTCall {}
    ///Container type for the return parameters of the [`IS_SCRIPT()`](IS_SCRIPTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_SCRIPTReturn {
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
            impl ::core::convert::From<IS_SCRIPTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_SCRIPTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_SCRIPTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<IS_SCRIPTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_SCRIPTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_SCRIPTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_SCRIPTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_SCRIPTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_SCRIPT()";
            const SELECTOR: [u8; 4] = [248u8, 204u8, 191u8, 71u8];
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
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
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
    /**Function with signature `allEigenPods(uint256)` and selector `0x52315640`.
```solidity
function allEigenPods(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allEigenPodsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`allEigenPods(uint256)`](allEigenPodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allEigenPodsReturn {
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
            impl ::core::convert::From<allEigenPodsCall> for UnderlyingRustTuple<'_> {
                fn from(value: allEigenPodsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allEigenPodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<allEigenPodsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allEigenPodsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allEigenPodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allEigenPodsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allEigenPodsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allEigenPods(uint256)";
            const SELECTOR: [u8; 4] = [82u8, 49u8, 86u8, 64u8];
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
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
```solidity
function allocationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
    /**Function with signature `allocationManagerImplementation()` and selector `0x32c08585`.
```solidity
function allocationManagerImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerImplementationCall {}
    ///Container type for the return parameters of the [`allocationManagerImplementation()`](allocationManagerImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerImplementationReturn {
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
            impl ::core::convert::From<allocationManagerImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerImplementationCall {
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
            impl ::core::convert::From<allocationManagerImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for allocationManagerImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManagerImplementation()";
            const SELECTOR: [u8; 4] = [50u8, 192u8, 133u8, 133u8];
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
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
```solidity
function avsDirectory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
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
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
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
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
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
    /**Function with signature `avsDirectoryImplementation()` and selector `0x3e2bee3b`.
```solidity
function avsDirectoryImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryImplementationCall {}
    ///Container type for the return parameters of the [`avsDirectoryImplementation()`](avsDirectoryImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryImplementationReturn {
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
            impl ::core::convert::From<avsDirectoryImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for avsDirectoryImplementationCall {
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
            impl ::core::convert::From<avsDirectoryImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for avsDirectoryImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectoryImplementation()";
            const SELECTOR: [u8; 4] = [62u8, 43u8, 238u8, 59u8];
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
    /**Function with signature `bEIGEN()` and selector `0x3f4da4c6`.
```solidity
function bEIGEN() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGENCall {}
    ///Container type for the return parameters of the [`bEIGEN()`](bEIGENCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGENReturn {
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
            impl ::core::convert::From<bEIGENCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGENCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGENCall {
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
            impl ::core::convert::From<bEIGENReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGENReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGENReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGENCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGENReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN()";
            const SELECTOR: [u8; 4] = [63u8, 77u8, 164u8, 198u8];
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
    /**Function with signature `bEIGENImpl()` and selector `0x26896363`.
```solidity
function bEIGENImpl() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGENImplCall {}
    ///Container type for the return parameters of the [`bEIGENImpl()`](bEIGENImplCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGENImplReturn {
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
            impl ::core::convert::From<bEIGENImplCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGENImplCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGENImplCall {
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
            impl ::core::convert::From<bEIGENImplReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGENImplReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGENImplReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGENImplCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGENImplReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGENImpl()";
            const SELECTOR: [u8; 4] = [38u8, 137u8, 99u8, 99u8];
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
    /**Function with signature `baseStrategyImplementation()` and selector `0x99c1ef2b`.
```solidity
function baseStrategyImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct baseStrategyImplementationCall {}
    ///Container type for the return parameters of the [`baseStrategyImplementation()`](baseStrategyImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct baseStrategyImplementationReturn {
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
            impl ::core::convert::From<baseStrategyImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: baseStrategyImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for baseStrategyImplementationCall {
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
            impl ::core::convert::From<baseStrategyImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: baseStrategyImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for baseStrategyImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for baseStrategyImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = baseStrategyImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "baseStrategyImplementation()";
            const SELECTOR: [u8; 4] = [153u8, 193u8, 239u8, 43u8];
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
    /**Function with signature `delegationManager()` and selector `0xea4d3c9b`.
```solidity
function delegationManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationManagerCall {}
    ///Container type for the return parameters of the [`delegationManager()`](delegationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationManagerReturn {
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
            impl ::core::convert::From<delegationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationManagerCall {
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
            impl ::core::convert::From<delegationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationManager()";
            const SELECTOR: [u8; 4] = [234u8, 77u8, 60u8, 155u8];
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
    /**Function with signature `delegationManagerImplementation()` and selector `0xbe5bb5f6`.
```solidity
function delegationManagerImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationManagerImplementationCall {}
    ///Container type for the return parameters of the [`delegationManagerImplementation()`](delegationManagerImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationManagerImplementationReturn {
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
            impl ::core::convert::From<delegationManagerImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationManagerImplementationCall {
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
            impl ::core::convert::From<delegationManagerImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationManagerImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationManagerImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationManagerImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationManagerImplementation()";
            const SELECTOR: [u8; 4] = [190u8, 91u8, 181u8, 246u8];
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
    /**Function with signature `deployedStrategyArray(uint256)` and selector `0xe7ac55fc`.
```solidity
function deployedStrategyArray(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deployedStrategyArrayCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deployedStrategyArray(uint256)`](deployedStrategyArrayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deployedStrategyArrayReturn {
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
            impl ::core::convert::From<deployedStrategyArrayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deployedStrategyArrayCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deployedStrategyArrayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<deployedStrategyArrayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deployedStrategyArrayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deployedStrategyArrayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deployedStrategyArrayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deployedStrategyArrayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deployedStrategyArray(uint256)";
            const SELECTOR: [u8; 4] = [231u8, 172u8, 85u8, 252u8];
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
    /**Function with signature `eigenLayerPauserReg()` and selector `0x6d42c750`.
```solidity
function eigenLayerPauserReg() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenLayerPauserRegCall {}
    ///Container type for the return parameters of the [`eigenLayerPauserReg()`](eigenLayerPauserRegCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenLayerPauserRegReturn {
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
            impl ::core::convert::From<eigenLayerPauserRegCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenLayerPauserRegCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenLayerPauserRegCall {
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
            impl ::core::convert::From<eigenLayerPauserRegReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenLayerPauserRegReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenLayerPauserRegReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenLayerPauserRegCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenLayerPauserRegReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenLayerPauserReg()";
            const SELECTOR: [u8; 4] = [109u8, 66u8, 199u8, 80u8];
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
    /**Function with signature `eigenLayerProxyAdmin()` and selector `0xd0af26e1`.
```solidity
function eigenLayerProxyAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenLayerProxyAdminCall {}
    ///Container type for the return parameters of the [`eigenLayerProxyAdmin()`](eigenLayerProxyAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenLayerProxyAdminReturn {
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
            impl ::core::convert::From<eigenLayerProxyAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenLayerProxyAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenLayerProxyAdminCall {
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
            impl ::core::convert::From<eigenLayerProxyAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenLayerProxyAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenLayerProxyAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenLayerProxyAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenLayerProxyAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenLayerProxyAdmin()";
            const SELECTOR: [u8; 4] = [208u8, 175u8, 38u8, 225u8];
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
    /**Function with signature `eigenPodBeacon()` and selector `0x292b7b2b`.
```solidity
function eigenPodBeacon() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodBeaconCall {}
    ///Container type for the return parameters of the [`eigenPodBeacon()`](eigenPodBeaconCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodBeaconReturn {
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
            impl ::core::convert::From<eigenPodBeaconCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodBeaconCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodBeaconCall {
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
            impl ::core::convert::From<eigenPodBeaconReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodBeaconReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodBeaconReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodBeaconCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodBeaconReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodBeacon()";
            const SELECTOR: [u8; 4] = [41u8, 43u8, 123u8, 43u8];
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
    /**Function with signature `eigenPodImplementation()` and selector `0xf7e76e36`.
```solidity
function eigenPodImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodImplementationCall {}
    ///Container type for the return parameters of the [`eigenPodImplementation()`](eigenPodImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodImplementationReturn {
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
            impl ::core::convert::From<eigenPodImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodImplementationCall {
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
            impl ::core::convert::From<eigenPodImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodImplementation()";
            const SELECTOR: [u8; 4] = [247u8, 231u8, 110u8, 54u8];
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
    /**Function with signature `eigenPodManager()` and selector `0x4665bcda`.
```solidity
function eigenPodManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerCall {}
    ///Container type for the return parameters of the [`eigenPodManager()`](eigenPodManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerReturn {
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
            impl ::core::convert::From<eigenPodManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerCall {
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
            impl ::core::convert::From<eigenPodManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManager()";
            const SELECTOR: [u8; 4] = [70u8, 101u8, 188u8, 218u8];
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
    /**Function with signature `eigenPodManagerImplementation()` and selector `0xf39e9160`.
```solidity
function eigenPodManagerImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerImplementationCall {}
    ///Container type for the return parameters of the [`eigenPodManagerImplementation()`](eigenPodManagerImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerImplementationReturn {
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
            impl ::core::convert::From<eigenPodManagerImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodManagerImplementationCall {
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
            impl ::core::convert::From<eigenPodManagerImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodManagerImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManagerImplementation()";
            const SELECTOR: [u8; 4] = [243u8, 158u8, 145u8, 96u8];
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
    /**Function with signature `eigenStrategy()` and selector `0xdb4df761`.
```solidity
function eigenStrategy() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenStrategyCall {}
    ///Container type for the return parameters of the [`eigenStrategy()`](eigenStrategyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenStrategyReturn {
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
            impl ::core::convert::From<eigenStrategyCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenStrategyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenStrategyCall {
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
            impl ::core::convert::From<eigenStrategyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eigenStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenStrategyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenStrategy()";
            const SELECTOR: [u8; 4] = [219u8, 77u8, 247u8, 97u8];
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
    /**Function with signature `eigenStrategyImpl()` and selector `0x21cb3e37`.
```solidity
function eigenStrategyImpl() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenStrategyImplCall {}
    ///Container type for the return parameters of the [`eigenStrategyImpl()`](eigenStrategyImplCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenStrategyImplReturn {
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
            impl ::core::convert::From<eigenStrategyImplCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenStrategyImplCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenStrategyImplCall {
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
            impl ::core::convert::From<eigenStrategyImplReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenStrategyImplReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenStrategyImplReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenStrategyImplCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenStrategyImplReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenStrategyImpl()";
            const SELECTOR: [u8; 4] = [33u8, 203u8, 62u8, 55u8];
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
    /**Function with signature `emptyContract()` and selector `0xe3a8b345`.
```solidity
function emptyContract() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emptyContractCall {}
    ///Container type for the return parameters of the [`emptyContract()`](emptyContractCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct emptyContractReturn {
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
            impl ::core::convert::From<emptyContractCall> for UnderlyingRustTuple<'_> {
                fn from(value: emptyContractCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emptyContractCall {
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
            impl ::core::convert::From<emptyContractReturn> for UnderlyingRustTuple<'_> {
                fn from(value: emptyContractReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for emptyContractReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for emptyContractCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = emptyContractReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "emptyContract()";
            const SELECTOR: [u8; 4] = [227u8, 168u8, 179u8, 69u8];
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
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
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
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
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
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        pub excludedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
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
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        pub excludedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
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
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
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
    /**Function with signature `inActivePods(uint256)` and selector `0x47c94dda`.
```solidity
function inActivePods(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inActivePodsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`inActivePods(uint256)`](inActivePodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inActivePodsReturn {
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
            impl ::core::convert::From<inActivePodsCall> for UnderlyingRustTuple<'_> {
                fn from(value: inActivePodsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for inActivePodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<inActivePodsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: inActivePodsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for inActivePodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for inActivePodsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = inActivePodsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "inActivePods(uint256)";
            const SELECTOR: [u8; 4] = [71u8, 201u8, 77u8, 218u8];
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
    /**Function with signature `logAndOutputContractAddresses(string)` and selector `0x516e2828`.
```solidity
function logAndOutputContractAddresses(string memory outputPath) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct logAndOutputContractAddressesCall {
        pub outputPath: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`logAndOutputContractAddresses(string)`](logAndOutputContractAddressesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct logAndOutputContractAddressesReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
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
            impl ::core::convert::From<logAndOutputContractAddressesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: logAndOutputContractAddressesCall) -> Self {
                    (value.outputPath,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for logAndOutputContractAddressesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { outputPath: tuple.0 }
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
            impl ::core::convert::From<logAndOutputContractAddressesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: logAndOutputContractAddressesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for logAndOutputContractAddressesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for logAndOutputContractAddressesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = logAndOutputContractAddressesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "logAndOutputContractAddresses(string)";
            const SELECTOR: [u8; 4] = [81u8, 110u8, 40u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.outputPath,
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
    /**Function with signature `logInitialDeploymentParams()` and selector `0x5da8b4ce`.
```solidity
function logInitialDeploymentParams() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct logInitialDeploymentParamsCall {}
    ///Container type for the return parameters of the [`logInitialDeploymentParams()`](logInitialDeploymentParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct logInitialDeploymentParamsReturn {}
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
            impl ::core::convert::From<logInitialDeploymentParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: logInitialDeploymentParamsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for logInitialDeploymentParamsCall {
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
            impl ::core::convert::From<logInitialDeploymentParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: logInitialDeploymentParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for logInitialDeploymentParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for logInitialDeploymentParamsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = logInitialDeploymentParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "logInitialDeploymentParams()";
            const SELECTOR: [u8; 4] = [93u8, 168u8, 180u8, 206u8];
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
    /**Function with signature `multiValidatorPods(uint256)` and selector `0xba8c65d8`.
```solidity
function multiValidatorPods(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multiValidatorPodsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`multiValidatorPods(uint256)`](multiValidatorPodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multiValidatorPodsReturn {
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
            impl ::core::convert::From<multiValidatorPodsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: multiValidatorPodsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for multiValidatorPodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<multiValidatorPodsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: multiValidatorPodsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for multiValidatorPodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multiValidatorPodsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = multiValidatorPodsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multiValidatorPods(uint256)";
            const SELECTOR: [u8; 4] = [186u8, 140u8, 101u8, 216u8];
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
    /**Function with signature `rewardsCoordinator()` and selector `0x8a2fc4e3`.
```solidity
function rewardsCoordinator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsCoordinatorCall {}
    ///Container type for the return parameters of the [`rewardsCoordinator()`](rewardsCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsCoordinatorReturn {
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
            impl ::core::convert::From<rewardsCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsCoordinatorCall {
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
            impl ::core::convert::From<rewardsCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsCoordinator()";
            const SELECTOR: [u8; 4] = [138u8, 47u8, 196u8, 227u8];
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
    /**Function with signature `rewardsCoordinatorImplementation()` and selector `0x71c56c32`.
```solidity
function rewardsCoordinatorImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsCoordinatorImplementationCall {}
    ///Container type for the return parameters of the [`rewardsCoordinatorImplementation()`](rewardsCoordinatorImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsCoordinatorImplementationReturn {
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
            impl ::core::convert::From<rewardsCoordinatorImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsCoordinatorImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsCoordinatorImplementationCall {
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
            impl ::core::convert::From<rewardsCoordinatorImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsCoordinatorImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsCoordinatorImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsCoordinatorImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsCoordinatorImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsCoordinatorImplementation()";
            const SELECTOR: [u8; 4] = [113u8, 197u8, 108u8, 50u8];
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
    /**Function with signature `singleValidatorPods(uint256)` and selector `0x3f483ffa`.
```solidity
function singleValidatorPods(uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct singleValidatorPodsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`singleValidatorPods(uint256)`](singleValidatorPodsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct singleValidatorPodsReturn {
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
            impl ::core::convert::From<singleValidatorPodsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: singleValidatorPodsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for singleValidatorPodsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<singleValidatorPodsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: singleValidatorPodsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for singleValidatorPodsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for singleValidatorPodsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = singleValidatorPodsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "singleValidatorPods(uint256)";
            const SELECTOR: [u8; 4] = [63u8, 72u8, 63u8, 250u8];
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
    /**Function with signature `strategiesToDeploy(uint256)` and selector `0x46e4e1bf`.
```solidity
function strategiesToDeploy(uint256) external view returns (address tokenAddress, string memory tokenName, string memory tokenSymbol);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesToDeployCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategiesToDeploy(uint256)`](strategiesToDeployCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesToDeployReturn {
        pub tokenAddress: alloy::sol_types::private::Address,
        pub tokenName: alloy::sol_types::private::String,
        pub tokenSymbol: alloy::sol_types::private::String,
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
            impl ::core::convert::From<strategiesToDeployCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategiesToDeployCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategiesToDeployCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
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
            impl ::core::convert::From<strategiesToDeployReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategiesToDeployReturn) -> Self {
                    (value.tokenAddress, value.tokenName, value.tokenSymbol)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategiesToDeployReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenAddress: tuple.0,
                        tokenName: tuple.1,
                        tokenSymbol: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategiesToDeployCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategiesToDeployReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategiesToDeploy(uint256)";
            const SELECTOR: [u8; 4] = [70u8, 228u8, 225u8, 191u8];
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
    /**Function with signature `strategyBeacon()` and selector `0xf0062d9a`.
```solidity
function strategyBeacon() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyBeaconCall {}
    ///Container type for the return parameters of the [`strategyBeacon()`](strategyBeaconCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyBeaconReturn {
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
            impl ::core::convert::From<strategyBeaconCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyBeaconCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyBeaconCall {
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
            impl ::core::convert::From<strategyBeaconReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyBeaconReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyBeaconReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyBeaconCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyBeaconReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyBeacon()";
            const SELECTOR: [u8; 4] = [240u8, 6u8, 45u8, 154u8];
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
    /**Function with signature `strategyFactory()` and selector `0x9ef35710`.
```solidity
function strategyFactory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyFactoryCall {}
    ///Container type for the return parameters of the [`strategyFactory()`](strategyFactoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyFactoryReturn {
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
            impl ::core::convert::From<strategyFactoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyFactoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyFactoryCall {
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
            impl ::core::convert::From<strategyFactoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyFactoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyFactoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyFactoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyFactoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyFactory()";
            const SELECTOR: [u8; 4] = [158u8, 243u8, 87u8, 16u8];
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
    /**Function with signature `strategyFactoryBeaconImplementation()` and selector `0x00919afe`.
```solidity
function strategyFactoryBeaconImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyFactoryBeaconImplementationCall {}
    ///Container type for the return parameters of the [`strategyFactoryBeaconImplementation()`](strategyFactoryBeaconImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyFactoryBeaconImplementationReturn {
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
            impl ::core::convert::From<strategyFactoryBeaconImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyFactoryBeaconImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyFactoryBeaconImplementationCall {
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
            impl ::core::convert::From<strategyFactoryBeaconImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyFactoryBeaconImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyFactoryBeaconImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyFactoryBeaconImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyFactoryBeaconImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyFactoryBeaconImplementation()";
            const SELECTOR: [u8; 4] = [0u8, 145u8, 154u8, 254u8];
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
    /**Function with signature `strategyFactoryImplementation()` and selector `0x1e2d334b`.
```solidity
function strategyFactoryImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyFactoryImplementationCall {}
    ///Container type for the return parameters of the [`strategyFactoryImplementation()`](strategyFactoryImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyFactoryImplementationReturn {
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
            impl ::core::convert::From<strategyFactoryImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyFactoryImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyFactoryImplementationCall {
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
            impl ::core::convert::From<strategyFactoryImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyFactoryImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyFactoryImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyFactoryImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyFactoryImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyFactoryImplementation()";
            const SELECTOR: [u8; 4] = [30u8, 45u8, 51u8, 75u8];
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
    /**Function with signature `strategyManagerImplementation()` and selector `0xc1daca80`.
```solidity
function strategyManagerImplementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerImplementationCall {}
    ///Container type for the return parameters of the [`strategyManagerImplementation()`](strategyManagerImplementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerImplementationReturn {
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
            impl ::core::convert::From<strategyManagerImplementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerImplementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyManagerImplementationCall {
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
            impl ::core::convert::From<strategyManagerImplementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerImplementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyManagerImplementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyManagerImplementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyManagerImplementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyManagerImplementation()";
            const SELECTOR: [u8; 4] = [193u8, 218u8, 202u8, 128u8];
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
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
        >,
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
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
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
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
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
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
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
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        pub targetedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
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
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
        >,
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
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
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
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        pub targetedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
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
    /**Function with signature `tokenProxyAdmin()` and selector `0xf2ebb0b6`.
```solidity
function tokenProxyAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenProxyAdminCall {}
    ///Container type for the return parameters of the [`tokenProxyAdmin()`](tokenProxyAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenProxyAdminReturn {
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
            impl ::core::convert::From<tokenProxyAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenProxyAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenProxyAdminCall {
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
            impl ::core::convert::From<tokenProxyAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: tokenProxyAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for tokenProxyAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenProxyAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenProxyAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenProxyAdmin()";
            const SELECTOR: [u8; 4] = [242u8, 235u8, 176u8, 182u8];
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
    ///Container for all the [`ExistingDeploymentParser`](self) function calls.
    pub enum ExistingDeploymentParserCalls {
        EIGEN(EIGENCall),
        EIGENImpl(EIGENImplCall),
        IS_SCRIPT(IS_SCRIPTCall),
        IS_TEST(IS_TESTCall),
        allEigenPods(allEigenPodsCall),
        allocationManager(allocationManagerCall),
        allocationManagerImplementation(allocationManagerImplementationCall),
        avsDirectory(avsDirectoryCall),
        avsDirectoryImplementation(avsDirectoryImplementationCall),
        bEIGEN(bEIGENCall),
        bEIGENImpl(bEIGENImplCall),
        baseStrategyImplementation(baseStrategyImplementationCall),
        delegationManager(delegationManagerCall),
        delegationManagerImplementation(delegationManagerImplementationCall),
        deployedStrategyArray(deployedStrategyArrayCall),
        eigenLayerPauserReg(eigenLayerPauserRegCall),
        eigenLayerProxyAdmin(eigenLayerProxyAdminCall),
        eigenPodBeacon(eigenPodBeaconCall),
        eigenPodImplementation(eigenPodImplementationCall),
        eigenPodManager(eigenPodManagerCall),
        eigenPodManagerImplementation(eigenPodManagerImplementationCall),
        eigenStrategy(eigenStrategyCall),
        eigenStrategyImpl(eigenStrategyImplCall),
        emptyContract(emptyContractCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        inActivePods(inActivePodsCall),
        logAndOutputContractAddresses(logAndOutputContractAddressesCall),
        logInitialDeploymentParams(logInitialDeploymentParamsCall),
        multiValidatorPods(multiValidatorPodsCall),
        rewardsCoordinator(rewardsCoordinatorCall),
        rewardsCoordinatorImplementation(rewardsCoordinatorImplementationCall),
        singleValidatorPods(singleValidatorPodsCall),
        strategiesToDeploy(strategiesToDeployCall),
        strategyBeacon(strategyBeaconCall),
        strategyFactory(strategyFactoryCall),
        strategyFactoryBeaconImplementation(strategyFactoryBeaconImplementationCall),
        strategyFactoryImplementation(strategyFactoryImplementationCall),
        strategyManager(strategyManagerCall),
        strategyManagerImplementation(strategyManagerImplementationCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        tokenProxyAdmin(tokenProxyAdminCall),
    }
    #[automatically_derived]
    impl ExistingDeploymentParserCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 145u8, 154u8, 254u8],
            [4u8, 146u8, 244u8, 188u8],
            [30u8, 45u8, 51u8, 75u8],
            [30u8, 215u8, 131u8, 28u8],
            [33u8, 203u8, 62u8, 55u8],
            [38u8, 137u8, 99u8, 99u8],
            [41u8, 43u8, 123u8, 43u8],
            [50u8, 192u8, 133u8, 133u8],
            [57u8, 183u8, 14u8, 56u8],
            [62u8, 43u8, 238u8, 59u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 72u8, 63u8, 250u8],
            [63u8, 77u8, 164u8, 198u8],
            [63u8, 114u8, 134u8, 244u8],
            [70u8, 101u8, 188u8, 218u8],
            [70u8, 228u8, 225u8, 191u8],
            [71u8, 201u8, 77u8, 218u8],
            [81u8, 110u8, 40u8, 40u8],
            [82u8, 49u8, 86u8, 64u8],
            [93u8, 168u8, 180u8, 206u8],
            [102u8, 217u8, 169u8, 160u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 66u8, 199u8, 80u8],
            [113u8, 197u8, 108u8, 50u8],
            [133u8, 34u8, 108u8, 129u8],
            [138u8, 47u8, 196u8, 227u8],
            [145u8, 106u8, 23u8, 198u8],
            [153u8, 193u8, 239u8, 43u8],
            [158u8, 243u8, 87u8, 16u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [186u8, 140u8, 101u8, 216u8],
            [190u8, 91u8, 181u8, 246u8],
            [193u8, 218u8, 202u8, 128u8],
            [202u8, 138u8, 167u8, 199u8],
            [208u8, 175u8, 38u8, 225u8],
            [219u8, 77u8, 247u8, 97u8],
            [226u8, 12u8, 159u8, 113u8],
            [227u8, 168u8, 179u8, 69u8],
            [231u8, 172u8, 85u8, 252u8],
            [234u8, 77u8, 60u8, 155u8],
            [240u8, 6u8, 45u8, 154u8],
            [242u8, 235u8, 176u8, 182u8],
            [243u8, 158u8, 145u8, 96u8],
            [247u8, 231u8, 110u8, 54u8],
            [248u8, 204u8, 191u8, 71u8],
            [250u8, 118u8, 38u8, 212u8],
            [253u8, 195u8, 113u8, 206u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ExistingDeploymentParserCalls {
        const NAME: &'static str = "ExistingDeploymentParserCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 48usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EIGEN(_) => <EIGENCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::EIGENImpl(_) => {
                    <EIGENImplCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_SCRIPT(_) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allEigenPods(_) => {
                    <allEigenPodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManagerImplementation(_) => {
                    <allocationManagerImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectoryImplementation(_) => {
                    <avsDirectoryImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bEIGEN(_) => <bEIGENCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bEIGENImpl(_) => {
                    <bEIGENImplCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::baseStrategyImplementation(_) => {
                    <baseStrategyImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationManager(_) => {
                    <delegationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationManagerImplementation(_) => {
                    <delegationManagerImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deployedStrategyArray(_) => {
                    <deployedStrategyArrayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenLayerPauserReg(_) => {
                    <eigenLayerPauserRegCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenLayerProxyAdmin(_) => {
                    <eigenLayerProxyAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodBeacon(_) => {
                    <eigenPodBeaconCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodImplementation(_) => {
                    <eigenPodImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManagerImplementation(_) => {
                    <eigenPodManagerImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenStrategy(_) => {
                    <eigenStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenStrategyImpl(_) => {
                    <eigenStrategyImplCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::emptyContract(_) => {
                    <emptyContractCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::inActivePods(_) => {
                    <inActivePodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::logAndOutputContractAddresses(_) => {
                    <logAndOutputContractAddressesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::logInitialDeploymentParams(_) => {
                    <logInitialDeploymentParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multiValidatorPods(_) => {
                    <multiValidatorPodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsCoordinator(_) => {
                    <rewardsCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsCoordinatorImplementation(_) => {
                    <rewardsCoordinatorImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::singleValidatorPods(_) => {
                    <singleValidatorPodsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategiesToDeploy(_) => {
                    <strategiesToDeployCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyBeacon(_) => {
                    <strategyBeaconCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyFactory(_) => {
                    <strategyFactoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyFactoryBeaconImplementation(_) => {
                    <strategyFactoryBeaconImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyFactoryImplementation(_) => {
                    <strategyFactoryImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManager(_) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManagerImplementation(_) => {
                    <strategyManagerImplementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tokenProxyAdmin(_) => {
                    <tokenProxyAdminCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls>] = &[
                {
                    fn strategyFactoryBeaconImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategyFactoryBeaconImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::strategyFactoryBeaconImplementation,
                            )
                    }
                    strategyFactoryBeaconImplementation
                },
                {
                    fn EIGENImpl(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <EIGENImplCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::EIGENImpl)
                    }
                    EIGENImpl
                },
                {
                    fn strategyFactoryImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategyFactoryImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::strategyFactoryImplementation,
                            )
                    }
                    strategyFactoryImplementation
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn eigenStrategyImpl(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenStrategyImplCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenStrategyImpl)
                    }
                    eigenStrategyImpl
                },
                {
                    fn bEIGENImpl(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <bEIGENImplCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::bEIGENImpl)
                    }
                    bEIGENImpl
                },
                {
                    fn eigenPodBeacon(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenPodBeaconCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenPodBeacon)
                    }
                    eigenPodBeacon
                },
                {
                    fn allocationManagerImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <allocationManagerImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::allocationManagerImplementation,
                            )
                    }
                    allocationManagerImplementation
                },
                {
                    fn strategyManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategyManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::strategyManager)
                    }
                    strategyManager
                },
                {
                    fn avsDirectoryImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <avsDirectoryImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::avsDirectoryImplementation,
                            )
                    }
                    avsDirectoryImplementation
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn singleValidatorPods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <singleValidatorPodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::singleValidatorPods)
                    }
                    singleValidatorPods
                },
                {
                    fn bEIGEN(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <bEIGENCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::bEIGEN)
                    }
                    bEIGEN
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn strategiesToDeploy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategiesToDeployCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::strategiesToDeploy)
                    }
                    strategiesToDeploy
                },
                {
                    fn inActivePods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <inActivePodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::inActivePods)
                    }
                    inActivePods
                },
                {
                    fn logAndOutputContractAddresses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <logAndOutputContractAddressesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::logAndOutputContractAddresses,
                            )
                    }
                    logAndOutputContractAddresses
                },
                {
                    fn allEigenPods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <allEigenPodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::allEigenPods)
                    }
                    allEigenPods
                },
                {
                    fn logInitialDeploymentParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <logInitialDeploymentParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::logInitialDeploymentParams,
                            )
                    }
                    logInitialDeploymentParams
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn eigenLayerPauserReg(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenLayerPauserRegCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenLayerPauserReg)
                    }
                    eigenLayerPauserReg
                },
                {
                    fn rewardsCoordinatorImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <rewardsCoordinatorImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::rewardsCoordinatorImplementation,
                            )
                    }
                    rewardsCoordinatorImplementation
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn rewardsCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <rewardsCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::rewardsCoordinator)
                    }
                    rewardsCoordinator
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn baseStrategyImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <baseStrategyImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::baseStrategyImplementation,
                            )
                    }
                    baseStrategyImplementation
                },
                {
                    fn strategyFactory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategyFactoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::strategyFactory)
                    }
                    strategyFactory
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::failed)
                    }
                    failed
                },
                {
                    fn multiValidatorPods(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <multiValidatorPodsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::multiValidatorPods)
                    }
                    multiValidatorPods
                },
                {
                    fn delegationManagerImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <delegationManagerImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::delegationManagerImplementation,
                            )
                    }
                    delegationManagerImplementation
                },
                {
                    fn strategyManagerImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategyManagerImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::strategyManagerImplementation,
                            )
                    }
                    strategyManagerImplementation
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn eigenLayerProxyAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenLayerProxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenLayerProxyAdmin)
                    }
                    eigenLayerProxyAdmin
                },
                {
                    fn eigenStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenStrategy)
                    }
                    eigenStrategy
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn emptyContract(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <emptyContractCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::emptyContract)
                    }
                    emptyContract
                },
                {
                    fn deployedStrategyArray(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <deployedStrategyArrayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::deployedStrategyArray)
                    }
                    deployedStrategyArray
                },
                {
                    fn delegationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <delegationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::delegationManager)
                    }
                    delegationManager
                },
                {
                    fn strategyBeacon(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <strategyBeaconCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::strategyBeacon)
                    }
                    strategyBeacon
                },
                {
                    fn tokenProxyAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <tokenProxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::tokenProxyAdmin)
                    }
                    tokenProxyAdmin
                },
                {
                    fn eigenPodManagerImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenPodManagerImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ExistingDeploymentParserCalls::eigenPodManagerImplementation,
                            )
                    }
                    eigenPodManagerImplementation
                },
                {
                    fn eigenPodImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <eigenPodImplementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::eigenPodImplementation)
                    }
                    eigenPodImplementation
                },
                {
                    fn IS_SCRIPT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::IS_SCRIPT)
                    }
                    IS_SCRIPT
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn EIGEN(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ExistingDeploymentParserCalls> {
                        <EIGENCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ExistingDeploymentParserCalls::EIGEN)
                    }
                    EIGEN
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
                Self::EIGEN(inner) => {
                    <EIGENCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::EIGENImpl(inner) => {
                    <EIGENImplCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::allEigenPods(inner) => {
                    <allEigenPodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManagerImplementation(inner) => {
                    <allocationManagerImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectoryImplementation(inner) => {
                    <avsDirectoryImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bEIGEN(inner) => {
                    <bEIGENCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGENImpl(inner) => {
                    <bEIGENImplCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::baseStrategyImplementation(inner) => {
                    <baseStrategyImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationManager(inner) => {
                    <delegationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationManagerImplementation(inner) => {
                    <delegationManagerImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deployedStrategyArray(inner) => {
                    <deployedStrategyArrayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenLayerPauserReg(inner) => {
                    <eigenLayerPauserRegCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenLayerProxyAdmin(inner) => {
                    <eigenLayerProxyAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodBeacon(inner) => {
                    <eigenPodBeaconCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodImplementation(inner) => {
                    <eigenPodImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManagerImplementation(inner) => {
                    <eigenPodManagerImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenStrategy(inner) => {
                    <eigenStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenStrategyImpl(inner) => {
                    <eigenStrategyImplCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::emptyContract(inner) => {
                    <emptyContractCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::inActivePods(inner) => {
                    <inActivePodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::logAndOutputContractAddresses(inner) => {
                    <logAndOutputContractAddressesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::logInitialDeploymentParams(inner) => {
                    <logInitialDeploymentParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::multiValidatorPods(inner) => {
                    <multiValidatorPodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsCoordinator(inner) => {
                    <rewardsCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsCoordinatorImplementation(inner) => {
                    <rewardsCoordinatorImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::singleValidatorPods(inner) => {
                    <singleValidatorPodsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategiesToDeploy(inner) => {
                    <strategiesToDeployCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyBeacon(inner) => {
                    <strategyBeaconCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyFactory(inner) => {
                    <strategyFactoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyFactoryBeaconImplementation(inner) => {
                    <strategyFactoryBeaconImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyFactoryImplementation(inner) => {
                    <strategyFactoryImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManagerImplementation(inner) => {
                    <strategyManagerImplementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::tokenProxyAdmin(inner) => {
                    <tokenProxyAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EIGEN(inner) => {
                    <EIGENCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::EIGENImpl(inner) => {
                    <EIGENImplCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allEigenPods(inner) => {
                    <allEigenPodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManagerImplementation(inner) => {
                    <allocationManagerImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectoryImplementation(inner) => {
                    <avsDirectoryImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bEIGEN(inner) => {
                    <bEIGENCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bEIGENImpl(inner) => {
                    <bEIGENImplCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::baseStrategyImplementation(inner) => {
                    <baseStrategyImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationManager(inner) => {
                    <delegationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationManagerImplementation(inner) => {
                    <delegationManagerImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deployedStrategyArray(inner) => {
                    <deployedStrategyArrayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenLayerPauserReg(inner) => {
                    <eigenLayerPauserRegCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenLayerProxyAdmin(inner) => {
                    <eigenLayerProxyAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodBeacon(inner) => {
                    <eigenPodBeaconCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodImplementation(inner) => {
                    <eigenPodImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodManagerImplementation(inner) => {
                    <eigenPodManagerImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenStrategy(inner) => {
                    <eigenStrategyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenStrategyImpl(inner) => {
                    <eigenStrategyImplCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::emptyContract(inner) => {
                    <emptyContractCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::inActivePods(inner) => {
                    <inActivePodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::logAndOutputContractAddresses(inner) => {
                    <logAndOutputContractAddressesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::logInitialDeploymentParams(inner) => {
                    <logInitialDeploymentParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multiValidatorPods(inner) => {
                    <multiValidatorPodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardsCoordinator(inner) => {
                    <rewardsCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardsCoordinatorImplementation(inner) => {
                    <rewardsCoordinatorImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::singleValidatorPods(inner) => {
                    <singleValidatorPodsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategiesToDeploy(inner) => {
                    <strategiesToDeployCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyBeacon(inner) => {
                    <strategyBeaconCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyFactory(inner) => {
                    <strategyFactoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyFactoryBeaconImplementation(inner) => {
                    <strategyFactoryBeaconImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyFactoryImplementation(inner) => {
                    <strategyFactoryImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::strategyManagerImplementation(inner) => {
                    <strategyManagerImplementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::tokenProxyAdmin(inner) => {
                    <tokenProxyAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ExistingDeploymentParser`](self) events.
    pub enum ExistingDeploymentParserEvents {
        log(log),
        log_address(log_address),
        log_array_0(log_array_0),
        log_array_1(log_array_1),
        log_array_2(log_array_2),
        log_bytes(log_bytes),
        log_bytes32(log_bytes32),
        log_int(log_int),
        log_named_address(log_named_address),
        log_named_array_0(log_named_array_0),
        log_named_array_1(log_named_array_1),
        log_named_array_2(log_named_array_2),
        log_named_bytes(log_named_bytes),
        log_named_bytes32(log_named_bytes32),
        log_named_decimal_int(log_named_decimal_int),
        log_named_decimal_uint(log_named_decimal_uint),
        log_named_int(log_named_int),
        log_named_string(log_named_string),
        log_named_uint(log_named_uint),
        log_string(log_string),
        log_uint(log_uint),
        logs(logs),
    }
    #[automatically_derived]
    impl ExistingDeploymentParserEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ],
            [
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ],
            [
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ],
            [
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ],
            [
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ],
            [
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ],
            [
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ],
            [
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ],
            [
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ],
            [
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ],
            [
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ],
            [
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ],
            [
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ],
            [
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ],
            [
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ],
            [
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ],
            [
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ],
            [
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ],
            [
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ],
            [
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ],
            [
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ],
            [
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ExistingDeploymentParserEvents {
        const NAME: &'static str = "ExistingDeploymentParserEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for ExistingDeploymentParserEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ExistingDeploymentParser`](self) contract instance.

See the [wrapper's documentation](`ExistingDeploymentParserInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ExistingDeploymentParserInstance<T, P, N> {
        ExistingDeploymentParserInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ExistingDeploymentParserInstance<T, P, N>>,
    > {
        ExistingDeploymentParserInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        ExistingDeploymentParserInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`ExistingDeploymentParser`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ExistingDeploymentParser`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ExistingDeploymentParserInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ExistingDeploymentParserInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ExistingDeploymentParserInstance")
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
    > ExistingDeploymentParserInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ExistingDeploymentParser`](self) contract instance.

See the [wrapper's documentation](`ExistingDeploymentParserInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ExistingDeploymentParserInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> ExistingDeploymentParserInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ExistingDeploymentParserInstance<T, P, N> {
            ExistingDeploymentParserInstance {
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
    > ExistingDeploymentParserInstance<T, P, N> {
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
        ///Creates a new call builder for the [`EIGEN`] function.
        pub fn EIGEN(&self) -> alloy_contract::SolCallBuilder<T, &P, EIGENCall, N> {
            self.call_builder(&EIGENCall {})
        }
        ///Creates a new call builder for the [`EIGENImpl`] function.
        pub fn EIGENImpl(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EIGENImplCall, N> {
            self.call_builder(&EIGENImplCall {})
        }
        ///Creates a new call builder for the [`IS_SCRIPT`] function.
        pub fn IS_SCRIPT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, IS_SCRIPTCall, N> {
            self.call_builder(&IS_SCRIPTCall {})
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`allEigenPods`] function.
        pub fn allEigenPods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, allEigenPodsCall, N> {
            self.call_builder(&allEigenPodsCall { _0 })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`allocationManagerImplementation`] function.
        pub fn allocationManagerImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            allocationManagerImplementationCall,
            N,
        > {
            self.call_builder(
                &allocationManagerImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`avsDirectoryImplementation`] function.
        pub fn avsDirectoryImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryImplementationCall, N> {
            self.call_builder(&avsDirectoryImplementationCall {})
        }
        ///Creates a new call builder for the [`bEIGEN`] function.
        pub fn bEIGEN(&self) -> alloy_contract::SolCallBuilder<T, &P, bEIGENCall, N> {
            self.call_builder(&bEIGENCall {})
        }
        ///Creates a new call builder for the [`bEIGENImpl`] function.
        pub fn bEIGENImpl(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, bEIGENImplCall, N> {
            self.call_builder(&bEIGENImplCall {})
        }
        ///Creates a new call builder for the [`baseStrategyImplementation`] function.
        pub fn baseStrategyImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, baseStrategyImplementationCall, N> {
            self.call_builder(&baseStrategyImplementationCall {})
        }
        ///Creates a new call builder for the [`delegationManager`] function.
        pub fn delegationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationManagerCall, N> {
            self.call_builder(&delegationManagerCall {})
        }
        ///Creates a new call builder for the [`delegationManagerImplementation`] function.
        pub fn delegationManagerImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            delegationManagerImplementationCall,
            N,
        > {
            self.call_builder(
                &delegationManagerImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`deployedStrategyArray`] function.
        pub fn deployedStrategyArray(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deployedStrategyArrayCall, N> {
            self.call_builder(&deployedStrategyArrayCall { _0 })
        }
        ///Creates a new call builder for the [`eigenLayerPauserReg`] function.
        pub fn eigenLayerPauserReg(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenLayerPauserRegCall, N> {
            self.call_builder(&eigenLayerPauserRegCall {})
        }
        ///Creates a new call builder for the [`eigenLayerProxyAdmin`] function.
        pub fn eigenLayerProxyAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenLayerProxyAdminCall, N> {
            self.call_builder(&eigenLayerProxyAdminCall {})
        }
        ///Creates a new call builder for the [`eigenPodBeacon`] function.
        pub fn eigenPodBeacon(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodBeaconCall, N> {
            self.call_builder(&eigenPodBeaconCall {})
        }
        ///Creates a new call builder for the [`eigenPodImplementation`] function.
        pub fn eigenPodImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodImplementationCall, N> {
            self.call_builder(&eigenPodImplementationCall {})
        }
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall {})
        }
        ///Creates a new call builder for the [`eigenPodManagerImplementation`] function.
        pub fn eigenPodManagerImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            eigenPodManagerImplementationCall,
            N,
        > {
            self.call_builder(
                &eigenPodManagerImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`eigenStrategy`] function.
        pub fn eigenStrategy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenStrategyCall, N> {
            self.call_builder(&eigenStrategyCall {})
        }
        ///Creates a new call builder for the [`eigenStrategyImpl`] function.
        pub fn eigenStrategyImpl(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenStrategyImplCall, N> {
            self.call_builder(&eigenStrategyImplCall {})
        }
        ///Creates a new call builder for the [`emptyContract`] function.
        pub fn emptyContract(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, emptyContractCall, N> {
            self.call_builder(&emptyContractCall {})
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall {})
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`inActivePods`] function.
        pub fn inActivePods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, inActivePodsCall, N> {
            self.call_builder(&inActivePodsCall { _0 })
        }
        ///Creates a new call builder for the [`logAndOutputContractAddresses`] function.
        pub fn logAndOutputContractAddresses(
            &self,
            outputPath: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            logAndOutputContractAddressesCall,
            N,
        > {
            self.call_builder(
                &logAndOutputContractAddressesCall {
                    outputPath,
                },
            )
        }
        ///Creates a new call builder for the [`logInitialDeploymentParams`] function.
        pub fn logInitialDeploymentParams(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, logInitialDeploymentParamsCall, N> {
            self.call_builder(&logInitialDeploymentParamsCall {})
        }
        ///Creates a new call builder for the [`multiValidatorPods`] function.
        pub fn multiValidatorPods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, multiValidatorPodsCall, N> {
            self.call_builder(&multiValidatorPodsCall { _0 })
        }
        ///Creates a new call builder for the [`rewardsCoordinator`] function.
        pub fn rewardsCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsCoordinatorCall, N> {
            self.call_builder(&rewardsCoordinatorCall {})
        }
        ///Creates a new call builder for the [`rewardsCoordinatorImplementation`] function.
        pub fn rewardsCoordinatorImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            rewardsCoordinatorImplementationCall,
            N,
        > {
            self.call_builder(
                &rewardsCoordinatorImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`singleValidatorPods`] function.
        pub fn singleValidatorPods(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, singleValidatorPodsCall, N> {
            self.call_builder(&singleValidatorPodsCall { _0 })
        }
        ///Creates a new call builder for the [`strategiesToDeploy`] function.
        pub fn strategiesToDeploy(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategiesToDeployCall, N> {
            self.call_builder(&strategiesToDeployCall { _0 })
        }
        ///Creates a new call builder for the [`strategyBeacon`] function.
        pub fn strategyBeacon(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyBeaconCall, N> {
            self.call_builder(&strategyBeaconCall {})
        }
        ///Creates a new call builder for the [`strategyFactory`] function.
        pub fn strategyFactory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyFactoryCall, N> {
            self.call_builder(&strategyFactoryCall {})
        }
        ///Creates a new call builder for the [`strategyFactoryBeaconImplementation`] function.
        pub fn strategyFactoryBeaconImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            strategyFactoryBeaconImplementationCall,
            N,
        > {
            self.call_builder(
                &strategyFactoryBeaconImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`strategyFactoryImplementation`] function.
        pub fn strategyFactoryImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            strategyFactoryImplementationCall,
            N,
        > {
            self.call_builder(
                &strategyFactoryImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`strategyManager`] function.
        pub fn strategyManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyManagerCall, N> {
            self.call_builder(&strategyManagerCall {})
        }
        ///Creates a new call builder for the [`strategyManagerImplementation`] function.
        pub fn strategyManagerImplementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            strategyManagerImplementationCall,
            N,
        > {
            self.call_builder(
                &strategyManagerImplementationCall {
                },
            )
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall {})
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`tokenProxyAdmin`] function.
        pub fn tokenProxyAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenProxyAdminCall, N> {
            self.call_builder(&tokenProxyAdminCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ExistingDeploymentParserInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<T, &P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<T, &P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<T, &P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<T, &P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
