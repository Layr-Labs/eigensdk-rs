///Module containing a contract's types and functions.
/**

```solidity
library IIndexRegistry {
    struct OperatorUpdate { uint32 fromBlockNumber; bytes32 operatorId; }
    struct QuorumUpdate { uint32 fromBlockNumber; uint32 numOperators; }
}
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod IIndexRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct OperatorUpdate { uint32 fromBlockNumber; bytes32 operatorId; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OperatorUpdate {
        pub fromBlockNumber: u32,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct QuorumUpdate {
        pub fromBlockNumber: u32,
        pub numOperators: u32,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod IndexRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561001057600080fd5b5060405161136138038061136183398101604081905261002f9161010c565b6001600160a01b0381166080528061004561004c565b505061013c565b600054610100900460ff16156100b85760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101561010a576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b60006020828403121561011e57600080fd5b81516001600160a01b038116811461013557600080fd5b9392505050565b60805161120361015e60003960008181610142015261085a01526112036000f3fe608060405234801561001057600080fd5b50600436106100b35760003560e01c8063890262451161007157806389026245146101b3578063a48bb0ac146101d3578063bd29b8cd146101e6578063caa3cd76146101f9578063e2e685801461020f578063f34109221461025557600080fd5b8062bff04d146100b857806312d1d74d146100e157806326d941f2146101155780632ed583e51461012a5780636d14a9871461013d5780638121906f1461017c575b600080fd5b6100cb6100c6366004610ebd565b610268565b6040516100d89190610f39565b60405180910390f35b6100f46100ef366004610fad565b61038a565b60408051825163ffffffff16815260209283015192810192909252016100d8565b610128610123366004610fe0565b6103d0565b005b6100f4610138366004610ffb565b6104b4565b6101647f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100d8565b61018f61018a366004610fe0565b61053a565b60408051825163ffffffff90811682526020938401511692810192909252016100d8565b6101c66101c1366004610fad565b610581565b6040516100d8919061103e565b61018f6101e1366004610fad565b6106eb565b6101286101f4366004610ebd565b610762565b610201600081565b6040519081526020016100d8565b61024061021d366004611076565b600160209081526000928352604080842090915290825290205463ffffffff1681565b60405163ffffffff90911681526020016100d8565b610240610263366004610fe0565b610830565b606061027261084f565b60008267ffffffffffffffff81111561028d5761028d6110a0565b6040519080825280602002602001820160405280156102b6578160200160208202803683370190505b50905060005b8381101561037f5760008585838181106102d8576102d86110b6565b919091013560f81c60008181526003602052604090205490925090508061031a5760405162461bcd60e51b8152600401610311906110cc565b60405180910390fd5b600061032583610905565b905061033c8984610337600185611137565b6109fe565b8085858151811061034f5761034f6110b6565b602002602001019063ffffffff16908163ffffffff168152505050505080806103779061115c565b9150506102bc565b5090505b9392505050565b60408051808201909152600080825260208201526103a88383610a88565b60408051808201909152815463ffffffff168152600190910154602082015290505b92915050565b6103d861084f565b60ff8116600090815260036020526040902054156104525760405162461bcd60e51b815260206004820152603160248201527f496e64657852656769737472792e63726561746551756f72756d3a2071756f72604482015270756d20616c72656164792065786973747360781b6064820152608401610311565b60ff166000908152600360209081526040808320815180830190925263ffffffff438116835282840185815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055565b604080518082019091526000808252602082015260ff8416600090815260026020908152604080832063ffffffff80881685529252909120805490918416908110610501576105016110b6565b600091825260209182902060408051808201909152600290920201805463ffffffff168252600101549181019190915290509392505050565b604080518082019091526000808252602082015261055782610ae0565b60408051808201909152905463ffffffff8082168352600160201b90910416602082015292915050565b6060600061058f8484610b22565b905060008163ffffffff1667ffffffffffffffff8111156105b2576105b26110a0565b6040519080825280602002602001820160405280156105db578160200160208202803683370190505b50905060005b8263ffffffff168110156106e2576105fa868287610c57565b82828151811061060c5761060c6110b6565b6020026020010181815250506000801b82828151811061062e5761062e6110b6565b602002602001015114156106d05760405162461bcd60e51b815260206004820152605d60248201527f496e64657852656769737472792e6765744f70657261746f724c69737441744260448201527f6c6f636b4e756d6265723a206f70657261746f7220646f6573206e6f7420657860648201527f6973742061742074686520676976656e20626c6f636b206e756d626572000000608482015260a401610311565b806106da8161115c565b9150506105e1565b50949350505050565b604080518082019091526000808252602082015260ff83166000908152600360205260409020805463ffffffff8416908110610729576107296110b6565b60009182526020918290206040805180820190915291015463ffffffff8082168352600160201b90910416918101919091529392505050565b61076a61084f565b60005b8181101561082a576000838383818110610789576107896110b6565b919091013560f81c6000818152600360205260409020549092509050806107c25760405162461bcd60e51b8152600401610311906110cc565b60ff8216600090815260016020908152604080832089845290915281205463ffffffff16906107f084610d2e565b905060006107fe8583610d68565b9050808914610812576108128186856109fe565b505050505080806108229061115c565b91505061076d565b50505050565b600061083b82610ae0565b54600160201b900463ffffffff1692915050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109035760405162461bcd60e51b815260206004820152604d60248201527f496e64657852656769737472792e6f6e6c795265676973747279436f6f72646960448201527f6e61746f723a2063616c6c6572206973206e6f7420746865207265676973747260648201526c3c9031b7b7b93234b730ba37b960991b608482015260a401610311565b565b60008061091183610ae0565b805490915060009061093190600160201b900463ffffffff166001611177565b905061093e848383610d92565b60ff841660009081526002602052604081209061095c600184611137565b63ffffffff1681526020810191909152604001600020546103835760ff8416600090815260026020526040812090610995600184611137565b63ffffffff908116825260208083019390935260409182016000908120835180850190945243831684528385018281528154600180820184559284529590922093516002909502909301805463ffffffff19169490921693909317815591519101559392505050565b6000610a0a8383610a88565b9050610a1883838387610e32565b60ff83166000818152600160209081526040808320888452825291829020805463ffffffff191663ffffffff871690811790915582519384529083015285917f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6910160405180910390a250505050565b60ff8216600090815260026020908152604080832063ffffffff851684529091528120805490610ab960018361119f565b81548110610ac957610ac96110b6565b906000526020600020906002020191505092915050565b60ff81166000908152600360205260408120805490610b0060018361119f565b81548110610b1057610b106110b6565b90600052602060002001915050919050565b60ff8216600090815260036020526040812054805b8015610bca5760ff85166000908152600360205260408120610b5a60018461119f565b81548110610b6a57610b6a6110b6565b60009182526020918290206040805180820190915291015463ffffffff808216808452600160201b90920481169383019390935290925090861610610bb7576020015192506103ca915050565b5080610bc2816111b6565b915050610b37565b5060405162461bcd60e51b815260206004820152605560248201527f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c60448201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060648201527430ba1033b4bb32b710313637b1b590373ab6b132b960591b608482015260a401610311565b60ff8316600090815260026020908152604080832063ffffffff86168452909152812054805b8015610d225760ff8616600090815260026020908152604080832063ffffffff891684529091528120610cb160018461119f565b81548110610cc157610cc16110b6565b600091825260209182902060408051808201909152600290920201805463ffffffff9081168084526001909201549383019390935290925090861610610d0f57602001519250610383915050565b5080610d1a816111b6565b915050610c7d565b50600095945050505050565b600080610d3a83610ae0565b8054909150600090610d5b90600190600160201b900463ffffffff16611137565b9050610383848383610d92565b600080610d758484610a88565b6001810154909150610d8a8585846000610e32565b949350505050565b81544363ffffffff90811691161415610dc957815463ffffffff8216600160201b0267ffffffff0000000019909116178255505050565b60ff83166000908152600360209081526040808320815180830190925263ffffffff438116835285811683850190815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055505050565b81544363ffffffff90811691161415610e51576001820181905561082a565b60ff93909316600090815260026020818152604080842063ffffffff968716855282528084208151808301909252438716825281830197885280546001808201835591865292909420905191909202909101805463ffffffff1916919094161783559251919092015550565b600080600060408486031215610ed257600080fd5b83359250602084013567ffffffffffffffff80821115610ef157600080fd5b818601915086601f830112610f0557600080fd5b813581811115610f1457600080fd5b876020828501011115610f2657600080fd5b6020830194508093505050509250925092565b6020808252825182820181905260009190848201906040850190845b81811015610f7757835163ffffffff1683529284019291840191600101610f55565b50909695505050505050565b803560ff81168114610f9457600080fd5b919050565b803563ffffffff81168114610f9457600080fd5b60008060408385031215610fc057600080fd5b610fc983610f83565b9150610fd760208401610f99565b90509250929050565b600060208284031215610ff257600080fd5b61038382610f83565b60008060006060848603121561101057600080fd5b61101984610f83565b925061102760208501610f99565b915061103560408501610f99565b90509250925092565b6020808252825182820181905260009190848201906040850190845b81811015610f775783518352928401929184019160010161105a565b6000806040838503121561108957600080fd5b61109283610f83565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b60208082526035908201527f496e64657852656769737472792e72656769737465724f70657261746f723a206040820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8381169083168181101561115457611154611121565b039392505050565b600060001982141561117057611170611121565b5060010190565b600063ffffffff80831681851680830382111561119657611196611121565b01949350505050565b6000828210156111b1576111b1611121565b500390565b6000816111c5576111c5611121565b50600019019056fea26469706673582212200dd424985d748126cfecb042df9778973b6cadcf9f9b71b13b2ee065b53d26c464736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x13a8\x03\x80a\x13a\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x0CV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80a\0Ea\0LV[PPa\x01<V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15a\x01\nW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15a\x01\x1EW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x015W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x12\x03a\x01^`\09`\0\x81\x81a\x01B\x01Ra\x08Z\x01Ra\x12\x03`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB3W`\x005`\xE0\x1C\x80c\x89\x02bE\x11a\0qW\x80c\x89\x02bE\x14a\x01\xB3W\x80c\xA4\x8B\xB0\xAC\x14a\x01\xD3W\x80c\xBD)\xB8\xCD\x14a\x01\xE6W\x80c\xCA\xA3\xCDv\x14a\x01\xF9W\x80c\xE2\xE6\x85\x80\x14a\x02\x0FW\x80c\xF3A\t\"\x14a\x02UW`\0\x80\xFD[\x80b\xBF\xF0M\x14a\0\xB8W\x80c\x12\xD1\xD7M\x14a\0\xE1W\x80c&\xD9A\xF2\x14a\x01\x15W\x80c.\xD5\x83\xE5\x14a\x01*W\x80cm\x14\xA9\x87\x14a\x01=W\x80c\x81!\x90o\x14a\x01|W[`\0\x80\xFD[a\0\xCBa\0\xC66`\x04a\x0E\xBDV[a\x02hV[`@Qa\0\xD8\x91\x90a\x0F9V[`@Q\x80\x91\x03\x90\xF3[a\0\xF4a\0\xEF6`\x04a\x0F\xADV[a\x03\x8AV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01(a\x01#6`\x04a\x0F\xE0V[a\x03\xD0V[\0[a\0\xF4a\x0186`\x04a\x0F\xFBV[a\x04\xB4V[a\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD8V[a\x01\x8Fa\x01\x8A6`\x04a\x0F\xE0V[a\x05:V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01\xC6a\x01\xC16`\x04a\x0F\xADV[a\x05\x81V[`@Qa\0\xD8\x91\x90a\x10>V[a\x01\x8Fa\x01\xE16`\x04a\x0F\xADV[a\x06\xEBV[a\x01(a\x01\xF46`\x04a\x0E\xBDV[a\x07bV[a\x02\x01`\0\x81V[`@Q\x90\x81R` \x01a\0\xD8V[a\x02@a\x02\x1D6`\x04a\x10vV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD8V[a\x02@a\x02c6`\x04a\x0F\xE0V[a\x080V[``a\x02ra\x08OV[`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x8DWa\x02\x8Da\x10\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xB6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x03\x7FW`\0\x85\x85\x83\x81\x81\x10a\x02\xD8Wa\x02\xD8a\x10\xB6V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x03\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x11\x90a\x10\xCCV[`@Q\x80\x91\x03\x90\xFD[`\0a\x03%\x83a\t\x05V[\x90Pa\x03<\x89\x84a\x037`\x01\x85a\x117V[a\t\xFEV[\x80\x85\x85\x81Q\x81\x10a\x03OWa\x03Oa\x10\xB6V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPPPP\x80\x80a\x03w\x90a\x11\\V[\x91PPa\x02\xBCV[P\x90P[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x03\xA8\x83\x83a\n\x88V[`@\x80Q\x80\x82\x01\x90\x91R\x81Tc\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01T` \x82\x01R\x90P[\x92\x91PPV[a\x03\xD8a\x08OV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x90 T\x15a\x04RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FIndexRegistry.createQuorum: quor`D\x82\x01Rpum already exists`x\x1B`d\x82\x01R`\x84\x01a\x03\x11V[`\xFF\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x82\x84\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x92R\x90\x91 \x80T\x90\x91\x84\x16\x90\x81\x10a\x05\x01Wa\x05\x01a\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x90P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05W\x82a\n\xE0V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16` \x82\x01R\x92\x91PPV[```\0a\x05\x8F\x84\x84a\x0B\"V[\x90P`\0\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xB2Wa\x05\xB2a\x10\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xDBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x06\xE2Wa\x05\xFA\x86\x82\x87a\x0CWV[\x82\x82\x81Q\x81\x10a\x06\x0CWa\x06\x0Ca\x10\xB6V[` \x02` \x01\x01\x81\x81RPP`\0\x80\x1B\x82\x82\x81Q\x81\x10a\x06.Wa\x06.a\x10\xB6V[` \x02` \x01\x01Q\x14\x15a\x06\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`]`$\x82\x01R\x7FIndexRegistry.getOperatorListAtB`D\x82\x01R\x7FlockNumber: operator does not ex`d\x82\x01R\x7Fist at the given block number\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\x11V[\x80a\x06\xDA\x81a\x11\\V[\x91PPa\x05\xE1V[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x07)Wa\x07)a\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x07ja\x08OV[`\0[\x81\x81\x10\x15a\x08*W`\0\x83\x83\x83\x81\x81\x10a\x07\x89Wa\x07\x89a\x10\xB6V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x07\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x11\x90a\x10\xCCV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x81 Tc\xFF\xFF\xFF\xFF\x16\x90a\x07\xF0\x84a\r.V[\x90P`\0a\x07\xFE\x85\x83a\rhV[\x90P\x80\x89\x14a\x08\x12Wa\x08\x12\x81\x86\x85a\t\xFEV[PPPPP\x80\x80a\x08\"\x90a\x11\\V[\x91PPa\x07mV[PPPPV[`\0a\x08;\x82a\n\xE0V[T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FIndexRegistry.onlyRegistryCoordi`D\x82\x01R\x7Fnator: caller is not the registr`d\x82\x01Rl<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x99\x1B`\x84\x82\x01R`\xA4\x01a\x03\x11V[V[`\0\x80a\t\x11\x83a\n\xE0V[\x80T\x90\x91P`\0\x90a\t1\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x11wV[\x90Pa\t>\x84\x83\x83a\r\x92V[`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\t\\`\x01\x84a\x117V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta\x03\x83W`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\t\x95`\x01\x84a\x117V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x83Q\x80\x85\x01\x90\x94RC\x83\x16\x84R\x83\x85\x01\x82\x81R\x81T`\x01\x80\x82\x01\x84U\x92\x84R\x95\x90\x92 \x93Q`\x02\x90\x95\x02\x90\x93\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x91Q\x91\x01U\x93\x92PPPV[`\0a\n\n\x83\x83a\n\x88V[\x90Pa\n\x18\x83\x83\x83\x87a\x0E2V[`\xFF\x83\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x85\x91\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\xFF\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x90a\n\xB9`\x01\x83a\x11\x9FV[\x81T\x81\x10a\n\xC9Wa\n\xC9a\x10\xB6V[\x90`\0R` `\0 \x90`\x02\x02\x01\x91PP\x92\x91PPV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x90a\x0B\0`\x01\x83a\x11\x9FV[\x81T\x81\x10a\x0B\x10Wa\x0B\x10a\x10\xB6V[\x90`\0R` `\0 \x01\x91PP\x91\x90PV[`\xFF\x82\x16`\0\x90\x81R`\x03` R`@\x81 T\x80[\x80\x15a\x0B\xCAW`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x81 a\x0BZ`\x01\x84a\x11\x9FV[\x81T\x81\x10a\x0BjWa\x0Bja\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\x0B\xB7W` \x01Q\x92Pa\x03\xCA\x91PPV[P\x80a\x0B\xC2\x81a\x11\xB6V[\x91PPa\x0B7V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x01a\x03\x11V[`\xFF\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a\r\"W`\xFF\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 a\x0C\xB1`\x01\x84a\x11\x9FV[\x81T\x81\x10a\x0C\xC1Wa\x0C\xC1a\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\r\x0FW` \x01Q\x92Pa\x03\x83\x91PPV[P\x80a\r\x1A\x81a\x11\xB6V[\x91PPa\x0C}V[P`\0\x95\x94PPPPPV[`\0\x80a\r:\x83a\n\xE0V[\x80T\x90\x91P`\0\x90a\r[\x90`\x01\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x117V[\x90Pa\x03\x83\x84\x83\x83a\r\x92V[`\0\x80a\ru\x84\x84a\n\x88V[`\x01\x81\x01T\x90\x91Pa\r\x8A\x85\x85\x84`\0a\x0E2V[\x94\x93PPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\r\xC9W\x81Tc\xFF\xFF\xFF\xFF\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x17\x82UPPPV[`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x85\x81\x16\x83\x85\x01\x90\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x0EQW`\x01\x82\x01\x81\x90Ua\x08*V[`\xFF\x93\x90\x93\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x96\x87\x16\x85R\x82R\x80\x84 \x81Q\x80\x83\x01\x90\x92RC\x87\x16\x82R\x81\x83\x01\x97\x88R\x80T`\x01\x80\x82\x01\x83U\x91\x86R\x92\x90\x94 \x90Q\x91\x90\x92\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x94\x16\x17\x83U\x92Q\x91\x90\x92\x01UPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0E\xD2W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xF1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F\x05W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\x14W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x0F&W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0FwW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0FUV[P\x90\x96\x95PPPPPPV[\x805`\xFF\x81\x16\x81\x14a\x0F\x94W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x94W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xC0W`\0\x80\xFD[a\x0F\xC9\x83a\x0F\x83V[\x91Pa\x0F\xD7` \x84\x01a\x0F\x99V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xF2W`\0\x80\xFD[a\x03\x83\x82a\x0F\x83V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\x10W`\0\x80\xFD[a\x10\x19\x84a\x0F\x83V[\x92Pa\x10'` \x85\x01a\x0F\x99V[\x91Pa\x105`@\x85\x01a\x0F\x99V[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0FwW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10ZV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x89W`\0\x80\xFD[a\x10\x92\x83a\x0F\x83V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`5\x90\x82\x01R\x7FIndexRegistry.registerOperator: `@\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x11TWa\x11Ta\x11!V[\x03\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a\x11pWa\x11pa\x11!V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x11\x96Wa\x11\x96a\x11!V[\x01\x94\x93PPPPV[`\0\x82\x82\x10\x15a\x11\xB1Wa\x11\xB1a\x11!V[P\x03\x90V[`\0\x81a\x11\xC5Wa\x11\xC5a\x11!V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \r\xD4$\x98]t\x81&\xCF\xEC\xB0B\xDF\x97x\x97;l\xAD\xCF\x9F\x9Bq\xB1;.\xE0e\xB5=&\xC4dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106100b35760003560e01c8063890262451161007157806389026245146101b3578063a48bb0ac146101d3578063bd29b8cd146101e6578063caa3cd76146101f9578063e2e685801461020f578063f34109221461025557600080fd5b8062bff04d146100b857806312d1d74d146100e157806326d941f2146101155780632ed583e51461012a5780636d14a9871461013d5780638121906f1461017c575b600080fd5b6100cb6100c6366004610ebd565b610268565b6040516100d89190610f39565b60405180910390f35b6100f46100ef366004610fad565b61038a565b60408051825163ffffffff16815260209283015192810192909252016100d8565b610128610123366004610fe0565b6103d0565b005b6100f4610138366004610ffb565b6104b4565b6101647f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100d8565b61018f61018a366004610fe0565b61053a565b60408051825163ffffffff90811682526020938401511692810192909252016100d8565b6101c66101c1366004610fad565b610581565b6040516100d8919061103e565b61018f6101e1366004610fad565b6106eb565b6101286101f4366004610ebd565b610762565b610201600081565b6040519081526020016100d8565b61024061021d366004611076565b600160209081526000928352604080842090915290825290205463ffffffff1681565b60405163ffffffff90911681526020016100d8565b610240610263366004610fe0565b610830565b606061027261084f565b60008267ffffffffffffffff81111561028d5761028d6110a0565b6040519080825280602002602001820160405280156102b6578160200160208202803683370190505b50905060005b8381101561037f5760008585838181106102d8576102d86110b6565b919091013560f81c60008181526003602052604090205490925090508061031a5760405162461bcd60e51b8152600401610311906110cc565b60405180910390fd5b600061032583610905565b905061033c8984610337600185611137565b6109fe565b8085858151811061034f5761034f6110b6565b602002602001019063ffffffff16908163ffffffff168152505050505080806103779061115c565b9150506102bc565b5090505b9392505050565b60408051808201909152600080825260208201526103a88383610a88565b60408051808201909152815463ffffffff168152600190910154602082015290505b92915050565b6103d861084f565b60ff8116600090815260036020526040902054156104525760405162461bcd60e51b815260206004820152603160248201527f496e64657852656769737472792e63726561746551756f72756d3a2071756f72604482015270756d20616c72656164792065786973747360781b6064820152608401610311565b60ff166000908152600360209081526040808320815180830190925263ffffffff438116835282840185815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055565b604080518082019091526000808252602082015260ff8416600090815260026020908152604080832063ffffffff80881685529252909120805490918416908110610501576105016110b6565b600091825260209182902060408051808201909152600290920201805463ffffffff168252600101549181019190915290509392505050565b604080518082019091526000808252602082015261055782610ae0565b60408051808201909152905463ffffffff8082168352600160201b90910416602082015292915050565b6060600061058f8484610b22565b905060008163ffffffff1667ffffffffffffffff8111156105b2576105b26110a0565b6040519080825280602002602001820160405280156105db578160200160208202803683370190505b50905060005b8263ffffffff168110156106e2576105fa868287610c57565b82828151811061060c5761060c6110b6565b6020026020010181815250506000801b82828151811061062e5761062e6110b6565b602002602001015114156106d05760405162461bcd60e51b815260206004820152605d60248201527f496e64657852656769737472792e6765744f70657261746f724c69737441744260448201527f6c6f636b4e756d6265723a206f70657261746f7220646f6573206e6f7420657860648201527f6973742061742074686520676976656e20626c6f636b206e756d626572000000608482015260a401610311565b806106da8161115c565b9150506105e1565b50949350505050565b604080518082019091526000808252602082015260ff83166000908152600360205260409020805463ffffffff8416908110610729576107296110b6565b60009182526020918290206040805180820190915291015463ffffffff8082168352600160201b90910416918101919091529392505050565b61076a61084f565b60005b8181101561082a576000838383818110610789576107896110b6565b919091013560f81c6000818152600360205260409020549092509050806107c25760405162461bcd60e51b8152600401610311906110cc565b60ff8216600090815260016020908152604080832089845290915281205463ffffffff16906107f084610d2e565b905060006107fe8583610d68565b9050808914610812576108128186856109fe565b505050505080806108229061115c565b91505061076d565b50505050565b600061083b82610ae0565b54600160201b900463ffffffff1692915050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109035760405162461bcd60e51b815260206004820152604d60248201527f496e64657852656769737472792e6f6e6c795265676973747279436f6f72646960448201527f6e61746f723a2063616c6c6572206973206e6f7420746865207265676973747260648201526c3c9031b7b7b93234b730ba37b960991b608482015260a401610311565b565b60008061091183610ae0565b805490915060009061093190600160201b900463ffffffff166001611177565b905061093e848383610d92565b60ff841660009081526002602052604081209061095c600184611137565b63ffffffff1681526020810191909152604001600020546103835760ff8416600090815260026020526040812090610995600184611137565b63ffffffff908116825260208083019390935260409182016000908120835180850190945243831684528385018281528154600180820184559284529590922093516002909502909301805463ffffffff19169490921693909317815591519101559392505050565b6000610a0a8383610a88565b9050610a1883838387610e32565b60ff83166000818152600160209081526040808320888452825291829020805463ffffffff191663ffffffff871690811790915582519384529083015285917f6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6910160405180910390a250505050565b60ff8216600090815260026020908152604080832063ffffffff851684529091528120805490610ab960018361119f565b81548110610ac957610ac96110b6565b906000526020600020906002020191505092915050565b60ff81166000908152600360205260408120805490610b0060018361119f565b81548110610b1057610b106110b6565b90600052602060002001915050919050565b60ff8216600090815260036020526040812054805b8015610bca5760ff85166000908152600360205260408120610b5a60018461119f565b81548110610b6a57610b6a6110b6565b60009182526020918290206040805180820190915291015463ffffffff808216808452600160201b90920481169383019390935290925090861610610bb7576020015192506103ca915050565b5080610bc2816111b6565b915050610b37565b5060405162461bcd60e51b815260206004820152605560248201527f496e64657852656769737472792e5f6f70657261746f72436f756e744174426c60448201527f6f636b4e756d6265723a2071756f72756d20646964206e6f742065786973742060648201527430ba1033b4bb32b710313637b1b590373ab6b132b960591b608482015260a401610311565b60ff8316600090815260026020908152604080832063ffffffff86168452909152812054805b8015610d225760ff8616600090815260026020908152604080832063ffffffff891684529091528120610cb160018461119f565b81548110610cc157610cc16110b6565b600091825260209182902060408051808201909152600290920201805463ffffffff9081168084526001909201549383019390935290925090861610610d0f57602001519250610383915050565b5080610d1a816111b6565b915050610c7d565b50600095945050505050565b600080610d3a83610ae0565b8054909150600090610d5b90600190600160201b900463ffffffff16611137565b9050610383848383610d92565b600080610d758484610a88565b6001810154909150610d8a8585846000610e32565b949350505050565b81544363ffffffff90811691161415610dc957815463ffffffff8216600160201b0267ffffffff0000000019909116178255505050565b60ff83166000908152600360209081526040808320815180830190925263ffffffff438116835285811683850190815282546001810184559286529390942091519101805492518416600160201b0267ffffffffffffffff199093169190931617179055505050565b81544363ffffffff90811691161415610e51576001820181905561082a565b60ff93909316600090815260026020818152604080842063ffffffff968716855282528084208151808301909252438716825281830197885280546001808201835591865292909420905191909202909101805463ffffffff1916919094161783559251919092015550565b600080600060408486031215610ed257600080fd5b83359250602084013567ffffffffffffffff80821115610ef157600080fd5b818601915086601f830112610f0557600080fd5b813581811115610f1457600080fd5b876020828501011115610f2657600080fd5b6020830194508093505050509250925092565b6020808252825182820181905260009190848201906040850190845b81811015610f7757835163ffffffff1683529284019291840191600101610f55565b50909695505050505050565b803560ff81168114610f9457600080fd5b919050565b803563ffffffff81168114610f9457600080fd5b60008060408385031215610fc057600080fd5b610fc983610f83565b9150610fd760208401610f99565b90509250929050565b600060208284031215610ff257600080fd5b61038382610f83565b60008060006060848603121561101057600080fd5b61101984610f83565b925061102760208501610f99565b915061103560408501610f99565b90509250925092565b6020808252825182820181905260009190848201906040850190845b81811015610f775783518352928401929184019160010161105a565b6000806040838503121561108957600080fd5b61109283610f83565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b60208082526035908201527f496e64657852656769737472792e72656769737465724f70657261746f723a206040820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8381169083168181101561115457611154611121565b039392505050565b600060001982141561117057611170611121565b5060010190565b600063ffffffff80831681851680830382111561119657611196611121565b01949350505050565b6000828210156111b1576111b1611121565b500390565b6000816111c5576111c5611121565b50600019019056fea26469706673582212200dd424985d748126cfecb042df9778973b6cadcf9f9b71b13b2ee065b53d26c464736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB3W`\x005`\xE0\x1C\x80c\x89\x02bE\x11a\0qW\x80c\x89\x02bE\x14a\x01\xB3W\x80c\xA4\x8B\xB0\xAC\x14a\x01\xD3W\x80c\xBD)\xB8\xCD\x14a\x01\xE6W\x80c\xCA\xA3\xCDv\x14a\x01\xF9W\x80c\xE2\xE6\x85\x80\x14a\x02\x0FW\x80c\xF3A\t\"\x14a\x02UW`\0\x80\xFD[\x80b\xBF\xF0M\x14a\0\xB8W\x80c\x12\xD1\xD7M\x14a\0\xE1W\x80c&\xD9A\xF2\x14a\x01\x15W\x80c.\xD5\x83\xE5\x14a\x01*W\x80cm\x14\xA9\x87\x14a\x01=W\x80c\x81!\x90o\x14a\x01|W[`\0\x80\xFD[a\0\xCBa\0\xC66`\x04a\x0E\xBDV[a\x02hV[`@Qa\0\xD8\x91\x90a\x0F9V[`@Q\x80\x91\x03\x90\xF3[a\0\xF4a\0\xEF6`\x04a\x0F\xADV[a\x03\x8AV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01(a\x01#6`\x04a\x0F\xE0V[a\x03\xD0V[\0[a\0\xF4a\x0186`\x04a\x0F\xFBV[a\x04\xB4V[a\x01d\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD8V[a\x01\x8Fa\x01\x8A6`\x04a\x0F\xE0V[a\x05:V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\0\xD8V[a\x01\xC6a\x01\xC16`\x04a\x0F\xADV[a\x05\x81V[`@Qa\0\xD8\x91\x90a\x10>V[a\x01\x8Fa\x01\xE16`\x04a\x0F\xADV[a\x06\xEBV[a\x01(a\x01\xF46`\x04a\x0E\xBDV[a\x07bV[a\x02\x01`\0\x81V[`@Q\x90\x81R` \x01a\0\xD8V[a\x02@a\x02\x1D6`\x04a\x10vV[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD8V[a\x02@a\x02c6`\x04a\x0F\xE0V[a\x080V[``a\x02ra\x08OV[`\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x8DWa\x02\x8Da\x10\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x02\xB6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x03\x7FW`\0\x85\x85\x83\x81\x81\x10a\x02\xD8Wa\x02\xD8a\x10\xB6V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x03\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x11\x90a\x10\xCCV[`@Q\x80\x91\x03\x90\xFD[`\0a\x03%\x83a\t\x05V[\x90Pa\x03<\x89\x84a\x037`\x01\x85a\x117V[a\t\xFEV[\x80\x85\x85\x81Q\x81\x10a\x03OWa\x03Oa\x10\xB6V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPPPP\x80\x80a\x03w\x90a\x11\\V[\x91PPa\x02\xBCV[P\x90P[\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x03\xA8\x83\x83a\n\x88V[`@\x80Q\x80\x82\x01\x90\x91R\x81Tc\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01T` \x82\x01R\x90P[\x92\x91PPV[a\x03\xD8a\x08OV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x90 T\x15a\x04RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FIndexRegistry.createQuorum: quor`D\x82\x01Rpum already exists`x\x1B`d\x82\x01R`\x84\x01a\x03\x11V[`\xFF\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x82\x84\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x84\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x92R\x90\x91 \x80T\x90\x91\x84\x16\x90\x81\x10a\x05\x01Wa\x05\x01a\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x01T\x91\x81\x01\x91\x90\x91R\x90P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05W\x82a\n\xE0V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16` \x82\x01R\x92\x91PPV[```\0a\x05\x8F\x84\x84a\x0B\"V[\x90P`\0\x81c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xB2Wa\x05\xB2a\x10\xA0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xDBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x06\xE2Wa\x05\xFA\x86\x82\x87a\x0CWV[\x82\x82\x81Q\x81\x10a\x06\x0CWa\x06\x0Ca\x10\xB6V[` \x02` \x01\x01\x81\x81RPP`\0\x80\x1B\x82\x82\x81Q\x81\x10a\x06.Wa\x06.a\x10\xB6V[` \x02` \x01\x01Q\x14\x15a\x06\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`]`$\x82\x01R\x7FIndexRegistry.getOperatorListAtB`D\x82\x01R\x7FlockNumber: operator does not ex`d\x82\x01R\x7Fist at the given block number\0\0\0`\x84\x82\x01R`\xA4\x01a\x03\x11V[\x80a\x06\xDA\x81a\x11\\V[\x91PPa\x05\xE1V[P\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x07)Wa\x07)a\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x07ja\x08OV[`\0[\x81\x81\x10\x15a\x08*W`\0\x83\x83\x83\x81\x81\x10a\x07\x89Wa\x07\x89a\x10\xB6V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x03` R`@\x90 T\x90\x92P\x90P\x80a\x07\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x11\x90a\x10\xCCV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x89\x84R\x90\x91R\x81 Tc\xFF\xFF\xFF\xFF\x16\x90a\x07\xF0\x84a\r.V[\x90P`\0a\x07\xFE\x85\x83a\rhV[\x90P\x80\x89\x14a\x08\x12Wa\x08\x12\x81\x86\x85a\t\xFEV[PPPPP\x80\x80a\x08\"\x90a\x11\\V[\x91PPa\x07mV[PPPPV[`\0a\x08;\x82a\n\xE0V[T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FIndexRegistry.onlyRegistryCoordi`D\x82\x01R\x7Fnator: caller is not the registr`d\x82\x01Rl<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x99\x1B`\x84\x82\x01R`\xA4\x01a\x03\x11V[V[`\0\x80a\t\x11\x83a\n\xE0V[\x80T\x90\x91P`\0\x90a\t1\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01a\x11wV[\x90Pa\t>\x84\x83\x83a\r\x92V[`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\t\\`\x01\x84a\x117V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta\x03\x83W`\xFF\x84\x16`\0\x90\x81R`\x02` R`@\x81 \x90a\t\x95`\x01\x84a\x117V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0\x90\x81 \x83Q\x80\x85\x01\x90\x94RC\x83\x16\x84R\x83\x85\x01\x82\x81R\x81T`\x01\x80\x82\x01\x84U\x92\x84R\x95\x90\x92 \x93Q`\x02\x90\x95\x02\x90\x93\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x91Q\x91\x01U\x93\x92PPPV[`\0a\n\n\x83\x83a\n\x88V[\x90Pa\n\x18\x83\x83\x83\x87a\x0E2V[`\xFF\x83\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x85\x91\x7Fn\xE1\xE4\xF4\x07_=\x06qv\x14\r4\xE8xt$M\xD2s)L\x05\xB2!\x813\xE4\x9A+\xA6\xF6\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPV[`\xFF\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x85\x16\x84R\x90\x91R\x81 \x80T\x90a\n\xB9`\x01\x83a\x11\x9FV[\x81T\x81\x10a\n\xC9Wa\n\xC9a\x10\xB6V[\x90`\0R` `\0 \x90`\x02\x02\x01\x91PP\x92\x91PPV[`\xFF\x81\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x90a\x0B\0`\x01\x83a\x11\x9FV[\x81T\x81\x10a\x0B\x10Wa\x0B\x10a\x10\xB6V[\x90`\0R` `\0 \x01\x91PP\x91\x90PV[`\xFF\x82\x16`\0\x90\x81R`\x03` R`@\x81 T\x80[\x80\x15a\x0B\xCAW`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x81 a\x0BZ`\x01\x84a\x11\x9FV[\x81T\x81\x10a\x0BjWa\x0Bja\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\x0B\xB7W` \x01Q\x92Pa\x03\xCA\x91PPV[P\x80a\x0B\xC2\x81a\x11\xB6V[\x91PPa\x0B7V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FIndexRegistry._operatorCountAtBl`D\x82\x01R\x7FockNumber: quorum did not exist `d\x82\x01Rt0\xBA\x103\xB4\xBB2\xB7\x10167\xB1\xB5\x907:\xB6\xB12\xB9`Y\x1B`\x84\x82\x01R`\xA4\x01a\x03\x11V[`\xFF\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a\r\"W`\xFF\x86\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 a\x0C\xB1`\x01\x84a\x11\x9FV[\x81T\x81\x10a\x0C\xC1Wa\x0C\xC1a\x10\xB6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x90\x92\x02\x01\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x86\x16\x10a\r\x0FW` \x01Q\x92Pa\x03\x83\x91PPV[P\x80a\r\x1A\x81a\x11\xB6V[\x91PPa\x0C}V[P`\0\x95\x94PPPPPV[`\0\x80a\r:\x83a\n\xE0V[\x80T\x90\x91P`\0\x90a\r[\x90`\x01\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x117V[\x90Pa\x03\x83\x84\x83\x83a\r\x92V[`\0\x80a\ru\x84\x84a\n\x88V[`\x01\x81\x01T\x90\x91Pa\r\x8A\x85\x85\x84`\0a\x0E2V[\x94\x93PPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\r\xC9W\x81Tc\xFF\xFF\xFF\xFF\x82\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x17\x82UPPPV[`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92Rc\xFF\xFF\xFF\xFFC\x81\x16\x83R\x85\x81\x16\x83\x85\x01\x90\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x91Q\x91\x01\x80T\x92Q\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPPPV[\x81TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x0EQW`\x01\x82\x01\x81\x90Ua\x08*V[`\xFF\x93\x90\x93\x16`\0\x90\x81R`\x02` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x96\x87\x16\x85R\x82R\x80\x84 \x81Q\x80\x83\x01\x90\x92RC\x87\x16\x82R\x81\x83\x01\x97\x88R\x80T`\x01\x80\x82\x01\x83U\x91\x86R\x92\x90\x94 \x90Q\x91\x90\x92\x02\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x94\x16\x17\x83U\x92Q\x91\x90\x92\x01UPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0E\xD2W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xF1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0F\x05W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\x14W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x0F&W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0FwW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0FUV[P\x90\x96\x95PPPPPPV[\x805`\xFF\x81\x16\x81\x14a\x0F\x94W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x94W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xC0W`\0\x80\xFD[a\x0F\xC9\x83a\x0F\x83V[\x91Pa\x0F\xD7` \x84\x01a\x0F\x99V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xF2W`\0\x80\xFD[a\x03\x83\x82a\x0F\x83V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\x10W`\0\x80\xFD[a\x10\x19\x84a\x0F\x83V[\x92Pa\x10'` \x85\x01a\x0F\x99V[\x91Pa\x105`@\x85\x01a\x0F\x99V[\x90P\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0FwW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10ZV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x89W`\0\x80\xFD[a\x10\x92\x83a\x0F\x83V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`5\x90\x82\x01R\x7FIndexRegistry.registerOperator: `@\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x11TWa\x11Ta\x11!V[\x03\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a\x11pWa\x11pa\x11!V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\x11\x96Wa\x11\x96a\x11!V[\x01\x94\x93PPPPV[`\0\x82\x82\x10\x15a\x11\xB1Wa\x11\xB1a\x11!V[P\x03\x90V[`\0\x81a\x11\xC5Wa\x11\xC5a\x11!V[P`\0\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \r\xD4$\x98]t\x81&\xCF\xEC\xB0B\xDF\x97x\x97;l\xAD\xCF\x9F\x9Bq\xB1;.\xE0e\xB5=&\xC4dsolcC\0\x08\x0C\x003",
    );
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
    /**Event with signature `QuorumIndexUpdate(bytes32,uint8,uint32)` and selector `0x6ee1e4f4075f3d067176140d34e87874244dd273294c05b2218133e49a2ba6f6`.
    ```solidity
    event QuorumIndexUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint32 newOperatorIndex);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct QuorumIndexUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub newOperatorIndex: u32,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OPERATOR_DOES_NOT_EXIST_IDCall {}
    ///Container type for the return parameters of the [`OPERATOR_DOES_NOT_EXIST_ID()`](OPERATOR_DOES_NOT_EXIST_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OPERATOR_DOES_NOT_EXIST_IDReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct currentOperatorIndexCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`currentOperatorIndex(uint8,bytes32)`](currentOperatorIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct currentOperatorIndexReturn {
        pub _0: u32,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(bytes32,bytes)`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLatestOperatorUpdateCall {
        pub quorumNumber: u8,
        pub operatorIndex: u32,
    }
    ///Container type for the return parameters of the [`getLatestOperatorUpdate(uint8,uint32)`](getLatestOperatorUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLatestOperatorUpdateReturn {
        pub _0: <IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLatestQuorumUpdateCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getLatestQuorumUpdate(uint8)`](getLatestQuorumUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getLatestQuorumUpdateReturn {
        pub _0: <IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorListAtBlockNumberCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorListAtBlockNumber(uint8,uint32)`](getOperatorListAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorListAtBlockNumberReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub operatorIndex: u32,
        pub arrayIndex: u32,
    }
    ///Container type for the return parameters of the [`getOperatorUpdateAtIndex(uint8,uint32,uint32)`](getOperatorUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getOperatorUpdateAtIndexReturn {
        pub _0: <IIndexRegistry::OperatorUpdate as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getQuorumUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub quorumIndex: u32,
    }
    ///Container type for the return parameters of the [`getQuorumUpdateAtIndex(uint8,uint32)`](getQuorumUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getQuorumUpdateAtIndexReturn {
        pub _0: <IIndexRegistry::QuorumUpdate as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`initializeQuorum(uint8)`](initializeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeQuorumReturn {}
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(bytes32,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
        pub _0: alloy::sol_types::private::Vec<u32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
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
    /**Function with signature `totalOperatorsForQuorum(uint8)` and selector `0xf3410922`.
    ```solidity
    function totalOperatorsForQuorum(uint8 quorumNumber) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`totalOperatorsForQuorum(uint8)`](totalOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct totalOperatorsForQuorumReturn {
        pub _0: u32,
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
