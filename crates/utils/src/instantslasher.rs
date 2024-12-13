///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IAllocationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub wadsToSlash: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub description: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
        impl ::core::convert::From<SlashingParams> for UnderlyingRustTuple<'_> {
            fn from(value: SlashingParams) -> Self {
                (
                    value.operator,
                    value.operatorSetId,
                    value.strategies,
                    value.wadsToSlash,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashingParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorSetId: tuple.1,
                    strategies: tuple.2,
                    wadsToSlash: tuple.3,
                    description: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SlashingParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SlashingParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
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
        impl alloy_sol_types::SolType for SlashingParams {
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
        impl alloy_sol_types::SolStruct for SlashingParams {
            const NAME: &'static str = "SlashingParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256[] wadsToSlash,string description)",
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
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.wadsToSlash)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SlashingParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wadsToSlash,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
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
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wadsToSlash,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
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
    /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAllocationManagerTypesInstance<T, P, N> {
        IAllocationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IAllocationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAllocationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAllocationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAllocationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAllocationManagerTypesInstance")
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
    > IAllocationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IAllocationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAllocationManagerTypesInstance<T, P, N> {
            IAllocationManagerTypesInstance {
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
    > IAllocationManagerTypesInstance<T, P, N> {
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
    > IAllocationManagerTypesInstance<T, P, N> {
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
library IAllocationManagerTypes {
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        address[] strategies;
        uint256[] wadsToSlash;
        string description;
    }
}

interface InstantSlasher {
    event Initialized(uint8 version);
    event OperatorSlashed(uint256 indexed slashingRequestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);
    event SlashingRequestCancelled(uint256 indexed requestId);
    event SlashingRequested(uint256 indexed requestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);

    function fulfillSlashingRequest(IAllocationManagerTypes.SlashingParams memory _slashingParams) external;
    function initialize(address _serviceManager, address _slasher) external;
    function nextRequestId() external view returns (uint256);
    function serviceManager() external view returns (address);
    function slasher() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "fulfillSlashingRequest",
    "inputs": [
      {
        "name": "_slashingParams",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.SlashingParams",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "wadsToSlash",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
          }
        ]
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
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "nextRequestId",
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
    "name": "serviceManager",
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
    "name": "slasher",
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
    "name": "OperatorSlashed",
    "inputs": [
      {
        "name": "slashingRequestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorSetId",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "wadsToSlash",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "description",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashingRequestCancelled",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SlashingRequested",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorSetId",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "wadsToSlash",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "description",
        "type": "string",
        "indexed": false,
        "internalType": "string"
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
pub mod InstantSlasher {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506111218061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80633998fdd314610059578063485cc955146100775780636a669b41146100935780636a84a985146100af578063b1344271146100cd575b5f5ffd5b6100616100eb565b60405161006e9190610560565b60405180910390f35b610091600480360381019061008c91906105b4565b610110565b005b6100ad60048036038101906100a89190610a3e565b61024e565b005b6100b761027e565b6040516100c49190610a94565b60405180910390f35b6100d5610284565b6040516100e29190610560565b60405180910390f35b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f60019054906101000a900460ff16159050808015610140575060015f5f9054906101000a900460ff1660ff16105b8061016d575061014f306102a9565b15801561016c575060015f5f9054906101000a900460ff1660ff16145b5b6101ac576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016101a390610b2d565b60405180910390fd5b60015f5f6101000a81548160ff021916908360ff16021790555080156101e75760015f60016101000a81548160ff0219169083151502179055505b6101f183836102cb565b8015610249575f5f60016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860016040516102409190610b99565b60405180910390a15b505050565b6102573361039d565b5f60025f81548092919061026a90610bdf565b91905055905061027a818361042f565b5050565b60025481565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f60019054906101000a900460ff16610319576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161031090610c96565b60405180910390fd5b815f60026101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508060015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461042c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161042390610d24565b60405180910390fd5b50565b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16633d071422826040518263ffffffff1660e01b81526004016104899190610ff2565b5f604051808303815f87803b1580156104a0575f5ffd5b505af11580156104b2573d5f5f3e3d5ffd5b50505050806020015163ffffffff16815f015173ffffffffffffffffffffffffffffffffffffffff16837f8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b846060015185608001516040516105159291906110b6565b60405180910390a45050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61054a82610521565b9050919050565b61055a81610540565b82525050565b5f6020820190506105735f830184610551565b92915050565b5f604051905090565b5f5ffd5b5f5ffd5b61059381610540565b811461059d575f5ffd5b50565b5f813590506105ae8161058a565b92915050565b5f5f604083850312156105ca576105c9610582565b5b5f6105d7858286016105a0565b92505060206105e8858286016105a0565b9150509250929050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61063c826105f6565b810181811067ffffffffffffffff8211171561065b5761065a610606565b5b80604052505050565b5f61066d610579565b90506106798282610633565b919050565b5f5ffd5b5f63ffffffff82169050919050565b61069a81610682565b81146106a4575f5ffd5b50565b5f813590506106b581610691565b92915050565b5f5ffd5b5f67ffffffffffffffff8211156106d9576106d8610606565b5b602082029050602081019050919050565b5f5ffd5b5f6106f882610540565b9050919050565b610708816106ee565b8114610712575f5ffd5b50565b5f81359050610723816106ff565b92915050565b5f61073b610736846106bf565b610664565b9050808382526020820190506020840283018581111561075e5761075d6106ea565b5b835b8181101561078757806107738882610715565b845260208401935050602081019050610760565b5050509392505050565b5f82601f8301126107a5576107a46106bb565b5b81356107b5848260208601610729565b91505092915050565b5f67ffffffffffffffff8211156107d8576107d7610606565b5b602082029050602081019050919050565b5f819050919050565b6107fb816107e9565b8114610805575f5ffd5b50565b5f81359050610816816107f2565b92915050565b5f61082e610829846107be565b610664565b90508083825260208201905060208402830185811115610851576108506106ea565b5b835b8181101561087a57806108668882610808565b845260208401935050602081019050610853565b5050509392505050565b5f82601f830112610898576108976106bb565b5b81356108a884826020860161081c565b91505092915050565b5f5ffd5b5f67ffffffffffffffff8211156108cf576108ce610606565b5b6108d8826105f6565b9050602081019050919050565b828183375f83830152505050565b5f610905610900846108b5565b610664565b905082815260208101848484011115610921576109206108b1565b5b61092c8482856108e5565b509392505050565b5f82601f830112610948576109476106bb565b5b81356109588482602086016108f3565b91505092915050565b5f60a08284031215610976576109756105f2565b5b61098060a0610664565b90505f61098f848285016105a0565b5f8301525060206109a2848285016106a7565b602083015250604082013567ffffffffffffffff8111156109c6576109c561067e565b5b6109d284828501610791565b604083015250606082013567ffffffffffffffff8111156109f6576109f561067e565b5b610a0284828501610884565b606083015250608082013567ffffffffffffffff811115610a2657610a2561067e565b5b610a3284828501610934565b60808301525092915050565b5f60208284031215610a5357610a52610582565b5b5f82013567ffffffffffffffff811115610a7057610a6f610586565b5b610a7c84828501610961565b91505092915050565b610a8e816107e9565b82525050565b5f602082019050610aa75f830184610a85565b92915050565b5f82825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c7265615f8201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b5f610b17602e83610aad565b9150610b2282610abd565b604082019050919050565b5f6020820190508181035f830152610b4481610b0b565b9050919050565b5f819050919050565b5f60ff82169050919050565b5f819050919050565b5f610b83610b7e610b7984610b4b565b610b60565b610b54565b9050919050565b610b9381610b69565b82525050565b5f602082019050610bac5f830184610b8a565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610be9826107e9565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610c1b57610c1a610bb2565b5b600182019050919050565b7f496e697469616c697a61626c653a20636f6e7472616374206973206e6f7420695f8201527f6e697469616c697a696e67000000000000000000000000000000000000000000602082015250565b5f610c80602b83610aad565b9150610c8b82610c26565b604082019050919050565b5f6020820190508181035f830152610cad81610c74565b9050919050565b7f536c6173686572426173652e5f636865636b536c61736865723a2063616c6c655f8201527f72206973206e6f742074686520736c6173686572000000000000000000000000602082015250565b5f610d0e603483610aad565b9150610d1982610cb4565b604082019050919050565b5f6020820190508181035f830152610d3b81610d02565b9050919050565b610d4b81610540565b82525050565b610d5a81610682565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f610da3610d9e610d9984610521565b610b60565b610521565b9050919050565b5f610db482610d89565b9050919050565b5f610dc582610daa565b9050919050565b610dd581610dbb565b82525050565b5f610de68383610dcc565b60208301905092915050565b5f602082019050919050565b5f610e0882610d60565b610e128185610d6a565b9350610e1d83610d7a565b805f5b83811015610e4d578151610e348882610ddb565b9750610e3f83610df2565b925050600181019050610e20565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b610e8c816107e9565b82525050565b5f610e9d8383610e83565b60208301905092915050565b5f602082019050919050565b5f610ebf82610e5a565b610ec98185610e64565b9350610ed483610e74565b805f5b83811015610f04578151610eeb8882610e92565b9750610ef683610ea9565b925050600181019050610ed7565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f610f4382610f11565b610f4d8185610f1b565b9350610f5d818560208601610f2b565b610f66816105f6565b840191505092915050565b5f60a083015f830151610f865f860182610d42565b506020830151610f996020860182610d51565b5060408301518482036040860152610fb18282610dfe565b91505060608301518482036060860152610fcb8282610eb5565b91505060808301518482036080860152610fe58282610f39565b9150508091505092915050565b5f6020820190508181035f83015261100a8184610f71565b905092915050565b5f82825260208201905092915050565b5f61102c82610e5a565b6110368185611012565b935061104183610e74565b805f5b838110156110715781516110588882610e92565b975061106383610ea9565b925050600181019050611044565b5085935050505092915050565b5f61108882610f11565b6110928185610aad565b93506110a2818560208601610f2b565b6110ab816105f6565b840191505092915050565b5f6040820190508181035f8301526110ce8185611022565b905081810360208301526110e2818461107e565b9050939250505056fea2646970667358221220c12618329a7639d1247b2d1aabb2666116855eebf18bb23f6a8b483056f98d7b64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x11!\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80c9\x98\xFD\xD3\x14a\0YW\x80cH\\\xC9U\x14a\0wW\x80cjf\x9BA\x14a\0\x93W\x80cj\x84\xA9\x85\x14a\0\xAFW\x80c\xB14Bq\x14a\0\xCDW[__\xFD[a\0aa\0\xEBV[`@Qa\0n\x91\x90a\x05`V[`@Q\x80\x91\x03\x90\xF3[a\0\x91`\x04\x806\x03\x81\x01\x90a\0\x8C\x91\x90a\x05\xB4V[a\x01\x10V[\0[a\0\xAD`\x04\x806\x03\x81\x01\x90a\0\xA8\x91\x90a\n>V[a\x02NV[\0[a\0\xB7a\x02~V[`@Qa\0\xC4\x91\x90a\n\x94V[`@Q\x80\x91\x03\x90\xF3[a\0\xD5a\x02\x84V[`@Qa\0\xE2\x91\x90a\x05`V[`@Q\x80\x91\x03\x90\xF3[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x90P\x80\x80\x15a\x01@WP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[\x80a\x01mWPa\x01O0a\x02\xA9V[\x15\x80\x15a\x01lWP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x14[[a\x01\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xA3\x90a\x0B-V[`@Q\x80\x91\x03\x90\xFD[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x15a\x01\xE7W`\x01_`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[a\x01\xF1\x83\x83a\x02\xCBV[\x80\x15a\x02IW__`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\x01`@Qa\x02@\x91\x90a\x0B\x99V[`@Q\x80\x91\x03\x90\xA1[PPPV[a\x02W3a\x03\x9DV[_`\x02_\x81T\x80\x92\x91\x90a\x02j\x90a\x0B\xDFV[\x91\x90PU\x90Pa\x02z\x81\x83a\x04/V[PPV[`\x02T\x81V[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x03\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x10\x90a\x0C\x96V[`@Q\x80\x91\x03\x90\xFD[\x81_`\x02a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04#\x90a\r$V[`@Q\x80\x91\x03\x90\xFD[PV[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\x07\x14\"\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x89\x91\x90a\x0F\xF2V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xA0W__\xFD[PZ\xF1\x15\x80\x15a\x04\xB2W=__>=_\xFD[PPPP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\x8A\x83\xCF\x9A\xFB\t\xA9\x811OO\xB3S\xB9[\x004Q\xDA\x17\n\x99\xF4\x8D\x8D\xB6GK\x06\xD7\x9F;\x84``\x01Q\x85`\x80\x01Q`@Qa\x05\x15\x92\x91\x90a\x10\xB6V[`@Q\x80\x91\x03\x90\xA4PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x05J\x82a\x05!V[\x90P\x91\x90PV[a\x05Z\x81a\x05@V[\x82RPPV[_` \x82\x01\x90Pa\x05s_\x83\x01\x84a\x05QV[\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[a\x05\x93\x81a\x05@V[\x81\x14a\x05\x9DW__\xFD[PV[_\x815\x90Pa\x05\xAE\x81a\x05\x8AV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x05\xCAWa\x05\xC9a\x05\x82V[[_a\x05\xD7\x85\x82\x86\x01a\x05\xA0V[\x92PP` a\x05\xE8\x85\x82\x86\x01a\x05\xA0V[\x91PP\x92P\x92\x90PV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x06<\x82a\x05\xF6V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06[Wa\x06Za\x06\x06V[[\x80`@RPPPV[_a\x06ma\x05yV[\x90Pa\x06y\x82\x82a\x063V[\x91\x90PV[__\xFD[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x06\x9A\x81a\x06\x82V[\x81\x14a\x06\xA4W__\xFD[PV[_\x815\x90Pa\x06\xB5\x81a\x06\x91V[\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\xD9Wa\x06\xD8a\x06\x06V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_a\x06\xF8\x82a\x05@V[\x90P\x91\x90PV[a\x07\x08\x81a\x06\xEEV[\x81\x14a\x07\x12W__\xFD[PV[_\x815\x90Pa\x07#\x81a\x06\xFFV[\x92\x91PPV[_a\x07;a\x076\x84a\x06\xBFV[a\x06dV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x07^Wa\x07]a\x06\xEAV[[\x83[\x81\x81\x10\x15a\x07\x87W\x80a\x07s\x88\x82a\x07\x15V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x07`V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x07\xA5Wa\x07\xA4a\x06\xBBV[[\x815a\x07\xB5\x84\x82` \x86\x01a\x07)V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xD8Wa\x07\xD7a\x06\x06V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x07\xFB\x81a\x07\xE9V[\x81\x14a\x08\x05W__\xFD[PV[_\x815\x90Pa\x08\x16\x81a\x07\xF2V[\x92\x91PPV[_a\x08.a\x08)\x84a\x07\xBEV[a\x06dV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x08QWa\x08Pa\x06\xEAV[[\x83[\x81\x81\x10\x15a\x08zW\x80a\x08f\x88\x82a\x08\x08V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x08SV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x08\x98Wa\x08\x97a\x06\xBBV[[\x815a\x08\xA8\x84\x82` \x86\x01a\x08\x1CV[\x91PP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xCFWa\x08\xCEa\x06\x06V[[a\x08\xD8\x82a\x05\xF6V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\t\x05a\t\0\x84a\x08\xB5V[a\x06dV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\t!Wa\t a\x08\xB1V[[a\t,\x84\x82\x85a\x08\xE5V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\tHWa\tGa\x06\xBBV[[\x815a\tX\x84\x82` \x86\x01a\x08\xF3V[\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a\tvWa\tua\x05\xF2V[[a\t\x80`\xA0a\x06dV[\x90P_a\t\x8F\x84\x82\x85\x01a\x05\xA0V[_\x83\x01RP` a\t\xA2\x84\x82\x85\x01a\x06\xA7V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xC6Wa\t\xC5a\x06~V[[a\t\xD2\x84\x82\x85\x01a\x07\x91V[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xF6Wa\t\xF5a\x06~V[[a\n\x02\x84\x82\x85\x01a\x08\x84V[``\x83\x01RP`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n&Wa\n%a\x06~V[[a\n2\x84\x82\x85\x01a\t4V[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\nSWa\nRa\x05\x82V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\npWa\noa\x05\x86V[[a\n|\x84\x82\x85\x01a\taV[\x91PP\x92\x91PPV[a\n\x8E\x81a\x07\xE9V[\x82RPPV[_` \x82\x01\x90Pa\n\xA7_\x83\x01\x84a\n\x85V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is alrea_\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x0B\x17`.\x83a\n\xADV[\x91Pa\x0B\"\x82a\n\xBDV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0BD\x81a\x0B\x0BV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x0B\x83a\x0B~a\x0By\x84a\x0BKV[a\x0B`V[a\x0BTV[\x90P\x91\x90PV[a\x0B\x93\x81a\x0BiV[\x82RPPV[_` \x82\x01\x90Pa\x0B\xAC_\x83\x01\x84a\x0B\x8AV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0B\xE9\x82a\x07\xE9V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0C\x1BWa\x0C\x1Aa\x0B\xB2V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FInitializable: contract is not i_\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x0C\x80`+\x83a\n\xADV[\x91Pa\x0C\x8B\x82a\x0C&V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0C\xAD\x81a\x0CtV[\x90P\x91\x90PV[\x7FSlasherBase._checkSlasher: calle_\x82\x01R\x7Fr is not the slasher\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\r\x0E`4\x83a\n\xADV[\x91Pa\r\x19\x82a\x0C\xB4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\r;\x81a\r\x02V[\x90P\x91\x90PV[a\rK\x81a\x05@V[\x82RPPV[a\rZ\x81a\x06\x82V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\r\xA3a\r\x9Ea\r\x99\x84a\x05!V[a\x0B`V[a\x05!V[\x90P\x91\x90PV[_a\r\xB4\x82a\r\x89V[\x90P\x91\x90PV[_a\r\xC5\x82a\r\xAAV[\x90P\x91\x90PV[a\r\xD5\x81a\r\xBBV[\x82RPPV[_a\r\xE6\x83\x83a\r\xCCV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0E\x08\x82a\r`V[a\x0E\x12\x81\x85a\rjV[\x93Pa\x0E\x1D\x83a\rzV[\x80_[\x83\x81\x10\x15a\x0EMW\x81Qa\x0E4\x88\x82a\r\xDBV[\x97Pa\x0E?\x83a\r\xF2V[\x92PP`\x01\x81\x01\x90Pa\x0E V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0E\x8C\x81a\x07\xE9V[\x82RPPV[_a\x0E\x9D\x83\x83a\x0E\x83V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0E\xBF\x82a\x0EZV[a\x0E\xC9\x81\x85a\x0EdV[\x93Pa\x0E\xD4\x83a\x0EtV[\x80_[\x83\x81\x10\x15a\x0F\x04W\x81Qa\x0E\xEB\x88\x82a\x0E\x92V[\x97Pa\x0E\xF6\x83a\x0E\xA9V[\x92PP`\x01\x81\x01\x90Pa\x0E\xD7V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0FC\x82a\x0F\x11V[a\x0FM\x81\x85a\x0F\x1BV[\x93Pa\x0F]\x81\x85` \x86\x01a\x0F+V[a\x0Ff\x81a\x05\xF6V[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa\x0F\x86_\x86\x01\x82a\rBV[P` \x83\x01Qa\x0F\x99` \x86\x01\x82a\rQV[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x0F\xB1\x82\x82a\r\xFEV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x0F\xCB\x82\x82a\x0E\xB5V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x0F\xE5\x82\x82a\x0F9V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\n\x81\x84a\x0FqV[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x10,\x82a\x0EZV[a\x106\x81\x85a\x10\x12V[\x93Pa\x10A\x83a\x0EtV[\x80_[\x83\x81\x10\x15a\x10qW\x81Qa\x10X\x88\x82a\x0E\x92V[\x97Pa\x10c\x83a\x0E\xA9V[\x92PP`\x01\x81\x01\x90Pa\x10DV[P\x85\x93PPPP\x92\x91PPV[_a\x10\x88\x82a\x0F\x11V[a\x10\x92\x81\x85a\n\xADV[\x93Pa\x10\xA2\x81\x85` \x86\x01a\x0F+V[a\x10\xAB\x81a\x05\xF6V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\xCE\x81\x85a\x10\"V[\x90P\x81\x81\x03` \x83\x01Ra\x10\xE2\x81\x84a\x10~V[\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xC1&\x182\x9Av9\xD1${-\x1A\xAB\xB2fa\x16\x85^\xEB\xF1\x8B\xB2?j\x8BH0V\xF9\x8D{dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610055575f3560e01c80633998fdd314610059578063485cc955146100775780636a669b41146100935780636a84a985146100af578063b1344271146100cd575b5f5ffd5b6100616100eb565b60405161006e9190610560565b60405180910390f35b610091600480360381019061008c91906105b4565b610110565b005b6100ad60048036038101906100a89190610a3e565b61024e565b005b6100b761027e565b6040516100c49190610a94565b60405180910390f35b6100d5610284565b6040516100e29190610560565b60405180910390f35b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f60019054906101000a900460ff16159050808015610140575060015f5f9054906101000a900460ff1660ff16105b8061016d575061014f306102a9565b15801561016c575060015f5f9054906101000a900460ff1660ff16145b5b6101ac576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016101a390610b2d565b60405180910390fd5b60015f5f6101000a81548160ff021916908360ff16021790555080156101e75760015f60016101000a81548160ff0219169083151502179055505b6101f183836102cb565b8015610249575f5f60016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860016040516102409190610b99565b60405180910390a15b505050565b6102573361039d565b5f60025f81548092919061026a90610bdf565b91905055905061027a818361042f565b5050565b60025481565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f60019054906101000a900460ff16610319576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161031090610c96565b60405180910390fd5b815f60026101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508060015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff161461042c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161042390610d24565b60405180910390fd5b50565b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16633d071422826040518263ffffffff1660e01b81526004016104899190610ff2565b5f604051808303815f87803b1580156104a0575f5ffd5b505af11580156104b2573d5f5f3e3d5ffd5b50505050806020015163ffffffff16815f015173ffffffffffffffffffffffffffffffffffffffff16837f8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b846060015185608001516040516105159291906110b6565b60405180910390a45050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61054a82610521565b9050919050565b61055a81610540565b82525050565b5f6020820190506105735f830184610551565b92915050565b5f604051905090565b5f5ffd5b5f5ffd5b61059381610540565b811461059d575f5ffd5b50565b5f813590506105ae8161058a565b92915050565b5f5f604083850312156105ca576105c9610582565b5b5f6105d7858286016105a0565b92505060206105e8858286016105a0565b9150509250929050565b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61063c826105f6565b810181811067ffffffffffffffff8211171561065b5761065a610606565b5b80604052505050565b5f61066d610579565b90506106798282610633565b919050565b5f5ffd5b5f63ffffffff82169050919050565b61069a81610682565b81146106a4575f5ffd5b50565b5f813590506106b581610691565b92915050565b5f5ffd5b5f67ffffffffffffffff8211156106d9576106d8610606565b5b602082029050602081019050919050565b5f5ffd5b5f6106f882610540565b9050919050565b610708816106ee565b8114610712575f5ffd5b50565b5f81359050610723816106ff565b92915050565b5f61073b610736846106bf565b610664565b9050808382526020820190506020840283018581111561075e5761075d6106ea565b5b835b8181101561078757806107738882610715565b845260208401935050602081019050610760565b5050509392505050565b5f82601f8301126107a5576107a46106bb565b5b81356107b5848260208601610729565b91505092915050565b5f67ffffffffffffffff8211156107d8576107d7610606565b5b602082029050602081019050919050565b5f819050919050565b6107fb816107e9565b8114610805575f5ffd5b50565b5f81359050610816816107f2565b92915050565b5f61082e610829846107be565b610664565b90508083825260208201905060208402830185811115610851576108506106ea565b5b835b8181101561087a57806108668882610808565b845260208401935050602081019050610853565b5050509392505050565b5f82601f830112610898576108976106bb565b5b81356108a884826020860161081c565b91505092915050565b5f5ffd5b5f67ffffffffffffffff8211156108cf576108ce610606565b5b6108d8826105f6565b9050602081019050919050565b828183375f83830152505050565b5f610905610900846108b5565b610664565b905082815260208101848484011115610921576109206108b1565b5b61092c8482856108e5565b509392505050565b5f82601f830112610948576109476106bb565b5b81356109588482602086016108f3565b91505092915050565b5f60a08284031215610976576109756105f2565b5b61098060a0610664565b90505f61098f848285016105a0565b5f8301525060206109a2848285016106a7565b602083015250604082013567ffffffffffffffff8111156109c6576109c561067e565b5b6109d284828501610791565b604083015250606082013567ffffffffffffffff8111156109f6576109f561067e565b5b610a0284828501610884565b606083015250608082013567ffffffffffffffff811115610a2657610a2561067e565b5b610a3284828501610934565b60808301525092915050565b5f60208284031215610a5357610a52610582565b5b5f82013567ffffffffffffffff811115610a7057610a6f610586565b5b610a7c84828501610961565b91505092915050565b610a8e816107e9565b82525050565b5f602082019050610aa75f830184610a85565b92915050565b5f82825260208201905092915050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c7265615f8201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b5f610b17602e83610aad565b9150610b2282610abd565b604082019050919050565b5f6020820190508181035f830152610b4481610b0b565b9050919050565b5f819050919050565b5f60ff82169050919050565b5f819050919050565b5f610b83610b7e610b7984610b4b565b610b60565b610b54565b9050919050565b610b9381610b69565b82525050565b5f602082019050610bac5f830184610b8a565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610be9826107e9565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610c1b57610c1a610bb2565b5b600182019050919050565b7f496e697469616c697a61626c653a20636f6e7472616374206973206e6f7420695f8201527f6e697469616c697a696e67000000000000000000000000000000000000000000602082015250565b5f610c80602b83610aad565b9150610c8b82610c26565b604082019050919050565b5f6020820190508181035f830152610cad81610c74565b9050919050565b7f536c6173686572426173652e5f636865636b536c61736865723a2063616c6c655f8201527f72206973206e6f742074686520736c6173686572000000000000000000000000602082015250565b5f610d0e603483610aad565b9150610d1982610cb4565b604082019050919050565b5f6020820190508181035f830152610d3b81610d02565b9050919050565b610d4b81610540565b82525050565b610d5a81610682565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f610da3610d9e610d9984610521565b610b60565b610521565b9050919050565b5f610db482610d89565b9050919050565b5f610dc582610daa565b9050919050565b610dd581610dbb565b82525050565b5f610de68383610dcc565b60208301905092915050565b5f602082019050919050565b5f610e0882610d60565b610e128185610d6a565b9350610e1d83610d7a565b805f5b83811015610e4d578151610e348882610ddb565b9750610e3f83610df2565b925050600181019050610e20565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b610e8c816107e9565b82525050565b5f610e9d8383610e83565b60208301905092915050565b5f602082019050919050565b5f610ebf82610e5a565b610ec98185610e64565b9350610ed483610e74565b805f5b83811015610f04578151610eeb8882610e92565b9750610ef683610ea9565b925050600181019050610ed7565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f610f4382610f11565b610f4d8185610f1b565b9350610f5d818560208601610f2b565b610f66816105f6565b840191505092915050565b5f60a083015f830151610f865f860182610d42565b506020830151610f996020860182610d51565b5060408301518482036040860152610fb18282610dfe565b91505060608301518482036060860152610fcb8282610eb5565b91505060808301518482036080860152610fe58282610f39565b9150508091505092915050565b5f6020820190508181035f83015261100a8184610f71565b905092915050565b5f82825260208201905092915050565b5f61102c82610e5a565b6110368185611012565b935061104183610e74565b805f5b838110156110715781516110588882610e92565b975061106383610ea9565b925050600181019050611044565b5085935050505092915050565b5f61108882610f11565b6110928185610aad565b93506110a2818560208601610f2b565b6110ab816105f6565b840191505092915050565b5f6040820190508181035f8301526110ce8185611022565b905081810360208301526110e2818461107e565b9050939250505056fea2646970667358221220c12618329a7639d1247b2d1aabb2666116855eebf18bb23f6a8b483056f98d7b64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0UW_5`\xE0\x1C\x80c9\x98\xFD\xD3\x14a\0YW\x80cH\\\xC9U\x14a\0wW\x80cjf\x9BA\x14a\0\x93W\x80cj\x84\xA9\x85\x14a\0\xAFW\x80c\xB14Bq\x14a\0\xCDW[__\xFD[a\0aa\0\xEBV[`@Qa\0n\x91\x90a\x05`V[`@Q\x80\x91\x03\x90\xF3[a\0\x91`\x04\x806\x03\x81\x01\x90a\0\x8C\x91\x90a\x05\xB4V[a\x01\x10V[\0[a\0\xAD`\x04\x806\x03\x81\x01\x90a\0\xA8\x91\x90a\n>V[a\x02NV[\0[a\0\xB7a\x02~V[`@Qa\0\xC4\x91\x90a\n\x94V[`@Q\x80\x91\x03\x90\xF3[a\0\xD5a\x02\x84V[`@Qa\0\xE2\x91\x90a\x05`V[`@Q\x80\x91\x03\x90\xF3[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x90P\x80\x80\x15a\x01@WP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[\x80a\x01mWPa\x01O0a\x02\xA9V[\x15\x80\x15a\x01lWP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x14[[a\x01\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x01\xA3\x90a\x0B-V[`@Q\x80\x91\x03\x90\xFD[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x15a\x01\xE7W`\x01_`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[a\x01\xF1\x83\x83a\x02\xCBV[\x80\x15a\x02IW__`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\x01`@Qa\x02@\x91\x90a\x0B\x99V[`@Q\x80\x91\x03\x90\xA1[PPPV[a\x02W3a\x03\x9DV[_`\x02_\x81T\x80\x92\x91\x90a\x02j\x90a\x0B\xDFV[\x91\x90PU\x90Pa\x02z\x81\x83a\x04/V[PPV[`\x02T\x81V[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x03\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x03\x10\x90a\x0C\x96V[`@Q\x80\x91\x03\x90\xFD[\x81_`\x02a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x04,W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04#\x90a\r$V[`@Q\x80\x91\x03\x90\xFD[PV[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\x07\x14\"\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x89\x91\x90a\x0F\xF2V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xA0W__\xFD[PZ\xF1\x15\x80\x15a\x04\xB2W=__>=_\xFD[PPPP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\x8A\x83\xCF\x9A\xFB\t\xA9\x811OO\xB3S\xB9[\x004Q\xDA\x17\n\x99\xF4\x8D\x8D\xB6GK\x06\xD7\x9F;\x84``\x01Q\x85`\x80\x01Q`@Qa\x05\x15\x92\x91\x90a\x10\xB6V[`@Q\x80\x91\x03\x90\xA4PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x05J\x82a\x05!V[\x90P\x91\x90PV[a\x05Z\x81a\x05@V[\x82RPPV[_` \x82\x01\x90Pa\x05s_\x83\x01\x84a\x05QV[\x92\x91PPV[_`@Q\x90P\x90V[__\xFD[__\xFD[a\x05\x93\x81a\x05@V[\x81\x14a\x05\x9DW__\xFD[PV[_\x815\x90Pa\x05\xAE\x81a\x05\x8AV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x05\xCAWa\x05\xC9a\x05\x82V[[_a\x05\xD7\x85\x82\x86\x01a\x05\xA0V[\x92PP` a\x05\xE8\x85\x82\x86\x01a\x05\xA0V[\x91PP\x92P\x92\x90PV[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x06<\x82a\x05\xF6V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06[Wa\x06Za\x06\x06V[[\x80`@RPPPV[_a\x06ma\x05yV[\x90Pa\x06y\x82\x82a\x063V[\x91\x90PV[__\xFD[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x06\x9A\x81a\x06\x82V[\x81\x14a\x06\xA4W__\xFD[PV[_\x815\x90Pa\x06\xB5\x81a\x06\x91V[\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\xD9Wa\x06\xD8a\x06\x06V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_a\x06\xF8\x82a\x05@V[\x90P\x91\x90PV[a\x07\x08\x81a\x06\xEEV[\x81\x14a\x07\x12W__\xFD[PV[_\x815\x90Pa\x07#\x81a\x06\xFFV[\x92\x91PPV[_a\x07;a\x076\x84a\x06\xBFV[a\x06dV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x07^Wa\x07]a\x06\xEAV[[\x83[\x81\x81\x10\x15a\x07\x87W\x80a\x07s\x88\x82a\x07\x15V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x07`V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x07\xA5Wa\x07\xA4a\x06\xBBV[[\x815a\x07\xB5\x84\x82` \x86\x01a\x07)V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\xD8Wa\x07\xD7a\x06\x06V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x07\xFB\x81a\x07\xE9V[\x81\x14a\x08\x05W__\xFD[PV[_\x815\x90Pa\x08\x16\x81a\x07\xF2V[\x92\x91PPV[_a\x08.a\x08)\x84a\x07\xBEV[a\x06dV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x08QWa\x08Pa\x06\xEAV[[\x83[\x81\x81\x10\x15a\x08zW\x80a\x08f\x88\x82a\x08\x08V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x08SV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x08\x98Wa\x08\x97a\x06\xBBV[[\x815a\x08\xA8\x84\x82` \x86\x01a\x08\x1CV[\x91PP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xCFWa\x08\xCEa\x06\x06V[[a\x08\xD8\x82a\x05\xF6V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\t\x05a\t\0\x84a\x08\xB5V[a\x06dV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\t!Wa\t a\x08\xB1V[[a\t,\x84\x82\x85a\x08\xE5V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\tHWa\tGa\x06\xBBV[[\x815a\tX\x84\x82` \x86\x01a\x08\xF3V[\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a\tvWa\tua\x05\xF2V[[a\t\x80`\xA0a\x06dV[\x90P_a\t\x8F\x84\x82\x85\x01a\x05\xA0V[_\x83\x01RP` a\t\xA2\x84\x82\x85\x01a\x06\xA7V[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xC6Wa\t\xC5a\x06~V[[a\t\xD2\x84\x82\x85\x01a\x07\x91V[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xF6Wa\t\xF5a\x06~V[[a\n\x02\x84\x82\x85\x01a\x08\x84V[``\x83\x01RP`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n&Wa\n%a\x06~V[[a\n2\x84\x82\x85\x01a\t4V[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\nSWa\nRa\x05\x82V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\npWa\noa\x05\x86V[[a\n|\x84\x82\x85\x01a\taV[\x91PP\x92\x91PPV[a\n\x8E\x81a\x07\xE9V[\x82RPPV[_` \x82\x01\x90Pa\n\xA7_\x83\x01\x84a\n\x85V[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FInitializable: contract is alrea_\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x0B\x17`.\x83a\n\xADV[\x91Pa\x0B\"\x82a\n\xBDV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0BD\x81a\x0B\x0BV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x0B\x83a\x0B~a\x0By\x84a\x0BKV[a\x0B`V[a\x0BTV[\x90P\x91\x90PV[a\x0B\x93\x81a\x0BiV[\x82RPPV[_` \x82\x01\x90Pa\x0B\xAC_\x83\x01\x84a\x0B\x8AV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x0B\xE9\x82a\x07\xE9V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x0C\x1BWa\x0C\x1Aa\x0B\xB2V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FInitializable: contract is not i_\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x0C\x80`+\x83a\n\xADV[\x91Pa\x0C\x8B\x82a\x0C&V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x0C\xAD\x81a\x0CtV[\x90P\x91\x90PV[\x7FSlasherBase._checkSlasher: calle_\x82\x01R\x7Fr is not the slasher\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\r\x0E`4\x83a\n\xADV[\x91Pa\r\x19\x82a\x0C\xB4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\r;\x81a\r\x02V[\x90P\x91\x90PV[a\rK\x81a\x05@V[\x82RPPV[a\rZ\x81a\x06\x82V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a\r\xA3a\r\x9Ea\r\x99\x84a\x05!V[a\x0B`V[a\x05!V[\x90P\x91\x90PV[_a\r\xB4\x82a\r\x89V[\x90P\x91\x90PV[_a\r\xC5\x82a\r\xAAV[\x90P\x91\x90PV[a\r\xD5\x81a\r\xBBV[\x82RPPV[_a\r\xE6\x83\x83a\r\xCCV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0E\x08\x82a\r`V[a\x0E\x12\x81\x85a\rjV[\x93Pa\x0E\x1D\x83a\rzV[\x80_[\x83\x81\x10\x15a\x0EMW\x81Qa\x0E4\x88\x82a\r\xDBV[\x97Pa\x0E?\x83a\r\xF2V[\x92PP`\x01\x81\x01\x90Pa\x0E V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0E\x8C\x81a\x07\xE9V[\x82RPPV[_a\x0E\x9D\x83\x83a\x0E\x83V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x0E\xBF\x82a\x0EZV[a\x0E\xC9\x81\x85a\x0EdV[\x93Pa\x0E\xD4\x83a\x0EtV[\x80_[\x83\x81\x10\x15a\x0F\x04W\x81Qa\x0E\xEB\x88\x82a\x0E\x92V[\x97Pa\x0E\xF6\x83a\x0E\xA9V[\x92PP`\x01\x81\x01\x90Pa\x0E\xD7V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x0FC\x82a\x0F\x11V[a\x0FM\x81\x85a\x0F\x1BV[\x93Pa\x0F]\x81\x85` \x86\x01a\x0F+V[a\x0Ff\x81a\x05\xF6V[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa\x0F\x86_\x86\x01\x82a\rBV[P` \x83\x01Qa\x0F\x99` \x86\x01\x82a\rQV[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x0F\xB1\x82\x82a\r\xFEV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x0F\xCB\x82\x82a\x0E\xB5V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x0F\xE5\x82\x82a\x0F9V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\n\x81\x84a\x0FqV[\x90P\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\x10,\x82a\x0EZV[a\x106\x81\x85a\x10\x12V[\x93Pa\x10A\x83a\x0EtV[\x80_[\x83\x81\x10\x15a\x10qW\x81Qa\x10X\x88\x82a\x0E\x92V[\x97Pa\x10c\x83a\x0E\xA9V[\x92PP`\x01\x81\x01\x90Pa\x10DV[P\x85\x93PPPP\x92\x91PPV[_a\x10\x88\x82a\x0F\x11V[a\x10\x92\x81\x85a\n\xADV[\x93Pa\x10\xA2\x81\x85` \x86\x01a\x0F+V[a\x10\xAB\x81a\x05\xF6V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x10\xCE\x81\x85a\x10\"V[\x90P\x81\x81\x03` \x83\x01Ra\x10\xE2\x81\x84a\x10~V[\x90P\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xC1&\x182\x9Av9\xD1${-\x1A\xAB\xB2fa\x16\x85^\xEB\xF1\x8B\xB2?j\x8BH0V\xF9\x8D{dsolcC\0\x08\x1B\x003",
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
    /**Event with signature `OperatorSlashed(uint256,address,uint32,uint256[],string)` and selector `0x8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b`.
```solidity
event OperatorSlashed(uint256 indexed slashingRequestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSlashed {
        #[allow(missing_docs)]
        pub slashingRequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub wadsToSlash: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for OperatorSlashed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "OperatorSlashed(uint256,address,uint32,uint256[],string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                138u8,
                131u8,
                207u8,
                154u8,
                251u8,
                9u8,
                169u8,
                129u8,
                49u8,
                79u8,
                79u8,
                179u8,
                83u8,
                185u8,
                91u8,
                0u8,
                52u8,
                81u8,
                218u8,
                23u8,
                10u8,
                153u8,
                244u8,
                141u8,
                141u8,
                182u8,
                71u8,
                75u8,
                6u8,
                215u8,
                159u8,
                59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    slashingRequestId: topics.1,
                    operator: topics.2,
                    operatorSetId: topics.3,
                    wadsToSlash: data.0,
                    description: data.1,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.slashingRequestId.clone(),
                    self.operator.clone(),
                    self.operatorSetId.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.slashingRequestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorSetId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSlashed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSlashed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSlashed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashingRequestCancelled(uint256)` and selector `0xb612b6d36795da938211ef5caee2c1d887a24430f76918733089eee28b7f54ac`.
```solidity
event SlashingRequestCancelled(uint256 indexed requestId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingRequestCancelled {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for SlashingRequestCancelled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "SlashingRequestCancelled(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                182u8,
                18u8,
                182u8,
                211u8,
                103u8,
                149u8,
                218u8,
                147u8,
                130u8,
                17u8,
                239u8,
                92u8,
                174u8,
                226u8,
                193u8,
                216u8,
                135u8,
                162u8,
                68u8,
                48u8,
                247u8,
                105u8,
                24u8,
                115u8,
                48u8,
                137u8,
                238u8,
                226u8,
                139u8,
                127u8,
                84u8,
                172u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { requestId: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.requestId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SlashingRequestCancelled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingRequestCancelled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &SlashingRequestCancelled,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SlashingRequested(uint256,address,uint32,uint256[],string)` and selector `0xadd285945f652e749df3dab9a584be524ec7fbd2a2cad39851278950f9b73227`.
```solidity
event SlashingRequested(uint256 indexed requestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SlashingRequested {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub wadsToSlash: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for SlashingRequested {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "SlashingRequested(uint256,address,uint32,uint256[],string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                173u8,
                210u8,
                133u8,
                148u8,
                95u8,
                101u8,
                46u8,
                116u8,
                157u8,
                243u8,
                218u8,
                185u8,
                165u8,
                132u8,
                190u8,
                82u8,
                78u8,
                199u8,
                251u8,
                210u8,
                162u8,
                202u8,
                211u8,
                152u8,
                81u8,
                39u8,
                137u8,
                80u8,
                249u8,
                183u8,
                50u8,
                39u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    requestId: topics.1,
                    operator: topics.2,
                    operatorSetId: topics.3,
                    wadsToSlash: data.0,
                    description: data.1,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.requestId.clone(),
                    self.operator.clone(),
                    self.operatorSetId.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.requestId);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorSetId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SlashingRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SlashingRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SlashingRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `fulfillSlashingRequest((address,uint32,address[],uint256[],string))` and selector `0x6a669b41`.
```solidity
function fulfillSlashingRequest(IAllocationManagerTypes.SlashingParams memory _slashingParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillSlashingRequestCall {
        pub _slashingParams: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`fulfillSlashingRequest((address,uint32,address[],uint256[],string))`](fulfillSlashingRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillSlashingRequestReturn {}
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
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::SlashingParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<fulfillSlashingRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillSlashingRequestCall) -> Self {
                    (value._slashingParams,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillSlashingRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _slashingParams: tuple.0 }
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
            impl ::core::convert::From<fulfillSlashingRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillSlashingRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillSlashingRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for fulfillSlashingRequestCall {
            type Parameters<'a> = (IAllocationManagerTypes::SlashingParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fulfillSlashingRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fulfillSlashingRequest((address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [106u8, 102u8, 155u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::SlashingParams as alloy_sol_types::SolType>::tokenize(
                        &self._slashingParams,
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
    /**Function with signature `initialize(address,address)` and selector `0x485cc955`.
```solidity
function initialize(address _serviceManager, address _slasher) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _slasher: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceManager, value._slasher)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        _slasher: tuple.1,
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
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address)";
            const SELECTOR: [u8; 4] = [72u8, 92u8, 201u8, 85u8];
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
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slasher,
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
    /**Function with signature `nextRequestId()` and selector `0x6a84a985`.
```solidity
function nextRequestId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextRequestIdCall {}
    ///Container type for the return parameters of the [`nextRequestId()`](nextRequestIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextRequestIdReturn {
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
            impl ::core::convert::From<nextRequestIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextRequestIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextRequestIdCall {
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
            impl ::core::convert::From<nextRequestIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextRequestIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextRequestIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextRequestIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextRequestIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextRequestId()";
            const SELECTOR: [u8; 4] = [106u8, 132u8, 169u8, 133u8];
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
    /**Function with signature `serviceManager()` and selector `0x3998fdd3`.
```solidity
function serviceManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerCall {}
    ///Container type for the return parameters of the [`serviceManager()`](serviceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerReturn {
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
            impl ::core::convert::From<serviceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerCall {
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
            impl ::core::convert::From<serviceManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceManager()";
            const SELECTOR: [u8; 4] = [57u8, 152u8, 253u8, 211u8];
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
    /**Function with signature `slasher()` and selector `0xb1344271`.
```solidity
function slasher() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherCall {}
    ///Container type for the return parameters of the [`slasher()`](slasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherReturn {
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
            impl ::core::convert::From<slasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: slasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherCall {
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
            impl ::core::convert::From<slasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasher()";
            const SELECTOR: [u8; 4] = [177u8, 52u8, 66u8, 113u8];
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
    ///Container for all the [`InstantSlasher`](self) function calls.
    pub enum InstantSlasherCalls {
        fulfillSlashingRequest(fulfillSlashingRequestCall),
        initialize(initializeCall),
        nextRequestId(nextRequestIdCall),
        serviceManager(serviceManagerCall),
        slasher(slasherCall),
    }
    #[automatically_derived]
    impl InstantSlasherCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [57u8, 152u8, 253u8, 211u8],
            [72u8, 92u8, 201u8, 85u8],
            [106u8, 102u8, 155u8, 65u8],
            [106u8, 132u8, 169u8, 133u8],
            [177u8, 52u8, 66u8, 113u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for InstantSlasherCalls {
        const NAME: &'static str = "InstantSlasherCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 5usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::fulfillSlashingRequest(_) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextRequestId(_) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<InstantSlasherCalls>] = &[
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::initialize)
                    }
                    initialize
                },
                {
                    fn fulfillSlashingRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::fulfillSlashingRequest)
                    }
                    fulfillSlashingRequest
                },
                {
                    fn nextRequestId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <nextRequestIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::nextRequestId)
                    }
                    nextRequestId
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<InstantSlasherCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(InstantSlasherCalls::slasher)
                    }
                    slasher
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
                Self::fulfillSlashingRequest(inner) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nextRequestId(inner) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::fulfillSlashingRequest(inner) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::nextRequestId(inner) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`InstantSlasher`](self) events.
    pub enum InstantSlasherEvents {
        Initialized(Initialized),
        OperatorSlashed(OperatorSlashed),
        SlashingRequestCancelled(SlashingRequestCancelled),
        SlashingRequested(SlashingRequested),
    }
    #[automatically_derived]
    impl InstantSlasherEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
                138u8,
                131u8,
                207u8,
                154u8,
                251u8,
                9u8,
                169u8,
                129u8,
                49u8,
                79u8,
                79u8,
                179u8,
                83u8,
                185u8,
                91u8,
                0u8,
                52u8,
                81u8,
                218u8,
                23u8,
                10u8,
                153u8,
                244u8,
                141u8,
                141u8,
                182u8,
                71u8,
                75u8,
                6u8,
                215u8,
                159u8,
                59u8,
            ],
            [
                173u8,
                210u8,
                133u8,
                148u8,
                95u8,
                101u8,
                46u8,
                116u8,
                157u8,
                243u8,
                218u8,
                185u8,
                165u8,
                132u8,
                190u8,
                82u8,
                78u8,
                199u8,
                251u8,
                210u8,
                162u8,
                202u8,
                211u8,
                152u8,
                81u8,
                39u8,
                137u8,
                80u8,
                249u8,
                183u8,
                50u8,
                39u8,
            ],
            [
                182u8,
                18u8,
                182u8,
                211u8,
                103u8,
                149u8,
                218u8,
                147u8,
                130u8,
                17u8,
                239u8,
                92u8,
                174u8,
                226u8,
                193u8,
                216u8,
                135u8,
                162u8,
                68u8,
                48u8,
                247u8,
                105u8,
                24u8,
                115u8,
                48u8,
                137u8,
                238u8,
                226u8,
                139u8,
                127u8,
                84u8,
                172u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for InstantSlasherEvents {
        const NAME: &'static str = "InstantSlasherEvents";
        const COUNT: usize = 4usize;
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
                Some(<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSlashed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSlashed)
                }
                Some(
                    <SlashingRequestCancelled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingRequestCancelled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingRequestCancelled)
                }
                Some(
                    <SlashingRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <SlashingRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SlashingRequested)
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
    impl alloy_sol_types::private::IntoLogData for InstantSlasherEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingRequestCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SlashingRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingRequestCancelled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SlashingRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`InstantSlasher`](self) contract instance.

See the [wrapper's documentation](`InstantSlasherInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> InstantSlasherInstance<T, P, N> {
        InstantSlasherInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<InstantSlasherInstance<T, P, N>>,
    > {
        InstantSlasherInstance::<T, P, N>::deploy(provider)
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
        InstantSlasherInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`InstantSlasher`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`InstantSlasher`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct InstantSlasherInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for InstantSlasherInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("InstantSlasherInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > InstantSlasherInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`InstantSlasher`](self) contract instance.

See the [wrapper's documentation](`InstantSlasherInstance`) for more details.*/
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
        ) -> alloy_contract::Result<InstantSlasherInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> InstantSlasherInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> InstantSlasherInstance<T, P, N> {
            InstantSlasherInstance {
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
    > InstantSlasherInstance<T, P, N> {
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
        ///Creates a new call builder for the [`fulfillSlashingRequest`] function.
        pub fn fulfillSlashingRequest(
            &self,
            _slashingParams: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, fulfillSlashingRequestCall, N> {
            self.call_builder(
                &fulfillSlashingRequestCall {
                    _slashingParams,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _serviceManager,
                    _slasher,
                },
            )
        }
        ///Creates a new call builder for the [`nextRequestId`] function.
        pub fn nextRequestId(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, nextRequestIdCall, N> {
            self.call_builder(&nextRequestIdCall {})
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
        }
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > InstantSlasherInstance<T, P, N> {
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
        ///Creates a new event filter for the [`OperatorSlashed`] event.
        pub fn OperatorSlashed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSlashed, N> {
            self.event_filter::<OperatorSlashed>()
        }
        ///Creates a new event filter for the [`SlashingRequestCancelled`] event.
        pub fn SlashingRequestCancelled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingRequestCancelled, N> {
            self.event_filter::<SlashingRequestCancelled>()
        }
        ///Creates a new event filter for the [`SlashingRequested`] event.
        pub fn SlashingRequested_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SlashingRequested, N> {
            self.event_filter::<SlashingRequested>()
        }
    }
}
