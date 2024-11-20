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

interface ProofParsing {
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

    function IS_TEST() external view returns (bool);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external returns (bool);
    function getBalanceUpdateProof() external returns (bytes memory);
    function getBeaconStateRoot() external returns (bytes32);
    function getBlockHeaderProof() external returns (bytes32[18] memory);
    function getBlockRoot() external returns (bytes32);
    function getBlockRootIndex() external returns (uint256);
    function getExecutionPayloadProof() external returns (bytes32[7] memory);
    function getExecutionPayloadRoot() external returns (bytes32);
    function getHistoricalSummaryIndex() external returns (uint256);
    function getHistoricalSummaryProof() external returns (bytes32[44] memory);
    function getLatestBlockRoot() external returns (bytes32);
    function getSlot() external returns (uint256);
    function getSlotProof() external returns (bytes32[3] memory);
    function getSlotRoot() external returns (bytes32);
    function getStateRootProof() external returns (bytes memory);
    function getTimestampProofCapella() external returns (bytes32[4] memory);
    function getTimestampProofDeneb() external returns (bytes32[5] memory);
    function getTimestampRoot() external returns (bytes32);
    function getValidatorFields() external returns (bytes32[] memory);
    function getValidatorFieldsProof() external returns (bytes32[] memory);
    function getValidatorIndex() external returns (uint256);
    function getValidatorProof() external returns (bytes32[46] memory);
    function getValidatorPubkeyHash() external returns (bytes32);
    function getWithdrawalCredentialProof() external returns (bytes memory);
    function getWithdrawalFields() external returns (bytes32[] memory);
    function getWithdrawalIndex() external returns (uint256);
    function getWithdrawalProofCapella() external returns (bytes32[9] memory);
    function getWithdrawalProofDeneb() external returns (bytes32[10] memory);
    function setJSON(string memory path) external;
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
    "name": "getBalanceUpdateProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBeaconStateRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBlockHeaderProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[18]",
        "internalType": "bytes32[18]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBlockRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBlockRootIndex",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getExecutionPayloadProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[7]",
        "internalType": "bytes32[7]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getExecutionPayloadRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getHistoricalSummaryIndex",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getHistoricalSummaryProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[44]",
        "internalType": "bytes32[44]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLatestBlockRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getSlot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getSlotProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[3]",
        "internalType": "bytes32[3]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getSlotRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getStateRootProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTimestampProofCapella",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[4]",
        "internalType": "bytes32[4]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTimestampProofDeneb",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[5]",
        "internalType": "bytes32[5]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTimestampRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorFields",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorFieldsProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorIndex",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[46]",
        "internalType": "bytes32[46]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorPubkeyHash",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalCredentialProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalFields",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalIndex",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalProofCapella",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[9]",
        "internalType": "bytes32[9]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalProofDeneb",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[10]",
        "internalType": "bytes32[10]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setJSON",
    "inputs": [
      {
        "name": "path",
        "type": "string",
        "internalType": "string"
      }
    ],
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
pub mod ProofParsing {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405260008054600160ff199182168117909255600480549091169091179055348015602c57600080fd5b5061319f8061003c6000396000f3fe608060405234801561001057600080fd5b50600436106102325760003560e01c8063765aa60611610130578063b38061bf116100b8578063db364a401161007c578063db364a401461040b578063df18c78314610420578063e20c9f7114610435578063f148082c1461043d578063fa7626d41461044557600080fd5b8063b38061bf146103d3578063b5508aa9146103db578063ba414fa6146103e3578063d6461cbb146103fb578063d944472f1461040357600080fd5b8063950bb682116100ff578063950bb682146103845780639acfac291461038c5780639de4a9b3146103a1578063a157ef4c146103b6578063a5077429146103be57600080fd5b8063765aa6061461035757806385226c811461035f578063893893ca14610374578063916a17c61461037c57600080fd5b806334e3d90e116101be5780634c38f913116101825780634c38f9131461030857806364c725351461031d57806364f38ed71461033257806366d9a9a01461033a5780636c877c841461034f57600080fd5b806334e3d90e146102d35780633e5e3c23146102db5780633f7286f4146102e357806342864734146102eb5780634c20f510146102f357600080fd5b8063196cdfbd11610205578063196cdfbd146102775780631ed7831c1461028c578063275378b1146102a15780632872e20c146102a95780632fd1793c146102be57600080fd5b806308a4d71f1461023757806316f071531461024c578063177541fc1461026757806318aadf311461026f575b600080fd5b61024a6102453660046127bd565b610452565b005b6102546104d9565b6040519081526020015b60405180910390f35b610254610595565b610254610653565b61027f610717565b60405161025e919061283d565b610294610958565b60405161025e919061286e565b6102546109b9565b6102b1610a77565b60405161025e91906128ba565b6102c6610ba5565b60405161025e9190612933565b6102c6610cf3565b610294610e17565b610294610e77565b610254610ed7565b6102fb610f9a565b60405161025e9190612946565b6103106110c7565b60405161025e919061297e565b6103256111f5565b60405161025e91906129a7565b610254611323565b6103426113e1565b60405161025e91906129d0565b6102546114d0565b610254611583565b610367611640565b60405161025e9190612a8a565b6102fb611710565b610342611837565b6102fb61191d565b610394611a42565b60405161025e9190612ae3565b6103a9611b70565b60405161025e9190612b0c565b6102c6611c9d565b6103c6611ca7565b60405161025e9190612b34565b610254611dd5565b610367611e91565b6103eb611f61565b604051901515815260200161025e565b610254612080565b61025461214b565b61041361220e565b60405161025e9190612b5d565b61042861233b565b60405161025e9190612b85565b610294612468565b6102546124c8565b6000546103eb9060ff1681565b6040516360f9bb1160e01b815260008051602061314a833981519152906360f9bb1190610483908490600401612933565b600060405180830381865afa1580156104a0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104c89190810190612bad565b601b906104d59082612ca7565b5050565b6000610590601b80546104eb90612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461051790612c24565b80156105645780601f1061053957610100808354040283529160200191610564565b820191906000526020600020905b81548152906001019060200180831161054757829003601f168201915b5050505050604051806040016040528060098152602001680b9cdb1bdd149bdbdd60ba1b815250612585565b905090565b6000610590601b80546105a790612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546105d390612c24565b80156106205780601f106105f557610100808354040283529160200191610620565b820191906000526020600020905b81548152906001019060200180831161060357829003601f168201915b50505050506040518060400160405280601081526020016f0b98995858dbdb94dd185d19549bdbdd60821b815250612585565b6000610590601b805461066590612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461069190612c24565b80156106de5780601f106106b3576101008083540402835291602001916106de565b820191906000526020600020905b8154815290600101906020018083116106c157829003601f168201915b5050505050604051806040016040528060168152602001750b9b185d195cdd109b1bd8dad2195859195c949bdbdd60521b815250612585565b61071f61263b565b60005b60058110156109235760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610775573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261079d9190810190612bad565b6040516020016107ad9190612d66565b60408051601f19818403018152908290526107ca91602001612d8b565b604051602081830303815290604052601c90816107e79190612ca7565b50610906601b80546107f890612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461082490612c24565b80156108715780601f1061084657610100808354040283529160200191610871565b820191906000526020600020905b81548152906001019060200180831161085457829003601f168201915b5050505050601c805461088390612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546108af90612c24565b80156108fc5780601f106108d1576101008083540402835291602001916108fc565b820191906000526020600020905b8154815290600101906020018083116108df57829003601f168201915b5050505050612585565b60a6826005811061091957610919612dc3565b0155600101610722565b506040805160a08101918290529060a69060059082845b81548152602001906001019080831161093a575b5050505050905090565b6060600d80548060200260200160405190810160405280929190818152602001828054801561094e57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610992575050505050905090565b6000610590601b80546109cb90612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546109f790612c24565b8015610a445780601f10610a1957610100808354040283529160200191610a44565b820191906000526020600020905b815481529060010190602001808311610a2757829003601f168201915b50505050506040518060400160405280601081526020016f0b989b1bd8dad2195859195c949bdbdd60821b815250612585565b610a7f612659565b60005b602c811015610b755760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610ad5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610afd9190810190612bad565b604051602001610b0d9190612d66565b60408051601f1981840301815290829052610b2a91602001612dd9565b604051602081830303815290604052601c9081610b479190612ca7565b50610b58601b80546107f890612c24565b607382602c8110610b6b57610b6b612dc3565b0155600101610a82565b50604080516105808101918290526073805482529091602c9060746020850180831161093a575050505050905090565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e811015610ccc5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610c24573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c4c9190810190612bad565b604051602001610c5c9190612d66565b60408051601f1981840301815290829052610c7991602001612e1e565b604051602081830303815290604052601c9081610c969190612ca7565b50610ca7601b80546107f890612c24565b828281518110610cb957610cb9612dc3565b6020908102919091010152600101610bd1565b5080604051602001610cde9190612e63565b60405160208183030381529060405291505090565b604080516003808252608082019092526060916000919060208201848036833701905050905060005b6003811015610ccc5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610d6f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d979190810190612bad565b604051602001610da79190612d66565b60408051601f1981840301815290829052610dc491602001612e99565b604051602081830303815290604052601c9081610de19190612ca7565b50610df2601b80546107f890612c24565b828281518110610e0457610e04612dc3565b6020908102919091010152600101610d1c565b6060600f80548060200260200160405190810160405280929190818152602001828054801561094e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610992575050505050905090565b6060600e80548060200260200160405190810160405280929190818152602001828054801561094e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610992575050505050905090565b6000610590601b8054610ee990612c24565b80601f0160208091040260200160405190810160405280929190818152602001828054610f1590612c24565b8015610f625780601f10610f3757610100808354040283529160200191610f62565b820191906000526020600020905b815481529060010190602001808311610f4557829003601f168201915b50505050506040518060400160405280601581526020017405cc4d8dec6d690cac2c8cae4a4dedee892dcc8caf605b1b815250612605565b604080516008808252610120820190925260609160009190602082016101008036833701905050905060005b60088110156110c15760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611019573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110419190810190612bad565b6040516020016110519190612d66565b60408051601f198184030181529082905261106e91602001612eef565b604051602081830303815290604052601c908161108b9190612ca7565b5061109c601b80546107f890612c24565b8282815181106110ae576110ae612dc3565b6020908102919091010152600101610fc6565b50919050565b6110cf612678565b60005b602e8110156111c55760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611125573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261114d9190810190612bad565b60405160200161115d9190612d66565b60408051601f198184030181529082905261117a91602001612f28565b604051602081830303815290604052601c90816111979190612ca7565b506111a8601b80546107f890612c24565b604582602e81106111bb576111bb612dc3565b01556001016110d2565b50604080516105c08101918290526045805482529091602e9060466020850180831161093a575050505050905090565b6111fd612697565b60005b60098110156112f35760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611253573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261127b9190810190612bad565b60405160200161128b9190612d66565b60408051601f19818403018152908290526112a891602001612f53565b604051602081830303815290604052601c90816112c59190612ca7565b506112d6601b80546107f890612c24565b603c82600981106112e9576112e9612dc3565b0155600101611200565b5060408051610120810191829052603c805482529091600990603d6020850180831161093a575050505050905090565b6000610590601b805461133590612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461136190612c24565b80156113ae5780601f10611383576101008083540402835291602001916113ae565b820191906000526020600020905b81548152906001019060200180831161139157829003601f168201915b50505050506040518060400160405280601081526020016f05ceed2e8d0c8e4c2eec2d892dcc8caf60831b815250612605565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156114c75760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156114af57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114715790505b50505050508152505081526020019060010190611405565b50505050905090565b6000610590601b80546114e290612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461150e90612c24565b801561155b5780601f106115305761010080835404028352916020019161155b565b820191906000526020600020905b81548152906001019060200180831161153e57829003601f168201915b5050505050604051806040016040528060058152602001640b9cdb1bdd60da1b815250612605565b6000610590601b805461159590612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546115c190612c24565b801561160e5780601f106115e35761010080835404028352916020019161160e565b820191906000526020600020905b8154815290600101906020018083116115f157829003601f168201915b50505050506040518060400160405280600f81526020016e05cecc2d8d2c8c2e8dee492dcc8caf608b1b815250612605565b60606011805480602002602001604051908101604052809291908181526020016000905b828210156114c757838290600052602060002001805461168390612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546116af90612c24565b80156116fc5780601f106116d1576101008083540402835291602001916116fc565b820191906000526020600020905b8154815290600101906020018083116116df57829003601f168201915b505050505081526020019060010190611664565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e8110156110c15760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa15801561178f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117b79190810190612bad565b6040516020016117c79190612d66565b60408051601f19818403018152908290526117e491602001612f7f565b604051602081830303815290604052601c90816118019190612ca7565b50611812601b80546107f890612c24565b82828151811061182457611824612dc3565b602090810291909101015260010161173c565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156114c75760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561190557602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116118c75790505b5050505050815250508152602001906001019061185b565b60408051600480825260a08201909252606091600091906020820160808036833701905050905060005b60048110156110c15760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa15801561199a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119c29190810190612bad565b6040516020016119d29190612d66565b60408051601f19818403018152908290526119ef91602001612fbd565b604051602081830303815290604052601c9081611a0c9190612ca7565b50611a1d601b80546107f890612c24565b828281518110611a2f57611a2f612dc3565b6020908102919091010152600101611947565b611a4a6126b6565b60005b600a811015611b405760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611aa0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611ac89190810190612bad565b604051602001611ad89190612d66565b60408051601f1981840301815290829052611af591602001612f53565b604051602081830303815290604052601c9081611b129190612ca7565b50611b23601b80546107f890612c24565b603282600a8110611b3657611b36612dc3565b0155600101611a4d565b50604080516101408101918290526032805482529091600a9060336020850180831161093a575050505050905090565b611b786126d5565b60005b6003811015611c6e5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611bce573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611bf69190810190612bad565b604051602001611c069190612d66565b60408051601f1981840301815290829052611c2391602001612ff7565b604051602081830303815290604052601c9081611c409190612ca7565b50611c51601b80546107f890612c24565b602f8260038110611c6457611c64612dc3565b0155600101611b7b565b50604080516060810191829052602f80548252909160039060306020850180831161093a575050505050905090565b6060610590610ba5565b611caf6126f3565b60005b6012811015611da55760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611d05573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d2d9190810190612bad565b604051602001611d3d9190612d66565b60408051601f1981840301815290829052611d5a9160200161302a565b604051602081830303815290604052601c9081611d779190612ca7565b50611d88601b80546107f890612c24565b601d8260128110611d9b57611d9b612dc3565b0155600101611cb2565b5060408051610240810191829052601d805482529091601290601e6020850180831161093a575050505050905090565b6000610590601b8054611de790612c24565b80601f0160208091040260200160405190810160405280929190818152602001828054611e1390612c24565b8015611e605780601f10611e3557610100808354040283529160200191611e60565b820191906000526020600020905b815481529060010190602001808311611e4357829003601f168201915b50505050506040518060400160405280600e81526020016d0b9d1a5b595cdd185b5c149bdbdd60921b815250612585565b60606010805480602002602001604051908101604052809291908181526020016000905b828210156114c7578382906000526020600020018054611ed490612c24565b80601f0160208091040260200160405190810160405280929190818152602001828054611f0090612c24565b8015611f4d5780601f10611f2257610100808354040283529160200191611f4d565b820191906000526020600020905b815481529060010190602001808311611f3057829003601f168201915b505050505081526020019060010190611eb5565b60008054610100900460ff1615611f815750600054610100900460ff1690565b600060008051602061314a8339815191523b1561207b576040805160008051602061314a833981519152602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091612003917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001613057565b60408051601f198184030181529082905261201d91613088565b6000604051808303816000865af19150503d806000811461205a576040519150601f19603f3d011682016040523d82523d6000602084013e61205f565b606091505b509150508080602001905181019061207791906130a4565b9150505b919050565b6000610590601b805461209290612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546120be90612c24565b801561210b5780601f106120e05761010080835404028352916020019161210b565b820191906000526020600020905b8154815290600101906020018083116120ee57829003601f168201915b50505050506040518060400160405280601781526020017f2e686973746f726963616c53756d6d617279496e646578000000000000000000815250612605565b6000610590601b805461215d90612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461218990612c24565b80156121d65780601f106121ab576101008083540402835291602001916121d6565b820191906000526020600020905b8154815290600101906020018083116121b957829003601f168201915b5050505050604051806040016040528060158152602001740b995e1958dd5d1a5bdb94185e5b1bd859149bdbdd605a1b815250612585565b612216612712565b60005b600781101561230c5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa15801561226c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526122949190810190612bad565b6040516020016122a49190612d66565b60408051601f19818403018152908290526122c1916020016130c6565b604051602081830303815290604052601c90816122de9190612ca7565b506122ef601b80546107f890612c24565b609f826007811061230257612302612dc3565b0155600101612219565b506040805160e0810191829052609f80548252909160079060a06020850180831161093a575050505050905090565b612343612730565b60005b60048110156124395760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015612399573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526123c19190810190612bad565b6040516020016123d19190612d66565b60408051601f19818403018152908290526123ee91602001612d8b565b604051602081830303815290604052601c908161240b9190612ca7565b5061241c601b80546107f890612c24565b60ab826004811061242f5761242f612dc3565b0155600101612346565b5060408051608081019182905260ab80548252909160049060ac6020850180831161093a575050505050905090565b6060600c80548060200260200160405190810160405280929190818152602001828054801561094e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610992575050505050905090565b6000610590601b80546124da90612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461250690612c24565b80156125535780601f1061252857610100808354040283529160200191612553565b820191906000526020600020905b81548152906001019060200180831161253657829003601f168201915b5050505050604051806040016040528060138152602001722e56616c696461746f724669656c64735b305d60681b8152505b604051631777e59d60e01b815260009060008051602061314a83398151915290631777e59d906125bb908690869060040161310b565b6020604051808303816000875af11580156125da573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125fe9190613130565b9392505050565b6040516356eef15b60e11b815260009060008051602061314a8339815191529063addde2b6906125bb908690869060040161310b565b6040518060a001604052806005906020820280368337509192915050565b604051806105800160405280602c906020820280368337509192915050565b604051806105c00160405280602e906020820280368337509192915050565b6040518061012001604052806009906020820280368337509192915050565b604051806101400160405280600a906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b6040518061024001604052806012906020820280368337509192915050565b6040518060e001604052806007906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561278d5761278d61274e565b604052919050565b600067ffffffffffffffff8211156127af576127af61274e565b50601f01601f191660200190565b6000602082840312156127cf57600080fd5b813567ffffffffffffffff8111156127e657600080fd5b8201601f810184136127f757600080fd5b803561280a61280582612795565b612764565b81815285602083850101111561281f57600080fd5b81602084016020830137600091810160200191909152949350505050565b60a08101818360005b6005811015612865578151835260209283019290910190600101612846565b50505092915050565b602080825282518282018190526000918401906040840190835b818110156128af5783516001600160a01b0316835260209384019390920191600101612888565b509095945050505050565b6105808101818360005b602c8110156128655781518352602092830192909101906001016128c4565b60005b838110156128fe5781810151838201526020016128e6565b50506000910152565b6000815180845261291f8160208601602086016128e3565b601f01601f19169290920160200192915050565b6020815260006125fe6020830184612907565b602080825282518282018190526000918401906040840190835b818110156128af578351835260209384019390920191600101612960565b6105c08101818360005b602e811015612865578151835260209283019290910190600101612988565b6101208101818360005b60098110156128655781518352602092830192909101906001016129b1565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612a7e57868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b80831015612a665783516001600160e01b03191682526020938401936001939093019290910190612a3a565b509650505060209384019391909101906001016129f8565b50929695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612a7e57603f19878603018452612ace858351612907565b94506020938401939190910190600101612ab2565b6101408101818360005b600a811015612865578151835260209283019290910190600101612aed565b60608101818360005b6003811015612865578151835260209283019290910190600101612b15565b6102408101818360005b6012811015612865578151835260209283019290910190600101612b3e565b60e08101818360005b6007811015612865578151835260209283019290910190600101612b66565b60808101818360005b6004811015612865578151835260209283019290910190600101612b8e565b600060208284031215612bbf57600080fd5b815167ffffffffffffffff811115612bd657600080fd5b8201601f81018413612be757600080fd5b8051612bf561280582612795565b818152856020838501011115612c0a57600080fd5b612c1b8260208301602086016128e3565b95945050505050565b600181811c90821680612c3857607f821691505b6020821081036110c157634e487b7160e01b600052602260045260246000fd5b601f821115612ca257806000526020600020601f840160051c81016020851015612c7f5750805b601f840160051c820191505b81811015612c9f5760008155600101612c8b565b50505b505050565b815167ffffffffffffffff811115612cc157612cc161274e565b612cd581612ccf8454612c24565b84612c58565b6020601f821160018114612d095760008315612cf15750848201515b600019600385901b1c1916600184901b178455612c9f565b600084815260208120601f198516915b82811015612d395787850151825560209485019460019092019101612d19565b5084821015612d575786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b60008251612d788184602087016128e3565b605d60f81b920191825250600101919050565b6f2e54696d657374616d7050726f6f665b60801b815260008251612db68160108501602087016128e3565b9190910160100192915050565b634e487b7160e01b600052603260045260246000fd5b7f2e486973746f726963616c53756d6d61727950726f6f665b0000000000000000815260008251612e118160188501602087016128e3565b9190910160180192915050565b7f2e5769746864726177616c43726564656e7469616c50726f6f665b0000000000815260008251612e5681601b8501602087016128e3565b91909101601b0192915050565b8151600090829060208501835b82811015612e8e578151845260209384019390910190600101612e70565b509195945050505050565b7f2e5374617465526f6f74416761696e73744c6174657374426c6f636b48656164815267657250726f6f665b60c01b602082015260008251612ee28160288501602087016128e3565b9190910160280192915050565b702e56616c696461746f724669656c64735b60781b815260008251612f1b8160118501602087016128e3565b9190910160110192915050565b6f2e56616c696461746f7250726f6f665b60801b815260008251612db68160108501602087016128e3565b702e5769746864726177616c50726f6f665b60781b815260008251612f1b8160118501602087016128e3565b752e56616c696461746f724669656c647350726f6f665b60501b815260008251612fb08160168501602087016128e3565b9190910160160192915050565b712e5769746864726177616c4669656c64735b60701b815260008251612fea8160128501602087016128e3565b9190910160120192915050565b6a2e536c6f7450726f6f665b60a81b81526000825161301d81600b8501602087016128e3565b91909101600b0192915050565b712e426c6f636b48656164657250726f6f665b60701b815260008251612fea8160128501602087016128e3565b6001600160e01b031983168152815160009061307a8160048501602087016128e3565b919091016004019392505050565b6000825161309a8184602087016128e3565b9190910192915050565b6000602082840312156130b657600080fd5b815180151581146125fe57600080fd5b7f2e457865637574696f6e5061796c6f616450726f6f665b0000000000000000008152600082516130fe8160178501602087016128e3565b9190910160170192915050565b60408152600061311e6040830185612907565b8281036020840152612c1b8185612907565b60006020828403121561314257600080fd5b505191905056fe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da26469706673582212208412441f099f6f49a1bfd5eaf659c4f38b506498ab9d813291dc4a86fa6b5d3a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15`,W`\0\x80\xFD[Pa1\x9F\x80a\0<`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x022W`\x005`\xE0\x1C\x80cvZ\xA6\x06\x11a\x010W\x80c\xB3\x80a\xBF\x11a\0\xB8W\x80c\xDB6J@\x11a\0|W\x80c\xDB6J@\x14a\x04\x0BW\x80c\xDF\x18\xC7\x83\x14a\x04 W\x80c\xE2\x0C\x9Fq\x14a\x045W\x80c\xF1H\x08,\x14a\x04=W\x80c\xFAv&\xD4\x14a\x04EW`\0\x80\xFD[\x80c\xB3\x80a\xBF\x14a\x03\xD3W\x80c\xB5P\x8A\xA9\x14a\x03\xDBW\x80c\xBAAO\xA6\x14a\x03\xE3W\x80c\xD6F\x1C\xBB\x14a\x03\xFBW\x80c\xD9DG/\x14a\x04\x03W`\0\x80\xFD[\x80c\x95\x0B\xB6\x82\x11a\0\xFFW\x80c\x95\x0B\xB6\x82\x14a\x03\x84W\x80c\x9A\xCF\xAC)\x14a\x03\x8CW\x80c\x9D\xE4\xA9\xB3\x14a\x03\xA1W\x80c\xA1W\xEFL\x14a\x03\xB6W\x80c\xA5\x07t)\x14a\x03\xBEW`\0\x80\xFD[\x80cvZ\xA6\x06\x14a\x03WW\x80c\x85\"l\x81\x14a\x03_W\x80c\x898\x93\xCA\x14a\x03tW\x80c\x91j\x17\xC6\x14a\x03|W`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x11a\x01\xBEW\x80cL8\xF9\x13\x11a\x01\x82W\x80cL8\xF9\x13\x14a\x03\x08W\x80cd\xC7%5\x14a\x03\x1DW\x80cd\xF3\x8E\xD7\x14a\x032W\x80cf\xD9\xA9\xA0\x14a\x03:W\x80cl\x87|\x84\x14a\x03OW`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x14a\x02\xD3W\x80c>^<#\x14a\x02\xDBW\x80c?r\x86\xF4\x14a\x02\xE3W\x80cB\x86G4\x14a\x02\xEBW\x80cL \xF5\x10\x14a\x02\xF3W`\0\x80\xFD[\x80c\x19l\xDF\xBD\x11a\x02\x05W\x80c\x19l\xDF\xBD\x14a\x02wW\x80c\x1E\xD7\x83\x1C\x14a\x02\x8CW\x80c'Sx\xB1\x14a\x02\xA1W\x80c(r\xE2\x0C\x14a\x02\xA9W\x80c/\xD1y<\x14a\x02\xBEW`\0\x80\xFD[\x80c\x08\xA4\xD7\x1F\x14a\x027W\x80c\x16\xF0qS\x14a\x02LW\x80c\x17uA\xFC\x14a\x02gW\x80c\x18\xAA\xDF1\x14a\x02oW[`\0\x80\xFD[a\x02Ja\x02E6`\x04a'\xBDV[a\x04RV[\0[a\x02Ta\x04\xD9V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02Ta\x05\x95V[a\x02Ta\x06SV[a\x02\x7Fa\x07\x17V[`@Qa\x02^\x91\x90a(=V[a\x02\x94a\tXV[`@Qa\x02^\x91\x90a(nV[a\x02Ta\t\xB9V[a\x02\xB1a\nwV[`@Qa\x02^\x91\x90a(\xBAV[a\x02\xC6a\x0B\xA5V[`@Qa\x02^\x91\x90a)3V[a\x02\xC6a\x0C\xF3V[a\x02\x94a\x0E\x17V[a\x02\x94a\x0EwV[a\x02Ta\x0E\xD7V[a\x02\xFBa\x0F\x9AV[`@Qa\x02^\x91\x90a)FV[a\x03\x10a\x10\xC7V[`@Qa\x02^\x91\x90a)~V[a\x03%a\x11\xF5V[`@Qa\x02^\x91\x90a)\xA7V[a\x02Ta\x13#V[a\x03Ba\x13\xE1V[`@Qa\x02^\x91\x90a)\xD0V[a\x02Ta\x14\xD0V[a\x02Ta\x15\x83V[a\x03ga\x16@V[`@Qa\x02^\x91\x90a*\x8AV[a\x02\xFBa\x17\x10V[a\x03Ba\x187V[a\x02\xFBa\x19\x1DV[a\x03\x94a\x1ABV[`@Qa\x02^\x91\x90a*\xE3V[a\x03\xA9a\x1BpV[`@Qa\x02^\x91\x90a+\x0CV[a\x02\xC6a\x1C\x9DV[a\x03\xC6a\x1C\xA7V[`@Qa\x02^\x91\x90a+4V[a\x02Ta\x1D\xD5V[a\x03ga\x1E\x91V[a\x03\xEBa\x1FaV[`@Q\x90\x15\x15\x81R` \x01a\x02^V[a\x02Ta \x80V[a\x02Ta!KV[a\x04\x13a\"\x0EV[`@Qa\x02^\x91\x90a+]V[a\x04(a#;V[`@Qa\x02^\x91\x90a+\x85V[a\x02\x94a$hV[a\x02Ta$\xC8V[`\0Ta\x03\xEB\x90`\xFF\x16\x81V[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R`\0\x80Q` a1J\x839\x81Q\x91R\x90c`\xF9\xBB\x11\x90a\x04\x83\x90\x84\x90`\x04\x01a)3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xC8\x91\x90\x81\x01\x90a+\xADV[`\x1B\x90a\x04\xD5\x90\x82a,\xA7V[PPV[`\0a\x05\x90`\x1B\x80Ta\x04\xEB\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x17\x90a,$V[\x80\x15a\x05dW\x80`\x1F\x10a\x059Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x0B\x9C\xDB\x1B\xDD\x14\x9B\xDB\xDD`\xBA\x1B\x81RPa%\x85V[\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x05\xA7\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD3\x90a,$V[\x80\x15a\x06 W\x80`\x1F\x10a\x05\xF5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06 V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x99XX\xDB\xDB\x94\xDD\x18]\x19T\x9B\xDB\xDD`\x82\x1B\x81RPa%\x85V[`\0a\x05\x90`\x1B\x80Ta\x06e\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x91\x90a,$V[\x80\x15a\x06\xDEW\x80`\x1F\x10a\x06\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u\x0B\x9B\x18]\x19\\\xDD\x10\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`R\x1B\x81RPa%\x85V[a\x07\x1Fa&;V[`\0[`\x05\x81\x10\x15a\t#W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x9D\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x07\xAD\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xCA\x91` \x01a-\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x07\xE7\x91\x90a,\xA7V[Pa\t\x06`\x1B\x80Ta\x07\xF8\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08$\x90a,$V[\x80\x15a\x08qW\x80`\x1F\x10a\x08FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08qV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08TW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\x1C\x80Ta\x08\x83\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xAF\x90a,$V[\x80\x15a\x08\xFCW\x80`\x1F\x10a\x08\xD1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xFCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xDFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa%\x85V[`\xA6\x82`\x05\x81\x10a\t\x19Wa\t\x19a-\xC3V[\x01U`\x01\x01a\x07\"V[P`@\x80Q`\xA0\x81\x01\x91\x82\x90R\x90`\xA6\x90`\x05\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t:W[PPPPP\x90P\x90V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\t\xCB\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF7\x90a,$V[\x80\x15a\nDW\x80`\x1F\x10a\n\x19Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n'W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`\x82\x1B\x81RPa%\x85V[a\n\x7Fa&YV[`\0[`,\x81\x10\x15a\x0BuW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xFD\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x0B\r\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B*\x91` \x01a-\xD9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x0BG\x91\x90a,\xA7V[Pa\x0BX`\x1B\x80Ta\x07\xF8\x90a,$V[`s\x82`,\x81\x10a\x0BkWa\x0Bka-\xC3V[\x01U`\x01\x01a\n\x82V[P`@\x80Qa\x05\x80\x81\x01\x91\x82\x90R`s\x80T\x82R\x90\x91`,\x90`t` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\x0C\xCCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0CL\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x0C\\\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0Cy\x91` \x01a.\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x0C\x96\x91\x90a,\xA7V[Pa\x0C\xA7`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x0C\xB9Wa\x0C\xB9a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0B\xD1V[P\x80`@Q` \x01a\x0C\xDE\x91\x90a.cV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01\x84\x806\x837\x01\x90PP\x90P`\0[`\x03\x81\x10\x15a\x0C\xCCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\roW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x97\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\r\xA7\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xC4\x91` \x01a.\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\r\xE1\x91\x90a,\xA7V[Pa\r\xF2`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x0E\x04Wa\x0E\x04a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r\x1CV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x0E\xE9\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x15\x90a,$V[\x80\x15a\x0FbW\x80`\x1F\x10a\x0F7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FbV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x05\xCCM\x8D\xECmi\x0C\xAC,\x8C\xAEJM\xED\xEE\x89-\xCC\x8C\xAF`[\x1B\x81RPa&\x05V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P`\0[`\x08\x81\x10\x15a\x10\xC1W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10A\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x10Q\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x10n\x91` \x01a.\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x10\x8B\x91\x90a,\xA7V[Pa\x10\x9C`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x10\xAEWa\x10\xAEa-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\xC6V[P\x91\x90PV[a\x10\xCFa&xV[`\0[`.\x81\x10\x15a\x11\xC5W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11M\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x11]\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11z\x91` \x01a/(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x11\x97\x91\x90a,\xA7V[Pa\x11\xA8`\x1B\x80Ta\x07\xF8\x90a,$V[`E\x82`.\x81\x10a\x11\xBBWa\x11\xBBa-\xC3V[\x01U`\x01\x01a\x10\xD2V[P`@\x80Qa\x05\xC0\x81\x01\x91\x82\x90R`E\x80T\x82R\x90\x91`.\x90`F` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[a\x11\xFDa&\x97V[`\0[`\t\x81\x10\x15a\x12\xF3W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12{\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x12\x8B\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x12\xA8\x91` \x01a/SV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x12\xC5\x91\x90a,\xA7V[Pa\x12\xD6`\x1B\x80Ta\x07\xF8\x90a,$V[`<\x82`\t\x81\x10a\x12\xE9Wa\x12\xE9a-\xC3V[\x01U`\x01\x01a\x12\0V[P`@\x80Qa\x01 \x81\x01\x91\x82\x90R`<\x80T\x82R\x90\x91`\t\x90`=` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x135\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13a\x90a,$V[\x80\x15a\x13\xAEW\x80`\x1F\x10a\x13\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xCE\xED.\x8D\x0C\x8EL.\xEC-\x89-\xCC\x8C\xAF`\x83\x1B\x81RPa&\x05V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14\xAFW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14qW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14\x05V[PPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x14\xE2\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x0E\x90a,$V[\x80\x15a\x15[W\x80`\x1F\x10a\x150Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15[V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15>W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x0B\x9C\xDB\x1B\xDD`\xDA\x1B\x81RPa&\x05V[`\0a\x05\x90`\x1B\x80Ta\x15\x95\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\xC1\x90a,$V[\x80\x15a\x16\x0EW\x80`\x1F\x10a\x15\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\x0EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x05\xCE\xCC-\x8D,\x8C.\x8D\xEEI-\xCC\x8C\xAF`\x8B\x1B\x81RPa&\x05V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x16\x83\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xAF\x90a,$V[\x80\x15a\x16\xFCW\x80`\x1F\x10a\x16\xD1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xFCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xDFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16dV[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\x10\xC1W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xB7\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x17\xC7\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xE4\x91` \x01a/\x7FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x18\x01\x91\x90a,\xA7V[Pa\x18\x12`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x18$Wa\x18$a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17<V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x19\x05W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18\xC7W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x18[V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\0[`\x04\x81\x10\x15a\x10\xC1W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\xC2\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x19\xD2\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19\xEF\x91` \x01a/\xBDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1A\x0C\x91\x90a,\xA7V[Pa\x1A\x1D`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x1A/Wa\x1A/a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x19GV[a\x1AJa&\xB6V[`\0[`\n\x81\x10\x15a\x1B@W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\xC8\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x1A\xD8\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1A\xF5\x91` \x01a/SV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1B\x12\x91\x90a,\xA7V[Pa\x1B#`\x1B\x80Ta\x07\xF8\x90a,$V[`2\x82`\n\x81\x10a\x1B6Wa\x1B6a-\xC3V[\x01U`\x01\x01a\x1AMV[P`@\x80Qa\x01@\x81\x01\x91\x82\x90R`2\x80T\x82R\x90\x91`\n\x90`3` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[a\x1Bxa&\xD5V[`\0[`\x03\x81\x10\x15a\x1CnW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xF6\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x1C\x06\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C#\x91` \x01a/\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1C@\x91\x90a,\xA7V[Pa\x1CQ`\x1B\x80Ta\x07\xF8\x90a,$V[`/\x82`\x03\x81\x10a\x1CdWa\x1Cda-\xC3V[\x01U`\x01\x01a\x1B{V[P`@\x80Q``\x81\x01\x91\x82\x90R`/\x80T\x82R\x90\x91`\x03\x90`0` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[``a\x05\x90a\x0B\xA5V[a\x1C\xAFa&\xF3V[`\0[`\x12\x81\x10\x15a\x1D\xA5W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D-\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x1D=\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1DZ\x91` \x01a0*V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1Dw\x91\x90a,\xA7V[Pa\x1D\x88`\x1B\x80Ta\x07\xF8\x90a,$V[`\x1D\x82`\x12\x81\x10a\x1D\x9BWa\x1D\x9Ba-\xC3V[\x01U`\x01\x01a\x1C\xB2V[P`@\x80Qa\x02@\x81\x01\x91\x82\x90R`\x1D\x80T\x82R\x90\x91`\x12\x90`\x1E` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x1D\xE7\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\x13\x90a,$V[\x80\x15a\x1E`W\x80`\x1F\x10a\x1E5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E`V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1ECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x0B\x9D\x1A[Y\\\xDD\x18[\\\x14\x9B\xDB\xDD`\x92\x1B\x81RPa%\x85V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1E\xD4\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\0\x90a,$V[\x80\x15a\x1FMW\x80`\x1F\x10a\x1F\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1FMV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xB5V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x1F\x81WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a1J\x839\x81Q\x91R;\x15a {W`@\x80Q`\0\x80Q` a1J\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a \x03\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a0WV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra \x1D\x91a0\x88V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a ZW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a _V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a w\x91\x90a0\xA4V[\x91PP[\x91\x90PV[`\0a\x05\x90`\x1B\x80Ta \x92\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \xBE\x90a,$V[\x80\x15a!\x0BW\x80`\x1F\x10a \xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x0BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \xEEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F.historicalSummaryIndex\0\0\0\0\0\0\0\0\0\x81RPa&\x05V[`\0a\x05\x90`\x1B\x80Ta!]\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\x89\x90a,$V[\x80\x15a!\xD6W\x80`\x1F\x10a!\xABWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xD6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\xB9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x0B\x99^\x19X\xDD]\x1A[\xDB\x94\x18^[\x1B\xD8Y\x14\x9B\xDB\xDD`Z\x1B\x81RPa%\x85V[a\"\x16a'\x12V[`\0[`\x07\x81\x10\x15a#\x0CW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\x94\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\"\xA4\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"\xC1\x91` \x01a0\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\"\xDE\x91\x90a,\xA7V[Pa\"\xEF`\x1B\x80Ta\x07\xF8\x90a,$V[`\x9F\x82`\x07\x81\x10a#\x02Wa#\x02a-\xC3V[\x01U`\x01\x01a\"\x19V[P`@\x80Q`\xE0\x81\x01\x91\x82\x90R`\x9F\x80T\x82R\x90\x91`\x07\x90`\xA0` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[a#Ca'0V[`\0[`\x04\x81\x10\x15a$9W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\xC1\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a#\xD1\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra#\xEE\x91` \x01a-\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a$\x0B\x91\x90a,\xA7V[Pa$\x1C`\x1B\x80Ta\x07\xF8\x90a,$V[`\xAB\x82`\x04\x81\x10a$/Wa$/a-\xC3V[\x01U`\x01\x01a#FV[P`@\x80Q`\x80\x81\x01\x91\x82\x90R`\xAB\x80T\x82R\x90\x91`\x04\x90`\xAC` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta$\xDA\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\x06\x90a,$V[\x80\x15a%SW\x80`\x1F\x10a%(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%SV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r.ValidatorFields[0]`h\x1B\x81RP[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R`\0\x90`\0\x80Q` a1J\x839\x81Q\x91R\x90c\x17w\xE5\x9D\x90a%\xBB\x90\x86\x90\x86\x90`\x04\x01a1\x0BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xFE\x91\x90a10V[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R`\0\x90`\0\x80Q` a1J\x839\x81Q\x91R\x90c\xAD\xDD\xE2\xB6\x90a%\xBB\x90\x86\x90\x86\x90`\x04\x01a1\x0BV[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\x80\x01`@R\x80`,\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\xC0\x01`@R\x80`.\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01@\x01`@R\x80`\n\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02@\x01`@R\x80`\x12\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80`\x07\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\x8DWa'\x8Da'NV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\xAFWa'\xAFa'NV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a'\xCFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xE6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a'\xF7W`\0\x80\xFD[\x805a(\na(\x05\x82a'\x95V[a'dV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a(\x1FW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\xA0\x81\x01\x81\x83`\0[`\x05\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a(FV[PPP\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\xAFW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(\x88V[P\x90\x95\x94PPPPPV[a\x05\x80\x81\x01\x81\x83`\0[`,\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a(\xC4V[`\0[\x83\x81\x10\x15a(\xFEW\x81\x81\x01Q\x83\x82\x01R` \x01a(\xE6V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra)\x1F\x81` \x86\x01` \x86\x01a(\xE3V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a%\xFE` \x83\x01\x84a)\x07V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\xAFW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a)`V[a\x05\xC0\x81\x01\x81\x83`\0[`.\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a)\x88V[a\x01 \x81\x01\x81\x83`\0[`\t\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a)\xB1V[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*~W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15a*fW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a*:V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a)\xF8V[P\x92\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*~W`?\x19\x87\x86\x03\x01\x84Ra*\xCE\x85\x83Qa)\x07V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*\xB2V[a\x01@\x81\x01\x81\x83`\0[`\n\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a*\xEDV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\x15V[a\x02@\x81\x01\x81\x83`\0[`\x12\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+>V[`\xE0\x81\x01\x81\x83`\0[`\x07\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+fV[`\x80\x81\x01\x81\x83`\0[`\x04\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\x8EV[`\0` \x82\x84\x03\x12\x15a+\xBFW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xD6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a+\xE7W`\0\x80\xFD[\x80Qa+\xF5a(\x05\x82a'\x95V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a,\nW`\0\x80\xFD[a,\x1B\x82` \x83\x01` \x86\x01a(\xE3V[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a,8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\xC1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a,\xA2W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a,\x7FWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a,\x9FW`\0\x81U`\x01\x01a,\x8BV[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xC1Wa,\xC1a'NV[a,\xD5\x81a,\xCF\x84Ta,$V[\x84a,XV[` `\x1F\x82\x11`\x01\x81\x14a-\tW`\0\x83\x15a,\xF1WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua,\x9FV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a-9W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a-\x19V[P\x84\x82\x10\x15a-WW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82Qa-x\x81\x84` \x87\x01a(\xE3V[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[o.TimestampProof[`\x80\x1B\x81R`\0\x82Qa-\xB6\x81`\x10\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x10\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x7F.HistoricalSummaryProof[\0\0\0\0\0\0\0\0\x81R`\0\x82Qa.\x11\x81`\x18\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x18\x01\x92\x91PPV[\x7F.WithdrawalCredentialProof[\0\0\0\0\0\x81R`\0\x82Qa.V\x81`\x1B\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x1B\x01\x92\x91PPV[\x81Q`\0\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15a.\x8EW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a.pV[P\x91\x95\x94PPPPPV[\x7F.StateRootAgainstLatestBlockHead\x81RgerProof[`\xC0\x1B` \x82\x01R`\0\x82Qa.\xE2\x81`(\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`(\x01\x92\x91PPV[p.ValidatorFields[`x\x1B\x81R`\0\x82Qa/\x1B\x81`\x11\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x11\x01\x92\x91PPV[o.ValidatorProof[`\x80\x1B\x81R`\0\x82Qa-\xB6\x81`\x10\x85\x01` \x87\x01a(\xE3V[p.WithdrawalProof[`x\x1B\x81R`\0\x82Qa/\x1B\x81`\x11\x85\x01` \x87\x01a(\xE3V[u.ValidatorFieldsProof[`P\x1B\x81R`\0\x82Qa/\xB0\x81`\x16\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x16\x01\x92\x91PPV[q.WithdrawalFields[`p\x1B\x81R`\0\x82Qa/\xEA\x81`\x12\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x12\x01\x92\x91PPV[j.SlotProof[`\xA8\x1B\x81R`\0\x82Qa0\x1D\x81`\x0B\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x0B\x01\x92\x91PPV[q.BlockHeaderProof[`p\x1B\x81R`\0\x82Qa/\xEA\x81`\x12\x85\x01` \x87\x01a(\xE3V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a0z\x81`\x04\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa0\x9A\x81\x84` \x87\x01a(\xE3V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a0\xB6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a%\xFEW`\0\x80\xFD[\x7F.ExecutionPayloadProof[\0\0\0\0\0\0\0\0\0\x81R`\0\x82Qa0\xFE\x81`\x17\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x17\x01\x92\x91PPV[`@\x81R`\0a1\x1E`@\x83\x01\x85a)\x07V[\x82\x81\x03` \x84\x01Ra,\x1B\x81\x85a)\x07V[`\0` \x82\x84\x03\x12\x15a1BW`\0\x80\xFD[PQ\x91\x90PV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \x84\x12D\x1F\t\x9FoI\xA1\xBF\xD5\xEA\xF6Y\xC4\xF3\x8BPd\x98\xAB\x9D\x812\x91\xDCJ\x86\xFAk]:dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106102325760003560e01c8063765aa60611610130578063b38061bf116100b8578063db364a401161007c578063db364a401461040b578063df18c78314610420578063e20c9f7114610435578063f148082c1461043d578063fa7626d41461044557600080fd5b8063b38061bf146103d3578063b5508aa9146103db578063ba414fa6146103e3578063d6461cbb146103fb578063d944472f1461040357600080fd5b8063950bb682116100ff578063950bb682146103845780639acfac291461038c5780639de4a9b3146103a1578063a157ef4c146103b6578063a5077429146103be57600080fd5b8063765aa6061461035757806385226c811461035f578063893893ca14610374578063916a17c61461037c57600080fd5b806334e3d90e116101be5780634c38f913116101825780634c38f9131461030857806364c725351461031d57806364f38ed71461033257806366d9a9a01461033a5780636c877c841461034f57600080fd5b806334e3d90e146102d35780633e5e3c23146102db5780633f7286f4146102e357806342864734146102eb5780634c20f510146102f357600080fd5b8063196cdfbd11610205578063196cdfbd146102775780631ed7831c1461028c578063275378b1146102a15780632872e20c146102a95780632fd1793c146102be57600080fd5b806308a4d71f1461023757806316f071531461024c578063177541fc1461026757806318aadf311461026f575b600080fd5b61024a6102453660046127bd565b610452565b005b6102546104d9565b6040519081526020015b60405180910390f35b610254610595565b610254610653565b61027f610717565b60405161025e919061283d565b610294610958565b60405161025e919061286e565b6102546109b9565b6102b1610a77565b60405161025e91906128ba565b6102c6610ba5565b60405161025e9190612933565b6102c6610cf3565b610294610e17565b610294610e77565b610254610ed7565b6102fb610f9a565b60405161025e9190612946565b6103106110c7565b60405161025e919061297e565b6103256111f5565b60405161025e91906129a7565b610254611323565b6103426113e1565b60405161025e91906129d0565b6102546114d0565b610254611583565b610367611640565b60405161025e9190612a8a565b6102fb611710565b610342611837565b6102fb61191d565b610394611a42565b60405161025e9190612ae3565b6103a9611b70565b60405161025e9190612b0c565b6102c6611c9d565b6103c6611ca7565b60405161025e9190612b34565b610254611dd5565b610367611e91565b6103eb611f61565b604051901515815260200161025e565b610254612080565b61025461214b565b61041361220e565b60405161025e9190612b5d565b61042861233b565b60405161025e9190612b85565b610294612468565b6102546124c8565b6000546103eb9060ff1681565b6040516360f9bb1160e01b815260008051602061314a833981519152906360f9bb1190610483908490600401612933565b600060405180830381865afa1580156104a0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104c89190810190612bad565b601b906104d59082612ca7565b5050565b6000610590601b80546104eb90612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461051790612c24565b80156105645780601f1061053957610100808354040283529160200191610564565b820191906000526020600020905b81548152906001019060200180831161054757829003601f168201915b5050505050604051806040016040528060098152602001680b9cdb1bdd149bdbdd60ba1b815250612585565b905090565b6000610590601b80546105a790612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546105d390612c24565b80156106205780601f106105f557610100808354040283529160200191610620565b820191906000526020600020905b81548152906001019060200180831161060357829003601f168201915b50505050506040518060400160405280601081526020016f0b98995858dbdb94dd185d19549bdbdd60821b815250612585565b6000610590601b805461066590612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461069190612c24565b80156106de5780601f106106b3576101008083540402835291602001916106de565b820191906000526020600020905b8154815290600101906020018083116106c157829003601f168201915b5050505050604051806040016040528060168152602001750b9b185d195cdd109b1bd8dad2195859195c949bdbdd60521b815250612585565b61071f61263b565b60005b60058110156109235760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610775573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261079d9190810190612bad565b6040516020016107ad9190612d66565b60408051601f19818403018152908290526107ca91602001612d8b565b604051602081830303815290604052601c90816107e79190612ca7565b50610906601b80546107f890612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461082490612c24565b80156108715780601f1061084657610100808354040283529160200191610871565b820191906000526020600020905b81548152906001019060200180831161085457829003601f168201915b5050505050601c805461088390612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546108af90612c24565b80156108fc5780601f106108d1576101008083540402835291602001916108fc565b820191906000526020600020905b8154815290600101906020018083116108df57829003601f168201915b5050505050612585565b60a6826005811061091957610919612dc3565b0155600101610722565b506040805160a08101918290529060a69060059082845b81548152602001906001019080831161093a575b5050505050905090565b6060600d80548060200260200160405190810160405280929190818152602001828054801561094e57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610992575050505050905090565b6000610590601b80546109cb90612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546109f790612c24565b8015610a445780601f10610a1957610100808354040283529160200191610a44565b820191906000526020600020905b815481529060010190602001808311610a2757829003601f168201915b50505050506040518060400160405280601081526020016f0b989b1bd8dad2195859195c949bdbdd60821b815250612585565b610a7f612659565b60005b602c811015610b755760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610ad5573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610afd9190810190612bad565b604051602001610b0d9190612d66565b60408051601f1981840301815290829052610b2a91602001612dd9565b604051602081830303815290604052601c9081610b479190612ca7565b50610b58601b80546107f890612c24565b607382602c8110610b6b57610b6b612dc3565b0155600101610a82565b50604080516105808101918290526073805482529091602c9060746020850180831161093a575050505050905090565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e811015610ccc5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610c24573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c4c9190810190612bad565b604051602001610c5c9190612d66565b60408051601f1981840301815290829052610c7991602001612e1e565b604051602081830303815290604052601c9081610c969190612ca7565b50610ca7601b80546107f890612c24565b828281518110610cb957610cb9612dc3565b6020908102919091010152600101610bd1565b5080604051602001610cde9190612e63565b60405160208183030381529060405291505090565b604080516003808252608082019092526060916000919060208201848036833701905050905060005b6003811015610ccc5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015610d6f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610d979190810190612bad565b604051602001610da79190612d66565b60408051601f1981840301815290829052610dc491602001612e99565b604051602081830303815290604052601c9081610de19190612ca7565b50610df2601b80546107f890612c24565b828281518110610e0457610e04612dc3565b6020908102919091010152600101610d1c565b6060600f80548060200260200160405190810160405280929190818152602001828054801561094e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610992575050505050905090565b6060600e80548060200260200160405190810160405280929190818152602001828054801561094e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610992575050505050905090565b6000610590601b8054610ee990612c24565b80601f0160208091040260200160405190810160405280929190818152602001828054610f1590612c24565b8015610f625780601f10610f3757610100808354040283529160200191610f62565b820191906000526020600020905b815481529060010190602001808311610f4557829003601f168201915b50505050506040518060400160405280601581526020017405cc4d8dec6d690cac2c8cae4a4dedee892dcc8caf605b1b815250612605565b604080516008808252610120820190925260609160009190602082016101008036833701905050905060005b60088110156110c15760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611019573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526110419190810190612bad565b6040516020016110519190612d66565b60408051601f198184030181529082905261106e91602001612eef565b604051602081830303815290604052601c908161108b9190612ca7565b5061109c601b80546107f890612c24565b8282815181106110ae576110ae612dc3565b6020908102919091010152600101610fc6565b50919050565b6110cf612678565b60005b602e8110156111c55760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611125573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261114d9190810190612bad565b60405160200161115d9190612d66565b60408051601f198184030181529082905261117a91602001612f28565b604051602081830303815290604052601c90816111979190612ca7565b506111a8601b80546107f890612c24565b604582602e81106111bb576111bb612dc3565b01556001016110d2565b50604080516105c08101918290526045805482529091602e9060466020850180831161093a575050505050905090565b6111fd612697565b60005b60098110156112f35760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611253573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261127b9190810190612bad565b60405160200161128b9190612d66565b60408051601f19818403018152908290526112a891602001612f53565b604051602081830303815290604052601c90816112c59190612ca7565b506112d6601b80546107f890612c24565b603c82600981106112e9576112e9612dc3565b0155600101611200565b5060408051610120810191829052603c805482529091600990603d6020850180831161093a575050505050905090565b6000610590601b805461133590612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461136190612c24565b80156113ae5780601f10611383576101008083540402835291602001916113ae565b820191906000526020600020905b81548152906001019060200180831161139157829003601f168201915b50505050506040518060400160405280601081526020016f05ceed2e8d0c8e4c2eec2d892dcc8caf60831b815250612605565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156114c75760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156114af57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114715790505b50505050508152505081526020019060010190611405565b50505050905090565b6000610590601b80546114e290612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461150e90612c24565b801561155b5780601f106115305761010080835404028352916020019161155b565b820191906000526020600020905b81548152906001019060200180831161153e57829003601f168201915b5050505050604051806040016040528060058152602001640b9cdb1bdd60da1b815250612605565b6000610590601b805461159590612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546115c190612c24565b801561160e5780601f106115e35761010080835404028352916020019161160e565b820191906000526020600020905b8154815290600101906020018083116115f157829003601f168201915b50505050506040518060400160405280600f81526020016e05cecc2d8d2c8c2e8dee492dcc8caf608b1b815250612605565b60606011805480602002602001604051908101604052809291908181526020016000905b828210156114c757838290600052602060002001805461168390612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546116af90612c24565b80156116fc5780601f106116d1576101008083540402835291602001916116fc565b820191906000526020600020905b8154815290600101906020018083116116df57829003601f168201915b505050505081526020019060010190611664565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e8110156110c15760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa15801561178f573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117b79190810190612bad565b6040516020016117c79190612d66565b60408051601f19818403018152908290526117e491602001612f7f565b604051602081830303815290604052601c90816118019190612ca7565b50611812601b80546107f890612c24565b82828151811061182457611824612dc3565b602090810291909101015260010161173c565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156114c75760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561190557602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116118c75790505b5050505050815250508152602001906001019061185b565b60408051600480825260a08201909252606091600091906020820160808036833701905050905060005b60048110156110c15760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa15801561199a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119c29190810190612bad565b6040516020016119d29190612d66565b60408051601f19818403018152908290526119ef91602001612fbd565b604051602081830303815290604052601c9081611a0c9190612ca7565b50611a1d601b80546107f890612c24565b828281518110611a2f57611a2f612dc3565b6020908102919091010152600101611947565b611a4a6126b6565b60005b600a811015611b405760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611aa0573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611ac89190810190612bad565b604051602001611ad89190612d66565b60408051601f1981840301815290829052611af591602001612f53565b604051602081830303815290604052601c9081611b129190612ca7565b50611b23601b80546107f890612c24565b603282600a8110611b3657611b36612dc3565b0155600101611a4d565b50604080516101408101918290526032805482529091600a9060336020850180831161093a575050505050905090565b611b786126d5565b60005b6003811015611c6e5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611bce573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611bf69190810190612bad565b604051602001611c069190612d66565b60408051601f1981840301815290829052611c2391602001612ff7565b604051602081830303815290604052601c9081611c409190612ca7565b50611c51601b80546107f890612c24565b602f8260038110611c6457611c64612dc3565b0155600101611b7b565b50604080516060810191829052602f80548252909160039060306020850180831161093a575050505050905090565b6060610590610ba5565b611caf6126f3565b60005b6012811015611da55760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015611d05573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d2d9190810190612bad565b604051602001611d3d9190612d66565b60408051601f1981840301815290829052611d5a9160200161302a565b604051602081830303815290604052601c9081611d779190612ca7565b50611d88601b80546107f890612c24565b601d8260128110611d9b57611d9b612dc3565b0155600101611cb2565b5060408051610240810191829052601d805482529091601290601e6020850180831161093a575050505050905090565b6000610590601b8054611de790612c24565b80601f0160208091040260200160405190810160405280929190818152602001828054611e1390612c24565b8015611e605780601f10611e3557610100808354040283529160200191611e60565b820191906000526020600020905b815481529060010190602001808311611e4357829003601f168201915b50505050506040518060400160405280600e81526020016d0b9d1a5b595cdd185b5c149bdbdd60921b815250612585565b60606010805480602002602001604051908101604052809291908181526020016000905b828210156114c7578382906000526020600020018054611ed490612c24565b80601f0160208091040260200160405190810160405280929190818152602001828054611f0090612c24565b8015611f4d5780601f10611f2257610100808354040283529160200191611f4d565b820191906000526020600020905b815481529060010190602001808311611f3057829003601f168201915b505050505081526020019060010190611eb5565b60008054610100900460ff1615611f815750600054610100900460ff1690565b600060008051602061314a8339815191523b1561207b576040805160008051602061314a833981519152602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091612003917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001613057565b60408051601f198184030181529082905261201d91613088565b6000604051808303816000865af19150503d806000811461205a576040519150601f19603f3d011682016040523d82523d6000602084013e61205f565b606091505b509150508080602001905181019061207791906130a4565b9150505b919050565b6000610590601b805461209290612c24565b80601f01602080910402602001604051908101604052809291908181526020018280546120be90612c24565b801561210b5780601f106120e05761010080835404028352916020019161210b565b820191906000526020600020905b8154815290600101906020018083116120ee57829003601f168201915b50505050506040518060400160405280601781526020017f2e686973746f726963616c53756d6d617279496e646578000000000000000000815250612605565b6000610590601b805461215d90612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461218990612c24565b80156121d65780601f106121ab576101008083540402835291602001916121d6565b820191906000526020600020905b8154815290600101906020018083116121b957829003601f168201915b5050505050604051806040016040528060158152602001740b995e1958dd5d1a5bdb94185e5b1bd859149bdbdd605a1b815250612585565b612216612712565b60005b600781101561230c5760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa15801561226c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526122949190810190612bad565b6040516020016122a49190612d66565b60408051601f19818403018152908290526122c1916020016130c6565b604051602081830303815290604052601c90816122de9190612ca7565b506122ef601b80546107f890612c24565b609f826007811061230257612302612dc3565b0155600101612219565b506040805160e0810191829052609f80548252909160079060a06020850180831161093a575050505050905090565b612343612730565b60005b60048110156124395760405163348051d760e11b81526004810182905260008051602061314a83398151915290636900a3ae90602401600060405180830381865afa158015612399573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526123c19190810190612bad565b6040516020016123d19190612d66565b60408051601f19818403018152908290526123ee91602001612d8b565b604051602081830303815290604052601c908161240b9190612ca7565b5061241c601b80546107f890612c24565b60ab826004811061242f5761242f612dc3565b0155600101612346565b5060408051608081019182905260ab80548252909160049060ac6020850180831161093a575050505050905090565b6060600c80548060200260200160405190810160405280929190818152602001828054801561094e576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610992575050505050905090565b6000610590601b80546124da90612c24565b80601f016020809104026020016040519081016040528092919081815260200182805461250690612c24565b80156125535780601f1061252857610100808354040283529160200191612553565b820191906000526020600020905b81548152906001019060200180831161253657829003601f168201915b5050505050604051806040016040528060138152602001722e56616c696461746f724669656c64735b305d60681b8152505b604051631777e59d60e01b815260009060008051602061314a83398151915290631777e59d906125bb908690869060040161310b565b6020604051808303816000875af11580156125da573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125fe9190613130565b9392505050565b6040516356eef15b60e11b815260009060008051602061314a8339815191529063addde2b6906125bb908690869060040161310b565b6040518060a001604052806005906020820280368337509192915050565b604051806105800160405280602c906020820280368337509192915050565b604051806105c00160405280602e906020820280368337509192915050565b6040518061012001604052806009906020820280368337509192915050565b604051806101400160405280600a906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b6040518061024001604052806012906020820280368337509192915050565b6040518060e001604052806007906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561278d5761278d61274e565b604052919050565b600067ffffffffffffffff8211156127af576127af61274e565b50601f01601f191660200190565b6000602082840312156127cf57600080fd5b813567ffffffffffffffff8111156127e657600080fd5b8201601f810184136127f757600080fd5b803561280a61280582612795565b612764565b81815285602083850101111561281f57600080fd5b81602084016020830137600091810160200191909152949350505050565b60a08101818360005b6005811015612865578151835260209283019290910190600101612846565b50505092915050565b602080825282518282018190526000918401906040840190835b818110156128af5783516001600160a01b0316835260209384019390920191600101612888565b509095945050505050565b6105808101818360005b602c8110156128655781518352602092830192909101906001016128c4565b60005b838110156128fe5781810151838201526020016128e6565b50506000910152565b6000815180845261291f8160208601602086016128e3565b601f01601f19169290920160200192915050565b6020815260006125fe6020830184612907565b602080825282518282018190526000918401906040840190835b818110156128af578351835260209384019390920191600101612960565b6105c08101818360005b602e811015612865578151835260209283019290910190600101612988565b6101208101818360005b60098110156128655781518352602092830192909101906001016129b1565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612a7e57868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b80831015612a665783516001600160e01b03191682526020938401936001939093019290910190612a3a565b509650505060209384019391909101906001016129f8565b50929695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b82811015612a7e57603f19878603018452612ace858351612907565b94506020938401939190910190600101612ab2565b6101408101818360005b600a811015612865578151835260209283019290910190600101612aed565b60608101818360005b6003811015612865578151835260209283019290910190600101612b15565b6102408101818360005b6012811015612865578151835260209283019290910190600101612b3e565b60e08101818360005b6007811015612865578151835260209283019290910190600101612b66565b60808101818360005b6004811015612865578151835260209283019290910190600101612b8e565b600060208284031215612bbf57600080fd5b815167ffffffffffffffff811115612bd657600080fd5b8201601f81018413612be757600080fd5b8051612bf561280582612795565b818152856020838501011115612c0a57600080fd5b612c1b8260208301602086016128e3565b95945050505050565b600181811c90821680612c3857607f821691505b6020821081036110c157634e487b7160e01b600052602260045260246000fd5b601f821115612ca257806000526020600020601f840160051c81016020851015612c7f5750805b601f840160051c820191505b81811015612c9f5760008155600101612c8b565b50505b505050565b815167ffffffffffffffff811115612cc157612cc161274e565b612cd581612ccf8454612c24565b84612c58565b6020601f821160018114612d095760008315612cf15750848201515b600019600385901b1c1916600184901b178455612c9f565b600084815260208120601f198516915b82811015612d395787850151825560209485019460019092019101612d19565b5084821015612d575786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b60008251612d788184602087016128e3565b605d60f81b920191825250600101919050565b6f2e54696d657374616d7050726f6f665b60801b815260008251612db68160108501602087016128e3565b9190910160100192915050565b634e487b7160e01b600052603260045260246000fd5b7f2e486973746f726963616c53756d6d61727950726f6f665b0000000000000000815260008251612e118160188501602087016128e3565b9190910160180192915050565b7f2e5769746864726177616c43726564656e7469616c50726f6f665b0000000000815260008251612e5681601b8501602087016128e3565b91909101601b0192915050565b8151600090829060208501835b82811015612e8e578151845260209384019390910190600101612e70565b509195945050505050565b7f2e5374617465526f6f74416761696e73744c6174657374426c6f636b48656164815267657250726f6f665b60c01b602082015260008251612ee28160288501602087016128e3565b9190910160280192915050565b702e56616c696461746f724669656c64735b60781b815260008251612f1b8160118501602087016128e3565b9190910160110192915050565b6f2e56616c696461746f7250726f6f665b60801b815260008251612db68160108501602087016128e3565b702e5769746864726177616c50726f6f665b60781b815260008251612f1b8160118501602087016128e3565b752e56616c696461746f724669656c647350726f6f665b60501b815260008251612fb08160168501602087016128e3565b9190910160160192915050565b712e5769746864726177616c4669656c64735b60701b815260008251612fea8160128501602087016128e3565b9190910160120192915050565b6a2e536c6f7450726f6f665b60a81b81526000825161301d81600b8501602087016128e3565b91909101600b0192915050565b712e426c6f636b48656164657250726f6f665b60701b815260008251612fea8160128501602087016128e3565b6001600160e01b031983168152815160009061307a8160048501602087016128e3565b919091016004019392505050565b6000825161309a8184602087016128e3565b9190910192915050565b6000602082840312156130b657600080fd5b815180151581146125fe57600080fd5b7f2e457865637574696f6e5061796c6f616450726f6f665b0000000000000000008152600082516130fe8160178501602087016128e3565b9190910160170192915050565b60408152600061311e6040830185612907565b8281036020840152612c1b8185612907565b60006020828403121561314257600080fd5b505191905056fe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da26469706673582212208412441f099f6f49a1bfd5eaf659c4f38b506498ab9d813291dc4a86fa6b5d3a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x022W`\x005`\xE0\x1C\x80cvZ\xA6\x06\x11a\x010W\x80c\xB3\x80a\xBF\x11a\0\xB8W\x80c\xDB6J@\x11a\0|W\x80c\xDB6J@\x14a\x04\x0BW\x80c\xDF\x18\xC7\x83\x14a\x04 W\x80c\xE2\x0C\x9Fq\x14a\x045W\x80c\xF1H\x08,\x14a\x04=W\x80c\xFAv&\xD4\x14a\x04EW`\0\x80\xFD[\x80c\xB3\x80a\xBF\x14a\x03\xD3W\x80c\xB5P\x8A\xA9\x14a\x03\xDBW\x80c\xBAAO\xA6\x14a\x03\xE3W\x80c\xD6F\x1C\xBB\x14a\x03\xFBW\x80c\xD9DG/\x14a\x04\x03W`\0\x80\xFD[\x80c\x95\x0B\xB6\x82\x11a\0\xFFW\x80c\x95\x0B\xB6\x82\x14a\x03\x84W\x80c\x9A\xCF\xAC)\x14a\x03\x8CW\x80c\x9D\xE4\xA9\xB3\x14a\x03\xA1W\x80c\xA1W\xEFL\x14a\x03\xB6W\x80c\xA5\x07t)\x14a\x03\xBEW`\0\x80\xFD[\x80cvZ\xA6\x06\x14a\x03WW\x80c\x85\"l\x81\x14a\x03_W\x80c\x898\x93\xCA\x14a\x03tW\x80c\x91j\x17\xC6\x14a\x03|W`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x11a\x01\xBEW\x80cL8\xF9\x13\x11a\x01\x82W\x80cL8\xF9\x13\x14a\x03\x08W\x80cd\xC7%5\x14a\x03\x1DW\x80cd\xF3\x8E\xD7\x14a\x032W\x80cf\xD9\xA9\xA0\x14a\x03:W\x80cl\x87|\x84\x14a\x03OW`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x14a\x02\xD3W\x80c>^<#\x14a\x02\xDBW\x80c?r\x86\xF4\x14a\x02\xE3W\x80cB\x86G4\x14a\x02\xEBW\x80cL \xF5\x10\x14a\x02\xF3W`\0\x80\xFD[\x80c\x19l\xDF\xBD\x11a\x02\x05W\x80c\x19l\xDF\xBD\x14a\x02wW\x80c\x1E\xD7\x83\x1C\x14a\x02\x8CW\x80c'Sx\xB1\x14a\x02\xA1W\x80c(r\xE2\x0C\x14a\x02\xA9W\x80c/\xD1y<\x14a\x02\xBEW`\0\x80\xFD[\x80c\x08\xA4\xD7\x1F\x14a\x027W\x80c\x16\xF0qS\x14a\x02LW\x80c\x17uA\xFC\x14a\x02gW\x80c\x18\xAA\xDF1\x14a\x02oW[`\0\x80\xFD[a\x02Ja\x02E6`\x04a'\xBDV[a\x04RV[\0[a\x02Ta\x04\xD9V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02Ta\x05\x95V[a\x02Ta\x06SV[a\x02\x7Fa\x07\x17V[`@Qa\x02^\x91\x90a(=V[a\x02\x94a\tXV[`@Qa\x02^\x91\x90a(nV[a\x02Ta\t\xB9V[a\x02\xB1a\nwV[`@Qa\x02^\x91\x90a(\xBAV[a\x02\xC6a\x0B\xA5V[`@Qa\x02^\x91\x90a)3V[a\x02\xC6a\x0C\xF3V[a\x02\x94a\x0E\x17V[a\x02\x94a\x0EwV[a\x02Ta\x0E\xD7V[a\x02\xFBa\x0F\x9AV[`@Qa\x02^\x91\x90a)FV[a\x03\x10a\x10\xC7V[`@Qa\x02^\x91\x90a)~V[a\x03%a\x11\xF5V[`@Qa\x02^\x91\x90a)\xA7V[a\x02Ta\x13#V[a\x03Ba\x13\xE1V[`@Qa\x02^\x91\x90a)\xD0V[a\x02Ta\x14\xD0V[a\x02Ta\x15\x83V[a\x03ga\x16@V[`@Qa\x02^\x91\x90a*\x8AV[a\x02\xFBa\x17\x10V[a\x03Ba\x187V[a\x02\xFBa\x19\x1DV[a\x03\x94a\x1ABV[`@Qa\x02^\x91\x90a*\xE3V[a\x03\xA9a\x1BpV[`@Qa\x02^\x91\x90a+\x0CV[a\x02\xC6a\x1C\x9DV[a\x03\xC6a\x1C\xA7V[`@Qa\x02^\x91\x90a+4V[a\x02Ta\x1D\xD5V[a\x03ga\x1E\x91V[a\x03\xEBa\x1FaV[`@Q\x90\x15\x15\x81R` \x01a\x02^V[a\x02Ta \x80V[a\x02Ta!KV[a\x04\x13a\"\x0EV[`@Qa\x02^\x91\x90a+]V[a\x04(a#;V[`@Qa\x02^\x91\x90a+\x85V[a\x02\x94a$hV[a\x02Ta$\xC8V[`\0Ta\x03\xEB\x90`\xFF\x16\x81V[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R`\0\x80Q` a1J\x839\x81Q\x91R\x90c`\xF9\xBB\x11\x90a\x04\x83\x90\x84\x90`\x04\x01a)3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xC8\x91\x90\x81\x01\x90a+\xADV[`\x1B\x90a\x04\xD5\x90\x82a,\xA7V[PPV[`\0a\x05\x90`\x1B\x80Ta\x04\xEB\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x17\x90a,$V[\x80\x15a\x05dW\x80`\x1F\x10a\x059Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x0B\x9C\xDB\x1B\xDD\x14\x9B\xDB\xDD`\xBA\x1B\x81RPa%\x85V[\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x05\xA7\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD3\x90a,$V[\x80\x15a\x06 W\x80`\x1F\x10a\x05\xF5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06 V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x99XX\xDB\xDB\x94\xDD\x18]\x19T\x9B\xDB\xDD`\x82\x1B\x81RPa%\x85V[`\0a\x05\x90`\x1B\x80Ta\x06e\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x91\x90a,$V[\x80\x15a\x06\xDEW\x80`\x1F\x10a\x06\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u\x0B\x9B\x18]\x19\\\xDD\x10\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`R\x1B\x81RPa%\x85V[a\x07\x1Fa&;V[`\0[`\x05\x81\x10\x15a\t#W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x9D\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x07\xAD\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xCA\x91` \x01a-\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x07\xE7\x91\x90a,\xA7V[Pa\t\x06`\x1B\x80Ta\x07\xF8\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08$\x90a,$V[\x80\x15a\x08qW\x80`\x1F\x10a\x08FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08qV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08TW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\x1C\x80Ta\x08\x83\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xAF\x90a,$V[\x80\x15a\x08\xFCW\x80`\x1F\x10a\x08\xD1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xFCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xDFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa%\x85V[`\xA6\x82`\x05\x81\x10a\t\x19Wa\t\x19a-\xC3V[\x01U`\x01\x01a\x07\"V[P`@\x80Q`\xA0\x81\x01\x91\x82\x90R\x90`\xA6\x90`\x05\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t:W[PPPPP\x90P\x90V[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\t\xCB\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xF7\x90a,$V[\x80\x15a\nDW\x80`\x1F\x10a\n\x19Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n'W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`\x82\x1B\x81RPa%\x85V[a\n\x7Fa&YV[`\0[`,\x81\x10\x15a\x0BuW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xFD\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x0B\r\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B*\x91` \x01a-\xD9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x0BG\x91\x90a,\xA7V[Pa\x0BX`\x1B\x80Ta\x07\xF8\x90a,$V[`s\x82`,\x81\x10a\x0BkWa\x0Bka-\xC3V[\x01U`\x01\x01a\n\x82V[P`@\x80Qa\x05\x80\x81\x01\x91\x82\x90R`s\x80T\x82R\x90\x91`,\x90`t` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\x0C\xCCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0CL\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x0C\\\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0Cy\x91` \x01a.\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x0C\x96\x91\x90a,\xA7V[Pa\x0C\xA7`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x0C\xB9Wa\x0C\xB9a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0B\xD1V[P\x80`@Q` \x01a\x0C\xDE\x91\x90a.cV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01\x84\x806\x837\x01\x90PP\x90P`\0[`\x03\x81\x10\x15a\x0C\xCCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\roW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x97\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\r\xA7\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xC4\x91` \x01a.\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\r\xE1\x91\x90a,\xA7V[Pa\r\xF2`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x0E\x04Wa\x0E\x04a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r\x1CV[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x0E\xE9\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\x15\x90a,$V[\x80\x15a\x0FbW\x80`\x1F\x10a\x0F7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0FbV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x05\xCCM\x8D\xECmi\x0C\xAC,\x8C\xAEJM\xED\xEE\x89-\xCC\x8C\xAF`[\x1B\x81RPa&\x05V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P`\0[`\x08\x81\x10\x15a\x10\xC1W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10A\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x10Q\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x10n\x91` \x01a.\xEFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x10\x8B\x91\x90a,\xA7V[Pa\x10\x9C`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x10\xAEWa\x10\xAEa-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0F\xC6V[P\x91\x90PV[a\x10\xCFa&xV[`\0[`.\x81\x10\x15a\x11\xC5W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11M\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x11]\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11z\x91` \x01a/(V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x11\x97\x91\x90a,\xA7V[Pa\x11\xA8`\x1B\x80Ta\x07\xF8\x90a,$V[`E\x82`.\x81\x10a\x11\xBBWa\x11\xBBa-\xC3V[\x01U`\x01\x01a\x10\xD2V[P`@\x80Qa\x05\xC0\x81\x01\x91\x82\x90R`E\x80T\x82R\x90\x91`.\x90`F` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[a\x11\xFDa&\x97V[`\0[`\t\x81\x10\x15a\x12\xF3W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12{\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x12\x8B\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x12\xA8\x91` \x01a/SV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x12\xC5\x91\x90a,\xA7V[Pa\x12\xD6`\x1B\x80Ta\x07\xF8\x90a,$V[`<\x82`\t\x81\x10a\x12\xE9Wa\x12\xE9a-\xC3V[\x01U`\x01\x01a\x12\0V[P`@\x80Qa\x01 \x81\x01\x91\x82\x90R`<\x80T\x82R\x90\x91`\t\x90`=` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x135\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13a\x90a,$V[\x80\x15a\x13\xAEW\x80`\x1F\x10a\x13\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xCE\xED.\x8D\x0C\x8EL.\xEC-\x89-\xCC\x8C\xAF`\x83\x1B\x81RPa&\x05V[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14\xAFW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14qW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14\x05V[PPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x14\xE2\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x0E\x90a,$V[\x80\x15a\x15[W\x80`\x1F\x10a\x150Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15[V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15>W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x0B\x9C\xDB\x1B\xDD`\xDA\x1B\x81RPa&\x05V[`\0a\x05\x90`\x1B\x80Ta\x15\x95\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\xC1\x90a,$V[\x80\x15a\x16\x0EW\x80`\x1F\x10a\x15\xE3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\x0EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x05\xCE\xCC-\x8D,\x8C.\x8D\xEEI-\xCC\x8C\xAF`\x8B\x1B\x81RPa&\x05V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x16\x83\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xAF\x90a,$V[\x80\x15a\x16\xFCW\x80`\x1F\x10a\x16\xD1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xFCV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xDFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16dV[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\x10\xC1W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xB7\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x17\xC7\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xE4\x91` \x01a/\x7FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x18\x01\x91\x90a,\xA7V[Pa\x18\x12`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x18$Wa\x18$a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17<V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x19\x05W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18\xC7W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x18[V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\0[`\x04\x81\x10\x15a\x10\xC1W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\xC2\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x19\xD2\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19\xEF\x91` \x01a/\xBDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1A\x0C\x91\x90a,\xA7V[Pa\x1A\x1D`\x1B\x80Ta\x07\xF8\x90a,$V[\x82\x82\x81Q\x81\x10a\x1A/Wa\x1A/a-\xC3V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x19GV[a\x1AJa&\xB6V[`\0[`\n\x81\x10\x15a\x1B@W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\xC8\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x1A\xD8\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1A\xF5\x91` \x01a/SV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1B\x12\x91\x90a,\xA7V[Pa\x1B#`\x1B\x80Ta\x07\xF8\x90a,$V[`2\x82`\n\x81\x10a\x1B6Wa\x1B6a-\xC3V[\x01U`\x01\x01a\x1AMV[P`@\x80Qa\x01@\x81\x01\x91\x82\x90R`2\x80T\x82R\x90\x91`\n\x90`3` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[a\x1Bxa&\xD5V[`\0[`\x03\x81\x10\x15a\x1CnW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xF6\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x1C\x06\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C#\x91` \x01a/\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1C@\x91\x90a,\xA7V[Pa\x1CQ`\x1B\x80Ta\x07\xF8\x90a,$V[`/\x82`\x03\x81\x10a\x1CdWa\x1Cda-\xC3V[\x01U`\x01\x01a\x1B{V[P`@\x80Q``\x81\x01\x91\x82\x90R`/\x80T\x82R\x90\x91`\x03\x90`0` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[``a\x05\x90a\x0B\xA5V[a\x1C\xAFa&\xF3V[`\0[`\x12\x81\x10\x15a\x1D\xA5W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D-\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\x1D=\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1DZ\x91` \x01a0*V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\x1Dw\x91\x90a,\xA7V[Pa\x1D\x88`\x1B\x80Ta\x07\xF8\x90a,$V[`\x1D\x82`\x12\x81\x10a\x1D\x9BWa\x1D\x9Ba-\xC3V[\x01U`\x01\x01a\x1C\xB2V[P`@\x80Qa\x02@\x81\x01\x91\x82\x90R`\x1D\x80T\x82R\x90\x91`\x12\x90`\x1E` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta\x1D\xE7\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\x13\x90a,$V[\x80\x15a\x1E`W\x80`\x1F\x10a\x1E5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E`V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1ECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x0B\x9D\x1A[Y\\\xDD\x18[\\\x14\x9B\xDB\xDD`\x92\x1B\x81RPa%\x85V[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x14\xC7W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1E\xD4\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\0\x90a,$V[\x80\x15a\x1FMW\x80`\x1F\x10a\x1F\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1FMV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xB5V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x1F\x81WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a1J\x839\x81Q\x91R;\x15a {W`@\x80Q`\0\x80Q` a1J\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a \x03\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a0WV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra \x1D\x91a0\x88V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a ZW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a _V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a w\x91\x90a0\xA4V[\x91PP[\x91\x90PV[`\0a\x05\x90`\x1B\x80Ta \x92\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \xBE\x90a,$V[\x80\x15a!\x0BW\x80`\x1F\x10a \xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x0BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \xEEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F.historicalSummaryIndex\0\0\0\0\0\0\0\0\0\x81RPa&\x05V[`\0a\x05\x90`\x1B\x80Ta!]\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\x89\x90a,$V[\x80\x15a!\xD6W\x80`\x1F\x10a!\xABWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xD6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\xB9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x0B\x99^\x19X\xDD]\x1A[\xDB\x94\x18^[\x1B\xD8Y\x14\x9B\xDB\xDD`Z\x1B\x81RPa%\x85V[a\"\x16a'\x12V[`\0[`\x07\x81\x10\x15a#\x0CW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\x94\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a\"\xA4\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"\xC1\x91` \x01a0\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a\"\xDE\x91\x90a,\xA7V[Pa\"\xEF`\x1B\x80Ta\x07\xF8\x90a,$V[`\x9F\x82`\x07\x81\x10a#\x02Wa#\x02a-\xC3V[\x01U`\x01\x01a\"\x19V[P`@\x80Q`\xE0\x81\x01\x91\x82\x90R`\x9F\x80T\x82R\x90\x91`\x07\x90`\xA0` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[a#Ca'0V[`\0[`\x04\x81\x10\x15a$9W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a1J\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\xC1\x91\x90\x81\x01\x90a+\xADV[`@Q` \x01a#\xD1\x91\x90a-fV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra#\xEE\x91` \x01a-\x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1C\x90\x81a$\x0B\x91\x90a,\xA7V[Pa$\x1C`\x1B\x80Ta\x07\xF8\x90a,$V[`\xAB\x82`\x04\x81\x10a$/Wa$/a-\xC3V[\x01U`\x01\x01a#FV[P`@\x80Q`\x80\x81\x01\x91\x82\x90R`\xAB\x80T\x82R\x90\x91`\x04\x90`\xAC` \x85\x01\x80\x83\x11a\t:WPPPPP\x90P\x90V[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tNW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x92WPPPPP\x90P\x90V[`\0a\x05\x90`\x1B\x80Ta$\xDA\x90a,$V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\x06\x90a,$V[\x80\x15a%SW\x80`\x1F\x10a%(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%SV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r.ValidatorFields[0]`h\x1B\x81RP[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R`\0\x90`\0\x80Q` a1J\x839\x81Q\x91R\x90c\x17w\xE5\x9D\x90a%\xBB\x90\x86\x90\x86\x90`\x04\x01a1\x0BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xFE\x91\x90a10V[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R`\0\x90`\0\x80Q` a1J\x839\x81Q\x91R\x90c\xAD\xDD\xE2\xB6\x90a%\xBB\x90\x86\x90\x86\x90`\x04\x01a1\x0BV[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\x80\x01`@R\x80`,\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\xC0\x01`@R\x80`.\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01@\x01`@R\x80`\n\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02@\x01`@R\x80`\x12\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80`\x07\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a'\x8DWa'\x8Da'NV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a'\xAFWa'\xAFa'NV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a'\xCFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\xE6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a'\xF7W`\0\x80\xFD[\x805a(\na(\x05\x82a'\x95V[a'dV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a(\x1FW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\xA0\x81\x01\x81\x83`\0[`\x05\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a(FV[PPP\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\xAFW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a(\x88V[P\x90\x95\x94PPPPPV[a\x05\x80\x81\x01\x81\x83`\0[`,\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a(\xC4V[`\0[\x83\x81\x10\x15a(\xFEW\x81\x81\x01Q\x83\x82\x01R` \x01a(\xE6V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra)\x1F\x81` \x86\x01` \x86\x01a(\xE3V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a%\xFE` \x83\x01\x84a)\x07V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a(\xAFW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a)`V[a\x05\xC0\x81\x01\x81\x83`\0[`.\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a)\x88V[a\x01 \x81\x01\x81\x83`\0[`\t\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a)\xB1V[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*~W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15a*fW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90a*:V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a)\xF8V[P\x92\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a*~W`?\x19\x87\x86\x03\x01\x84Ra*\xCE\x85\x83Qa)\x07V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a*\xB2V[a\x01@\x81\x01\x81\x83`\0[`\n\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a*\xEDV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\x15V[a\x02@\x81\x01\x81\x83`\0[`\x12\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+>V[`\xE0\x81\x01\x81\x83`\0[`\x07\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+fV[`\x80\x81\x01\x81\x83`\0[`\x04\x81\x10\x15a(eW\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\x8EV[`\0` \x82\x84\x03\x12\x15a+\xBFW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xD6W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a+\xE7W`\0\x80\xFD[\x80Qa+\xF5a(\x05\x82a'\x95V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a,\nW`\0\x80\xFD[a,\x1B\x82` \x83\x01` \x86\x01a(\xE3V[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a,8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10\xC1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a,\xA2W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a,\x7FWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a,\x9FW`\0\x81U`\x01\x01a,\x8BV[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xC1Wa,\xC1a'NV[a,\xD5\x81a,\xCF\x84Ta,$V[\x84a,XV[` `\x1F\x82\x11`\x01\x81\x14a-\tW`\0\x83\x15a,\xF1WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua,\x9FV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a-9W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a-\x19V[P\x84\x82\x10\x15a-WW\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x82Qa-x\x81\x84` \x87\x01a(\xE3V[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[o.TimestampProof[`\x80\x1B\x81R`\0\x82Qa-\xB6\x81`\x10\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x10\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x7F.HistoricalSummaryProof[\0\0\0\0\0\0\0\0\x81R`\0\x82Qa.\x11\x81`\x18\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x18\x01\x92\x91PPV[\x7F.WithdrawalCredentialProof[\0\0\0\0\0\x81R`\0\x82Qa.V\x81`\x1B\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x1B\x01\x92\x91PPV[\x81Q`\0\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15a.\x8EW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a.pV[P\x91\x95\x94PPPPPV[\x7F.StateRootAgainstLatestBlockHead\x81RgerProof[`\xC0\x1B` \x82\x01R`\0\x82Qa.\xE2\x81`(\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`(\x01\x92\x91PPV[p.ValidatorFields[`x\x1B\x81R`\0\x82Qa/\x1B\x81`\x11\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x11\x01\x92\x91PPV[o.ValidatorProof[`\x80\x1B\x81R`\0\x82Qa-\xB6\x81`\x10\x85\x01` \x87\x01a(\xE3V[p.WithdrawalProof[`x\x1B\x81R`\0\x82Qa/\x1B\x81`\x11\x85\x01` \x87\x01a(\xE3V[u.ValidatorFieldsProof[`P\x1B\x81R`\0\x82Qa/\xB0\x81`\x16\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x16\x01\x92\x91PPV[q.WithdrawalFields[`p\x1B\x81R`\0\x82Qa/\xEA\x81`\x12\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x12\x01\x92\x91PPV[j.SlotProof[`\xA8\x1B\x81R`\0\x82Qa0\x1D\x81`\x0B\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x0B\x01\x92\x91PPV[q.BlockHeaderProof[`p\x1B\x81R`\0\x82Qa/\xEA\x81`\x12\x85\x01` \x87\x01a(\xE3V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a0z\x81`\x04\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa0\x9A\x81\x84` \x87\x01a(\xE3V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a0\xB6W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a%\xFEW`\0\x80\xFD[\x7F.ExecutionPayloadProof[\0\0\0\0\0\0\0\0\0\x81R`\0\x82Qa0\xFE\x81`\x17\x85\x01` \x87\x01a(\xE3V[\x91\x90\x91\x01`\x17\x01\x92\x91PPV[`@\x81R`\0a1\x1E`@\x83\x01\x85a)\x07V[\x82\x81\x03` \x84\x01Ra,\x1B\x81\x85a)\x07V[`\0` \x82\x84\x03\x12\x15a1BW`\0\x80\xFD[PQ\x91\x90PV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \x84\x12D\x1F\t\x9FoI\xA1\xBF\xD5\xEA\xF6Y\xC4\xF3\x8BPd\x98\xAB\x9D\x812\x91\xDCJ\x86\xFAk]:dsolcC\0\x08\x1B\x003",
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
    /**Function with signature `getBalanceUpdateProof()` and selector `0xa157ef4c`.
```solidity
function getBalanceUpdateProof() external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceUpdateProofCall {}
    ///Container type for the return parameters of the [`getBalanceUpdateProof()`](getBalanceUpdateProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceUpdateProofReturn {
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
            impl ::core::convert::From<getBalanceUpdateProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceUpdateProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceUpdateProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<getBalanceUpdateProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceUpdateProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceUpdateProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceUpdateProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceUpdateProofReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalanceUpdateProof()";
            const SELECTOR: [u8; 4] = [161u8, 87u8, 239u8, 76u8];
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
    /**Function with signature `getBeaconStateRoot()` and selector `0x177541fc`.
```solidity
function getBeaconStateRoot() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBeaconStateRootCall {}
    ///Container type for the return parameters of the [`getBeaconStateRoot()`](getBeaconStateRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBeaconStateRootReturn {
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
            impl ::core::convert::From<getBeaconStateRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBeaconStateRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBeaconStateRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getBeaconStateRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBeaconStateRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBeaconStateRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBeaconStateRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBeaconStateRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBeaconStateRoot()";
            const SELECTOR: [u8; 4] = [23u8, 117u8, 65u8, 252u8];
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
    /**Function with signature `getBlockHeaderProof()` and selector `0xa5077429`.
```solidity
function getBlockHeaderProof() external returns (bytes32[18] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeaderProofCall {}
    ///Container type for the return parameters of the [`getBlockHeaderProof()`](getBlockHeaderProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeaderProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 18usize],
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
            impl ::core::convert::From<getBlockHeaderProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeaderProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockHeaderProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    18usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 18usize],
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
            impl ::core::convert::From<getBlockHeaderProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeaderProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockHeaderProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockHeaderProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockHeaderProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    18usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockHeaderProof()";
            const SELECTOR: [u8; 4] = [165u8, 7u8, 116u8, 41u8];
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
    /**Function with signature `getBlockRoot()` and selector `0x275378b1`.
```solidity
function getBlockRoot() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootCall {}
    ///Container type for the return parameters of the [`getBlockRoot()`](getBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootReturn {
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
            impl ::core::convert::From<getBlockRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getBlockRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockRoot()";
            const SELECTOR: [u8; 4] = [39u8, 83u8, 120u8, 177u8];
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
    /**Function with signature `getBlockRootIndex()` and selector `0x42864734`.
```solidity
function getBlockRootIndex() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootIndexCall {}
    ///Container type for the return parameters of the [`getBlockRootIndex()`](getBlockRootIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootIndexReturn {
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
            impl ::core::convert::From<getBlockRootIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockRootIndexCall {
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
            impl ::core::convert::From<getBlockRootIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockRootIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockRootIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockRootIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockRootIndex()";
            const SELECTOR: [u8; 4] = [66u8, 134u8, 71u8, 52u8];
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
    /**Function with signature `getExecutionPayloadProof()` and selector `0xdb364a40`.
```solidity
function getExecutionPayloadProof() external returns (bytes32[7] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadProofCall {}
    ///Container type for the return parameters of the [`getExecutionPayloadProof()`](getExecutionPayloadProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 7usize],
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
            impl ::core::convert::From<getExecutionPayloadProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    7usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 7usize],
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
            impl ::core::convert::From<getExecutionPayloadProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getExecutionPayloadProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getExecutionPayloadProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    7usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getExecutionPayloadProof()";
            const SELECTOR: [u8; 4] = [219u8, 54u8, 74u8, 64u8];
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
    /**Function with signature `getExecutionPayloadRoot()` and selector `0xd944472f`.
```solidity
function getExecutionPayloadRoot() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadRootCall {}
    ///Container type for the return parameters of the [`getExecutionPayloadRoot()`](getExecutionPayloadRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadRootReturn {
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
            impl ::core::convert::From<getExecutionPayloadRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getExecutionPayloadRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getExecutionPayloadRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getExecutionPayloadRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getExecutionPayloadRoot()";
            const SELECTOR: [u8; 4] = [217u8, 68u8, 71u8, 47u8];
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
    /**Function with signature `getHistoricalSummaryIndex()` and selector `0xd6461cbb`.
```solidity
function getHistoricalSummaryIndex() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryIndexCall {}
    ///Container type for the return parameters of the [`getHistoricalSummaryIndex()`](getHistoricalSummaryIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryIndexReturn {
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
            impl ::core::convert::From<getHistoricalSummaryIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryIndexCall {
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
            impl ::core::convert::From<getHistoricalSummaryIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHistoricalSummaryIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHistoricalSummaryIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHistoricalSummaryIndex()";
            const SELECTOR: [u8; 4] = [214u8, 70u8, 28u8, 187u8];
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
    /**Function with signature `getHistoricalSummaryProof()` and selector `0x2872e20c`.
```solidity
function getHistoricalSummaryProof() external returns (bytes32[44] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryProofCall {}
    ///Container type for the return parameters of the [`getHistoricalSummaryProof()`](getHistoricalSummaryProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 44usize],
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
            impl ::core::convert::From<getHistoricalSummaryProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    44usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 44usize],
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
            impl ::core::convert::From<getHistoricalSummaryProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHistoricalSummaryProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHistoricalSummaryProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    44usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHistoricalSummaryProof()";
            const SELECTOR: [u8; 4] = [40u8, 114u8, 226u8, 12u8];
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
    /**Function with signature `getLatestBlockRoot()` and selector `0x18aadf31`.
```solidity
function getLatestBlockRoot() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestBlockRootCall {}
    ///Container type for the return parameters of the [`getLatestBlockRoot()`](getLatestBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestBlockRootReturn {
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
            impl ::core::convert::From<getLatestBlockRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestBlockRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getLatestBlockRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestBlockRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestBlockRoot()";
            const SELECTOR: [u8; 4] = [24u8, 170u8, 223u8, 49u8];
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
    /**Function with signature `getSlot()` and selector `0x6c877c84`.
```solidity
function getSlot() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotCall {}
    ///Container type for the return parameters of the [`getSlot()`](getSlotCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotReturn {
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
            impl ::core::convert::From<getSlotCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotCall {
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
            impl ::core::convert::From<getSlotReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlot()";
            const SELECTOR: [u8; 4] = [108u8, 135u8, 124u8, 132u8];
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
    /**Function with signature `getSlotProof()` and selector `0x9de4a9b3`.
```solidity
function getSlotProof() external returns (bytes32[3] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotProofCall {}
    ///Container type for the return parameters of the [`getSlotProof()`](getSlotProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 3usize],
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
            impl ::core::convert::From<getSlotProofCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    3usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 3usize],
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
            impl ::core::convert::From<getSlotProofReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    3usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlotProof()";
            const SELECTOR: [u8; 4] = [157u8, 228u8, 169u8, 179u8];
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
    /**Function with signature `getSlotRoot()` and selector `0x16f07153`.
```solidity
function getSlotRoot() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotRootCall {}
    ///Container type for the return parameters of the [`getSlotRoot()`](getSlotRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotRootReturn {
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
            impl ::core::convert::From<getSlotRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getSlotRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlotRoot()";
            const SELECTOR: [u8; 4] = [22u8, 240u8, 113u8, 83u8];
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
    /**Function with signature `getStateRootProof()` and selector `0x34e3d90e`.
```solidity
function getStateRootProof() external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateRootProofCall {}
    ///Container type for the return parameters of the [`getStateRootProof()`](getStateRootProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateRootProofReturn {
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
            impl ::core::convert::From<getStateRootProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStateRootProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStateRootProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<getStateRootProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStateRootProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStateRootProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStateRootProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStateRootProofReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStateRootProof()";
            const SELECTOR: [u8; 4] = [52u8, 227u8, 217u8, 14u8];
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
    /**Function with signature `getTimestampProofCapella()` and selector `0xdf18c783`.
```solidity
function getTimestampProofCapella() external returns (bytes32[4] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofCapellaCall {}
    ///Container type for the return parameters of the [`getTimestampProofCapella()`](getTimestampProofCapellaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofCapellaReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 4usize],
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
            impl ::core::convert::From<getTimestampProofCapellaCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofCapellaCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofCapellaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    4usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 4usize],
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
            impl ::core::convert::From<getTimestampProofCapellaReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofCapellaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofCapellaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampProofCapellaCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampProofCapellaReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    4usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampProofCapella()";
            const SELECTOR: [u8; 4] = [223u8, 24u8, 199u8, 131u8];
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
    /**Function with signature `getTimestampProofDeneb()` and selector `0x196cdfbd`.
```solidity
function getTimestampProofDeneb() external returns (bytes32[5] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofDenebCall {}
    ///Container type for the return parameters of the [`getTimestampProofDeneb()`](getTimestampProofDenebCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofDenebReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 5usize],
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
            impl ::core::convert::From<getTimestampProofDenebCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofDenebCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofDenebCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    5usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 5usize],
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
            impl ::core::convert::From<getTimestampProofDenebReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofDenebReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofDenebReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampProofDenebCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampProofDenebReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    5usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampProofDeneb()";
            const SELECTOR: [u8; 4] = [25u8, 108u8, 223u8, 189u8];
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
    /**Function with signature `getTimestampRoot()` and selector `0xb38061bf`.
```solidity
function getTimestampRoot() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampRootCall {}
    ///Container type for the return parameters of the [`getTimestampRoot()`](getTimestampRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampRootReturn {
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
            impl ::core::convert::From<getTimestampRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getTimestampRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampRoot()";
            const SELECTOR: [u8; 4] = [179u8, 128u8, 97u8, 191u8];
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
    /**Function with signature `getValidatorFields()` and selector `0x4c20f510`.
```solidity
function getValidatorFields() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsCall {}
    ///Container type for the return parameters of the [`getValidatorFields()`](getValidatorFieldsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getValidatorFieldsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getValidatorFieldsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorFieldsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorFieldsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorFields()";
            const SELECTOR: [u8; 4] = [76u8, 32u8, 245u8, 16u8];
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
    /**Function with signature `getValidatorFieldsProof()` and selector `0x893893ca`.
```solidity
function getValidatorFieldsProof() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsProofCall {}
    ///Container type for the return parameters of the [`getValidatorFieldsProof()`](getValidatorFieldsProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsProofReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getValidatorFieldsProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getValidatorFieldsProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorFieldsProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorFieldsProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorFieldsProof()";
            const SELECTOR: [u8; 4] = [137u8, 56u8, 147u8, 202u8];
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
    /**Function with signature `getValidatorIndex()` and selector `0x765aa606`.
```solidity
function getValidatorIndex() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorIndexCall {}
    ///Container type for the return parameters of the [`getValidatorIndex()`](getValidatorIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorIndexReturn {
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
            impl ::core::convert::From<getValidatorIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorIndexCall {
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
            impl ::core::convert::From<getValidatorIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorIndex()";
            const SELECTOR: [u8; 4] = [118u8, 90u8, 166u8, 6u8];
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
    /**Function with signature `getValidatorProof()` and selector `0x4c38f913`.
```solidity
function getValidatorProof() external returns (bytes32[46] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorProofCall {}
    ///Container type for the return parameters of the [`getValidatorProof()`](getValidatorProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 46usize],
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
            impl ::core::convert::From<getValidatorProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    46usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 46usize],
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
            impl ::core::convert::From<getValidatorProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    46usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorProof()";
            const SELECTOR: [u8; 4] = [76u8, 56u8, 249u8, 19u8];
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
    /**Function with signature `getValidatorPubkeyHash()` and selector `0xf148082c`.
```solidity
function getValidatorPubkeyHash() external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorPubkeyHashCall {}
    ///Container type for the return parameters of the [`getValidatorPubkeyHash()`](getValidatorPubkeyHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorPubkeyHashReturn {
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
            impl ::core::convert::From<getValidatorPubkeyHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorPubkeyHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorPubkeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<getValidatorPubkeyHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorPubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorPubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorPubkeyHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorPubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorPubkeyHash()";
            const SELECTOR: [u8; 4] = [241u8, 72u8, 8u8, 44u8];
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
    /**Function with signature `getWithdrawalCredentialProof()` and selector `0x2fd1793c`.
```solidity
function getWithdrawalCredentialProof() external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalCredentialProofCall {}
    ///Container type for the return parameters of the [`getWithdrawalCredentialProof()`](getWithdrawalCredentialProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalCredentialProofReturn {
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
            impl ::core::convert::From<getWithdrawalCredentialProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalCredentialProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalCredentialProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<getWithdrawalCredentialProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalCredentialProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalCredentialProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalCredentialProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalCredentialProofReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalCredentialProof()";
            const SELECTOR: [u8; 4] = [47u8, 209u8, 121u8, 60u8];
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
    /**Function with signature `getWithdrawalFields()` and selector `0x950bb682`.
```solidity
function getWithdrawalFields() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalFieldsCall {}
    ///Container type for the return parameters of the [`getWithdrawalFields()`](getWithdrawalFieldsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalFieldsReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getWithdrawalFieldsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalFieldsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalFieldsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getWithdrawalFieldsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalFieldsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalFieldsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalFieldsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalFieldsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalFields()";
            const SELECTOR: [u8; 4] = [149u8, 11u8, 182u8, 130u8];
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
    /**Function with signature `getWithdrawalIndex()` and selector `0x64f38ed7`.
```solidity
function getWithdrawalIndex() external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalIndexCall {}
    ///Container type for the return parameters of the [`getWithdrawalIndex()`](getWithdrawalIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalIndexReturn {
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
            impl ::core::convert::From<getWithdrawalIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalIndexCall {
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
            impl ::core::convert::From<getWithdrawalIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalIndex()";
            const SELECTOR: [u8; 4] = [100u8, 243u8, 142u8, 215u8];
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
    /**Function with signature `getWithdrawalProofCapella()` and selector `0x64c72535`.
```solidity
function getWithdrawalProofCapella() external returns (bytes32[9] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofCapellaCall {}
    ///Container type for the return parameters of the [`getWithdrawalProofCapella()`](getWithdrawalProofCapellaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofCapellaReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 9usize],
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
            impl ::core::convert::From<getWithdrawalProofCapellaCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofCapellaCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofCapellaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    9usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 9usize],
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
            impl ::core::convert::From<getWithdrawalProofCapellaReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofCapellaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofCapellaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalProofCapellaCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalProofCapellaReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    9usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalProofCapella()";
            const SELECTOR: [u8; 4] = [100u8, 199u8, 37u8, 53u8];
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
    /**Function with signature `getWithdrawalProofDeneb()` and selector `0x9acfac29`.
```solidity
function getWithdrawalProofDeneb() external returns (bytes32[10] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofDenebCall {}
    ///Container type for the return parameters of the [`getWithdrawalProofDeneb()`](getWithdrawalProofDenebCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofDenebReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 10usize],
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
            impl ::core::convert::From<getWithdrawalProofDenebCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofDenebCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofDenebCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    10usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 10usize],
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
            impl ::core::convert::From<getWithdrawalProofDenebReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofDenebReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofDenebReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalProofDenebCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalProofDenebReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    10usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalProofDeneb()";
            const SELECTOR: [u8; 4] = [154u8, 207u8, 172u8, 41u8];
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
    /**Function with signature `setJSON(string)` and selector `0x08a4d71f`.
```solidity
function setJSON(string memory path) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setJSONCall {
        pub path: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`setJSON(string)`](setJSONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setJSONReturn {}
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
            impl ::core::convert::From<setJSONCall> for UnderlyingRustTuple<'_> {
                fn from(value: setJSONCall) -> Self {
                    (value.path,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setJSONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { path: tuple.0 }
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
            impl ::core::convert::From<setJSONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setJSONReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setJSONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setJSONCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setJSONReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setJSON(string)";
            const SELECTOR: [u8; 4] = [8u8, 164u8, 215u8, 31u8];
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
                        &self.path,
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
    ///Container for all the [`ProofParsing`](self) function calls.
    pub enum ProofParsingCalls {
        IS_TEST(IS_TESTCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        getBalanceUpdateProof(getBalanceUpdateProofCall),
        getBeaconStateRoot(getBeaconStateRootCall),
        getBlockHeaderProof(getBlockHeaderProofCall),
        getBlockRoot(getBlockRootCall),
        getBlockRootIndex(getBlockRootIndexCall),
        getExecutionPayloadProof(getExecutionPayloadProofCall),
        getExecutionPayloadRoot(getExecutionPayloadRootCall),
        getHistoricalSummaryIndex(getHistoricalSummaryIndexCall),
        getHistoricalSummaryProof(getHistoricalSummaryProofCall),
        getLatestBlockRoot(getLatestBlockRootCall),
        getSlot(getSlotCall),
        getSlotProof(getSlotProofCall),
        getSlotRoot(getSlotRootCall),
        getStateRootProof(getStateRootProofCall),
        getTimestampProofCapella(getTimestampProofCapellaCall),
        getTimestampProofDeneb(getTimestampProofDenebCall),
        getTimestampRoot(getTimestampRootCall),
        getValidatorFields(getValidatorFieldsCall),
        getValidatorFieldsProof(getValidatorFieldsProofCall),
        getValidatorIndex(getValidatorIndexCall),
        getValidatorProof(getValidatorProofCall),
        getValidatorPubkeyHash(getValidatorPubkeyHashCall),
        getWithdrawalCredentialProof(getWithdrawalCredentialProofCall),
        getWithdrawalFields(getWithdrawalFieldsCall),
        getWithdrawalIndex(getWithdrawalIndexCall),
        getWithdrawalProofCapella(getWithdrawalProofCapellaCall),
        getWithdrawalProofDeneb(getWithdrawalProofDenebCall),
        setJSON(setJSONCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
    }
    #[automatically_derived]
    impl ProofParsingCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 164u8, 215u8, 31u8],
            [22u8, 240u8, 113u8, 83u8],
            [23u8, 117u8, 65u8, 252u8],
            [24u8, 170u8, 223u8, 49u8],
            [25u8, 108u8, 223u8, 189u8],
            [30u8, 215u8, 131u8, 28u8],
            [39u8, 83u8, 120u8, 177u8],
            [40u8, 114u8, 226u8, 12u8],
            [47u8, 209u8, 121u8, 60u8],
            [52u8, 227u8, 217u8, 14u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [66u8, 134u8, 71u8, 52u8],
            [76u8, 32u8, 245u8, 16u8],
            [76u8, 56u8, 249u8, 19u8],
            [100u8, 199u8, 37u8, 53u8],
            [100u8, 243u8, 142u8, 215u8],
            [102u8, 217u8, 169u8, 160u8],
            [108u8, 135u8, 124u8, 132u8],
            [118u8, 90u8, 166u8, 6u8],
            [133u8, 34u8, 108u8, 129u8],
            [137u8, 56u8, 147u8, 202u8],
            [145u8, 106u8, 23u8, 198u8],
            [149u8, 11u8, 182u8, 130u8],
            [154u8, 207u8, 172u8, 41u8],
            [157u8, 228u8, 169u8, 179u8],
            [161u8, 87u8, 239u8, 76u8],
            [165u8, 7u8, 116u8, 41u8],
            [179u8, 128u8, 97u8, 191u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [214u8, 70u8, 28u8, 187u8],
            [217u8, 68u8, 71u8, 47u8],
            [219u8, 54u8, 74u8, 64u8],
            [223u8, 24u8, 199u8, 131u8],
            [226u8, 12u8, 159u8, 113u8],
            [241u8, 72u8, 8u8, 44u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProofParsingCalls {
        const NAME: &'static str = "ProofParsingCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 38usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::getBalanceUpdateProof(_) => {
                    <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBeaconStateRoot(_) => {
                    <getBeaconStateRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlockHeaderProof(_) => {
                    <getBlockHeaderProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlockRoot(_) => {
                    <getBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlockRootIndex(_) => {
                    <getBlockRootIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getExecutionPayloadProof(_) => {
                    <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getExecutionPayloadRoot(_) => {
                    <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHistoricalSummaryIndex(_) => {
                    <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHistoricalSummaryProof(_) => {
                    <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestBlockRoot(_) => {
                    <getLatestBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlot(_) => <getSlotCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getSlotProof(_) => {
                    <getSlotProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlotRoot(_) => {
                    <getSlotRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStateRootProof(_) => {
                    <getStateRootProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampProofCapella(_) => {
                    <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampProofDeneb(_) => {
                    <getTimestampProofDenebCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampRoot(_) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorFields(_) => {
                    <getValidatorFieldsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorFieldsProof(_) => {
                    <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorIndex(_) => {
                    <getValidatorIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorProof(_) => {
                    <getValidatorProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorPubkeyHash(_) => {
                    <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalCredentialProof(_) => {
                    <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalFields(_) => {
                    <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalIndex(_) => {
                    <getWithdrawalIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalProofCapella(_) => {
                    <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalProofDeneb(_) => {
                    <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setJSON(_) => <setJSONCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<ProofParsingCalls>] = &[
                {
                    fn setJSON(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <setJSONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::setJSON)
                    }
                    setJSON
                },
                {
                    fn getSlotRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getSlotRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getSlotRoot)
                    }
                    getSlotRoot
                },
                {
                    fn getBeaconStateRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBeaconStateRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBeaconStateRoot)
                    }
                    getBeaconStateRoot
                },
                {
                    fn getLatestBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getLatestBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getLatestBlockRoot)
                    }
                    getLatestBlockRoot
                },
                {
                    fn getTimestampProofDeneb(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampProofDenebCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampProofDeneb)
                    }
                    getTimestampProofDeneb
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn getBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBlockRoot)
                    }
                    getBlockRoot
                },
                {
                    fn getHistoricalSummaryProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getHistoricalSummaryProof)
                    }
                    getHistoricalSummaryProof
                },
                {
                    fn getWithdrawalCredentialProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalCredentialProof)
                    }
                    getWithdrawalCredentialProof
                },
                {
                    fn getStateRootProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getStateRootProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getStateRootProof)
                    }
                    getStateRootProof
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn getBlockRootIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBlockRootIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBlockRootIndex)
                    }
                    getBlockRootIndex
                },
                {
                    fn getValidatorFields(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorFields)
                    }
                    getValidatorFields
                },
                {
                    fn getValidatorProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorProof)
                    }
                    getValidatorProof
                },
                {
                    fn getWithdrawalProofCapella(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalProofCapella)
                    }
                    getWithdrawalProofCapella
                },
                {
                    fn getWithdrawalIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalIndex)
                    }
                    getWithdrawalIndex
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn getSlot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getSlotCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getSlot)
                    }
                    getSlot
                },
                {
                    fn getValidatorIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorIndex)
                    }
                    getValidatorIndex
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn getValidatorFieldsProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorFieldsProof)
                    }
                    getValidatorFieldsProof
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn getWithdrawalFields(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalFields)
                    }
                    getWithdrawalFields
                },
                {
                    fn getWithdrawalProofDeneb(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalProofDeneb)
                    }
                    getWithdrawalProofDeneb
                },
                {
                    fn getSlotProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getSlotProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getSlotProof)
                    }
                    getSlotProof
                },
                {
                    fn getBalanceUpdateProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBalanceUpdateProof)
                    }
                    getBalanceUpdateProof
                },
                {
                    fn getBlockHeaderProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBlockHeaderProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBlockHeaderProof)
                    }
                    getBlockHeaderProof
                },
                {
                    fn getTimestampRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampRoot)
                    }
                    getTimestampRoot
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::failed)
                    }
                    failed
                },
                {
                    fn getHistoricalSummaryIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getHistoricalSummaryIndex)
                    }
                    getHistoricalSummaryIndex
                },
                {
                    fn getExecutionPayloadRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getExecutionPayloadRoot)
                    }
                    getExecutionPayloadRoot
                },
                {
                    fn getExecutionPayloadProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getExecutionPayloadProof)
                    }
                    getExecutionPayloadProof
                },
                {
                    fn getTimestampProofCapella(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampProofCapella)
                    }
                    getTimestampProofCapella
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn getValidatorPubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorPubkeyHash)
                    }
                    getValidatorPubkeyHash
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::IS_TEST)
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::getBalanceUpdateProof(inner) => {
                    <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBeaconStateRoot(inner) => {
                    <getBeaconStateRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlockHeaderProof(inner) => {
                    <getBlockHeaderProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlockRoot(inner) => {
                    <getBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlockRootIndex(inner) => {
                    <getBlockRootIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getExecutionPayloadProof(inner) => {
                    <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getExecutionPayloadRoot(inner) => {
                    <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHistoricalSummaryIndex(inner) => {
                    <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHistoricalSummaryProof(inner) => {
                    <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestBlockRoot(inner) => {
                    <getLatestBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlot(inner) => {
                    <getSlotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getSlotProof(inner) => {
                    <getSlotProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlotRoot(inner) => {
                    <getSlotRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStateRootProof(inner) => {
                    <getStateRootProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampProofCapella(inner) => {
                    <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampProofDeneb(inner) => {
                    <getTimestampProofDenebCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampRoot(inner) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorFields(inner) => {
                    <getValidatorFieldsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorFieldsProof(inner) => {
                    <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorIndex(inner) => {
                    <getValidatorIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorProof(inner) => {
                    <getValidatorProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorPubkeyHash(inner) => {
                    <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalCredentialProof(inner) => {
                    <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalFields(inner) => {
                    <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalIndex(inner) => {
                    <getWithdrawalIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalProofCapella(inner) => {
                    <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalProofDeneb(inner) => {
                    <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setJSON(inner) => {
                    <setJSONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::getBalanceUpdateProof(inner) => {
                    <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBeaconStateRoot(inner) => {
                    <getBeaconStateRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlockHeaderProof(inner) => {
                    <getBlockHeaderProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlockRoot(inner) => {
                    <getBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlockRootIndex(inner) => {
                    <getBlockRootIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getExecutionPayloadProof(inner) => {
                    <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getExecutionPayloadRoot(inner) => {
                    <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHistoricalSummaryIndex(inner) => {
                    <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHistoricalSummaryProof(inner) => {
                    <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestBlockRoot(inner) => {
                    <getLatestBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlot(inner) => {
                    <getSlotCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getSlotProof(inner) => {
                    <getSlotProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlotRoot(inner) => {
                    <getSlotRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStateRootProof(inner) => {
                    <getStateRootProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestampProofCapella(inner) => {
                    <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestampProofDeneb(inner) => {
                    <getTimestampProofDenebCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestampRoot(inner) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorFields(inner) => {
                    <getValidatorFieldsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorFieldsProof(inner) => {
                    <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorIndex(inner) => {
                    <getValidatorIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorProof(inner) => {
                    <getValidatorProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorPubkeyHash(inner) => {
                    <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalCredentialProof(inner) => {
                    <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalFields(inner) => {
                    <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalIndex(inner) => {
                    <getWithdrawalIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalProofCapella(inner) => {
                    <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalProofDeneb(inner) => {
                    <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setJSON(inner) => {
                    <setJSONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
    ///Container for all the [`ProofParsing`](self) events.
    pub enum ProofParsingEvents {
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
    impl ProofParsingEvents {
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
    impl alloy_sol_types::SolEventInterface for ProofParsingEvents {
        const NAME: &'static str = "ProofParsingEvents";
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
    impl alloy_sol_types::private::IntoLogData for ProofParsingEvents {
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
    /**Creates a new wrapper around an on-chain [`ProofParsing`](self) contract instance.

See the [wrapper's documentation](`ProofParsingInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ProofParsingInstance<T, P, N> {
        ProofParsingInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ProofParsingInstance<T, P, N>>,
    > {
        ProofParsingInstance::<T, P, N>::deploy(provider)
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
        ProofParsingInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`ProofParsing`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ProofParsing`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ProofParsingInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ProofParsingInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ProofParsingInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ProofParsingInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ProofParsing`](self) contract instance.

See the [wrapper's documentation](`ProofParsingInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ProofParsingInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> ProofParsingInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ProofParsingInstance<T, P, N> {
            ProofParsingInstance {
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
    > ProofParsingInstance<T, P, N> {
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
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
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
        ///Creates a new call builder for the [`getBalanceUpdateProof`] function.
        pub fn getBalanceUpdateProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBalanceUpdateProofCall, N> {
            self.call_builder(&getBalanceUpdateProofCall {})
        }
        ///Creates a new call builder for the [`getBeaconStateRoot`] function.
        pub fn getBeaconStateRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBeaconStateRootCall, N> {
            self.call_builder(&getBeaconStateRootCall {})
        }
        ///Creates a new call builder for the [`getBlockHeaderProof`] function.
        pub fn getBlockHeaderProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockHeaderProofCall, N> {
            self.call_builder(&getBlockHeaderProofCall {})
        }
        ///Creates a new call builder for the [`getBlockRoot`] function.
        pub fn getBlockRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockRootCall, N> {
            self.call_builder(&getBlockRootCall {})
        }
        ///Creates a new call builder for the [`getBlockRootIndex`] function.
        pub fn getBlockRootIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockRootIndexCall, N> {
            self.call_builder(&getBlockRootIndexCall {})
        }
        ///Creates a new call builder for the [`getExecutionPayloadProof`] function.
        pub fn getExecutionPayloadProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getExecutionPayloadProofCall, N> {
            self.call_builder(&getExecutionPayloadProofCall {})
        }
        ///Creates a new call builder for the [`getExecutionPayloadRoot`] function.
        pub fn getExecutionPayloadRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getExecutionPayloadRootCall, N> {
            self.call_builder(&getExecutionPayloadRootCall {})
        }
        ///Creates a new call builder for the [`getHistoricalSummaryIndex`] function.
        pub fn getHistoricalSummaryIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHistoricalSummaryIndexCall, N> {
            self.call_builder(&getHistoricalSummaryIndexCall {})
        }
        ///Creates a new call builder for the [`getHistoricalSummaryProof`] function.
        pub fn getHistoricalSummaryProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHistoricalSummaryProofCall, N> {
            self.call_builder(&getHistoricalSummaryProofCall {})
        }
        ///Creates a new call builder for the [`getLatestBlockRoot`] function.
        pub fn getLatestBlockRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestBlockRootCall, N> {
            self.call_builder(&getLatestBlockRootCall {})
        }
        ///Creates a new call builder for the [`getSlot`] function.
        pub fn getSlot(&self) -> alloy_contract::SolCallBuilder<T, &P, getSlotCall, N> {
            self.call_builder(&getSlotCall {})
        }
        ///Creates a new call builder for the [`getSlotProof`] function.
        pub fn getSlotProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlotProofCall, N> {
            self.call_builder(&getSlotProofCall {})
        }
        ///Creates a new call builder for the [`getSlotRoot`] function.
        pub fn getSlotRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlotRootCall, N> {
            self.call_builder(&getSlotRootCall {})
        }
        ///Creates a new call builder for the [`getStateRootProof`] function.
        pub fn getStateRootProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStateRootProofCall, N> {
            self.call_builder(&getStateRootProofCall {})
        }
        ///Creates a new call builder for the [`getTimestampProofCapella`] function.
        pub fn getTimestampProofCapella(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampProofCapellaCall, N> {
            self.call_builder(&getTimestampProofCapellaCall {})
        }
        ///Creates a new call builder for the [`getTimestampProofDeneb`] function.
        pub fn getTimestampProofDeneb(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampProofDenebCall, N> {
            self.call_builder(&getTimestampProofDenebCall {})
        }
        ///Creates a new call builder for the [`getTimestampRoot`] function.
        pub fn getTimestampRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampRootCall, N> {
            self.call_builder(&getTimestampRootCall {})
        }
        ///Creates a new call builder for the [`getValidatorFields`] function.
        pub fn getValidatorFields(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorFieldsCall, N> {
            self.call_builder(&getValidatorFieldsCall {})
        }
        ///Creates a new call builder for the [`getValidatorFieldsProof`] function.
        pub fn getValidatorFieldsProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorFieldsProofCall, N> {
            self.call_builder(&getValidatorFieldsProofCall {})
        }
        ///Creates a new call builder for the [`getValidatorIndex`] function.
        pub fn getValidatorIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorIndexCall, N> {
            self.call_builder(&getValidatorIndexCall {})
        }
        ///Creates a new call builder for the [`getValidatorProof`] function.
        pub fn getValidatorProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorProofCall, N> {
            self.call_builder(&getValidatorProofCall {})
        }
        ///Creates a new call builder for the [`getValidatorPubkeyHash`] function.
        pub fn getValidatorPubkeyHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorPubkeyHashCall, N> {
            self.call_builder(&getValidatorPubkeyHashCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalCredentialProof`] function.
        pub fn getWithdrawalCredentialProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalCredentialProofCall, N> {
            self.call_builder(
                &getWithdrawalCredentialProofCall {
                },
            )
        }
        ///Creates a new call builder for the [`getWithdrawalFields`] function.
        pub fn getWithdrawalFields(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalFieldsCall, N> {
            self.call_builder(&getWithdrawalFieldsCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalIndex`] function.
        pub fn getWithdrawalIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalIndexCall, N> {
            self.call_builder(&getWithdrawalIndexCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalProofCapella`] function.
        pub fn getWithdrawalProofCapella(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalProofCapellaCall, N> {
            self.call_builder(&getWithdrawalProofCapellaCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalProofDeneb`] function.
        pub fn getWithdrawalProofDeneb(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalProofDenebCall, N> {
            self.call_builder(&getWithdrawalProofDenebCall {})
        }
        ///Creates a new call builder for the [`setJSON`] function.
        pub fn setJSON(
            &self,
            path: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, setJSONCall, N> {
            self.call_builder(&setJSONCall { path })
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
    > ProofParsingInstance<T, P, N> {
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
