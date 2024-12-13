///Module containing a contract's types and functions.
/**

```solidity
library IIndexRegistry {
    struct OperatorUpdate { uint32 fromBlockNumber; bytes32 operatorId; }
    struct QuorumUpdate { uint32 fromBlockNumber; uint32 numOperators; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IIndexRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct OperatorUpdate { uint32 fromBlockNumber; bytes32 operatorId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorUpdate {
        pub fromBlockNumber: u32,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorUpdate) -> Self {
                (value.fromBlockNumber, value.operatorId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    fromBlockNumber: tuple.0,
                    operatorId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromBlockNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
        impl alloy_sol_types::SolType for OperatorUpdate {
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
        impl alloy_sol_types::SolStruct for OperatorUpdate {
            const NAME: &'static str = "OperatorUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorUpdate(uint32 fromBlockNumber,bytes32 operatorId)",
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
                            &self.fromBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fromBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
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
                    &rust.fromBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
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
    /**```solidity
struct QuorumUpdate { uint32 fromBlockNumber; uint32 numOperators; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumUpdate {
        pub fromBlockNumber: u32,
        pub numOperators: u32,
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
        impl ::core::convert::From<QuorumUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumUpdate) -> Self {
                (value.fromBlockNumber, value.numOperators)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    fromBlockNumber: tuple.0,
                    numOperators: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.numOperators),
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
        impl alloy_sol_types::SolType for QuorumUpdate {
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
        impl alloy_sol_types::SolStruct for QuorumUpdate {
            const NAME: &'static str = "QuorumUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumUpdate(uint32 fromBlockNumber,uint32 numOperators)",
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
                            &self.fromBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numOperators)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fromBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numOperators,
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
                    &rust.fromBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numOperators,
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
    /**Creates a new wrapper around an on-chain [`IIndexRegistry`](self) contract instance.

See the [wrapper's documentation](`IIndexRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IIndexRegistryInstance<T, P, N> {
        IIndexRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IIndexRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IIndexRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IIndexRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IIndexRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IIndexRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IIndexRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IIndexRegistry`](self) contract instance.

See the [wrapper's documentation](`IIndexRegistryInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IIndexRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IIndexRegistryInstance<T, P, N> {
            IIndexRegistryInstance {
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
    > IIndexRegistryInstance<T, P, N> {
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
    > IIndexRegistryInstance<T, P, N> {
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
library IIndexRegistry {
    struct OperatorUpdate {
        uint32 fromBlockNumber;
        bytes32 operatorId;
    }
    struct QuorumUpdate {
        uint32 fromBlockNumber;
        uint32 numOperators;
    }
}

interface IndexRegistry {
    event Initialized(uint8 version);
    event QuorumIndexUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint32 newOperatorIndex);

    constructor(address _registryCoordinator);

    function OPERATOR_DOES_NOT_EXIST_ID() external view returns (bytes32);
    function currentOperatorIndex(uint8, bytes32) external view returns (uint32);
    function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
    function getLatestOperatorUpdate(uint8 quorumNumber, uint32 operatorIndex) external view returns (IIndexRegistry.OperatorUpdate memory);
    function getLatestQuorumUpdate(uint8 quorumNumber) external view returns (IIndexRegistry.QuorumUpdate memory);
    function getOperatorListAtBlockNumber(uint8 quorumNumber, uint32 blockNumber) external view returns (bytes32[] memory);
    function getOperatorUpdateAtIndex(uint8 quorumNumber, uint32 operatorIndex, uint32 arrayIndex) external view returns (IIndexRegistry.OperatorUpdate memory);
    function getQuorumUpdateAtIndex(uint8 quorumNumber, uint32 quorumIndex) external view returns (IIndexRegistry.QuorumUpdate memory);
    function initializeQuorum(uint8 quorumNumber) external;
    function registerOperator(bytes32 operatorId, bytes memory quorumNumbers) external returns (uint32[] memory);
    function registryCoordinator() external view returns (address);
    function totalOperatorsForQuorum(uint8 quorumNumber) external view returns (uint32);
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
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "OPERATOR_DOES_NOT_EXIST_ID",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentOperatorIndex",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "deregisterOperator",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLatestOperatorUpdate",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistry.OperatorUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestQuorumUpdate",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistry.QuorumUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numOperators",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorListAtBlockNumber",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "arrayIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistry.OperatorUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "quorumIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IIndexRegistry.QuorumUpdate",
        "components": [
          {
            "name": "fromBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "numOperators",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registryCoordinator",
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
    "name": "totalOperatorsForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "QuorumIndexUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "newOperatorIndex",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
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
pub mod IndexRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561000f575f5ffd5b50604051611f1f380380611f1f833981810160405281019061003191906101a9565b808073ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff168152505061007461007b60201b60201c565b50506102a6565b5f60019054906101000a900460ff16156100ca576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016100c190610254565b60405180910390fd5b60ff80165f5f9054906101000a900460ff1660ff1610156101385760ff5f5f6101000a81548160ff021916908360ff1602179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860ff60405161012f919061028d565b60405180910390a15b565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101678261013e565b9050919050565b5f6101788261015d565b9050919050565b6101888161016e565b8114610192575f5ffd5b50565b5f815190506101a38161017f565b92915050565b5f602082840312156101be576101bd61013a565b5b5f6101cb84828501610195565b91505092915050565b5f82825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320696e6974695f8201527f616c697a696e6700000000000000000000000000000000000000000000000000602082015250565b5f61023e6027836101d4565b9150610249826101e4565b604082019050919050565b5f6020820190508181035f83015261026b81610232565b9050919050565b5f60ff82169050919050565b61028781610272565b82525050565b5f6020820190506102a05f83018461027e565b92915050565b608051611c5a6102c55f395f818161060401526109b30152611c5a5ff3fe608060405234801561000f575f5ffd5b50600436106100b1575f3560e01c8063890262451161006f57806389026245146101af578063a48bb0ac146101df578063bd29b8cd1461020f578063caa3cd761461022b578063e2e6858014610249578063f341092214610279576100b1565b8062bff04d146100b557806312d1d74d146100e557806326d941f2146101155780632ed583e5146101315780636d14a987146101615780638121906f1461017f575b5f5ffd5b6100cf60048036038101906100ca91906112a9565b6102a9565b6040516100dc91906113cc565b60405180910390f35b6100ff60048036038101906100fa919061144c565b610402565b60405161010c91906114c6565b60405180910390f35b61012f600480360381019061012a91906114df565b610455565b005b61014b6004803603810190610146919061150a565b610562565b60405161015891906114c6565b60405180910390f35b610169610602565b6040516101769190611599565b60405180910390f35b610199600480360381019061019491906114df565b610626565b6040516101a691906115df565b60405180910390f35b6101c960048036038101906101c4919061144c565b610692565b6040516101d691906116a0565b60405180910390f35b6101f960048036038101906101f4919061144c565b6107a7565b60405161020691906115df565b60405180910390f35b610229600480360381019061022491906112a9565b610842565b005b61023361095a565b60405161024091906116cf565b60405180910390f35b610263600480360381019061025e91906116e8565b610960565b6040516102709190611735565b60405180910390f35b610293600480360381019061028e91906114df565b61098d565b6040516102a09190611735565b60405180910390f35b60606102b36109b1565b5f8383905067ffffffffffffffff8111156102d1576102d061174e565b5b6040519080825280602002602001820160405280156102ff5781602001602082028036833780820191505090505b5090505f5f90505b848490508110156103f6575f8585838181106103265761032561177b565b5b9050013560f81c60f81b60f81c90505f60035f8360ff1660ff1681526020019081526020015f208054905090505f8103610395576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161038c90611828565b60405180910390fd5b5f61039f83610a41565b90506103b889846001846103b39190611873565b610b80565b808585815181106103cc576103cb61177b565b5b602002602001019063ffffffff16908163ffffffff16815250505050508080600101915050610307565b50809150509392505050565b61040a6111cb565b6104148383610c1d565b6040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff168152602001600182015481525050905092915050565b61045d6109b1565b5f60035f8360ff1660ff1681526020019081526020015f2080549050146104b9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b09061191a565b60405180910390fd5b60035f8260ff1660ff1681526020019081526020015f2060405180604001604052804363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff160217905550505050565b61056a6111cb565b60025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f208263ffffffff16815481106105b4576105b361177b565b5b905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff16815260200160018201548152505090509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b61062e6111e9565b61063782610cbc565b6040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff16815250509050919050565b60605f61069f8484610d20565b90505f8163ffffffff1667ffffffffffffffff8111156106c2576106c161174e565b5b6040519080825280602002602001820160405280156106f05781602001602082028036833780820191505090505b5090505f5f90505b8263ffffffff1681101561079b57610711868287610e5b565b8282815181106107245761072361177b565b5b6020026020010181815250505f5f1b8282815181106107465761074561177b565b5b60200260200101510361078e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610785906119ce565b60405180910390fd5b80806001019150506106f8565b50809250505092915050565b6107af6111e9565b60035f8460ff1660ff1681526020019081526020015f208263ffffffff16815481106107de576107dd61177b565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681525050905092915050565b61084a6109b1565b5f5f90505b82829050811015610954575f83838381811061086e5761086d61177b565b5b9050013560f81c60f81b60f81c90505f60035f8360ff1660ff1681526020019081526020015f208054905090505f81036108dd576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108d490611828565b60405180910390fd5b5f60015f8460ff1660ff1681526020019081526020015f205f8881526020019081526020015f205f9054906101000a900463ffffffff1690505f61092084610f82565b90505f61092d8583610fc7565b905080891461094257610941818685610b80565b5b5050505050808060010191505061084f565b50505050565b5f5f1b81565b6001602052815f5260405f20602052805f5260405f205f915091509054906101000a900463ffffffff1681565b5f61099782610cbc565b5f0160049054906101000a900463ffffffff169050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610a3f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a3690611a82565b60405180910390fd5b565b5f5f610a4c83610cbc565b90505f6001825f0160049054906101000a900463ffffffff16610a6f9190611aa0565b9050610a7c848383610ff6565b5f60025f8660ff1660ff1681526020019081526020015f205f600184610aa29190611873565b63ffffffff1663ffffffff1681526020019081526020015f208054905003610b765760025f8560ff1660ff1681526020019081526020015f205f600183610ae99190611873565b63ffffffff1663ffffffff1681526020019081526020015f2060405180604001604052804363ffffffff1681526020015f5f1b815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151816001015550505b8092505050919050565b5f610b8b8383610c1d565b9050610b99838383876110ee565b8160015f8560ff1660ff1681526020019081526020015f205f8681526020019081526020015f205f6101000a81548163ffffffff021916908363ffffffff160217905550837f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f68484604051610c0f929190611ae6565b60405180910390a250505050565b5f5f60025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f2080549050905060025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f20600182610c969190611b16565b81548110610ca757610ca661177b565b5b905f5260205f20906002020191505092915050565b5f5f60035f8460ff1660ff1681526020019081526020015f2080549050905060035f8460ff1660ff1681526020019081526020015f20600182610cff9190611b16565b81548110610d1057610d0f61177b565b5b905f5260205f2001915050919050565b5f5f60035f8560ff1660ff1681526020019081526020015f208054905090505f8190505b5f811115610e19575f60035f8760ff1660ff1681526020019081526020015f20600183610d719190611b16565b81548110610d8257610d8161177b565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff168152505090508463ffffffff16815f015163ffffffff1611610e055780602001519350505050610e55565b508080610e1190611b49565b915050610d44565b506040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e4c90611c06565b60405180910390fd5b92915050565b5f5f60025f8660ff1660ff1681526020019081526020015f205f8563ffffffff1663ffffffff1681526020019081526020015f208054905090505f8190505b5f811115610f73575f60025f8860ff1660ff1681526020019081526020015f205f8763ffffffff1663ffffffff1681526020019081526020015f20600183610ee29190611b16565b81548110610ef357610ef261177b565b5b905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff16815260200160018201548152505090508463ffffffff16815f015163ffffffff1611610f5f5780602001519350505050610f7b565b508080610f6b90611b49565b915050610e9a565b505f5f1b9150505b9392505050565b5f5f610f8d83610cbc565b90505f6001825f0160049054906101000a900463ffffffff16610fb09190611873565b9050610fbd848383610ff6565b8092505050919050565b5f5f610fd38484610c1d565b90505f81600101549050610feb8585845f5f1b6110ee565b809250505092915050565b4363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff16036110425780825f0160046101000a81548163ffffffff021916908363ffffffff1602179055506110e9565b60035f8460ff1660ff1681526020019081526020015f2060405180604001604052804363ffffffff1681526020018363ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff16021790555050505b505050565b4363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff1603611121578082600101819055506111c5565b60025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f2060405180604001604052804363ffffffff16815260200183815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151816001015550505b50505050565b60405180604001604052805f63ffffffff1681526020015f81525090565b60405180604001604052805f63ffffffff1681526020015f63ffffffff1681525090565b5f5ffd5b5f5ffd5b5f819050919050565b61122781611215565b8114611231575f5ffd5b50565b5f813590506112428161121e565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261126957611268611248565b5b8235905067ffffffffffffffff8111156112865761128561124c565b5b6020830191508360018202830111156112a2576112a1611250565b5b9250929050565b5f5f5f604084860312156112c0576112bf61120d565b5b5f6112cd86828701611234565b935050602084013567ffffffffffffffff8111156112ee576112ed611211565b5b6112fa86828701611254565b92509250509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f63ffffffff82169050919050565b6113478161132f565b82525050565b5f611358838361133e565b60208301905092915050565b5f602082019050919050565b5f61137a82611306565b6113848185611310565b935061138f83611320565b805f5b838110156113bf5781516113a6888261134d565b97506113b183611364565b925050600181019050611392565b5085935050505092915050565b5f6020820190508181035f8301526113e48184611370565b905092915050565b5f60ff82169050919050565b611401816113ec565b811461140b575f5ffd5b50565b5f8135905061141c816113f8565b92915050565b61142b8161132f565b8114611435575f5ffd5b50565b5f8135905061144681611422565b92915050565b5f5f604083850312156114625761146161120d565b5b5f61146f8582860161140e565b925050602061148085828601611438565b9150509250929050565b61149381611215565b82525050565b604082015f8201516114ad5f85018261133e565b5060208201516114c0602085018261148a565b50505050565b5f6040820190506114d95f830184611499565b92915050565b5f602082840312156114f4576114f361120d565b5b5f6115018482850161140e565b91505092915050565b5f5f5f606084860312156115215761152061120d565b5b5f61152e8682870161140e565b935050602061153f86828701611438565b925050604061155086828701611438565b9150509250925092565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6115838261155a565b9050919050565b61159381611579565b82525050565b5f6020820190506115ac5f83018461158a565b92915050565b604082015f8201516115c65f85018261133e565b5060208201516115d9602085018261133e565b50505050565b5f6040820190506115f25f8301846115b2565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f61162c838361148a565b60208301905092915050565b5f602082019050919050565b5f61164e826115f8565b6116588185611602565b935061166383611612565b805f5b8381101561169357815161167a8882611621565b975061168583611638565b925050600181019050611666565b5085935050505092915050565b5f6020820190508181035f8301526116b88184611644565b905092915050565b6116c981611215565b82525050565b5f6020820190506116e25f8301846116c0565b92915050565b5f5f604083850312156116fe576116fd61120d565b5b5f61170b8582860161140e565b925050602061171c85828601611234565b9150509250929050565b61172f8161132f565b82525050565b5f6020820190506117485f830184611726565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b7f496e64657852656769737472792e72656769737465724f70657261746f723a205f8201527f71756f72756d20646f6573206e6f742065786973740000000000000000000000602082015250565b5f6118126035836117a8565b915061181d826117b8565b604082019050919050565b5f6020820190508181035f83015261183f81611806565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61187d8261132f565b91506118888361132f565b9250828203905063ffffffff8111156118a4576118a3611846565b5b92915050565b7f496e64657852656769737472792e63726561746551756f72756d3a2071756f725f8201527f756d20616c726561647920657869737473000000000000000000000000000000602082015250565b5f6119046031836117a8565b915061190f826118aa565b604082019050919050565b5f6020820190508181035f830152611931816118f8565b9050919050565b7f496e64657852656769737472792e6765744f70657261746f724c6973744174425f8201527f6c6f636b4e756d6265723a206f70657261746f7220646f6573206e6f7420657860208201527f6973742061742074686520676976656e20626c6f636b206e756d626572000000604082015250565b5f6119b8605d836117a8565b91506119c382611938565b606082019050919050565b5f6020820190508181035f8301526119e5816119ac565b9050919050565b7f496e64657852656769737472792e5f636865636b5265676973747279436f6f725f8201527f64696e61746f723a2063616c6c6572206973206e6f742074686520726567697360208201527f74727920636f6f7264696e61746f720000000000000000000000000000000000604082015250565b5f611a6c604f836117a8565b9150611a77826119ec565b606082019050919050565b5f6020820190508181035f830152611a9981611a60565b9050919050565b5f611aaa8261132f565b9150611ab58361132f565b9250828201905063ffffffff811115611ad157611ad0611846565b5b92915050565b611ae0816113ec565b82525050565b5f604082019050611af95f830185611ad7565b611b066020830184611726565b9392505050565b5f819050919050565b5f611b2082611b0d565b9150611b2b83611b0d565b9250828203905081811115611b4357611b42611846565b5b92915050565b5f611b5382611b0d565b91505f8203611b6557611b64611846565b5b600182039050919050565b7f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c5f8201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060208201527f617420676976656e20626c6f636b206e756d6265720000000000000000000000604082015250565b5f611bf06055836117a8565b9150611bfb82611b70565b606082019050919050565b5f6020820190508181035f830152611c1d81611be4565b905091905056fea26469706673582212209f760e1941a5da3a38b510b4b789fecd0a4df76d61939ddb0518f3f8aa71977364736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x1F\x1F8\x03\x80a\x1F\x1F\x839\x81\x81\x01`@R\x81\x01\x90a\x001\x91\x90a\x01\xA9V[\x80\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\0ta\0{` \x1B` \x1CV[PPa\x02\xA6V[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\0\xCAW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0\xC1\x90a\x02TV[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10\x15a\x018W`\xFF__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\xFF`@Qa\x01/\x91\x90a\x02\x8DV[`@Q\x80\x91\x03\x90\xA1[V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01g\x82a\x01>V[\x90P\x91\x90PV[_a\x01x\x82a\x01]V[\x90P\x91\x90PV[a\x01\x88\x81a\x01nV[\x81\x14a\x01\x92W__\xFD[PV[_\x81Q\x90Pa\x01\xA3\x81a\x01\x7FV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x01\xBEWa\x01\xBDa\x01:V[[_a\x01\xCB\x84\x82\x85\x01a\x01\x95V[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is initi_\x82\x01R\x7Falizing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x02>`'\x83a\x01\xD4V[\x91Pa\x02I\x82a\x01\xE4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x02k\x81a\x022V[\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x02\x87\x81a\x02rV[\x82RPPV[_` \x82\x01\x90Pa\x02\xA0_\x83\x01\x84a\x02~V[\x92\x91PPV[`\x80Qa\x1CZa\x02\xC5_9_\x81\x81a\x06\x04\x01Ra\t\xB3\x01Ra\x1CZ_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80c\x89\x02bE\x11a\0oW\x80c\x89\x02bE\x14a\x01\xAFW\x80c\xA4\x8B\xB0\xAC\x14a\x01\xDFW\x80c\xBD)\xB8\xCD\x14a\x02\x0FW\x80c\xCA\xA3\xCDv\x14a\x02+W\x80c\xE2\xE6\x85\x80\x14a\x02IW\x80c\xF3A\t\"\x14a\x02yWa\0\xB1V[\x80b\xBF\xF0M\x14a\0\xB5W\x80c\x12\xD1\xD7M\x14a\0\xE5W\x80c&\xD9A\xF2\x14a\x01\x15W\x80c.\xD5\x83\xE5\x14a\x011W\x80cm\x14\xA9\x87\x14a\x01aW\x80c\x81!\x90o\x14a\x01\x7FW[__\xFD[a\0\xCF`\x04\x806\x03\x81\x01\x90a\0\xCA\x91\x90a\x12\xA9V[a\x02\xA9V[`@Qa\0\xDC\x91\x90a\x13\xCCV[`@Q\x80\x91\x03\x90\xF3[a\0\xFF`\x04\x806\x03\x81\x01\x90a\0\xFA\x91\x90a\x14LV[a\x04\x02V[`@Qa\x01\x0C\x91\x90a\x14\xC6V[`@Q\x80\x91\x03\x90\xF3[a\x01/`\x04\x806\x03\x81\x01\x90a\x01*\x91\x90a\x14\xDFV[a\x04UV[\0[a\x01K`\x04\x806\x03\x81\x01\x90a\x01F\x91\x90a\x15\nV[a\x05bV[`@Qa\x01X\x91\x90a\x14\xC6V[`@Q\x80\x91\x03\x90\xF3[a\x01ia\x06\x02V[`@Qa\x01v\x91\x90a\x15\x99V[`@Q\x80\x91\x03\x90\xF3[a\x01\x99`\x04\x806\x03\x81\x01\x90a\x01\x94\x91\x90a\x14\xDFV[a\x06&V[`@Qa\x01\xA6\x91\x90a\x15\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x01\xC9`\x04\x806\x03\x81\x01\x90a\x01\xC4\x91\x90a\x14LV[a\x06\x92V[`@Qa\x01\xD6\x91\x90a\x16\xA0V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9`\x04\x806\x03\x81\x01\x90a\x01\xF4\x91\x90a\x14LV[a\x07\xA7V[`@Qa\x02\x06\x91\x90a\x15\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x02)`\x04\x806\x03\x81\x01\x90a\x02$\x91\x90a\x12\xA9V[a\x08BV[\0[a\x023a\tZV[`@Qa\x02@\x91\x90a\x16\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x02c`\x04\x806\x03\x81\x01\x90a\x02^\x91\x90a\x16\xE8V[a\t`V[`@Qa\x02p\x91\x90a\x175V[`@Q\x80\x91\x03\x90\xF3[a\x02\x93`\x04\x806\x03\x81\x01\x90a\x02\x8E\x91\x90a\x14\xDFV[a\t\x8DV[`@Qa\x02\xA0\x91\x90a\x175V[`@Q\x80\x91\x03\x90\xF3[``a\x02\xB3a\t\xB1V[_\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xD1Wa\x02\xD0a\x17NV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xFFW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x84\x84\x90P\x81\x10\x15a\x03\xF6W_\x85\x85\x83\x81\x81\x10a\x03&Wa\x03%a\x17{V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x03\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x8C\x90a\x18(V[`@Q\x80\x91\x03\x90\xFD[_a\x03\x9F\x83a\nAV[\x90Pa\x03\xB8\x89\x84`\x01\x84a\x03\xB3\x91\x90a\x18sV[a\x0B\x80V[\x80\x85\x85\x81Q\x81\x10a\x03\xCCWa\x03\xCBa\x17{V[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPPPP\x80\x80`\x01\x01\x91PPa\x03\x07V[P\x80\x91PP\x93\x92PPPV[a\x04\na\x11\xCBV[a\x04\x14\x83\x83a\x0C\x1DV[`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x92\x91PPV[a\x04]a\t\xB1V[_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14a\x04\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB0\x90a\x19\x1AV[`@Q\x80\x91\x03\x90\xFD[`\x03_\x82`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[a\x05ja\x11\xCBV[`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x05\xB4Wa\x05\xB3a\x17{V[[\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06.a\x11\xE9V[a\x067\x82a\x0C\xBCV[`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[``_a\x06\x9F\x84\x84a\r V[\x90P_\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC2Wa\x06\xC1a\x17NV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xF0W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07\x9BWa\x07\x11\x86\x82\x87a\x0E[V[\x82\x82\x81Q\x81\x10a\x07$Wa\x07#a\x17{V[[` \x02` \x01\x01\x81\x81RPP__\x1B\x82\x82\x81Q\x81\x10a\x07FWa\x07Ea\x17{V[[` \x02` \x01\x01Q\x03a\x07\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x85\x90a\x19\xCEV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x06\xF8V[P\x80\x92PPP\x92\x91PPV[a\x07\xAFa\x11\xE9V[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x07\xDEWa\x07\xDDa\x17{V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x08Ja\t\xB1V[__\x90P[\x82\x82\x90P\x81\x10\x15a\tTW_\x83\x83\x83\x81\x81\x10a\x08nWa\x08ma\x17{V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x08\xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD4\x90a\x18(V[`@Q\x80\x91\x03\x90\xFD[_`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P_a\t \x84a\x0F\x82V[\x90P_a\t-\x85\x83a\x0F\xC7V[\x90P\x80\x89\x14a\tBWa\tA\x81\x86\x85a\x0B\x80V[[PPPPP\x80\x80`\x01\x01\x91PPa\x08OV[PPPPV[__\x1B\x81V[`\x01` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91P\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[_a\t\x97\x82a\x0C\xBCV[_\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n6\x90a\x1A\x82V[`@Q\x80\x91\x03\x90\xFD[V[__a\nL\x83a\x0C\xBCV[\x90P_`\x01\x82_\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\no\x91\x90a\x1A\xA0V[\x90Pa\n|\x84\x83\x83a\x0F\xF6V[_`\x02_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x01\x84a\n\xA2\x91\x90a\x18sV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x03a\x0BvW`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x01\x83a\n\xE9\x91\x90a\x18sV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01__\x1B\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01UPP[\x80\x92PPP\x91\x90PV[_a\x0B\x8B\x83\x83a\x0C\x1DV[\x90Pa\x0B\x99\x83\x83\x83\x87a\x10\xEEV[\x81`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x86\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x84\x84`@Qa\x0C\x0F\x92\x91\x90a\x1A\xE6V[`@Q\x80\x91\x03\x90\xA2PPPPV[__`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x82a\x0C\x96\x91\x90a\x1B\x16V[\x81T\x81\x10a\x0C\xA7Wa\x0C\xA6a\x17{V[[\x90_R` _ \x90`\x02\x02\x01\x91PP\x92\x91PPV[__`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x82a\x0C\xFF\x91\x90a\x1B\x16V[\x81T\x81\x10a\r\x10Wa\r\x0Fa\x17{V[[\x90_R` _ \x01\x91PP\x91\x90PV[__`\x03_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a\x0E\x19W_`\x03_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\rq\x91\x90a\x1B\x16V[\x81T\x81\x10a\r\x82Wa\r\x81a\x17{V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x84c\xFF\xFF\xFF\xFF\x16\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0E\x05W\x80` \x01Q\x93PPPPa\x0EUV[P\x80\x80a\x0E\x11\x90a\x1BIV[\x91PPa\rDV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EL\x90a\x1C\x06V[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[__`\x02_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a\x0FsW_`\x02_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x87c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\x0E\xE2\x91\x90a\x1B\x16V[\x81T\x81\x10a\x0E\xF3Wa\x0E\xF2a\x17{V[[\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x84c\xFF\xFF\xFF\xFF\x16\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0F_W\x80` \x01Q\x93PPPPa\x0F{V[P\x80\x80a\x0Fk\x90a\x1BIV[\x91PPa\x0E\x9AV[P__\x1B\x91PP[\x93\x92PPPV[__a\x0F\x8D\x83a\x0C\xBCV[\x90P_`\x01\x82_\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\x0F\xB0\x91\x90a\x18sV[\x90Pa\x0F\xBD\x84\x83\x83a\x0F\xF6V[\x80\x92PPP\x91\x90PV[__a\x0F\xD3\x84\x84a\x0C\x1DV[\x90P_\x81`\x01\x01T\x90Pa\x0F\xEB\x85\x85\x84__\x1Ba\x10\xEEV[\x80\x92PPP\x92\x91PPV[Cc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x10BW\x80\x82_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x10\xE9V[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[PPPV[Cc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x11!W\x80\x82`\x01\x01\x81\x90UPa\x11\xC5V[`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01UPP[PPPPV[`@Q\x80`@\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x12'\x81a\x12\x15V[\x81\x14a\x121W__\xFD[PV[_\x815\x90Pa\x12B\x81a\x12\x1EV[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a\x12iWa\x12ha\x12HV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x86Wa\x12\x85a\x12LV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x12\xA2Wa\x12\xA1a\x12PV[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a\x12\xC0Wa\x12\xBFa\x12\rV[[_a\x12\xCD\x86\x82\x87\x01a\x124V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xEEWa\x12\xEDa\x12\x11V[[a\x12\xFA\x86\x82\x87\x01a\x12TV[\x92P\x92PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x13G\x81a\x13/V[\x82RPPV[_a\x13X\x83\x83a\x13>V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x13z\x82a\x13\x06V[a\x13\x84\x81\x85a\x13\x10V[\x93Pa\x13\x8F\x83a\x13 V[\x80_[\x83\x81\x10\x15a\x13\xBFW\x81Qa\x13\xA6\x88\x82a\x13MV[\x97Pa\x13\xB1\x83a\x13dV[\x92PP`\x01\x81\x01\x90Pa\x13\x92V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x13\xE4\x81\x84a\x13pV[\x90P\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x14\x01\x81a\x13\xECV[\x81\x14a\x14\x0BW__\xFD[PV[_\x815\x90Pa\x14\x1C\x81a\x13\xF8V[\x92\x91PPV[a\x14+\x81a\x13/V[\x81\x14a\x145W__\xFD[PV[_\x815\x90Pa\x14F\x81a\x14\"V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x14bWa\x14aa\x12\rV[[_a\x14o\x85\x82\x86\x01a\x14\x0EV[\x92PP` a\x14\x80\x85\x82\x86\x01a\x148V[\x91PP\x92P\x92\x90PV[a\x14\x93\x81a\x12\x15V[\x82RPPV[`@\x82\x01_\x82\x01Qa\x14\xAD_\x85\x01\x82a\x13>V[P` \x82\x01Qa\x14\xC0` \x85\x01\x82a\x14\x8AV[PPPPV[_`@\x82\x01\x90Pa\x14\xD9_\x83\x01\x84a\x14\x99V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xF4Wa\x14\xF3a\x12\rV[[_a\x15\x01\x84\x82\x85\x01a\x14\x0EV[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x15!Wa\x15 a\x12\rV[[_a\x15.\x86\x82\x87\x01a\x14\x0EV[\x93PP` a\x15?\x86\x82\x87\x01a\x148V[\x92PP`@a\x15P\x86\x82\x87\x01a\x148V[\x91PP\x92P\x92P\x92V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x15\x83\x82a\x15ZV[\x90P\x91\x90PV[a\x15\x93\x81a\x15yV[\x82RPPV[_` \x82\x01\x90Pa\x15\xAC_\x83\x01\x84a\x15\x8AV[\x92\x91PPV[`@\x82\x01_\x82\x01Qa\x15\xC6_\x85\x01\x82a\x13>V[P` \x82\x01Qa\x15\xD9` \x85\x01\x82a\x13>V[PPPPV[_`@\x82\x01\x90Pa\x15\xF2_\x83\x01\x84a\x15\xB2V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\x16,\x83\x83a\x14\x8AV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x16N\x82a\x15\xF8V[a\x16X\x81\x85a\x16\x02V[\x93Pa\x16c\x83a\x16\x12V[\x80_[\x83\x81\x10\x15a\x16\x93W\x81Qa\x16z\x88\x82a\x16!V[\x97Pa\x16\x85\x83a\x168V[\x92PP`\x01\x81\x01\x90Pa\x16fV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x16\xB8\x81\x84a\x16DV[\x90P\x92\x91PPV[a\x16\xC9\x81a\x12\x15V[\x82RPPV[_` \x82\x01\x90Pa\x16\xE2_\x83\x01\x84a\x16\xC0V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x16\xFEWa\x16\xFDa\x12\rV[[_a\x17\x0B\x85\x82\x86\x01a\x14\x0EV[\x92PP` a\x17\x1C\x85\x82\x86\x01a\x124V[\x91PP\x92P\x92\x90PV[a\x17/\x81a\x13/V[\x82RPPV[_` \x82\x01\x90Pa\x17H_\x83\x01\x84a\x17&V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FIndexRegistry.registerOperator: _\x82\x01R\x7Fquorum does not exist\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x18\x12`5\x83a\x17\xA8V[\x91Pa\x18\x1D\x82a\x17\xB8V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x18?\x81a\x18\x06V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x18}\x82a\x13/V[\x91Pa\x18\x88\x83a\x13/V[\x92P\x82\x82\x03\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xA4Wa\x18\xA3a\x18FV[[\x92\x91PPV[\x7FIndexRegistry.createQuorum: quor_\x82\x01R\x7Fum already exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x19\x04`1\x83a\x17\xA8V[\x91Pa\x19\x0F\x82a\x18\xAAV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x191\x81a\x18\xF8V[\x90P\x91\x90PV[\x7FIndexRegistry.getOperatorListAtB_\x82\x01R\x7FlockNumber: operator does not ex` \x82\x01R\x7Fist at the given block number\0\0\0`@\x82\x01RPV[_a\x19\xB8`]\x83a\x17\xA8V[\x91Pa\x19\xC3\x82a\x198V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19\xE5\x81a\x19\xACV[\x90P\x91\x90PV[\x7FIndexRegistry._checkRegistryCoor_\x82\x01R\x7Fdinator: caller is not the regis` \x82\x01R\x7Ftry coordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1Al`O\x83a\x17\xA8V[\x91Pa\x1Aw\x82a\x19\xECV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1A\x99\x81a\x1A`V[\x90P\x91\x90PV[_a\x1A\xAA\x82a\x13/V[\x91Pa\x1A\xB5\x83a\x13/V[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xD1Wa\x1A\xD0a\x18FV[[\x92\x91PPV[a\x1A\xE0\x81a\x13\xECV[\x82RPPV[_`@\x82\x01\x90Pa\x1A\xF9_\x83\x01\x85a\x1A\xD7V[a\x1B\x06` \x83\x01\x84a\x17&V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x1B \x82a\x1B\rV[\x91Pa\x1B+\x83a\x1B\rV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1BCWa\x1BBa\x18FV[[\x92\x91PPV[_a\x1BS\x82a\x1B\rV[\x91P_\x82\x03a\x1BeWa\x1Bda\x18FV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FIndexRegistry._operatorCountAtBl_\x82\x01R\x7FockNumber: quorum did not exist ` \x82\x01R\x7Fat given block number\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1B\xF0`U\x83a\x17\xA8V[\x91Pa\x1B\xFB\x82a\x1BpV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C\x1D\x81a\x1B\xE4V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x9Fv\x0E\x19A\xA5\xDA:8\xB5\x10\xB4\xB7\x89\xFE\xCD\nM\xF7ma\x93\x9D\xDB\x05\x18\xF3\xF8\xAAq\x97sdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100b1575f3560e01c8063890262451161006f57806389026245146101af578063a48bb0ac146101df578063bd29b8cd1461020f578063caa3cd761461022b578063e2e6858014610249578063f341092214610279576100b1565b8062bff04d146100b557806312d1d74d146100e557806326d941f2146101155780632ed583e5146101315780636d14a987146101615780638121906f1461017f575b5f5ffd5b6100cf60048036038101906100ca91906112a9565b6102a9565b6040516100dc91906113cc565b60405180910390f35b6100ff60048036038101906100fa919061144c565b610402565b60405161010c91906114c6565b60405180910390f35b61012f600480360381019061012a91906114df565b610455565b005b61014b6004803603810190610146919061150a565b610562565b60405161015891906114c6565b60405180910390f35b610169610602565b6040516101769190611599565b60405180910390f35b610199600480360381019061019491906114df565b610626565b6040516101a691906115df565b60405180910390f35b6101c960048036038101906101c4919061144c565b610692565b6040516101d691906116a0565b60405180910390f35b6101f960048036038101906101f4919061144c565b6107a7565b60405161020691906115df565b60405180910390f35b610229600480360381019061022491906112a9565b610842565b005b61023361095a565b60405161024091906116cf565b60405180910390f35b610263600480360381019061025e91906116e8565b610960565b6040516102709190611735565b60405180910390f35b610293600480360381019061028e91906114df565b61098d565b6040516102a09190611735565b60405180910390f35b60606102b36109b1565b5f8383905067ffffffffffffffff8111156102d1576102d061174e565b5b6040519080825280602002602001820160405280156102ff5781602001602082028036833780820191505090505b5090505f5f90505b848490508110156103f6575f8585838181106103265761032561177b565b5b9050013560f81c60f81b60f81c90505f60035f8360ff1660ff1681526020019081526020015f208054905090505f8103610395576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161038c90611828565b60405180910390fd5b5f61039f83610a41565b90506103b889846001846103b39190611873565b610b80565b808585815181106103cc576103cb61177b565b5b602002602001019063ffffffff16908163ffffffff16815250505050508080600101915050610307565b50809150509392505050565b61040a6111cb565b6104148383610c1d565b6040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff168152602001600182015481525050905092915050565b61045d6109b1565b5f60035f8360ff1660ff1681526020019081526020015f2080549050146104b9576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104b09061191a565b60405180910390fd5b60035f8260ff1660ff1681526020019081526020015f2060405180604001604052804363ffffffff1681526020015f63ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff160217905550505050565b61056a6111cb565b60025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f208263ffffffff16815481106105b4576105b361177b565b5b905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff16815260200160018201548152505090509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b61062e6111e9565b61063782610cbc565b6040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff16815250509050919050565b60605f61069f8484610d20565b90505f8163ffffffff1667ffffffffffffffff8111156106c2576106c161174e565b5b6040519080825280602002602001820160405280156106f05781602001602082028036833780820191505090505b5090505f5f90505b8263ffffffff1681101561079b57610711868287610e5b565b8282815181106107245761072361177b565b5b6020026020010181815250505f5f1b8282815181106107465761074561177b565b5b60200260200101510361078e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610785906119ce565b60405180910390fd5b80806001019150506106f8565b50809250505092915050565b6107af6111e9565b60035f8460ff1660ff1681526020019081526020015f208263ffffffff16815481106107de576107dd61177b565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681525050905092915050565b61084a6109b1565b5f5f90505b82829050811015610954575f83838381811061086e5761086d61177b565b5b9050013560f81c60f81b60f81c90505f60035f8360ff1660ff1681526020019081526020015f208054905090505f81036108dd576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016108d490611828565b60405180910390fd5b5f60015f8460ff1660ff1681526020019081526020015f205f8881526020019081526020015f205f9054906101000a900463ffffffff1690505f61092084610f82565b90505f61092d8583610fc7565b905080891461094257610941818685610b80565b5b5050505050808060010191505061084f565b50505050565b5f5f1b81565b6001602052815f5260405f20602052805f5260405f205f915091509054906101000a900463ffffffff1681565b5f61099782610cbc565b5f0160049054906101000a900463ffffffff169050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610a3f576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a3690611a82565b60405180910390fd5b565b5f5f610a4c83610cbc565b90505f6001825f0160049054906101000a900463ffffffff16610a6f9190611aa0565b9050610a7c848383610ff6565b5f60025f8660ff1660ff1681526020019081526020015f205f600184610aa29190611873565b63ffffffff1663ffffffff1681526020019081526020015f208054905003610b765760025f8560ff1660ff1681526020019081526020015f205f600183610ae99190611873565b63ffffffff1663ffffffff1681526020019081526020015f2060405180604001604052804363ffffffff1681526020015f5f1b815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151816001015550505b8092505050919050565b5f610b8b8383610c1d565b9050610b99838383876110ee565b8160015f8560ff1660ff1681526020019081526020015f205f8681526020019081526020015f205f6101000a81548163ffffffff021916908363ffffffff160217905550837f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f68484604051610c0f929190611ae6565b60405180910390a250505050565b5f5f60025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f2080549050905060025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f20600182610c969190611b16565b81548110610ca757610ca661177b565b5b905f5260205f20906002020191505092915050565b5f5f60035f8460ff1660ff1681526020019081526020015f2080549050905060035f8460ff1660ff1681526020019081526020015f20600182610cff9190611b16565b81548110610d1057610d0f61177b565b5b905f5260205f2001915050919050565b5f5f60035f8560ff1660ff1681526020019081526020015f208054905090505f8190505b5f811115610e19575f60035f8760ff1660ff1681526020019081526020015f20600183610d719190611b16565b81548110610d8257610d8161177b565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff168152505090508463ffffffff16815f015163ffffffff1611610e055780602001519350505050610e55565b508080610e1190611b49565b915050610d44565b506040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610e4c90611c06565b60405180910390fd5b92915050565b5f5f60025f8660ff1660ff1681526020019081526020015f205f8563ffffffff1663ffffffff1681526020019081526020015f208054905090505f8190505b5f811115610f73575f60025f8860ff1660ff1681526020019081526020015f205f8763ffffffff1663ffffffff1681526020019081526020015f20600183610ee29190611b16565b81548110610ef357610ef261177b565b5b905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff16815260200160018201548152505090508463ffffffff16815f015163ffffffff1611610f5f5780602001519350505050610f7b565b508080610f6b90611b49565b915050610e9a565b505f5f1b9150505b9392505050565b5f5f610f8d83610cbc565b90505f6001825f0160049054906101000a900463ffffffff16610fb09190611873565b9050610fbd848383610ff6565b8092505050919050565b5f5f610fd38484610c1d565b90505f81600101549050610feb8585845f5f1b6110ee565b809250505092915050565b4363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff16036110425780825f0160046101000a81548163ffffffff021916908363ffffffff1602179055506110e9565b60035f8460ff1660ff1681526020019081526020015f2060405180604001604052804363ffffffff1681526020018363ffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff16021790555050505b505050565b4363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff1603611121578082600101819055506111c5565b60025f8560ff1660ff1681526020019081526020015f205f8463ffffffff1663ffffffff1681526020019081526020015f2060405180604001604052804363ffffffff16815260200183815250908060018154018082558091505060019003905f5260205f2090600202015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151816001015550505b50505050565b60405180604001604052805f63ffffffff1681526020015f81525090565b60405180604001604052805f63ffffffff1681526020015f63ffffffff1681525090565b5f5ffd5b5f5ffd5b5f819050919050565b61122781611215565b8114611231575f5ffd5b50565b5f813590506112428161121e565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261126957611268611248565b5b8235905067ffffffffffffffff8111156112865761128561124c565b5b6020830191508360018202830111156112a2576112a1611250565b5b9250929050565b5f5f5f604084860312156112c0576112bf61120d565b5b5f6112cd86828701611234565b935050602084013567ffffffffffffffff8111156112ee576112ed611211565b5b6112fa86828701611254565b92509250509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f63ffffffff82169050919050565b6113478161132f565b82525050565b5f611358838361133e565b60208301905092915050565b5f602082019050919050565b5f61137a82611306565b6113848185611310565b935061138f83611320565b805f5b838110156113bf5781516113a6888261134d565b97506113b183611364565b925050600181019050611392565b5085935050505092915050565b5f6020820190508181035f8301526113e48184611370565b905092915050565b5f60ff82169050919050565b611401816113ec565b811461140b575f5ffd5b50565b5f8135905061141c816113f8565b92915050565b61142b8161132f565b8114611435575f5ffd5b50565b5f8135905061144681611422565b92915050565b5f5f604083850312156114625761146161120d565b5b5f61146f8582860161140e565b925050602061148085828601611438565b9150509250929050565b61149381611215565b82525050565b604082015f8201516114ad5f85018261133e565b5060208201516114c0602085018261148a565b50505050565b5f6040820190506114d95f830184611499565b92915050565b5f602082840312156114f4576114f361120d565b5b5f6115018482850161140e565b91505092915050565b5f5f5f606084860312156115215761152061120d565b5b5f61152e8682870161140e565b935050602061153f86828701611438565b925050604061155086828701611438565b9150509250925092565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6115838261155a565b9050919050565b61159381611579565b82525050565b5f6020820190506115ac5f83018461158a565b92915050565b604082015f8201516115c65f85018261133e565b5060208201516115d9602085018261133e565b50505050565b5f6040820190506115f25f8301846115b2565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f61162c838361148a565b60208301905092915050565b5f602082019050919050565b5f61164e826115f8565b6116588185611602565b935061166383611612565b805f5b8381101561169357815161167a8882611621565b975061168583611638565b925050600181019050611666565b5085935050505092915050565b5f6020820190508181035f8301526116b88184611644565b905092915050565b6116c981611215565b82525050565b5f6020820190506116e25f8301846116c0565b92915050565b5f5f604083850312156116fe576116fd61120d565b5b5f61170b8582860161140e565b925050602061171c85828601611234565b9150509250929050565b61172f8161132f565b82525050565b5f6020820190506117485f830184611726565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b7f496e64657852656769737472792e72656769737465724f70657261746f723a205f8201527f71756f72756d20646f6573206e6f742065786973740000000000000000000000602082015250565b5f6118126035836117a8565b915061181d826117b8565b604082019050919050565b5f6020820190508181035f83015261183f81611806565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61187d8261132f565b91506118888361132f565b9250828203905063ffffffff8111156118a4576118a3611846565b5b92915050565b7f496e64657852656769737472792e63726561746551756f72756d3a2071756f725f8201527f756d20616c726561647920657869737473000000000000000000000000000000602082015250565b5f6119046031836117a8565b915061190f826118aa565b604082019050919050565b5f6020820190508181035f830152611931816118f8565b9050919050565b7f496e64657852656769737472792e6765744f70657261746f724c6973744174425f8201527f6c6f636b4e756d6265723a206f70657261746f7220646f6573206e6f7420657860208201527f6973742061742074686520676976656e20626c6f636b206e756d626572000000604082015250565b5f6119b8605d836117a8565b91506119c382611938565b606082019050919050565b5f6020820190508181035f8301526119e5816119ac565b9050919050565b7f496e64657852656769737472792e5f636865636b5265676973747279436f6f725f8201527f64696e61746f723a2063616c6c6572206973206e6f742074686520726567697360208201527f74727920636f6f7264696e61746f720000000000000000000000000000000000604082015250565b5f611a6c604f836117a8565b9150611a77826119ec565b606082019050919050565b5f6020820190508181035f830152611a9981611a60565b9050919050565b5f611aaa8261132f565b9150611ab58361132f565b9250828201905063ffffffff811115611ad157611ad0611846565b5b92915050565b611ae0816113ec565b82525050565b5f604082019050611af95f830185611ad7565b611b066020830184611726565b9392505050565b5f819050919050565b5f611b2082611b0d565b9150611b2b83611b0d565b9250828203905081811115611b4357611b42611846565b5b92915050565b5f611b5382611b0d565b91505f8203611b6557611b64611846565b5b600182039050919050565b7f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c5f8201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060208201527f617420676976656e20626c6f636b206e756d6265720000000000000000000000604082015250565b5f611bf06055836117a8565b9150611bfb82611b70565b606082019050919050565b5f6020820190508181035f830152611c1d81611be4565b905091905056fea26469706673582212209f760e1941a5da3a38b510b4b789fecd0a4df76d61939ddb0518f3f8aa71977364736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xB1W_5`\xE0\x1C\x80c\x89\x02bE\x11a\0oW\x80c\x89\x02bE\x14a\x01\xAFW\x80c\xA4\x8B\xB0\xAC\x14a\x01\xDFW\x80c\xBD)\xB8\xCD\x14a\x02\x0FW\x80c\xCA\xA3\xCDv\x14a\x02+W\x80c\xE2\xE6\x85\x80\x14a\x02IW\x80c\xF3A\t\"\x14a\x02yWa\0\xB1V[\x80b\xBF\xF0M\x14a\0\xB5W\x80c\x12\xD1\xD7M\x14a\0\xE5W\x80c&\xD9A\xF2\x14a\x01\x15W\x80c.\xD5\x83\xE5\x14a\x011W\x80cm\x14\xA9\x87\x14a\x01aW\x80c\x81!\x90o\x14a\x01\x7FW[__\xFD[a\0\xCF`\x04\x806\x03\x81\x01\x90a\0\xCA\x91\x90a\x12\xA9V[a\x02\xA9V[`@Qa\0\xDC\x91\x90a\x13\xCCV[`@Q\x80\x91\x03\x90\xF3[a\0\xFF`\x04\x806\x03\x81\x01\x90a\0\xFA\x91\x90a\x14LV[a\x04\x02V[`@Qa\x01\x0C\x91\x90a\x14\xC6V[`@Q\x80\x91\x03\x90\xF3[a\x01/`\x04\x806\x03\x81\x01\x90a\x01*\x91\x90a\x14\xDFV[a\x04UV[\0[a\x01K`\x04\x806\x03\x81\x01\x90a\x01F\x91\x90a\x15\nV[a\x05bV[`@Qa\x01X\x91\x90a\x14\xC6V[`@Q\x80\x91\x03\x90\xF3[a\x01ia\x06\x02V[`@Qa\x01v\x91\x90a\x15\x99V[`@Q\x80\x91\x03\x90\xF3[a\x01\x99`\x04\x806\x03\x81\x01\x90a\x01\x94\x91\x90a\x14\xDFV[a\x06&V[`@Qa\x01\xA6\x91\x90a\x15\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x01\xC9`\x04\x806\x03\x81\x01\x90a\x01\xC4\x91\x90a\x14LV[a\x06\x92V[`@Qa\x01\xD6\x91\x90a\x16\xA0V[`@Q\x80\x91\x03\x90\xF3[a\x01\xF9`\x04\x806\x03\x81\x01\x90a\x01\xF4\x91\x90a\x14LV[a\x07\xA7V[`@Qa\x02\x06\x91\x90a\x15\xDFV[`@Q\x80\x91\x03\x90\xF3[a\x02)`\x04\x806\x03\x81\x01\x90a\x02$\x91\x90a\x12\xA9V[a\x08BV[\0[a\x023a\tZV[`@Qa\x02@\x91\x90a\x16\xCFV[`@Q\x80\x91\x03\x90\xF3[a\x02c`\x04\x806\x03\x81\x01\x90a\x02^\x91\x90a\x16\xE8V[a\t`V[`@Qa\x02p\x91\x90a\x175V[`@Q\x80\x91\x03\x90\xF3[a\x02\x93`\x04\x806\x03\x81\x01\x90a\x02\x8E\x91\x90a\x14\xDFV[a\t\x8DV[`@Qa\x02\xA0\x91\x90a\x175V[`@Q\x80\x91\x03\x90\xF3[``a\x02\xB3a\t\xB1V[_\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xD1Wa\x02\xD0a\x17NV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xFFW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x84\x84\x90P\x81\x10\x15a\x03\xF6W_\x85\x85\x83\x81\x81\x10a\x03&Wa\x03%a\x17{V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x03\x95W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x8C\x90a\x18(V[`@Q\x80\x91\x03\x90\xFD[_a\x03\x9F\x83a\nAV[\x90Pa\x03\xB8\x89\x84`\x01\x84a\x03\xB3\x91\x90a\x18sV[a\x0B\x80V[\x80\x85\x85\x81Q\x81\x10a\x03\xCCWa\x03\xCBa\x17{V[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPPPP\x80\x80`\x01\x01\x91PPa\x03\x07V[P\x80\x91PP\x93\x92PPPV[a\x04\na\x11\xCBV[a\x04\x14\x83\x83a\x0C\x1DV[`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x92\x91PPV[a\x04]a\t\xB1V[_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14a\x04\xB9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xB0\x90a\x19\x1AV[`@Q\x80\x91\x03\x90\xFD[`\x03_\x82`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[a\x05ja\x11\xCBV[`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x05\xB4Wa\x05\xB3a\x17{V[[\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06.a\x11\xE9V[a\x067\x82a\x0C\xBCV[`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x91\x90PV[``_a\x06\x9F\x84\x84a\r V[\x90P_\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xC2Wa\x06\xC1a\x17NV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xF0W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07\x9BWa\x07\x11\x86\x82\x87a\x0E[V[\x82\x82\x81Q\x81\x10a\x07$Wa\x07#a\x17{V[[` \x02` \x01\x01\x81\x81RPP__\x1B\x82\x82\x81Q\x81\x10a\x07FWa\x07Ea\x17{V[[` \x02` \x01\x01Q\x03a\x07\x8EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x85\x90a\x19\xCEV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x06\xF8V[P\x80\x92PPP\x92\x91PPV[a\x07\xAFa\x11\xE9V[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x07\xDEWa\x07\xDDa\x17{V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x08Ja\t\xB1V[__\x90P[\x82\x82\x90P\x81\x10\x15a\tTW_\x83\x83\x83\x81\x81\x10a\x08nWa\x08ma\x17{V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a\x08\xDDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\xD4\x90a\x18(V[`@Q\x80\x91\x03\x90\xFD[_`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x88\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P_a\t \x84a\x0F\x82V[\x90P_a\t-\x85\x83a\x0F\xC7V[\x90P\x80\x89\x14a\tBWa\tA\x81\x86\x85a\x0B\x80V[[PPPPP\x80\x80`\x01\x01\x91PPa\x08OV[PPPPV[__\x1B\x81V[`\x01` R\x81_R`@_ ` R\x80_R`@_ _\x91P\x91P\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[_a\t\x97\x82a\x0C\xBCV[_\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n?W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n6\x90a\x1A\x82V[`@Q\x80\x91\x03\x90\xFD[V[__a\nL\x83a\x0C\xBCV[\x90P_`\x01\x82_\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\no\x91\x90a\x1A\xA0V[\x90Pa\n|\x84\x83\x83a\x0F\xF6V[_`\x02_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x01\x84a\n\xA2\x91\x90a\x18sV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x03a\x0BvW`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _`\x01\x83a\n\xE9\x91\x90a\x18sV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01__\x1B\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01UPP[\x80\x92PPP\x91\x90PV[_a\x0B\x8B\x83\x83a\x0C\x1DV[\x90Pa\x0B\x99\x83\x83\x83\x87a\x10\xEEV[\x81`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x86\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x84\x84`@Qa\x0C\x0F\x92\x91\x90a\x1A\xE6V[`@Q\x80\x91\x03\x90\xA2PPPPV[__`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x82a\x0C\x96\x91\x90a\x1B\x16V[\x81T\x81\x10a\x0C\xA7Wa\x0C\xA6a\x17{V[[\x90_R` _ \x90`\x02\x02\x01\x91PP\x92\x91PPV[__`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x82a\x0C\xFF\x91\x90a\x1B\x16V[\x81T\x81\x10a\r\x10Wa\r\x0Fa\x17{V[[\x90_R` _ \x01\x91PP\x91\x90PV[__`\x03_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a\x0E\x19W_`\x03_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\rq\x91\x90a\x1B\x16V[\x81T\x81\x10a\r\x82Wa\r\x81a\x17{V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x84c\xFF\xFF\xFF\xFF\x16\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0E\x05W\x80` \x01Q\x93PPPPa\x0EUV[P\x80\x80a\x0E\x11\x90a\x1BIV[\x91PPa\rDV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0EL\x90a\x1C\x06V[`@Q\x80\x91\x03\x90\xFD[\x92\x91PPV[__`\x02_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a\x0FsW_`\x02_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x87c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\x0E\xE2\x91\x90a\x1B\x16V[\x81T\x81\x10a\x0E\xF3Wa\x0E\xF2a\x17{V[[\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x84c\xFF\xFF\xFF\xFF\x16\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0F_W\x80` \x01Q\x93PPPPa\x0F{V[P\x80\x80a\x0Fk\x90a\x1BIV[\x91PPa\x0E\x9AV[P__\x1B\x91PP[\x93\x92PPPV[__a\x0F\x8D\x83a\x0C\xBCV[\x90P_`\x01\x82_\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\x0F\xB0\x91\x90a\x18sV[\x90Pa\x0F\xBD\x84\x83\x83a\x0F\xF6V[\x80\x92PPP\x91\x90PV[__a\x0F\xD3\x84\x84a\x0C\x1DV[\x90P_\x81`\x01\x01T\x90Pa\x0F\xEB\x85\x85\x84__\x1Ba\x10\xEEV[\x80\x92PPP\x92\x91PPV[Cc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x10BW\x80\x82_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x10\xE9V[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83c\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[PPPV[Cc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a\x11!W\x80\x82`\x01\x01\x81\x90UPa\x11\xC5V[`\x02_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01UPP[PPPPV[`@Q\x80`@\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81RP\x90V[`@Q\x80`@\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[__\xFD[__\xFD[_\x81\x90P\x91\x90PV[a\x12'\x81a\x12\x15V[\x81\x14a\x121W__\xFD[PV[_\x815\x90Pa\x12B\x81a\x12\x1EV[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a\x12iWa\x12ha\x12HV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x86Wa\x12\x85a\x12LV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x12\xA2Wa\x12\xA1a\x12PV[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15a\x12\xC0Wa\x12\xBFa\x12\rV[[_a\x12\xCD\x86\x82\x87\x01a\x124V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xEEWa\x12\xEDa\x12\x11V[[a\x12\xFA\x86\x82\x87\x01a\x12TV[\x92P\x92PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x13G\x81a\x13/V[\x82RPPV[_a\x13X\x83\x83a\x13>V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x13z\x82a\x13\x06V[a\x13\x84\x81\x85a\x13\x10V[\x93Pa\x13\x8F\x83a\x13 V[\x80_[\x83\x81\x10\x15a\x13\xBFW\x81Qa\x13\xA6\x88\x82a\x13MV[\x97Pa\x13\xB1\x83a\x13dV[\x92PP`\x01\x81\x01\x90Pa\x13\x92V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x13\xE4\x81\x84a\x13pV[\x90P\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[a\x14\x01\x81a\x13\xECV[\x81\x14a\x14\x0BW__\xFD[PV[_\x815\x90Pa\x14\x1C\x81a\x13\xF8V[\x92\x91PPV[a\x14+\x81a\x13/V[\x81\x14a\x145W__\xFD[PV[_\x815\x90Pa\x14F\x81a\x14\"V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x14bWa\x14aa\x12\rV[[_a\x14o\x85\x82\x86\x01a\x14\x0EV[\x92PP` a\x14\x80\x85\x82\x86\x01a\x148V[\x91PP\x92P\x92\x90PV[a\x14\x93\x81a\x12\x15V[\x82RPPV[`@\x82\x01_\x82\x01Qa\x14\xAD_\x85\x01\x82a\x13>V[P` \x82\x01Qa\x14\xC0` \x85\x01\x82a\x14\x8AV[PPPPV[_`@\x82\x01\x90Pa\x14\xD9_\x83\x01\x84a\x14\x99V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x14\xF4Wa\x14\xF3a\x12\rV[[_a\x15\x01\x84\x82\x85\x01a\x14\x0EV[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x15!Wa\x15 a\x12\rV[[_a\x15.\x86\x82\x87\x01a\x14\x0EV[\x93PP` a\x15?\x86\x82\x87\x01a\x148V[\x92PP`@a\x15P\x86\x82\x87\x01a\x148V[\x91PP\x92P\x92P\x92V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x15\x83\x82a\x15ZV[\x90P\x91\x90PV[a\x15\x93\x81a\x15yV[\x82RPPV[_` \x82\x01\x90Pa\x15\xAC_\x83\x01\x84a\x15\x8AV[\x92\x91PPV[`@\x82\x01_\x82\x01Qa\x15\xC6_\x85\x01\x82a\x13>V[P` \x82\x01Qa\x15\xD9` \x85\x01\x82a\x13>V[PPPPV[_`@\x82\x01\x90Pa\x15\xF2_\x83\x01\x84a\x15\xB2V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\x16,\x83\x83a\x14\x8AV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x16N\x82a\x15\xF8V[a\x16X\x81\x85a\x16\x02V[\x93Pa\x16c\x83a\x16\x12V[\x80_[\x83\x81\x10\x15a\x16\x93W\x81Qa\x16z\x88\x82a\x16!V[\x97Pa\x16\x85\x83a\x168V[\x92PP`\x01\x81\x01\x90Pa\x16fV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x16\xB8\x81\x84a\x16DV[\x90P\x92\x91PPV[a\x16\xC9\x81a\x12\x15V[\x82RPPV[_` \x82\x01\x90Pa\x16\xE2_\x83\x01\x84a\x16\xC0V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x16\xFEWa\x16\xFDa\x12\rV[[_a\x17\x0B\x85\x82\x86\x01a\x14\x0EV[\x92PP` a\x17\x1C\x85\x82\x86\x01a\x124V[\x91PP\x92P\x92\x90PV[a\x17/\x81a\x13/V[\x82RPPV[_` \x82\x01\x90Pa\x17H_\x83\x01\x84a\x17&V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FIndexRegistry.registerOperator: _\x82\x01R\x7Fquorum does not exist\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x18\x12`5\x83a\x17\xA8V[\x91Pa\x18\x1D\x82a\x17\xB8V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x18?\x81a\x18\x06V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x18}\x82a\x13/V[\x91Pa\x18\x88\x83a\x13/V[\x92P\x82\x82\x03\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\xA4Wa\x18\xA3a\x18FV[[\x92\x91PPV[\x7FIndexRegistry.createQuorum: quor_\x82\x01R\x7Fum already exists\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x19\x04`1\x83a\x17\xA8V[\x91Pa\x19\x0F\x82a\x18\xAAV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x191\x81a\x18\xF8V[\x90P\x91\x90PV[\x7FIndexRegistry.getOperatorListAtB_\x82\x01R\x7FlockNumber: operator does not ex` \x82\x01R\x7Fist at the given block number\0\0\0`@\x82\x01RPV[_a\x19\xB8`]\x83a\x17\xA8V[\x91Pa\x19\xC3\x82a\x198V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19\xE5\x81a\x19\xACV[\x90P\x91\x90PV[\x7FIndexRegistry._checkRegistryCoor_\x82\x01R\x7Fdinator: caller is not the regis` \x82\x01R\x7Ftry coordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1Al`O\x83a\x17\xA8V[\x91Pa\x1Aw\x82a\x19\xECV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1A\x99\x81a\x1A`V[\x90P\x91\x90PV[_a\x1A\xAA\x82a\x13/V[\x91Pa\x1A\xB5\x83a\x13/V[\x92P\x82\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xD1Wa\x1A\xD0a\x18FV[[\x92\x91PPV[a\x1A\xE0\x81a\x13\xECV[\x82RPPV[_`@\x82\x01\x90Pa\x1A\xF9_\x83\x01\x85a\x1A\xD7V[a\x1B\x06` \x83\x01\x84a\x17&V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a\x1B \x82a\x1B\rV[\x91Pa\x1B+\x83a\x1B\rV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a\x1BCWa\x1BBa\x18FV[[\x92\x91PPV[_a\x1BS\x82a\x1B\rV[\x91P_\x82\x03a\x1BeWa\x1Bda\x18FV[[`\x01\x82\x03\x90P\x91\x90PV[\x7FIndexRegistry._operatorCountAtBl_\x82\x01R\x7FockNumber: quorum did not exist ` \x82\x01R\x7Fat given block number\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1B\xF0`U\x83a\x17\xA8V[\x91Pa\x1B\xFB\x82a\x1BpV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C\x1D\x81a\x1B\xE4V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x9Fv\x0E\x19A\xA5\xDA:8\xB5\x10\xB4\xB7\x89\xFE\xCD\nM\xF7ma\x93\x9D\xDB\x05\x18\xF3\xF8\xAAq\x97sdsolcC\0\x08\x1B\x003",
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
    /**Event with signature `QuorumIndexUpdate(bytes32,uint8,uint32)` and selector `0x6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6`.
```solidity
event QuorumIndexUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint32 newOperatorIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumIndexUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub newOperatorIndex: u32,
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
        impl alloy_sol_types::SolEvent for QuorumIndexUpdate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "QuorumIndexUpdate(bytes32,uint8,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8,
                225u8,
                228u8,
                244u8,
                7u8,
                95u8,
                61u8,
                6u8,
                113u8,
                118u8,
                20u8,
                13u8,
                52u8,
                232u8,
                120u8,
                116u8,
                36u8,
                77u8,
                210u8,
                115u8,
                41u8,
                76u8,
                5u8,
                178u8,
                33u8,
                129u8,
                51u8,
                228u8,
                154u8,
                43u8,
                166u8,
                246u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorId: topics.1,
                    quorumNumber: data.0,
                    newOperatorIndex: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newOperatorIndex),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operatorId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumIndexUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumIndexUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumIndexUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _registryCoordinator);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._registryCoordinator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                )
            }
        }
    };
    /**Function with signature `OPERATOR_DOES_NOT_EXIST_ID()` and selector `0xcaa3cd76`.
```solidity
function OPERATOR_DOES_NOT_EXIST_ID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_DOES_NOT_EXIST_IDCall {}
    ///Container type for the return parameters of the [`OPERATOR_DOES_NOT_EXIST_ID()`](OPERATOR_DOES_NOT_EXIST_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_DOES_NOT_EXIST_IDReturn {
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
            impl ::core::convert::From<OPERATOR_DOES_NOT_EXIST_IDCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_DOES_NOT_EXIST_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_DOES_NOT_EXIST_IDCall {
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
            impl ::core::convert::From<OPERATOR_DOES_NOT_EXIST_IDReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_DOES_NOT_EXIST_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_DOES_NOT_EXIST_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_DOES_NOT_EXIST_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_DOES_NOT_EXIST_IDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_DOES_NOT_EXIST_ID()";
            const SELECTOR: [u8; 4] = [202u8, 163u8, 205u8, 118u8];
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
    /**Function with signature `currentOperatorIndex(uint8,bytes32)` and selector `0xe2e68580`.
```solidity
function currentOperatorIndex(uint8, bytes32) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentOperatorIndexCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`currentOperatorIndex(uint8,bytes32)`](currentOperatorIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentOperatorIndexReturn {
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<currentOperatorIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentOperatorIndexCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentOperatorIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<currentOperatorIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentOperatorIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentOperatorIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentOperatorIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentOperatorIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentOperatorIndex(uint8,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 230u8, 133u8, 128u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
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
    /**Function with signature `deregisterOperator(bytes32,bytes)` and selector `0xbd29b8cd`.
```solidity
function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(bytes32,bytes)`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumbers: tuple.1,
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
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [189u8, 41u8, 184u8, 205u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
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
    /**Function with signature `getLatestOperatorUpdate(uint8,uint32)` and selector `0x12d1d74d`.
```solidity
function getLatestOperatorUpdate(uint8 quorumNumber, uint32 operatorIndex) external view returns (IIndexRegistry.OperatorUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestOperatorUpdateCall {
        pub quorumNumber: u8,
        pub operatorIndex: u32,
    }
    ///Container type for the return parameters of the [`getLatestOperatorUpdate(uint8,uint32)`](getLatestOperatorUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestOperatorUpdateReturn {
        pub _0: <IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestOperatorUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorUpdateCall) -> Self {
                    (value.quorumNumber, value.operatorIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestOperatorUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorIndex: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistry::OperatorUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestOperatorUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestOperatorUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestOperatorUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestOperatorUpdateReturn;
            type ReturnTuple<'a> = (IIndexRegistry::OperatorUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestOperatorUpdate(uint8,uint32)";
            const SELECTOR: [u8; 4] = [18u8, 209u8, 215u8, 77u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIndex),
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
    /**Function with signature `getLatestQuorumUpdate(uint8)` and selector `0x8121906f`.
```solidity
function getLatestQuorumUpdate(uint8 quorumNumber) external view returns (IIndexRegistry.QuorumUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestQuorumUpdateCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getLatestQuorumUpdate(uint8)`](getLatestQuorumUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestQuorumUpdateReturn {
        pub _0: <IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestQuorumUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestQuorumUpdateCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestQuorumUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestQuorumUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestQuorumUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestQuorumUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestQuorumUpdateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestQuorumUpdateReturn;
            type ReturnTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestQuorumUpdate(uint8)";
            const SELECTOR: [u8; 4] = [129u8, 33u8, 144u8, 111u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getOperatorListAtBlockNumber(uint8,uint32)` and selector `0x89026245`.
```solidity
function getOperatorListAtBlockNumber(uint8 quorumNumber, uint32 blockNumber) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorListAtBlockNumberCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorListAtBlockNumber(uint8,uint32)`](getOperatorListAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorListAtBlockNumberReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorListAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorListAtBlockNumberCall) -> Self {
                    (value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorListAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                    }
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
            impl ::core::convert::From<getOperatorListAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorListAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorListAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorListAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorListAtBlockNumberReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorListAtBlockNumber(uint8,uint32)";
            const SELECTOR: [u8; 4] = [137u8, 2u8, 98u8, 69u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getOperatorUpdateAtIndex(uint8,uint32,uint32)` and selector `0x2ed583e5`.
```solidity
function getOperatorUpdateAtIndex(uint8 quorumNumber, uint32 operatorIndex, uint32 arrayIndex) external view returns (IIndexRegistry.OperatorUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub operatorIndex: u32,
        pub arrayIndex: u32,
    }
    ///Container type for the return parameters of the [`getOperatorUpdateAtIndex(uint8,uint32,uint32)`](getOperatorUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorUpdateAtIndexReturn {
        pub _0: <IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorIndex, value.arrayIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorIndex: tuple.1,
                        arrayIndex: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistry::OperatorUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IIndexRegistry::OperatorUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorUpdateAtIndex(uint8,uint32,uint32)";
            const SELECTOR: [u8; 4] = [46u8, 213u8, 131u8, 229u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.arrayIndex),
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
    /**Function with signature `getQuorumUpdateAtIndex(uint8,uint32)` and selector `0xa48bb0ac`.
```solidity
function getQuorumUpdateAtIndex(uint8 quorumNumber, uint32 quorumIndex) external view returns (IIndexRegistry.QuorumUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub quorumIndex: u32,
    }
    ///Container type for the return parameters of the [`getQuorumUpdateAtIndex(uint8,uint32)`](getQuorumUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumUpdateAtIndexReturn {
        pub _0: <IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.quorumIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        quorumIndex: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getQuorumUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getQuorumUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getQuorumUpdateAtIndex(uint8,uint32)";
            const SELECTOR: [u8; 4] = [164u8, 139u8, 176u8, 172u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumIndex),
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
    /**Function with signature `initializeQuorum(uint8)` and selector `0x26d941f2`.
```solidity
function initializeQuorum(uint8 quorumNumber) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`initializeQuorum(uint8)`](initializeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumReturn {}
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
            impl ::core::convert::From<initializeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<initializeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeQuorum(uint8)";
            const SELECTOR: [u8; 4] = [38u8, 217u8, 65u8, 242u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `registerOperator(bytes32,bytes)` and selector `0x00bff04d`.
```solidity
function registerOperator(bytes32 operatorId, bytes memory quorumNumbers) external returns (uint32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(bytes32,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
        pub _0: alloy::sol_types::private::Vec<u32>,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<registerOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumbers: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [0u8, 191u8, 240u8, 77u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
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
    /**Function with signature `totalOperatorsForQuorum(uint8)` and selector `0xf3410922`.
```solidity
function totalOperatorsForQuorum(uint8 quorumNumber) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`totalOperatorsForQuorum(uint8)`](totalOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumReturn {
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
            impl ::core::convert::From<totalOperatorsForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<totalOperatorsForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalOperatorsForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalOperatorsForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalOperatorsForQuorum(uint8)";
            const SELECTOR: [u8; 4] = [243u8, 65u8, 9u8, 34u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    ///Container for all the [`IndexRegistry`](self) function calls.
    pub enum IndexRegistryCalls {
        OPERATOR_DOES_NOT_EXIST_ID(OPERATOR_DOES_NOT_EXIST_IDCall),
        currentOperatorIndex(currentOperatorIndexCall),
        deregisterOperator(deregisterOperatorCall),
        getLatestOperatorUpdate(getLatestOperatorUpdateCall),
        getLatestQuorumUpdate(getLatestQuorumUpdateCall),
        getOperatorListAtBlockNumber(getOperatorListAtBlockNumberCall),
        getOperatorUpdateAtIndex(getOperatorUpdateAtIndexCall),
        getQuorumUpdateAtIndex(getQuorumUpdateAtIndexCall),
        initializeQuorum(initializeQuorumCall),
        registerOperator(registerOperatorCall),
        registryCoordinator(registryCoordinatorCall),
        totalOperatorsForQuorum(totalOperatorsForQuorumCall),
    }
    #[automatically_derived]
    impl IndexRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 191u8, 240u8, 77u8],
            [18u8, 209u8, 215u8, 77u8],
            [38u8, 217u8, 65u8, 242u8],
            [46u8, 213u8, 131u8, 229u8],
            [109u8, 20u8, 169u8, 135u8],
            [129u8, 33u8, 144u8, 111u8],
            [137u8, 2u8, 98u8, 69u8],
            [164u8, 139u8, 176u8, 172u8],
            [189u8, 41u8, 184u8, 205u8],
            [202u8, 163u8, 205u8, 118u8],
            [226u8, 230u8, 133u8, 128u8],
            [243u8, 65u8, 9u8, 34u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IndexRegistryCalls {
        const NAME: &'static str = "IndexRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 12usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::OPERATOR_DOES_NOT_EXIST_ID(_) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentOperatorIndex(_) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestOperatorUpdate(_) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestQuorumUpdate(_) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorListAtBlockNumber(_) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorUpdateAtIndex(_) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumUpdateAtIndex(_) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeQuorum(_) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::totalOperatorsForQuorum(_) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<IndexRegistryCalls>] = &[
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getLatestOperatorUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::getLatestOperatorUpdate)
                    }
                    getLatestOperatorUpdate
                },
                {
                    fn initializeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::initializeQuorum)
                    }
                    initializeQuorum
                },
                {
                    fn getOperatorUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::getOperatorUpdateAtIndex)
                    }
                    getOperatorUpdateAtIndex
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn getLatestQuorumUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::getLatestQuorumUpdate)
                    }
                    getLatestQuorumUpdate
                },
                {
                    fn getOperatorListAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::getOperatorListAtBlockNumber)
                    }
                    getOperatorListAtBlockNumber
                },
                {
                    fn getQuorumUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::getQuorumUpdateAtIndex)
                    }
                    getQuorumUpdateAtIndex
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn OPERATOR_DOES_NOT_EXIST_ID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::OPERATOR_DOES_NOT_EXIST_ID)
                    }
                    OPERATOR_DOES_NOT_EXIST_ID
                },
                {
                    fn currentOperatorIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::currentOperatorIndex)
                    }
                    currentOperatorIndex
                },
                {
                    fn totalOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IndexRegistryCalls::totalOperatorsForQuorum)
                    }
                    totalOperatorsForQuorum
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
                Self::OPERATOR_DOES_NOT_EXIST_ID(inner) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentOperatorIndex(inner) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestOperatorUpdate(inner) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestQuorumUpdate(inner) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorListAtBlockNumber(inner) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorUpdateAtIndex(inner) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumUpdateAtIndex(inner) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::totalOperatorsForQuorum(inner) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::OPERATOR_DOES_NOT_EXIST_ID(inner) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentOperatorIndex(inner) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestOperatorUpdate(inner) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestQuorumUpdate(inner) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorListAtBlockNumber(inner) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorUpdateAtIndex(inner) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getQuorumUpdateAtIndex(inner) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::totalOperatorsForQuorum(inner) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IndexRegistry`](self) events.
    pub enum IndexRegistryEvents {
        Initialized(Initialized),
        QuorumIndexUpdate(QuorumIndexUpdate),
    }
    #[automatically_derived]
    impl IndexRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                110u8,
                225u8,
                228u8,
                244u8,
                7u8,
                95u8,
                61u8,
                6u8,
                113u8,
                118u8,
                20u8,
                13u8,
                52u8,
                232u8,
                120u8,
                116u8,
                36u8,
                77u8,
                210u8,
                115u8,
                41u8,
                76u8,
                5u8,
                178u8,
                33u8,
                129u8,
                51u8,
                228u8,
                154u8,
                43u8,
                166u8,
                246u8,
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IndexRegistryEvents {
        const NAME: &'static str = "IndexRegistryEvents";
        const COUNT: usize = 2usize;
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
                    <QuorumIndexUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <QuorumIndexUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumIndexUpdate)
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
    impl alloy_sol_types::private::IntoLogData for IndexRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumIndexUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumIndexUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IndexRegistry`](self) contract instance.

See the [wrapper's documentation](`IndexRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IndexRegistryInstance<T, P, N> {
        IndexRegistryInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IndexRegistryInstance<T, P, N>>,
    > {
        IndexRegistryInstance::<T, P, N>::deploy(provider, _registryCoordinator)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        IndexRegistryInstance::<T, P, N>::deploy_builder(provider, _registryCoordinator)
    }
    /**A [`IndexRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IndexRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IndexRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IndexRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IndexRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IndexRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IndexRegistry`](self) contract instance.

See the [wrapper's documentation](`IndexRegistryInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IndexRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _registryCoordinator);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
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
    impl<T, P: ::core::clone::Clone, N> IndexRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IndexRegistryInstance<T, P, N> {
            IndexRegistryInstance {
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
    > IndexRegistryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`OPERATOR_DOES_NOT_EXIST_ID`] function.
        pub fn OPERATOR_DOES_NOT_EXIST_ID(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, OPERATOR_DOES_NOT_EXIST_IDCall, N> {
            self.call_builder(&OPERATOR_DOES_NOT_EXIST_IDCall {})
        }
        ///Creates a new call builder for the [`currentOperatorIndex`] function.
        pub fn currentOperatorIndex(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentOperatorIndexCall, N> {
            self.call_builder(&currentOperatorIndexCall { _0, _1 })
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(
                &deregisterOperatorCall {
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`getLatestOperatorUpdate`] function.
        pub fn getLatestOperatorUpdate(
            &self,
            quorumNumber: u8,
            operatorIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestOperatorUpdateCall, N> {
            self.call_builder(
                &getLatestOperatorUpdateCall {
                    quorumNumber,
                    operatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getLatestQuorumUpdate`] function.
        pub fn getLatestQuorumUpdate(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestQuorumUpdateCall, N> {
            self.call_builder(
                &getLatestQuorumUpdateCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorListAtBlockNumber`] function.
        pub fn getOperatorListAtBlockNumber(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorListAtBlockNumberCall, N> {
            self.call_builder(
                &getOperatorListAtBlockNumberCall {
                    quorumNumber,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorUpdateAtIndex`] function.
        pub fn getOperatorUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorIndex: u32,
            arrayIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorUpdateAtIndexCall, N> {
            self.call_builder(
                &getOperatorUpdateAtIndexCall {
                    quorumNumber,
                    operatorIndex,
                    arrayIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getQuorumUpdateAtIndex`] function.
        pub fn getQuorumUpdateAtIndex(
            &self,
            quorumNumber: u8,
            quorumIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumUpdateAtIndexCall, N> {
            self.call_builder(
                &getQuorumUpdateAtIndexCall {
                    quorumNumber,
                    quorumIndex,
                },
            )
        }
        ///Creates a new call builder for the [`initializeQuorum`] function.
        pub fn initializeQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeQuorumCall, N> {
            self.call_builder(
                &initializeQuorumCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`totalOperatorsForQuorum`] function.
        pub fn totalOperatorsForQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalOperatorsForQuorumCall, N> {
            self.call_builder(
                &totalOperatorsForQuorumCall {
                    quorumNumber,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IndexRegistryInstance<T, P, N> {
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
        ///Creates a new event filter for the [`QuorumIndexUpdate`] event.
        pub fn QuorumIndexUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumIndexUpdate, N> {
            self.event_filter::<QuorumIndexUpdate>()
        }
    }
}
