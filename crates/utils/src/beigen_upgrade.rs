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
        pub selectors: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("StdInvariantInstance")
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
        > StdInvariantInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

        See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
        > StdInvariantInstance<T, P, N>
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
        > StdInvariantInstance<T, P, N>
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
    ///0x60806040525f805460ff199081166001908117909255600480549091169091179055601b8054747109709ecfa91a80626ff3989d68f67f5b1dd12d016001600160a81b0319909116179055601c80546001600160a01b03199081167383e9115d334d248ce39a6f36144aeab5b3456e7517909155601e80548216733f5ab2d4418d38568705bfd6672630fcc3435cc9179055601f8054821673d6ec41e453c5e7da5494f4d51a053ab571712e6f1790556020805490911673bb00dda2832850a43840a3a86515e3fe226865f217905534801560d8575f5ffd5b5061418d806100e65f395ff3fe608060405234801561000f575f5ffd5b5060043610610127575f3560e01c8063916a17c6116100a9578063c3ea3fc91161006e578063c3ea3fc914610229578063d715bc6e1461023c578063e20c9f711461024f578063f8ccbf4714610257578063fa7626d414610264575f5ffd5b8063916a17c6146101e6578063a5617cfd146101ee578063b5508aa914610201578063ba414fa614610209578063c040622614610221575f5ffd5b80633f7286f4116100ef5780633f7286f41461019957806354755b99146101a157806366d9a9a0146101a9578063811ffeeb146101be57806385226c81146101d1575f5ffd5b806314f8ffac1461012b5780631ed7831c1461013557806335e4a03614610153578063370c30d91461017e5780633e5e3c2314610191575b5f5ffd5b610133610270565b005b61013d610481565b60405161014a9190611106565b60405180910390f35b602054610166906001600160a01b031681565b6040516001600160a01b03909116815260200161014a565b602154610166906001600160a01b031681565b61013d6104e1565b61013d61053f565b61013361059d565b6101b16108ae565b60405161014a9190611151565b601f54610166906001600160a01b031681565b6101d9610998565b60405161014a9190611236565b6101b1610a63565b601c54610166906001600160a01b031681565b6101d9610b44565b610211610c0f565b604051901515815260200161014a565b610133610d32565b601d54610166906001600160a01b031681565b601e54610166906001600160a01b031681565b61013d61109b565b601b546102119060ff1681565b5f546102119060ff1681565b601f5460405163ca669fa760e01b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ca669fa7906024015f604051808303815f87803b1580156102c6575f5ffd5b505af11580156102d8573d5f5f3e3d5ffd5b5050601d54601e54601c546040516310270e3d60e11b81526001600160a01b039182166004820152928116945016915063204e1c7a90602401602060405180830381865afa15801561032c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061035091906112a1565b6001600160a01b0316146103ab5760405162461bcd60e51b815260206004820152601e60248201527f696d706c656d656e746174696f6e2073657420696e636f72726563746c79000060448201526064015b60405180910390fd5b602154601c5460408051637ee1b8e760e11b815290516001600160a01b03938416939092169163fdc371ce916004808201926020929091908290030181865afa1580156103fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061041e91906112a1565b6001600160a01b03161461047f5760405162461bcd60e51b815260206004820152602260248201527f454947454e2061646472657373206368616e67656420756e65787065637465646044820152616c7960f01b60648201526084016103a2565b565b6060600d8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116104b9575b5050505050905090565b6060600f8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116104b9575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116104b9575050505050905090565b601f546040805163793d064960e11b815290515f926001600160a01b03169163f27a0c929160048083019260209291908290030181865afa1580156105e4573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061060891906112c3565b601c54601d54604080516001600160a01b0393841660248201529290911660448084019190915281518084039091018152606490920181526020820180516001600160e01b031663266a23b160e21b17905251919250907fd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf189061068c9083906112da565b60405180910390a16020546040516303223eab60e11b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b1580156106ea575f5ffd5b505af11580156106fc573d5f5f3e3d5ffd5b5050601f54601e5460405162ea831560e11b81526001600160a01b0392831694506301d5062a935061073e92909116905f908690829081908a90600401611307565b5f604051808303815f87803b158015610755575f5ffd5b505af1158015610767573d5f5f3e3d5ffd5b50737109709ecfa91a80626ff3989d68f67f5b1dd12d925063e5d6bf0291506107929050844261134b565b6040518263ffffffff1660e01b81526004016107b091815260200190565b5f604051808303815f87803b1580156107c7575f5ffd5b505af11580156107d9573d5f5f3e3d5ffd5b5050601f54601e5460405163134008d360e01b81526001600160a01b03928316945063134008d3935061081a92909116905f90869082908190600401611370565b5f604051808303815f87803b158015610831575f5ffd5b505af1158015610843573d5f5f3e3d5ffd5b50505050601b60019054906101000a90046001600160a01b03166001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610894575f5ffd5b505af11580156108a6573d5f5f3e3d5ffd5b505050505050565b60606012805480602002602001604051908101604052809291908181526020015f905b8282101561098f575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561097757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116109395790505b505050505081525050815260200190600101906108d1565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020015f905b8282101561098f578382905f5260205f200180546109d8906113aa565b80601f0160208091040260200160405190810160405280929190818152602001828054610a04906113aa565b8015610a4f5780601f10610a2657610100808354040283529160200191610a4f565b820191905f5260205f20905b815481529060010190602001808311610a3257829003601f168201915b5050505050815260200190600101906109bb565b60606013805480602002602001604051908101604052809291908181526020015f905b8282101561098f575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610b2c57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610aee5790505b50505050508152505081526020019060010190610a86565b60606010805480602002602001604051908101604052809291908181526020015f905b8282101561098f578382905f5260205f20018054610b84906113aa565b80601f0160208091040260200160405190810160405280929190818152602001828054610bb0906113aa565b8015610bfb5780601f10610bd257610100808354040283529160200191610bfb565b820191905f5260205f20905b815481529060010190602001808311610bde57829003601f168201915b505050505081526020019060010190610b67565b5f8054610100900460ff1615610c2d57505f54610100900460ff1690565b5f737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610d2d5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093525f929091610cb9917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016113f9565b60408051601f1981840301815290829052610cd39161141c565b5f604051808303815f865af19150503d805f8114610d0c576040519150601f19603f3d011682016040523d82523d5f602084013e610d11565b606091505b5091505080806020019051810190610d299190611427565b9150505b919050565b60408051818152601c818301527f596f7520617265206465706c6f79696e67206f6e20436861696e4944000000006060820152466020820181905291517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160018114610ddf5760405162461bcd60e51b815260206004820152601360248201527210da185a5b881b9bdd081cdd5c1c1bdc9d1959606a1b60448201526064016103a2565b601c5f9054906101000a90046001600160a01b03166001600160a01b031663fdc371ce6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e2f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e5391906112a1565b602180546001600160a01b0319166001600160a01b0392909216918217905573ec53bf9167f50cdeb3ae105f56099aaab9061f8314610ed45760405162461bcd60e51b815260206004820152601860248201527f736f6d657468696e6720686f727269626c792077726f6e67000000000000000060448201526064016103a2565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c6001600160a01b0316637fb5297f6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610f2e575f5ffd5b505af1158015610f40573d5f5f3e3d5ffd5b50506021546040516001600160a01b039091169250610f5f91506110f9565b6001600160a01b039091168152602001604051809103905ff080158015610f88573d5f5f3e3d5ffd5b50601d5f6101000a8154816001600160a01b0302191690836001600160a01b031602179055507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c6001600160a01b03166376eadd366040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611008575f5ffd5b505af115801561101a573d5f5f3e3d5ffd5b5050601d5460408051818152601581830152743122a4a3a2a72fb4b6b83632b6b2b73a30ba34b7b760591b60608201526001600160a01b039092166020830152517f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f9350908190036080019150a161109061059d565b611098610270565b50565b6060600c8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116104b9575050505050905090565b612d118061144783390190565b602080825282518282018190525f918401906040840190835b818110156111465783516001600160a01b031683526020938401939092019160010161111f565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156111fc57868503603f19018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101905f9060608801905b808310156111e45783516001600160e01b031916825260209384019360019390930192909101906111b8565b50965050506020938401939190910190600101611177565b50929695505050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156111fc57603f19878603018452611278858351611208565b9450602093840193919091019060010161125c565b6001600160a01b0381168114611098575f5ffd5b5f602082840312156112b1575f5ffd5b81516112bc8161128d565b9392505050565b5f602082840312156112d3575f5ffd5b5051919050565b6040815260046040820152636461746160e01b6060820152608060208201525f6112bc6080830184611208565b60018060a01b038716815285602082015260c060408201525f61132d60c0830187611208565b606083019590955250608081019290925260a0909101529392505050565b8082018082111561136a57634e487b7160e01b5f52601160045260245ffd5b92915050565b60018060a01b038616815284602082015260a060408201525f61139660a0830186611208565b606083019490945250608001529392505050565b600181811c908216806113be57607f821691505b6020821081036113dc57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f81518060208401855e5f93019283525090919050565b6001600160e01b0319831681525f61141460048301846113e2565b949350505050565b5f6112bc82846113e2565b5f60208284031215611437575f5ffd5b815180151581146112bc575f5ffdfe60a060405234801561000f575f5ffd5b50604051612d11380380612d1183398101604081905261002e91610105565b6001600160a01b038116608052610043610049565b50610132565b5f54610100900460ff16156100b45760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff90811614610103575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b5f60208284031215610115575f5ffd5b81516001600160a01b038116811461012b575f5ffd5b9392505050565b608051612bb261015f5f395f81816105e901528181610dae01528181610dd90152610e040152612bb25ff3fe608060405234801561000f575f5ffd5b506004361061023f575f3560e01c80637ecebe0011610135578063aa271e1a116100b4578063dd62ed3e11610079578063dd62ed3e14610579578063eb415f451461058c578063f1127ed814610594578063f2fde38b146105d1578063fdc371ce146105e4575f5ffd5b8063aa271e1a1461050a578063b8c255941461052d578063c3cda52014610540578063c4d66de814610553578063d505accf14610566575f5ffd5b806395d89b41116100fa57806395d89b41146104bf5780639ab24eb0146104c75780639aec4bae146104da578063a457c2d7146104e4578063a9059cbb146104f7575f5ffd5b80637ecebe001461044e57806384b0196e146104615780638da5cb5b1461047c5780638e539e8c1461048d57806391ddadf4146104a0575f5ffd5b806340c10f19116101c157806366eb399f1161018657806366eb399f146103c05780636fcfff45146103d357806370a08231146103fb578063715018a61461042357806378aa33ba1461042b575f5ffd5b806340c10f191461031a57806342966c681461032d5780634bf5d7e914610340578063587cde1e1461036a5780635c19a95c146103ad575f5ffd5b806323b872dd1161020757806323b872dd146102ca578063313ce567146102dd5780633644e515146102ec57806339509351146102f45780633a46b1a814610307575f5ffd5b80630455e6941461024357806306fdde031461027b578063095ea7b31461029057806318160ddd146102a35780631ffacdef146102b5575b5f5ffd5b610266610251366004612682565b6101316020525f908152604090205460ff1681565b60405190151581526020015b60405180910390f35b61028361060b565b60405161027291906126c9565b61026661029e3660046126db565b61069b565b6067545b604051908152602001610272565b6102c86102c3366004612703565b6106b4565b005b6102666102d836600461273c565b6106ca565b60405160128152602001610272565b6102a76106ed565b6102666103023660046126db565b6106fb565b6102a76103153660046126db565b61071c565b6102c86103283660046126db565b6107a4565b6102c861033b366004612776565b61081f565b60408051808201909152600e81526d06d6f64653d74696d657374616d760941b6020820152610283565b610395610378366004612682565b6001600160a01b039081165f90815260fe60205260409020541690565b6040516001600160a01b039091168152602001610272565b6102c86103bb366004612682565b61082c565b6102c86103ce366004612703565b610836565b6103e66103e1366004612682565b6108ae565b60405163ffffffff9091168152602001610272565b6102a7610409366004612682565b6001600160a01b03165f9081526065602052604090205490565b6102c86108cf565b610266610439366004612682565b6101326020525f908152604090205460ff1681565b6102a761045c366004612682565b6108e2565b6104696108ff565b604051610272979695949392919061278d565b6033546001600160a01b0316610395565b6102a761049b366004612776565b610998565b6104a86109ff565b60405165ffffffffffff9091168152602001610272565b610283610a09565b6102a76104d5366004612682565b610a18565b6102a76101305481565b6102666104f23660046126db565b610a95565b6102666105053660046126db565b610b0f565b610266610518366004612682565b6101336020525f908152604090205460ff1681565b6102c861053b366004612703565b610b1c565b6102c861054e366004612833565b610b2e565b6102c8610561366004612682565b610c63565b6102c8610574366004612887565b610ea5565b6102a76105873660046128ed565b611006565b6102c8611030565b6105a76105a236600461291e565b6110fe565b60408051825163ffffffff1681526020928301516001600160e01b03169281019290925201610272565b6102c86105df366004612682565b61117f565b6103957f000000000000000000000000000000000000000000000000000000000000000081565b60606068805461061a90612950565b80601f016020809104026020016040519081016040528092919081815260200182805461064690612950565b80156106915780601f1061066857610100808354040283529160200191610691565b820191905f5260205f20905b81548152906001019060200180831161067457829003601f168201915b5050505050905090565b5f336106a88185856111f5565b60019150505b92915050565b6106bc611318565b6106c68282611372565b5050565b5f336106d78582856113d2565b6106e285858561144a565b506001949350505050565b5f6106f6611604565b905090565b5f336106a881858561070d8383611006565b6107179190612996565b6111f5565b5f6107256109ff565b65ffffffffffff16821061077c5760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064015b60405180910390fd5b6001600160a01b0383165f90815260ff6020526040902061079d908361160d565b9392505050565b335f908152610133602052604090205460ff166108155760405162461bcd60e51b815260206004820152602960248201527f4261636b696e67456967656e2e6d696e743a2063616c6c6572206973206e6f7460448201526810309036b4b73a32b960b91b6064820152608401610773565b6106c682826116ee565b6108293382611779565b50565b6108293382611792565b61083e611318565b816001600160a01b03167f0124b12503bddc2616c0f3f54fd23ed283f5ef0c1483a75409e42612176b8bde8260405161087b911515815260200190565b60405180910390a26001600160a01b03919091165f90815261013360205260409020805460ff1916911515919091179055565b6001600160a01b0381165f90815260ff60205260408120546106ae9061180b565b6108d7611318565b6108e05f611873565b565b6001600160a01b0381165f90815260cb60205260408120546106ae565b5f6060805f5f5f60606097545f5f1b14801561091b5750609854155b61095f5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b6044820152606401610773565b6109676118c4565b61096f6118d3565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f6109a16109ff565b65ffffffffffff1682106109f35760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b6044820152606401610773565b6106ae6101008361160d565b5f6106f6426118e2565b60606069805461061a90612950565b6001600160a01b0381165f90815260ff60205260408120548015610a83576001600160a01b0383165f90815260ff6020526040902080545f198301908110610a6257610a626129bd565b5f9182526020909120015464010000000090046001600160e01b0316610a85565b5f5b6001600160e01b03169392505050565b5f3381610aa28286611006565b905083811015610b025760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401610773565b6106e282868684036111f5565b5f336106a881858561144a565b610b24611318565b6106c68282611948565b83421115610b7e5760405162461bcd60e51b815260206004820152601d60248201527f4552433230566f7465733a207369676e617475726520657870697265640000006044820152606401610773565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590525f90610bf790610bef9060a001604051602081830303815290604052805190602001206119a0565b8585856119cc565b9050610c02816119f2565b8614610c505760405162461bcd60e51b815260206004820152601960248201527f4552433230566f7465733a20696e76616c6964206e6f6e6365000000000000006044820152606401610773565b610c5a8188611792565b50505050505050565b5f54610100900460ff1615808015610c8157505f54600160ff909116105b80610c9a5750303b158015610c9a57505f5460ff166001145b610cfd5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610773565b5f805460ff191660011790558015610d1e575f805461ff0019166101001790555b610d26611a19565b610d736040518060400160405280600d81526020016c2130b1b5b4b7339022b4b3b2b760991b815250604051806040016040528060068152602001653122a4a3a2a760d11b815250611a47565b610d7c82611873565b610da3604051806040016040528060068152602001653122a4a3a2a760d11b815250611a77565b5f1961013055610dd47f00000000000000000000000000000000000000000000000000000000000000006001611372565b610dff7f00000000000000000000000000000000000000000000000000000000000000006001611948565b610e357f00000000000000000000000000000000000000000000000000000000000000006b05686877afb5cbccbf7340006116ee565b6040517fb7c23c1e2e36f298e9879a88ecfcd07e28fbb439bcfa9c78ca1363ca14370d26905f90a180156106c6575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b83421115610ef55760405162461bcd60e51b815260206004820152601d60248201527f45524332305065726d69743a206578706972656420646561646c696e650000006044820152606401610773565b5f7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9888888610f238c6119f2565b6040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810186905260e0016040516020818303038152906040528051906020012090505f610f7d826119a0565b90505f610f8c828787876119cc565b9050896001600160a01b0316816001600160a01b031614610fef5760405162461bcd60e51b815260206004820152601e60248201527f45524332305065726d69743a20696e76616c6964207369676e617475726500006044820152606401610773565b610ffa8a8a8a6111f5565b50505050505050505050565b6001600160a01b039182165f90815260666020908152604080832093909416825291909152205490565b611038611318565b5f1961013054146110ce5760405162461bcd60e51b815260206004820152605460248201527f4261636b696e67456967656e2e64697361626c655472616e736665725265737460448201527f72696374696f6e733a207472616e73666572207265737472696374696f6e7320606482015273185c9948185b1c9958591e48191a5cd8589b195960621b608482015260a401610773565b5f6101308190556040517f2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f649190a1565b604080518082019091525f80825260208201526001600160a01b0383165f90815260ff60205260409020805463ffffffff8416908110611140576111406129bd565b5f9182526020918290206040805180820190915291015463ffffffff8116825264010000000090046001600160e01b0316918101919091529392505050565b611187611318565b6001600160a01b0381166111ec5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610773565b61082981611873565b6001600160a01b0383166112575760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401610773565b6001600160a01b0382166112b85760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401610773565b6001600160a01b038381165f8181526066602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b6033546001600160a01b031633146108e05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610773565b6001600160a01b0382165f8181526101316020908152604091829020805460ff191685151590811790915591519182527fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227191015b60405180910390a25050565b5f6113dd8484611006565b90505f19811461144457818110156114375760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401610773565b61144484848484036111f5565b50505050565b6001600160a01b0383166114ae5760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401610773565b6001600160a01b0382166115105760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401610773565b61151b838383611ac0565b6001600160a01b0383165f90815260656020526040902054818110156115925760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401610773565b6001600160a01b038085165f8181526065602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef906115f19086815260200190565b60405180910390a3611444848484611b9c565b5f6106f6611bcd565b81545f9081816005811115611664575f61162684611c40565b61163090856129d1565b5f88815260209020909150869082015463ffffffff16111561165457809150611662565b61165f816001612996565b92505b505b808210156116af575f6116778383611d24565b5f88815260209020909150869082015463ffffffff16111561169b578091506116a9565b6116a6816001612996565b92505b50611664565b80156116d9575f8681526020902081015f19015464010000000090046001600160e01b03166116db565b5f5b6001600160e01b03169695505050505050565b6116f88282611d3e565b6067546001600160e01b03101561176a5760405162461bcd60e51b815260206004820152603060248201527f4552433230566f7465733a20746f74616c20737570706c79207269736b73206f60448201526f766572666c6f77696e6720766f74657360801b6064820152608401610773565b611444610100611e0f83611e1a565b6117838282611f86565b6114446101006120c983611e1a565b6001600160a01b038281165f81815260fe6020818152604080842080546065845282862054949093528787166001600160a01b03198416811790915590519190951694919391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a46114448284836120d4565b5f63ffffffff82111561186f5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610773565b5090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60606099805461061a90612950565b6060609a805461061a90612950565b5f65ffffffffffff82111561186f5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203460448201526538206269747360d01b6064820152608401610773565b6001600160a01b0382165f8181526101326020908152604091829020805460ff191685151590811790915591519182527f72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b91016113c6565b5f6106ae6119ac611604565b8360405161190160f01b8152600281019290925260228201526042902090565b5f5f5f6119db8787878761220e565b915091506119e8816122cb565b5095945050505050565b6001600160a01b0381165f90815260cb602052604090208054600181018255905b50919050565b5f54610100900460ff16611a3f5760405162461bcd60e51b8152600401610773906129e4565b6108e0612414565b5f54610100900460ff16611a6d5760405162461bcd60e51b8152600401610773906129e4565b6106c68282612443565b5f54610100900460ff16611a9d5760405162461bcd60e51b8152600401610773906129e4565b61082981604051806040016040528060018152602001603160f81b815250612482565b610130544211611b97576001600160a01b0383165f908152610131602052604090205460ff1680611b0957506001600160a01b0382165f908152610132602052604090205460ff165b80611b1b57506001600160a01b038316155b611b975760405162461bcd60e51b815260206004820152604160248201527f4261636b696e67456967656e2e5f6265666f7265546f6b656e5472616e73666560448201527f723a2066726f6d206f7220746f206d7573742062652077686974656c697374656064820152601960fa1b608482015260a401610773565b505050565b6001600160a01b038381165f90815260fe6020526040808220548584168352912054611b97929182169116836120d4565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611bf76124cf565b611bff612527565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f815f03611c4f57505f919050565b5f6001611c5b84612557565b901c6001901b90506001818481611c7457611c74612a2f565b048201901c90506001818481611c8c57611c8c612a2f565b048201901c90506001818481611ca457611ca4612a2f565b048201901c90506001818481611cbc57611cbc612a2f565b048201901c90506001818481611cd457611cd4612a2f565b048201901c90506001818481611cec57611cec612a2f565b048201901c90506001818481611d0457611d04612a2f565b048201901c905061079d81828581611d1e57611d1e612a2f565b046125ea565b5f611d326002848418612a43565b61079d90848416612996565b6001600160a01b038216611d945760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401610773565b611d9f5f8383611ac0565b8060675f828254611db09190612996565b90915550506001600160a01b0382165f818152606560209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a36106c65f8383611b9c565b5f61079d8284612996565b82545f908190818115611e64575f8781526020902082015f190160408051808201909152905463ffffffff8116825264010000000090046001600160e01b03166020820152611e78565b604080518082019091525f80825260208201525b905080602001516001600160e01b03169350611e9884868863ffffffff16565b92505f82118015611ec05750611eac6109ff565b65ffffffffffff16815f015163ffffffff16145b15611f0357611ece836125ff565b5f8881526020902083015f190180546001600160e01b03929092166401000000000263ffffffff909216919091179055611f7c565b866040518060400160405280611f27611f1a6109ff565b65ffffffffffff1661180b565b63ffffffff168152602001611f3b866125ff565b6001600160e01b0390811690915282546001810184555f938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b5050935093915050565b6001600160a01b038216611fe65760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401610773565b611ff1825f83611ac0565b6001600160a01b0382165f90815260656020526040902054818110156120645760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401610773565b6001600160a01b0383165f8181526065602090815260408083208686039055606780548790039055518581529192917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3611b97835f84611b9c565b5f61079d82846129d1565b816001600160a01b0316836001600160a01b0316141580156120f557505f81115b15611b97576001600160a01b03831615612182576001600160a01b0383165f90815260ff60205260408120819061212f906120c985611e1a565b91509150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7248383604051612177929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615611b97576001600160a01b0382165f90815260ff6020526040812081906121b790611e0f85611e1a565b91509150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516121ff929190918252602082015260400190565b60405180910390a25050505050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561224357505f905060036122c2565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015612294573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b0381166122bc575f600192509250506122c2565b91505f90505b94509492505050565b5f8160048111156122de576122de612a62565b036122e65750565b60018160048111156122fa576122fa612a62565b036123475760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610773565b600281600481111561235b5761235b612a62565b036123a85760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610773565b60038160048111156123bc576123bc612a62565b036108295760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610773565b5f54610100900460ff1661243a5760405162461bcd60e51b8152600401610773906129e4565b6108e033611873565b5f54610100900460ff166124695760405162461bcd60e51b8152600401610773906129e4565b60686124758382612ac1565b506069611b978282612ac1565b5f54610100900460ff166124a85760405162461bcd60e51b8152600401610773906129e4565b60996124b48382612ac1565b50609a6124c18282612ac1565b50505f609781905560985550565b5f5f6124d96118c4565b8051909150156124f0578051602090910120919050565b60975480156124ff5792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b5f5f6125316118d3565b805190915015612548578051602090910120919050565b60985480156124ff5792915050565b5f80608083901c1561256b57608092831c92015b604083901c1561257d57604092831c92015b602083901c1561258f57602092831c92015b601083901c156125a157601092831c92015b600883901c156125b357600892831c92015b600483901c156125c557600492831c92015b600283901c156125d757600292831c92015b600183901c156106ae5760010192915050565b5f8183106125f8578161079d565b5090919050565b5f6001600160e01b0382111561186f5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610773565b80356001600160a01b038116811461267d575f5ffd5b919050565b5f60208284031215612692575f5ffd5b61079d82612667565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61079d602083018461269b565b5f5f604083850312156126ec575f5ffd5b6126f583612667565b946020939093013593505050565b5f5f60408385031215612714575f5ffd5b61271d83612667565b915060208301358015158114612731575f5ffd5b809150509250929050565b5f5f5f6060848603121561274e575f5ffd5b61275784612667565b925061276560208501612667565b929592945050506040919091013590565b5f60208284031215612786575f5ffd5b5035919050565b60ff60f81b8816815260e060208201525f6127ab60e083018961269b565b82810360408401526127bd818961269b565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b818110156128125783518352602093840193909201916001016127f4565b50909b9a5050505050505050505050565b803560ff8116811461267d575f5ffd5b5f5f5f5f5f5f60c08789031215612848575f5ffd5b61285187612667565b9550602087013594506040870135935061286d60608801612823565b9598949750929560808101359460a0909101359350915050565b5f5f5f5f5f5f5f60e0888a03121561289d575f5ffd5b6128a688612667565b96506128b460208901612667565b955060408801359450606088013593506128d060808901612823565b9699959850939692959460a0840135945060c09093013592915050565b5f5f604083850312156128fe575f5ffd5b61290783612667565b915061291560208401612667565b90509250929050565b5f5f6040838503121561292f575f5ffd5b61293883612667565b9150602083013563ffffffff81168114612731575f5ffd5b600181811c9082168061296457607f821691505b602082108103611a1357634e487b7160e01b5f52602260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b808201808211156106ae576106ae612982565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b818103818111156106ae576106ae612982565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b5f52601260045260245ffd5b5f82612a5d57634e487b7160e01b5f52601260045260245ffd5b500490565b634e487b7160e01b5f52602160045260245ffd5b601f821115611b9757805f5260205f20601f840160051c81016020851015612a9b5750805b601f840160051c820191505b81811015612aba575f8155600101612aa7565b5050505050565b815167ffffffffffffffff811115612adb57612adb6129a9565b612aef81612ae98454612950565b84612a76565b6020601f821160018114612b21575f8315612b0a5750848201515b5f19600385901b1c1916600184901b178455612aba565b5f84815260208120601f198516915b82811015612b505787850151825560209485019460019092019101612b30565b5084821015612b6d57868401515f19600387901b60f8161c191681555b50505050600190811b0190555056fea2646970667358221220bc65aa22c82ea1986fef941c8ffca5d6f9991e3fb75b2eefff53e26501e45f9064736f6c634300081b0033a2646970667358221220a8adc4eee0589780040daa29e7e01f5d6e4b8e17eada37d0bfbdaf228290c29e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R_\x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U`\x1B\x80Ttq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x01`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16\x17\x90U`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16s\x83\xE9\x11]3M$\x8C\xE3\x9Ao6\x14J\xEA\xB5\xB3Enu\x17\x90\x91U`\x1E\x80T\x82\x16s?Z\xB2\xD4A\x8D8V\x87\x05\xBF\xD6g&0\xFC\xC3C\\\xC9\x17\x90U`\x1F\x80T\x82\x16s\xD6\xECA\xE4S\xC5\xE7\xDAT\x94\xF4\xD5\x1A\x05:\xB5qq.o\x17\x90U` \x80T\x90\x91\x16s\xBB\0\xDD\xA2\x83(P\xA48@\xA3\xA8e\x15\xE3\xFE\"he\xF2\x17\x90U4\x80\x15`\xD8W__\xFD[PaA\x8D\x80a\0\xE6_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01'W_5`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xA9W\x80c\xC3\xEA?\xC9\x11a\0nW\x80c\xC3\xEA?\xC9\x14a\x02)W\x80c\xD7\x15\xBCn\x14a\x02<W\x80c\xE2\x0C\x9Fq\x14a\x02OW\x80c\xF8\xCC\xBFG\x14a\x02WW\x80c\xFAv&\xD4\x14a\x02dW__\xFD[\x80c\x91j\x17\xC6\x14a\x01\xE6W\x80c\xA5a|\xFD\x14a\x01\xEEW\x80c\xB5P\x8A\xA9\x14a\x02\x01W\x80c\xBAAO\xA6\x14a\x02\tW\x80c\xC0@b&\x14a\x02!W__\xFD[\x80c?r\x86\xF4\x11a\0\xEFW\x80c?r\x86\xF4\x14a\x01\x99W\x80cTu[\x99\x14a\x01\xA1W\x80cf\xD9\xA9\xA0\x14a\x01\xA9W\x80c\x81\x1F\xFE\xEB\x14a\x01\xBEW\x80c\x85\"l\x81\x14a\x01\xD1W__\xFD[\x80c\x14\xF8\xFF\xAC\x14a\x01+W\x80c\x1E\xD7\x83\x1C\x14a\x015W\x80c5\xE4\xA06\x14a\x01SW\x80c7\x0C0\xD9\x14a\x01~W\x80c>^<#\x14a\x01\x91W[__\xFD[a\x013a\x02pV[\0[a\x01=a\x04\x81V[`@Qa\x01J\x91\x90a\x11\x06V[`@Q\x80\x91\x03\x90\xF3[` Ta\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01JV[`!Ta\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01=a\x04\xE1V[a\x01=a\x05?V[a\x013a\x05\x9DV[a\x01\xB1a\x08\xAEV[`@Qa\x01J\x91\x90a\x11QV[`\x1FTa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD9a\t\x98V[`@Qa\x01J\x91\x90a\x126V[a\x01\xB1a\ncV[`\x1CTa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD9a\x0BDV[a\x02\x11a\x0C\x0FV[`@Q\x90\x15\x15\x81R` \x01a\x01JV[a\x013a\r2V[`\x1DTa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1ETa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01=a\x10\x9BV[`\x1BTa\x02\x11\x90`\xFF\x16\x81V[_Ta\x02\x11\x90`\xFF\x16\x81V[`\x1FT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC6W__\xFD[PZ\xF1\x15\x80\x15a\x02\xD8W=__>=_\xFD[PP`\x1DT`\x1ET`\x1CT`@Qc\x10'\x0E=`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x81\x16\x94P\x16\x91Pc N\x1Cz\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03P\x91\x90a\x12\xA1V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fimplementation set incorrectly\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`!T`\x1CT`@\x80Qc~\xE1\xB8\xE7`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x90\x92\x16\x91c\xFD\xC3q\xCE\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x03\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1E\x91\x90a\x12\xA1V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEIGEN address changed unexpected`D\x82\x01Raly`\xF0\x1B`d\x82\x01R`\x84\x01a\x03\xA2V[V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9WPPPPP\x90P\x90V[`\x1FT`@\x80Qcy=\x06I`\xE1\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2z\x0C\x92\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x08\x91\x90a\x12\xC3V[`\x1CT`\x1DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x92\x90\x91\x16`D\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c&j#\xB1`\xE2\x1B\x17\x90RQ\x91\x92P\x90\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x90a\x06\x8C\x90\x83\x90a\x12\xDAV[`@Q\x80\x91\x03\x90\xA1` T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xEAW__\xFD[PZ\xF1\x15\x80\x15a\x06\xFCW=__>=_\xFD[PP`\x1FT`\x1ET`@Qb\xEA\x83\x15`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x01\xD5\x06*\x93Pa\x07>\x92\x90\x91\x16\x90_\x90\x86\x90\x82\x90\x81\x90\x8A\x90`\x04\x01a\x13\x07V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07UW__\xFD[PZ\xF1\x15\x80\x15a\x07gW=__>=_\xFD[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92Pc\xE5\xD6\xBF\x02\x91Pa\x07\x92\x90P\x84Ba\x13KV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xB0\x91\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xC7W__\xFD[PZ\xF1\x15\x80\x15a\x07\xD9W=__>=_\xFD[PP`\x1FT`\x1ET`@Qc\x13@\x08\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x13@\x08\xD3\x93Pa\x08\x1A\x92\x90\x91\x16\x90_\x90\x86\x90\x82\x90\x81\x90`\x04\x01a\x13pV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x081W__\xFD[PZ\xF1\x15\x80\x15a\x08CW=__>=_\xFD[PPPP`\x1B`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x94W__\xFD[PZ\xF1\x15\x80\x15a\x08\xA6W=__>=_\xFD[PPPPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\twW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x08\xD1V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW\x83\x82\x90_R` _ \x01\x80Ta\t\xD8\x90a\x13\xAAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x04\x90a\x13\xAAV[\x80\x15a\nOW\x80`\x1F\x10a\n&Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nOV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t\xBBV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0B,W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\n\xEEW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\x86V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW\x83\x82\x90_R` _ \x01\x80Ta\x0B\x84\x90a\x13\xAAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB0\x90a\x13\xAAV[\x80\x15a\x0B\xFBW\x80`\x1F\x10a\x0B\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xDEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0BgV[_\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0C-WP_Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r-W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x0C\xB9\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x13\xF9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\xD3\x91a\x14\x1CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\r\x0CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\r\x11V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\r)\x91\x90a\x14'V[\x91PP[\x91\x90PV[`@\x80Q\x81\x81R`\x1C\x81\x83\x01R\x7FYou are deploying on ChainID\0\0\0\0``\x82\x01RF` \x82\x01\x81\x90R\x91Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`\x01\x81\x14a\r\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10\xDA\x18Z[\x88\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`D\x82\x01R`d\x01a\x03\xA2V[`\x1C_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xC3q\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ES\x91\x90a\x12\xA1V[`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Us\xECS\xBF\x91g\xF5\x0C\xDE\xB3\xAE\x10_V\t\x9A\xAA\xB9\x06\x1F\x83\x14a\x0E\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fsomething horribly wrong\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xA2V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F.W__\xFD[PZ\xF1\x15\x80\x15a\x0F@W=__>=_\xFD[PP`!T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pa\x0F_\x91Pa\x10\xF9V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x0F\x88W=__>=_\xFD[P`\x1D_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x08W__\xFD[PZ\xF1\x15\x80\x15a\x10\x1AW=__>=_\xFD[PP`\x1DT`@\x80Q\x81\x81R`\x15\x81\x83\x01Rt1\"\xA4\xA3\xA2\xA7/\xB4\xB6\xB862\xB6\xB2\xB7:0\xBA4\xB7\xB7`Y\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x93P\x90\x81\x90\x03`\x80\x01\x91P\xA1a\x10\x90a\x05\x9DV[a\x10\x98a\x02pV[PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9WPPPPP\x90P\x90V[a-\x11\x80a\x14G\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x11FW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x11\x1FV[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x11\xFCW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15a\x11\xE4W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a\x11\xB8V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x11wV[P\x92\x96\x95PPPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x11\xFCW`?\x19\x87\x86\x03\x01\x84Ra\x12x\x85\x83Qa\x12\x08V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x12\\V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x98W__\xFD[_` \x82\x84\x03\x12\x15a\x12\xB1W__\xFD[\x81Qa\x12\xBC\x81a\x12\x8DV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x12\xD3W__\xFD[PQ\x91\x90PV[`@\x81R`\x04`@\x82\x01Rcdata`\xE0\x1B``\x82\x01R`\x80` \x82\x01R_a\x12\xBC`\x80\x83\x01\x84a\x12\x08V[`\x01\x80`\xA0\x1B\x03\x87\x16\x81R\x85` \x82\x01R`\xC0`@\x82\x01R_a\x13-`\xC0\x83\x01\x87a\x12\x08V[``\x83\x01\x95\x90\x95RP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x13jWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R_a\x13\x96`\xA0\x83\x01\x86a\x12\x08V[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xBEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\xDCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R_a\x14\x14`\x04\x83\x01\x84a\x13\xE2V[\x94\x93PPPPV[_a\x12\xBC\x82\x84a\x13\xE2V[_` \x82\x84\x03\x12\x15a\x147W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12\xBCW__\xFD\xFE`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa-\x118\x03\x80a-\x11\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Ra\0Ca\0IV[Pa\x012V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x01\x03W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[_` \x82\x84\x03\x12\x15a\x01\x15W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01+W__\xFD[\x93\x92PPPV[`\x80Qa+\xB2a\x01__9_\x81\x81a\x05\xE9\x01R\x81\x81a\r\xAE\x01R\x81\x81a\r\xD9\x01Ra\x0E\x04\x01Ra+\xB2_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02?W_5`\xE0\x1C\x80c~\xCE\xBE\0\x11a\x015W\x80c\xAA'\x1E\x1A\x11a\0\xB4W\x80c\xDDb\xED>\x11a\0yW\x80c\xDDb\xED>\x14a\x05yW\x80c\xEBA_E\x14a\x05\x8CW\x80c\xF1\x12~\xD8\x14a\x05\x94W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xD1W\x80c\xFD\xC3q\xCE\x14a\x05\xE4W__\xFD[\x80c\xAA'\x1E\x1A\x14a\x05\nW\x80c\xB8\xC2U\x94\x14a\x05-W\x80c\xC3\xCD\xA5 \x14a\x05@W\x80c\xC4\xD6m\xE8\x14a\x05SW\x80c\xD5\x05\xAC\xCF\x14a\x05fW__\xFD[\x80c\x95\xD8\x9BA\x11a\0\xFAW\x80c\x95\xD8\x9BA\x14a\x04\xBFW\x80c\x9A\xB2N\xB0\x14a\x04\xC7W\x80c\x9A\xECK\xAE\x14a\x04\xDAW\x80c\xA4W\xC2\xD7\x14a\x04\xE4W\x80c\xA9\x05\x9C\xBB\x14a\x04\xF7W__\xFD[\x80c~\xCE\xBE\0\x14a\x04NW\x80c\x84\xB0\x19n\x14a\x04aW\x80c\x8D\xA5\xCB[\x14a\x04|W\x80c\x8ES\x9E\x8C\x14a\x04\x8DW\x80c\x91\xDD\xAD\xF4\x14a\x04\xA0W__\xFD[\x80c@\xC1\x0F\x19\x11a\x01\xC1W\x80cf\xEB9\x9F\x11a\x01\x86W\x80cf\xEB9\x9F\x14a\x03\xC0W\x80co\xCF\xFFE\x14a\x03\xD3W\x80cp\xA0\x821\x14a\x03\xFBW\x80cqP\x18\xA6\x14a\x04#W\x80cx\xAA3\xBA\x14a\x04+W__\xFD[\x80c@\xC1\x0F\x19\x14a\x03\x1AW\x80cB\x96lh\x14a\x03-W\x80cK\xF5\xD7\xE9\x14a\x03@W\x80cX|\xDE\x1E\x14a\x03jW\x80c\\\x19\xA9\\\x14a\x03\xADW__\xFD[\x80c#\xB8r\xDD\x11a\x02\x07W\x80c#\xB8r\xDD\x14a\x02\xCAW\x80c1<\xE5g\x14a\x02\xDDW\x80c6D\xE5\x15\x14a\x02\xECW\x80c9P\x93Q\x14a\x02\xF4W\x80c:F\xB1\xA8\x14a\x03\x07W__\xFD[\x80c\x04U\xE6\x94\x14a\x02CW\x80c\x06\xFD\xDE\x03\x14a\x02{W\x80c\t^\xA7\xB3\x14a\x02\x90W\x80c\x18\x16\r\xDD\x14a\x02\xA3W\x80c\x1F\xFA\xCD\xEF\x14a\x02\xB5W[__\xFD[a\x02fa\x02Q6`\x04a&\x82V[a\x011` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x83a\x06\x0BV[`@Qa\x02r\x91\x90a&\xC9V[a\x02fa\x02\x9E6`\x04a&\xDBV[a\x06\x9BV[`gT[`@Q\x90\x81R` \x01a\x02rV[a\x02\xC8a\x02\xC36`\x04a'\x03V[a\x06\xB4V[\0[a\x02fa\x02\xD86`\x04a'<V[a\x06\xCAV[`@Q`\x12\x81R` \x01a\x02rV[a\x02\xA7a\x06\xEDV[a\x02fa\x03\x026`\x04a&\xDBV[a\x06\xFBV[a\x02\xA7a\x03\x156`\x04a&\xDBV[a\x07\x1CV[a\x02\xC8a\x03(6`\x04a&\xDBV[a\x07\xA4V[a\x02\xC8a\x03;6`\x04a'vV[a\x08\x1FV[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\x83V[a\x03\x95a\x03x6`\x04a&\x82V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\xFE` R`@\x90 T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02rV[a\x02\xC8a\x03\xBB6`\x04a&\x82V[a\x08,V[a\x02\xC8a\x03\xCE6`\x04a'\x03V[a\x086V[a\x03\xE6a\x03\xE16`\x04a&\x82V[a\x08\xAEV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02rV[a\x02\xA7a\x04\t6`\x04a&\x82V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`e` R`@\x90 T\x90V[a\x02\xC8a\x08\xCFV[a\x02fa\x0496`\x04a&\x82V[a\x012` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA7a\x04\\6`\x04a&\x82V[a\x08\xE2V[a\x04ia\x08\xFFV[`@Qa\x02r\x97\x96\x95\x94\x93\x92\x91\x90a'\x8DV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x95V[a\x02\xA7a\x04\x9B6`\x04a'vV[a\t\x98V[a\x04\xA8a\t\xFFV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02rV[a\x02\x83a\n\tV[a\x02\xA7a\x04\xD56`\x04a&\x82V[a\n\x18V[a\x02\xA7a\x010T\x81V[a\x02fa\x04\xF26`\x04a&\xDBV[a\n\x95V[a\x02fa\x05\x056`\x04a&\xDBV[a\x0B\x0FV[a\x02fa\x05\x186`\x04a&\x82V[a\x013` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xC8a\x05;6`\x04a'\x03V[a\x0B\x1CV[a\x02\xC8a\x05N6`\x04a(3V[a\x0B.V[a\x02\xC8a\x05a6`\x04a&\x82V[a\x0CcV[a\x02\xC8a\x05t6`\x04a(\x87V[a\x0E\xA5V[a\x02\xA7a\x05\x876`\x04a(\xEDV[a\x10\x06V[a\x02\xC8a\x100V[a\x05\xA7a\x05\xA26`\x04a)\x1EV[a\x10\xFEV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02rV[a\x02\xC8a\x05\xDF6`\x04a&\x82V[a\x11\x7FV[a\x03\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[```h\x80Ta\x06\x1A\x90a)PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06F\x90a)PV[\x80\x15a\x06\x91W\x80`\x1F\x10a\x06hWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x91V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_3a\x06\xA8\x81\x85\x85a\x11\xF5V[`\x01\x91PP[\x92\x91PPV[a\x06\xBCa\x13\x18V[a\x06\xC6\x82\x82a\x13rV[PPV[_3a\x06\xD7\x85\x82\x85a\x13\xD2V[a\x06\xE2\x85\x85\x85a\x14JV[P`\x01\x94\x93PPPPV[_a\x06\xF6a\x16\x04V[\x90P\x90V[_3a\x06\xA8\x81\x85\x85a\x07\r\x83\x83a\x10\x06V[a\x07\x17\x91\x90a)\x96V[a\x11\xF5V[_a\x07%a\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x90 a\x07\x9D\x90\x83a\x16\rV[\x93\x92PPPV[3_\x90\x81Ra\x013` R`@\x90 T`\xFF\x16a\x08\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBackingEigen.mint: caller is not`D\x82\x01Rh\x100\x906\xB4\xB7:2\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x06\xC6\x82\x82a\x16\xEEV[a\x08)3\x82a\x17yV[PV[a\x08)3\x82a\x17\x92V[a\x08>a\x13\x18V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x01$\xB1%\x03\xBD\xDC&\x16\xC0\xF3\xF5O\xD2>\xD2\x83\xF5\xEF\x0C\x14\x83\xA7T\t\xE4&\x12\x17k\x8B\xDE\x82`@Qa\x08{\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16_\x90\x81Ra\x013` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xFF` R`@\x81 Ta\x06\xAE\x90a\x18\x0BV[a\x08\xD7a\x13\x18V[a\x08\xE0_a\x18sV[V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xCB` R`@\x81 Ta\x06\xAEV[_``\x80___```\x97T__\x1B\x14\x80\x15a\t\x1BWP`\x98T\x15[a\t_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07sV[a\tga\x18\xC4V[a\toa\x18\xD3V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_a\t\xA1a\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07sV[a\x06\xAEa\x01\0\x83a\x16\rV[_a\x06\xF6Ba\x18\xE2V[```i\x80Ta\x06\x1A\x90a)PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xFF` R`@\x81 T\x80\x15a\n\x83W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x90 \x80T_\x19\x83\x01\x90\x81\x10a\nbWa\nba)\xBDV[_\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\n\x85V[_[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[_3\x81a\n\xA2\x82\x86a\x10\x06V[\x90P\x83\x81\x10\x15a\x0B\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x06\xE2\x82\x86\x86\x84\x03a\x11\xF5V[_3a\x06\xA8\x81\x85\x85a\x14JV[a\x0B$a\x13\x18V[a\x06\xC6\x82\x82a\x19HV[\x83B\x11\x15a\x0B~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Votes: signature expired\0\0\0`D\x82\x01R`d\x01a\x07sV[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R_\x90a\x0B\xF7\x90a\x0B\xEF\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19\xA0V[\x85\x85\x85a\x19\xCCV[\x90Pa\x0C\x02\x81a\x19\xF2V[\x86\x14a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC20Votes: invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07sV[a\x0CZ\x81\x88a\x17\x92V[PPPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0C\x81WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0C\x9AWP0;\x15\x80\x15a\x0C\x9AWP_T`\xFF\x16`\x01\x14[a\x0C\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07sV[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\r\x1EW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\r&a\x1A\x19V[a\rs`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l!0\xB1\xB5\xB4\xB73\x90\"\xB4\xB3\xB2\xB7`\x99\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1AGV[a\r|\x82a\x18sV[a\r\xA3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1AwV[_\x19a\x010Ua\r\xD4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x13rV[a\r\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x19HV[a\x0E5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0k\x05hhw\xAF\xB5\xCB\xCC\xBFs@\0a\x16\xEEV[`@Q\x7F\xB7\xC2<\x1E.6\xF2\x98\xE9\x87\x9A\x88\xEC\xFC\xD0~(\xFB\xB49\xBC\xFA\x9Cx\xCA\x13c\xCA\x147\r&\x90_\x90\xA1\x80\x15a\x06\xC6W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[\x83B\x11\x15a\x0E\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x07sV[_\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x0F#\x8Ca\x19\xF2V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x0F}\x82a\x19\xA0V[\x90P_a\x0F\x8C\x82\x87\x87\x87a\x19\xCCV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x07sV[a\x0F\xFA\x8A\x8A\x8Aa\x11\xF5V[PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`f` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x108a\x13\x18V[_\x19a\x010T\x14a\x10\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FBackingEigen.disableTransferRest`D\x82\x01R\x7Frictions: transfer restrictions `d\x82\x01Rs\x18\\\x99H\x18[\x1C\x99XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`b\x1B`\x84\x82\x01R`\xA4\x01a\x07sV[_a\x010\x81\x90U`@Q\x7F+\x18\x98m;\xA8\t\xDB/\x13\xA5\xD7\xBF\x17\xF6\r5{7\xD9\xCB\xB5]\xD7\x1C\xBB\xAC\x8D\xC4\x06\x0Fd\x91\x90\xA1V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x11@Wa\x11@a)\xBDV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x11\x87a\x13\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x08)\x81a\x18sV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`f` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81Ra\x011` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\x13\xDD\x84\x84a\x10\x06V[\x90P_\x19\x81\x14a\x14DW\x81\x81\x10\x15a\x147W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07sV[a\x14D\x84\x84\x84\x84\x03a\x11\xF5V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x14\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x15\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x15\x1B\x83\x83\x83a\x1A\xC0V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a\x15\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x81\x81R`e` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x15\xF1\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x14D\x84\x84\x84a\x1B\x9CV[_a\x06\xF6a\x1B\xCDV[\x81T_\x90\x81\x81`\x05\x81\x11\x15a\x16dW_a\x16&\x84a\x1C@V[a\x160\x90\x85a)\xD1V[_\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16TW\x80\x91Pa\x16bV[a\x16_\x81`\x01a)\x96V[\x92P[P[\x80\x82\x10\x15a\x16\xAFW_a\x16w\x83\x83a\x1D$V[_\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\x9BW\x80\x91Pa\x16\xA9V[a\x16\xA6\x81`\x01a)\x96V[\x92P[Pa\x16dV[\x80\x15a\x16\xD9W_\x86\x81R` \x90 \x81\x01_\x19\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x16\xDBV[_[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[a\x16\xF8\x82\x82a\x1D>V[`gT`\x01`\x01`\xE0\x1B\x03\x10\x15a\x17jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC20Votes: total supply risks o`D\x82\x01Roverflowing votes`\x80\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x14Da\x01\0a\x1E\x0F\x83a\x1E\x1AV[a\x17\x83\x82\x82a\x1F\x86V[a\x14Da\x01\0a \xC9\x83a\x1E\x1AV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xFE` \x81\x81R`@\x80\x84 \x80T`e\x84R\x82\x86 T\x94\x90\x93R\x87\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x84\x16\x81\x17\x90\x91U\x90Q\x91\x90\x95\x16\x94\x91\x93\x91\x92\x85\x92\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x14D\x82\x84\x83a \xD4V[_c\xFF\xFF\xFF\xFF\x82\x11\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[P\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[```\x99\x80Ta\x06\x1A\x90a)PV[```\x9A\x80Ta\x06\x1A\x90a)PV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81Ra\x012` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7Fr\xA5a\xD1\xAFt\tF}\xAEO\x1E\x9F\xC5%\x90\xA93Z\x1D\xDA\x17r~+j\xA8\xC4\xDB5\x10\x9B\x91\x01a\x13\xC6V[_a\x06\xAEa\x19\xACa\x16\x04V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[___a\x19\xDB\x87\x87\x87\x87a\"\x0EV[\x91P\x91Pa\x19\xE8\x81a\"\xCBV[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xCB` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[_Ta\x01\0\x90\x04`\xFF\x16a\x1A?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x08\xE0a$\x14V[_Ta\x01\0\x90\x04`\xFF\x16a\x1AmW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x06\xC6\x82\x82a$CV[_Ta\x01\0\x90\x04`\xFF\x16a\x1A\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x08)\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RPa$\x82V[a\x010TB\x11a\x1B\x97W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81Ra\x011` R`@\x90 T`\xFF\x16\x80a\x1B\tWP`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81Ra\x012` R`@\x90 T`\xFF\x16[\x80a\x1B\x1BWP`\x01`\x01`\xA0\x1B\x03\x83\x16\x15[a\x1B\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FBackingEigen._beforeTokenTransfe`D\x82\x01R\x7Fr: from or to must be whiteliste`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x07sV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xFE` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x1B\x97\x92\x91\x82\x16\x91\x16\x83a \xD4V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1B\xF7a$\xCFV[a\x1B\xFFa%'V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81_\x03a\x1COWP_\x91\x90PV[_`\x01a\x1C[\x84a%WV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x1CtWa\x1Cta*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\x8CWa\x1C\x8Ca*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xA4Wa\x1C\xA4a*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xBCWa\x1C\xBCa*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xD4Wa\x1C\xD4a*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xECWa\x1C\xECa*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D\x04Wa\x1D\x04a*/V[\x04\x82\x01\x90\x1C\x90Pa\x07\x9D\x81\x82\x85\x81a\x1D\x1EWa\x1D\x1Ea*/V[\x04a%\xEAV[_a\x1D2`\x02\x84\x84\x18a*CV[a\x07\x9D\x90\x84\x84\x16a)\x96V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07sV[a\x1D\x9F_\x83\x83a\x1A\xC0V[\x80`g_\x82\x82Ta\x1D\xB0\x91\x90a)\x96V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`e` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x06\xC6_\x83\x83a\x1B\x9CV[_a\x07\x9D\x82\x84a)\x96V[\x82T_\x90\x81\x90\x81\x81\x15a\x1EdW_\x87\x81R` \x90 \x82\x01_\x19\x01`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16` \x82\x01Ra\x1ExV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R[\x90P\x80` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93Pa\x1E\x98\x84\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x92P_\x82\x11\x80\x15a\x1E\xC0WPa\x1E\xACa\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x1F\x03Wa\x1E\xCE\x83a%\xFFV[_\x88\x81R` \x90 \x83\x01_\x19\x01\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1F|V[\x86`@Q\x80`@\x01`@R\x80a\x1F'a\x1F\x1Aa\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x0BV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x1F;\x86a%\xFFV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U_\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[PP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x1F\xF1\x82_\x83a\x1A\xC0V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x81\x81R`e` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`g\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x1B\x97\x83_\x84a\x1B\x9CV[_a\x07\x9D\x82\x84a)\xD1V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a \xF5WP_\x81\x11[\x15a\x1B\x97W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a!\x82W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x81 \x81\x90a!/\x90a \xC9\x85a\x1E\x1AV[\x91P\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!w\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x1B\x97W`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\xFF` R`@\x81 \x81\x90a!\xB7\x90a\x1E\x0F\x85a\x1E\x1AV[\x91P\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"CWP_\x90P`\x03a\"\xC2V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\"\x94W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\"\xBCW_`\x01\x92P\x92PPa\"\xC2V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x81`\x04\x81\x11\x15a\"\xDEWa\"\xDEa*bV[\x03a\"\xE6WPV[`\x01\x81`\x04\x81\x11\x15a\"\xFAWa\"\xFAa*bV[\x03a#GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07sV[`\x02\x81`\x04\x81\x11\x15a#[Wa#[a*bV[\x03a#\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x07sV[`\x03\x81`\x04\x81\x11\x15a#\xBCWa#\xBCa*bV[\x03a\x08)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x07sV[_Ta\x01\0\x90\x04`\xFF\x16a$:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x08\xE03a\x18sV[_Ta\x01\0\x90\x04`\xFF\x16a$iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[`ha$u\x83\x82a*\xC1V[P`ia\x1B\x97\x82\x82a*\xC1V[_Ta\x01\0\x90\x04`\xFF\x16a$\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[`\x99a$\xB4\x83\x82a*\xC1V[P`\x9Aa$\xC1\x82\x82a*\xC1V[PP_`\x97\x81\x90U`\x98UPV[__a$\xD9a\x18\xC4V[\x80Q\x90\x91P\x15a$\xF0W\x80Q` \x90\x91\x01 \x91\x90PV[`\x97T\x80\x15a$\xFFW\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[__a%1a\x18\xD3V[\x80Q\x90\x91P\x15a%HW\x80Q` \x90\x91\x01 \x91\x90PV[`\x98T\x80\x15a$\xFFW\x92\x91PPV[_\x80`\x80\x83\x90\x1C\x15a%kW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a%}W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a%\x8FW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a%\xA1W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a%\xB3W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a%\xC5W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a%\xD7W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x06\xAEW`\x01\x01\x92\x91PPV[_\x81\x83\x10a%\xF8W\x81a\x07\x9DV[P\x90\x91\x90PV[_`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07sV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a&}W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a&\x92W__\xFD[a\x07\x9D\x82a&gV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\x9D` \x83\x01\x84a&\x9BV[__`@\x83\x85\x03\x12\x15a&\xECW__\xFD[a&\xF5\x83a&gV[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a'\x14W__\xFD[a'\x1D\x83a&gV[\x91P` \x83\x015\x80\x15\x15\x81\x14a'1W__\xFD[\x80\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a'NW__\xFD[a'W\x84a&gV[\x92Pa'e` \x85\x01a&gV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a'\x86W__\xFD[P5\x91\x90PV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a'\xAB`\xE0\x83\x01\x89a&\x9BV[\x82\x81\x03`@\x84\x01Ra'\xBD\x81\x89a&\x9BV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a(\x12W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xF4V[P\x90\x9B\x9APPPPPPPPPPPV[\x805`\xFF\x81\x16\x81\x14a&}W__\xFD[______`\xC0\x87\x89\x03\x12\x15a(HW__\xFD[a(Q\x87a&gV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa(m``\x88\x01a(#V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_______`\xE0\x88\x8A\x03\x12\x15a(\x9DW__\xFD[a(\xA6\x88a&gV[\x96Pa(\xB4` \x89\x01a&gV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa(\xD0`\x80\x89\x01a(#V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a(\xFEW__\xFD[a)\x07\x83a&gV[\x91Pa)\x15` \x84\x01a&gV[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a)/W__\xFD[a)8\x83a&gV[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'1W__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a)dW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1A\x13WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xAEWa\x06\xAEa)\x82V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xAEWa\x06\xAEa)\x82V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a*]WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x1B\x97W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a*\x9BWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a*\xBAW_\x81U`\x01\x01a*\xA7V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xDBWa*\xDBa)\xA9V[a*\xEF\x81a*\xE9\x84Ta)PV[\x84a*vV[` `\x1F\x82\x11`\x01\x81\x14a+!W_\x83\x15a+\nWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua*\xBAV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a+PW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a+0V[P\x84\x82\x10\x15a+mW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xBCe\xAA\"\xC8.\xA1\x98o\xEF\x94\x1C\x8F\xFC\xA5\xD6\xF9\x99\x1E?\xB7[.\xEF\xFFS\xE2e\x01\xE4_\x90dsolcC\0\x08\x1B\x003\xA2dipfsX\"\x12 \xA8\xAD\xC4\xEE\xE0X\x97\x80\x04\r\xAA)\xE7\xE0\x1F]nK\x8E\x17\xEA\xDA7\xD0\xBF\xBD\xAF\"\x82\x90\xC2\x9EdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610127575f3560e01c8063916a17c6116100a9578063c3ea3fc91161006e578063c3ea3fc914610229578063d715bc6e1461023c578063e20c9f711461024f578063f8ccbf4714610257578063fa7626d414610264575f5ffd5b8063916a17c6146101e6578063a5617cfd146101ee578063b5508aa914610201578063ba414fa614610209578063c040622614610221575f5ffd5b80633f7286f4116100ef5780633f7286f41461019957806354755b99146101a157806366d9a9a0146101a9578063811ffeeb146101be57806385226c81146101d1575f5ffd5b806314f8ffac1461012b5780631ed7831c1461013557806335e4a03614610153578063370c30d91461017e5780633e5e3c2314610191575b5f5ffd5b610133610270565b005b61013d610481565b60405161014a9190611106565b60405180910390f35b602054610166906001600160a01b031681565b6040516001600160a01b03909116815260200161014a565b602154610166906001600160a01b031681565b61013d6104e1565b61013d61053f565b61013361059d565b6101b16108ae565b60405161014a9190611151565b601f54610166906001600160a01b031681565b6101d9610998565b60405161014a9190611236565b6101b1610a63565b601c54610166906001600160a01b031681565b6101d9610b44565b610211610c0f565b604051901515815260200161014a565b610133610d32565b601d54610166906001600160a01b031681565b601e54610166906001600160a01b031681565b61013d61109b565b601b546102119060ff1681565b5f546102119060ff1681565b601f5460405163ca669fa760e01b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ca669fa7906024015f604051808303815f87803b1580156102c6575f5ffd5b505af11580156102d8573d5f5f3e3d5ffd5b5050601d54601e54601c546040516310270e3d60e11b81526001600160a01b039182166004820152928116945016915063204e1c7a90602401602060405180830381865afa15801561032c573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061035091906112a1565b6001600160a01b0316146103ab5760405162461bcd60e51b815260206004820152601e60248201527f696d706c656d656e746174696f6e2073657420696e636f72726563746c79000060448201526064015b60405180910390fd5b602154601c5460408051637ee1b8e760e11b815290516001600160a01b03938416939092169163fdc371ce916004808201926020929091908290030181865afa1580156103fa573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061041e91906112a1565b6001600160a01b03161461047f5760405162461bcd60e51b815260206004820152602260248201527f454947454e2061646472657373206368616e67656420756e65787065637465646044820152616c7960f01b60648201526084016103a2565b565b6060600d8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116104b9575b5050505050905090565b6060600f8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116104b9575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116104b9575050505050905090565b601f546040805163793d064960e11b815290515f926001600160a01b03169163f27a0c929160048083019260209291908290030181865afa1580156105e4573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061060891906112c3565b601c54601d54604080516001600160a01b0393841660248201529290911660448084019190915281518084039091018152606490920181526020820180516001600160e01b031663266a23b160e21b17905251919250907fd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf189061068c9083906112da565b60405180910390a16020546040516303223eab60e11b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b1580156106ea575f5ffd5b505af11580156106fc573d5f5f3e3d5ffd5b5050601f54601e5460405162ea831560e11b81526001600160a01b0392831694506301d5062a935061073e92909116905f908690829081908a90600401611307565b5f604051808303815f87803b158015610755575f5ffd5b505af1158015610767573d5f5f3e3d5ffd5b50737109709ecfa91a80626ff3989d68f67f5b1dd12d925063e5d6bf0291506107929050844261134b565b6040518263ffffffff1660e01b81526004016107b091815260200190565b5f604051808303815f87803b1580156107c7575f5ffd5b505af11580156107d9573d5f5f3e3d5ffd5b5050601f54601e5460405163134008d360e01b81526001600160a01b03928316945063134008d3935061081a92909116905f90869082908190600401611370565b5f604051808303815f87803b158015610831575f5ffd5b505af1158015610843573d5f5f3e3d5ffd5b50505050601b60019054906101000a90046001600160a01b03166001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610894575f5ffd5b505af11580156108a6573d5f5f3e3d5ffd5b505050505050565b60606012805480602002602001604051908101604052809291908181526020015f905b8282101561098f575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561097757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116109395790505b505050505081525050815260200190600101906108d1565b50505050905090565b60606011805480602002602001604051908101604052809291908181526020015f905b8282101561098f578382905f5260205f200180546109d8906113aa565b80601f0160208091040260200160405190810160405280929190818152602001828054610a04906113aa565b8015610a4f5780601f10610a2657610100808354040283529160200191610a4f565b820191905f5260205f20905b815481529060010190602001808311610a3257829003601f168201915b5050505050815260200190600101906109bb565b60606013805480602002602001604051908101604052809291908181526020015f905b8282101561098f575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015610b2c57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411610aee5790505b50505050508152505081526020019060010190610a86565b60606010805480602002602001604051908101604052809291908181526020015f905b8282101561098f578382905f5260205f20018054610b84906113aa565b80601f0160208091040260200160405190810160405280929190818152602001828054610bb0906113aa565b8015610bfb5780601f10610bd257610100808354040283529160200191610bfb565b820191905f5260205f20905b815481529060010190602001808311610bde57829003601f168201915b505050505081526020019060010190610b67565b5f8054610100900460ff1615610c2d57505f54610100900460ff1690565b5f737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610d2d5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093525f929091610cb9917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016113f9565b60408051601f1981840301815290829052610cd39161141c565b5f604051808303815f865af19150503d805f8114610d0c576040519150601f19603f3d011682016040523d82523d5f602084013e610d11565b606091505b5091505080806020019051810190610d299190611427565b9150505b919050565b60408051818152601c818301527f596f7520617265206465706c6f79696e67206f6e20436861696e4944000000006060820152466020820181905291517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160018114610ddf5760405162461bcd60e51b815260206004820152601360248201527210da185a5b881b9bdd081cdd5c1c1bdc9d1959606a1b60448201526064016103a2565b601c5f9054906101000a90046001600160a01b03166001600160a01b031663fdc371ce6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e2f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e5391906112a1565b602180546001600160a01b0319166001600160a01b0392909216918217905573ec53bf9167f50cdeb3ae105f56099aaab9061f8314610ed45760405162461bcd60e51b815260206004820152601860248201527f736f6d657468696e6720686f727269626c792077726f6e67000000000000000060448201526064016103a2565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c6001600160a01b0316637fb5297f6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610f2e575f5ffd5b505af1158015610f40573d5f5f3e3d5ffd5b50506021546040516001600160a01b039091169250610f5f91506110f9565b6001600160a01b039091168152602001604051809103905ff080158015610f88573d5f5f3e3d5ffd5b50601d5f6101000a8154816001600160a01b0302191690836001600160a01b031602179055507f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c6001600160a01b03166376eadd366040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611008575f5ffd5b505af115801561101a573d5f5f3e3d5ffd5b5050601d5460408051818152601581830152743122a4a3a2a72fb4b6b83632b6b2b73a30ba34b7b760591b60608201526001600160a01b039092166020830152517f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f9350908190036080019150a161109061059d565b611098610270565b50565b6060600c8054806020026020016040519081016040528092919081815260200182805480156104d757602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116104b9575050505050905090565b612d118061144783390190565b602080825282518282018190525f918401906040840190835b818110156111465783516001600160a01b031683526020938401939092019160010161111f565b509095945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156111fc57868503603f19018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101905f9060608801905b808310156111e45783516001600160e01b031916825260209384019360019390930192909101906111b8565b50965050506020938401939190910190600101611177565b50929695505050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156111fc57603f19878603018452611278858351611208565b9450602093840193919091019060010161125c565b6001600160a01b0381168114611098575f5ffd5b5f602082840312156112b1575f5ffd5b81516112bc8161128d565b9392505050565b5f602082840312156112d3575f5ffd5b5051919050565b6040815260046040820152636461746160e01b6060820152608060208201525f6112bc6080830184611208565b60018060a01b038716815285602082015260c060408201525f61132d60c0830187611208565b606083019590955250608081019290925260a0909101529392505050565b8082018082111561136a57634e487b7160e01b5f52601160045260245ffd5b92915050565b60018060a01b038616815284602082015260a060408201525f61139660a0830186611208565b606083019490945250608001529392505050565b600181811c908216806113be57607f821691505b6020821081036113dc57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f81518060208401855e5f93019283525090919050565b6001600160e01b0319831681525f61141460048301846113e2565b949350505050565b5f6112bc82846113e2565b5f60208284031215611437575f5ffd5b815180151581146112bc575f5ffdfe60a060405234801561000f575f5ffd5b50604051612d11380380612d1183398101604081905261002e91610105565b6001600160a01b038116608052610043610049565b50610132565b5f54610100900460ff16156100b45760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff90811614610103575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b5f60208284031215610115575f5ffd5b81516001600160a01b038116811461012b575f5ffd5b9392505050565b608051612bb261015f5f395f81816105e901528181610dae01528181610dd90152610e040152612bb25ff3fe608060405234801561000f575f5ffd5b506004361061023f575f3560e01c80637ecebe0011610135578063aa271e1a116100b4578063dd62ed3e11610079578063dd62ed3e14610579578063eb415f451461058c578063f1127ed814610594578063f2fde38b146105d1578063fdc371ce146105e4575f5ffd5b8063aa271e1a1461050a578063b8c255941461052d578063c3cda52014610540578063c4d66de814610553578063d505accf14610566575f5ffd5b806395d89b41116100fa57806395d89b41146104bf5780639ab24eb0146104c75780639aec4bae146104da578063a457c2d7146104e4578063a9059cbb146104f7575f5ffd5b80637ecebe001461044e57806384b0196e146104615780638da5cb5b1461047c5780638e539e8c1461048d57806391ddadf4146104a0575f5ffd5b806340c10f19116101c157806366eb399f1161018657806366eb399f146103c05780636fcfff45146103d357806370a08231146103fb578063715018a61461042357806378aa33ba1461042b575f5ffd5b806340c10f191461031a57806342966c681461032d5780634bf5d7e914610340578063587cde1e1461036a5780635c19a95c146103ad575f5ffd5b806323b872dd1161020757806323b872dd146102ca578063313ce567146102dd5780633644e515146102ec57806339509351146102f45780633a46b1a814610307575f5ffd5b80630455e6941461024357806306fdde031461027b578063095ea7b31461029057806318160ddd146102a35780631ffacdef146102b5575b5f5ffd5b610266610251366004612682565b6101316020525f908152604090205460ff1681565b60405190151581526020015b60405180910390f35b61028361060b565b60405161027291906126c9565b61026661029e3660046126db565b61069b565b6067545b604051908152602001610272565b6102c86102c3366004612703565b6106b4565b005b6102666102d836600461273c565b6106ca565b60405160128152602001610272565b6102a76106ed565b6102666103023660046126db565b6106fb565b6102a76103153660046126db565b61071c565b6102c86103283660046126db565b6107a4565b6102c861033b366004612776565b61081f565b60408051808201909152600e81526d06d6f64653d74696d657374616d760941b6020820152610283565b610395610378366004612682565b6001600160a01b039081165f90815260fe60205260409020541690565b6040516001600160a01b039091168152602001610272565b6102c86103bb366004612682565b61082c565b6102c86103ce366004612703565b610836565b6103e66103e1366004612682565b6108ae565b60405163ffffffff9091168152602001610272565b6102a7610409366004612682565b6001600160a01b03165f9081526065602052604090205490565b6102c86108cf565b610266610439366004612682565b6101326020525f908152604090205460ff1681565b6102a761045c366004612682565b6108e2565b6104696108ff565b604051610272979695949392919061278d565b6033546001600160a01b0316610395565b6102a761049b366004612776565b610998565b6104a86109ff565b60405165ffffffffffff9091168152602001610272565b610283610a09565b6102a76104d5366004612682565b610a18565b6102a76101305481565b6102666104f23660046126db565b610a95565b6102666105053660046126db565b610b0f565b610266610518366004612682565b6101336020525f908152604090205460ff1681565b6102c861053b366004612703565b610b1c565b6102c861054e366004612833565b610b2e565b6102c8610561366004612682565b610c63565b6102c8610574366004612887565b610ea5565b6102a76105873660046128ed565b611006565b6102c8611030565b6105a76105a236600461291e565b6110fe565b60408051825163ffffffff1681526020928301516001600160e01b03169281019290925201610272565b6102c86105df366004612682565b61117f565b6103957f000000000000000000000000000000000000000000000000000000000000000081565b60606068805461061a90612950565b80601f016020809104026020016040519081016040528092919081815260200182805461064690612950565b80156106915780601f1061066857610100808354040283529160200191610691565b820191905f5260205f20905b81548152906001019060200180831161067457829003601f168201915b5050505050905090565b5f336106a88185856111f5565b60019150505b92915050565b6106bc611318565b6106c68282611372565b5050565b5f336106d78582856113d2565b6106e285858561144a565b506001949350505050565b5f6106f6611604565b905090565b5f336106a881858561070d8383611006565b6107179190612996565b6111f5565b5f6107256109ff565b65ffffffffffff16821061077c5760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b60448201526064015b60405180910390fd5b6001600160a01b0383165f90815260ff6020526040902061079d908361160d565b9392505050565b335f908152610133602052604090205460ff166108155760405162461bcd60e51b815260206004820152602960248201527f4261636b696e67456967656e2e6d696e743a2063616c6c6572206973206e6f7460448201526810309036b4b73a32b960b91b6064820152608401610773565b6106c682826116ee565b6108293382611779565b50565b6108293382611792565b61083e611318565b816001600160a01b03167f0124b12503bddc2616c0f3f54fd23ed283f5ef0c1483a75409e42612176b8bde8260405161087b911515815260200190565b60405180910390a26001600160a01b03919091165f90815261013360205260409020805460ff1916911515919091179055565b6001600160a01b0381165f90815260ff60205260408120546106ae9061180b565b6108d7611318565b6108e05f611873565b565b6001600160a01b0381165f90815260cb60205260408120546106ae565b5f6060805f5f5f60606097545f5f1b14801561091b5750609854155b61095f5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b6044820152606401610773565b6109676118c4565b61096f6118d3565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f6109a16109ff565b65ffffffffffff1682106109f35760405162461bcd60e51b815260206004820152601960248201527804552433230566f7465733a20667574757265206c6f6f6b757603c1b6044820152606401610773565b6106ae6101008361160d565b5f6106f6426118e2565b60606069805461061a90612950565b6001600160a01b0381165f90815260ff60205260408120548015610a83576001600160a01b0383165f90815260ff6020526040902080545f198301908110610a6257610a626129bd565b5f9182526020909120015464010000000090046001600160e01b0316610a85565b5f5b6001600160e01b03169392505050565b5f3381610aa28286611006565b905083811015610b025760405162461bcd60e51b815260206004820152602560248201527f45524332303a2064656372656173656420616c6c6f77616e63652062656c6f77604482015264207a65726f60d81b6064820152608401610773565b6106e282868684036111f5565b5f336106a881858561144a565b610b24611318565b6106c68282611948565b83421115610b7e5760405162461bcd60e51b815260206004820152601d60248201527f4552433230566f7465733a207369676e617475726520657870697265640000006044820152606401610773565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590525f90610bf790610bef9060a001604051602081830303815290604052805190602001206119a0565b8585856119cc565b9050610c02816119f2565b8614610c505760405162461bcd60e51b815260206004820152601960248201527f4552433230566f7465733a20696e76616c6964206e6f6e6365000000000000006044820152606401610773565b610c5a8188611792565b50505050505050565b5f54610100900460ff1615808015610c8157505f54600160ff909116105b80610c9a5750303b158015610c9a57505f5460ff166001145b610cfd5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610773565b5f805460ff191660011790558015610d1e575f805461ff0019166101001790555b610d26611a19565b610d736040518060400160405280600d81526020016c2130b1b5b4b7339022b4b3b2b760991b815250604051806040016040528060068152602001653122a4a3a2a760d11b815250611a47565b610d7c82611873565b610da3604051806040016040528060068152602001653122a4a3a2a760d11b815250611a77565b5f1961013055610dd47f00000000000000000000000000000000000000000000000000000000000000006001611372565b610dff7f00000000000000000000000000000000000000000000000000000000000000006001611948565b610e357f00000000000000000000000000000000000000000000000000000000000000006b05686877afb5cbccbf7340006116ee565b6040517fb7c23c1e2e36f298e9879a88ecfcd07e28fbb439bcfa9c78ca1363ca14370d26905f90a180156106c6575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b83421115610ef55760405162461bcd60e51b815260206004820152601d60248201527f45524332305065726d69743a206578706972656420646561646c696e650000006044820152606401610773565b5f7f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9888888610f238c6119f2565b6040805160208101969096526001600160a01b0394851690860152929091166060840152608083015260a082015260c0810186905260e0016040516020818303038152906040528051906020012090505f610f7d826119a0565b90505f610f8c828787876119cc565b9050896001600160a01b0316816001600160a01b031614610fef5760405162461bcd60e51b815260206004820152601e60248201527f45524332305065726d69743a20696e76616c6964207369676e617475726500006044820152606401610773565b610ffa8a8a8a6111f5565b50505050505050505050565b6001600160a01b039182165f90815260666020908152604080832093909416825291909152205490565b611038611318565b5f1961013054146110ce5760405162461bcd60e51b815260206004820152605460248201527f4261636b696e67456967656e2e64697361626c655472616e736665725265737460448201527f72696374696f6e733a207472616e73666572207265737472696374696f6e7320606482015273185c9948185b1c9958591e48191a5cd8589b195960621b608482015260a401610773565b5f6101308190556040517f2b18986d3ba809db2f13a5d7bf17f60d357b37d9cbb55dd71cbbac8dc4060f649190a1565b604080518082019091525f80825260208201526001600160a01b0383165f90815260ff60205260409020805463ffffffff8416908110611140576111406129bd565b5f9182526020918290206040805180820190915291015463ffffffff8116825264010000000090046001600160e01b0316918101919091529392505050565b611187611318565b6001600160a01b0381166111ec5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610773565b61082981611873565b6001600160a01b0383166112575760405162461bcd60e51b8152602060048201526024808201527f45524332303a20617070726f76652066726f6d20746865207a65726f206164646044820152637265737360e01b6064820152608401610773565b6001600160a01b0382166112b85760405162461bcd60e51b815260206004820152602260248201527f45524332303a20617070726f766520746f20746865207a65726f206164647265604482015261737360f01b6064820152608401610773565b6001600160a01b038381165f8181526066602090815260408083209487168084529482529182902085905590518481527f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a3505050565b6033546001600160a01b031633146108e05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610773565b6001600160a01b0382165f8181526101316020908152604091829020805460ff191685151590811790915591519182527fcf20b1ecb604b0e8888d579c64e8a3b10e590d45c1c2dddb393bed284362227191015b60405180910390a25050565b5f6113dd8484611006565b90505f19811461144457818110156114375760405162461bcd60e51b815260206004820152601d60248201527f45524332303a20696e73756666696369656e7420616c6c6f77616e63650000006044820152606401610773565b61144484848484036111f5565b50505050565b6001600160a01b0383166114ae5760405162461bcd60e51b815260206004820152602560248201527f45524332303a207472616e736665722066726f6d20746865207a65726f206164604482015264647265737360d81b6064820152608401610773565b6001600160a01b0382166115105760405162461bcd60e51b815260206004820152602360248201527f45524332303a207472616e7366657220746f20746865207a65726f206164647260448201526265737360e81b6064820152608401610773565b61151b838383611ac0565b6001600160a01b0383165f90815260656020526040902054818110156115925760405162461bcd60e51b815260206004820152602660248201527f45524332303a207472616e7366657220616d6f756e7420657863656564732062604482015265616c616e636560d01b6064820152608401610773565b6001600160a01b038085165f8181526065602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef906115f19086815260200190565b60405180910390a3611444848484611b9c565b5f6106f6611bcd565b81545f9081816005811115611664575f61162684611c40565b61163090856129d1565b5f88815260209020909150869082015463ffffffff16111561165457809150611662565b61165f816001612996565b92505b505b808210156116af575f6116778383611d24565b5f88815260209020909150869082015463ffffffff16111561169b578091506116a9565b6116a6816001612996565b92505b50611664565b80156116d9575f8681526020902081015f19015464010000000090046001600160e01b03166116db565b5f5b6001600160e01b03169695505050505050565b6116f88282611d3e565b6067546001600160e01b03101561176a5760405162461bcd60e51b815260206004820152603060248201527f4552433230566f7465733a20746f74616c20737570706c79207269736b73206f60448201526f766572666c6f77696e6720766f74657360801b6064820152608401610773565b611444610100611e0f83611e1a565b6117838282611f86565b6114446101006120c983611e1a565b6001600160a01b038281165f81815260fe6020818152604080842080546065845282862054949093528787166001600160a01b03198416811790915590519190951694919391928592917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a46114448284836120d4565b5f63ffffffff82111561186f5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610773565b5090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60606099805461061a90612950565b6060609a805461061a90612950565b5f65ffffffffffff82111561186f5760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203460448201526538206269747360d01b6064820152608401610773565b6001600160a01b0382165f8181526101326020908152604091829020805460ff191685151590811790915591519182527f72a561d1af7409467dae4f1e9fc52590a9335a1dda17727e2b6aa8c4db35109b91016113c6565b5f6106ae6119ac611604565b8360405161190160f01b8152600281019290925260228201526042902090565b5f5f5f6119db8787878761220e565b915091506119e8816122cb565b5095945050505050565b6001600160a01b0381165f90815260cb602052604090208054600181018255905b50919050565b5f54610100900460ff16611a3f5760405162461bcd60e51b8152600401610773906129e4565b6108e0612414565b5f54610100900460ff16611a6d5760405162461bcd60e51b8152600401610773906129e4565b6106c68282612443565b5f54610100900460ff16611a9d5760405162461bcd60e51b8152600401610773906129e4565b61082981604051806040016040528060018152602001603160f81b815250612482565b610130544211611b97576001600160a01b0383165f908152610131602052604090205460ff1680611b0957506001600160a01b0382165f908152610132602052604090205460ff165b80611b1b57506001600160a01b038316155b611b975760405162461bcd60e51b815260206004820152604160248201527f4261636b696e67456967656e2e5f6265666f7265546f6b656e5472616e73666560448201527f723a2066726f6d206f7220746f206d7573742062652077686974656c697374656064820152601960fa1b608482015260a401610773565b505050565b6001600160a01b038381165f90815260fe6020526040808220548584168352912054611b97929182169116836120d4565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611bf76124cf565b611bff612527565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f815f03611c4f57505f919050565b5f6001611c5b84612557565b901c6001901b90506001818481611c7457611c74612a2f565b048201901c90506001818481611c8c57611c8c612a2f565b048201901c90506001818481611ca457611ca4612a2f565b048201901c90506001818481611cbc57611cbc612a2f565b048201901c90506001818481611cd457611cd4612a2f565b048201901c90506001818481611cec57611cec612a2f565b048201901c90506001818481611d0457611d04612a2f565b048201901c905061079d81828581611d1e57611d1e612a2f565b046125ea565b5f611d326002848418612a43565b61079d90848416612996565b6001600160a01b038216611d945760405162461bcd60e51b815260206004820152601f60248201527f45524332303a206d696e7420746f20746865207a65726f2061646472657373006044820152606401610773565b611d9f5f8383611ac0565b8060675f828254611db09190612996565b90915550506001600160a01b0382165f818152606560209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a36106c65f8383611b9c565b5f61079d8284612996565b82545f908190818115611e64575f8781526020902082015f190160408051808201909152905463ffffffff8116825264010000000090046001600160e01b03166020820152611e78565b604080518082019091525f80825260208201525b905080602001516001600160e01b03169350611e9884868863ffffffff16565b92505f82118015611ec05750611eac6109ff565b65ffffffffffff16815f015163ffffffff16145b15611f0357611ece836125ff565b5f8881526020902083015f190180546001600160e01b03929092166401000000000263ffffffff909216919091179055611f7c565b866040518060400160405280611f27611f1a6109ff565b65ffffffffffff1661180b565b63ffffffff168152602001611f3b866125ff565b6001600160e01b0390811690915282546001810184555f938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b5050935093915050565b6001600160a01b038216611fe65760405162461bcd60e51b815260206004820152602160248201527f45524332303a206275726e2066726f6d20746865207a65726f206164647265736044820152607360f81b6064820152608401610773565b611ff1825f83611ac0565b6001600160a01b0382165f90815260656020526040902054818110156120645760405162461bcd60e51b815260206004820152602260248201527f45524332303a206275726e20616d6f756e7420657863656564732062616c616e604482015261636560f01b6064820152608401610773565b6001600160a01b0383165f8181526065602090815260408083208686039055606780548790039055518581529192917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a3611b97835f84611b9c565b5f61079d82846129d1565b816001600160a01b0316836001600160a01b0316141580156120f557505f81115b15611b97576001600160a01b03831615612182576001600160a01b0383165f90815260ff60205260408120819061212f906120c985611e1a565b91509150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a7248383604051612177929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615611b97576001600160a01b0382165f90815260ff6020526040812081906121b790611e0f85611e1a565b91509150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516121ff929190918252602082015260400190565b60405180910390a25050505050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561224357505f905060036122c2565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015612294573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b0381166122bc575f600192509250506122c2565b91505f90505b94509492505050565b5f8160048111156122de576122de612a62565b036122e65750565b60018160048111156122fa576122fa612a62565b036123475760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610773565b600281600481111561235b5761235b612a62565b036123a85760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610773565b60038160048111156123bc576123bc612a62565b036108295760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610773565b5f54610100900460ff1661243a5760405162461bcd60e51b8152600401610773906129e4565b6108e033611873565b5f54610100900460ff166124695760405162461bcd60e51b8152600401610773906129e4565b60686124758382612ac1565b506069611b978282612ac1565b5f54610100900460ff166124a85760405162461bcd60e51b8152600401610773906129e4565b60996124b48382612ac1565b50609a6124c18282612ac1565b50505f609781905560985550565b5f5f6124d96118c4565b8051909150156124f0578051602090910120919050565b60975480156124ff5792915050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4709250505090565b5f5f6125316118d3565b805190915015612548578051602090910120919050565b60985480156124ff5792915050565b5f80608083901c1561256b57608092831c92015b604083901c1561257d57604092831c92015b602083901c1561258f57602092831c92015b601083901c156125a157601092831c92015b600883901c156125b357600892831c92015b600483901c156125c557600492831c92015b600283901c156125d757600292831c92015b600183901c156106ae5760010192915050565b5f8183106125f8578161079d565b5090919050565b5f6001600160e01b0382111561186f5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610773565b80356001600160a01b038116811461267d575f5ffd5b919050565b5f60208284031215612692575f5ffd5b61079d82612667565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61079d602083018461269b565b5f5f604083850312156126ec575f5ffd5b6126f583612667565b946020939093013593505050565b5f5f60408385031215612714575f5ffd5b61271d83612667565b915060208301358015158114612731575f5ffd5b809150509250929050565b5f5f5f6060848603121561274e575f5ffd5b61275784612667565b925061276560208501612667565b929592945050506040919091013590565b5f60208284031215612786575f5ffd5b5035919050565b60ff60f81b8816815260e060208201525f6127ab60e083018961269b565b82810360408401526127bd818961269b565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b818110156128125783518352602093840193909201916001016127f4565b50909b9a5050505050505050505050565b803560ff8116811461267d575f5ffd5b5f5f5f5f5f5f60c08789031215612848575f5ffd5b61285187612667565b9550602087013594506040870135935061286d60608801612823565b9598949750929560808101359460a0909101359350915050565b5f5f5f5f5f5f5f60e0888a03121561289d575f5ffd5b6128a688612667565b96506128b460208901612667565b955060408801359450606088013593506128d060808901612823565b9699959850939692959460a0840135945060c09093013592915050565b5f5f604083850312156128fe575f5ffd5b61290783612667565b915061291560208401612667565b90509250929050565b5f5f6040838503121561292f575f5ffd5b61293883612667565b9150602083013563ffffffff81168114612731575f5ffd5b600181811c9082168061296457607f821691505b602082108103611a1357634e487b7160e01b5f52602260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b808201808211156106ae576106ae612982565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b818103818111156106ae576106ae612982565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b634e487b7160e01b5f52601260045260245ffd5b5f82612a5d57634e487b7160e01b5f52601260045260245ffd5b500490565b634e487b7160e01b5f52602160045260245ffd5b601f821115611b9757805f5260205f20601f840160051c81016020851015612a9b5750805b601f840160051c820191505b81811015612aba575f8155600101612aa7565b5050505050565b815167ffffffffffffffff811115612adb57612adb6129a9565b612aef81612ae98454612950565b84612a76565b6020601f821160018114612b21575f8315612b0a5750848201515b5f19600385901b1c1916600184901b178455612aba565b5f84815260208120601f198516915b82811015612b505787850151825560209485019460019092019101612b30565b5084821015612b6d57868401515f19600387901b60f8161c191681555b50505050600190811b0190555056fea2646970667358221220bc65aa22c82ea1986fef941c8ffca5d6f9991e3fb75b2eefff53e26501e45f9064736f6c634300081b0033a2646970667358221220a8adc4eee0589780040daa29e7e01f5d6e4b8e17eada37d0bfbdaf228290c29e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01'W_5`\xE0\x1C\x80c\x91j\x17\xC6\x11a\0\xA9W\x80c\xC3\xEA?\xC9\x11a\0nW\x80c\xC3\xEA?\xC9\x14a\x02)W\x80c\xD7\x15\xBCn\x14a\x02<W\x80c\xE2\x0C\x9Fq\x14a\x02OW\x80c\xF8\xCC\xBFG\x14a\x02WW\x80c\xFAv&\xD4\x14a\x02dW__\xFD[\x80c\x91j\x17\xC6\x14a\x01\xE6W\x80c\xA5a|\xFD\x14a\x01\xEEW\x80c\xB5P\x8A\xA9\x14a\x02\x01W\x80c\xBAAO\xA6\x14a\x02\tW\x80c\xC0@b&\x14a\x02!W__\xFD[\x80c?r\x86\xF4\x11a\0\xEFW\x80c?r\x86\xF4\x14a\x01\x99W\x80cTu[\x99\x14a\x01\xA1W\x80cf\xD9\xA9\xA0\x14a\x01\xA9W\x80c\x81\x1F\xFE\xEB\x14a\x01\xBEW\x80c\x85\"l\x81\x14a\x01\xD1W__\xFD[\x80c\x14\xF8\xFF\xAC\x14a\x01+W\x80c\x1E\xD7\x83\x1C\x14a\x015W\x80c5\xE4\xA06\x14a\x01SW\x80c7\x0C0\xD9\x14a\x01~W\x80c>^<#\x14a\x01\x91W[__\xFD[a\x013a\x02pV[\0[a\x01=a\x04\x81V[`@Qa\x01J\x91\x90a\x11\x06V[`@Q\x80\x91\x03\x90\xF3[` Ta\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01JV[`!Ta\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01=a\x04\xE1V[a\x01=a\x05?V[a\x013a\x05\x9DV[a\x01\xB1a\x08\xAEV[`@Qa\x01J\x91\x90a\x11QV[`\x1FTa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD9a\t\x98V[`@Qa\x01J\x91\x90a\x126V[a\x01\xB1a\ncV[`\x1CTa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xD9a\x0BDV[a\x02\x11a\x0C\x0FV[`@Q\x90\x15\x15\x81R` \x01a\x01JV[a\x013a\r2V[`\x1DTa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x1ETa\x01f\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01=a\x10\x9BV[`\x1BTa\x02\x11\x90`\xFF\x16\x81V[_Ta\x02\x11\x90`\xFF\x16\x81V[`\x1FT`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x02\xC6W__\xFD[PZ\xF1\x15\x80\x15a\x02\xD8W=__>=_\xFD[PP`\x1DT`\x1ET`\x1CT`@Qc\x10'\x0E=`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x92\x81\x16\x94P\x16\x91Pc N\x1Cz\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03P\x91\x90a\x12\xA1V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fimplementation set incorrectly\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`!T`\x1CT`@\x80Qc~\xE1\xB8\xE7`\xE1\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x93\x90\x92\x16\x91c\xFD\xC3q\xCE\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x03\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1E\x91\x90a\x12\xA1V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEIGEN address changed unexpected`D\x82\x01Raly`\xF0\x1B`d\x82\x01R`\x84\x01a\x03\xA2V[V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9W[PPPPP\x90P\x90V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9WPPPPP\x90P\x90V[`\x1FT`@\x80Qcy=\x06I`\xE1\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2z\x0C\x92\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x08\x91\x90a\x12\xC3V[`\x1CT`\x1DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x92\x90\x91\x16`D\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c&j#\xB1`\xE2\x1B\x17\x90RQ\x91\x92P\x90\x7F\xD2n\x16\xCA\xD4T\x87\x05\xE4\xC9\xE2\xD9O\x98\xEE\x91\xC2\x89\x08^\xE4%YO\xD5c_\xA2\x96L\xCF\x18\x90a\x06\x8C\x90\x83\x90a\x12\xDAV[`@Q\x80\x91\x03\x90\xA1` T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xEAW__\xFD[PZ\xF1\x15\x80\x15a\x06\xFCW=__>=_\xFD[PP`\x1FT`\x1ET`@Qb\xEA\x83\x15`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x01\xD5\x06*\x93Pa\x07>\x92\x90\x91\x16\x90_\x90\x86\x90\x82\x90\x81\x90\x8A\x90`\x04\x01a\x13\x07V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07UW__\xFD[PZ\xF1\x15\x80\x15a\x07gW=__>=_\xFD[Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92Pc\xE5\xD6\xBF\x02\x91Pa\x07\x92\x90P\x84Ba\x13KV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xB0\x91\x81R` \x01\x90V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xC7W__\xFD[PZ\xF1\x15\x80\x15a\x07\xD9W=__>=_\xFD[PP`\x1FT`\x1ET`@Qc\x13@\x08\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\x13@\x08\xD3\x93Pa\x08\x1A\x92\x90\x91\x16\x90_\x90\x86\x90\x82\x90\x81\x90`\x04\x01a\x13pV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x081W__\xFD[PZ\xF1\x15\x80\x15a\x08CW=__>=_\xFD[PPPP`\x1B`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x94W__\xFD[PZ\xF1\x15\x80\x15a\x08\xA6W=__>=_\xFD[PPPPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\twW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\t9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x08\xD1V[PPPP\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW\x83\x82\x90_R` _ \x01\x80Ta\t\xD8\x90a\x13\xAAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x04\x90a\x13\xAAV[\x80\x15a\nOW\x80`\x1F\x10a\n&Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nOV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t\xBBV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0B,W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\n\xEEW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\x86V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\t\x8FW\x83\x82\x90_R` _ \x01\x80Ta\x0B\x84\x90a\x13\xAAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB0\x90a\x13\xAAV[\x80\x15a\x0B\xFBW\x80`\x1F\x10a\x0B\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFBV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xDEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0BgV[_\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x0C-WP_Ta\x01\0\x90\x04`\xFF\x16\x90V[_sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\r-W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R_\x92\x90\x91a\x0C\xB9\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x13\xF9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\xD3\x91a\x14\x1CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\r\x0CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\r\x11V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\r)\x91\x90a\x14'V[\x91PP[\x91\x90PV[`@\x80Q\x81\x81R`\x1C\x81\x83\x01R\x7FYou are deploying on ChainID\0\0\0\0``\x82\x01RF` \x82\x01\x81\x90R\x91Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`\x01\x81\x14a\r\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10\xDA\x18Z[\x88\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`D\x82\x01R`d\x01a\x03\xA2V[`\x1C_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xFD\xC3q\xCE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ES\x91\x90a\x12\xA1V[`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Us\xECS\xBF\x91g\xF5\x0C\xDE\xB3\xAE\x10_V\t\x9A\xAA\xB9\x06\x1F\x83\x14a\x0E\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fsomething horribly wrong\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\xA2V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F.W__\xFD[PZ\xF1\x15\x80\x15a\x0F@W=__>=_\xFD[PP`!T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pa\x0F_\x91Pa\x10\xF9V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x0F\x88W=__>=_\xFD[P`\x1D_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x08W__\xFD[PZ\xF1\x15\x80\x15a\x10\x1AW=__>=_\xFD[PP`\x1DT`@\x80Q\x81\x81R`\x15\x81\x83\x01Rt1\"\xA4\xA3\xA2\xA7/\xB4\xB6\xB862\xB6\xB2\xB7:0\xBA4\xB7\xB7`Y\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x83\x01RQ\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x93P\x90\x81\x90\x03`\x80\x01\x91P\xA1a\x10\x90a\x05\x9DV[a\x10\x98a\x02pV[PV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xB9WPPPPP\x90P\x90V[a-\x11\x80a\x14G\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x11FW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x11\x1FV[P\x90\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x11\xFCW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90_\x90``\x88\x01\x90[\x80\x83\x10\x15a\x11\xE4W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a\x11\xB8V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x11wV[P\x92\x96\x95PPPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x11\xFCW`?\x19\x87\x86\x03\x01\x84Ra\x12x\x85\x83Qa\x12\x08V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x12\\V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\x98W__\xFD[_` \x82\x84\x03\x12\x15a\x12\xB1W__\xFD[\x81Qa\x12\xBC\x81a\x12\x8DV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x12\xD3W__\xFD[PQ\x91\x90PV[`@\x81R`\x04`@\x82\x01Rcdata`\xE0\x1B``\x82\x01R`\x80` \x82\x01R_a\x12\xBC`\x80\x83\x01\x84a\x12\x08V[`\x01\x80`\xA0\x1B\x03\x87\x16\x81R\x85` \x82\x01R`\xC0`@\x82\x01R_a\x13-`\xC0\x83\x01\x87a\x12\x08V[``\x83\x01\x95\x90\x95RP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x13jWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\xA0`@\x82\x01R_a\x13\x96`\xA0\x83\x01\x86a\x12\x08V[``\x83\x01\x94\x90\x94RP`\x80\x01R\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x13\xBEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x13\xDCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R_a\x14\x14`\x04\x83\x01\x84a\x13\xE2V[\x94\x93PPPPV[_a\x12\xBC\x82\x84a\x13\xE2V[_` \x82\x84\x03\x12\x15a\x147W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12\xBCW__\xFD\xFE`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa-\x118\x03\x80a-\x11\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x05V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Ra\0Ca\0IV[Pa\x012V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x01\x03W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[_` \x82\x84\x03\x12\x15a\x01\x15W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01+W__\xFD[\x93\x92PPPV[`\x80Qa+\xB2a\x01__9_\x81\x81a\x05\xE9\x01R\x81\x81a\r\xAE\x01R\x81\x81a\r\xD9\x01Ra\x0E\x04\x01Ra+\xB2_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02?W_5`\xE0\x1C\x80c~\xCE\xBE\0\x11a\x015W\x80c\xAA'\x1E\x1A\x11a\0\xB4W\x80c\xDDb\xED>\x11a\0yW\x80c\xDDb\xED>\x14a\x05yW\x80c\xEBA_E\x14a\x05\x8CW\x80c\xF1\x12~\xD8\x14a\x05\x94W\x80c\xF2\xFD\xE3\x8B\x14a\x05\xD1W\x80c\xFD\xC3q\xCE\x14a\x05\xE4W__\xFD[\x80c\xAA'\x1E\x1A\x14a\x05\nW\x80c\xB8\xC2U\x94\x14a\x05-W\x80c\xC3\xCD\xA5 \x14a\x05@W\x80c\xC4\xD6m\xE8\x14a\x05SW\x80c\xD5\x05\xAC\xCF\x14a\x05fW__\xFD[\x80c\x95\xD8\x9BA\x11a\0\xFAW\x80c\x95\xD8\x9BA\x14a\x04\xBFW\x80c\x9A\xB2N\xB0\x14a\x04\xC7W\x80c\x9A\xECK\xAE\x14a\x04\xDAW\x80c\xA4W\xC2\xD7\x14a\x04\xE4W\x80c\xA9\x05\x9C\xBB\x14a\x04\xF7W__\xFD[\x80c~\xCE\xBE\0\x14a\x04NW\x80c\x84\xB0\x19n\x14a\x04aW\x80c\x8D\xA5\xCB[\x14a\x04|W\x80c\x8ES\x9E\x8C\x14a\x04\x8DW\x80c\x91\xDD\xAD\xF4\x14a\x04\xA0W__\xFD[\x80c@\xC1\x0F\x19\x11a\x01\xC1W\x80cf\xEB9\x9F\x11a\x01\x86W\x80cf\xEB9\x9F\x14a\x03\xC0W\x80co\xCF\xFFE\x14a\x03\xD3W\x80cp\xA0\x821\x14a\x03\xFBW\x80cqP\x18\xA6\x14a\x04#W\x80cx\xAA3\xBA\x14a\x04+W__\xFD[\x80c@\xC1\x0F\x19\x14a\x03\x1AW\x80cB\x96lh\x14a\x03-W\x80cK\xF5\xD7\xE9\x14a\x03@W\x80cX|\xDE\x1E\x14a\x03jW\x80c\\\x19\xA9\\\x14a\x03\xADW__\xFD[\x80c#\xB8r\xDD\x11a\x02\x07W\x80c#\xB8r\xDD\x14a\x02\xCAW\x80c1<\xE5g\x14a\x02\xDDW\x80c6D\xE5\x15\x14a\x02\xECW\x80c9P\x93Q\x14a\x02\xF4W\x80c:F\xB1\xA8\x14a\x03\x07W__\xFD[\x80c\x04U\xE6\x94\x14a\x02CW\x80c\x06\xFD\xDE\x03\x14a\x02{W\x80c\t^\xA7\xB3\x14a\x02\x90W\x80c\x18\x16\r\xDD\x14a\x02\xA3W\x80c\x1F\xFA\xCD\xEF\x14a\x02\xB5W[__\xFD[a\x02fa\x02Q6`\x04a&\x82V[a\x011` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x83a\x06\x0BV[`@Qa\x02r\x91\x90a&\xC9V[a\x02fa\x02\x9E6`\x04a&\xDBV[a\x06\x9BV[`gT[`@Q\x90\x81R` \x01a\x02rV[a\x02\xC8a\x02\xC36`\x04a'\x03V[a\x06\xB4V[\0[a\x02fa\x02\xD86`\x04a'<V[a\x06\xCAV[`@Q`\x12\x81R` \x01a\x02rV[a\x02\xA7a\x06\xEDV[a\x02fa\x03\x026`\x04a&\xDBV[a\x06\xFBV[a\x02\xA7a\x03\x156`\x04a&\xDBV[a\x07\x1CV[a\x02\xC8a\x03(6`\x04a&\xDBV[a\x07\xA4V[a\x02\xC8a\x03;6`\x04a'vV[a\x08\x1FV[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\x83V[a\x03\x95a\x03x6`\x04a&\x82V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\xFE` R`@\x90 T\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02rV[a\x02\xC8a\x03\xBB6`\x04a&\x82V[a\x08,V[a\x02\xC8a\x03\xCE6`\x04a'\x03V[a\x086V[a\x03\xE6a\x03\xE16`\x04a&\x82V[a\x08\xAEV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02rV[a\x02\xA7a\x04\t6`\x04a&\x82V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`e` R`@\x90 T\x90V[a\x02\xC8a\x08\xCFV[a\x02fa\x0496`\x04a&\x82V[a\x012` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA7a\x04\\6`\x04a&\x82V[a\x08\xE2V[a\x04ia\x08\xFFV[`@Qa\x02r\x97\x96\x95\x94\x93\x92\x91\x90a'\x8DV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x95V[a\x02\xA7a\x04\x9B6`\x04a'vV[a\t\x98V[a\x04\xA8a\t\xFFV[`@Qe\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02rV[a\x02\x83a\n\tV[a\x02\xA7a\x04\xD56`\x04a&\x82V[a\n\x18V[a\x02\xA7a\x010T\x81V[a\x02fa\x04\xF26`\x04a&\xDBV[a\n\x95V[a\x02fa\x05\x056`\x04a&\xDBV[a\x0B\x0FV[a\x02fa\x05\x186`\x04a&\x82V[a\x013` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xC8a\x05;6`\x04a'\x03V[a\x0B\x1CV[a\x02\xC8a\x05N6`\x04a(3V[a\x0B.V[a\x02\xC8a\x05a6`\x04a&\x82V[a\x0CcV[a\x02\xC8a\x05t6`\x04a(\x87V[a\x0E\xA5V[a\x02\xA7a\x05\x876`\x04a(\xEDV[a\x10\x06V[a\x02\xC8a\x100V[a\x05\xA7a\x05\xA26`\x04a)\x1EV[a\x10\xFEV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xE0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02rV[a\x02\xC8a\x05\xDF6`\x04a&\x82V[a\x11\x7FV[a\x03\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[```h\x80Ta\x06\x1A\x90a)PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06F\x90a)PV[\x80\x15a\x06\x91W\x80`\x1F\x10a\x06hWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x91V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_3a\x06\xA8\x81\x85\x85a\x11\xF5V[`\x01\x91PP[\x92\x91PPV[a\x06\xBCa\x13\x18V[a\x06\xC6\x82\x82a\x13rV[PPV[_3a\x06\xD7\x85\x82\x85a\x13\xD2V[a\x06\xE2\x85\x85\x85a\x14JV[P`\x01\x94\x93PPPPV[_a\x06\xF6a\x16\x04V[\x90P\x90V[_3a\x06\xA8\x81\x85\x85a\x07\r\x83\x83a\x10\x06V[a\x07\x17\x91\x90a)\x96V[a\x11\xF5V[_a\x07%a\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\x07|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x90 a\x07\x9D\x90\x83a\x16\rV[\x93\x92PPPV[3_\x90\x81Ra\x013` R`@\x90 T`\xFF\x16a\x08\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBackingEigen.mint: caller is not`D\x82\x01Rh\x100\x906\xB4\xB7:2\xB9`\xB9\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x06\xC6\x82\x82a\x16\xEEV[a\x08)3\x82a\x17yV[PV[a\x08)3\x82a\x17\x92V[a\x08>a\x13\x18V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x01$\xB1%\x03\xBD\xDC&\x16\xC0\xF3\xF5O\xD2>\xD2\x83\xF5\xEF\x0C\x14\x83\xA7T\t\xE4&\x12\x17k\x8B\xDE\x82`@Qa\x08{\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16_\x90\x81Ra\x013` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xFF` R`@\x81 Ta\x06\xAE\x90a\x18\x0BV[a\x08\xD7a\x13\x18V[a\x08\xE0_a\x18sV[V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xCB` R`@\x81 Ta\x06\xAEV[_``\x80___```\x97T__\x1B\x14\x80\x15a\t\x1BWP`\x98T\x15[a\t_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07sV[a\tga\x18\xC4V[a\toa\x18\xD3V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_a\t\xA1a\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x10a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x04U$3#\x05f\xF7FW3\xA2\x06gWGW&R\x06\xC6\xF6\xF6\xB7W`<\x1B`D\x82\x01R`d\x01a\x07sV[a\x06\xAEa\x01\0\x83a\x16\rV[_a\x06\xF6Ba\x18\xE2V[```i\x80Ta\x06\x1A\x90a)PV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xFF` R`@\x81 T\x80\x15a\n\x83W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x90 \x80T_\x19\x83\x01\x90\x81\x10a\nbWa\nba)\xBDV[_\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\n\x85V[_[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[_3\x81a\n\xA2\x82\x86a\x10\x06V[\x90P\x83\x81\x10\x15a\x0B\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x06\xE2\x82\x86\x86\x84\x03a\x11\xF5V[_3a\x06\xA8\x81\x85\x85a\x14JV[a\x0B$a\x13\x18V[a\x06\xC6\x82\x82a\x19HV[\x83B\x11\x15a\x0B~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Votes: signature expired\0\0\0`D\x82\x01R`d\x01a\x07sV[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R_\x90a\x0B\xF7\x90a\x0B\xEF\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x19\xA0V[\x85\x85\x85a\x19\xCCV[\x90Pa\x0C\x02\x81a\x19\xF2V[\x86\x14a\x0CPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FERC20Votes: invalid nonce\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07sV[a\x0CZ\x81\x88a\x17\x92V[PPPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0C\x81WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0C\x9AWP0;\x15\x80\x15a\x0C\x9AWP_T`\xFF\x16`\x01\x14[a\x0C\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07sV[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\r\x1EW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\r&a\x1A\x19V[a\rs`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l!0\xB1\xB5\xB4\xB73\x90\"\xB4\xB3\xB2\xB7`\x99\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1AGV[a\r|\x82a\x18sV[a\r\xA3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e1\"\xA4\xA3\xA2\xA7`\xD1\x1B\x81RPa\x1AwV[_\x19a\x010Ua\r\xD4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x13rV[a\r\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01a\x19HV[a\x0E5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0k\x05hhw\xAF\xB5\xCB\xCC\xBFs@\0a\x16\xEEV[`@Q\x7F\xB7\xC2<\x1E.6\xF2\x98\xE9\x87\x9A\x88\xEC\xFC\xD0~(\xFB\xB49\xBC\xFA\x9Cx\xCA\x13c\xCA\x147\r&\x90_\x90\xA1\x80\x15a\x06\xC6W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[\x83B\x11\x15a\x0E\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x07sV[_\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a\x0F#\x8Ca\x19\xF2V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x0F}\x82a\x19\xA0V[\x90P_a\x0F\x8C\x82\x87\x87\x87a\x19\xCCV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x07sV[a\x0F\xFA\x8A\x8A\x8Aa\x11\xF5V[PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`f` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x108a\x13\x18V[_\x19a\x010T\x14a\x10\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FBackingEigen.disableTransferRest`D\x82\x01R\x7Frictions: transfer restrictions `d\x82\x01Rs\x18\\\x99H\x18[\x1C\x99XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`b\x1B`\x84\x82\x01R`\xA4\x01a\x07sV[_a\x010\x81\x90U`@Q\x7F+\x18\x98m;\xA8\t\xDB/\x13\xA5\xD7\xBF\x17\xF6\r5{7\xD9\xCB\xB5]\xD7\x1C\xBB\xAC\x8D\xC4\x06\x0Fd\x91\x90\xA1V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x11@Wa\x11@a)\xBDV[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x11\x87a\x13\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x08)\x81a\x18sV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x12WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`f` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81Ra\x011` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xCF \xB1\xEC\xB6\x04\xB0\xE8\x88\x8DW\x9Cd\xE8\xA3\xB1\x0EY\rE\xC1\xC2\xDD\xDB9;\xED(Cb\"q\x91\x01[`@Q\x80\x91\x03\x90\xA2PPV[_a\x13\xDD\x84\x84a\x10\x06V[\x90P_\x19\x81\x14a\x14DW\x81\x81\x10\x15a\x147W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x07sV[a\x14D\x84\x84\x84\x84\x03a\x11\xF5V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x14\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x15\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x15\x1B\x83\x83\x83a\x1A\xC0V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a\x15\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x81\x81R`e` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x15\xF1\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x14D\x84\x84\x84a\x1B\x9CV[_a\x06\xF6a\x1B\xCDV[\x81T_\x90\x81\x81`\x05\x81\x11\x15a\x16dW_a\x16&\x84a\x1C@V[a\x160\x90\x85a)\xD1V[_\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16TW\x80\x91Pa\x16bV[a\x16_\x81`\x01a)\x96V[\x92P[P[\x80\x82\x10\x15a\x16\xAFW_a\x16w\x83\x83a\x1D$V[_\x88\x81R` \x90 \x90\x91P\x86\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\x9BW\x80\x91Pa\x16\xA9V[a\x16\xA6\x81`\x01a)\x96V[\x92P[Pa\x16dV[\x80\x15a\x16\xD9W_\x86\x81R` \x90 \x81\x01_\x19\x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x16\xDBV[_[`\x01`\x01`\xE0\x1B\x03\x16\x96\x95PPPPPPV[a\x16\xF8\x82\x82a\x1D>V[`gT`\x01`\x01`\xE0\x1B\x03\x10\x15a\x17jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FERC20Votes: total supply risks o`D\x82\x01Roverflowing votes`\x80\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x14Da\x01\0a\x1E\x0F\x83a\x1E\x1AV[a\x17\x83\x82\x82a\x1F\x86V[a\x14Da\x01\0a \xC9\x83a\x1E\x1AV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xFE` \x81\x81R`@\x80\x84 \x80T`e\x84R\x82\x86 T\x94\x90\x93R\x87\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x84\x16\x81\x17\x90\x91U\x90Q\x91\x90\x95\x16\x94\x91\x93\x91\x92\x85\x92\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x14D\x82\x84\x83a \xD4V[_c\xFF\xFF\xFF\xFF\x82\x11\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[P\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[```\x99\x80Ta\x06\x1A\x90a)PV[```\x9A\x80Ta\x06\x1A\x90a)PV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 4`D\x82\x01Re8 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81Ra\x012` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x85\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7Fr\xA5a\xD1\xAFt\tF}\xAEO\x1E\x9F\xC5%\x90\xA93Z\x1D\xDA\x17r~+j\xA8\xC4\xDB5\x10\x9B\x91\x01a\x13\xC6V[_a\x06\xAEa\x19\xACa\x16\x04V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[___a\x19\xDB\x87\x87\x87\x87a\"\x0EV[\x91P\x91Pa\x19\xE8\x81a\"\xCBV[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\xCB` R`@\x90 \x80T`\x01\x81\x01\x82U\x90[P\x91\x90PV[_Ta\x01\0\x90\x04`\xFF\x16a\x1A?W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x08\xE0a$\x14V[_Ta\x01\0\x90\x04`\xFF\x16a\x1AmW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x06\xC6\x82\x82a$CV[_Ta\x01\0\x90\x04`\xFF\x16a\x1A\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x08)\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RPa$\x82V[a\x010TB\x11a\x1B\x97W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81Ra\x011` R`@\x90 T`\xFF\x16\x80a\x1B\tWP`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81Ra\x012` R`@\x90 T`\xFF\x16[\x80a\x1B\x1BWP`\x01`\x01`\xA0\x1B\x03\x83\x16\x15[a\x1B\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FBackingEigen._beforeTokenTransfe`D\x82\x01R\x7Fr: from or to must be whiteliste`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x07sV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xFE` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x1B\x97\x92\x91\x82\x16\x91\x16\x83a \xD4V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1B\xF7a$\xCFV[a\x1B\xFFa%'V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81_\x03a\x1COWP_\x91\x90PV[_`\x01a\x1C[\x84a%WV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x1CtWa\x1Cta*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\x8CWa\x1C\x8Ca*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xA4Wa\x1C\xA4a*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xBCWa\x1C\xBCa*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xD4Wa\x1C\xD4a*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1C\xECWa\x1C\xECa*/V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1D\x04Wa\x1D\x04a*/V[\x04\x82\x01\x90\x1C\x90Pa\x07\x9D\x81\x82\x85\x81a\x1D\x1EWa\x1D\x1Ea*/V[\x04a%\xEAV[_a\x1D2`\x02\x84\x84\x18a*CV[a\x07\x9D\x90\x84\x84\x16a)\x96V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1D\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x07sV[a\x1D\x9F_\x83\x83a\x1A\xC0V[\x80`g_\x82\x82Ta\x1D\xB0\x91\x90a)\x96V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`e` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x06\xC6_\x83\x83a\x1B\x9CV[_a\x07\x9D\x82\x84a)\x96V[\x82T_\x90\x81\x90\x81\x81\x15a\x1EdW_\x87\x81R` \x90 \x82\x01_\x19\x01`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x82Rd\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16` \x82\x01Ra\x1ExV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R[\x90P\x80` \x01Q`\x01`\x01`\xE0\x1B\x03\x16\x93Pa\x1E\x98\x84\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x92P_\x82\x11\x80\x15a\x1E\xC0WPa\x1E\xACa\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x1F\x03Wa\x1E\xCE\x83a%\xFFV[_\x88\x81R` \x90 \x83\x01_\x19\x01\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1F|V[\x86`@Q\x80`@\x01`@R\x80a\x1F'a\x1F\x1Aa\t\xFFV[e\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x18\x0BV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x1F;\x86a%\xFFV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U_\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[PP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1F\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x07sV[a\x1F\xF1\x82_\x83a\x1A\xC0V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`e` R`@\x90 T\x81\x81\x10\x15a dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x07sV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x81\x81R`e` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`g\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3a\x1B\x97\x83_\x84a\x1B\x9CV[_a\x07\x9D\x82\x84a)\xD1V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a \xF5WP_\x81\x11[\x15a\x1B\x97W`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a!\x82W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\xFF` R`@\x81 \x81\x90a!/\x90a \xC9\x85a\x1E\x1AV[\x91P\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!w\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x1B\x97W`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\xFF` R`@\x81 \x81\x90a!\xB7\x90a\x1E\x0F\x85a\x1E\x1AV[\x91P\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"CWP_\x90P`\x03a\"\xC2V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\"\x94W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\"\xBCW_`\x01\x92P\x92PPa\"\xC2V[\x91P_\x90P[\x94P\x94\x92PPPV[_\x81`\x04\x81\x11\x15a\"\xDEWa\"\xDEa*bV[\x03a\"\xE6WPV[`\x01\x81`\x04\x81\x11\x15a\"\xFAWa\"\xFAa*bV[\x03a#GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07sV[`\x02\x81`\x04\x81\x11\x15a#[Wa#[a*bV[\x03a#\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x07sV[`\x03\x81`\x04\x81\x11\x15a#\xBCWa#\xBCa*bV[\x03a\x08)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x07sV[_Ta\x01\0\x90\x04`\xFF\x16a$:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[a\x08\xE03a\x18sV[_Ta\x01\0\x90\x04`\xFF\x16a$iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[`ha$u\x83\x82a*\xC1V[P`ia\x1B\x97\x82\x82a*\xC1V[_Ta\x01\0\x90\x04`\xFF\x16a$\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07s\x90a)\xE4V[`\x99a$\xB4\x83\x82a*\xC1V[P`\x9Aa$\xC1\x82\x82a*\xC1V[PP_`\x97\x81\x90U`\x98UPV[__a$\xD9a\x18\xC4V[\x80Q\x90\x91P\x15a$\xF0W\x80Q` \x90\x91\x01 \x91\x90PV[`\x97T\x80\x15a$\xFFW\x92\x91PPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x92PPP\x90V[__a%1a\x18\xD3V[\x80Q\x90\x91P\x15a%HW\x80Q` \x90\x91\x01 \x91\x90PV[`\x98T\x80\x15a$\xFFW\x92\x91PPV[_\x80`\x80\x83\x90\x1C\x15a%kW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a%}W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a%\x8FW` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a%\xA1W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a%\xB3W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a%\xC5W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a%\xD7W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x06\xAEW`\x01\x01\x92\x91PPV[_\x81\x83\x10a%\xF8W\x81a\x07\x9DV[P\x90\x91\x90PV[_`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x07sV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a&}W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a&\x92W__\xFD[a\x07\x9D\x82a&gV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\x9D` \x83\x01\x84a&\x9BV[__`@\x83\x85\x03\x12\x15a&\xECW__\xFD[a&\xF5\x83a&gV[\x94` \x93\x90\x93\x015\x93PPPV[__`@\x83\x85\x03\x12\x15a'\x14W__\xFD[a'\x1D\x83a&gV[\x91P` \x83\x015\x80\x15\x15\x81\x14a'1W__\xFD[\x80\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a'NW__\xFD[a'W\x84a&gV[\x92Pa'e` \x85\x01a&gV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a'\x86W__\xFD[P5\x91\x90PV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a'\xAB`\xE0\x83\x01\x89a&\x9BV[\x82\x81\x03`@\x84\x01Ra'\xBD\x81\x89a&\x9BV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a(\x12W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a'\xF4V[P\x90\x9B\x9APPPPPPPPPPPV[\x805`\xFF\x81\x16\x81\x14a&}W__\xFD[______`\xC0\x87\x89\x03\x12\x15a(HW__\xFD[a(Q\x87a&gV[\x95P` \x87\x015\x94P`@\x87\x015\x93Pa(m``\x88\x01a(#V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_______`\xE0\x88\x8A\x03\x12\x15a(\x9DW__\xFD[a(\xA6\x88a&gV[\x96Pa(\xB4` \x89\x01a&gV[\x95P`@\x88\x015\x94P``\x88\x015\x93Pa(\xD0`\x80\x89\x01a(#V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a(\xFEW__\xFD[a)\x07\x83a&gV[\x91Pa)\x15` \x84\x01a&gV[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a)/W__\xFD[a)8\x83a&gV[\x91P` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'1W__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a)dW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1A\x13WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xAEWa\x06\xAEa)\x82V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x06\xAEWa\x06\xAEa)\x82V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a*]WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x1B\x97W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a*\x9BWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a*\xBAW_\x81U`\x01\x01a*\xA7V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xDBWa*\xDBa)\xA9V[a*\xEF\x81a*\xE9\x84Ta)PV[\x84a*vV[` `\x1F\x82\x11`\x01\x81\x14a+!W_\x83\x15a+\nWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua*\xBAV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a+PW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a+0V[P\x84\x82\x10\x15a+mW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \xBCe\xAA\"\xC8.\xA1\x98o\xEF\x94\x1C\x8F\xFC\xA5\xD6\xF9\x99\x1E?\xB7[.\xEF\xFFS\xE2e\x01\xE4_\x90dsolcC\0\x08\x1B\x003\xA2dipfsX\"\x12 \xA8\xAD\xC4\xEE\xE0X\x97\x80\x04\r\xAA)\xE7\xE0\x1F]nK\x8E\x17\xEA\xDA7\xD0\xBF\xBD\xAF\"\x82\x90\xC2\x9EdsolcC\0\x08\x1B\x003",
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8, 214u8,
                    9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8, 202u8, 240u8,
                    233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8, 71u8,
                    177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8, 120u8, 85u8,
                    214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                    181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8, 92u8,
                    141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.val
                ),)
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::I256>,
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
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8, 237u8,
                    155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8, 163u8, 100u8,
                    124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Int<256>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.val
                ),)
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
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8, 45u8,
                    155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8, 159u8,
                    241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.val
                ),)
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8, 86u8,
                    3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8, 86u8, 225u8,
                    26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8, 88u8,
                    139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8, 115u8, 175u8,
                    212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8, 90u8,
                    140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8, 27u8, 113u8,
                    181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
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
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8, 16u8,
                    143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8, 67u8,
                    122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8, 56u8,
                    12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8, 127u8, 201u8,
                    83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::I256>,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8, 70u8,
                    17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8, 247u8, 225u8,
                    123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8, 207u8,
                    39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8, 228u8, 112u8,
                    223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8, 217u8,
                    79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8, 89u8, 79u8,
                    213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8, 111u8,
                    146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8, 221u8, 184u8,
                    211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8, 89u8,
                    239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8, 10u8,
                    232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.decimals,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8, 67u8,
                    232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8, 128u8, 28u8,
                    19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.decimals,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8, 142u8,
                    151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8, 216u8, 31u8,
                    126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8, 101u8,
                    141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8, 243u8, 120u8,
                    83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                    253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8, 197u8,
                    108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8, 85u8,
                    131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8, 50u8,
                    156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8, 82u8,
                    136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8, 239u8, 197u8,
                    66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8, 27u8,
                    245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8, 67u8, 8u8,
                    201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<EIGEN_addressBeforeCall> for UnderlyingRustTuple<'_> {
                fn from(value: EIGEN_addressBeforeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIGEN_addressBeforeCall {
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
            impl ::core::convert::From<EIGEN_addressBeforeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: EIGEN_addressBeforeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for EIGEN_addressBeforeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EIGEN_addressBeforeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = EIGEN_addressBeforeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_SCRIPTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bEIGEN_ProxyAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_ProxyAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_ProxyAdminCall {
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
            impl ::core::convert::From<bEIGEN_ProxyAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_ProxyAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_ProxyAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_ProxyAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_ProxyAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bEIGEN_TimelockAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockAdminCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_TimelockAdminCall {
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
            impl ::core::convert::From<bEIGEN_TimelockAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_TimelockAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_TimelockAdminCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_TimelockAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bEIGEN_TimelockControllerCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_TimelockControllerCall {
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
            impl ::core::convert::From<bEIGEN_TimelockControllerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_TimelockControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_TimelockControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_TimelockControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_TimelockControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bEIGEN_implementationCall> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_implementationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_implementationCall {
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
            impl ::core::convert::From<bEIGEN_implementationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bEIGEN_implementationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bEIGEN_implementationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bEIGEN_implementationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_implementationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bEIGEN_proxyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkUpgradeCorrectnessCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkUpgradeCorrectnessCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkUpgradeCorrectnessCall {
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
            impl ::core::convert::From<checkUpgradeCorrectnessReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkUpgradeCorrectnessReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkUpgradeCorrectnessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkUpgradeCorrectnessCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkUpgradeCorrectnessReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub excludedArtifacts_: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::String>,);
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
            impl ::core::convert::From<excludeArtifactsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeArtifactsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub excludedContracts_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
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
            impl ::core::convert::From<excludeContractsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeContractsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub excludedSenders_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
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
            impl ::core::convert::From<excludeSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSenders_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = runReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<simulatePerformingUpgradeCall> for UnderlyingRustTuple<'_> {
                fn from(value: simulatePerformingUpgradeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for simulatePerformingUpgradeCall {
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
            impl ::core::convert::From<simulatePerformingUpgradeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: simulatePerformingUpgradeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for simulatePerformingUpgradeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for simulatePerformingUpgradeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = simulatePerformingUpgradeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactSelectorsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub targetedArtifacts_: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::String>,);
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
            impl ::core::convert::From<targetArtifactsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub targetedContracts_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
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
            impl ::core::convert::From<targetContractsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub targetedSenders_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSenders_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                Self::IS_SCRIPT(_) => <IS_SCRIPTCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::bEIGEN_proxy(_) => <bEIGEN_proxyCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::targetSenders(_) => <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR,
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
                -> alloy_sol_types::Result<bEIGEN_upgradeCalls>] = &[
                {
                    fn checkUpgradeCorrectness(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(bEIGEN_upgradeCalls::failed)
                    }
                    failed
                },
                {
                    fn run(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <runCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(bEIGEN_upgradeCalls::IS_SCRIPT)
                    }
                    IS_SCRIPT
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<bEIGEN_upgradeCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(bEIGEN_upgradeCalls::IS_TEST)
                    }
                    IS_TEST
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
                Self::EIGEN_addressBefore(inner) => {
                    <EIGEN_addressBeforeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGEN_ProxyAdmin(inner) => {
                    <bEIGEN_ProxyAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGEN_TimelockAdmin(inner) => {
                    <bEIGEN_TimelockAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGEN_TimelockController(inner) => {
                    <bEIGEN_TimelockControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bEIGEN_implementation(inner) => {
                    <bEIGEN_implementationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bEIGEN_proxy(inner) => {
                    <bEIGEN_proxyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkUpgradeCorrectness(inner) => {
                    <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::run(inner) => <runCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
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
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EIGEN_addressBefore(inner) => {
                    <EIGEN_addressBeforeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bEIGEN_ProxyAdmin(inner) => {
                    <bEIGEN_ProxyAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bEIGEN_TimelockAdmin(inner) => {
                    <bEIGEN_TimelockAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::bEIGEN_TimelockController(inner) => {
                    <bEIGEN_TimelockControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::bEIGEN_implementation(inner) => {
                    <bEIGEN_implementationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::bEIGEN_proxy(inner) => {
                    <bEIGEN_proxyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkUpgradeCorrectness(inner) => {
                    <checkUpgradeCorrectnessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::run(inner) => {
                    <runCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::simulatePerformingUpgrade(inner) => {
                    <simulatePerformingUpgradeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8, 56u8, 12u8,
                115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8, 127u8, 201u8, 83u8,
                40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8, 85u8, 131u8,
                237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8, 50u8, 156u8, 79u8,
                187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8, 90u8,
                140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8, 27u8, 113u8,
                181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8, 86u8, 3u8,
                145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8, 86u8, 225u8, 26u8,
                162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8, 101u8,
                141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8, 243u8, 120u8,
                83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8, 82u8,
                136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8, 239u8, 197u8, 66u8,
                124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8, 142u8,
                151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8, 216u8, 31u8, 126u8,
                142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8, 207u8,
                39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8, 228u8, 112u8, 223u8,
                59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8, 45u8,
                155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8, 159u8, 241u8,
                3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8, 214u8, 9u8,
                203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8, 202u8, 240u8, 233u8,
                177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8, 89u8, 239u8,
                36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8, 10u8, 232u8, 67u8,
                78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8, 71u8,
                177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8, 120u8, 85u8, 214u8,
                126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8, 237u8,
                155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8, 163u8, 100u8, 124u8,
                33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8, 16u8,
                143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8, 67u8, 122u8,
                97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8, 70u8, 17u8,
                56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8, 247u8, 225u8, 123u8,
                4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8, 111u8,
                146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8, 221u8, 184u8,
                211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8, 253u8,
                68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8, 197u8, 108u8,
                129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8, 217u8,
                79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8, 89u8, 79u8, 213u8,
                99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8, 27u8,
                245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8, 67u8, 8u8,
                201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8, 88u8,
                139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8, 115u8, 175u8, 212u8,
                63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8, 67u8,
                232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8, 128u8, 28u8,
                19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8, 181u8,
                170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8, 92u8, 141u8, 4u8,
                113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log_int)
                }
                Some(<log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_address)
                }
                Some(<log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_array_0)
                }
                Some(<log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_array_1)
                }
                Some(<log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_bytes)
                }
                Some(<log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_bytes32)
                }
                Some(<log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_decimal_int)
                }
                Some(<log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for bEIGEN_upgradeEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::log_bytes(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::log_uint(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::logs(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
                Self::log_int(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
                Self::logs(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<bEIGEN_upgradeInstance<T, P, N>>>
    {
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
    >(
        provider: P,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
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
            f.debug_tuple("bEIGEN_upgradeInstance")
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
        > bEIGEN_upgradeInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`bEIGEN_upgrade`](self) contract instance.

        See the [wrapper's documentation](`bEIGEN_upgradeInstance`) for more details.*/
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
        > bEIGEN_upgradeInstance<T, P, N>
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
        ///Creates a new call builder for the [`EIGEN_addressBefore`] function.
        pub fn EIGEN_addressBefore(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, EIGEN_addressBeforeCall, N> {
            self.call_builder(&EIGEN_addressBeforeCall {})
        }
        ///Creates a new call builder for the [`IS_SCRIPT`] function.
        pub fn IS_SCRIPT(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_SCRIPTCall, N> {
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
        pub fn bEIGEN_proxy(&self) -> alloy_contract::SolCallBuilder<T, &P, bEIGEN_proxyCall, N> {
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
        pub fn targetSenders(&self) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > bEIGEN_upgradeInstance<T, P, N>
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
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(&self) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(&self) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(&self) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(&self) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(&self) -> alloy_contract::Event<T, &P, log_bytes32, N> {
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
        pub fn log_named_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
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
        pub fn log_named_int_filter(&self) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(&self) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(&self) -> alloy_contract::Event<T, &P, log_named_uint, N> {
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
