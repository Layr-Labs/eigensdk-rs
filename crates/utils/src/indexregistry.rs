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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for OperatorUpdate {
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
        impl alloy_sol_types::SolStruct for OperatorUpdate {
            const NAME: &'static str = "OperatorUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorUpdate(uint32 fromBlockNumber,bytes32 operatorId)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.fromBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.numOperators,
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
        impl alloy_sol_types::SolType for QuorumUpdate {
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
        impl alloy_sol_types::SolStruct for QuorumUpdate {
            const NAME: &'static str = "QuorumUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumUpdate(uint32 fromBlockNumber,uint32 numOperators)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("IIndexRegistryInstance")
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
        > IIndexRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IIndexRegistry`](self) contract instance.

        See the [wrapper's documentation](`IIndexRegistryInstance`) for more details.*/
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
        > IIndexRegistryInstance<T, P, N>
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
        > IIndexRegistryInstance<T, P, N>
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
    ///0x60a03461011a57601f610ffd38819003918201601f19168301916001600160401b0383118484101761011e5780849260209460405283398101031261011a57516001600160a01b0381169081900361011a576080525f5460ff8160081c166100c55760ff8082161061008b575b604051610eca9081610133823960805181818161048a0152610a710152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f61006c565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8062bff04d1461065957806312d1d74d1461061457806326d941f2146105535780632ed583e5146104b95780636d14a987146104755780638121906f1461043957806389026245146102ca578063a48bb0ac1461024d578063bd29b8cd14610132578063caa3cd7614610118578063e2e68580146100d45763f34109221461009c575f80fd5b346100d05760203660031901126100d057602063ffffffff6100c46100bf610849565b610c5b565b54821c16604051908152f35b5f80fd5b346100d05760403660031901126100d05760ff6100ef610849565b165f52600160205260405f206024355f52602052602063ffffffff60405f205416604051908152f35b346100d0575f3660031901126100d05760206040515f8152f35b346100d057610140366107f5565b919061014a610a6f565b4363ffffffff16915f5b84811061015d57005b8061016b60019287866108de565b3560f81c805f52600360205261018660405f205415156108fe565b805f528260205260405f20845f5260205263ffffffff60405f2054166101ab82610c5b565b916101ca6101c263ffffffff855460201c16610968565b809483610e21565b6101d48382610c1f565b928584018963ffffffff8254965416145f14610211575f9150555b828603610200575b50505001610154565b61020992610b6a565b8680806101f7565b50815f52600260205263ffffffff60405f2091165f5260205261024860405f2061023961086c565b908a82525f6020830152610b24565b6101ef565b346100d05760403660031901126100d0576102c66102a161029b61026f610849565b60ff610279610859565b915f602061028561086c565b8281520152165f52600360205260405f206109b3565b50610a4f565b60405191829182919091602063ffffffff816040840195828151168552015116910152565b0390f35b346100d05760403660031901126100d0576102e3610849565b6102eb610859565b9063ffffffff6102fb8383610c8d565b1661030d610308826108c6565b6108a0565b9281845261031a826108c6565b602085019390601f19013685375f5b838110610374578486604051918291602083019060208452518091526040830191905f5b81811061035b575050500390f35b825184528594506020938401939092019160010161034d565b6103858363ffffffff831684610d88565b61038f8288610980565b5261039a8187610980565b51156103a857600101610329565b60405162461bcd60e51b815260206004820152605d60248201527f496e64657852656769737472792e6765744f70657261746f724c69737441744260448201527f6c6f636b4e756d6265723a206f70657261746f7220646f6573206e6f7420657860648201527f6973742061742074686520676976656e20626c6f636b206e756d626572000000608482015260a490fd5b346100d05760203660031901126100d0576102c66102a161047061045b610849565b5f602061046661086c565b8281520152610c5b565b610a4f565b346100d0575f3660031901126100d0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346100d05760603660031901126100d0576104d2610849565b6104da610859565b6044359163ffffffff831683036100d0576102c6926105309260ff61052a935f602061050461086c565b8281520152165f52600260205263ffffffff60405f2091165f5260205260405f20610a36565b50610994565b60405191829182919091602080604083019463ffffffff81511684520151910152565b346100d05760203660031901126100d05760ff61056e610849565b610576610a6f565b16805f52600360205260405f20546105b5575f5260036020526105b360405f2061059e61086c565b9063ffffffff431682525f60208301526109c8565b005b60405162461bcd60e51b815260206004820152603160248201527f496e64657852656769737472792e63726561746551756f72756d3a2071756f72604482015270756d20616c72656164792065786973747360781b6064820152608490fd5b346100d05760403660031901126100d0576102c6610530610654610636610849565b61063e610859565b905f602061064a61086c565b8281520152610c1f565b610994565b346100d057610667366107f5565b90610670610a6f565b61067c610308836108c6565b92828452610689836108c6565b602085019390601f19013685375f5b8181106106e9578486604051918291602083019060208452518091526040830191905f5b8181106106ca575050500390f35b825163ffffffff168452859450602093840193909201916001016106bc565b6106f48183866108de565b3560f81c90815f52600360205261071060405f205415156108fe565b61071982610c5b565b600163ffffffff825460201c16019063ffffffff82116107e157836107448361077f93600197610e21565b805f52600260205260405f2063ffffffff61075e85610968565b165f5260205260405f205415610798575b61077883610968565b9087610b6a565b63ffffffff61078e838a610980565b9116905201610698565b805f52600260205260405f2063ffffffff6107b285610968565b165f526020526107dc60405f206107c761086c565b9063ffffffff431682525f6020830152610b24565b61076f565b634e487b7160e01b5f52601160045260245ffd5b60406003198201126100d0576004359160243567ffffffffffffffff81116100d057826023820112156100d05780600401359267ffffffffffffffff84116100d057602484830101116100d0576024019190565b6004359060ff821682036100d057565b6024359063ffffffff821682036100d057565b604051906040820182811067ffffffffffffffff82111761088c57604052565b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761088c57604052565b67ffffffffffffffff811161088c5760051b60200190565b908210156108ea570190565b634e487b7160e01b5f52603260045260245ffd5b1561090557565b60405162461bcd60e51b815260206004820152603560248201527f496e64657852656769737472792e72656769737465724f70657261746f723a206044820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b6064820152608490fd5b63ffffffff5f199116019063ffffffff82116107e157565b80518210156108ea5760209160051b010190565b90600161099f61086c565b9263ffffffff815416845201546020830152565b80548210156108ea575f5260205f2001905f90565b8054600160401b81101561088c576109e5916001820181556109b3565b610a23578151815460209384015167ffffffffffffffff1990911663ffffffff9290921691909117921b67ffffffff0000000016919091179055565b565b634e487b7160e01b5f525f60045260245ffd5b80548210156108ea575f5260205f209060011b01905f90565b9063ffffffff610a5d61086c565b9254818116845260201c166020830152565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303610aa157565b60405162461bcd60e51b815260206004820152604f60248201527f496e64657852656769737472792e5f636865636b5265676973747279436f6f7260448201527f64696e61746f723a2063616c6c6572206973206e6f742074686520726567697360648201526e3a393c9031b7b7b93234b730ba37b960891b608482015260a490fd5b8054600160401b81101561088c57610b4191600182018155610a36565b919091610a235760208163ffffffff8060019451161663ffffffff198554161784550151910155565b9160409063ffffffff60ff7f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f69486610ba28583610c1f565b80544386169086168103610be55750600101555b1691825f526001602052835f20865f52602052835f2082821683198254161790558351928352166020820152a2565b9050610c1a91508383165f526002602052865f208587165f52602052865f20610c0c61086c565b918252896020830152610b24565b610bb6565b60ff165f90815260026020908152604080832063ffffffff9490941683529290522080545f1981019081116107e157610c5791610a36565b5090565b60ff165f90815260036020526040902080545f1981019081116107e157610c57916109b3565b80156107e1575f190190565b60ff1690815f52600360205260405f2054805b610d2d5760405162461bcd60e51b815260206004820152605560248201527f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c60448201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060648201527430ba1033b4bb32b710313637b1b590373ab6b132b960591b608482015260a490fd5b825f52600360205260405f205f198201908282116107e157610d529161029b916109b3565b63ffffffff81511663ffffffff84161015610d775750610d7190610c81565b80610ca0565b6020015163ffffffff169392505050565b60ff909291921691825f52600260205260405f2063ffffffff82165f5260205260405f2054805b610dbb57505050505f90565b835f52600260205260405f2063ffffffff83165f5260205260405f205f198201908282116107e157610df09161052a91610a36565b63ffffffff81511663ffffffff85161015610e155750610e0f90610c81565b80610daf565b60200151949350505050565b919063ffffffff81541663ffffffff43168091145f14610e635750610a2192509067ffffffff0000000082549160201b169067ffffffff000000001916179055565b91905060ff610a2193165f52600360205263ffffffff60405f2091610e8661086c565b9384521660208301526109c856fea26469706673582212205abe67ddeb674d2bcbcc31eb9506f73c41f02638f5d5516176da77605ebff1c264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA04a\x01\x1AW`\x1Fa\x0F\xFD8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\x1EW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\x01\x1AWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\x01\x1AW`\x80R_T`\xFF\x81`\x08\x1C\x16a\0\xC5W`\xFF\x80\x82\x16\x10a\0\x8BW[`@Qa\x0E\xCA\x90\x81a\x013\x829`\x80Q\x81\x81\x81a\x04\x8A\x01Ra\nq\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0lV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xBF\xF0M\x14a\x06YW\x80c\x12\xD1\xD7M\x14a\x06\x14W\x80c&\xD9A\xF2\x14a\x05SW\x80c.\xD5\x83\xE5\x14a\x04\xB9W\x80cm\x14\xA9\x87\x14a\x04uW\x80c\x81!\x90o\x14a\x049W\x80c\x89\x02bE\x14a\x02\xCAW\x80c\xA4\x8B\xB0\xAC\x14a\x02MW\x80c\xBD)\xB8\xCD\x14a\x012W\x80c\xCA\xA3\xCDv\x14a\x01\x18W\x80c\xE2\xE6\x85\x80\x14a\0\xD4Wc\xF3A\t\"\x14a\0\x9CW_\x80\xFD[4a\0\xD0W` 6`\x03\x19\x01\x12a\0\xD0W` c\xFF\xFF\xFF\xFFa\0\xC4a\0\xBFa\x08IV[a\x0C[V[T\x82\x1C\x16`@Q\x90\x81R\xF3[_\x80\xFD[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0W`\xFFa\0\xEFa\x08IV[\x16_R`\x01` R`@_ `$5_R` R` c\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[4a\0\xD0W_6`\x03\x19\x01\x12a\0\xD0W` `@Q_\x81R\xF3[4a\0\xD0Wa\x01@6a\x07\xF5V[\x91\x90a\x01Ja\noV[Cc\xFF\xFF\xFF\xFF\x16\x91_[\x84\x81\x10a\x01]W\0[\x80a\x01k`\x01\x92\x87\x86a\x08\xDEV[5`\xF8\x1C\x80_R`\x03` Ra\x01\x86`@_ T\x15\x15a\x08\xFEV[\x80_R\x82` R`@_ \x84_R` Rc\xFF\xFF\xFF\xFF`@_ T\x16a\x01\xAB\x82a\x0C[V[\x91a\x01\xCAa\x01\xC2c\xFF\xFF\xFF\xFF\x85T` \x1C\x16a\thV[\x80\x94\x83a\x0E!V[a\x01\xD4\x83\x82a\x0C\x1FV[\x92\x85\x84\x01\x89c\xFF\xFF\xFF\xFF\x82T\x96T\x16\x14_\x14a\x02\x11W_\x91PU[\x82\x86\x03a\x02\0W[PPP\x01a\x01TV[a\x02\t\x92a\x0BjV[\x86\x80\x80a\x01\xF7V[P\x81_R`\x02` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` Ra\x02H`@_ a\x029a\x08lV[\x90\x8A\x82R_` \x83\x01Ra\x0B$V[a\x01\xEFV[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0Wa\x02\xC6a\x02\xA1a\x02\x9Ba\x02oa\x08IV[`\xFFa\x02ya\x08YV[\x91_` a\x02\x85a\x08lV[\x82\x81R\x01R\x16_R`\x03` R`@_ a\t\xB3V[Pa\nOV[`@Q\x91\x82\x91\x82\x91\x90\x91` c\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95\x82\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x03\x90\xF3[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0Wa\x02\xE3a\x08IV[a\x02\xEBa\x08YV[\x90c\xFF\xFF\xFF\xFFa\x02\xFB\x83\x83a\x0C\x8DV[\x16a\x03\ra\x03\x08\x82a\x08\xC6V[a\x08\xA0V[\x92\x81\x84Ra\x03\x1A\x82a\x08\xC6V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x83\x81\x10a\x03tW\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x03[WPPP\x03\x90\xF3[\x82Q\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x03MV[a\x03\x85\x83c\xFF\xFF\xFF\xFF\x83\x16\x84a\r\x88V[a\x03\x8F\x82\x88a\t\x80V[Ra\x03\x9A\x81\x87a\t\x80V[Q\x15a\x03\xA8W`\x01\x01a\x03)V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`]`$\x82\x01R\x7FIndexRegistry.getOperatorListAtB`D\x82\x01R\x7FlockNumber: operator does not ex`d\x82\x01R\x7Fist at the given block number\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[4a\0\xD0W` 6`\x03\x19\x01\x12a\0\xD0Wa\x02\xC6a\x02\xA1a\x04pa\x04[a\x08IV[_` a\x04fa\x08lV[\x82\x81R\x01Ra\x0C[V[a\nOV[4a\0\xD0W_6`\x03\x19\x01\x12a\0\xD0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\0\xD0W``6`\x03\x19\x01\x12a\0\xD0Wa\x04\xD2a\x08IV[a\x04\xDAa\x08YV[`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\0\xD0Wa\x02\xC6\x92a\x050\x92`\xFFa\x05*\x93_` a\x05\x04a\x08lV[\x82\x81R\x01R\x16_R`\x02` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`@_ a\n6V[Pa\t\x94V[`@Q\x91\x82\x91\x82\x91\x90\x91` \x80`@\x83\x01\x94c\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[4a\0\xD0W` 6`\x03\x19\x01\x12a\0\xD0W`\xFFa\x05na\x08IV[a\x05va\noV[\x16\x80_R`\x03` R`@_ Ta\x05\xB5W_R`\x03` Ra\x05\xB3`@_ a\x05\x9Ea\x08lV[\x90c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01Ra\t\xC8V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FIndexRegistry.createQuorum: quor`D\x82\x01Rpum already exists`x\x1B`d\x82\x01R`\x84\x90\xFD[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0Wa\x02\xC6a\x050a\x06Ta\x066a\x08IV[a\x06>a\x08YV[\x90_` a\x06Ja\x08lV[\x82\x81R\x01Ra\x0C\x1FV[a\t\x94V[4a\0\xD0Wa\x06g6a\x07\xF5V[\x90a\x06pa\noV[a\x06|a\x03\x08\x83a\x08\xC6V[\x92\x82\x84Ra\x06\x89\x83a\x08\xC6V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x81\x81\x10a\x06\xE9W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x06\xCAWPPP\x03\x90\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xBCV[a\x06\xF4\x81\x83\x86a\x08\xDEV[5`\xF8\x1C\x90\x81_R`\x03` Ra\x07\x10`@_ T\x15\x15a\x08\xFEV[a\x07\x19\x82a\x0C[V[`\x01c\xFF\xFF\xFF\xFF\x82T` \x1C\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x07\xE1W\x83a\x07D\x83a\x07\x7F\x93`\x01\x97a\x0E!V[\x80_R`\x02` R`@_ c\xFF\xFF\xFF\xFFa\x07^\x85a\thV[\x16_R` R`@_ T\x15a\x07\x98W[a\x07x\x83a\thV[\x90\x87a\x0BjV[c\xFF\xFF\xFF\xFFa\x07\x8E\x83\x8Aa\t\x80V[\x91\x16\x90R\x01a\x06\x98V[\x80_R`\x02` R`@_ c\xFF\xFF\xFF\xFFa\x07\xB2\x85a\thV[\x16_R` Ra\x07\xDC`@_ a\x07\xC7a\x08lV[\x90c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01Ra\x0B$V[a\x07oV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@`\x03\x19\x82\x01\x12a\0\xD0W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xD0W\x82`#\x82\x01\x12\x15a\0\xD0W\x80`\x04\x015\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\0\xD0W`$\x84\x83\x01\x01\x11a\0\xD0W`$\x01\x91\x90V[`\x045\x90`\xFF\x82\x16\x82\x03a\0\xD0WV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xD0WV[`@Q\x90`@\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x8CW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x08\x8CW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\x8CW`\x05\x1B` \x01\x90V[\x90\x82\x10\x15a\x08\xEAW\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x15a\t\x05WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FIndexRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x07\xE1WV[\x80Q\x82\x10\x15a\x08\xEAW` \x91`\x05\x1B\x01\x01\x90V[\x90`\x01a\t\x9Fa\x08lV[\x92c\xFF\xFF\xFF\xFF\x81T\x16\x84R\x01T` \x83\x01RV[\x80T\x82\x10\x15a\x08\xEAW_R` _ \x01\x90_\x90V[\x80T`\x01`@\x1B\x81\x10\x15a\x08\x8CWa\t\xE5\x91`\x01\x82\x01\x81Ua\t\xB3V[a\n#W\x81Q\x81T` \x93\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x92\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x91\x90\x91\x17\x90UV[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x82\x10\x15a\x08\xEAW_R` _ \x90`\x01\x1B\x01\x90_\x90V[\x90c\xFF\xFF\xFF\xFFa\n]a\x08lV[\x92T\x81\x81\x16\x84R` \x1C\x16` \x83\x01RV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\n\xA1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FIndexRegistry._checkRegistryCoor`D\x82\x01R\x7Fdinator: caller is not the regis`d\x82\x01Rn:9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x89\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x08\x8CWa\x0BA\x91`\x01\x82\x01\x81Ua\n6V[\x91\x90\x91a\n#W` \x81c\xFF\xFF\xFF\xFF\x80`\x01\x94Q\x16\x16c\xFF\xFF\xFF\xFF\x19\x85T\x16\x17\x84U\x01Q\x91\x01UV[\x91`@\x90c\xFF\xFF\xFF\xFF`\xFF\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x94\x86a\x0B\xA2\x85\x83a\x0C\x1FV[\x80TC\x86\x16\x90\x86\x16\x81\x03a\x0B\xE5WP`\x01\x01U[\x16\x91\x82_R`\x01` R\x83_ \x86_R` R\x83_ \x82\x82\x16\x83\x19\x82T\x16\x17\x90U\x83Q\x92\x83R\x16` \x82\x01R\xA2V[\x90Pa\x0C\x1A\x91P\x83\x83\x16_R`\x02` R\x86_ \x85\x87\x16_R` R\x86_ a\x0C\x0Ca\x08lV[\x91\x82R\x89` \x83\x01Ra\x0B$V[a\x0B\xB6V[`\xFF\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x83R\x92\x90R \x80T_\x19\x81\x01\x90\x81\x11a\x07\xE1Wa\x0CW\x91a\n6V[P\x90V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 \x80T_\x19\x81\x01\x90\x81\x11a\x07\xE1Wa\x0CW\x91a\t\xB3V[\x80\x15a\x07\xE1W_\x19\x01\x90V[`\xFF\x16\x90\x81_R`\x03` R`@_ T\x80[a\r-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x82_R`\x03` R`@_ _\x19\x82\x01\x90\x82\x82\x11a\x07\xE1Wa\rR\x91a\x02\x9B\x91a\t\xB3V[c\xFF\xFF\xFF\xFF\x81Q\x16c\xFF\xFF\xFF\xFF\x84\x16\x10\x15a\rwWPa\rq\x90a\x0C\x81V[\x80a\x0C\xA0V[` \x01Qc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[`\xFF\x90\x92\x91\x92\x16\x91\x82_R`\x02` R`@_ c\xFF\xFF\xFF\xFF\x82\x16_R` R`@_ T\x80[a\r\xBBWPPPP_\x90V[\x83_R`\x02` R`@_ c\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ _\x19\x82\x01\x90\x82\x82\x11a\x07\xE1Wa\r\xF0\x91a\x05*\x91a\n6V[c\xFF\xFF\xFF\xFF\x81Q\x16c\xFF\xFF\xFF\xFF\x85\x16\x10\x15a\x0E\x15WPa\x0E\x0F\x90a\x0C\x81V[\x80a\r\xAFV[` \x01Q\x94\x93PPPPV[\x91\x90c\xFF\xFF\xFF\xFF\x81T\x16c\xFF\xFF\xFF\xFFC\x16\x80\x91\x14_\x14a\x0EcWPa\n!\x92P\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x91` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x90UV[\x91\x90P`\xFFa\n!\x93\x16_R`\x03` Rc\xFF\xFF\xFF\xFF`@_ \x91a\x0E\x86a\x08lV[\x93\x84R\x16` \x83\x01Ra\t\xC8V\xFE\xA2dipfsX\"\x12 Z\xBEg\xDD\xEBgM+\xCB\xCC1\xEB\x95\x06\xF7<A\xF0&8\xF5\xD5Qav\xDAw`^\xBF\xF1\xC2dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8062bff04d1461065957806312d1d74d1461061457806326d941f2146105535780632ed583e5146104b95780636d14a987146104755780638121906f1461043957806389026245146102ca578063a48bb0ac1461024d578063bd29b8cd14610132578063caa3cd7614610118578063e2e68580146100d45763f34109221461009c575f80fd5b346100d05760203660031901126100d057602063ffffffff6100c46100bf610849565b610c5b565b54821c16604051908152f35b5f80fd5b346100d05760403660031901126100d05760ff6100ef610849565b165f52600160205260405f206024355f52602052602063ffffffff60405f205416604051908152f35b346100d0575f3660031901126100d05760206040515f8152f35b346100d057610140366107f5565b919061014a610a6f565b4363ffffffff16915f5b84811061015d57005b8061016b60019287866108de565b3560f81c805f52600360205261018660405f205415156108fe565b805f528260205260405f20845f5260205263ffffffff60405f2054166101ab82610c5b565b916101ca6101c263ffffffff855460201c16610968565b809483610e21565b6101d48382610c1f565b928584018963ffffffff8254965416145f14610211575f9150555b828603610200575b50505001610154565b61020992610b6a565b8680806101f7565b50815f52600260205263ffffffff60405f2091165f5260205261024860405f2061023961086c565b908a82525f6020830152610b24565b6101ef565b346100d05760403660031901126100d0576102c66102a161029b61026f610849565b60ff610279610859565b915f602061028561086c565b8281520152165f52600360205260405f206109b3565b50610a4f565b60405191829182919091602063ffffffff816040840195828151168552015116910152565b0390f35b346100d05760403660031901126100d0576102e3610849565b6102eb610859565b9063ffffffff6102fb8383610c8d565b1661030d610308826108c6565b6108a0565b9281845261031a826108c6565b602085019390601f19013685375f5b838110610374578486604051918291602083019060208452518091526040830191905f5b81811061035b575050500390f35b825184528594506020938401939092019160010161034d565b6103858363ffffffff831684610d88565b61038f8288610980565b5261039a8187610980565b51156103a857600101610329565b60405162461bcd60e51b815260206004820152605d60248201527f496e64657852656769737472792e6765744f70657261746f724c69737441744260448201527f6c6f636b4e756d6265723a206f70657261746f7220646f6573206e6f7420657860648201527f6973742061742074686520676976656e20626c6f636b206e756d626572000000608482015260a490fd5b346100d05760203660031901126100d0576102c66102a161047061045b610849565b5f602061046661086c565b8281520152610c5b565b610a4f565b346100d0575f3660031901126100d0576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346100d05760603660031901126100d0576104d2610849565b6104da610859565b6044359163ffffffff831683036100d0576102c6926105309260ff61052a935f602061050461086c565b8281520152165f52600260205263ffffffff60405f2091165f5260205260405f20610a36565b50610994565b60405191829182919091602080604083019463ffffffff81511684520151910152565b346100d05760203660031901126100d05760ff61056e610849565b610576610a6f565b16805f52600360205260405f20546105b5575f5260036020526105b360405f2061059e61086c565b9063ffffffff431682525f60208301526109c8565b005b60405162461bcd60e51b815260206004820152603160248201527f496e64657852656769737472792e63726561746551756f72756d3a2071756f72604482015270756d20616c72656164792065786973747360781b6064820152608490fd5b346100d05760403660031901126100d0576102c6610530610654610636610849565b61063e610859565b905f602061064a61086c565b8281520152610c1f565b610994565b346100d057610667366107f5565b90610670610a6f565b61067c610308836108c6565b92828452610689836108c6565b602085019390601f19013685375f5b8181106106e9578486604051918291602083019060208452518091526040830191905f5b8181106106ca575050500390f35b825163ffffffff168452859450602093840193909201916001016106bc565b6106f48183866108de565b3560f81c90815f52600360205261071060405f205415156108fe565b61071982610c5b565b600163ffffffff825460201c16019063ffffffff82116107e157836107448361077f93600197610e21565b805f52600260205260405f2063ffffffff61075e85610968565b165f5260205260405f205415610798575b61077883610968565b9087610b6a565b63ffffffff61078e838a610980565b9116905201610698565b805f52600260205260405f2063ffffffff6107b285610968565b165f526020526107dc60405f206107c761086c565b9063ffffffff431682525f6020830152610b24565b61076f565b634e487b7160e01b5f52601160045260245ffd5b60406003198201126100d0576004359160243567ffffffffffffffff81116100d057826023820112156100d05780600401359267ffffffffffffffff84116100d057602484830101116100d0576024019190565b6004359060ff821682036100d057565b6024359063ffffffff821682036100d057565b604051906040820182811067ffffffffffffffff82111761088c57604052565b634e487b7160e01b5f52604160045260245ffd5b6040519190601f01601f1916820167ffffffffffffffff81118382101761088c57604052565b67ffffffffffffffff811161088c5760051b60200190565b908210156108ea570190565b634e487b7160e01b5f52603260045260245ffd5b1561090557565b60405162461bcd60e51b815260206004820152603560248201527f496e64657852656769737472792e72656769737465724f70657261746f723a206044820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b6064820152608490fd5b63ffffffff5f199116019063ffffffff82116107e157565b80518210156108ea5760209160051b010190565b90600161099f61086c565b9263ffffffff815416845201546020830152565b80548210156108ea575f5260205f2001905f90565b8054600160401b81101561088c576109e5916001820181556109b3565b610a23578151815460209384015167ffffffffffffffff1990911663ffffffff9290921691909117921b67ffffffff0000000016919091179055565b565b634e487b7160e01b5f525f60045260245ffd5b80548210156108ea575f5260205f209060011b01905f90565b9063ffffffff610a5d61086c565b9254818116845260201c166020830152565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303610aa157565b60405162461bcd60e51b815260206004820152604f60248201527f496e64657852656769737472792e5f636865636b5265676973747279436f6f7260448201527f64696e61746f723a2063616c6c6572206973206e6f742074686520726567697360648201526e3a393c9031b7b7b93234b730ba37b960891b608482015260a490fd5b8054600160401b81101561088c57610b4191600182018155610a36565b919091610a235760208163ffffffff8060019451161663ffffffff198554161784550151910155565b9160409063ffffffff60ff7f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f69486610ba28583610c1f565b80544386169086168103610be55750600101555b1691825f526001602052835f20865f52602052835f2082821683198254161790558351928352166020820152a2565b9050610c1a91508383165f526002602052865f208587165f52602052865f20610c0c61086c565b918252896020830152610b24565b610bb6565b60ff165f90815260026020908152604080832063ffffffff9490941683529290522080545f1981019081116107e157610c5791610a36565b5090565b60ff165f90815260036020526040902080545f1981019081116107e157610c57916109b3565b80156107e1575f190190565b60ff1690815f52600360205260405f2054805b610d2d5760405162461bcd60e51b815260206004820152605560248201527f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c60448201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060648201527430ba1033b4bb32b710313637b1b590373ab6b132b960591b608482015260a490fd5b825f52600360205260405f205f198201908282116107e157610d529161029b916109b3565b63ffffffff81511663ffffffff84161015610d775750610d7190610c81565b80610ca0565b6020015163ffffffff169392505050565b60ff909291921691825f52600260205260405f2063ffffffff82165f5260205260405f2054805b610dbb57505050505f90565b835f52600260205260405f2063ffffffff83165f5260205260405f205f198201908282116107e157610df09161052a91610a36565b63ffffffff81511663ffffffff85161015610e155750610e0f90610c81565b80610daf565b60200151949350505050565b919063ffffffff81541663ffffffff43168091145f14610e635750610a2192509067ffffffff0000000082549160201b169067ffffffff000000001916179055565b91905060ff610a2193165f52600360205263ffffffff60405f2091610e8661086c565b9384521660208301526109c856fea26469706673582212205abe67ddeb674d2bcbcc31eb9506f73c41f02638f5d5516176da77605ebff1c264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\xBF\xF0M\x14a\x06YW\x80c\x12\xD1\xD7M\x14a\x06\x14W\x80c&\xD9A\xF2\x14a\x05SW\x80c.\xD5\x83\xE5\x14a\x04\xB9W\x80cm\x14\xA9\x87\x14a\x04uW\x80c\x81!\x90o\x14a\x049W\x80c\x89\x02bE\x14a\x02\xCAW\x80c\xA4\x8B\xB0\xAC\x14a\x02MW\x80c\xBD)\xB8\xCD\x14a\x012W\x80c\xCA\xA3\xCDv\x14a\x01\x18W\x80c\xE2\xE6\x85\x80\x14a\0\xD4Wc\xF3A\t\"\x14a\0\x9CW_\x80\xFD[4a\0\xD0W` 6`\x03\x19\x01\x12a\0\xD0W` c\xFF\xFF\xFF\xFFa\0\xC4a\0\xBFa\x08IV[a\x0C[V[T\x82\x1C\x16`@Q\x90\x81R\xF3[_\x80\xFD[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0W`\xFFa\0\xEFa\x08IV[\x16_R`\x01` R`@_ `$5_R` R` c\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[4a\0\xD0W_6`\x03\x19\x01\x12a\0\xD0W` `@Q_\x81R\xF3[4a\0\xD0Wa\x01@6a\x07\xF5V[\x91\x90a\x01Ja\noV[Cc\xFF\xFF\xFF\xFF\x16\x91_[\x84\x81\x10a\x01]W\0[\x80a\x01k`\x01\x92\x87\x86a\x08\xDEV[5`\xF8\x1C\x80_R`\x03` Ra\x01\x86`@_ T\x15\x15a\x08\xFEV[\x80_R\x82` R`@_ \x84_R` Rc\xFF\xFF\xFF\xFF`@_ T\x16a\x01\xAB\x82a\x0C[V[\x91a\x01\xCAa\x01\xC2c\xFF\xFF\xFF\xFF\x85T` \x1C\x16a\thV[\x80\x94\x83a\x0E!V[a\x01\xD4\x83\x82a\x0C\x1FV[\x92\x85\x84\x01\x89c\xFF\xFF\xFF\xFF\x82T\x96T\x16\x14_\x14a\x02\x11W_\x91PU[\x82\x86\x03a\x02\0W[PPP\x01a\x01TV[a\x02\t\x92a\x0BjV[\x86\x80\x80a\x01\xF7V[P\x81_R`\x02` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` Ra\x02H`@_ a\x029a\x08lV[\x90\x8A\x82R_` \x83\x01Ra\x0B$V[a\x01\xEFV[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0Wa\x02\xC6a\x02\xA1a\x02\x9Ba\x02oa\x08IV[`\xFFa\x02ya\x08YV[\x91_` a\x02\x85a\x08lV[\x82\x81R\x01R\x16_R`\x03` R`@_ a\t\xB3V[Pa\nOV[`@Q\x91\x82\x91\x82\x91\x90\x91` c\xFF\xFF\xFF\xFF\x81`@\x84\x01\x95\x82\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x03\x90\xF3[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0Wa\x02\xE3a\x08IV[a\x02\xEBa\x08YV[\x90c\xFF\xFF\xFF\xFFa\x02\xFB\x83\x83a\x0C\x8DV[\x16a\x03\ra\x03\x08\x82a\x08\xC6V[a\x08\xA0V[\x92\x81\x84Ra\x03\x1A\x82a\x08\xC6V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x83\x81\x10a\x03tW\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x03[WPPP\x03\x90\xF3[\x82Q\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x03MV[a\x03\x85\x83c\xFF\xFF\xFF\xFF\x83\x16\x84a\r\x88V[a\x03\x8F\x82\x88a\t\x80V[Ra\x03\x9A\x81\x87a\t\x80V[Q\x15a\x03\xA8W`\x01\x01a\x03)V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`]`$\x82\x01R\x7FIndexRegistry.getOperatorListAtB`D\x82\x01R\x7FlockNumber: operator does not ex`d\x82\x01R\x7Fist at the given block number\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[4a\0\xD0W` 6`\x03\x19\x01\x12a\0\xD0Wa\x02\xC6a\x02\xA1a\x04pa\x04[a\x08IV[_` a\x04fa\x08lV[\x82\x81R\x01Ra\x0C[V[a\nOV[4a\0\xD0W_6`\x03\x19\x01\x12a\0\xD0W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\0\xD0W``6`\x03\x19\x01\x12a\0\xD0Wa\x04\xD2a\x08IV[a\x04\xDAa\x08YV[`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\0\xD0Wa\x02\xC6\x92a\x050\x92`\xFFa\x05*\x93_` a\x05\x04a\x08lV[\x82\x81R\x01R\x16_R`\x02` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`@_ a\n6V[Pa\t\x94V[`@Q\x91\x82\x91\x82\x91\x90\x91` \x80`@\x83\x01\x94c\xFF\xFF\xFF\xFF\x81Q\x16\x84R\x01Q\x91\x01RV[4a\0\xD0W` 6`\x03\x19\x01\x12a\0\xD0W`\xFFa\x05na\x08IV[a\x05va\noV[\x16\x80_R`\x03` R`@_ Ta\x05\xB5W_R`\x03` Ra\x05\xB3`@_ a\x05\x9Ea\x08lV[\x90c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01Ra\t\xC8V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FIndexRegistry.createQuorum: quor`D\x82\x01Rpum already exists`x\x1B`d\x82\x01R`\x84\x90\xFD[4a\0\xD0W`@6`\x03\x19\x01\x12a\0\xD0Wa\x02\xC6a\x050a\x06Ta\x066a\x08IV[a\x06>a\x08YV[\x90_` a\x06Ja\x08lV[\x82\x81R\x01Ra\x0C\x1FV[a\t\x94V[4a\0\xD0Wa\x06g6a\x07\xF5V[\x90a\x06pa\noV[a\x06|a\x03\x08\x83a\x08\xC6V[\x92\x82\x84Ra\x06\x89\x83a\x08\xC6V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x81\x81\x10a\x06\xE9W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x06\xCAWPPP\x03\x90\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xBCV[a\x06\xF4\x81\x83\x86a\x08\xDEV[5`\xF8\x1C\x90\x81_R`\x03` Ra\x07\x10`@_ T\x15\x15a\x08\xFEV[a\x07\x19\x82a\x0C[V[`\x01c\xFF\xFF\xFF\xFF\x82T` \x1C\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x07\xE1W\x83a\x07D\x83a\x07\x7F\x93`\x01\x97a\x0E!V[\x80_R`\x02` R`@_ c\xFF\xFF\xFF\xFFa\x07^\x85a\thV[\x16_R` R`@_ T\x15a\x07\x98W[a\x07x\x83a\thV[\x90\x87a\x0BjV[c\xFF\xFF\xFF\xFFa\x07\x8E\x83\x8Aa\t\x80V[\x91\x16\x90R\x01a\x06\x98V[\x80_R`\x02` R`@_ c\xFF\xFF\xFF\xFFa\x07\xB2\x85a\thV[\x16_R` Ra\x07\xDC`@_ a\x07\xC7a\x08lV[\x90c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01Ra\x0B$V[a\x07oV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@`\x03\x19\x82\x01\x12a\0\xD0W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\0\xD0W\x82`#\x82\x01\x12\x15a\0\xD0W\x80`\x04\x015\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\0\xD0W`$\x84\x83\x01\x01\x11a\0\xD0W`$\x01\x91\x90V[`\x045\x90`\xFF\x82\x16\x82\x03a\0\xD0WV[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\0\xD0WV[`@Q\x90`@\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x8CW`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q\x91\x90`\x1F\x01`\x1F\x19\x16\x82\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x83\x82\x10\x17a\x08\x8CW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x08\x8CW`\x05\x1B` \x01\x90V[\x90\x82\x10\x15a\x08\xEAW\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x15a\t\x05WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FIndexRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x90\xFD[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x07\xE1WV[\x80Q\x82\x10\x15a\x08\xEAW` \x91`\x05\x1B\x01\x01\x90V[\x90`\x01a\t\x9Fa\x08lV[\x92c\xFF\xFF\xFF\xFF\x81T\x16\x84R\x01T` \x83\x01RV[\x80T\x82\x10\x15a\x08\xEAW_R` _ \x01\x90_\x90V[\x80T`\x01`@\x1B\x81\x10\x15a\x08\x8CWa\t\xE5\x91`\x01\x82\x01\x81Ua\t\xB3V[a\n#W\x81Q\x81T` \x93\x84\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x92\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x91\x90\x91\x17\x90UV[V[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x82\x10\x15a\x08\xEAW_R` _ \x90`\x01\x1B\x01\x90_\x90V[\x90c\xFF\xFF\xFF\xFFa\n]a\x08lV[\x92T\x81\x81\x16\x84R` \x1C\x16` \x83\x01RV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\n\xA1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FIndexRegistry._checkRegistryCoor`D\x82\x01R\x7Fdinator: caller is not the regis`d\x82\x01Rn:9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x89\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x08\x8CWa\x0BA\x91`\x01\x82\x01\x81Ua\n6V[\x91\x90\x91a\n#W` \x81c\xFF\xFF\xFF\xFF\x80`\x01\x94Q\x16\x16c\xFF\xFF\xFF\xFF\x19\x85T\x16\x17\x84U\x01Q\x91\x01UV[\x91`@\x90c\xFF\xFF\xFF\xFF`\xFF\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x94\x86a\x0B\xA2\x85\x83a\x0C\x1FV[\x80TC\x86\x16\x90\x86\x16\x81\x03a\x0B\xE5WP`\x01\x01U[\x16\x91\x82_R`\x01` R\x83_ \x86_R` R\x83_ \x82\x82\x16\x83\x19\x82T\x16\x17\x90U\x83Q\x92\x83R\x16` \x82\x01R\xA2V[\x90Pa\x0C\x1A\x91P\x83\x83\x16_R`\x02` R\x86_ \x85\x87\x16_R` R\x86_ a\x0C\x0Ca\x08lV[\x91\x82R\x89` \x83\x01Ra\x0B$V[a\x0B\xB6V[`\xFF\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x83R\x92\x90R \x80T_\x19\x81\x01\x90\x81\x11a\x07\xE1Wa\x0CW\x91a\n6V[P\x90V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 \x80T_\x19\x81\x01\x90\x81\x11a\x07\xE1Wa\x0CW\x91a\t\xB3V[\x80\x15a\x07\xE1W_\x19\x01\x90V[`\xFF\x16\x90\x81_R`\x03` R`@_ T\x80[a\r-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x82_R`\x03` R`@_ _\x19\x82\x01\x90\x82\x82\x11a\x07\xE1Wa\rR\x91a\x02\x9B\x91a\t\xB3V[c\xFF\xFF\xFF\xFF\x81Q\x16c\xFF\xFF\xFF\xFF\x84\x16\x10\x15a\rwWPa\rq\x90a\x0C\x81V[\x80a\x0C\xA0V[` \x01Qc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[`\xFF\x90\x92\x91\x92\x16\x91\x82_R`\x02` R`@_ c\xFF\xFF\xFF\xFF\x82\x16_R` R`@_ T\x80[a\r\xBBWPPPP_\x90V[\x83_R`\x02` R`@_ c\xFF\xFF\xFF\xFF\x83\x16_R` R`@_ _\x19\x82\x01\x90\x82\x82\x11a\x07\xE1Wa\r\xF0\x91a\x05*\x91a\n6V[c\xFF\xFF\xFF\xFF\x81Q\x16c\xFF\xFF\xFF\xFF\x85\x16\x10\x15a\x0E\x15WPa\x0E\x0F\x90a\x0C\x81V[\x80a\r\xAFV[` \x01Q\x94\x93PPPPV[\x91\x90c\xFF\xFF\xFF\xFF\x81T\x16c\xFF\xFF\xFF\xFFC\x16\x80\x91\x14_\x14a\x0EcWPa\n!\x92P\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x91` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x90UV[\x91\x90P`\xFFa\n!\x93\x16_R`\x03` Rc\xFF\xFF\xFF\xFF`@_ \x91a\x0E\x86a\x08lV[\x93\x84R\x16` \x83\x01Ra\t\xC8V\xFE\xA2dipfsX\"\x12 Z\xBEg\xDD\xEBgM+\xCB\xCC1\xEB\x95\x06\xF7<A\xF0&8\xF5\xD5Qav\xDAw`^\xBF\xF1\xC2dsolcC\0\x08\x1B\x003",
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "QuorumIndexUpdate(bytes32,uint8,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    110u8, 225u8, 228u8, 244u8, 7u8, 95u8, 61u8, 6u8, 113u8, 118u8, 20u8, 13u8,
                    52u8, 232u8, 120u8, 116u8, 36u8, 77u8, 210u8, 115u8, 41u8, 76u8, 5u8, 178u8,
                    33u8, 129u8, 51u8, 228u8, 154u8, 43u8, 166u8, 246u8,
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
                        &self.newOperatorIndex,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPERATOR_DOES_NOT_EXIST_IDCall> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_DOES_NOT_EXIST_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_DOES_NOT_EXIST_IDCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<OPERATOR_DOES_NOT_EXIST_IDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_DOES_NOT_EXIST_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_DOES_NOT_EXIST_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_DOES_NOT_EXIST_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_DOES_NOT_EXIST_IDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::FixedBytes<32>);
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
            impl ::core::convert::From<currentOperatorIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentOperatorIndexCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentOperatorIndexCall {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<currentOperatorIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentOperatorIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentOperatorIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentOperatorIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestOperatorUpdateCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorUpdateCall) -> Self {
                    (value.quorumNumber, value.operatorIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestOperatorUpdateCall {
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
            type UnderlyingRustTuple<'a> =
                (<IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getLatestOperatorUpdateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestOperatorUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestOperatorUpdateReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestOperatorUpdateReturn;
            type ReturnTuple<'a> = (IIndexRegistry::OperatorUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.operatorIndex,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getLatestQuorumUpdateCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestQuorumUpdateCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestQuorumUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getLatestQuorumUpdateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestQuorumUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestQuorumUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestQuorumUpdateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestQuorumUpdateReturn;
            type ReturnTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorListAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorListAtBlockNumberCall) -> Self {
                    (value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorListAtBlockNumberCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
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
            impl ::core::convert::From<getOperatorListAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorListAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorListAtBlockNumberReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorListAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorIndex, value.arrayIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorUpdateAtIndexCall {
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
            type UnderlyingRustTuple<'a> =
                (<IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getOperatorUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorUpdateAtIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IIndexRegistry::OperatorUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.operatorIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.arrayIndex,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getQuorumUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.quorumIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumUpdateAtIndexCall {
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
            type UnderlyingRustTuple<'a> =
                (<IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getQuorumUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumUpdateAtIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IIndexRegistry::QuorumUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumIndex,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
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
            impl ::core::convert::From<initializeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
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
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalOperatorsForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalOperatorsForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalOperatorsForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalOperatorsForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalOperatorsForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
            )
                -> alloy_sol_types::Result<IndexRegistryCalls>] = &[
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IndexRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(IndexRegistryCalls::totalOperatorsForQuorum)
                    }
                    totalOperatorsForQuorum
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
                Self::OPERATOR_DOES_NOT_EXIST_ID(inner) => {
                    <OPERATOR_DOES_NOT_EXIST_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentOperatorIndex(inner) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getLatestOperatorUpdate(inner) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestQuorumUpdate(inner) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                        inner, out,
                    )
                }
                Self::currentOperatorIndex(inner) => {
                    <currentOperatorIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getLatestOperatorUpdate(inner) => {
                    <getLatestOperatorUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getLatestQuorumUpdate(inner) => {
                    <getLatestQuorumUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getOperatorListAtBlockNumber(inner) => {
                    <getOperatorListAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getOperatorUpdateAtIndex(inner) => {
                    <getOperatorUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getQuorumUpdateAtIndex(inner) => {
                    <getQuorumUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::totalOperatorsForQuorum(inner) => {
                    <totalOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
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
                110u8, 225u8, 228u8, 244u8, 7u8, 95u8, 61u8, 6u8, 113u8, 118u8, 20u8, 13u8, 52u8,
                232u8, 120u8, 116u8, 36u8, 77u8, 210u8, 115u8, 41u8, 76u8, 5u8, 178u8, 33u8, 129u8,
                51u8, 228u8, 154u8, 43u8, 166u8, 246u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
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
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<QuorumIndexUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumIndexUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumIndexUpdate)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<IndexRegistryInstance<T, P, N>>>
    {
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
            f.debug_tuple("IndexRegistryInstance")
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
        > IndexRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IndexRegistry`](self) contract instance.

        See the [wrapper's documentation](`IndexRegistryInstance`) for more details.*/
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
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
        > IndexRegistryInstance<T, P, N>
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
            self.call_builder(&deregisterOperatorCall {
                operatorId,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`getLatestOperatorUpdate`] function.
        pub fn getLatestOperatorUpdate(
            &self,
            quorumNumber: u8,
            operatorIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestOperatorUpdateCall, N> {
            self.call_builder(&getLatestOperatorUpdateCall {
                quorumNumber,
                operatorIndex,
            })
        }
        ///Creates a new call builder for the [`getLatestQuorumUpdate`] function.
        pub fn getLatestQuorumUpdate(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestQuorumUpdateCall, N> {
            self.call_builder(&getLatestQuorumUpdateCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getOperatorListAtBlockNumber`] function.
        pub fn getOperatorListAtBlockNumber(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorListAtBlockNumberCall, N> {
            self.call_builder(&getOperatorListAtBlockNumberCall {
                quorumNumber,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getOperatorUpdateAtIndex`] function.
        pub fn getOperatorUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorIndex: u32,
            arrayIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorUpdateAtIndexCall, N> {
            self.call_builder(&getOperatorUpdateAtIndexCall {
                quorumNumber,
                operatorIndex,
                arrayIndex,
            })
        }
        ///Creates a new call builder for the [`getQuorumUpdateAtIndex`] function.
        pub fn getQuorumUpdateAtIndex(
            &self,
            quorumNumber: u8,
            quorumIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumUpdateAtIndexCall, N> {
            self.call_builder(&getQuorumUpdateAtIndexCall {
                quorumNumber,
                quorumIndex,
            })
        }
        ///Creates a new call builder for the [`initializeQuorum`] function.
        pub fn initializeQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeQuorumCall, N> {
            self.call_builder(&initializeQuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall {
                operatorId,
                quorumNumbers,
            })
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
            self.call_builder(&totalOperatorsForQuorumCall { quorumNumber })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IndexRegistryInstance<T, P, N>
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
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
