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

interface bEIGEN_upgrade {
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

    function EIGEN_addressBefore() external view returns (address);
    function IS_SCRIPT() external view returns (bool);
    function IS_TEST() external view returns (bool);
    function bEIGEN_ProxyAdmin() external view returns (address);
    function bEIGEN_TimelockAdmin() external view returns (address);
    function bEIGEN_TimelockController() external view returns (address);
    function bEIGEN_implementation() external view returns (address);
    function bEIGEN_proxy() external view returns (address);
    function checkUpgradeCorrectness() external;
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external returns (bool);
    function run() external;
    function simulatePerformingUpgrade() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "EIGEN_addressBefore",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC20"
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
    "name": "bEIGEN_ProxyAdmin",
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
    "name": "bEIGEN_TimelockAdmin",
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
    "name": "bEIGEN_TimelockController",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract TimelockController"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bEIGEN_implementation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract BackingEigen"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bEIGEN_proxy",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract BackingEigen"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkUpgradeCorrectness",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "run",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "simulatePerformingUpgrade",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod bEIGEN_upgrade {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60806040526000805460ff199081166001908117909255600480549091169091179055601b8054747109709ecfa91a80626ff3989d68f67f5b1dd12d016001600160a81b0319909116179055601c80546001600160a01b03199081167383e9115d334d248ce39a6f36144aeab5b3456e7517909155601e80548216733f5ab2d4418d38568705bfd6672630fcc3435cc9179055601f8054821673d6ec41e453c5e7da5494f4d51a053ab571712e6f1790556020805490911673bb00dda2832850a43840a3a86515e3fe226865f217905534801560da57600080fd5b5061430f806100ea6000396000f3fe608060405234801561001057600080fd5b506004361061012c5760003560e01c8063916a17c6116100ad578063c3ea3fc911610071578063c3ea3fc91461022f578063d715bc6e14610242578063e20c9f7114610255578063f8ccbf471461025d578063fa7626d41461026a57600080fd5b8063916a17c6146101ec578063a5617cfd146101f4578063b5508aa914610207578063ba414fa61461020f578063c04062261461022757600080fd5b80633f7286f4116100f45780633f7286f41461019f57806354755b99146101a757806366d9a9a0146101af578063811ffeeb146101c457806385226c81146101d757600080fd5b806314f8ffac146101315780631ed7831c1461013b57806335e4a03614610159578063370c30d9146101845780633e5e3c2314610197575b600080fd5b610139610277565b005b610143610491565b604051610150919061116b565b60405180910390f35b60205461016c906001600160a01b031681565b6040516001600160a01b039091168152602001610150565b60215461016c906001600160a01b031681565b6101436104f3565b610143610553565b6101396105b3565b6101b76108e2565b60405161015091906111b7565b601f5461016c906001600160a01b031681565b6101df6109d1565b60405161015091906112c1565b6101b7610aa1565b601c5461016c906001600160a01b031681565b6101df610b87565b610217610c57565b6040519015158152602001610150565b610139610d82565b601d5461016c906001600160a01b031681565b601e5461016c906001600160a01b031681565b6101436110fe565b601b546102179060ff1681565b6000546102179060ff1681565b601f5460405163ca669fa760e01b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ca669fa790602401600060405180830381600087803b1580156102d057600080fd5b505af11580156102e4573d6000803e3d6000fd5b5050601d54601e54601c546040516310270e3d60e11b81526001600160a01b039182166004820152928116945016915063204e1c7a90602401602060405180830381865afa15801561033a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061035e919061132f565b6001600160a01b0316146103b95760405162461bcd60e51b815260206004820152601e60248201527f696d706c656d656e746174696f6e2073657420696e636f72726563746c79000060448201526064015b60405180910390fd5b602154601c5460408051637ee1b8e760e11b815290516001600160a01b03938416939092169163fdc371ce916004808201926020929091908290030181865afa15801561040a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061042e919061132f565b6001600160a01b03161461048f5760405162461bcd60e51b815260206004820152602260248201527f454947454e2061646472657373206368616e67656420756e65787065637465646044820152616c7960f01b60648201526084016103b0565b565b6060600d8054806020026020016040519081016040528092919081815260200182805480156104e957602002820191906000526020600020905b81546001600160a01b031681526001909101906020018083116104cb575b5050505050905090565b6060600f8054806020026020016040519081016040528092919081815260200182805480156104e9576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116104cb575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156104e9576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116104cb575050505050905090565b601f546040805163793d064960e11b815290516000926001600160a01b03169163f27a0c929160048083019260209291908290030181865afa1580156105fd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106219190611353565b601c54601d54604080516001600160a01b0393841660248201529290911660448084019190915281518084039091018152606490920181526020820180516001600160e01b031663266a23b160e21b17905251919250907fd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18906106a590839061136c565b60405180910390a16020546040516303223eab60e11b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d5690602401600060405180830381600087803b15801561070657600080fd5b505af115801561071a573d6000803e3d6000fd5b5050601f54601e5460405162ea831560e11b81526001600160a01b0392831694506301d5062a935061075d92909116906000908690829081908a9060040161139a565b600060405180830381600087803b15801561077757600080fd5b505af115801561078b573d6000803e3d6000fd5b50737109709ecfa91a80626ff3989d68f67f5b1dd12d925063e5d6bf0291506107b6905084426113df565b6040518263ffffffff1660e01b81526004016107d491815260200190565b600060405180830381600087803b1580156107ee57600080fd5b505af1158015610802573d6000803e3d6000fd5b5050601f54601e5460405163134008d360e01b81526001600160a01b03928316945063134008d393506108449290911690600090869082908190600401611406565b600060405180830381600087803b15801561085e57600080fd5b505af1158015610872573d6000803e3d6000fd5b50505050601b60019054906101000a90046001600160a01b03166001600160a01b03166390c5013b6040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156108c657600080fd5b505af11580156108da573d6000803e3d6000fd5b505050505050565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156109c85760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156109b057602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116109725790505b50505050508152505081526020019060010190610906565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020016000905b828210156109c8578382906000526020600020018054610a1490611441565b80601f0160208091040260200160405190810160405280929190818152602001828054610a4090611441565b8015610a8d5780601f10610a6257610100808354040283529160200191610a8d565b820191906000526020600020905b815481529060010190602001808311610a7057829003601f168201915b5050505050815260200190600101906109f5565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156109c85760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610b6f57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610b315790505b50505050508152505081526020019060010190610ac5565b60606010805480602002602001604051908101604052809291908181526020016000905b828210156109c8578382906000526020600020018054610bca90611441565b80601f0160208091040260200160405190810160405280929190818152602001828054610bf690611441565b8015610c435780601f10610c1857610100808354040283529160200191610c43565b820191906000526020600020905b815481529060010190602001808311610c2657829003601f168201915b505050505081526020019060010190610bab565b60008054610100900460ff1615610c775750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610d7d5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091610d05917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc49160800161147b565b60408051601f1981840301815290829052610d1f916114ac565b6000604051808303816000865af19150503d8060008114610d5c576040519150601f19603f3d011682016040523d82523d6000602084013e610d61565b606091505b5091505080806020019051810190610d7991906114c8565b9150505b919050565b60408051818152601c818301527f596f7520617265206465706c6f79696e67206f6e20436861696e4944000000006060820152466020820181905291517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160018114610e2f5760405162461bcd60e51b815260206004820152601360248201527210da185a5b881b9bdd081cdd5c1c1bdc9d1959606a1b60448201526064016103b0565b601c60009054906101000a90046001600160a01b03166001600160a01b031663fdc371ce6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e82573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ea6919061132f565b602180546001600160a01b0319166001600160a01b0392909216918217905573ec53bf9167f50cdeb3ae105f56099aaab9061f8314610f275760405162461bcd60e51b815260206004820152601860248201527f736f6d657468696e6720686f727269626c792077726f6e67000000000000000060448201526064016103b0565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b0316637fb5297f6040518163ffffffff1660e01b8152600401600060405180830381600087803b158015610f8557600080fd5b505af1158015610f99573d6000803e3d6000fd5b50506021546040516001600160a01b039091169250610fb8915061115e565b6001600160a01b039091168152602001604051809103906000f080158015610fe4573d6000803e3d6000fd5b50601d60006101000a8154816001600160a01b0302191690836001600160a01b031602179055507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b03166376eadd366040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561106957600080fd5b505af115801561107d573d6000803e3d6000fd5b5050601d5460408051818152601581830152743122a4a3a2a72fb4b6b83632b6b2b73a30ba34b7b760591b60608201526001600160a01b039092166020830152517f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f9350908190036080019150a16110f36105b3565b6110fb610277565b50565b6060600c8054806020026020016040519081016040528092919081815260200182805480156104e9576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116104cb575050505050905090565b612def806114eb83390190565b602080825282518282018190526000918401906040840190835b818110156111ac5783516001600160a01b0316835260209384019390920191600101611185565b509095945050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561126557868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b8083101561124d5783516001600160e01b03191682526020938401936001939093019290910190611221565b509650505060209384019391909101906001016111df565b50929695505050505050565b60005b8381101561128c578181015183820152602001611274565b50506000910152565b600081518084526112ad816020860160208601611271565b601f01601f19169290920160200192915050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561126557603f19878603018452611305858351611295565b945060209384019391909101906001016112e9565b6001600160a01b03811681146110fb57600080fd5b60006020828403121561134157600080fd5b815161134c8161131a565b9392505050565b60006020828403121561136557600080fd5b5051919050565b6040815260046040820152636461746160e01b606082015260806020820152600061134c6080830184611295565b60018060a01b038716815285602082015260c0604082015260006113c160c0830187611295565b606083019590955250608081019290925260a0909101529392505050565b8082018082111561140057634e487b7160e01b600052601160045260246000fd5b92915050565b60018060a01b038616815284602082015260a06040820152600061142d60a0830186611295565b606083019490945250608001529392505050565b600181811c9082168061145557607f821691505b60208210810361147557634e487b7160e01b600052602260045260246000fd5b50919050565b6001600160e01b031983168152815160009061149e816004850160208701611271565b919091016004019392505050565b600082516114be818460208701611271565b9190910192915050565b6000602082840312156114da57600080fd5b8151801515811461134c57600080fdfe60a060405234801561001057600080fd5b50604051612def380380612def83398101604081905261002f91610109565b6001600160a01b03811660805261004461004a565b50610139565b600054610100900460ff16156100b65760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811614610107576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b60006020828403121561011b57600080fd5b81516001600160a01b038116811461013257600080fd5b9392505050565b608051612c86610169600039600081816105f801528181610ddf01528181610e0a0152610e350152612c866000f3fe608060405234801561001057600080fd5b50600436106102485760003560e01c80637ecebe001161013b578063aa271e1a116100b8578063dd62ed3e1161007c578063dd62ed3e14610588578063eb415f451461059b578063f1127ed8146105a3578063f2fde38b146105e0578063fdc371ce146105f357600080fd5b8063aa271e1a14610518578063b8c255941461053c578063c3cda5201461054f578063c4d66de814610562578063d505accf1461057557600080fd5b806395d89b41116100ff57806395d89b41146104cd5780639ab24eb0146104d55780639aec4bae146104e8578063a457c2d7146104f2578063a9059cbb1461050557600080fd5b80637ecebe001461045c57806384b0196e1461046f5780638da5cb5b1461048a5780638e539e8c1461049b57806391ddadf4146104ae57600080fd5b806340c10f19116101c957806366eb399f1161018d57806366eb399f146103cc5780636fcfff45146103df57806370a0823114610407578063715018a61461043057806378aa33ba1461043857600080fd5b806340c10f191461032557806342966c68146103385780634bf5d7e91461034b578063587cde1e146103755780635c19a95c146103b957600080fd5b806323b872dd1161021057806323b872dd146102d5578063313ce567146102e85780633644e515146102f757806339509351146102ff5780633a46b1a81461031257600080fd5b80630455e6941461024d57806306fdde0314610286578063095ea7b31461029b57806318160ddd146102ae5780631ffacdef146102c0575b600080fd5b61027161025b36600461270a565b6101316020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b61028e61061a565b60405161027d919061276b565b6102716102a936600461277e565b6106ac565b6067545b60405190815260200161027d565b6102d36102ce3660046127a8565b6106c6565b005b6102716102e33660046127e4565b6106dc565b6040516012815260200161027d565b6102b2610700565b61027161030d36600461277e565b61070f565b6102b261032036600461277e565b610731565b6102d361033336600461277e565b6107bb565b6102d3610346366004612821565b610837565b60408051808201909152600e81526d06d6f64653d74696d657374616d760941b602082015261028e565b6103a161038336600461270a565b6001600160a01b03908116600090815260fe60205260409020541690565b6040516001600160a01b03909116815260200161027d565b6102d36103c736600461270a565b610844565b6102d36103da3660046127a8565b61084e565b6103f26103ed36600461270a565b6108c7565b60405163ffffffff909116815260200161027d565b6102b261041536600461270a565b6001600160a01b031660009081526065602052604090205490565b6102d36108e9565b61027161044636600461270a565b6101326020526000908152604090205460ff1681565b6102b261046a36600461270a565b6108fd565b61047761091b565b60405161027d979695949392919061283a565b6033546001600160a01b03166103a1565b6102b26104a9366004612821565b6109b9565b6104b6610a21565b60405165ffffffffffff909116815260200161027d565b61028e610a2c565b6102b26104e336600461270a565b610a3b565b6102b26101305481565b61027161050036600461277e565b610abd565b61027161051336600461277e565b610b38565b61027161052636600461270a565b6101336020526000908152604090205460ff1681565b6102d361054a3660046127a8565b610b46565b6102d361055d3660046128e3565b610b58565b6102d361057036600461270a565b610c8e565b6102d361058336600461293b565b610ed8565b6102b26105963660046129a6565b61103c565b6102d3611067565b6105b66105b13660046129d9565b611137565b60408051825163ffffffff1681526020928301516001600160e01b0316928101929092520161027d565b6102d36105ee36600461270a565b6111bb565b6103a17f000000000000000000000000000000000000000000000000000000000000000081565b60606068805461062990612a0e565b80601f016020809104026020016040519081016040528092919081815260200182805461065590612a0e565b80156106a25780601f10610677576101008083540402835291602001916106a2565b820191906000526020600020905b81548152906001019060200180831161068557829003601f168201915b5050505050905090565b6000336106ba818585611231565b60019150505b92915050565b6106ce611355565b6106d882826113af565b5050565b6000336106ea858285611410565b6106f585858561148a565b506001949350505050565b600061070a611646565b905090565b6000336106ba818585610722838361103c565b61072c9190612a58565b611231565b600061073b610a21565b65ffffffffffff1682106107925760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064015b60405180910390fd5b6001600160a01b038316600090815260ff602052604090206107b49083611650565b9392505050565b336000908152610133602052604090205460ff1661082d5760405162461bcd60e51b815260206004820152602960248201527f4261636b696e67456967656e2e6d696e743a2063616c6c6572206973206e6f7460448201526810309036b4b73a32b960b91b6064820152608401610789565b6106d88282611739565b61084133826117c4565b50565b61084133826117dd565b610856611355565b816001600160a01b03167f0124b12503bddc2616c0f3f54fd23ed283f5ef0c1483a75409e42612176b8bde82604051610893911515815260200190565b60405180910390a26001600160a01b0391909116600090815261013360205260409020805460ff1916911515919091179055565b6001600160a01b038116600090815260ff60205260408120546106c090611857565b6108f1611355565b6108fb60006118c0565b565b6001600160a01b038116600090815260cb60205260408120546106c0565b6000606080600080600060606097546000801b14801561093b5750609854155b61097f5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b6044820152606401610789565b610987611912565b61098f611921565b60408051600080825260208201909252600f60f81b9b939a50919850469750309650945092509050565b60006109c3610a21565b65ffffffffffff168210610a155760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b6044820152606401610789565b6106c061010083611650565b600061070a42611930565b60606069805461062990612a0e565b6001600160a01b038116600090815260ff60205260408120548015610aaa576001600160a01b038316600090815260ff6020526040902080546000198301908110610a8857610a88612a81565b60009182526020909120015464010000000090046001600160e01b0316610aad565b60005b6001600160e01b03169392505050565b60003381610acb828661103c565b905083811015610b2b5760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401610789565b6106f58286868403611231565b6000336106ba81858561148a565b610b4e611355565b6106d88282611997565b83421115610ba85760405162461bcd60e51b815260206004820152601d60248201527f4552433230566f7465733a207369676e617475726520657870697265640000006044820152606401610789565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b038816918101919091526060810186905260808101859052600090610c2290610c1a9060a001604051602081830303815290604052805190602001206119f0565b858585611a1d565b9050610c2d81611a45565b8614610c7b5760405162461bcd60e51b815260206004820152601960248201527f4552433230566f7465733a20696e76616c6964206e6f6e6365000000000000006044820152606401610789565b610c8581886117dd565b50505050505050565b600054610100900460ff1615808015610cae5750600054600160ff909116105b80610cc85750303b158015610cc8575060005460ff166001145b610d2b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610789565b6000805460ff191660011790558015610d4e576000805461ff0019166101001790555b610d56611a6d565b610da36040518060400160405280600d81526020016c2130b1b5b4b7339022b4b3b2b760991b815250604051806040016040528060068152602001653122a4a3a2a760d11b815250611a9c565b610dac826118c0565b610dd3604051806040016040528060068152602001653122a4a3a2a760d11b815250611acd565b60001961013055610e057f000000000000000000000000000000000000000000000000000000000000000060016113af565b610e307f00000000000000000000000000000000000000000000000000000000000000006001611997565b610e667f00000000000000000000000000000000000000000000000000000000000000006b05686877afb5cbccbf734000611739565b6040517fb7c23c1e2e36f298e9879a88ecfcd07e28fbb439bcfa9c78ca1363ca14370d2690600090a180156106d8576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b83421115610f285760405162461bcd60e51b815260206004820152601d60248201527f45524332305065726d69743a206578706972656420646561646c696e650000006044820152606401610789565b60007f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9888888610f578c611a45565b6040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810186905260e0016040516020818303038152906040528051906020012090506000610fb2826119f0565b90506000610fc282878787611a1d565b9050896001600160a01b0316816001600160a01b0316146110255760405162461bcd60e51b815260206004820152601e60248201527f45524332305065726d69743a20696e76616c6964207369676e617475726500006044820152606401610789565b6110308a8a8a611231565b50505050505050505050565b6001600160a01b03918216600090815260666020908152604080832093909416825291909152205490565b61106f611355565b60001961013054146111065760405162461bcd60e51b815260206004820152605460248201527f4261636b696e67456967656e2e64697361626c655472616e736665725265737460448201527f72696374696f6e733a207472616e73666572207265737472696374696f6e7320606482015273185c9948185b1c9958591e48191a5cd8589b195960621b608482015260a401610789565b60006101308190556040517f2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f649190a1565b60408051808201909152600080825260208201526001600160a01b038316600090815260ff60205260409020805463ffffffff841690811061117b5761117b612a81565b60009182526020918290206040805180820190915291015463ffffffff8116825264010000000090046001600160e01b0316918101919091529392505050565b6111c3611355565b6001600160a01b0381166112285760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610789565b610841816118c0565b6001600160a01b0383166112935760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401610789565b6001600160a01b0382166112f45760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401610789565b6001600160a01b0383811660008181526066602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b6033546001600160a01b031633146108fb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610789565b6001600160a01b03821660008181526101316020908152604091829020805460ff191685151590811790915591519182527fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227191015b60405180910390a25050565b600061141c848461103c565b9050600019811461148457818110156114775760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401610789565b6114848484848403611231565b50505050565b6001600160a01b0383166114ee5760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401610789565b6001600160a01b0382166115505760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401610789565b61155b838383611b17565b6001600160a01b038316600090815260656020526040902054818110156115d35760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401610789565b6001600160a01b0380851660008181526065602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef906116339086815260200190565b60405180910390a3611484848484611bf5565b600061070a611c27565b8154600090818160058111156116aa57600061166b84611c9b565b6116759085612a97565b600088815260209020909150869082015463ffffffff16111561169a578091506116a8565b6116a5816001612a58565b92505b505b808210156116f75760006116be8383611d83565b600088815260209020909150869082015463ffffffff1611156116e3578091506116f1565b6116ee816001612a58565b92505b506116aa565b8015611723576000868152602090208101600019015464010000000090046001600160e01b0316611726565b60005b6001600160e01b03169695505050505050565b6117438282611d9e565b6067546001600160e01b0310156117b55760405162461bcd60e51b815260206004820152603060248201527f4552433230566f7465733a20746f74616c20737570706c79207269736b73206f60448201526f766572666c6f77696e6720766f74657360801b6064820152608401610789565b611484610100611e7383611e7f565b6117ce8282611ff4565b61148461010061213b83611e7f565b6001600160a01b03828116600081815260fe6020818152604080842080546065845282862054949093528787166001600160a01b03198416811790915590519190951694919391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4611484828483612147565b600063ffffffff8211156118bc5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610789565b5090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60606099805461062990612a0e565b6060609a805461062990612a0e565b600065ffffffffffff8211156118bc5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203460448201526538206269747360d01b6064820152608401610789565b6001600160a01b03821660008181526101326020908152604091829020805460ff191685151590811790915591519182527f72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b9101611404565b60006106c06119fd611646565b8360405161190160f01b8152600281019290925260228201526042902090565b6000806000611a2e87878787612284565b91509150611a3b81612348565b5095945050505050565b6001600160a01b038116600090815260cb602052604090208054600181018255905b50919050565b600054610100900460ff16611a945760405162461bcd60e51b815260040161078990612aaa565b6108fb612492565b600054610100900460ff16611ac35760405162461bcd60e51b815260040161078990612aaa565b6106d882826124c2565b600054610100900460ff16611af45760405162461bcd60e51b815260040161078990612aaa565b61084181604051806040016040528060018152602001603160f81b815250612502565b610130544211611bf0576001600160a01b0383166000908152610131602052604090205460ff1680611b6257506001600160a01b0382166000908152610132602052604090205460ff165b80611b7457506001600160a01b038316155b611bf05760405162461bcd60e51b815260206004820152604160248201527f4261636b696e67456967656e2e5f6265666f7265546f6b656e5472616e73666560448201527f723a2066726f6d206f7220746f206d7573742062652077686974656c697374656064820152601960fa1b608482015260a401610789565b505050565b6001600160a01b03838116600090815260fe6020526040808220548584168352912054611bf092918216911683612147565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611c52612551565b611c5a6125aa565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b600081600003611cad57506000919050565b60006001611cba846125db565b901c6001901b90506001818481611cd357611cd3612af5565b048201901c90506001818481611ceb57611ceb612af5565b048201901c90506001818481611d0357611d03612af5565b048201901c90506001818481611d1b57611d1b612af5565b048201901c90506001818481611d3357611d33612af5565b048201901c90506001818481611d4b57611d4b612af5565b048201901c90506001818481611d6357611d63612af5565b048201901c90506107b481828581611d7d57611d7d612af5565b0461266f565b6000611d926002848418612b0b565b6107b490848416612a58565b6001600160a01b038216611df45760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401610789565b611e0060008383611b17565b8060676000828254611e129190612a58565b90915550506001600160a01b0382166000818152606560209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a36106d860008383611bf5565b60006107b48284612a58565b82546000908190818115611ecc5760008781526020902082016000190160408051808201909152905463ffffffff8116825264010000000090046001600160e01b03166020820152611ee1565b60408051808201909152600080825260208201525b905080602001516001600160e01b03169350611f0184868863ffffffff16565b9250600082118015611f2b5750611f16610a21565b65ffffffffffff16816000015163ffffffff16145b15611f7057611f3983612685565b60008881526020902083016000190180546001600160e01b03929092166401000000000263ffffffff909216919091179055611fea565b866040518060400160405280611f94611f87610a21565b65ffffffffffff16611857565b63ffffffff168152602001611fa886612685565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b5050935093915050565b6001600160a01b0382166120545760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401610789565b61206082600083611b17565b6001600160a01b038216600090815260656020526040902054818110156120d45760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401610789565b6001600160a01b03831660008181526065602090815260408083208686039055606780548790039055518581529192917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3611bf083600084611bf5565b60006107b48284612a97565b816001600160a01b0316836001600160a01b0316141580156121695750600081115b15611bf0576001600160a01b038316156121f7576001600160a01b038316600090815260ff6020526040812081906121a49061213b85611e7f565b91509150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516121ec929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615611bf0576001600160a01b038216600090815260ff60205260408120819061222d90611e7385611e7f565b91509150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7248383604051612275929190918252602082015260400190565b60405180910390a25050505050565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156122bb575060009050600361233f565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561230f573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166123385760006001925092505061233f565b9150600090505b94509492505050565b600081600481111561235c5761235c612b2d565b036123645750565b600181600481111561237857612378612b2d565b036123c55760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610789565b60028160048111156123d9576123d9612b2d565b036124265760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610789565b600381600481111561243a5761243a612b2d565b036108415760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610789565b600054610100900460ff166124b95760405162461bcd60e51b815260040161078990612aaa565b6108fb336118c0565b600054610100900460ff166124e95760405162461bcd60e51b815260040161078990612aaa565b60686124f58382612b91565b506069611bf08282612b91565b600054610100900460ff166125295760405162461bcd60e51b815260040161078990612aaa565b60996125358382612b91565b50609a6125428282612b91565b50506000609781905560985550565b60008061255c611912565b805190915015612573578051602090910120919050565b60975480156125825792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b6000806125b5611921565b8051909150156125cc578051602090910120919050565b60985480156125825792915050565b600080608083901c156125f057608092831c92015b604083901c1561260257604092831c92015b602083901c1561261457602092831c92015b601083901c1561262657601092831c92015b600883901c1561263857600892831c92015b600483901c1561264a57600492831c92015b600283901c1561265c57600292831c92015b600183901c156106c05760010192915050565b600081831061267e57816107b4565b5090919050565b60006001600160e01b038211156118bc5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610789565b80356001600160a01b038116811461270557600080fd5b919050565b60006020828403121561271c57600080fd5b6107b4826126ee565b6000815180845260005b8181101561274b5760208185018101518683018201520161272f565b506000602082860101526020601f19601f83011685010191505092915050565b6020815260006107b46020830184612725565b6000806040838503121561279157600080fd5b61279a836126ee565b946020939093013593505050565b600080604083850312156127bb57600080fd5b6127c4836126ee565b9150602083013580151581146127d957600080fd5b809150509250929050565b6000806000606084860312156127f957600080fd5b612802846126ee565b9250612810602085016126ee565b929592945050506040919091013590565b60006020828403121561283357600080fd5b5035919050565b60ff60f81b8816815260e06020820152600061285960e0830189612725565b828103604084015261286b8189612725565b606084018890526001600160a01b038716608085015260a0840186905283810360c08501528451808252602080870193509091019060005b818110156128c15783518352602093840193909201916001016128a3565b50909b9a5050505050505050505050565b803560ff8116811461270557600080fd5b60008060008060008060c087890312156128fc57600080fd5b612905876126ee565b95506020870135945060408701359350612921606088016128d2565b9598949750929560808101359460a0909101359350915050565b600080600080600080600060e0888a03121561295657600080fd5b61295f886126ee565b965061296d602089016126ee565b95506040880135945060608801359350612989608089016128d2565b9699959850939692959460a0840135945060c09093013592915050565b600080604083850312156129b957600080fd5b6129c2836126ee565b91506129d0602084016126ee565b90509250929050565b600080604083850312156129ec57600080fd5b6129f5836126ee565b9150602083013563ffffffff811681146127d957600080fd5b600181811c90821680612a2257607f821691505b602082108103611a6757634e487b7160e01b600052602260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b808201808211156106c0576106c0612a42565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b818103818111156106c0576106c0612a42565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b600052601260045260246000fd5b600082612b2857634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052602160045260246000fd5b601f821115611bf057806000526020600020601f840160051c81016020851015612b6a5750805b601f840160051c820191505b81811015612b8a5760008155600101612b76565b5050505050565b815167ffffffffffffffff811115612bab57612bab612a6b565b612bbf81612bb98454612a0e565b84612b43565b6020601f821160018114612bf35760008315612bdb5750848201515b600019600385901b1c1916600184901b178455612b8a565b600084815260208120601f198516915b82811015612c235787850151825560209485019460019092019101612c03565b5084821015612c415786840151600019600387901b60f8161c191681555b50505050600190811b0190555056fea2646970667358221220ece641739db4ecd9d6c5836deb2fdfa19b3f0cb259540572e9f71711f002422d64736f6c634300081b0033a264697066735822122073b748488a764c806566a10b75297b4e1e0ae839b69da6f59f6eda8242d4973264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\0\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U`\x1B\x80Ttq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x01`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16\x17\x90U`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16s\x83\xE9\x11]3M$\x8C\xE3\x9Ao6\x14J\xEA\xB5\xB3Enu\x17\x90\x91U`\x1E\x80T\x82\x16s?Z\xB2\xD4A\x8D8V\x87\x05\xBF\xD6g&0\xFC\xC3C\\\xC9\x17\x90U`\x1F\x80T\x82\x16s\xD6\xECA\xE4S\xC5\xE7\xDAT\x94\xF4\xD5\x1A\x05:\xB5qq.o\x17\x90U` \x80T\x90\x91\x16s\xBB\0\xDD\xA2\x83(P\xA48@\xA3\xA8e\x15\xE3\xFE\"he\xF2\x17\x90U4\x80\x15`\xDAW`\0\x80\xFD[PaC\x0F\x80a\0\xEA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01,W`\x005`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xADW\x80c\xC3\xEA?\xC9\x11a\0qW\x80c\xC3\xEA?\xC9\x14a\x02/W\x80c\xD7\x15\xBCn\x14a\x02BW\x80c\xE2\x0C\x9Fq\x14a\x02UW\x80c\xF8\xCC\xBFG\x14a\x02]W\x80c\xFAv&\xD4\x14a\x02jW`\0\x80\xFD[\x80c\x91j\x17\xC6\x14a\x01\xECW\x80c\xA5a|\xFD\x14a\x01\xF4W\x80c\xB5P\x8A\xA9\x14a\x02\x07W\x80c\xBAAO\xA6\x14a\x02\x0FW\x80c\xC0@b&\x14a\x02'W`\0\x80\xFD[\x80c?r\x86\xF4\x11a\0\xF4W\x80c?r\x86\xF4\x14a\x01\x9FW\x80cTu[\x99\x14a\x01\xA7W\x80cf\xD9\xA9\xA0\x14a\x01\xAFW\x80c\x81\x1F\xFE\xEB\x14a\x01\xC4W\x80c\x85\"l\x81\x14a\x01\xD7W`\0\x80\xFD[\x80c\x14\xF8\xFF\xAC\x14a\x011W\x80c\x1E\xD7\x83\x1C\x14a\x01;W\x80c5\xE4\xA06\x14a\x01YW\x80c7\x0C0\xD9\x14a\x01\x84W\x80c>^<#\x14a\x01\x97W[`\0\x80\xFD[a\x019a\x02wV[\0[a\x01Ca\x04\x91V[`@Qa\x01P\x91\x90a\x11kV[`@Q\x80\x91\x03\x90\xF3[` Ta\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01PV[`!Ta\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01Ca\x04\xF3V[a\x01Ca\x05SV[a\x019a\x05\xB3V[a\x01\xB7a\x08\xE2V[`@Qa\x01P\x91\x90a\x11\xB7V[`\x1FTa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDFa\t\xD1V[`@Qa\x01P\x91\x90a\x12\xC1V[a\x01\xB7a\n\xA1V[`\x1CTa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDFa\x0B\x87V[a\x02\x17a\x0CWV[`@Q\x90\x15\x15\x81R` \x01a\x01PV[a\x019a\r\x82V[`\x1DTa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1ETa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01Ca\x10\xFEV[`\x1BTa\x02\x17\x90`\xFF\x16\x81V[`\0Ta\x02\x17\x90`\xFF\x16\x81V[`\x1FT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xE4W=`\0\x80>=`\0\xFD[PP`\x1DT`\x1ET`\x1CT`@Qc\x10'\x0E=`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x81\x16\x94P\x16\x91Pc N\x1Cz\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03^\x91\x90a\x13/V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fimplementation set incorrectly\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`!T`\x1CT`@\x80Qc~\xE1\xB8\xE7`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x90\x92\x16\x91c\xFD\xC3q\xCE\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90a\x13/V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEIGEN address changed unexpected`D\x82\x01Raly`\xF0\x1B`d\x82\x01R`\x84\x01a\x03\xB0V[V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBW[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBWPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBWPPPPP\x90P\x90V[`\x1FT`@\x80Qcy=\x06I`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2z\x0C\x92\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06!\x91\x90a\x13SV[`\x1CT`\x1DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x92\x90\x91\x16`D\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c&j#\xB1`\xE2\x1B\x17\x90RQ\x91\x92P\x90\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x90a\x06\xA5\x90\x83\x90a\x13lV[`@Q\x80\x91\x03\x90\xA1` T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x06W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x1AW=`\0\x80>=`\0\xFD[PP`\x1FT`\x1ET`@Qb\xEA\x83\x15`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x01\xD5\x06*\x93Pa\x07]\x92\x90\x91\x16\x90`\0\x90\x86\x90\x82\x90\x81\x90\x8A\x90`\x04\x01a\x13\x9AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07wW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x8BW=`\0\x80>=`\0\xFD[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92Pc\xE5\xD6\xBF\x02\x91Pa\x07\xB6\x90P\x84Ba\x13\xDFV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xD4\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xEEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x02W=`\0\x80>=`\0\xFD[PP`\x1FT`\x1ET`@Qc\x13@\x08\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x13@\x08\xD3\x93Pa\x08D\x92\x90\x91\x16\x90`\0\x90\x86\x90\x82\x90\x81\x90`\x04\x01a\x14\x06V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08rW=`\0\x80>=`\0\xFD[PPPP`\x1B`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xDAW=`\0\x80>=`\0\xFD[PPPPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\t\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\trW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\t\x06V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\x14\x90a\x14AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n@\x90a\x14AV[\x80\x15a\n\x8DW\x80`\x1F\x10a\nbWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x8DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\npW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t\xF5V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0BoW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\xC5V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0B\xCA\x90a\x14AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xF6\x90a\x14AV[\x80\x15a\x0CCW\x80`\x1F\x10a\x0C\x18Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0CCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C&W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0B\xABV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0CwWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r}W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\r\x05\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x14{V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\x1F\x91a\x14\xACV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\\W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\raV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\ry\x91\x90a\x14\xC8V[\x91PP[\x91\x90PV[`@\x80Q\x81\x81R`\x1C\x81\x83\x01R\x7FYou are deploying on ChainID\0\0\0\0``\x82\x01RF` \x82\x01\x81\x90R\x91Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`\x01\x81\x14a\x0E/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10\xDA\x18Z[\x88\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`D\x82\x01R`d\x01a\x03\xB0V[`\x1C`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xC3q\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA6\x91\x90a\x13/V[`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Us\xECS\xBF\x91g\xF5\x0C\xDE\xB3\xAE\x10_V\t\x9A\xAA\xB9\x06\x1F\x83\x14a\x0F'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fsomething horribly wrong\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x99W=`\0\x80>=`\0\xFD[PP`!T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pa\x0F\xB8\x91Pa\x11^V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x0F\xE4W=`\0\x80>=`\0\xFD[P`\x1D`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10iW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10}W=`\0\x80>=`\0\xFD[PP`\x1DT`@\x80Q\x81\x81R`\x15\x81\x83\x01Rt1\"\xA4\xA3\xA2\xA7/\xB4\xB6\xB862\xB6\xB2\xB7:0\xBA4\xB7\xB7`Y\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x93P\x90\x81\x90\x03`\x80\x01\x91P\xA1a\x10\xF3a\x05\xB3V[a\x10\xFBa\x02wV[PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBWPPPPP\x90P\x90V[a-\xEF\x80a\x14\xEB\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x11\xACW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x11\x85V[P\x90\x95\x94PPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x12eW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15a\x12MW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a\x12!V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x11\xDFV[P\x92\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a\x12\x8CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12tV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12\xAD\x81` \x86\x01` \x86\x01a\x12qV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x12eW`?\x19\x87\x86\x03\x01\x84Ra\x13\x05\x85\x83Qa\x12\x95V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x12\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13AW`\0\x80\xFD[\x81Qa\x13L\x81a\x13\x1AV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13eW`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\x04`@\x82\x01Rcdata`\xE0\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13L`\x80\x83\x01\x84a\x12\x95V[`\x01\x80`\xA0\x1B\x03\x87\x16\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0a\x13\xC1`\xC0\x83\x01\x87a\x12\x95V[``\x83\x01\x95\x90\x95RP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x14\0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a\x14-`\xA0\x83\x01\x86a\x12\x95V[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x14UW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14uWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x14\x9E\x81`\x04\x85\x01` \x87\x01a\x12qV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x14\xBE\x81\x84` \x87\x01a\x12qV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\xDAW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13LW`\0\x80\xFD\xFE`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa-\xEF8\x03\x80a-\xEF\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\tV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Ra\0Da\0JV[Pa\x019V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\x01\x07W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15a\x01\x1BW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x012W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa,\x86a\x01i`\09`\0\x81\x81a\x05\xF8\x01R\x81\x81a\r\xDF\x01R\x81\x81a\x0E\n\x01Ra\x0E5\x01Ra,\x86`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02HW`\x005`\xE0\x1C\x80c~\xCE\xBE\0\x11a\x01;W\x80c\xAA'\x1E\x1A\x11a\0\xB8W\x80c\xDDb\xED>\x11a\0|W\x80c\xDDb\xED>\x14a\x05\x88W\x80c\xEBA_E\x14a\x05\x9BW\x80c\xF1\x12~\xD8\x14a\x05\xA3W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE0W\x80c\xFD\xC3q\xCE\x14a\x05\xF3W`\0\x80\xFD[\x80c\xAA'\x1E\x1A\x14a\x05\x18W\x80c\xB8\xC2U\x94\x14a\x05<W\x80c\xC3\xCD\xA5 \x14a\x05OW\x80c\xC4\xD6m\xE8\x14a\x05bW\x80c\xD5\x05\xAC\xCF\x14a\x05uW`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\0\xFFW\x80c\x95\xD8\x9BA\x14a\x04\xCDW\x80c\x9A\xB2N\xB0\x14a\x04\xD5W\x80c\x9A\xECK\xAE\x14a\x04\xE8W\x80c\xA4W\xC2\xD7\x14a\x04\xF2W\x80c\xA9\x05\x9C\xBB\x14a\x05\x05W`\0\x80\xFD[\x80c~\xCE\xBE\0\x14a\x04\\W\x80c\x84\xB0\x19n\x14a\x04oW\x80c\x8D\xA5\xCB[\x14a\x04\x8AW\x80c\x8ES\x9E\x8C\x14a\x04\x9BW\x80c\x91\xDD\xAD\xF4\x14a\x04\xAEW`\0\x80\xFD[\x80c@\xC1\x0F\x19\x11a\x01\xC9W\x80cf\xEB9\x9F\x11a\x01\x8DW\x80cf\xEB9\x9F\x14a\x03\xCCW\x80co\xCF\xFFE\x14a\x03\xDFW\x80cp\xA0\x821\x14a\x04\x07W\x80cqP\x18\xA6\x14a\x040W\x80cx\xAA3\xBA\x14a\x048W`\0\x80\xFD[\x80c@\xC1\x0F\x19\x14a\x03%W\x80cB\x96lh\x14a\x038W\x80cK\xF5\xD7\xE9\x14a\x03KW\x80cX|\xDE\x1E\x14a\x03uW\x80c\\\x19\xA9\\\x14a\x03\xB9W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x02\x10W\x80c#\xB8r\xDD\x14a\x02\xD5W\x80c1<\xE5g\x14a\x02\xE8W\x80c6D\xE5\x15\x14a\x02\xF7W\x80c9P\x93Q\x14a\x02\xFFW\x80c:F\xB1\xA8\x14a\x03\x12W`\0\x80\xFD[\x80c\x04U\xE6\x94\x14a\x02MW\x80c\x06\xFD\xDE\x03\x14a\x02\x86W\x80c\t^\xA7\xB3\x14a\x02\x9BW\x80c\x18\x16\r\xDD\x14a\x02\xAEW\x80c\x1F\xFA\xCD\xEF\x14a\x02\xC0W[`\0\x80\xFD[a\x02qa\x02[6`\x04a'\nV[a\x011` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x8Ea\x06\x1AV[`@Qa\x02}\x91\x90a'kV[a\x02qa\x02\xA96`\x04a'~V[a\x06\xACV[`gT[`@Q\x90\x81R` \x01a\x02}V[a\x02\xD3a\x02\xCE6`\x04a'\xA8V[a\x06\xC6V[\0[a\x02qa\x02\xE36`\x04a'\xE4V[a\x06\xDCV[`@Q`\x12\x81R` \x01a\x02}V[a\x02\xB2a\x07\0V[a\x02qa\x03\r6`\x04a'~V[a\x07\x0FV[a\x02\xB2a\x03 6`\x04a'~V[a\x071V[a\x02\xD3a\x0336`\x04a'~V[a\x07\xBBV[a\x02\xD3a\x03F6`\x04a(!V[a\x087V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\x8EV[a\x03\xA1a\x03\x836`\x04a'\nV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\xFE` R`@\x90 T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02}V[a\x02\xD3a\x03\xC76`\x04a'\nV[a\x08DV[a\x02\xD3a\x03\xDA6`\x04a'\xA8V[a\x08NV[a\x03\xF2a\x03\xED6`\x04a'\nV[a\x08\xC7V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02}V[a\x02\xB2a\x04\x156`\x04a'\nV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`e` R`@\x90 T\x90V[a\x02\xD3a\x08\xE9V[a\x02qa\x04F6`\x04a'\nV[a\x012` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xB2a\x04j6`\x04a'\nV[a\x08\xFDV[a\x04wa\t\x1BV[`@Qa\x02}\x97\x96\x95\x94\x93\x92\x91\x90a(:V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA1V[a\x02\xB2a\x04\xA96`\x04a(!V[a\t\xB9V[a\x04\xB6a\n!V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02}V[a\x02\x8Ea\n,V[a\x02\xB2a\x04\xE36`\x04a'\nV[a\n;V[a\x02\xB2a\x010T\x81V[a\x02qa\x05\x006`\x04a'~V[a\n\xBDV[a\x02qa\x05\x136`\x04a'~V[a\x0B8V[a\x02qa\x05&6`\x04a'\nV[a\x013` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xD3a\x05J6`\x04a'\xA8V[a\x0BFV[a\x02\xD3a\x05]6`\x04a(\xE3V[a\x0BXV[a\x02\xD3a\x05p6`\x04a'\nV[a\x0C\x8EV[a\x02\xD3a\x05\x836`\x04a);V[a\x0E\xD8V[a\x02\xB2a\x05\x966`\x04a)\xA6V[a\x10<V[a\x02\xD3a\x10gV[a\x05\xB6a\x05\xB16`\x04a)\xD9V[a\x117V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02}V[a\x02\xD3a\x05\xEE6`\x04a'\nV[a\x11\xBBV[a\x03\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[```h\x80Ta\x06)\x90a*\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06U\x90a*\x0EV[\x80\x15a\x06\xA2W\x80`\x1F\x10a\x06wWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xA2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x85W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\xBA\x81\x85\x85a\x121V[`\x01\x91PP[\x92\x91PPV[a\x06\xCEa\x13UV[a\x06\xD8\x82\x82a\x13\xAFV[PPV[`\x003a\x06\xEA\x85\x82\x85a\x14\x10V[a\x06\xF5\x85\x85\x85a\x14\x8AV[P`\x01\x94\x93PPPPV[`\0a\x07\na\x16FV[\x90P\x90V[`\x003a\x06\xBA\x81\x85\x85a\x07\"\x83\x83a\x10<V[a\x07,\x91\x90a*XV[a\x121V[`\0a\x07;a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 a\x07\xB4\x90\x83a\x16PV[\x93\x92PPPV[3`\0\x90\x81Ra\x013` R`@\x90 T`\xFF\x16a\x08-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBackingEigen.mint: caller is not`D\x82\x01Rh\x100\x906\xB4\xB7:2\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x06\xD8\x82\x82a\x179V[a\x08A3\x82a\x17\xC4V[PV[a\x08A3\x82a\x17\xDDV[a\x08Va\x13UV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x01$\xB1%\x03\xBD\xDC&\x16\xC0\xF3\xF5O\xD2>\xD2\x83\xF5\xEF\x0C\x14\x83\xA7T\t\xE4&\x12\x17k\x8B\xDE\x82`@Qa\x08\x93\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81Ra\x013` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 Ta\x06\xC0\x90a\x18WV[a\x08\xF1a\x13UV[a\x08\xFB`\0a\x18\xC0V[V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x81 Ta\x06\xC0V[`\0``\x80`\0\x80`\0```\x97T`\0\x80\x1B\x14\x80\x15a\t;WP`\x98T\x15[a\t\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07\x89V[a\t\x87a\x19\x12V[a\t\x8Fa\x19!V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\t\xC3a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\n\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07\x89V[a\x06\xC0a\x01\0\x83a\x16PV[`\0a\x07\nBa\x190V[```i\x80Ta\x06)\x90a*\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 T\x80\x15a\n\xAAW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80T`\0\x19\x83\x01\x90\x81\x10a\n\x88Wa\n\x88a*\x81V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\n\xADV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x003\x81a\n\xCB\x82\x86a\x10<V[\x90P\x83\x81\x10\x15a\x0B+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x06\xF5\x82\x86\x86\x84\x03a\x121V[`\x003a\x06\xBA\x81\x85\x85a\x14\x8AV[a\x0BNa\x13UV[a\x06\xD8\x82\x82a\x19\x97V[\x83B\x11\x15a\x0B\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Votes: signature expired\0\0\0`D\x82\x01R`d\x01a\x07\x89V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x0C\"\x90a\x0C\x1A\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19\xF0V[\x85\x85\x85a\x1A\x1DV[\x90Pa\x0C-\x81a\x1AEV[\x86\x14a\x0C{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC20Votes: invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x89V[a\x0C\x85\x81\x88a\x17\xDDV[PPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0C\xAEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0C\xC8WP0;\x15\x80\x15a\x0C\xC8WP`\0T`\xFF\x16`\x01\x14[a\r+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\rNW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\rVa\x1AmV[a\r\xA3`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l!0\xB1\xB5\xB4\xB73\x90\"\xB4\xB3\xB2\xB7`\x99\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1A\x9CV[a\r\xAC\x82a\x18\xC0V[a\r\xD3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1A\xCDV[`\0\x19a\x010Ua\x0E\x05\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x13\xAFV[a\x0E0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x19\x97V[a\x0Ef\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0k\x05hhw\xAF\xB5\xCB\xCC\xBFs@\0a\x179V[`@Q\x7F\xB7\xC2<\x1E.6\xF2\x98\xE9\x87\x9A\x88\xEC\xFC\xD0~(\xFB\xB49\xBC\xFA\x9Cx\xCA\x13c\xCA\x147\r&\x90`\0\x90\xA1\x80\x15a\x06\xD8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[\x83B\x11\x15a\x0F(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x07\x89V[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x0FW\x8Ca\x1AEV[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x0F\xB2\x82a\x19\xF0V[\x90P`\0a\x0F\xC2\x82\x87\x87\x87a\x1A\x1DV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x07\x89V[a\x100\x8A\x8A\x8Aa\x121V[PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x10oa\x13UV[`\0\x19a\x010T\x14a\x11\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FBackingEigen.disableTransferRest`D\x82\x01R\x7Frictions: transfer restrictions `d\x82\x01Rs\x18\\\x99H\x18[\x1C\x99XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`b\x1B`\x84\x82\x01R`\xA4\x01a\x07\x89V[`\0a\x010\x81\x90U`@Q\x7F+\x18\x98m;\xA8\t\xDB/\x13\xA5\xD7\xBF\x17\xF6\r5{7\xD9\xCB\xB5]\xD7\x1C\xBB\xAC\x8D\xC4\x06\x0Fd\x91\x90\xA1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x11{Wa\x11{a*\x81V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x11\xC3a\x13UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x08A\x81a\x18\xC0V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x12\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`f` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x011` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x14\x1C\x84\x84a\x10<V[\x90P`\0\x19\x81\x14a\x14\x84W\x81\x81\x10\x15a\x14wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07\x89V[a\x14\x84\x84\x84\x84\x84\x03a\x121V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x14\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x15PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x15[\x83\x83\x83a\x1B\x17V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a\x15\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`e` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x163\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x14\x84\x84\x84\x84a\x1B\xF5V[`\0a\x07\na\x1C'V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x16\xAAW`\0a\x16k\x84a\x1C\x9BV[a\x16u\x90\x85a*\x97V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\x9AW\x80\x91Pa\x16\xA8V[a\x16\xA5\x81`\x01a*XV[\x92P[P[\x80\x82\x10\x15a\x16\xF7W`\0a\x16\xBE\x83\x83a\x1D\x83V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\xE3W\x80\x91Pa\x16\xF1V[a\x16\xEE\x81`\x01a*XV[\x92P[Pa\x16\xAAV[\x80\x15a\x17#W`\0\x86\x81R` \x90 \x81\x01`\0\x19\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x17&V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[a\x17C\x82\x82a\x1D\x9EV[`gT`\x01`\x01`\xE0\x1B\x03\x10\x15a\x17\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC20Votes: total supply risks o`D\x82\x01Roverflowing votes`\x80\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x14\x84a\x01\0a\x1Es\x83a\x1E\x7FV[a\x17\xCE\x82\x82a\x1F\xF4V[a\x14\x84a\x01\0a!;\x83a\x1E\x7FV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\xFE` \x81\x81R`@\x80\x84 \x80T`e\x84R\x82\x86 T\x94\x90\x93R\x87\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x84\x16\x81\x17\x90\x91U\x90Q\x91\x90\x95\x16\x94\x91\x93\x91\x92\x85\x92\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x14\x84\x82\x84\x83a!GV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[P\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\x99\x80Ta\x06)\x90a*\x0EV[```\x9A\x80Ta\x06)\x90a*\x0EV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x012` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7Fr\xA5a\xD1\xAFt\tF}\xAEO\x1E\x9F\xC5%\x90\xA93Z\x1D\xDA\x17r~+j\xA8\xC4\xDB5\x10\x9B\x91\x01a\x14\x04V[`\0a\x06\xC0a\x19\xFDa\x16FV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x1A.\x87\x87\x87\x87a\"\x84V[\x91P\x91Pa\x1A;\x81a#HV[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x08\xFBa$\x92V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x06\xD8\x82\x82a$\xC2V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x08A\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RPa%\x02V[a\x010TB\x11a\x1B\xF0W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x011` R`@\x90 T`\xFF\x16\x80a\x1BbWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x012` R`@\x90 T`\xFF\x16[\x80a\x1BtWP`\x01`\x01`\xA0\x1B\x03\x83\x16\x15[a\x1B\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FBackingEigen._beforeTokenTransfe`D\x82\x01R\x7Fr: from or to must be whiteliste`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x07\x89V[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\xFE` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x1B\xF0\x92\x91\x82\x16\x91\x16\x83a!GV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1CRa%QV[a\x1CZa%\xAAV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81`\0\x03a\x1C\xADWP`\0\x91\x90PV[`\0`\x01a\x1C\xBA\x84a%\xDBV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x1C\xD3Wa\x1C\xD3a*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xEBWa\x1C\xEBa*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D\x03Wa\x1D\x03a*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D\x1BWa\x1D\x1Ba*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D3Wa\x1D3a*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1DKWa\x1DKa*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1DcWa\x1Dca*\xF5V[\x04\x82\x01\x90\x1C\x90Pa\x07\xB4\x81\x82\x85\x81a\x1D}Wa\x1D}a*\xF5V[\x04a&oV[`\0a\x1D\x92`\x02\x84\x84\x18a+\x0BV[a\x07\xB4\x90\x84\x84\x16a*XV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07\x89V[a\x1E\0`\0\x83\x83a\x1B\x17V[\x80`g`\0\x82\x82Ta\x1E\x12\x91\x90a*XV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x06\xD8`\0\x83\x83a\x1B\xF5V[`\0a\x07\xB4\x82\x84a*XV[\x82T`\0\x90\x81\x90\x81\x81\x15a\x1E\xCCW`\0\x87\x81R` \x90 \x82\x01`\0\x19\x01`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16` \x82\x01Ra\x1E\xE1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x90P\x80` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93Pa\x1F\x01\x84\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x92P`\0\x82\x11\x80\x15a\x1F+WPa\x1F\x16a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x1FpWa\x1F9\x83a&\x85V[`\0\x88\x81R` \x90 \x83\x01`\0\x19\x01\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1F\xEAV[\x86`@Q\x80`@\x01`@R\x80a\x1F\x94a\x1F\x87a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18WV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x1F\xA8\x86a&\x85V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[PP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a `\x82`\0\x83a\x1B\x17V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a \xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`g\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x1B\xF0\x83`\0\x84a\x1B\xF5V[`\0a\x07\xB4\x82\x84a*\x97V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!iWP`\0\x81\x11[\x15a\x1B\xF0W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a!\xF7W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a!\xA4\x90a!;\x85a\x1E\x7FV[\x91P\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x1B\xF0W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a\"-\x90a\x1Es\x85a\x1E\x7FV[\x91P\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\"u\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\xBBWP`\0\x90P`\x03a#?V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a#\x0FW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a#8W`\0`\x01\x92P\x92PPa#?V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a#\\Wa#\\a+-V[\x03a#dWPV[`\x01\x81`\x04\x81\x11\x15a#xWa#xa+-V[\x03a#\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x89V[`\x02\x81`\x04\x81\x11\x15a#\xD9Wa#\xD9a+-V[\x03a$&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x07\x89V[`\x03\x81`\x04\x81\x11\x15a$:Wa$:a+-V[\x03a\x08AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\0Ta\x01\0\x90\x04`\xFF\x16a$\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x08\xFB3a\x18\xC0V[`\0Ta\x01\0\x90\x04`\xFF\x16a$\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[`ha$\xF5\x83\x82a+\x91V[P`ia\x1B\xF0\x82\x82a+\x91V[`\0Ta\x01\0\x90\x04`\xFF\x16a%)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[`\x99a%5\x83\x82a+\x91V[P`\x9Aa%B\x82\x82a+\x91V[PP`\0`\x97\x81\x90U`\x98UPV[`\0\x80a%\\a\x19\x12V[\x80Q\x90\x91P\x15a%sW\x80Q` \x90\x91\x01 \x91\x90PV[`\x97T\x80\x15a%\x82W\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[`\0\x80a%\xB5a\x19!V[\x80Q\x90\x91P\x15a%\xCCW\x80Q` \x90\x91\x01 \x91\x90PV[`\x98T\x80\x15a%\x82W\x92\x91PPV[`\0\x80`\x80\x83\x90\x1C\x15a%\xF0W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a&\x02W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a&\x14W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a&&W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a&8W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a&JW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a&\\W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x06\xC0W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a&~W\x81a\x07\xB4V[P\x90\x91\x90PV[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x18\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\x05W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a'\x1CW`\0\x80\xFD[a\x07\xB4\x82a&\xEEV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a'KW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a'/V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x07\xB4` \x83\x01\x84a'%V[`\0\x80`@\x83\x85\x03\x12\x15a'\x91W`\0\x80\xFD[a'\x9A\x83a&\xEEV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\xBBW`\0\x80\xFD[a'\xC4\x83a&\xEEV[\x91P` \x83\x015\x80\x15\x15\x81\x14a'\xD9W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xF9W`\0\x80\xFD[a(\x02\x84a&\xEEV[\x92Pa(\x10` \x85\x01a&\xEEV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a(3W`\0\x80\xFD[P5\x91\x90PV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a(Y`\xE0\x83\x01\x89a'%V[\x82\x81\x03`@\x84\x01Ra(k\x81\x89a'%V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a(\xC1W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(\xA3V[P\x90\x9B\x9APPPPPPPPPPPV[\x805`\xFF\x81\x16\x81\x14a'\x05W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a(\xFCW`\0\x80\xFD[a)\x05\x87a&\xEEV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa)!``\x88\x01a(\xD2V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a)VW`\0\x80\xFD[a)_\x88a&\xEEV[\x96Pa)m` \x89\x01a&\xEEV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa)\x89`\x80\x89\x01a(\xD2V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a)\xB9W`\0\x80\xFD[a)\xC2\x83a&\xEEV[\x91Pa)\xD0` \x84\x01a&\xEEV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a)\xECW`\0\x80\xFD[a)\xF5\x83a&\xEEV[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'\xD9W`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a*\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1AgWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xC0Wa\x06\xC0a*BV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xC0Wa\x06\xC0a*BV[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a+(WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x1B\xF0W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a+jWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a+\x8AW`\0\x81U`\x01\x01a+vV[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xABWa+\xABa*kV[a+\xBF\x81a+\xB9\x84Ta*\x0EV[\x84a+CV[` `\x1F\x82\x11`\x01\x81\x14a+\xF3W`\0\x83\x15a+\xDBWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua+\x8AV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a,#W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a,\x03V[P\x84\x82\x10\x15a,AW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xEC\xE6As\x9D\xB4\xEC\xD9\xD6\xC5\x83m\xEB/\xDF\xA1\x9B?\x0C\xB2YT\x05r\xE9\xF7\x17\x11\xF0\x02B-dsolcC\0\x08\x1B\x003\xA2dipfsX\"\x12 s\xB7HH\x8AvL\x80ef\xA1\x0Bu){N\x1E\n\xE89\xB6\x9D\xA6\xF5\x9Fn\xDA\x82B\xD4\x972dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061012c5760003560e01c8063916a17c6116100ad578063c3ea3fc911610071578063c3ea3fc91461022f578063d715bc6e14610242578063e20c9f7114610255578063f8ccbf471461025d578063fa7626d41461026a57600080fd5b8063916a17c6146101ec578063a5617cfd146101f4578063b5508aa914610207578063ba414fa61461020f578063c04062261461022757600080fd5b80633f7286f4116100f45780633f7286f41461019f57806354755b99146101a757806366d9a9a0146101af578063811ffeeb146101c457806385226c81146101d757600080fd5b806314f8ffac146101315780631ed7831c1461013b57806335e4a03614610159578063370c30d9146101845780633e5e3c2314610197575b600080fd5b610139610277565b005b610143610491565b604051610150919061116b565b60405180910390f35b60205461016c906001600160a01b031681565b6040516001600160a01b039091168152602001610150565b60215461016c906001600160a01b031681565b6101436104f3565b610143610553565b6101396105b3565b6101b76108e2565b60405161015091906111b7565b601f5461016c906001600160a01b031681565b6101df6109d1565b60405161015091906112c1565b6101b7610aa1565b601c5461016c906001600160a01b031681565b6101df610b87565b610217610c57565b6040519015158152602001610150565b610139610d82565b601d5461016c906001600160a01b031681565b601e5461016c906001600160a01b031681565b6101436110fe565b601b546102179060ff1681565b6000546102179060ff1681565b601f5460405163ca669fa760e01b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ca669fa790602401600060405180830381600087803b1580156102d057600080fd5b505af11580156102e4573d6000803e3d6000fd5b5050601d54601e54601c546040516310270e3d60e11b81526001600160a01b039182166004820152928116945016915063204e1c7a90602401602060405180830381865afa15801561033a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061035e919061132f565b6001600160a01b0316146103b95760405162461bcd60e51b815260206004820152601e60248201527f696d706c656d656e746174696f6e2073657420696e636f72726563746c79000060448201526064015b60405180910390fd5b602154601c5460408051637ee1b8e760e11b815290516001600160a01b03938416939092169163fdc371ce916004808201926020929091908290030181865afa15801561040a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061042e919061132f565b6001600160a01b03161461048f5760405162461bcd60e51b815260206004820152602260248201527f454947454e2061646472657373206368616e67656420756e65787065637465646044820152616c7960f01b60648201526084016103b0565b565b6060600d8054806020026020016040519081016040528092919081815260200182805480156104e957602002820191906000526020600020905b81546001600160a01b031681526001909101906020018083116104cb575b5050505050905090565b6060600f8054806020026020016040519081016040528092919081815260200182805480156104e9576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116104cb575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156104e9576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116104cb575050505050905090565b601f546040805163793d064960e11b815290516000926001600160a01b03169163f27a0c929160048083019260209291908290030181865afa1580156105fd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106219190611353565b601c54601d54604080516001600160a01b0393841660248201529290911660448084019190915281518084039091018152606490920181526020820180516001600160e01b031663266a23b160e21b17905251919250907fd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18906106a590839061136c565b60405180910390a16020546040516303223eab60e11b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d5690602401600060405180830381600087803b15801561070657600080fd5b505af115801561071a573d6000803e3d6000fd5b5050601f54601e5460405162ea831560e11b81526001600160a01b0392831694506301d5062a935061075d92909116906000908690829081908a9060040161139a565b600060405180830381600087803b15801561077757600080fd5b505af115801561078b573d6000803e3d6000fd5b50737109709ecfa91a80626ff3989d68f67f5b1dd12d925063e5d6bf0291506107b6905084426113df565b6040518263ffffffff1660e01b81526004016107d491815260200190565b600060405180830381600087803b1580156107ee57600080fd5b505af1158015610802573d6000803e3d6000fd5b5050601f54601e5460405163134008d360e01b81526001600160a01b03928316945063134008d393506108449290911690600090869082908190600401611406565b600060405180830381600087803b15801561085e57600080fd5b505af1158015610872573d6000803e3d6000fd5b50505050601b60019054906101000a90046001600160a01b03166001600160a01b03166390c5013b6040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156108c657600080fd5b505af11580156108da573d6000803e3d6000fd5b505050505050565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156109c85760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156109b057602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116109725790505b50505050508152505081526020019060010190610906565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020016000905b828210156109c8578382906000526020600020018054610a1490611441565b80601f0160208091040260200160405190810160405280929190818152602001828054610a4090611441565b8015610a8d5780601f10610a6257610100808354040283529160200191610a8d565b820191906000526020600020905b815481529060010190602001808311610a7057829003601f168201915b5050505050815260200190600101906109f5565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156109c85760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610b6f57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610b315790505b50505050508152505081526020019060010190610ac5565b60606010805480602002602001604051908101604052809291908181526020016000905b828210156109c8578382906000526020600020018054610bca90611441565b80601f0160208091040260200160405190810160405280929190818152602001828054610bf690611441565b8015610c435780601f10610c1857610100808354040283529160200191610c43565b820191906000526020600020905b815481529060010190602001808311610c2657829003601f168201915b505050505081526020019060010190610bab565b60008054610100900460ff1615610c775750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610d7d5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091610d05917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc49160800161147b565b60408051601f1981840301815290829052610d1f916114ac565b6000604051808303816000865af19150503d8060008114610d5c576040519150601f19603f3d011682016040523d82523d6000602084013e610d61565b606091505b5091505080806020019051810190610d7991906114c8565b9150505b919050565b60408051818152601c818301527f596f7520617265206465706c6f79696e67206f6e20436861696e4944000000006060820152466020820181905291517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160018114610e2f5760405162461bcd60e51b815260206004820152601360248201527210da185a5b881b9bdd081cdd5c1c1bdc9d1959606a1b60448201526064016103b0565b601c60009054906101000a90046001600160a01b03166001600160a01b031663fdc371ce6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e82573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ea6919061132f565b602180546001600160a01b0319166001600160a01b0392909216918217905573ec53bf9167f50cdeb3ae105f56099aaab9061f8314610f275760405162461bcd60e51b815260206004820152601860248201527f736f6d657468696e6720686f727269626c792077726f6e67000000000000000060448201526064016103b0565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b0316637fb5297f6040518163ffffffff1660e01b8152600401600060405180830381600087803b158015610f8557600080fd5b505af1158015610f99573d6000803e3d6000fd5b50506021546040516001600160a01b039091169250610fb8915061115e565b6001600160a01b039091168152602001604051809103906000f080158015610fe4573d6000803e3d6000fd5b50601d60006101000a8154816001600160a01b0302191690836001600160a01b031602179055507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d60001c6001600160a01b03166376eadd366040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561106957600080fd5b505af115801561107d573d6000803e3d6000fd5b5050601d5460408051818152601581830152743122a4a3a2a72fb4b6b83632b6b2b73a30ba34b7b760591b60608201526001600160a01b039092166020830152517f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f9350908190036080019150a16110f36105b3565b6110fb610277565b50565b6060600c8054806020026020016040519081016040528092919081815260200182805480156104e9576020028201919060005260206000209081546001600160a01b031681526001909101906020018083116104cb575050505050905090565b612def806114eb83390190565b602080825282518282018190526000918401906040840190835b818110156111ac5783516001600160a01b0316835260209384019390920191600101611185565b509095945050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561126557868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b8083101561124d5783516001600160e01b03191682526020938401936001939093019290910190611221565b509650505060209384019391909101906001016111df565b50929695505050505050565b60005b8381101561128c578181015183820152602001611274565b50506000910152565b600081518084526112ad816020860160208601611271565b601f01601f19169290920160200192915050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561126557603f19878603018452611305858351611295565b945060209384019391909101906001016112e9565b6001600160a01b03811681146110fb57600080fd5b60006020828403121561134157600080fd5b815161134c8161131a565b9392505050565b60006020828403121561136557600080fd5b5051919050565b6040815260046040820152636461746160e01b606082015260806020820152600061134c6080830184611295565b60018060a01b038716815285602082015260c0604082015260006113c160c0830187611295565b606083019590955250608081019290925260a0909101529392505050565b8082018082111561140057634e487b7160e01b600052601160045260246000fd5b92915050565b60018060a01b038616815284602082015260a06040820152600061142d60a0830186611295565b606083019490945250608001529392505050565b600181811c9082168061145557607f821691505b60208210810361147557634e487b7160e01b600052602260045260246000fd5b50919050565b6001600160e01b031983168152815160009061149e816004850160208701611271565b919091016004019392505050565b600082516114be818460208701611271565b9190910192915050565b6000602082840312156114da57600080fd5b8151801515811461134c57600080fdfe60a060405234801561001057600080fd5b50604051612def380380612def83398101604081905261002f91610109565b6001600160a01b03811660805261004461004a565b50610139565b600054610100900460ff16156100b65760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811614610107576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b60006020828403121561011b57600080fd5b81516001600160a01b038116811461013257600080fd5b9392505050565b608051612c86610169600039600081816105f801528181610ddf01528181610e0a0152610e350152612c866000f3fe608060405234801561001057600080fd5b50600436106102485760003560e01c80637ecebe001161013b578063aa271e1a116100b8578063dd62ed3e1161007c578063dd62ed3e14610588578063eb415f451461059b578063f1127ed8146105a3578063f2fde38b146105e0578063fdc371ce146105f357600080fd5b8063aa271e1a14610518578063b8c255941461053c578063c3cda5201461054f578063c4d66de814610562578063d505accf1461057557600080fd5b806395d89b41116100ff57806395d89b41146104cd5780639ab24eb0146104d55780639aec4bae146104e8578063a457c2d7146104f2578063a9059cbb1461050557600080fd5b80637ecebe001461045c57806384b0196e1461046f5780638da5cb5b1461048a5780638e539e8c1461049b57806391ddadf4146104ae57600080fd5b806340c10f19116101c957806366eb399f1161018d57806366eb399f146103cc5780636fcfff45146103df57806370a0823114610407578063715018a61461043057806378aa33ba1461043857600080fd5b806340c10f191461032557806342966c68146103385780634bf5d7e91461034b578063587cde1e146103755780635c19a95c146103b957600080fd5b806323b872dd1161021057806323b872dd146102d5578063313ce567146102e85780633644e515146102f757806339509351146102ff5780633a46b1a81461031257600080fd5b80630455e6941461024d57806306fdde0314610286578063095ea7b31461029b57806318160ddd146102ae5780631ffacdef146102c0575b600080fd5b61027161025b36600461270a565b6101316020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b61028e61061a565b60405161027d919061276b565b6102716102a936600461277e565b6106ac565b6067545b60405190815260200161027d565b6102d36102ce3660046127a8565b6106c6565b005b6102716102e33660046127e4565b6106dc565b6040516012815260200161027d565b6102b2610700565b61027161030d36600461277e565b61070f565b6102b261032036600461277e565b610731565b6102d361033336600461277e565b6107bb565b6102d3610346366004612821565b610837565b60408051808201909152600e81526d06d6f64653d74696d657374616d760941b602082015261028e565b6103a161038336600461270a565b6001600160a01b03908116600090815260fe60205260409020541690565b6040516001600160a01b03909116815260200161027d565b6102d36103c736600461270a565b610844565b6102d36103da3660046127a8565b61084e565b6103f26103ed36600461270a565b6108c7565b60405163ffffffff909116815260200161027d565b6102b261041536600461270a565b6001600160a01b031660009081526065602052604090205490565b6102d36108e9565b61027161044636600461270a565b6101326020526000908152604090205460ff1681565b6102b261046a36600461270a565b6108fd565b61047761091b565b60405161027d979695949392919061283a565b6033546001600160a01b03166103a1565b6102b26104a9366004612821565b6109b9565b6104b6610a21565b60405165ffffffffffff909116815260200161027d565b61028e610a2c565b6102b26104e336600461270a565b610a3b565b6102b26101305481565b61027161050036600461277e565b610abd565b61027161051336600461277e565b610b38565b61027161052636600461270a565b6101336020526000908152604090205460ff1681565b6102d361054a3660046127a8565b610b46565b6102d361055d3660046128e3565b610b58565b6102d361057036600461270a565b610c8e565b6102d361058336600461293b565b610ed8565b6102b26105963660046129a6565b61103c565b6102d3611067565b6105b66105b13660046129d9565b611137565b60408051825163ffffffff1681526020928301516001600160e01b0316928101929092520161027d565b6102d36105ee36600461270a565b6111bb565b6103a17f000000000000000000000000000000000000000000000000000000000000000081565b60606068805461062990612a0e565b80601f016020809104026020016040519081016040528092919081815260200182805461065590612a0e565b80156106a25780601f10610677576101008083540402835291602001916106a2565b820191906000526020600020905b81548152906001019060200180831161068557829003601f168201915b5050505050905090565b6000336106ba818585611231565b60019150505b92915050565b6106ce611355565b6106d882826113af565b5050565b6000336106ea858285611410565b6106f585858561148a565b506001949350505050565b600061070a611646565b905090565b6000336106ba818585610722838361103c565b61072c9190612a58565b611231565b600061073b610a21565b65ffffffffffff1682106107925760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064015b60405180910390fd5b6001600160a01b038316600090815260ff602052604090206107b49083611650565b9392505050565b336000908152610133602052604090205460ff1661082d5760405162461bcd60e51b815260206004820152602960248201527f4261636b696e67456967656e2e6d696e743a2063616c6c6572206973206e6f7460448201526810309036b4b73a32b960b91b6064820152608401610789565b6106d88282611739565b61084133826117c4565b50565b61084133826117dd565b610856611355565b816001600160a01b03167f0124b12503bddc2616c0f3f54fd23ed283f5ef0c1483a75409e42612176b8bde82604051610893911515815260200190565b60405180910390a26001600160a01b0391909116600090815261013360205260409020805460ff1916911515919091179055565b6001600160a01b038116600090815260ff60205260408120546106c090611857565b6108f1611355565b6108fb60006118c0565b565b6001600160a01b038116600090815260cb60205260408120546106c0565b6000606080600080600060606097546000801b14801561093b5750609854155b61097f5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b6044820152606401610789565b610987611912565b61098f611921565b60408051600080825260208201909252600f60f81b9b939a50919850469750309650945092509050565b60006109c3610a21565b65ffffffffffff168210610a155760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b6044820152606401610789565b6106c061010083611650565b600061070a42611930565b60606069805461062990612a0e565b6001600160a01b038116600090815260ff60205260408120548015610aaa576001600160a01b038316600090815260ff6020526040902080546000198301908110610a8857610a88612a81565b60009182526020909120015464010000000090046001600160e01b0316610aad565b60005b6001600160e01b03169392505050565b60003381610acb828661103c565b905083811015610b2b5760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401610789565b6106f58286868403611231565b6000336106ba81858561148a565b610b4e611355565b6106d88282611997565b83421115610ba85760405162461bcd60e51b815260206004820152601d60248201527f4552433230566f7465733a207369676e617475726520657870697265640000006044820152606401610789565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b038816918101919091526060810186905260808101859052600090610c2290610c1a9060a001604051602081830303815290604052805190602001206119f0565b858585611a1d565b9050610c2d81611a45565b8614610c7b5760405162461bcd60e51b815260206004820152601960248201527f4552433230566f7465733a20696e76616c6964206e6f6e6365000000000000006044820152606401610789565b610c8581886117dd565b50505050505050565b600054610100900460ff1615808015610cae5750600054600160ff909116105b80610cc85750303b158015610cc8575060005460ff166001145b610d2b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610789565b6000805460ff191660011790558015610d4e576000805461ff0019166101001790555b610d56611a6d565b610da36040518060400160405280600d81526020016c2130b1b5b4b7339022b4b3b2b760991b815250604051806040016040528060068152602001653122a4a3a2a760d11b815250611a9c565b610dac826118c0565b610dd3604051806040016040528060068152602001653122a4a3a2a760d11b815250611acd565b60001961013055610e057f000000000000000000000000000000000000000000000000000000000000000060016113af565b610e307f00000000000000000000000000000000000000000000000000000000000000006001611997565b610e667f00000000000000000000000000000000000000000000000000000000000000006b05686877afb5cbccbf734000611739565b6040517fb7c23c1e2e36f298e9879a88ecfcd07e28fbb439bcfa9c78ca1363ca14370d2690600090a180156106d8576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b83421115610f285760405162461bcd60e51b815260206004820152601d60248201527f45524332305065726d69743a206578706972656420646561646c696e650000006044820152606401610789565b60007f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9888888610f578c611a45565b6040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810186905260e0016040516020818303038152906040528051906020012090506000610fb2826119f0565b90506000610fc282878787611a1d565b9050896001600160a01b0316816001600160a01b0316146110255760405162461bcd60e51b815260206004820152601e60248201527f45524332305065726d69743a20696e76616c6964207369676e617475726500006044820152606401610789565b6110308a8a8a611231565b50505050505050505050565b6001600160a01b03918216600090815260666020908152604080832093909416825291909152205490565b61106f611355565b60001961013054146111065760405162461bcd60e51b815260206004820152605460248201527f4261636b696e67456967656e2e64697361626c655472616e736665725265737460448201527f72696374696f6e733a207472616e73666572207265737472696374696f6e7320606482015273185c9948185b1c9958591e48191a5cd8589b195960621b608482015260a401610789565b60006101308190556040517f2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f649190a1565b60408051808201909152600080825260208201526001600160a01b038316600090815260ff60205260409020805463ffffffff841690811061117b5761117b612a81565b60009182526020918290206040805180820190915291015463ffffffff8116825264010000000090046001600160e01b0316918101919091529392505050565b6111c3611355565b6001600160a01b0381166112285760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610789565b610841816118c0565b6001600160a01b0383166112935760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401610789565b6001600160a01b0382166112f45760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401610789565b6001600160a01b0383811660008181526066602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b6033546001600160a01b031633146108fb5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610789565b6001600160a01b03821660008181526101316020908152604091829020805460ff191685151590811790915591519182527fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227191015b60405180910390a25050565b600061141c848461103c565b9050600019811461148457818110156114775760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401610789565b6114848484848403611231565b50505050565b6001600160a01b0383166114ee5760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401610789565b6001600160a01b0382166115505760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401610789565b61155b838383611b17565b6001600160a01b038316600090815260656020526040902054818110156115d35760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401610789565b6001600160a01b0380851660008181526065602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef906116339086815260200190565b60405180910390a3611484848484611bf5565b600061070a611c27565b8154600090818160058111156116aa57600061166b84611c9b565b6116759085612a97565b600088815260209020909150869082015463ffffffff16111561169a578091506116a8565b6116a5816001612a58565b92505b505b808210156116f75760006116be8383611d83565b600088815260209020909150869082015463ffffffff1611156116e3578091506116f1565b6116ee816001612a58565b92505b506116aa565b8015611723576000868152602090208101600019015464010000000090046001600160e01b0316611726565b60005b6001600160e01b03169695505050505050565b6117438282611d9e565b6067546001600160e01b0310156117b55760405162461bcd60e51b815260206004820152603060248201527f4552433230566f7465733a20746f74616c20737570706c79207269736b73206f60448201526f766572666c6f77696e6720766f74657360801b6064820152608401610789565b611484610100611e7383611e7f565b6117ce8282611ff4565b61148461010061213b83611e7f565b6001600160a01b03828116600081815260fe6020818152604080842080546065845282862054949093528787166001600160a01b03198416811790915590519190951694919391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4611484828483612147565b600063ffffffff8211156118bc5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610789565b5090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60606099805461062990612a0e565b6060609a805461062990612a0e565b600065ffffffffffff8211156118bc5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203460448201526538206269747360d01b6064820152608401610789565b6001600160a01b03821660008181526101326020908152604091829020805460ff191685151590811790915591519182527f72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b9101611404565b60006106c06119fd611646565b8360405161190160f01b8152600281019290925260228201526042902090565b6000806000611a2e87878787612284565b91509150611a3b81612348565b5095945050505050565b6001600160a01b038116600090815260cb602052604090208054600181018255905b50919050565b600054610100900460ff16611a945760405162461bcd60e51b815260040161078990612aaa565b6108fb612492565b600054610100900460ff16611ac35760405162461bcd60e51b815260040161078990612aaa565b6106d882826124c2565b600054610100900460ff16611af45760405162461bcd60e51b815260040161078990612aaa565b61084181604051806040016040528060018152602001603160f81b815250612502565b610130544211611bf0576001600160a01b0383166000908152610131602052604090205460ff1680611b6257506001600160a01b0382166000908152610132602052604090205460ff165b80611b7457506001600160a01b038316155b611bf05760405162461bcd60e51b815260206004820152604160248201527f4261636b696e67456967656e2e5f6265666f7265546f6b656e5472616e73666560448201527f723a2066726f6d206f7220746f206d7573742062652077686974656c697374656064820152601960fa1b608482015260a401610789565b505050565b6001600160a01b03838116600090815260fe6020526040808220548584168352912054611bf092918216911683612147565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611c52612551565b611c5a6125aa565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b600081600003611cad57506000919050565b60006001611cba846125db565b901c6001901b90506001818481611cd357611cd3612af5565b048201901c90506001818481611ceb57611ceb612af5565b048201901c90506001818481611d0357611d03612af5565b048201901c90506001818481611d1b57611d1b612af5565b048201901c90506001818481611d3357611d33612af5565b048201901c90506001818481611d4b57611d4b612af5565b048201901c90506001818481611d6357611d63612af5565b048201901c90506107b481828581611d7d57611d7d612af5565b0461266f565b6000611d926002848418612b0b565b6107b490848416612a58565b6001600160a01b038216611df45760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401610789565b611e0060008383611b17565b8060676000828254611e129190612a58565b90915550506001600160a01b0382166000818152606560209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a36106d860008383611bf5565b60006107b48284612a58565b82546000908190818115611ecc5760008781526020902082016000190160408051808201909152905463ffffffff8116825264010000000090046001600160e01b03166020820152611ee1565b60408051808201909152600080825260208201525b905080602001516001600160e01b03169350611f0184868863ffffffff16565b9250600082118015611f2b5750611f16610a21565b65ffffffffffff16816000015163ffffffff16145b15611f7057611f3983612685565b60008881526020902083016000190180546001600160e01b03929092166401000000000263ffffffff909216919091179055611fea565b866040518060400160405280611f94611f87610a21565b65ffffffffffff16611857565b63ffffffff168152602001611fa886612685565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b5050935093915050565b6001600160a01b0382166120545760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401610789565b61206082600083611b17565b6001600160a01b038216600090815260656020526040902054818110156120d45760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401610789565b6001600160a01b03831660008181526065602090815260408083208686039055606780548790039055518581529192917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3611bf083600084611bf5565b60006107b48284612a97565b816001600160a01b0316836001600160a01b0316141580156121695750600081115b15611bf0576001600160a01b038316156121f7576001600160a01b038316600090815260ff6020526040812081906121a49061213b85611e7f565b91509150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516121ec929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615611bf0576001600160a01b038216600090815260ff60205260408120819061222d90611e7385611e7f565b91509150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7248383604051612275929190918252602082015260400190565b60405180910390a25050505050565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156122bb575060009050600361233f565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561230f573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166123385760006001925092505061233f565b9150600090505b94509492505050565b600081600481111561235c5761235c612b2d565b036123645750565b600181600481111561237857612378612b2d565b036123c55760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610789565b60028160048111156123d9576123d9612b2d565b036124265760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610789565b600381600481111561243a5761243a612b2d565b036108415760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610789565b600054610100900460ff166124b95760405162461bcd60e51b815260040161078990612aaa565b6108fb336118c0565b600054610100900460ff166124e95760405162461bcd60e51b815260040161078990612aaa565b60686124f58382612b91565b506069611bf08282612b91565b600054610100900460ff166125295760405162461bcd60e51b815260040161078990612aaa565b60996125358382612b91565b50609a6125428282612b91565b50506000609781905560985550565b60008061255c611912565b805190915015612573578051602090910120919050565b60975480156125825792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b6000806125b5611921565b8051909150156125cc578051602090910120919050565b60985480156125825792915050565b600080608083901c156125f057608092831c92015b604083901c1561260257604092831c92015b602083901c1561261457602092831c92015b601083901c1561262657601092831c92015b600883901c1561263857600892831c92015b600483901c1561264a57600492831c92015b600283901c1561265c57600292831c92015b600183901c156106c05760010192915050565b600081831061267e57816107b4565b5090919050565b60006001600160e01b038211156118bc5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610789565b80356001600160a01b038116811461270557600080fd5b919050565b60006020828403121561271c57600080fd5b6107b4826126ee565b6000815180845260005b8181101561274b5760208185018101518683018201520161272f565b506000602082860101526020601f19601f83011685010191505092915050565b6020815260006107b46020830184612725565b6000806040838503121561279157600080fd5b61279a836126ee565b946020939093013593505050565b600080604083850312156127bb57600080fd5b6127c4836126ee565b9150602083013580151581146127d957600080fd5b809150509250929050565b6000806000606084860312156127f957600080fd5b612802846126ee565b9250612810602085016126ee565b929592945050506040919091013590565b60006020828403121561283357600080fd5b5035919050565b60ff60f81b8816815260e06020820152600061285960e0830189612725565b828103604084015261286b8189612725565b606084018890526001600160a01b038716608085015260a0840186905283810360c08501528451808252602080870193509091019060005b818110156128c15783518352602093840193909201916001016128a3565b50909b9a5050505050505050505050565b803560ff8116811461270557600080fd5b60008060008060008060c087890312156128fc57600080fd5b612905876126ee565b95506020870135945060408701359350612921606088016128d2565b9598949750929560808101359460a0909101359350915050565b600080600080600080600060e0888a03121561295657600080fd5b61295f886126ee565b965061296d602089016126ee565b95506040880135945060608801359350612989608089016128d2565b9699959850939692959460a0840135945060c09093013592915050565b600080604083850312156129b957600080fd5b6129c2836126ee565b91506129d0602084016126ee565b90509250929050565b600080604083850312156129ec57600080fd5b6129f5836126ee565b9150602083013563ffffffff811681146127d957600080fd5b600181811c90821680612a2257607f821691505b602082108103611a6757634e487b7160e01b600052602260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b808201808211156106c0576106c0612a42565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b818103818111156106c0576106c0612a42565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b600052601260045260246000fd5b600082612b2857634e487b7160e01b600052601260045260246000fd5b500490565b634e487b7160e01b600052602160045260246000fd5b601f821115611bf057806000526020600020601f840160051c81016020851015612b6a5750805b601f840160051c820191505b81811015612b8a5760008155600101612b76565b5050505050565b815167ffffffffffffffff811115612bab57612bab612a6b565b612bbf81612bb98454612a0e565b84612b43565b6020601f821160018114612bf35760008315612bdb5750848201515b600019600385901b1c1916600184901b178455612b8a565b600084815260208120601f198516915b82811015612c235787850151825560209485019460019092019101612c03565b5084821015612c415786840151600019600387901b60f8161c191681555b50505050600190811b0190555056fea2646970667358221220ece641739db4ecd9d6c5836deb2fdfa19b3f0cb259540572e9f71711f002422d64736f6c634300081b0033a264697066735822122073b748488a764c806566a10b75297b4e1e0ae839b69da6f59f6eda8242d4973264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01,W`\x005`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xADW\x80c\xC3\xEA?\xC9\x11a\0qW\x80c\xC3\xEA?\xC9\x14a\x02/W\x80c\xD7\x15\xBCn\x14a\x02BW\x80c\xE2\x0C\x9Fq\x14a\x02UW\x80c\xF8\xCC\xBFG\x14a\x02]W\x80c\xFAv&\xD4\x14a\x02jW`\0\x80\xFD[\x80c\x91j\x17\xC6\x14a\x01\xECW\x80c\xA5a|\xFD\x14a\x01\xF4W\x80c\xB5P\x8A\xA9\x14a\x02\x07W\x80c\xBAAO\xA6\x14a\x02\x0FW\x80c\xC0@b&\x14a\x02'W`\0\x80\xFD[\x80c?r\x86\xF4\x11a\0\xF4W\x80c?r\x86\xF4\x14a\x01\x9FW\x80cTu[\x99\x14a\x01\xA7W\x80cf\xD9\xA9\xA0\x14a\x01\xAFW\x80c\x81\x1F\xFE\xEB\x14a\x01\xC4W\x80c\x85\"l\x81\x14a\x01\xD7W`\0\x80\xFD[\x80c\x14\xF8\xFF\xAC\x14a\x011W\x80c\x1E\xD7\x83\x1C\x14a\x01;W\x80c5\xE4\xA06\x14a\x01YW\x80c7\x0C0\xD9\x14a\x01\x84W\x80c>^<#\x14a\x01\x97W[`\0\x80\xFD[a\x019a\x02wV[\0[a\x01Ca\x04\x91V[`@Qa\x01P\x91\x90a\x11kV[`@Q\x80\x91\x03\x90\xF3[` Ta\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01PV[`!Ta\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01Ca\x04\xF3V[a\x01Ca\x05SV[a\x019a\x05\xB3V[a\x01\xB7a\x08\xE2V[`@Qa\x01P\x91\x90a\x11\xB7V[`\x1FTa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDFa\t\xD1V[`@Qa\x01P\x91\x90a\x12\xC1V[a\x01\xB7a\n\xA1V[`\x1CTa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDFa\x0B\x87V[a\x02\x17a\x0CWV[`@Q\x90\x15\x15\x81R` \x01a\x01PV[a\x019a\r\x82V[`\x1DTa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1ETa\x01l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01Ca\x10\xFEV[`\x1BTa\x02\x17\x90`\xFF\x16\x81V[`\0Ta\x02\x17\x90`\xFF\x16\x81V[`\x1FT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xE4W=`\0\x80>=`\0\xFD[PP`\x1DT`\x1ET`\x1CT`@Qc\x10'\x0E=`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x81\x16\x94P\x16\x91Pc N\x1Cz\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03^\x91\x90a\x13/V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fimplementation set incorrectly\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`!T`\x1CT`@\x80Qc~\xE1\xB8\xE7`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x90\x92\x16\x91c\xFD\xC3q\xCE\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x04\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04.\x91\x90a\x13/V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEIGEN address changed unexpected`D\x82\x01Raly`\xF0\x1B`d\x82\x01R`\x84\x01a\x03\xB0V[V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBW[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBWPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBWPPPPP\x90P\x90V[`\x1FT`@\x80Qcy=\x06I`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2z\x0C\x92\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06!\x91\x90a\x13SV[`\x1CT`\x1DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x92\x90\x91\x16`D\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c&j#\xB1`\xE2\x1B\x17\x90RQ\x91\x92P\x90\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x90a\x06\xA5\x90\x83\x90a\x13lV[`@Q\x80\x91\x03\x90\xA1` T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x06W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x1AW=`\0\x80>=`\0\xFD[PP`\x1FT`\x1ET`@Qb\xEA\x83\x15`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x01\xD5\x06*\x93Pa\x07]\x92\x90\x91\x16\x90`\0\x90\x86\x90\x82\x90\x81\x90\x8A\x90`\x04\x01a\x13\x9AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07wW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\x8BW=`\0\x80>=`\0\xFD[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92Pc\xE5\xD6\xBF\x02\x91Pa\x07\xB6\x90P\x84Ba\x13\xDFV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xD4\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xEEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x02W=`\0\x80>=`\0\xFD[PP`\x1FT`\x1ET`@Qc\x13@\x08\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x13@\x08\xD3\x93Pa\x08D\x92\x90\x91\x16\x90`\0\x90\x86\x90\x82\x90\x81\x90`\x04\x01a\x14\x06V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08rW=`\0\x80>=`\0\xFD[PPPP`\x1B`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xDAW=`\0\x80>=`\0\xFD[PPPPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\t\xB0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\trW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\t\x06V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W\x83\x82\x90`\0R` `\0 \x01\x80Ta\n\x14\x90a\x14AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n@\x90a\x14AV[\x80\x15a\n\x8DW\x80`\x1F\x10a\nbWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x8DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\npW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t\xF5V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0BoW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0B1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\xC5V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\t\xC8W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0B\xCA\x90a\x14AV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xF6\x90a\x14AV[\x80\x15a\x0CCW\x80`\x1F\x10a\x0C\x18Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0CCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C&W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0B\xABV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0CwWP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r}W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\r\x05\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x14{V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\x1F\x91a\x14\xACV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\\W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\raV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\ry\x91\x90a\x14\xC8V[\x91PP[\x91\x90PV[`@\x80Q\x81\x81R`\x1C\x81\x83\x01R\x7FYou are deploying on ChainID\0\0\0\0``\x82\x01RF` \x82\x01\x81\x90R\x91Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`\x01\x81\x14a\x0E/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10\xDA\x18Z[\x88\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`D\x82\x01R`d\x01a\x03\xB0V[`\x1C`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xC3q\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA6\x91\x90a\x13/V[`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Us\xECS\xBF\x91g\xF5\x0C\xDE\xB3\xAE\x10_V\t\x9A\xAA\xB9\x06\x1F\x83\x14a\x0F'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fsomething horribly wrong\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xB0V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x85W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x99W=`\0\x80>=`\0\xFD[PP`!T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pa\x0F\xB8\x91Pa\x11^V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x0F\xE4W=`\0\x80>=`\0\xFD[P`\x1D`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10iW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10}W=`\0\x80>=`\0\xFD[PP`\x1DT`@\x80Q\x81\x81R`\x15\x81\x83\x01Rt1\"\xA4\xA3\xA2\xA7/\xB4\xB6\xB862\xB6\xB2\xB7:0\xBA4\xB7\xB7`Y\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x93P\x90\x81\x90\x03`\x80\x01\x91P\xA1a\x10\xF3a\x05\xB3V[a\x10\xFBa\x02wV[PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xE9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xCBWPPPPP\x90P\x90V[a-\xEF\x80a\x14\xEB\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x11\xACW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x11\x85V[P\x90\x95\x94PPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x12eW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15a\x12MW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a\x12!V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x11\xDFV[P\x92\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a\x12\x8CW\x81\x81\x01Q\x83\x82\x01R` \x01a\x12tV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x12\xAD\x81` \x86\x01` \x86\x01a\x12qV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a\x12eW`?\x19\x87\x86\x03\x01\x84Ra\x13\x05\x85\x83Qa\x12\x95V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x12\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13AW`\0\x80\xFD[\x81Qa\x13L\x81a\x13\x1AV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x13eW`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\x04`@\x82\x01Rcdata`\xE0\x1B``\x82\x01R`\x80` \x82\x01R`\0a\x13L`\x80\x83\x01\x84a\x12\x95V[`\x01\x80`\xA0\x1B\x03\x87\x16\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0a\x13\xC1`\xC0\x83\x01\x87a\x12\x95V[``\x83\x01\x95\x90\x95RP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x14\0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a\x14-`\xA0\x83\x01\x86a\x12\x95V[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x14UW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14uWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x14\x9E\x81`\x04\x85\x01` \x87\x01a\x12qV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x14\xBE\x81\x84` \x87\x01a\x12qV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\xDAW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x13LW`\0\x80\xFD\xFE`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa-\xEF8\x03\x80a-\xEF\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\tV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Ra\0Da\0JV[Pa\x019V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\x01\x07W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15a\x01\x1BW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x012W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa,\x86a\x01i`\09`\0\x81\x81a\x05\xF8\x01R\x81\x81a\r\xDF\x01R\x81\x81a\x0E\n\x01Ra\x0E5\x01Ra,\x86`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02HW`\x005`\xE0\x1C\x80c~\xCE\xBE\0\x11a\x01;W\x80c\xAA'\x1E\x1A\x11a\0\xB8W\x80c\xDDb\xED>\x11a\0|W\x80c\xDDb\xED>\x14a\x05\x88W\x80c\xEBA_E\x14a\x05\x9BW\x80c\xF1\x12~\xD8\x14a\x05\xA3W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xE0W\x80c\xFD\xC3q\xCE\x14a\x05\xF3W`\0\x80\xFD[\x80c\xAA'\x1E\x1A\x14a\x05\x18W\x80c\xB8\xC2U\x94\x14a\x05<W\x80c\xC3\xCD\xA5 \x14a\x05OW\x80c\xC4\xD6m\xE8\x14a\x05bW\x80c\xD5\x05\xAC\xCF\x14a\x05uW`\0\x80\xFD[\x80c\x95\xD8\x9BA\x11a\0\xFFW\x80c\x95\xD8\x9BA\x14a\x04\xCDW\x80c\x9A\xB2N\xB0\x14a\x04\xD5W\x80c\x9A\xECK\xAE\x14a\x04\xE8W\x80c\xA4W\xC2\xD7\x14a\x04\xF2W\x80c\xA9\x05\x9C\xBB\x14a\x05\x05W`\0\x80\xFD[\x80c~\xCE\xBE\0\x14a\x04\\W\x80c\x84\xB0\x19n\x14a\x04oW\x80c\x8D\xA5\xCB[\x14a\x04\x8AW\x80c\x8ES\x9E\x8C\x14a\x04\x9BW\x80c\x91\xDD\xAD\xF4\x14a\x04\xAEW`\0\x80\xFD[\x80c@\xC1\x0F\x19\x11a\x01\xC9W\x80cf\xEB9\x9F\x11a\x01\x8DW\x80cf\xEB9\x9F\x14a\x03\xCCW\x80co\xCF\xFFE\x14a\x03\xDFW\x80cp\xA0\x821\x14a\x04\x07W\x80cqP\x18\xA6\x14a\x040W\x80cx\xAA3\xBA\x14a\x048W`\0\x80\xFD[\x80c@\xC1\x0F\x19\x14a\x03%W\x80cB\x96lh\x14a\x038W\x80cK\xF5\xD7\xE9\x14a\x03KW\x80cX|\xDE\x1E\x14a\x03uW\x80c\\\x19\xA9\\\x14a\x03\xB9W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\x02\x10W\x80c#\xB8r\xDD\x14a\x02\xD5W\x80c1<\xE5g\x14a\x02\xE8W\x80c6D\xE5\x15\x14a\x02\xF7W\x80c9P\x93Q\x14a\x02\xFFW\x80c:F\xB1\xA8\x14a\x03\x12W`\0\x80\xFD[\x80c\x04U\xE6\x94\x14a\x02MW\x80c\x06\xFD\xDE\x03\x14a\x02\x86W\x80c\t^\xA7\xB3\x14a\x02\x9BW\x80c\x18\x16\r\xDD\x14a\x02\xAEW\x80c\x1F\xFA\xCD\xEF\x14a\x02\xC0W[`\0\x80\xFD[a\x02qa\x02[6`\x04a'\nV[a\x011` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x8Ea\x06\x1AV[`@Qa\x02}\x91\x90a'kV[a\x02qa\x02\xA96`\x04a'~V[a\x06\xACV[`gT[`@Q\x90\x81R` \x01a\x02}V[a\x02\xD3a\x02\xCE6`\x04a'\xA8V[a\x06\xC6V[\0[a\x02qa\x02\xE36`\x04a'\xE4V[a\x06\xDCV[`@Q`\x12\x81R` \x01a\x02}V[a\x02\xB2a\x07\0V[a\x02qa\x03\r6`\x04a'~V[a\x07\x0FV[a\x02\xB2a\x03 6`\x04a'~V[a\x071V[a\x02\xD3a\x0336`\x04a'~V[a\x07\xBBV[a\x02\xD3a\x03F6`\x04a(!V[a\x087V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\x8EV[a\x03\xA1a\x03\x836`\x04a'\nV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\xFE` R`@\x90 T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02}V[a\x02\xD3a\x03\xC76`\x04a'\nV[a\x08DV[a\x02\xD3a\x03\xDA6`\x04a'\xA8V[a\x08NV[a\x03\xF2a\x03\xED6`\x04a'\nV[a\x08\xC7V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02}V[a\x02\xB2a\x04\x156`\x04a'\nV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`e` R`@\x90 T\x90V[a\x02\xD3a\x08\xE9V[a\x02qa\x04F6`\x04a'\nV[a\x012` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xB2a\x04j6`\x04a'\nV[a\x08\xFDV[a\x04wa\t\x1BV[`@Qa\x02}\x97\x96\x95\x94\x93\x92\x91\x90a(:V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA1V[a\x02\xB2a\x04\xA96`\x04a(!V[a\t\xB9V[a\x04\xB6a\n!V[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02}V[a\x02\x8Ea\n,V[a\x02\xB2a\x04\xE36`\x04a'\nV[a\n;V[a\x02\xB2a\x010T\x81V[a\x02qa\x05\x006`\x04a'~V[a\n\xBDV[a\x02qa\x05\x136`\x04a'~V[a\x0B8V[a\x02qa\x05&6`\x04a'\nV[a\x013` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xD3a\x05J6`\x04a'\xA8V[a\x0BFV[a\x02\xD3a\x05]6`\x04a(\xE3V[a\x0BXV[a\x02\xD3a\x05p6`\x04a'\nV[a\x0C\x8EV[a\x02\xD3a\x05\x836`\x04a);V[a\x0E\xD8V[a\x02\xB2a\x05\x966`\x04a)\xA6V[a\x10<V[a\x02\xD3a\x10gV[a\x05\xB6a\x05\xB16`\x04a)\xD9V[a\x117V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02}V[a\x02\xD3a\x05\xEE6`\x04a'\nV[a\x11\xBBV[a\x03\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[```h\x80Ta\x06)\x90a*\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06U\x90a*\x0EV[\x80\x15a\x06\xA2W\x80`\x1F\x10a\x06wWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xA2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x85W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\x003a\x06\xBA\x81\x85\x85a\x121V[`\x01\x91PP[\x92\x91PPV[a\x06\xCEa\x13UV[a\x06\xD8\x82\x82a\x13\xAFV[PPV[`\x003a\x06\xEA\x85\x82\x85a\x14\x10V[a\x06\xF5\x85\x85\x85a\x14\x8AV[P`\x01\x94\x93PPPPV[`\0a\x07\na\x16FV[\x90P\x90V[`\x003a\x06\xBA\x81\x85\x85a\x07\"\x83\x83a\x10<V[a\x07,\x91\x90a*XV[a\x121V[`\0a\x07;a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 a\x07\xB4\x90\x83a\x16PV[\x93\x92PPPV[3`\0\x90\x81Ra\x013` R`@\x90 T`\xFF\x16a\x08-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBackingEigen.mint: caller is not`D\x82\x01Rh\x100\x906\xB4\xB7:2\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x06\xD8\x82\x82a\x179V[a\x08A3\x82a\x17\xC4V[PV[a\x08A3\x82a\x17\xDDV[a\x08Va\x13UV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x01$\xB1%\x03\xBD\xDC&\x16\xC0\xF3\xF5O\xD2>\xD2\x83\xF5\xEF\x0C\x14\x83\xA7T\t\xE4&\x12\x17k\x8B\xDE\x82`@Qa\x08\x93\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81Ra\x013` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 Ta\x06\xC0\x90a\x18WV[a\x08\xF1a\x13UV[a\x08\xFB`\0a\x18\xC0V[V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x81 Ta\x06\xC0V[`\0``\x80`\0\x80`\0```\x97T`\0\x80\x1B\x14\x80\x15a\t;WP`\x98T\x15[a\t\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07\x89V[a\t\x87a\x19\x12V[a\t\x8Fa\x19!V[`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[`\0a\t\xC3a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\n\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07\x89V[a\x06\xC0a\x01\0\x83a\x16PV[`\0a\x07\nBa\x190V[```i\x80Ta\x06)\x90a*\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xFF` R`@\x81 T\x80\x15a\n\xAAW`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80T`\0\x19\x83\x01\x90\x81\x10a\n\x88Wa\n\x88a*\x81V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\n\xADV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x003\x81a\n\xCB\x82\x86a\x10<V[\x90P\x83\x81\x10\x15a\x0B+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x06\xF5\x82\x86\x86\x84\x03a\x121V[`\x003a\x06\xBA\x81\x85\x85a\x14\x8AV[a\x0BNa\x13UV[a\x06\xD8\x82\x82a\x19\x97V[\x83B\x11\x15a\x0B\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Votes: signature expired\0\0\0`D\x82\x01R`d\x01a\x07\x89V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\0\x90a\x0C\"\x90a\x0C\x1A\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19\xF0V[\x85\x85\x85a\x1A\x1DV[\x90Pa\x0C-\x81a\x1AEV[\x86\x14a\x0C{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC20Votes: invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x89V[a\x0C\x85\x81\x88a\x17\xDDV[PPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0C\xAEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0C\xC8WP0;\x15\x80\x15a\x0C\xC8WP`\0T`\xFF\x16`\x01\x14[a\r+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\rNW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\rVa\x1AmV[a\r\xA3`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l!0\xB1\xB5\xB4\xB73\x90\"\xB4\xB3\xB2\xB7`\x99\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1A\x9CV[a\r\xAC\x82a\x18\xC0V[a\r\xD3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1A\xCDV[`\0\x19a\x010Ua\x0E\x05\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x13\xAFV[a\x0E0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x19\x97V[a\x0Ef\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0k\x05hhw\xAF\xB5\xCB\xCC\xBFs@\0a\x179V[`@Q\x7F\xB7\xC2<\x1E.6\xF2\x98\xE9\x87\x9A\x88\xEC\xFC\xD0~(\xFB\xB49\xBC\xFA\x9Cx\xCA\x13c\xCA\x147\r&\x90`\0\x90\xA1\x80\x15a\x06\xD8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[\x83B\x11\x15a\x0F(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x07\x89V[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x0FW\x8Ca\x1AEV[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x0F\xB2\x82a\x19\xF0V[\x90P`\0a\x0F\xC2\x82\x87\x87\x87a\x1A\x1DV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x07\x89V[a\x100\x8A\x8A\x8Aa\x121V[PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`f` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x10oa\x13UV[`\0\x19a\x010T\x14a\x11\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FBackingEigen.disableTransferRest`D\x82\x01R\x7Frictions: transfer restrictions `d\x82\x01Rs\x18\\\x99H\x18[\x1C\x99XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`b\x1B`\x84\x82\x01R`\xA4\x01a\x07\x89V[`\0a\x010\x81\x90U`@Q\x7F+\x18\x98m;\xA8\t\xDB/\x13\xA5\xD7\xBF\x17\xF6\r5{7\xD9\xCB\xB5]\xD7\x1C\xBB\xAC\x8D\xC4\x06\x0Fd\x91\x90\xA1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x11{Wa\x11{a*\x81V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x11\xC3a\x13UV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x08A\x81a\x18\xC0V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x12\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`f` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x011` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x14\x1C\x84\x84a\x10<V[\x90P`\0\x19\x81\x14a\x14\x84W\x81\x81\x10\x15a\x14wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07\x89V[a\x14\x84\x84\x84\x84\x84\x03a\x121V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x14\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x15PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x15[\x83\x83\x83a\x1B\x17V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a\x15\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`e` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x163\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x14\x84\x84\x84\x84a\x1B\xF5V[`\0a\x07\na\x1C'V[\x81T`\0\x90\x81\x81`\x05\x81\x11\x15a\x16\xAAW`\0a\x16k\x84a\x1C\x9BV[a\x16u\x90\x85a*\x97V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\x9AW\x80\x91Pa\x16\xA8V[a\x16\xA5\x81`\x01a*XV[\x92P[P[\x80\x82\x10\x15a\x16\xF7W`\0a\x16\xBE\x83\x83a\x1D\x83V[`\0\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\xE3W\x80\x91Pa\x16\xF1V[a\x16\xEE\x81`\x01a*XV[\x92P[Pa\x16\xAAV[\x80\x15a\x17#W`\0\x86\x81R` \x90 \x81\x01`\0\x19\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x17&V[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[a\x17C\x82\x82a\x1D\x9EV[`gT`\x01`\x01`\xE0\x1B\x03\x10\x15a\x17\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC20Votes: total supply risks o`D\x82\x01Roverflowing votes`\x80\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a\x14\x84a\x01\0a\x1Es\x83a\x1E\x7FV[a\x17\xCE\x82\x82a\x1F\xF4V[a\x14\x84a\x01\0a!;\x83a\x1E\x7FV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\xFE` \x81\x81R`@\x80\x84 \x80T`e\x84R\x82\x86 T\x94\x90\x93R\x87\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x84\x16\x81\x17\x90\x91U\x90Q\x91\x90\x95\x16\x94\x91\x93\x91\x92\x85\x92\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x14\x84\x82\x84\x83a!GV[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[P\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\x99\x80Ta\x06)\x90a*\x0EV[```\x9A\x80Ta\x06)\x90a*\x0EV[`\0e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81Ra\x012` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7Fr\xA5a\xD1\xAFt\tF}\xAEO\x1E\x9F\xC5%\x90\xA93Z\x1D\xDA\x17r~+j\xA8\xC4\xDB5\x10\x9B\x91\x01a\x14\x04V[`\0a\x06\xC0a\x19\xFDa\x16FV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[`\0\x80`\0a\x1A.\x87\x87\x87\x87a\"\x84V[\x91P\x91Pa\x1A;\x81a#HV[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCB` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x08\xFBa$\x92V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x06\xD8\x82\x82a$\xC2V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x08A\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RPa%\x02V[a\x010TB\x11a\x1B\xF0W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81Ra\x011` R`@\x90 T`\xFF\x16\x80a\x1BbWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81Ra\x012` R`@\x90 T`\xFF\x16[\x80a\x1BtWP`\x01`\x01`\xA0\x1B\x03\x83\x16\x15[a\x1B\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FBackingEigen._beforeTokenTransfe`D\x82\x01R\x7Fr: from or to must be whiteliste`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x07\x89V[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\xFE` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x1B\xF0\x92\x91\x82\x16\x91\x16\x83a!GV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1CRa%QV[a\x1CZa%\xAAV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81`\0\x03a\x1C\xADWP`\0\x91\x90PV[`\0`\x01a\x1C\xBA\x84a%\xDBV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x1C\xD3Wa\x1C\xD3a*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xEBWa\x1C\xEBa*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D\x03Wa\x1D\x03a*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D\x1BWa\x1D\x1Ba*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D3Wa\x1D3a*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1DKWa\x1DKa*\xF5V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1DcWa\x1Dca*\xF5V[\x04\x82\x01\x90\x1C\x90Pa\x07\xB4\x81\x82\x85\x81a\x1D}Wa\x1D}a*\xF5V[\x04a&oV[`\0a\x1D\x92`\x02\x84\x84\x18a+\x0BV[a\x07\xB4\x90\x84\x84\x16a*XV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07\x89V[a\x1E\0`\0\x83\x83a\x1B\x17V[\x80`g`\0\x82\x82Ta\x1E\x12\x91\x90a*XV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x06\xD8`\0\x83\x83a\x1B\xF5V[`\0a\x07\xB4\x82\x84a*XV[\x82T`\0\x90\x81\x90\x81\x81\x15a\x1E\xCCW`\0\x87\x81R` \x90 \x82\x01`\0\x19\x01`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16` \x82\x01Ra\x1E\xE1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x90P\x80` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93Pa\x1F\x01\x84\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x92P`\0\x82\x11\x80\x15a\x1F+WPa\x1F\x16a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x1FpWa\x1F9\x83a&\x85V[`\0\x88\x81R` \x90 \x83\x01`\0\x19\x01\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1F\xEAV[\x86`@Q\x80`@\x01`@R\x80a\x1F\x94a\x1F\x87a\n!V[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18WV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x1F\xA8\x86a&\x85V[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[PP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[a `\x82`\0\x83a\x1B\x17V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a \xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`e` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`g\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x1B\xF0\x83`\0\x84a\x1B\xF5V[`\0a\x07\xB4\x82\x84a*\x97V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!iWP`\0\x81\x11[\x15a\x1B\xF0W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a!\xF7W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a!\xA4\x90a!;\x85a\x1E\x7FV[\x91P\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!\xEC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x1B\xF0W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xFF` R`@\x81 \x81\x90a\"-\x90a\x1Es\x85a\x1E\x7FV[\x91P\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa\"u\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\xBBWP`\0\x90P`\x03a#?V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a#\x0FW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a#8W`\0`\x01\x92P\x92PPa#?V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15a#\\Wa#\\a+-V[\x03a#dWPV[`\x01\x81`\x04\x81\x11\x15a#xWa#xa+-V[\x03a#\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x89V[`\x02\x81`\x04\x81\x11\x15a#\xD9Wa#\xD9a+-V[\x03a$&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x07\x89V[`\x03\x81`\x04\x81\x11\x15a$:Wa$:a+-V[\x03a\x08AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x07\x89V[`\0Ta\x01\0\x90\x04`\xFF\x16a$\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[a\x08\xFB3a\x18\xC0V[`\0Ta\x01\0\x90\x04`\xFF\x16a$\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[`ha$\xF5\x83\x82a+\x91V[P`ia\x1B\xF0\x82\x82a+\x91V[`\0Ta\x01\0\x90\x04`\xFF\x16a%)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x89\x90a*\xAAV[`\x99a%5\x83\x82a+\x91V[P`\x9Aa%B\x82\x82a+\x91V[PP`\0`\x97\x81\x90U`\x98UPV[`\0\x80a%\\a\x19\x12V[\x80Q\x90\x91P\x15a%sW\x80Q` \x90\x91\x01 \x91\x90PV[`\x97T\x80\x15a%\x82W\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[`\0\x80a%\xB5a\x19!V[\x80Q\x90\x91P\x15a%\xCCW\x80Q` \x90\x91\x01 \x91\x90PV[`\x98T\x80\x15a%\x82W\x92\x91PPV[`\0\x80`\x80\x83\x90\x1C\x15a%\xF0W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a&\x02W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a&\x14W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a&&W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a&8W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a&JW`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a&\\W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x06\xC0W`\x01\x01\x92\x91PPV[`\0\x81\x83\x10a&~W\x81a\x07\xB4V[P\x90\x91\x90PV[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x18\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07\x89V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a'\x05W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a'\x1CW`\0\x80\xFD[a\x07\xB4\x82a&\xEEV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a'KW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a'/V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x07\xB4` \x83\x01\x84a'%V[`\0\x80`@\x83\x85\x03\x12\x15a'\x91W`\0\x80\xFD[a'\x9A\x83a&\xEEV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a'\xBBW`\0\x80\xFD[a'\xC4\x83a&\xEEV[\x91P` \x83\x015\x80\x15\x15\x81\x14a'\xD9W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xF9W`\0\x80\xFD[a(\x02\x84a&\xEEV[\x92Pa(\x10` \x85\x01a&\xEEV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a(3W`\0\x80\xFD[P5\x91\x90PV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R`\0a(Y`\xE0\x83\x01\x89a'%V[\x82\x81\x03`@\x84\x01Ra(k\x81\x89a'%V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90`\0[\x81\x81\x10\x15a(\xC1W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(\xA3V[P\x90\x9B\x9APPPPPPPPPPPV[\x805`\xFF\x81\x16\x81\x14a'\x05W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a(\xFCW`\0\x80\xFD[a)\x05\x87a&\xEEV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa)!``\x88\x01a(\xD2V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a)VW`\0\x80\xFD[a)_\x88a&\xEEV[\x96Pa)m` \x89\x01a&\xEEV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa)\x89`\x80\x89\x01a(\xD2V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a)\xB9W`\0\x80\xFD[a)\xC2\x83a&\xEEV[\x91Pa)\xD0` \x84\x01a&\xEEV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a)\xECW`\0\x80\xFD[a)\xF5\x83a&\xEEV[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'\xD9W`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a*\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1AgWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xC0Wa\x06\xC0a*BV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xC0Wa\x06\xC0a*BV[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a+(WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x1B\xF0W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a+jWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a+\x8AW`\0\x81U`\x01\x01a+vV[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xABWa+\xABa*kV[a+\xBF\x81a+\xB9\x84Ta*\x0EV[\x84a+CV[` `\x1F\x82\x11`\x01\x81\x14a+\xF3W`\0\x83\x15a+\xDBWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua+\x8AV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a,#W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a,\x03V[P\x84\x82\x10\x15a,AW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xEC\xE6As\x9D\xB4\xEC\xD9\xD6\xC5\x83m\xEB/\xDF\xA1\x9B?\x0C\xB2YT\x05r\xE9\xF7\x17\x11\xF0\x02B-dsolcC\0\x08\x1B\x003\xA2dipfsX\"\x12 s\xB7HH\x8AvL\x80ef\xA1\x0Bu){N\x1E\n\xE89\xB6\x9D\xA6\xF5\x9Fn\xDA\x82B\xD4\x972dsolcC\0\x08\x1B\x003",
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
    /**Function with signature `EIGEN_addressBefore()` and selector `0x370c30d9`.
```solidity
function EIGEN_addressBefore() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIGEN_addressBeforeCall {}
    ///Container type for the return parameters of the [`EIGEN_addressBefore()`](EIGEN_addressBeforeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIGEN_addressBeforeReturn {
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
            impl ::core::convert::From<EIGEN_addressBeforeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EIGEN_addressBeforeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EIGEN_addressBeforeCall {
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
            impl ::core::convert::From<EIGEN_addressBeforeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EIGEN_addressBeforeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EIGEN_addressBeforeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EIGEN_addressBeforeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = EIGEN_addressBeforeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EIGEN_addressBefore()";
            const SELECTOR: [u8; 4] = [55u8, 12u8, 48u8, 217u8];
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
    /**Function with signature `bEIGEN_ProxyAdmin()` and selector `0xd715bc6e`.
```solidity
function bEIGEN_ProxyAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_ProxyAdminCall {}
    ///Container type for the return parameters of the [`bEIGEN_ProxyAdmin()`](bEIGEN_ProxyAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_ProxyAdminReturn {
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
            impl ::core::convert::From<bEIGEN_ProxyAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_ProxyAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_ProxyAdminCall {
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
            impl ::core::convert::From<bEIGEN_ProxyAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_ProxyAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_ProxyAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_ProxyAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_ProxyAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN_ProxyAdmin()";
            const SELECTOR: [u8; 4] = [215u8, 21u8, 188u8, 110u8];
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
    /**Function with signature `bEIGEN_TimelockAdmin()` and selector `0x35e4a036`.
```solidity
function bEIGEN_TimelockAdmin() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_TimelockAdminCall {}
    ///Container type for the return parameters of the [`bEIGEN_TimelockAdmin()`](bEIGEN_TimelockAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_TimelockAdminReturn {
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
            impl ::core::convert::From<bEIGEN_TimelockAdminCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_TimelockAdminCall {
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
            impl ::core::convert::From<bEIGEN_TimelockAdminReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_TimelockAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_TimelockAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_TimelockAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN_TimelockAdmin()";
            const SELECTOR: [u8; 4] = [53u8, 228u8, 160u8, 54u8];
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
    /**Function with signature `bEIGEN_TimelockController()` and selector `0x811ffeeb`.
```solidity
function bEIGEN_TimelockController() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_TimelockControllerCall {}
    ///Container type for the return parameters of the [`bEIGEN_TimelockController()`](bEIGEN_TimelockControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_TimelockControllerReturn {
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
            impl ::core::convert::From<bEIGEN_TimelockControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_TimelockControllerCall {
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
            impl ::core::convert::From<bEIGEN_TimelockControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_TimelockControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_TimelockControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_TimelockControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN_TimelockController()";
            const SELECTOR: [u8; 4] = [129u8, 31u8, 254u8, 235u8];
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
    /**Function with signature `bEIGEN_implementation()` and selector `0xc3ea3fc9`.
```solidity
function bEIGEN_implementation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_implementationCall {}
    ///Container type for the return parameters of the [`bEIGEN_implementation()`](bEIGEN_implementationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_implementationReturn {
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
            impl ::core::convert::From<bEIGEN_implementationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_implementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_implementationCall {
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
            impl ::core::convert::From<bEIGEN_implementationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_implementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for bEIGEN_implementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_implementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_implementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN_implementation()";
            const SELECTOR: [u8; 4] = [195u8, 234u8, 63u8, 201u8];
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
    /**Function with signature `bEIGEN_proxy()` and selector `0xa5617cfd`.
```solidity
function bEIGEN_proxy() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_proxyCall {}
    ///Container type for the return parameters of the [`bEIGEN_proxy()`](bEIGEN_proxyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bEIGEN_proxyReturn {
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
            impl ::core::convert::From<bEIGEN_proxyCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_proxyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_proxyCall {
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
            impl ::core::convert::From<bEIGEN_proxyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_proxyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_proxyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_proxyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_proxyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bEIGEN_proxy()";
            const SELECTOR: [u8; 4] = [165u8, 97u8, 124u8, 253u8];
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
    /**Function with signature `checkUpgradeCorrectness()` and selector `0x14f8ffac`.
```solidity
function checkUpgradeCorrectness() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkUpgradeCorrectnessCall {}
    ///Container type for the return parameters of the [`checkUpgradeCorrectness()`](checkUpgradeCorrectnessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkUpgradeCorrectnessReturn {}
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
            impl ::core::convert::From<checkUpgradeCorrectnessCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkUpgradeCorrectnessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkUpgradeCorrectnessCall {
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
            impl ::core::convert::From<checkUpgradeCorrectnessReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkUpgradeCorrectnessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkUpgradeCorrectnessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkUpgradeCorrectnessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkUpgradeCorrectnessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkUpgradeCorrectness()";
            const SELECTOR: [u8; 4] = [20u8, 248u8, 255u8, 172u8];
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
    /**Function with signature `run()` and selector `0xc0406226`.
```solidity
function run() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct runCall {}
    ///Container type for the return parameters of the [`run()`](runCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct runReturn {}
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
            impl ::core::convert::From<runCall> for UnderlyingRustTuple<'_> {
                fn from(value: runCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for runCall {
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
            impl ::core::convert::From<runReturn> for UnderlyingRustTuple<'_> {
                fn from(value: runReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for runReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for runCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = runReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "run()";
            const SELECTOR: [u8; 4] = [192u8, 64u8, 98u8, 38u8];
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
    /**Function with signature `simulatePerformingUpgrade()` and selector `0x54755b99`.
```solidity
function simulatePerformingUpgrade() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulatePerformingUpgradeCall {}
    ///Container type for the return parameters of the [`simulatePerformingUpgrade()`](simulatePerformingUpgradeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulatePerformingUpgradeReturn {}
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
            impl ::core::convert::From<simulatePerformingUpgradeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulatePerformingUpgradeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulatePerformingUpgradeCall {
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
            impl ::core::convert::From<simulatePerformingUpgradeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: simulatePerformingUpgradeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for simulatePerformingUpgradeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for simulatePerformingUpgradeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = simulatePerformingUpgradeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "simulatePerformingUpgrade()";
            const SELECTOR: [u8; 4] = [84u8, 117u8, 91u8, 153u8];
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
    ///Container for all the [`bEIGEN_upgrade`](self) function calls.
    pub enum bEIGEN_upgradeCalls {
        EIGEN_addressBefore(EIGEN_addressBeforeCall),
        IS_SCRIPT(IS_SCRIPTCall),
        IS_TEST(IS_TESTCall),
        bEIGEN_ProxyAdmin(bEIGEN_ProxyAdminCall),
        bEIGEN_TimelockAdmin(bEIGEN_TimelockAdminCall),
        bEIGEN_TimelockController(bEIGEN_TimelockControllerCall),
        bEIGEN_implementation(bEIGEN_implementationCall),
        bEIGEN_proxy(bEIGEN_proxyCall),
        checkUpgradeCorrectness(checkUpgradeCorrectnessCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        run(runCall),
        simulatePerformingUpgrade(simulatePerformingUpgradeCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
    }
    #[automatically_derived]
    impl bEIGEN_upgradeCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [20u8, 248u8, 255u8, 172u8],
            [30u8, 215u8, 131u8, 28u8],
            [53u8, 228u8, 160u8, 54u8],
            [55u8, 12u8, 48u8, 217u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [84u8, 117u8, 91u8, 153u8],
            [102u8, 217u8, 169u8, 160u8],
            [129u8, 31u8, 254u8, 235u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [165u8, 97u8, 124u8, 253u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [192u8, 64u8, 98u8, 38u8],
            [195u8, 234u8, 63u8, 201u8],
            [215u8, 21u8, 188u8, 110u8],
            [226u8, 12u8, 159u8, 113u8],
            [248u8, 204u8, 191u8, 71u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for bEIGEN_upgradeCalls {
        const NAME: &'static str = "bEIGEN_upgradeCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EIGEN_addressBefore(_) => {
                    <EIGEN_addressBeforeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_SCRIPT(_) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bEIGEN_ProxyAdmin(_) => {
                    <bEIGEN_ProxyAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bEIGEN_TimelockAdmin(_) => {
                    <bEIGEN_TimelockAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bEIGEN_TimelockController(_) => {
                    <bEIGEN_TimelockControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bEIGEN_implementation(_) => {
                    <bEIGEN_implementationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bEIGEN_proxy(_) => {
                    <bEIGEN_proxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkUpgradeCorrectness(_) => {
                    <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::run(_) => <runCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::simulatePerformingUpgrade(_) => {
                    <simulatePerformingUpgradeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls>] = &[
                {
                    fn checkUpgradeCorrectness(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::checkUpgradeCorrectness)
                    }
                    checkUpgradeCorrectness
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn bEIGEN_TimelockAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <bEIGEN_TimelockAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::bEIGEN_TimelockAdmin)
                    }
                    bEIGEN_TimelockAdmin
                },
                {
                    fn EIGEN_addressBefore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <EIGEN_addressBeforeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::EIGEN_addressBefore)
                    }
                    EIGEN_addressBefore
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn simulatePerformingUpgrade(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <simulatePerformingUpgradeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::simulatePerformingUpgrade)
                    }
                    simulatePerformingUpgrade
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn bEIGEN_TimelockController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <bEIGEN_TimelockControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::bEIGEN_TimelockController)
                    }
                    bEIGEN_TimelockController
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn bEIGEN_proxy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <bEIGEN_proxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::bEIGEN_proxy)
                    }
                    bEIGEN_proxy
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::failed)
                    }
                    failed
                },
                {
                    fn run(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <runCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::run)
                    }
                    run
                },
                {
                    fn bEIGEN_implementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <bEIGEN_implementationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::bEIGEN_implementation)
                    }
                    bEIGEN_implementation
                },
                {
                    fn bEIGEN_ProxyAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <bEIGEN_ProxyAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::bEIGEN_ProxyAdmin)
                    }
                    bEIGEN_ProxyAdmin
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_SCRIPT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::IS_SCRIPT)
                    }
                    IS_SCRIPT
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(bEIGEN_upgradeCalls::IS_TEST)
                    }
                    IS_TEST
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
                Self::EIGEN_addressBefore(inner) => {
                    <EIGEN_addressBeforeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGEN_ProxyAdmin(inner) => {
                    <bEIGEN_ProxyAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bEIGEN_TimelockAdmin(inner) => {
                    <bEIGEN_TimelockAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bEIGEN_TimelockController(inner) => {
                    <bEIGEN_TimelockControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bEIGEN_implementation(inner) => {
                    <bEIGEN_implementationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bEIGEN_proxy(inner) => {
                    <bEIGEN_proxyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkUpgradeCorrectness(inner) => {
                    <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::run(inner) => {
                    <runCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::simulatePerformingUpgrade(inner) => {
                    <simulatePerformingUpgradeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EIGEN_addressBefore(inner) => {
                    <EIGEN_addressBeforeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::bEIGEN_ProxyAdmin(inner) => {
                    <bEIGEN_ProxyAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bEIGEN_TimelockAdmin(inner) => {
                    <bEIGEN_TimelockAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bEIGEN_TimelockController(inner) => {
                    <bEIGEN_TimelockControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bEIGEN_implementation(inner) => {
                    <bEIGEN_implementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bEIGEN_proxy(inner) => {
                    <bEIGEN_proxyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkUpgradeCorrectness(inner) => {
                    <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::run(inner) => {
                    <runCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::simulatePerformingUpgrade(inner) => {
                    <simulatePerformingUpgradeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`bEIGEN_upgrade`](self) events.
    pub enum bEIGEN_upgradeEvents {
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
    impl bEIGEN_upgradeEvents {
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
    impl alloy_sol_types::SolEventInterface for bEIGEN_upgradeEvents {
        const NAME: &'static str = "bEIGEN_upgradeEvents";
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
    impl alloy_sol_types::private::IntoLogData for bEIGEN_upgradeEvents {
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
    /**Creates a new wrapper around an on-chain [`bEIGEN_upgrade`](self) contract instance.

See the [wrapper's documentation](`bEIGEN_upgradeInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> bEIGEN_upgradeInstance<T, P, N> {
        bEIGEN_upgradeInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<bEIGEN_upgradeInstance<T, P, N>>,
    > {
        bEIGEN_upgradeInstance::<T, P, N>::deploy(provider)
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
        bEIGEN_upgradeInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`bEIGEN_upgrade`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`bEIGEN_upgrade`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct bEIGEN_upgradeInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for bEIGEN_upgradeInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("bEIGEN_upgradeInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > bEIGEN_upgradeInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`bEIGEN_upgrade`](self) contract instance.

See the [wrapper's documentation](`bEIGEN_upgradeInstance`) for more details.*/
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
        ) -> alloy_contract::Result<bEIGEN_upgradeInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> bEIGEN_upgradeInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> bEIGEN_upgradeInstance<T, P, N> {
            bEIGEN_upgradeInstance {
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
    > bEIGEN_upgradeInstance<T, P, N> {
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
        ///Creates a new call builder for the [`EIGEN_addressBefore`] function.
        pub fn EIGEN_addressBefore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EIGEN_addressBeforeCall, N> {
            self.call_builder(&EIGEN_addressBeforeCall {})
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
        ///Creates a new call builder for the [`bEIGEN_ProxyAdmin`] function.
        pub fn bEIGEN_ProxyAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, bEIGEN_ProxyAdminCall, N> {
            self.call_builder(&bEIGEN_ProxyAdminCall {})
        }
        ///Creates a new call builder for the [`bEIGEN_TimelockAdmin`] function.
        pub fn bEIGEN_TimelockAdmin(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, bEIGEN_TimelockAdminCall, N> {
            self.call_builder(&bEIGEN_TimelockAdminCall {})
        }
        ///Creates a new call builder for the [`bEIGEN_TimelockController`] function.
        pub fn bEIGEN_TimelockController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, bEIGEN_TimelockControllerCall, N> {
            self.call_builder(&bEIGEN_TimelockControllerCall {})
        }
        ///Creates a new call builder for the [`bEIGEN_implementation`] function.
        pub fn bEIGEN_implementation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, bEIGEN_implementationCall, N> {
            self.call_builder(&bEIGEN_implementationCall {})
        }
        ///Creates a new call builder for the [`bEIGEN_proxy`] function.
        pub fn bEIGEN_proxy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, bEIGEN_proxyCall, N> {
            self.call_builder(&bEIGEN_proxyCall {})
        }
        ///Creates a new call builder for the [`checkUpgradeCorrectness`] function.
        pub fn checkUpgradeCorrectness(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkUpgradeCorrectnessCall, N> {
            self.call_builder(&checkUpgradeCorrectnessCall {})
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
        ///Creates a new call builder for the [`run`] function.
        pub fn run(&self) -> alloy_contract::SolCallBuilder<T, &P, runCall, N> {
            self.call_builder(&runCall {})
        }
        ///Creates a new call builder for the [`simulatePerformingUpgrade`] function.
        pub fn simulatePerformingUpgrade(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, simulatePerformingUpgradeCall, N> {
            self.call_builder(&simulatePerformingUpgradeCall {})
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > bEIGEN_upgradeInstance<T, P, N> {
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
