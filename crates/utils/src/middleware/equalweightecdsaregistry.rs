///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzInterface { address addr; string[] artifacts; }
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
    struct FuzzInterface { address addr; string[] artifacts; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        pub addr: alloy::sol_types::private::Address,
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
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
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
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
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface EqualWeightECDSARegistry {
    struct Quorum {
        StrategyParams[] strategies;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }

    error InsufficientSignedStake();
    error InsufficientWeight();
    error InvalidLength();
    error InvalidQuorum();
    error InvalidSignature();
    error InvalidSignedWeight();
    error InvalidThreshold();
    error LengthMismatch();
    error MustUpdateAllOperators();
    error NotSorted();
    error OperatorAlreadyRegistered();
    error OperatorNotRegistered();

    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    event QuorumUpdated(Quorum _old, Quorum _new);
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
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
    function mockDelegationManager() external view returns (address);
    function mockServiceManager() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function test_FixedStakeUpdates() external;
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
    "name": "mockDelegationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockDelegationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "mockServiceManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockServiceManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setUp",
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
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
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
    "name": "test_FixedStakeUpdates",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "MinimumWeightUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "_new",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWeightUpdated",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      },
      {
        "name": "_new",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ThresholdWeightUpdated",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalWeightUpdated",
    "inputs": [
      {
        "name": "oldTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMinimumWeight",
    "inputs": [
      {
        "name": "oldMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
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
  },
  {
    "type": "error",
    "name": "InsufficientSignedStake",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignedWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustUpdateAllOperators",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod EqualWeightECDSARegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405260078054600160ff199182168117909255600b8054909116909117905534801561002d57600080fd5b50614c728061003d6000396000f3fe608060405234801561001057600080fd5b50600436106100f55760003560e01c806385226c8111610097578063b5508aa911610066578063b5508aa9146101bf578063ba414fa6146101c7578063e20c9f71146101df578063fa7626d4146101e757600080fd5b806385226c811461016457806386777e0614610179578063916a17c6146101a4578063b52d472a146101ac57600080fd5b80633e5e3c23116100d35780633e5e3c23146101375780633f7286f41461013f57806366d9a9a014610147578063707a92411461015c57600080fd5b80630a9254e4146100fa5780631ed7831c146101045780632ade388014610122575b600080fd5b6101026101f4565b005b61010c610518565b604051610119919061187f565b60405180910390f35b61012a61057a565b6040516101199190611928565b61010c6106bc565b61010c61071c565b61014f61077c565b60405161011991906119e8565b610102610862565b61016c610fcc565b6040516101199190611a9b565b601d5461018c906001600160a01b031681565b6040516001600160a01b039091168152602001610119565b61014f61109c565b601c5461018c906001600160a01b031681565b61016c611182565b6101cf611252565b6040519015158152602001610119565b61010c61137f565b6007546101cf9060ff1681565b6101fc6113df565b601c546040516001600160a01b039091169061021790611858565b6001600160a01b039091168152602001604051809103906000f080158015610243573d6000803e3d6000fd5b50602780546001600160a01b0319166001600160a01b039290921691909117905560408051600160208201818152606083018452611234936000939283929183015b60408051808201909152600080825260208201528152602001906001900390816102855750509052604080518082019091526001600160a01b038416815261271060208201528151805192935090916000906102e3576102e3611afd565b6020908102919091010152602754601d5460405163ab11899560e01b81526001600160a01b039283169263ab11899592610327929116906064908690600401611b13565b600060405180830381600087803b15801561034157600080fd5b505af1158015610355573d6000803e3d6000fd5b5050602754601e546040516358c1eb1760e01b81526001600160a01b039182166004820152911692506358c1eb179150602401600060405180830381600087803b1580156103a257600080fd5b505af11580156103b6573d6000803e3d6000fd5b5050602754601f546040516358c1eb1760e01b81526001600160a01b039182166004820152911692506358c1eb179150602401600060405180830381600087803b15801561040357600080fd5b505af1158015610417573d6000803e3d6000fd5b5050505061044260405180606001604052806060815260200160008019168152602001600081525090565b602754601e546040516305300d0960e11b81526001600160a01b0392831692630a601a1292610478929116908590600401611b9b565b600060405180830381600087803b15801561049257600080fd5b505af11580156104a6573d6000803e3d6000fd5b5050602754601f546040516305300d0960e11b81526001600160a01b039283169450630a601a1293506104e192909116908590600401611b9b565b600060405180830381600087803b1580156104fb57600080fd5b505af115801561050f573d6000803e3d6000fd5b50505050505050565b6060601480548060200260200160405190810160405280929190818152602001828054801561057057602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610552575b5050505050905090565b6060601b805480602002602001604051908101604052809291908181526020016000905b828210156106b357600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561069c57838290600052602060002001805461060f90611be6565b80601f016020809104026020016040519081016040528092919081815260200182805461063b90611be6565b80156106885780601f1061065d57610100808354040283529160200191610688565b820191906000526020600020905b81548152906001019060200180831161066b57829003601f168201915b5050505050815260200190600101906105f0565b50505050815250508152602001906001019061059e565b50505050905090565b60606016805480602002602001604051908101604052809291908181526020018280548015610570576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610552575050505050905090565b60606015805480602002602001604051908101604052809291908181526020018280548015610570576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610552575050505050905090565b60606019805480602002602001604051908101604052809291908181526020016000905b828210156106b35760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561084a57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161080c5790505b505050505081525050815260200190600101906107a0565b602754601e54604051631d92172560e11b81526001600160a01b0391821660048201526108e0929190911690633b242e4a906024015b602060405180830381865afa1580156108b5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108d99190611c21565b600161150f565b602754601f54604051631d92172560e11b81526001600160a01b03918216600482015261091a929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610990926001600160a01b03169163314f3a499160048083019260209291908290030181865afa158015610965573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109899190611c21565b600261150f565b737109709ecfa91a80626ff3989d68f67f5b1dd12d631f7b4f306109b5436001611c3a565b6040518263ffffffff1660e01b81526004016109d391815260200190565b600060405180830381600087803b1580156109ed57600080fd5b505af1158015610a01573d6000803e3d6000fd5b5050601e5460405163ca669fa760e01b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d925063ca669fa79150602401600060405180830381600087803b158015610a5e57600080fd5b505af1158015610a72573d6000803e3d6000fd5b50505050602760009054906101000a90046001600160a01b03166001600160a01b031663857dc1906040518163ffffffff1660e01b8152600401600060405180830381600087803b158015610ac657600080fd5b505af1158015610ada573d6000803e3d6000fd5b5050602754601e54604051631d92172560e11b81526001600160a01b039182166004820152610b59945091169150633b242e4a90602401602060405180830381865afa158015610b2e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b529190611c21565b600061150f565b602754601f54604051631d92172560e11b81526001600160a01b039182166004820152610b93929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610bde926001600160a01b03169163314f3a499160048083019260209291908290030181865afa1580156108b5573d6000803e3d6000fd5b737109709ecfa91a80626ff3989d68f67f5b1dd12d631f7b4f30610c03436001611c3a565b6040518263ffffffff1660e01b8152600401610c2191815260200190565b600060405180830381600087803b158015610c3b57600080fd5b505af1158015610c4f573d6000803e3d6000fd5b50505050610c7a60405180606001604052806060815260200160008019168152602001600081525090565b602754601e546040516305300d0960e11b81526001600160a01b0392831692630a601a1292610cb0929116908590600401611b9b565b600060405180830381600087803b158015610cca57600080fd5b505af1158015610cde573d6000803e3d6000fd5b5050602754601e54604051631d92172560e11b81526001600160a01b039182166004820152610d1a945091169150633b242e4a90602401610898565b602754601f54604051631d92172560e11b81526001600160a01b039182166004820152610d54929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610d9f926001600160a01b03169163314f3a499160048083019260209291908290030181865afa158015610965573d6000803e3d6000fd5b737109709ecfa91a80626ff3989d68f67f5b1dd12d631f7b4f30610dc4436001611c3a565b6040518263ffffffff1660e01b8152600401610de291815260200190565b600060405180830381600087803b158015610dfc57600080fd5b505af1158015610e10573d6000803e3d6000fd5b506000925060029150610e209050565b604051908082528060200260200182016040528015610e49578160200160208202803683370190505b50601e5481519192506001600160a01b0316908290600090610e6d57610e6d611afd565b6001600160a01b039283166020918202929092010152601f54825191169082906001908110610e9e57610e9e611afd565b6001600160a01b03928316602091820292909201015260275460405162cf2ab560e01b815291169062cf2ab590610ed990849060040161187f565b600060405180830381600087803b158015610ef357600080fd5b505af1158015610f07573d6000803e3d6000fd5b5050602754601e54604051631d92172560e11b81526001600160a01b039182166004820152610f43945091169150633b242e4a90602401610898565b602754601f54604051631d92172560e11b81526001600160a01b039182166004820152610f7d929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610fc8926001600160a01b03169163314f3a499160048083019260209291908290030181865afa158015610965573d6000803e3d6000fd5b5050565b60606018805480602002602001604051908101604052809291908181526020016000905b828210156106b357838290600052602060002001805461100f90611be6565b80601f016020809104026020016040519081016040528092919081815260200182805461103b90611be6565b80156110885780601f1061105d57610100808354040283529160200191611088565b820191906000526020600020905b81548152906001019060200180831161106b57829003601f168201915b505050505081526020019060010190610ff0565b6060601a805480602002602001604051908101604052809291908181526020016000905b828210156106b35760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561116a57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161112c5790505b505050505081525050815260200190600101906110c0565b60606017805480602002602001604051908101604052809291908181526020016000905b828210156106b35783829060005260206000200180546111c590611be6565b80601f01602080910402602001604051908101604052809291908181526020018280546111f190611be6565b801561123e5780601f106112135761010080835404028352916020019161123e565b820191906000526020600020905b81548152906001019060200180831161122157829003601f168201915b5050505050815260200190600101906111a6565b600754600090610100900460ff16156112745750600754610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561137a5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611302917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001611c60565b60408051601f198184030181529082905261131c91611c91565b6000604051808303816000865af19150503d8060008114611359576040519150601f19603f3d011682016040523d82523d6000602084013e61135e565b606091505b50915050808060200190518101906113769190611cad565b9150505b919050565b60606013805480602002602001604051908101604052809291908181526020018280548015610570576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610552575050505050905090565b611408604051806040016040528060088152602001675369676e6572203160c01b815250611636565b6020908155601e80546001600160a01b0319166001600160a01b03939093169290921790915560408051808201909152600881526729b4b3b732b9101960c11b9181019190915261145890611636565b602155601f80546001600160a01b0319166001600160a01b039290921691909117905560405161148790611865565b604051809103906000f0801580156114a3573d6000803e3d6000fd5b50601c80546001600160a01b0319166001600160a01b03929092169190911790556040516114d090611872565b604051809103906000f0801580156114ec573d6000803e3d6000fd5b50601d80546001600160a01b0319166001600160a01b0392909216919091179055565b808214610fc8577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f506040516115809060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a1610fc861174c565b6000808260405160200161164a9190611c91565b60408051808303601f190181529082905280516020909101206001625e79b760e01b03198252600482018190529150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ffa1864990602401602060405180830381865afa1580156116b5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116d99190611cd6565b6040516318caf8e360e31b8152909250737109709ecfa91a80626ff3989d68f67f5b1dd12d9063c657c718906117159085908790600401611cff565b600060405180830381600087803b15801561172f57600080fd5b505af1158015611743573d6000803e3d6000fd5b50505050915091565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156118475760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526117e69291602001611c60565b60408051601f198184030181529082905261180091611c91565b6000604051808303816000865af19150503d806000811461183d576040519150601f19603f3d011682016040523d82523d6000602084013e611842565b606091505b505050505b6007805461ff001916610100179055565b61299d80611d2c83390190565b61032b806146c983390190565b610249806149f483390190565b6020808252825182820181905260009190848201906040850190845b818110156118c05783516001600160a01b03168352928401929184019160010161189b565b50909695505050505050565b60005b838110156118e75781810151838201526020016118cf565b838111156118f6576000848401525b50505050565b600081518084526119148160208601602086016118cc565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b858110156119d857603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b818110156119c257605f198985030183526119b08486516118fc565b948e01949350918d0191600101611994565b505050978a01979450509188019160010161194f565b50919a9950505050505050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015611a8c57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015611a775783516001600160e01b0319168252928b019260019290920191908b0190611a4d565b50978a01979550505091870191600101611a10565b50919998505050505050505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015611af057603f19888603018452611ade8583516118fc565b94509285019290850190600101611ac2565b5092979650505050505050565b634e487b7160e01b600052603260045260246000fd5b6001600160a01b03848116825260208083018590526060604080850182905285519185018390528151608086018190526000949392830190859060a08801905b80831015611b8c5783518051881683528601516bffffffffffffffffffffffff1686830152928501926001929092019190840190611b53565b509a9950505050505050505050565b60018060a01b0383168152604060208201526000825160606040840152611bc560a08401826118fc565b90506020840151606084015260408401516080840152809150509392505050565b600181811c90821680611bfa57607f821691505b60208210811415611c1b57634e487b7160e01b600052602260045260246000fd5b50919050565b600060208284031215611c3357600080fd5b5051919050565b60008219821115611c5b57634e487b7160e01b600052601160045260246000fd5b500190565b6001600160e01b0319831681528151600090611c838160048501602087016118cc565b919091016004019392505050565b60008251611ca38184602087016118cc565b9190910192915050565b600060208284031215611cbf57600080fd5b81518015158114611ccf57600080fd5b9392505050565b600060208284031215611ce857600080fd5b81516001600160a01b0381168114611ccf57600080fd5b6001600160a01b0383168152604060208201819052600090611d23908301846118fc565b94935050505056fe60a06040523480156200001157600080fd5b506040516200299d3803806200299d833981016040819052620000349162000046565b6001600160a01b031660805262000078565b6000602082840312156200005957600080fd5b81516001600160a01b03811681146200007157600080fd5b9392505050565b6080516129096200009460003960006106fd01526129096000f3fe608060405234801561001057600080fd5b506004361061018d5760003560e01c80636d5be926116100de578063ab11899511610097578063e5d98f9411610071578063e5d98f9414610355578063ec7fbb3114610368578063f2fde38b14610394578063fad8b32a146103a757600080fd5b8063ab11899514610327578063b933fa741461033a578063dec5d1f61461034257600080fd5b80636d5be926146102a3578063715018a6146102d6578063857dc190146102de5780638da5cb5b146102e6578063955f2d901461030157806398ec1ac91461031457600080fd5b8063314f3a491161014b5780635140a548116101255780635140a5481461025757806358c1eb171461026a5780635ef533291461027d578063696255be1461029057600080fd5b8063314f3a49146102345780633b242e4a1461023c57806340bf2fb71461024f57600080fd5b8062cf2ab5146101925780630a601a12146101a75780630dba3394146101ba5780631626ba7e146101e05780631703a0181461020c5780631e4cd85e14610221575b600080fd5b6101a56101a0366004611e1e565b6103ba565b005b6101a56101b5366004611ed2565b6103c6565b6101cd6101c8366004611f8f565b6103d4565b6040519081526020015b60405180910390f35b6101f36101ee366004611fac565b6103f0565b6040516001600160e01b031990911681526020016101d7565b61021461042e565b6040516101d79190612055565b6101cd61022f366004611f8f565b6104c1565b6101cd6104d7565b6101cd61024a366004612068565b6104e8565b6067546101cd565b6101a5610265366004612085565b610509565b6101a5610278366004612068565b61052c565b6101a561028b36600461214e565b61053d565b6101a561029e366004612167565b61054e565b6102c66102b1366004612068565b60986020526000908152604090205460ff1681565b60405190151581526020016101d7565b6101a5610568565b6101a561057c565b6033546040516001600160a01b0390911681526020016101d7565b6101cd61030f3660046121a3565b610585565b6101cd610322366004612068565b6105b7565b6101a56103353660046122bd565b61081e565b6101cd61093a565b6101a5610350366004612315565b610946565b6101a5610363366004612068565b610957565b6102c6610376366004612068565b6001600160a01b03166000908152606d602052604090205460ff1690565b6101a56103a2366004612068565b610968565b6101a56103b5366004612068565b6109de565b6103c3816109ef565b50565b6103d08282610a46565b5050565b60006103ea606a63ffffffff80851690610a8e16565b92915050565b6000806000808480602001905181019061040a9190612460565b92509250925061041c86848484610b9d565b50630b135d3f60e11b95945050505050565b6040805160208101909152606081526040805160668054602081810284018501855283018181529293919284929091849160009085015b828210156104b457600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101610465565b5050505081525050905090565b60006103ea606b63ffffffff80851690610a8e16565b60006104e3606a610c4e565b905090565b6001600160a01b0381166000908152606c602052604081206103ea90610c4e565b6103d08260008151811061051f5761051f612534565b6020026020010151610caa565b610534610ccd565b6103c381610d27565b610545610ccd565b6103c381610dad565b610556610ccd565b61055f82610df0565b6103d0816109ef565b610570610ccd565b61057a6000610e36565b565b61057a33610e88565b6001600160a01b0382166000908152606c602052604081206105b09063ffffffff80851690610a8e16565b9392505050565b6000806066600001805480602002602001604051908101604052809291908181526020016000905b8282101561062e57600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105df565b50505050905060008082516001600160401b0381111561065057610650611ce2565b604051908082528060200260200182016040528015610679578160200160208202803683370190505b50905060005b83518110156106e25783818151811061069a5761069a612534565b6020026020010151600001518282815181106106b8576106b8612534565b6001600160a01b0390921660209283029190910190910152806106da81612560565b91505061067f565b50604051639004134760e01b81526000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639004134790610734908990869060040161257b565b600060405180830381865afa158015610751573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261077991908101906125d7565b905060005b84518110156107f05784818151811061079957610799612534565b6020026020010151602001516001600160601b03168282815181106107c0576107c0612534565b60200260200101516107d29190612667565b6107dc9085612686565b9350806107e881612560565b91505061077e565b506107fd6127108461269e565b92506067548310610812575090949350505050565b50600095945050505050565b600054610100900460ff161580801561083e5750600054600160ff909116105b806108585750303b158015610858575060005460ff166001145b6108c05760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff1916600117905580156108e3576000805461ff0019166101001790555b6108ee848484610fab565b8015610934576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60006104e3606b610c4e565b61094e610ccd565b61055f8261100c565b61095f610ccd565b6103c38161116b565b610970610ccd565b6001600160a01b0381166109d55760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016108b7565b6103c381610e36565b6109e6610ccd565b6103c3816111ab565b6000805b8251811015610a3c57610a1e838281518110610a1157610a11612534565b6020026020010151611254565b610a2890836126c0565b915080610a3481612560565b9150506109f3565b5061093481611331565b6001600160a01b03821660009081526098602052604090205460ff161515600114610a845760405163380fa21360e11b815260040160405180910390fd5b6103d0828261139d565b6000438210610adf5760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e656460448201526064016108b7565b825460005b81811015610b44576000610af882846114ca565b905084866000018281548110610b1057610b10612534565b60009182526020909120015463ffffffff161115610b3057809250610b3e565b610b3b816001612686565b91505b50610ae4565b8115610b885784610b56600184612701565b81548110610b6657610b66612534565b60009182526020909120015464010000000090046001600160e01b0316610b8b565b60005b6001600160e01b031695945050505050565b600083519050600080610bb18386516114e5565b60005b83811015610c3a576000878281518110610bd057610bd0612534565b60200260200101519050610be48482611526565b610c08818a898581518110610bfb57610bfb612534565b6020026020010151611558565b8093506000610c178288611589565b9050610c238185612686565b935050508080610c3290612560565b915050610bb4565b50610c4581856115ec565b50505050505050565b80546000908015610c975782610c65600183612701565b81548110610c7557610c75612534565b60009182526020909120015464010000000090046001600160e01b0316610c9a565b60005b6001600160e01b03169392505050565b6065548151146103ba5760405163169efb5b60e11b815260040160405180910390fd5b6033546001600160a01b0316331461057a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016108b7565b6001600160a01b03811660009081526098602052604090205460ff1615610d61576040516378f5ee6160e11b815260040160405180910390fd5b6001600160a01b038116600081815260986020526040808220805460ff19166001179055517fe9bc8eb00c0766d789ecba000f585406075b053bf1842aa19d4af52c52bc69209190a250565b610db8606b82611648565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0381166000908152606d602052604090205460ff16610ec1576040516325ec6c1f60e01b815260040160405180910390fd5b60658054906000610ed183612718565b90915550506001600160a01b0381166000908152606d60205260408120805460ff19169055610eff82611254565b9050610f0a81611331565b50506068546040516351b27a6d60e11b81526001600160a01b0384811660048301529091169063a364f4da90602401600060405180830381600087803b158015610f5357600080fd5b505af1158015610f67573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed58090600090a35050565b600054610100900460ff16610fd25760405162461bcd60e51b81526004016108b79061272f565b606880546001600160a01b0319166001600160a01b038516179055610ff682610dad565b610fff8161100c565b611007611773565b505050565b611015816117a2565b6110325760405163d173577960e01b815260040160405180910390fd5b60408051606680546020818102840185018552830181815260009484928491879085015b828210156110a557600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101611056565b5050509152509091506066905060006110be8282611cb4565b505060005b825151811015611139578251805160669190839081106110e5576110e5612534565b6020908102919091018101518254600181018455600093845292829020815191909201516001600160601b0316600160a01b026001600160a01b03909116179101558061113181612560565b9150506110c3565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610e2a92919061277a565b61117481610e88565b6040516001600160a01b038216907f44cc80141b47717cc60edd3ad54b38b00efe9fe23b2898f15bcf884b0f3ad49590600090a250565b6001600160a01b03811660009081526098602052604090205460ff166111e45760405163380fa21360e11b815260040160405180910390fd5b6001600160a01b038116600081815260986020526040808220805460ff19169055517fa5f3b7626fd86ff989f1d22cf3d41d74591ea6eb99241079400b0c332a9a8f119190a26001600160a01b0381166000908152606d602052604090205460ff16156103c3576103c38161116b565b6001600160a01b0381166000908152606d602052604081205481908190819060ff16156112b3576001600160a01b0385166000908152606c6020526040902061129e906001611648565b5092506112ac8360016127a8565b90506112e5565b6001600160a01b0385166000908152606c602052604081206112d491611648565b5092506112e28360006127a8565b90505b60408051848152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a2949350505050565b60008061133e606a610c4e565b9150600061134c84846126c0565b915081905061135c606a82611648565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b6001600160a01b0382166000908152606d602052604090205460ff16156113d7576040516342ee68b560e01b815260040160405180910390fd5b606580549060006113e783612560565b90915550506001600160a01b0382166000908152606d60205260408120805460ff1916600117905561141883611254565b905061142381611331565b5050606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d906114579086908690600401612813565b600060405180830381600087803b15801561147157600080fd5b505af1158015611485573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090861691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190600090a3505050565b60006114d9600284841861269e565b6105b090848416612686565b808214611508576040516001621398b960e31b0319815260040160405180910390fd5b816103d05760405163251f56a160e21b815260040160405180910390fd5b806001600160a01b0316826001600160a01b0316106103d05760405163ba50f91160e01b815260040160405180910390fd5b61156c6001600160a01b0384168383611872565b61100757604051638baa579f60e01b815260040160405180910390fd5b600063ffffffff82811614156115c1576001600160a01b0383166000908152606c602052604090206115ba90610c4e565b90506103ea565b6001600160a01b0383166000908152606c602052604090206115ba9063ffffffff80851690610a8e16565b60006115f7826119be565b90508083111561161a57604051634b05a0f760e11b815260040160405180910390fd5b6000611625836119f1565b9050838111156109345760405163e121632f60e01b815260040160405180910390fd5b815460009081908161165986610c4e565b905060008211801561169757504386611673600185612701565b8154811061168357611683612534565b60009182526020909120015463ffffffff16145b156116f7576116a585611a1f565b866116b1600185612701565b815481106116c1576116c1612534565b9060005260206000200160000160046101000a8154816001600160e01b0302191690836001600160e01b03160217905550611765565b85600001604051806040016040528061170f43611a8c565b63ffffffff16815260200161172388611a1f565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b9250839150505b9250929050565b600054610100900460ff1661179a5760405162461bcd60e51b81526004016108b79061272f565b61057a611af1565b8051600090818080805b8451811015611850578481815181106117c7576117c7612534565b6020026020010151600001519250826001600160a01b0316846001600160a01b0316106118075760405163ba50f91160e01b815260040160405180910390fd5b82935084818151811061181c5761181c612534565b6020026020010151602001516001600160601b03168261183c9190612686565b91508061184881612560565b9150506117ac565b5061271081146118665750600095945050505050565b50600195945050505050565b60008060006118818585611b21565b9092509050600081600481111561189a5761189a61285e565b1480156118b85750856001600160a01b0316826001600160a01b0316145b156118c8576001925050506105b0565b600080876001600160a01b0316631626ba7e60e01b88886040516024016118f0929190612874565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161192e919061288d565b600060405180830381855afa9150503d8060008114611969576040519150601f19603f3d011682016040523d82523d6000602084013e61196e565b606091505b5091509150818015611981575080516020145b80156119b257508051630b135d3f60e11b906119a690830160209081019084016128a9565b6001600160e01b031916145b98975050505050505050565b600063ffffffff82811614156119d8576103ea606a610c4e565b6103ea606a63ffffffff80851690610a8e16565b919050565b600063ffffffff8281161415611a0b576103ea606b610c4e565b6103ea606b63ffffffff80851690610a8e16565b60006001600160e01b03821115611a885760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b60648201526084016108b7565b5090565b600063ffffffff821115611a885760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b60648201526084016108b7565b600054610100900460ff16611b185760405162461bcd60e51b81526004016108b79061272f565b61057a33610e36565b600080825160411415611b585760208301516040840151606085015160001a611b4c87828585611b8e565b9450945050505061176c565b825160401415611b825760208301516040840151611b77868383611c7b565b93509350505061176c565b5060009050600261176c565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611bc55750600090506003611c72565b8460ff16601b14158015611bdd57508460ff16601c14155b15611bee5750600090506004611c72565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611c42573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116611c6b57600060019250925050611c72565b9150600090505b94509492505050565b6000806001600160ff1b03831681611c9860ff86901c601b612686565b9050611ca687828885611b8e565b935093505050935093915050565b50805460008255906000526020600020908101906103c391905b80821115611a885760008155600101611cce565b634e487b7160e01b600052604160045260246000fd5b604051602081016001600160401b0381118282101715611d1a57611d1a611ce2565b60405290565b604080519081016001600160401b0381118282101715611d1a57611d1a611ce2565b604051601f8201601f191681016001600160401b0381118282101715611d6a57611d6a611ce2565b604052919050565b60006001600160401b03821115611d8b57611d8b611ce2565b5060051b60200190565b6001600160a01b03811681146103c357600080fd5b600082601f830112611dbb57600080fd5b81356020611dd0611dcb83611d72565b611d42565b82815260059290921b84018101918181019086841115611def57600080fd5b8286015b84811015611e13578035611e0681611d95565b8352918301918301611df3565b509695505050505050565b600060208284031215611e3057600080fd5b81356001600160401b03811115611e4657600080fd5b611e5284828501611daa565b949350505050565b60006001600160401b03821115611e7357611e73611ce2565b50601f01601f191660200190565b600082601f830112611e9257600080fd5b8135611ea0611dcb82611e5a565b818152846020838601011115611eb557600080fd5b816020850160208301376000918101602001919091529392505050565b60008060408385031215611ee557600080fd5b8235611ef081611d95565b915060208301356001600160401b0380821115611f0c57600080fd5b9084019060608287031215611f2057600080fd5b604051606081018181108382111715611f3b57611f3b611ce2565b604052823582811115611f4d57600080fd5b611f5988828601611e81565b82525060208301356020820152604083013560408201528093505050509250929050565b63ffffffff811681146103c357600080fd5b600060208284031215611fa157600080fd5b81356105b081611f7d565b60008060408385031215611fbf57600080fd5b8235915060208301356001600160401b03811115611fdc57600080fd5b611fe885828601611e81565b9150509250929050565b8051602080845281518482018190526000926040919083019082870190855b8181101561204857835180516001600160a01b031684528601516001600160601b0316868401529285019291840191600101612011565b5090979650505050505050565b6020815260006105b06020830184611ff2565b60006020828403121561207a57600080fd5b81356105b081611d95565b6000806040838503121561209857600080fd5b82356001600160401b03808211156120af57600080fd5b818501915085601f8301126120c357600080fd5b813560206120d3611dcb83611d72565b82815260059290921b840181019181810190898411156120f257600080fd5b8286015b8481101561212a5780358681111561210e5760008081fd5b61211c8c86838b0101611daa565b8452509183019183016120f6565b509650508601359250508082111561214157600080fd5b50611fe885828601611e81565b60006020828403121561216057600080fd5b5035919050565b6000806040838503121561217a57600080fd5b8235915060208301356001600160401b0381111561219757600080fd5b611fe885828601611daa565b600080604083850312156121b657600080fd5b82356121c181611d95565b915060208301356121d181611f7d565b809150509250929050565b600060208083850312156121ef57600080fd5b6121f7611cf8565b915082356001600160401b0381111561220f57600080fd5b8301601f8101851361222057600080fd5b803561222e611dcb82611d72565b81815260069190911b8201830190838101908783111561224d57600080fd5b928401925b828410156122b0576040848903121561226b5760008081fd5b612273611d20565b843561227e81611d95565b8152848601356001600160601b038116811461229a5760008081fd5b8187015282526040939093019290840190612252565b8552509295945050505050565b6000806000606084860312156122d257600080fd5b83356122dd81611d95565b92506020840135915060408401356001600160401b038111156122ff57600080fd5b61230b868287016121dc565b9150509250925092565b6000806040838503121561232857600080fd5b82356001600160401b038082111561233f57600080fd5b61234b868387016121dc565b9350602085013591508082111561236157600080fd5b50611fe885828601611daa565b60005b83811015612389578181015183820152602001612371565b838111156109345750506000910152565b600082601f8301126123ab57600080fd5b815160206123bb611dcb83611d72565b82815260059290921b840181019181810190868411156123da57600080fd5b8286015b84811015611e135780516001600160401b038111156123fd5760008081fd5b8701603f8101891361240f5760008081fd5b848101516040612421611dcb83611e5a565b8281528b828486010111156124365760008081fd5b6124458389830184870161236e565b86525050509183019183016123de565b80516119ec81611f7d565b60008060006060848603121561247557600080fd5b83516001600160401b038082111561248c57600080fd5b818601915086601f8301126124a057600080fd5b815160206124b0611dcb83611d72565b82815260059290921b8401810191818101908a8411156124cf57600080fd5b948201945b838610156124f65785516124e781611d95565b825294820194908201906124d4565b9189015191975090935050508082111561250f57600080fd5b5061251c8682870161239a565b92505061252b60408501612455565b90509250925092565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60006000198214156125745761257461254a565b5060010190565b6001600160a01b038381168252604060208084018290528451918401829052600092858201929091906060860190855b818110156125c95785518516835294830194918301916001016125ab565b509098975050505050505050565b600060208083850312156125ea57600080fd5b82516001600160401b0381111561260057600080fd5b8301601f8101851361261157600080fd5b805161261f611dcb82611d72565b81815260059190911b8201830190838101908783111561263e57600080fd5b928401925b8284101561265c57835182529284019290840190612643565b979650505050505050565b60008160001904831182151516156126815761268161254a565b500290565b600082198211156126995761269961254a565b500190565b6000826126bb57634e487b7160e01b600052601260045260246000fd5b500490565b600080821280156001600160ff1b03849003851316156126e2576126e261254a565b600160ff1b83900384128116156126fb576126fb61254a565b50500190565b6000828210156127135761271361254a565b500390565b6000816127275761272761254a565b506000190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b60408152600061278d6040830185611ff2565b828103602084015261279f8185611ff2565b95945050505050565b60008083128015600160ff1b8501841216156127c6576127c661254a565b6001600160ff1b03840183138116156127e1576127e161254a565b50500390565b600081518084526127ff81602086016020860161236e565b601f01601f19169290920160200192915050565b60018060a01b038316815260406020820152600082516060604084015261283d60a08401826127e7565b90506020840151606084015260408401516080840152809150509392505050565b634e487b7160e01b600052602160045260246000fd5b828152604060208201526000611e5260408301846127e7565b6000825161289f81846020870161236e565b9190910192915050565b6000602082840312156128bb57600080fd5b81516001600160e01b0319811681146105b057600080fdfea26469706673582212201b12ccb84dc133a9f910d479d0b076293d022c27386084146694193cf41f609b64736f6c634300080c0033608060405234801561001057600080fd5b5061030b806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063778e55f31461003b5780639004134714610065575b600080fd5b610052610049366004610131565b6103e892915050565b6040519081526020015b60405180910390f35b61007861007336600461017a565b610085565b60405161005c9190610252565b60606000825167ffffffffffffffff8111156100a3576100a3610164565b6040519080825280602002602001820160405280156100cc578160200160208202803683370190505b50905060005b835181101561010d576103e88282815181106100f0576100f0610296565b602090810291909101015280610105816102ac565b9150506100d2565b509392505050565b80356001600160a01b038116811461012c57600080fd5b919050565b6000806040838503121561014457600080fd5b61014d83610115565b915061015b60208401610115565b90509250929050565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561018d57600080fd5b61019683610115565b915060208084013567ffffffffffffffff808211156101b457600080fd5b818601915086601f8301126101c857600080fd5b8135818111156101da576101da610164565b8060051b604051601f19603f830116810181811085821117156101ff576101ff610164565b60405291825284820192508381018501918983111561021d57600080fd5b938501935b828510156102425761023385610115565b84529385019392850192610222565b8096505050505050509250929050565b6020808252825182820181905260009190848201906040850190845b8181101561028a5783518352928401929184019160010161026e565b50909695505050505050565b634e487b7160e01b600052603260045260246000fd5b60006000198214156102ce57634e487b7160e01b600052601160045260246000fd5b506001019056fea2646970667358221220044609968e4209af57558f74bde90d5690cc68a4d97bf9dc5f705209c70b0d9064736f6c634300080c0033608060405234801561001057600080fd5b50610229806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80639926ee7d1461003b578063a364f4da1461004f575b600080fd5b61004d6100493660046100ec565b5050565b005b61004d61005d3660046101d1565b50565b80356001600160a01b038116811461007757600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b6040516060810167ffffffffffffffff811182821017156100b5576100b561007c565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156100e4576100e461007c565b604052919050565b600080604083850312156100ff57600080fd5b61010883610060565b915060208084013567ffffffffffffffff8082111561012657600080fd5b908501906060828803121561013a57600080fd5b610142610092565b82358281111561015157600080fd5b8301601f8101891361016257600080fd5b8035838111156101745761017461007c565b610186601f8201601f191687016100bb565b9350808452898682840101111561019c57600080fd5b808683018786013760008682860101525050818152838301358482015260408301356040820152809450505050509250929050565b6000602082840312156101e357600080fd5b6101ec82610060565b939250505056fea26469706673582212206943cdf3fa94350d7917a12c2d4b4a05065ffd77579c5291937d46934ca9fabd64736f6c634300080c0033a264697066735822122010e49fae1b8359c50bf09fbcb6403d93e07ecbf4bbda7d4ba660023feaa9780664736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x07\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0B\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[PaLr\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x97W\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01\xBFW\x80c\xBAAO\xA6\x14a\x01\xC7W\x80c\xE2\x0C\x9Fq\x14a\x01\xDFW\x80c\xFAv&\xD4\x14a\x01\xE7W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01dW\x80c\x86w~\x06\x14a\x01yW\x80c\x91j\x17\xC6\x14a\x01\xA4W\x80c\xB5-G*\x14a\x01\xACW`\0\x80\xFD[\x80c>^<#\x11a\0\xD3W\x80c>^<#\x14a\x017W\x80c?r\x86\xF4\x14a\x01?W\x80cf\xD9\xA9\xA0\x14a\x01GW\x80cpz\x92A\x14a\x01\\W`\0\x80\xFD[\x80c\n\x92T\xE4\x14a\0\xFAW\x80c\x1E\xD7\x83\x1C\x14a\x01\x04W\x80c*\xDE8\x80\x14a\x01\"W[`\0\x80\xFD[a\x01\x02a\x01\xF4V[\0[a\x01\x0Ca\x05\x18V[`@Qa\x01\x19\x91\x90a\x18\x7FV[`@Q\x80\x91\x03\x90\xF3[a\x01*a\x05zV[`@Qa\x01\x19\x91\x90a\x19(V[a\x01\x0Ca\x06\xBCV[a\x01\x0Ca\x07\x1CV[a\x01Oa\x07|V[`@Qa\x01\x19\x91\x90a\x19\xE8V[a\x01\x02a\x08bV[a\x01la\x0F\xCCV[`@Qa\x01\x19\x91\x90a\x1A\x9BV[`\x1DTa\x01\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[a\x01Oa\x10\x9CV[`\x1CTa\x01\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01la\x11\x82V[a\x01\xCFa\x12RV[`@Q\x90\x15\x15\x81R` \x01a\x01\x19V[a\x01\x0Ca\x13\x7FV[`\x07Ta\x01\xCF\x90`\xFF\x16\x81V[a\x01\xFCa\x13\xDFV[`\x1CT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x02\x17\x90a\x18XV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x02CW=`\0\x80>=`\0\xFD[P`'\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q`\x01` \x82\x01\x81\x81R``\x83\x01\x84Ra\x124\x93`\0\x93\x92\x83\x92\x91\x83\x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\x85WPP\x90R`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra'\x10` \x82\x01R\x81Q\x80Q\x92\x93P\x90\x91`\0\x90a\x02\xE3Wa\x02\xE3a\x1A\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`'T`\x1DT`@Qc\xAB\x11\x89\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92c\xAB\x11\x89\x95\x92a\x03'\x92\x91\x16\x90`d\x90\x86\x90`\x04\x01a\x1B\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03UW=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@QcX\xC1\xEB\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x92PcX\xC1\xEB\x17\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xB6W=`\0\x80>=`\0\xFD[PP`'T`\x1FT`@QcX\xC1\xEB\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x92PcX\xC1\xEB\x17\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x03W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x17W=`\0\x80>=`\0\xFD[PPPPa\x04B`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x80\x19\x16\x81R` \x01`\0\x81RP\x90V[`'T`\x1ET`@Qc\x050\r\t`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92c\n`\x1A\x12\x92a\x04x\x92\x91\x16\x90\x85\x90`\x04\x01a\x1B\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x92W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xA6W=`\0\x80>=`\0\xFD[PP`'T`\x1FT`@Qc\x050\r\t`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\n`\x1A\x12\x93Pa\x04\xE1\x92\x90\x91\x16\x90\x85\x90`\x04\x01a\x1B\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x0FW=`\0\x80>=`\0\xFD[PPPPPPPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RW[PPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x06\x9CW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\x0F\x90a\x1B\xE6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06;\x90a\x1B\xE6V[\x80\x15a\x06\x88W\x80`\x1F\x10a\x06]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x05\xF0V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05\x9EV[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RWPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RWPPPPP\x90P\x90V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08JW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\x0CW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x07\xA0V[`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x08\xE0\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD9\x91\x90a\x1C!V[`\x01a\x15\x0FV[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\t\x1A\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\t\x90\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\teW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x89\x91\x90a\x1C!V[`\x02a\x15\x0FV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-c\x1F{O0a\t\xB5C`\x01a\x1C:V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xD3\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xEDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x01W=`\0\x80>=`\0\xFD[PP`\x1ET`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92Pc\xCAf\x9F\xA7\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\nrW=`\0\x80>=`\0\xFD[PPPP`'`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x85}\xC1\x90`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xDAW=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0BY\x94P\x91\x16\x91Pc;$.J\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BR\x91\x90a\x1C!V[`\0a\x15\x0FV[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0B\x93\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\x0B\xDE\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xB5W=`\0\x80>=`\0\xFD[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-c\x1F{O0a\x0C\x03C`\x01a\x1C:V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C!\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C;W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0COW=`\0\x80>=`\0\xFD[PPPPa\x0Cz`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x80\x19\x16\x81R` \x01`\0\x81RP\x90V[`'T`\x1ET`@Qc\x050\r\t`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92c\n`\x1A\x12\x92a\x0C\xB0\x92\x91\x16\x90\x85\x90`\x04\x01a\x1B\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xCAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xDEW=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\r\x1A\x94P\x91\x16\x91Pc;$.J\x90`$\x01a\x08\x98V[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\rT\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\r\x9F\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\teW=`\0\x80>=`\0\xFD[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-c\x1F{O0a\r\xC4C`\x01a\x1C:V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xE2\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x10W=`\0\x80>=`\0\xFD[P`\0\x92P`\x02\x91Pa\x0E \x90PV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EIW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x1ET\x81Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90`\0\x90a\x0EmWa\x0Ema\x1A\xFDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\x1FT\x82Q\x91\x16\x90\x82\x90`\x01\x90\x81\x10a\x0E\x9EWa\x0E\x9Ea\x1A\xFDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'T`@Qb\xCF*\xB5`\xE0\x1B\x81R\x91\x16\x90b\xCF*\xB5\x90a\x0E\xD9\x90\x84\x90`\x04\x01a\x18\x7FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xF3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x07W=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0FC\x94P\x91\x16\x91Pc;$.J\x90`$\x01a\x08\x98V[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0F}\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\x0F\xC8\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\teW=`\0\x80>=`\0\xFD[PPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x10\x0F\x90a\x1B\xE6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10;\x90a\x1B\xE6V[\x80\x15a\x10\x88W\x80`\x1F\x10a\x10]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xF0V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x11jW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11,W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10\xC0V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x11\xC5\x90a\x1B\xE6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xF1\x90a\x1B\xE6V[\x80\x15a\x12>W\x80`\x1F\x10a\x12\x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12>V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12!W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x11\xA6V[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x12tWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13zW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x13\x02\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x1C`V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x13\x1C\x91a\x1C\x91V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x13YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13^V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x13v\x91\x90a\x1C\xADV[\x91PP[\x91\x90PV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RWPPPPP\x90P\x90V[a\x14\x08`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01gSigner 1`\xC0\x1B\x81RPa\x166V[` \x90\x81U`\x1E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81Rg)\xB4\xB3\xB72\xB9\x10\x19`\xC1\x1B\x91\x81\x01\x91\x90\x91Ra\x14X\x90a\x166V[`!U`\x1F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x14\x87\x90a\x18eV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x14\xA3W=`\0\x80>=`\0\xFD[P`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x14\xD0\x90a\x18rV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x14\xECW=`\0\x80>=`\0\xFD[P`\x1D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80\x82\x14a\x0F\xC8W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x15\x80\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\x0F\xC8a\x17LV[`\0\x80\x82`@Q` \x01a\x16J\x91\x90a\x1C\x91V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD9\x91\x90a\x1C\xD6V[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xC6W\xC7\x18\x90a\x17\x15\x90\x85\x90\x87\x90`\x04\x01a\x1C\xFFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17/W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17CW=`\0\x80>=`\0\xFD[PPPP\x91P\x91V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x18GW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xE6\x92\x91` \x01a\x1C`V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x18\0\x91a\x1C\x91V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x18=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18BV[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a)\x9D\x80a\x1D,\x839\x01\x90V[a\x03+\x80aF\xC9\x839\x01\x90V[a\x02I\x80aI\xF4\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x18\xC0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x18\x9BV[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a\x18\xE7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18\xCFV[\x83\x81\x11\x15a\x18\xF6W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x19\x14\x81` \x86\x01` \x86\x01a\x18\xCCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a\x19\xD8W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a\x19\xC2W`_\x19\x89\x85\x03\x01\x83Ra\x19\xB0\x84\x86Qa\x18\xFCV[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a\x19\x94V[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a\x19OV[P\x91\x9A\x99PPPPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x1A\x8CW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x1AwW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x1AMV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x1A\x10V[P\x91\x99\x98PPPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1A\xF0W`?\x19\x88\x86\x03\x01\x84Ra\x1A\xDE\x85\x83Qa\x18\xFCV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1A\xC2V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R` \x80\x83\x01\x85\x90R```@\x80\x85\x01\x82\x90R\x85Q\x91\x85\x01\x83\x90R\x81Q`\x80\x86\x01\x81\x90R`\0\x94\x93\x92\x83\x01\x90\x85\x90`\xA0\x88\x01\x90[\x80\x83\x10\x15a\x1B\x8CW\x83Q\x80Q\x88\x16\x83R\x86\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x83\x01R\x92\x85\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x1BSV[P\x9A\x99PPPPPPPPPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x1B\xC5`\xA0\x84\x01\x82a\x18\xFCV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1B\xFAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x1C\x1BWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1C3W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15a\x1C[WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x1C\x83\x81`\x04\x85\x01` \x87\x01a\x18\xCCV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x1C\xA3\x81\x84` \x87\x01a\x18\xCCV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\xBFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\xCFW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xE8W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xCFW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x1D#\x90\x83\x01\x84a\x18\xFCV[\x94\x93PPPPV\xFE`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0)\x9D8\x03\x80b\0)\x9D\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0FV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0xV[`\0` \x82\x84\x03\x12\x15b\0\0YW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0qW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa)\tb\0\0\x94`\09`\0a\x06\xFD\x01Ra)\t`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8DW`\x005`\xE0\x1C\x80cm[\xE9&\x11a\0\xDEW\x80c\xAB\x11\x89\x95\x11a\0\x97W\x80c\xE5\xD9\x8F\x94\x11a\0qW\x80c\xE5\xD9\x8F\x94\x14a\x03UW\x80c\xEC\x7F\xBB1\x14a\x03hW\x80c\xF2\xFD\xE3\x8B\x14a\x03\x94W\x80c\xFA\xD8\xB3*\x14a\x03\xA7W`\0\x80\xFD[\x80c\xAB\x11\x89\x95\x14a\x03'W\x80c\xB93\xFAt\x14a\x03:W\x80c\xDE\xC5\xD1\xF6\x14a\x03BW`\0\x80\xFD[\x80cm[\xE9&\x14a\x02\xA3W\x80cqP\x18\xA6\x14a\x02\xD6W\x80c\x85}\xC1\x90\x14a\x02\xDEW\x80c\x8D\xA5\xCB[\x14a\x02\xE6W\x80c\x95_-\x90\x14a\x03\x01W\x80c\x98\xEC\x1A\xC9\x14a\x03\x14W`\0\x80\xFD[\x80c1O:I\x11a\x01KW\x80cQ@\xA5H\x11a\x01%W\x80cQ@\xA5H\x14a\x02WW\x80cX\xC1\xEB\x17\x14a\x02jW\x80c^\xF53)\x14a\x02}W\x80cibU\xBE\x14a\x02\x90W`\0\x80\xFD[\x80c1O:I\x14a\x024W\x80c;$.J\x14a\x02<W\x80c@\xBF/\xB7\x14a\x02OW`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x01\x92W\x80c\n`\x1A\x12\x14a\x01\xA7W\x80c\r\xBA3\x94\x14a\x01\xBAW\x80c\x16&\xBA~\x14a\x01\xE0W\x80c\x17\x03\xA0\x18\x14a\x02\x0CW\x80c\x1EL\xD8^\x14a\x02!W[`\0\x80\xFD[a\x01\xA5a\x01\xA06`\x04a\x1E\x1EV[a\x03\xBAV[\0[a\x01\xA5a\x01\xB56`\x04a\x1E\xD2V[a\x03\xC6V[a\x01\xCDa\x01\xC86`\x04a\x1F\x8FV[a\x03\xD4V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF3a\x01\xEE6`\x04a\x1F\xACV[a\x03\xF0V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01\xD7V[a\x02\x14a\x04.V[`@Qa\x01\xD7\x91\x90a UV[a\x01\xCDa\x02/6`\x04a\x1F\x8FV[a\x04\xC1V[a\x01\xCDa\x04\xD7V[a\x01\xCDa\x02J6`\x04a hV[a\x04\xE8V[`gTa\x01\xCDV[a\x01\xA5a\x02e6`\x04a \x85V[a\x05\tV[a\x01\xA5a\x02x6`\x04a hV[a\x05,V[a\x01\xA5a\x02\x8B6`\x04a!NV[a\x05=V[a\x01\xA5a\x02\x9E6`\x04a!gV[a\x05NV[a\x02\xC6a\x02\xB16`\x04a hV[`\x98` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xD7V[a\x01\xA5a\x05hV[a\x01\xA5a\x05|V[`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xD7V[a\x01\xCDa\x03\x0F6`\x04a!\xA3V[a\x05\x85V[a\x01\xCDa\x03\"6`\x04a hV[a\x05\xB7V[a\x01\xA5a\x0356`\x04a\"\xBDV[a\x08\x1EV[a\x01\xCDa\t:V[a\x01\xA5a\x03P6`\x04a#\x15V[a\tFV[a\x01\xA5a\x03c6`\x04a hV[a\tWV[a\x02\xC6a\x03v6`\x04a hV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16\x90V[a\x01\xA5a\x03\xA26`\x04a hV[a\thV[a\x01\xA5a\x03\xB56`\x04a hV[a\t\xDEV[a\x03\xC3\x81a\t\xEFV[PV[a\x03\xD0\x82\x82a\nFV[PPV[`\0a\x03\xEA`jc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[\x92\x91PPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x04\n\x91\x90a$`V[\x92P\x92P\x92Pa\x04\x1C\x86\x84\x84\x84a\x0B\x9DV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91`\0\x90\x85\x01[\x82\x82\x10\x15a\x04\xB4W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x04eV[PPPP\x81RPP\x90P\x90V[`\0a\x03\xEA`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[`\0a\x04\xE3`ja\x0CNV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`l` R`@\x81 a\x03\xEA\x90a\x0CNV[a\x03\xD0\x82`\0\x81Q\x81\x10a\x05\x1FWa\x05\x1Fa%4V[` \x02` \x01\x01Qa\x0C\xAAV[a\x054a\x0C\xCDV[a\x03\xC3\x81a\r'V[a\x05Ea\x0C\xCDV[a\x03\xC3\x81a\r\xADV[a\x05Va\x0C\xCDV[a\x05_\x82a\r\xF0V[a\x03\xD0\x81a\t\xEFV[a\x05pa\x0C\xCDV[a\x05z`\0a\x0E6V[V[a\x05z3a\x0E\x88V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`l` R`@\x81 a\x05\xB0\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[\x93\x92PPPV[`\0\x80`f`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06.W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xDFV[PPPP\x90P`\0\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06PWa\x06Pa\x1C\xE2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06yW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x06\xE2W\x83\x81\x81Q\x81\x10a\x06\x9AWa\x06\x9Aa%4V[` \x02` \x01\x01Q`\0\x01Q\x82\x82\x81Q\x81\x10a\x06\xB8Wa\x06\xB8a%4V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x06\xDA\x81a%`V[\x91PPa\x06\x7FV[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x074\x90\x89\x90\x86\x90`\x04\x01a%{V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07y\x91\x90\x81\x01\x90a%\xD7V[\x90P`\0[\x84Q\x81\x10\x15a\x07\xF0W\x84\x81\x81Q\x81\x10a\x07\x99Wa\x07\x99a%4V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07\xC0Wa\x07\xC0a%4V[` \x02` \x01\x01Qa\x07\xD2\x91\x90a&gV[a\x07\xDC\x90\x85a&\x86V[\x93P\x80a\x07\xE8\x81a%`V[\x91PPa\x07~V[Pa\x07\xFDa'\x10\x84a&\x9EV[\x92P`gT\x83\x10a\x08\x12WP\x90\x94\x93PPPPV[P`\0\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08>WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08XWP0;\x15\x80\x15a\x08XWP`\0T`\xFF\x16`\x01\x14[a\x08\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xE3W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\xEE\x84\x84\x84a\x0F\xABV[\x80\x15a\t4W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`\0a\x04\xE3`ka\x0CNV[a\tNa\x0C\xCDV[a\x05_\x82a\x10\x0CV[a\t_a\x0C\xCDV[a\x03\xC3\x81a\x11kV[a\tpa\x0C\xCDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xB7V[a\x03\xC3\x81a\x0E6V[a\t\xE6a\x0C\xCDV[a\x03\xC3\x81a\x11\xABV[`\0\x80[\x82Q\x81\x10\x15a\n<Wa\n\x1E\x83\x82\x81Q\x81\x10a\n\x11Wa\n\x11a%4V[` \x02` \x01\x01Qa\x12TV[a\n(\x90\x83a&\xC0V[\x91P\x80a\n4\x81a%`V[\x91PPa\t\xF3V[Pa\t4\x81a\x131V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x15\x15`\x01\x14a\n\x84W`@Qc8\x0F\xA2\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xD0\x82\x82a\x13\x9DV[`\0C\x82\x10a\n\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08\xB7V[\x82T`\0[\x81\x81\x10\x15a\x0BDW`\0a\n\xF8\x82\x84a\x14\xCAV[\x90P\x84\x86`\0\x01\x82\x81T\x81\x10a\x0B\x10Wa\x0B\x10a%4V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B0W\x80\x92Pa\x0B>V[a\x0B;\x81`\x01a&\x86V[\x91P[Pa\n\xE4V[\x81\x15a\x0B\x88W\x84a\x0BV`\x01\x84a'\x01V[\x81T\x81\x10a\x0BfWa\x0Bfa%4V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B\x8BV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[`\0\x83Q\x90P`\0\x80a\x0B\xB1\x83\x86Qa\x14\xE5V[`\0[\x83\x81\x10\x15a\x0C:W`\0\x87\x82\x81Q\x81\x10a\x0B\xD0Wa\x0B\xD0a%4V[` \x02` \x01\x01Q\x90Pa\x0B\xE4\x84\x82a\x15&V[a\x0C\x08\x81\x8A\x89\x85\x81Q\x81\x10a\x0B\xFBWa\x0B\xFBa%4V[` \x02` \x01\x01Qa\x15XV[\x80\x93P`\0a\x0C\x17\x82\x88a\x15\x89V[\x90Pa\x0C#\x81\x85a&\x86V[\x93PPP\x80\x80a\x0C2\x90a%`V[\x91PPa\x0B\xB4V[Pa\x0CE\x81\x85a\x15\xECV[PPPPPPPV[\x80T`\0\x90\x80\x15a\x0C\x97W\x82a\x0Ce`\x01\x83a'\x01V[\x81T\x81\x10a\x0CuWa\x0Cua%4V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C\x9AV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`eT\x81Q\x14a\x03\xBAW`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\xB7V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x15a\raW`@Qcx\xF5\xEEa`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xE9\xBC\x8E\xB0\x0C\x07f\xD7\x89\xEC\xBA\0\x0FXT\x06\x07[\x05;\xF1\x84*\xA1\x9DJ\xF5,R\xBCi \x91\x90\xA2PV[a\r\xB8`k\x82a\x16HV[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16a\x0E\xC1W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x0E\xD1\x83a'\x18V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x0E\xFF\x82a\x12TV[\x90Pa\x0F\n\x81a\x131V[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FSW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xB7\x90a'/V[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x0F\xF6\x82a\r\xADV[a\x0F\xFF\x81a\x10\x0CV[a\x10\x07a\x17sV[PPPV[a\x10\x15\x81a\x17\xA2V[a\x102W`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R`\0\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x10\xA5W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x10VV[PPP\x91RP\x90\x91P`f\x90P`\0a\x10\xBE\x82\x82a\x1C\xB4V[PP`\0[\x82QQ\x81\x10\x15a\x119W\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x10\xE5Wa\x10\xE5a%4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x80a\x111\x81a%`V[\x91PPa\x10\xC3V[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0E*\x92\x91\x90a'zV[a\x11t\x81a\x0E\x88V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7FD\xCC\x80\x14\x1BGq|\xC6\x0E\xDD:\xD5K8\xB0\x0E\xFE\x9F\xE2;(\x98\xF1[\xCF\x88K\x0F:\xD4\x95\x90`\0\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16a\x11\xE4W`@Qc8\x0F\xA2\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\xA5\xF3\xB7bo\xD8o\xF9\x89\xF1\xD2,\xF3\xD4\x1DtY\x1E\xA6\xEB\x99$\x10y@\x0B\x0C3*\x9A\x8F\x11\x91\x90\xA2`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16\x15a\x03\xC3Wa\x03\xC3\x81a\x11kV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 T\x81\x90\x81\x90\x81\x90`\xFF\x16\x15a\x12\xB3W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`l` R`@\x90 a\x12\x9E\x90`\x01a\x16HV[P\x92Pa\x12\xAC\x83`\x01a'\xA8V[\x90Pa\x12\xE5V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`l` R`@\x81 a\x12\xD4\x91a\x16HV[P\x92Pa\x12\xE2\x83`\0a'\xA8V[\x90P[`@\x80Q\x84\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[`\0\x80a\x13>`ja\x0CNV[\x91P`\0a\x13L\x84\x84a&\xC0V[\x91P\x81\x90Pa\x13\\`j\x82a\x16HV[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16\x15a\x13\xD7W`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x13\xE7\x83a%`V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`m` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x14\x18\x83a\x12TV[\x90Pa\x14#\x81a\x131V[PP`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\x14W\x90\x86\x90\x86\x90`\x04\x01a(\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14qW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x85W=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x86\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90`\0\x90\xA3PPPV[`\0a\x14\xD9`\x02\x84\x84\x18a&\x9EV[a\x05\xB0\x90\x84\x84\x16a&\x86V[\x80\x82\x14a\x15\x08W`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x03\xD0W`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x03\xD0W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15l`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x18rV[a\x10\x07W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x81\x16\x14\x15a\x15\xC1W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`l` R`@\x90 a\x15\xBA\x90a\x0CNV[\x90Pa\x03\xEAV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`l` R`@\x90 a\x15\xBA\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[`\0a\x15\xF7\x82a\x19\xBEV[\x90P\x80\x83\x11\x15a\x16\x1AW`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16%\x83a\x19\xF1V[\x90P\x83\x81\x11\x15a\t4W`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T`\0\x90\x81\x90\x81a\x16Y\x86a\x0CNV[\x90P`\0\x82\x11\x80\x15a\x16\x97WPC\x86a\x16s`\x01\x85a'\x01V[\x81T\x81\x10a\x16\x83Wa\x16\x83a%4V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x16\xF7Wa\x16\xA5\x85a\x1A\x1FV[\x86a\x16\xB1`\x01\x85a'\x01V[\x81T\x81\x10a\x16\xC1Wa\x16\xC1a%4V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x17eV[\x85`\0\x01`@Q\x80`@\x01`@R\x80a\x17\x0FCa\x1A\x8CV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x17#\x88a\x1A\x1FV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x17\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xB7\x90a'/V[a\x05za\x1A\xF1V[\x80Q`\0\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x18PW\x84\x81\x81Q\x81\x10a\x17\xC7Wa\x17\xC7a%4V[` \x02` \x01\x01Q`\0\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x18\x07W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x18\x1CWa\x18\x1Ca%4V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x18<\x91\x90a&\x86V[\x91P\x80a\x18H\x81a%`V[\x91PPa\x17\xACV[Pa'\x10\x81\x14a\x18fWP`\0\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[`\0\x80`\0a\x18\x81\x85\x85a\x1B!V[\x90\x92P\x90P`\0\x81`\x04\x81\x11\x15a\x18\x9AWa\x18\x9Aa(^V[\x14\x80\x15a\x18\xB8WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x18\xC8W`\x01\x92PPPa\x05\xB0V[`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x18\xF0\x92\x91\x90a(tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x19.\x91\x90a(\x8DV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x19iW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19nV[``\x91P[P\x91P\x91P\x81\x80\x15a\x19\x81WP\x80Q` \x14[\x80\x15a\x19\xB2WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x19\xA6\x90\x83\x01` \x90\x81\x01\x90\x84\x01a(\xA9V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x81\x16\x14\x15a\x19\xD8Wa\x03\xEA`ja\x0CNV[a\x03\xEA`jc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[\x91\x90PV[`\0c\xFF\xFF\xFF\xFF\x82\x81\x16\x14\x15a\x1A\x0BWa\x03\xEA`ka\x0CNV[a\x03\xEA`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x1A\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08\xB7V[P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xB7V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1B\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xB7\x90a'/V[a\x05z3a\x0E6V[`\0\x80\x82Q`A\x14\x15a\x1BXW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1BL\x87\x82\x85\x85a\x1B\x8EV[\x94P\x94PPPPa\x17lV[\x82Q`@\x14\x15a\x1B\x82W` \x83\x01Q`@\x84\x01Qa\x1Bw\x86\x83\x83a\x1C{V[\x93P\x93PPPa\x17lV[P`\0\x90P`\x02a\x17lV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1B\xC5WP`\0\x90P`\x03a\x1CrV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1B\xDDWP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1B\xEEWP`\0\x90P`\x04a\x1CrV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1CBW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1CkW`\0`\x01\x92P\x92PPa\x1CrV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1C\x98`\xFF\x86\x90\x1C`\x1Ba&\x86V[\x90Pa\x1C\xA6\x87\x82\x88\x85a\x1B\x8EV[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x03\xC3\x91\x90[\x80\x82\x11\x15a\x1A\x88W`\0\x81U`\x01\x01a\x1C\xCEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\x1AWa\x1D\x1Aa\x1C\xE2V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\x1AWa\x1D\x1Aa\x1C\xE2V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1DjWa\x1Dja\x1C\xE2V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\x8BWa\x1D\x8Ba\x1C\xE2V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xC3W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\xBBW`\0\x80\xFD[\x815` a\x1D\xD0a\x1D\xCB\x83a\x1DrV[a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1D\xEFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1E\x13W\x805a\x1E\x06\x81a\x1D\x95V[\x83R\x91\x83\x01\x91\x83\x01a\x1D\xF3V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1E0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EFW`\0\x80\xFD[a\x1ER\x84\x82\x85\x01a\x1D\xAAV[\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1EsWa\x1Esa\x1C\xE2V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\x92W`\0\x80\xFD[\x815a\x1E\xA0a\x1D\xCB\x82a\x1EZV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E\xB5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xE5W`\0\x80\xFD[\x825a\x1E\xF0\x81a\x1D\x95V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\x0CW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x1F W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x1F;Wa\x1F;a\x1C\xE2V[`@R\x825\x82\x81\x11\x15a\x1FMW`\0\x80\xFD[a\x1FY\x88\x82\x86\x01a\x1E\x81V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xC3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1F\xA1W`\0\x80\xFD[\x815a\x05\xB0\x81a\x1F}V[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xBFW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xDCW`\0\x80\xFD[a\x1F\xE8\x85\x82\x86\x01a\x1E\x81V[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R`\0\x92`@\x91\x90\x83\x01\x90\x82\x87\x01\x90\x85[\x81\x81\x10\x15a HW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q`\x01`\x01``\x1B\x03\x16\x86\x84\x01R\x92\x85\x01\x92\x91\x84\x01\x91`\x01\x01a \x11V[P\x90\x97\x96PPPPPPPV[` \x81R`\0a\x05\xB0` \x83\x01\x84a\x1F\xF2V[`\0` \x82\x84\x03\x12\x15a zW`\0\x80\xFD[\x815a\x05\xB0\x81a\x1D\x95V[`\0\x80`@\x83\x85\x03\x12\x15a \x98W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \xAFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \xC3W`\0\x80\xFD[\x815` a \xD3a\x1D\xCB\x83a\x1DrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a \xF2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a!*W\x805\x86\x81\x11\x15a!\x0EW`\0\x80\x81\xFD[a!\x1C\x8C\x86\x83\x8B\x01\x01a\x1D\xAAV[\x84RP\x91\x83\x01\x91\x83\x01a \xF6V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a!AW`\0\x80\xFD[Pa\x1F\xE8\x85\x82\x86\x01a\x1E\x81V[`\0` \x82\x84\x03\x12\x15a!`W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!zW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x97W`\0\x80\xFD[a\x1F\xE8\x85\x82\x86\x01a\x1D\xAAV[`\0\x80`@\x83\x85\x03\x12\x15a!\xB6W`\0\x80\xFD[\x825a!\xC1\x81a\x1D\x95V[\x91P` \x83\x015a!\xD1\x81a\x1F}V[\x80\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a!\xEFW`\0\x80\xFD[a!\xF7a\x1C\xF8V[\x91P\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x0FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\" W`\0\x80\xFD[\x805a\".a\x1D\xCB\x82a\x1DrV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\"MW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"\xB0W`@\x84\x89\x03\x12\x15a\"kW`\0\x80\x81\xFD[a\"sa\x1D V[\x845a\"~\x81a\x1D\x95V[\x81R\x84\x86\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\"\x9AW`\0\x80\x81\xFD[\x81\x87\x01R\x82R`@\x93\x90\x93\x01\x92\x90\x84\x01\x90a\"RV[\x85RP\x92\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\xD2W`\0\x80\xFD[\x835a\"\xDD\x81a\x1D\x95V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xFFW`\0\x80\xFD[a#\x0B\x86\x82\x87\x01a!\xDCV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a#(W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#?W`\0\x80\xFD[a#K\x86\x83\x87\x01a!\xDCV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a#aW`\0\x80\xFD[Pa\x1F\xE8\x85\x82\x86\x01a\x1D\xAAV[`\0[\x83\x81\x10\x15a#\x89W\x81\x81\x01Q\x83\x82\x01R` \x01a#qV[\x83\x81\x11\x15a\t4WPP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a#\xABW`\0\x80\xFD[\x81Q` a#\xBBa\x1D\xCB\x83a\x1DrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a#\xDAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1E\x13W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xFDW`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x89\x13a$\x0FW`\0\x80\x81\xFD[\x84\x81\x01Q`@a$!a\x1D\xCB\x83a\x1EZV[\x82\x81R\x8B\x82\x84\x86\x01\x01\x11\x15a$6W`\0\x80\x81\xFD[a$E\x83\x89\x83\x01\x84\x87\x01a#nV[\x86RPPP\x91\x83\x01\x91\x83\x01a#\xDEV[\x80Qa\x19\xEC\x81a\x1F}V[`\0\x80`\0``\x84\x86\x03\x12\x15a$uW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\x8CW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\xA0W`\0\x80\xFD[\x81Q` a$\xB0a\x1D\xCB\x83a\x1DrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a$\xCFW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a$\xF6W\x85Qa$\xE7\x81a\x1D\x95V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$\xD4V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a%\x0FW`\0\x80\xFD[Pa%\x1C\x86\x82\x87\x01a#\x9AV[\x92PPa%+`@\x85\x01a$UV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a%tWa%ta%JV[P`\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x84Q\x91\x84\x01\x82\x90R`\0\x92\x85\x82\x01\x92\x90\x91\x90``\x86\x01\x90\x85[\x81\x81\x10\x15a%\xC9W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a%\xABV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a%\xEAW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a&\x11W`\0\x80\xFD[\x80Qa&\x1Fa\x1D\xCB\x82a\x1DrV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a&>W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a&\\W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a&CV[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a&\x81Wa&\x81a%JV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a&\x99Wa&\x99a%JV[P\x01\x90V[`\0\x82a&\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15a&\xE2Wa&\xE2a%JV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a&\xFBWa&\xFBa%JV[PP\x01\x90V[`\0\x82\x82\x10\x15a'\x13Wa'\x13a%JV[P\x03\x90V[`\0\x81a''Wa''a%JV[P`\0\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R`\0a'\x8D`@\x83\x01\x85a\x1F\xF2V[\x82\x81\x03` \x84\x01Ra'\x9F\x81\x85a\x1F\xF2V[\x95\x94PPPPPV[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a'\xC6Wa'\xC6a%JV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a'\xE1Wa'\xE1a%JV[PP\x03\x90V[`\0\x81Q\x80\x84Ra'\xFF\x81` \x86\x01` \x86\x01a#nV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra(=`\xA0\x84\x01\x82a'\xE7V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@` \x82\x01R`\0a\x1ER`@\x83\x01\x84a'\xE7V[`\0\x82Qa(\x9F\x81\x84` \x87\x01a#nV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\xBBW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\xB0W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x1B\x12\xCC\xB8M\xC13\xA9\xF9\x10\xD4y\xD0\xB0v)=\x02,'8`\x84\x14f\x94\x19<\xF4\x1F`\x9BdsolcC\0\x08\x0C\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\x0B\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cw\x8EU\xF3\x14a\0;W\x80c\x90\x04\x13G\x14a\0eW[`\0\x80\xFD[a\0Ra\0I6`\x04a\x011V[a\x03\xE8\x92\x91PPV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0xa\0s6`\x04a\x01zV[a\0\x85V[`@Qa\0\\\x91\x90a\x02RV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xA3Wa\0\xA3a\x01dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xCCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x01\rWa\x03\xE8\x82\x82\x81Q\x81\x10a\0\xF0Wa\0\xF0a\x02\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x01\x05\x81a\x02\xACV[\x91PPa\0\xD2V[P\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01,W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01DW`\0\x80\xFD[a\x01M\x83a\x01\x15V[\x91Pa\x01[` \x84\x01a\x01\x15V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x8DW`\0\x80\xFD[a\x01\x96\x83a\x01\x15V[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xB4W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01\xC8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xDAWa\x01\xDAa\x01dV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x01\xFFWa\x01\xFFa\x01dV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x89\x83\x11\x15a\x02\x1DW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x02BWa\x023\x85a\x01\x15V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x02\"V[\x80\x96PPPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x02\x8AW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x02nV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x02\xCEWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x04F\t\x96\x8EB\t\xAFWU\x8Ft\xBD\xE9\rV\x90\xCCh\xA4\xD9{\xF9\xDC_pR\t\xC7\x0B\r\x90dsolcC\0\x08\x0C\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02)\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x99&\xEE}\x14a\0;W\x80c\xA3d\xF4\xDA\x14a\0OW[`\0\x80\xFD[a\0Ma\0I6`\x04a\0\xECV[PPV[\0[a\0Ma\0]6`\x04a\x01\xD1V[PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0wW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xB5Wa\0\xB5a\0|V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xE4Wa\0\xE4a\0|V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0\xFFW`\0\x80\xFD[a\x01\x08\x83a\0`V[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01&W`\0\x80\xFD[\x90\x85\x01\x90``\x82\x88\x03\x12\x15a\x01:W`\0\x80\xFD[a\x01Ba\0\x92V[\x825\x82\x81\x11\x15a\x01QW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x89\x13a\x01bW`\0\x80\xFD[\x805\x83\x81\x11\x15a\x01tWa\x01ta\0|V[a\x01\x86`\x1F\x82\x01`\x1F\x19\x16\x87\x01a\0\xBBV[\x93P\x80\x84R\x89\x86\x82\x84\x01\x01\x11\x15a\x01\x9CW`\0\x80\xFD[\x80\x86\x83\x01\x87\x86\x017`\0\x86\x82\x86\x01\x01RPP\x81\x81R\x83\x83\x015\x84\x82\x01R`@\x83\x015`@\x82\x01R\x80\x94PPPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xE3W`\0\x80\xFD[a\x01\xEC\x82a\0`V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 iC\xCD\xF3\xFA\x945\ry\x17\xA1,-KJ\x05\x06_\xFDwW\x9CR\x91\x93}F\x93L\xA9\xFA\xBDdsolcC\0\x08\x0C\x003\xA2dipfsX\"\x12 \x10\xE4\x9F\xAE\x1B\x83Y\xC5\x0B\xF0\x9F\xBC\xB6@=\x93\xE0~\xCB\xF4\xBB\xDA}K\xA6`\x02?\xEA\xA9x\x06dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106100f55760003560e01c806385226c8111610097578063b5508aa911610066578063b5508aa9146101bf578063ba414fa6146101c7578063e20c9f71146101df578063fa7626d4146101e757600080fd5b806385226c811461016457806386777e0614610179578063916a17c6146101a4578063b52d472a146101ac57600080fd5b80633e5e3c23116100d35780633e5e3c23146101375780633f7286f41461013f57806366d9a9a014610147578063707a92411461015c57600080fd5b80630a9254e4146100fa5780631ed7831c146101045780632ade388014610122575b600080fd5b6101026101f4565b005b61010c610518565b604051610119919061187f565b60405180910390f35b61012a61057a565b6040516101199190611928565b61010c6106bc565b61010c61071c565b61014f61077c565b60405161011991906119e8565b610102610862565b61016c610fcc565b6040516101199190611a9b565b601d5461018c906001600160a01b031681565b6040516001600160a01b039091168152602001610119565b61014f61109c565b601c5461018c906001600160a01b031681565b61016c611182565b6101cf611252565b6040519015158152602001610119565b61010c61137f565b6007546101cf9060ff1681565b6101fc6113df565b601c546040516001600160a01b039091169061021790611858565b6001600160a01b039091168152602001604051809103906000f080158015610243573d6000803e3d6000fd5b50602780546001600160a01b0319166001600160a01b039290921691909117905560408051600160208201818152606083018452611234936000939283929183015b60408051808201909152600080825260208201528152602001906001900390816102855750509052604080518082019091526001600160a01b038416815261271060208201528151805192935090916000906102e3576102e3611afd565b6020908102919091010152602754601d5460405163ab11899560e01b81526001600160a01b039283169263ab11899592610327929116906064908690600401611b13565b600060405180830381600087803b15801561034157600080fd5b505af1158015610355573d6000803e3d6000fd5b5050602754601e546040516358c1eb1760e01b81526001600160a01b039182166004820152911692506358c1eb179150602401600060405180830381600087803b1580156103a257600080fd5b505af11580156103b6573d6000803e3d6000fd5b5050602754601f546040516358c1eb1760e01b81526001600160a01b039182166004820152911692506358c1eb179150602401600060405180830381600087803b15801561040357600080fd5b505af1158015610417573d6000803e3d6000fd5b5050505061044260405180606001604052806060815260200160008019168152602001600081525090565b602754601e546040516305300d0960e11b81526001600160a01b0392831692630a601a1292610478929116908590600401611b9b565b600060405180830381600087803b15801561049257600080fd5b505af11580156104a6573d6000803e3d6000fd5b5050602754601f546040516305300d0960e11b81526001600160a01b039283169450630a601a1293506104e192909116908590600401611b9b565b600060405180830381600087803b1580156104fb57600080fd5b505af115801561050f573d6000803e3d6000fd5b50505050505050565b6060601480548060200260200160405190810160405280929190818152602001828054801561057057602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610552575b5050505050905090565b6060601b805480602002602001604051908101604052809291908181526020016000905b828210156106b357600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561069c57838290600052602060002001805461060f90611be6565b80601f016020809104026020016040519081016040528092919081815260200182805461063b90611be6565b80156106885780601f1061065d57610100808354040283529160200191610688565b820191906000526020600020905b81548152906001019060200180831161066b57829003601f168201915b5050505050815260200190600101906105f0565b50505050815250508152602001906001019061059e565b50505050905090565b60606016805480602002602001604051908101604052809291908181526020018280548015610570576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610552575050505050905090565b60606015805480602002602001604051908101604052809291908181526020018280548015610570576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610552575050505050905090565b60606019805480602002602001604051908101604052809291908181526020016000905b828210156106b35760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561084a57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161080c5790505b505050505081525050815260200190600101906107a0565b602754601e54604051631d92172560e11b81526001600160a01b0391821660048201526108e0929190911690633b242e4a906024015b602060405180830381865afa1580156108b5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108d99190611c21565b600161150f565b602754601f54604051631d92172560e11b81526001600160a01b03918216600482015261091a929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610990926001600160a01b03169163314f3a499160048083019260209291908290030181865afa158015610965573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109899190611c21565b600261150f565b737109709ecfa91a80626ff3989d68f67f5b1dd12d631f7b4f306109b5436001611c3a565b6040518263ffffffff1660e01b81526004016109d391815260200190565b600060405180830381600087803b1580156109ed57600080fd5b505af1158015610a01573d6000803e3d6000fd5b5050601e5460405163ca669fa760e01b81526001600160a01b039091166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d925063ca669fa79150602401600060405180830381600087803b158015610a5e57600080fd5b505af1158015610a72573d6000803e3d6000fd5b50505050602760009054906101000a90046001600160a01b03166001600160a01b031663857dc1906040518163ffffffff1660e01b8152600401600060405180830381600087803b158015610ac657600080fd5b505af1158015610ada573d6000803e3d6000fd5b5050602754601e54604051631d92172560e11b81526001600160a01b039182166004820152610b59945091169150633b242e4a90602401602060405180830381865afa158015610b2e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b529190611c21565b600061150f565b602754601f54604051631d92172560e11b81526001600160a01b039182166004820152610b93929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610bde926001600160a01b03169163314f3a499160048083019260209291908290030181865afa1580156108b5573d6000803e3d6000fd5b737109709ecfa91a80626ff3989d68f67f5b1dd12d631f7b4f30610c03436001611c3a565b6040518263ffffffff1660e01b8152600401610c2191815260200190565b600060405180830381600087803b158015610c3b57600080fd5b505af1158015610c4f573d6000803e3d6000fd5b50505050610c7a60405180606001604052806060815260200160008019168152602001600081525090565b602754601e546040516305300d0960e11b81526001600160a01b0392831692630a601a1292610cb0929116908590600401611b9b565b600060405180830381600087803b158015610cca57600080fd5b505af1158015610cde573d6000803e3d6000fd5b5050602754601e54604051631d92172560e11b81526001600160a01b039182166004820152610d1a945091169150633b242e4a90602401610898565b602754601f54604051631d92172560e11b81526001600160a01b039182166004820152610d54929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610d9f926001600160a01b03169163314f3a499160048083019260209291908290030181865afa158015610965573d6000803e3d6000fd5b737109709ecfa91a80626ff3989d68f67f5b1dd12d631f7b4f30610dc4436001611c3a565b6040518263ffffffff1660e01b8152600401610de291815260200190565b600060405180830381600087803b158015610dfc57600080fd5b505af1158015610e10573d6000803e3d6000fd5b506000925060029150610e209050565b604051908082528060200260200182016040528015610e49578160200160208202803683370190505b50601e5481519192506001600160a01b0316908290600090610e6d57610e6d611afd565b6001600160a01b039283166020918202929092010152601f54825191169082906001908110610e9e57610e9e611afd565b6001600160a01b03928316602091820292909201015260275460405162cf2ab560e01b815291169062cf2ab590610ed990849060040161187f565b600060405180830381600087803b158015610ef357600080fd5b505af1158015610f07573d6000803e3d6000fd5b5050602754601e54604051631d92172560e11b81526001600160a01b039182166004820152610f43945091169150633b242e4a90602401610898565b602754601f54604051631d92172560e11b81526001600160a01b039182166004820152610f7d929190911690633b242e4a90602401610898565b6027546040805163314f3a4960e01b81529051610fc8926001600160a01b03169163314f3a499160048083019260209291908290030181865afa158015610965573d6000803e3d6000fd5b5050565b60606018805480602002602001604051908101604052809291908181526020016000905b828210156106b357838290600052602060002001805461100f90611be6565b80601f016020809104026020016040519081016040528092919081815260200182805461103b90611be6565b80156110885780601f1061105d57610100808354040283529160200191611088565b820191906000526020600020905b81548152906001019060200180831161106b57829003601f168201915b505050505081526020019060010190610ff0565b6060601a805480602002602001604051908101604052809291908181526020016000905b828210156106b35760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561116a57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161112c5790505b505050505081525050815260200190600101906110c0565b60606017805480602002602001604051908101604052809291908181526020016000905b828210156106b35783829060005260206000200180546111c590611be6565b80601f01602080910402602001604051908101604052809291908181526020018280546111f190611be6565b801561123e5780601f106112135761010080835404028352916020019161123e565b820191906000526020600020905b81548152906001019060200180831161122157829003601f168201915b5050505050815260200190600101906111a6565b600754600090610100900460ff16156112745750600754610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b1561137a5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611302917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001611c60565b60408051601f198184030181529082905261131c91611c91565b6000604051808303816000865af19150503d8060008114611359576040519150601f19603f3d011682016040523d82523d6000602084013e61135e565b606091505b50915050808060200190518101906113769190611cad565b9150505b919050565b60606013805480602002602001604051908101604052809291908181526020018280548015610570576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610552575050505050905090565b611408604051806040016040528060088152602001675369676e6572203160c01b815250611636565b6020908155601e80546001600160a01b0319166001600160a01b03939093169290921790915560408051808201909152600881526729b4b3b732b9101960c11b9181019190915261145890611636565b602155601f80546001600160a01b0319166001600160a01b039290921691909117905560405161148790611865565b604051809103906000f0801580156114a3573d6000803e3d6000fd5b50601c80546001600160a01b0319166001600160a01b03929092169190911790556040516114d090611872565b604051809103906000f0801580156114ec573d6000803e3d6000fd5b50601d80546001600160a01b0319166001600160a01b0392909216919091179055565b808214610fc8577f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f506040516115809060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b60608201526020810184905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b60608201526020810183905290517fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a89181900360800190a1610fc861174c565b6000808260405160200161164a9190611c91565b60408051808303601f190181529082905280516020909101206001625e79b760e01b03198252600482018190529150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ffa1864990602401602060405180830381865afa1580156116b5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116d99190611cd6565b6040516318caf8e360e31b8152909250737109709ecfa91a80626ff3989d68f67f5b1dd12d9063c657c718906117159085908790600401611cff565b600060405180830381600087803b15801561172f57600080fd5b505af1158015611743573d6000803e3d6000fd5b50505050915091565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156118475760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526117e69291602001611c60565b60408051601f198184030181529082905261180091611c91565b6000604051808303816000865af19150503d806000811461183d576040519150601f19603f3d011682016040523d82523d6000602084013e611842565b606091505b505050505b6007805461ff001916610100179055565b61299d80611d2c83390190565b61032b806146c983390190565b610249806149f483390190565b6020808252825182820181905260009190848201906040850190845b818110156118c05783516001600160a01b03168352928401929184019160010161189b565b50909695505050505050565b60005b838110156118e75781810151838201526020016118cf565b838111156118f6576000848401525b50505050565b600081518084526119148160208601602086016118cc565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b858110156119d857603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b818110156119c257605f198985030183526119b08486516118fc565b948e01949350918d0191600101611994565b505050978a01979450509188019160010161194f565b50919a9950505050505050505050565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015611a8c57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015611a775783516001600160e01b0319168252928b019260019290920191908b0190611a4d565b50978a01979550505091870191600101611a10565b50919998505050505050505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015611af057603f19888603018452611ade8583516118fc565b94509285019290850190600101611ac2565b5092979650505050505050565b634e487b7160e01b600052603260045260246000fd5b6001600160a01b03848116825260208083018590526060604080850182905285519185018390528151608086018190526000949392830190859060a08801905b80831015611b8c5783518051881683528601516bffffffffffffffffffffffff1686830152928501926001929092019190840190611b53565b509a9950505050505050505050565b60018060a01b0383168152604060208201526000825160606040840152611bc560a08401826118fc565b90506020840151606084015260408401516080840152809150509392505050565b600181811c90821680611bfa57607f821691505b60208210811415611c1b57634e487b7160e01b600052602260045260246000fd5b50919050565b600060208284031215611c3357600080fd5b5051919050565b60008219821115611c5b57634e487b7160e01b600052601160045260246000fd5b500190565b6001600160e01b0319831681528151600090611c838160048501602087016118cc565b919091016004019392505050565b60008251611ca38184602087016118cc565b9190910192915050565b600060208284031215611cbf57600080fd5b81518015158114611ccf57600080fd5b9392505050565b600060208284031215611ce857600080fd5b81516001600160a01b0381168114611ccf57600080fd5b6001600160a01b0383168152604060208201819052600090611d23908301846118fc565b94935050505056fe60a06040523480156200001157600080fd5b506040516200299d3803806200299d833981016040819052620000349162000046565b6001600160a01b031660805262000078565b6000602082840312156200005957600080fd5b81516001600160a01b03811681146200007157600080fd5b9392505050565b6080516129096200009460003960006106fd01526129096000f3fe608060405234801561001057600080fd5b506004361061018d5760003560e01c80636d5be926116100de578063ab11899511610097578063e5d98f9411610071578063e5d98f9414610355578063ec7fbb3114610368578063f2fde38b14610394578063fad8b32a146103a757600080fd5b8063ab11899514610327578063b933fa741461033a578063dec5d1f61461034257600080fd5b80636d5be926146102a3578063715018a6146102d6578063857dc190146102de5780638da5cb5b146102e6578063955f2d901461030157806398ec1ac91461031457600080fd5b8063314f3a491161014b5780635140a548116101255780635140a5481461025757806358c1eb171461026a5780635ef533291461027d578063696255be1461029057600080fd5b8063314f3a49146102345780633b242e4a1461023c57806340bf2fb71461024f57600080fd5b8062cf2ab5146101925780630a601a12146101a75780630dba3394146101ba5780631626ba7e146101e05780631703a0181461020c5780631e4cd85e14610221575b600080fd5b6101a56101a0366004611e1e565b6103ba565b005b6101a56101b5366004611ed2565b6103c6565b6101cd6101c8366004611f8f565b6103d4565b6040519081526020015b60405180910390f35b6101f36101ee366004611fac565b6103f0565b6040516001600160e01b031990911681526020016101d7565b61021461042e565b6040516101d79190612055565b6101cd61022f366004611f8f565b6104c1565b6101cd6104d7565b6101cd61024a366004612068565b6104e8565b6067546101cd565b6101a5610265366004612085565b610509565b6101a5610278366004612068565b61052c565b6101a561028b36600461214e565b61053d565b6101a561029e366004612167565b61054e565b6102c66102b1366004612068565b60986020526000908152604090205460ff1681565b60405190151581526020016101d7565b6101a5610568565b6101a561057c565b6033546040516001600160a01b0390911681526020016101d7565b6101cd61030f3660046121a3565b610585565b6101cd610322366004612068565b6105b7565b6101a56103353660046122bd565b61081e565b6101cd61093a565b6101a5610350366004612315565b610946565b6101a5610363366004612068565b610957565b6102c6610376366004612068565b6001600160a01b03166000908152606d602052604090205460ff1690565b6101a56103a2366004612068565b610968565b6101a56103b5366004612068565b6109de565b6103c3816109ef565b50565b6103d08282610a46565b5050565b60006103ea606a63ffffffff80851690610a8e16565b92915050565b6000806000808480602001905181019061040a9190612460565b92509250925061041c86848484610b9d565b50630b135d3f60e11b95945050505050565b6040805160208101909152606081526040805160668054602081810284018501855283018181529293919284929091849160009085015b828210156104b457600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101610465565b5050505081525050905090565b60006103ea606b63ffffffff80851690610a8e16565b60006104e3606a610c4e565b905090565b6001600160a01b0381166000908152606c602052604081206103ea90610c4e565b6103d08260008151811061051f5761051f612534565b6020026020010151610caa565b610534610ccd565b6103c381610d27565b610545610ccd565b6103c381610dad565b610556610ccd565b61055f82610df0565b6103d0816109ef565b610570610ccd565b61057a6000610e36565b565b61057a33610e88565b6001600160a01b0382166000908152606c602052604081206105b09063ffffffff80851690610a8e16565b9392505050565b6000806066600001805480602002602001604051908101604052809291908181526020016000905b8282101561062e57600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105df565b50505050905060008082516001600160401b0381111561065057610650611ce2565b604051908082528060200260200182016040528015610679578160200160208202803683370190505b50905060005b83518110156106e25783818151811061069a5761069a612534565b6020026020010151600001518282815181106106b8576106b8612534565b6001600160a01b0390921660209283029190910190910152806106da81612560565b91505061067f565b50604051639004134760e01b81526000906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639004134790610734908990869060040161257b565b600060405180830381865afa158015610751573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261077991908101906125d7565b905060005b84518110156107f05784818151811061079957610799612534565b6020026020010151602001516001600160601b03168282815181106107c0576107c0612534565b60200260200101516107d29190612667565b6107dc9085612686565b9350806107e881612560565b91505061077e565b506107fd6127108461269e565b92506067548310610812575090949350505050565b50600095945050505050565b600054610100900460ff161580801561083e5750600054600160ff909116105b806108585750303b158015610858575060005460ff166001145b6108c05760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff1916600117905580156108e3576000805461ff0019166101001790555b6108ee848484610fab565b8015610934576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b60006104e3606b610c4e565b61094e610ccd565b61055f8261100c565b61095f610ccd565b6103c38161116b565b610970610ccd565b6001600160a01b0381166109d55760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016108b7565b6103c381610e36565b6109e6610ccd565b6103c3816111ab565b6000805b8251811015610a3c57610a1e838281518110610a1157610a11612534565b6020026020010151611254565b610a2890836126c0565b915080610a3481612560565b9150506109f3565b5061093481611331565b6001600160a01b03821660009081526098602052604090205460ff161515600114610a845760405163380fa21360e11b815260040160405180910390fd5b6103d0828261139d565b6000438210610adf5760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e656460448201526064016108b7565b825460005b81811015610b44576000610af882846114ca565b905084866000018281548110610b1057610b10612534565b60009182526020909120015463ffffffff161115610b3057809250610b3e565b610b3b816001612686565b91505b50610ae4565b8115610b885784610b56600184612701565b81548110610b6657610b66612534565b60009182526020909120015464010000000090046001600160e01b0316610b8b565b60005b6001600160e01b031695945050505050565b600083519050600080610bb18386516114e5565b60005b83811015610c3a576000878281518110610bd057610bd0612534565b60200260200101519050610be48482611526565b610c08818a898581518110610bfb57610bfb612534565b6020026020010151611558565b8093506000610c178288611589565b9050610c238185612686565b935050508080610c3290612560565b915050610bb4565b50610c4581856115ec565b50505050505050565b80546000908015610c975782610c65600183612701565b81548110610c7557610c75612534565b60009182526020909120015464010000000090046001600160e01b0316610c9a565b60005b6001600160e01b03169392505050565b6065548151146103ba5760405163169efb5b60e11b815260040160405180910390fd5b6033546001600160a01b0316331461057a5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016108b7565b6001600160a01b03811660009081526098602052604090205460ff1615610d61576040516378f5ee6160e11b815260040160405180910390fd5b6001600160a01b038116600081815260986020526040808220805460ff19166001179055517fe9bc8eb00c0766d789ecba000f585406075b053bf1842aa19d4af52c52bc69209190a250565b610db8606b82611648565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0381166000908152606d602052604090205460ff16610ec1576040516325ec6c1f60e01b815260040160405180910390fd5b60658054906000610ed183612718565b90915550506001600160a01b0381166000908152606d60205260408120805460ff19169055610eff82611254565b9050610f0a81611331565b50506068546040516351b27a6d60e11b81526001600160a01b0384811660048301529091169063a364f4da90602401600060405180830381600087803b158015610f5357600080fd5b505af1158015610f67573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed58090600090a35050565b600054610100900460ff16610fd25760405162461bcd60e51b81526004016108b79061272f565b606880546001600160a01b0319166001600160a01b038516179055610ff682610dad565b610fff8161100c565b611007611773565b505050565b611015816117a2565b6110325760405163d173577960e01b815260040160405180910390fd5b60408051606680546020818102840185018552830181815260009484928491879085015b828210156110a557600084815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101611056565b5050509152509091506066905060006110be8282611cb4565b505060005b825151811015611139578251805160669190839081106110e5576110e5612534565b6020908102919091018101518254600181018455600093845292829020815191909201516001600160601b0316600160a01b026001600160a01b03909116179101558061113181612560565b9150506110c3565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610e2a92919061277a565b61117481610e88565b6040516001600160a01b038216907f44cc80141b47717cc60edd3ad54b38b00efe9fe23b2898f15bcf884b0f3ad49590600090a250565b6001600160a01b03811660009081526098602052604090205460ff166111e45760405163380fa21360e11b815260040160405180910390fd5b6001600160a01b038116600081815260986020526040808220805460ff19169055517fa5f3b7626fd86ff989f1d22cf3d41d74591ea6eb99241079400b0c332a9a8f119190a26001600160a01b0381166000908152606d602052604090205460ff16156103c3576103c38161116b565b6001600160a01b0381166000908152606d602052604081205481908190819060ff16156112b3576001600160a01b0385166000908152606c6020526040902061129e906001611648565b5092506112ac8360016127a8565b90506112e5565b6001600160a01b0385166000908152606c602052604081206112d491611648565b5092506112e28360006127a8565b90505b60408051848152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a2949350505050565b60008061133e606a610c4e565b9150600061134c84846126c0565b915081905061135c606a82611648565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b6001600160a01b0382166000908152606d602052604090205460ff16156113d7576040516342ee68b560e01b815260040160405180910390fd5b606580549060006113e783612560565b90915550506001600160a01b0382166000908152606d60205260408120805460ff1916600117905561141883611254565b905061142381611331565b5050606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d906114579086908690600401612813565b600060405180830381600087803b15801561147157600080fd5b505af1158015611485573d6000803e3d6000fd5b50506068546040516001600160a01b03918216935090861691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c190600090a3505050565b60006114d9600284841861269e565b6105b090848416612686565b808214611508576040516001621398b960e31b0319815260040160405180910390fd5b816103d05760405163251f56a160e21b815260040160405180910390fd5b806001600160a01b0316826001600160a01b0316106103d05760405163ba50f91160e01b815260040160405180910390fd5b61156c6001600160a01b0384168383611872565b61100757604051638baa579f60e01b815260040160405180910390fd5b600063ffffffff82811614156115c1576001600160a01b0383166000908152606c602052604090206115ba90610c4e565b90506103ea565b6001600160a01b0383166000908152606c602052604090206115ba9063ffffffff80851690610a8e16565b60006115f7826119be565b90508083111561161a57604051634b05a0f760e11b815260040160405180910390fd5b6000611625836119f1565b9050838111156109345760405163e121632f60e01b815260040160405180910390fd5b815460009081908161165986610c4e565b905060008211801561169757504386611673600185612701565b8154811061168357611683612534565b60009182526020909120015463ffffffff16145b156116f7576116a585611a1f565b866116b1600185612701565b815481106116c1576116c1612534565b9060005260206000200160000160046101000a8154816001600160e01b0302191690836001600160e01b03160217905550611765565b85600001604051806040016040528061170f43611a8c565b63ffffffff16815260200161172388611a1f565b6001600160e01b0390811690915282546001810184556000938452602093849020835194909301519091166401000000000263ffffffff909316929092179101555b9250839150505b9250929050565b600054610100900460ff1661179a5760405162461bcd60e51b81526004016108b79061272f565b61057a611af1565b8051600090818080805b8451811015611850578481815181106117c7576117c7612534565b6020026020010151600001519250826001600160a01b0316846001600160a01b0316106118075760405163ba50f91160e01b815260040160405180910390fd5b82935084818151811061181c5761181c612534565b6020026020010151602001516001600160601b03168261183c9190612686565b91508061184881612560565b9150506117ac565b5061271081146118665750600095945050505050565b50600195945050505050565b60008060006118818585611b21565b9092509050600081600481111561189a5761189a61285e565b1480156118b85750856001600160a01b0316826001600160a01b0316145b156118c8576001925050506105b0565b600080876001600160a01b0316631626ba7e60e01b88886040516024016118f0929190612874565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161192e919061288d565b600060405180830381855afa9150503d8060008114611969576040519150601f19603f3d011682016040523d82523d6000602084013e61196e565b606091505b5091509150818015611981575080516020145b80156119b257508051630b135d3f60e11b906119a690830160209081019084016128a9565b6001600160e01b031916145b98975050505050505050565b600063ffffffff82811614156119d8576103ea606a610c4e565b6103ea606a63ffffffff80851690610a8e16565b919050565b600063ffffffff8281161415611a0b576103ea606b610c4e565b6103ea606b63ffffffff80851690610a8e16565b60006001600160e01b03821115611a885760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b60648201526084016108b7565b5090565b600063ffffffff821115611a885760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b60648201526084016108b7565b600054610100900460ff16611b185760405162461bcd60e51b81526004016108b79061272f565b61057a33610e36565b600080825160411415611b585760208301516040840151606085015160001a611b4c87828585611b8e565b9450945050505061176c565b825160401415611b825760208301516040840151611b77868383611c7b565b93509350505061176c565b5060009050600261176c565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0831115611bc55750600090506003611c72565b8460ff16601b14158015611bdd57508460ff16601c14155b15611bee5750600090506004611c72565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611c42573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116611c6b57600060019250925050611c72565b9150600090505b94509492505050565b6000806001600160ff1b03831681611c9860ff86901c601b612686565b9050611ca687828885611b8e565b935093505050935093915050565b50805460008255906000526020600020908101906103c391905b80821115611a885760008155600101611cce565b634e487b7160e01b600052604160045260246000fd5b604051602081016001600160401b0381118282101715611d1a57611d1a611ce2565b60405290565b604080519081016001600160401b0381118282101715611d1a57611d1a611ce2565b604051601f8201601f191681016001600160401b0381118282101715611d6a57611d6a611ce2565b604052919050565b60006001600160401b03821115611d8b57611d8b611ce2565b5060051b60200190565b6001600160a01b03811681146103c357600080fd5b600082601f830112611dbb57600080fd5b81356020611dd0611dcb83611d72565b611d42565b82815260059290921b84018101918181019086841115611def57600080fd5b8286015b84811015611e13578035611e0681611d95565b8352918301918301611df3565b509695505050505050565b600060208284031215611e3057600080fd5b81356001600160401b03811115611e4657600080fd5b611e5284828501611daa565b949350505050565b60006001600160401b03821115611e7357611e73611ce2565b50601f01601f191660200190565b600082601f830112611e9257600080fd5b8135611ea0611dcb82611e5a565b818152846020838601011115611eb557600080fd5b816020850160208301376000918101602001919091529392505050565b60008060408385031215611ee557600080fd5b8235611ef081611d95565b915060208301356001600160401b0380821115611f0c57600080fd5b9084019060608287031215611f2057600080fd5b604051606081018181108382111715611f3b57611f3b611ce2565b604052823582811115611f4d57600080fd5b611f5988828601611e81565b82525060208301356020820152604083013560408201528093505050509250929050565b63ffffffff811681146103c357600080fd5b600060208284031215611fa157600080fd5b81356105b081611f7d565b60008060408385031215611fbf57600080fd5b8235915060208301356001600160401b03811115611fdc57600080fd5b611fe885828601611e81565b9150509250929050565b8051602080845281518482018190526000926040919083019082870190855b8181101561204857835180516001600160a01b031684528601516001600160601b0316868401529285019291840191600101612011565b5090979650505050505050565b6020815260006105b06020830184611ff2565b60006020828403121561207a57600080fd5b81356105b081611d95565b6000806040838503121561209857600080fd5b82356001600160401b03808211156120af57600080fd5b818501915085601f8301126120c357600080fd5b813560206120d3611dcb83611d72565b82815260059290921b840181019181810190898411156120f257600080fd5b8286015b8481101561212a5780358681111561210e5760008081fd5b61211c8c86838b0101611daa565b8452509183019183016120f6565b509650508601359250508082111561214157600080fd5b50611fe885828601611e81565b60006020828403121561216057600080fd5b5035919050565b6000806040838503121561217a57600080fd5b8235915060208301356001600160401b0381111561219757600080fd5b611fe885828601611daa565b600080604083850312156121b657600080fd5b82356121c181611d95565b915060208301356121d181611f7d565b809150509250929050565b600060208083850312156121ef57600080fd5b6121f7611cf8565b915082356001600160401b0381111561220f57600080fd5b8301601f8101851361222057600080fd5b803561222e611dcb82611d72565b81815260069190911b8201830190838101908783111561224d57600080fd5b928401925b828410156122b0576040848903121561226b5760008081fd5b612273611d20565b843561227e81611d95565b8152848601356001600160601b038116811461229a5760008081fd5b8187015282526040939093019290840190612252565b8552509295945050505050565b6000806000606084860312156122d257600080fd5b83356122dd81611d95565b92506020840135915060408401356001600160401b038111156122ff57600080fd5b61230b868287016121dc565b9150509250925092565b6000806040838503121561232857600080fd5b82356001600160401b038082111561233f57600080fd5b61234b868387016121dc565b9350602085013591508082111561236157600080fd5b50611fe885828601611daa565b60005b83811015612389578181015183820152602001612371565b838111156109345750506000910152565b600082601f8301126123ab57600080fd5b815160206123bb611dcb83611d72565b82815260059290921b840181019181810190868411156123da57600080fd5b8286015b84811015611e135780516001600160401b038111156123fd5760008081fd5b8701603f8101891361240f5760008081fd5b848101516040612421611dcb83611e5a565b8281528b828486010111156124365760008081fd5b6124458389830184870161236e565b86525050509183019183016123de565b80516119ec81611f7d565b60008060006060848603121561247557600080fd5b83516001600160401b038082111561248c57600080fd5b818601915086601f8301126124a057600080fd5b815160206124b0611dcb83611d72565b82815260059290921b8401810191818101908a8411156124cf57600080fd5b948201945b838610156124f65785516124e781611d95565b825294820194908201906124d4565b9189015191975090935050508082111561250f57600080fd5b5061251c8682870161239a565b92505061252b60408501612455565b90509250925092565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60006000198214156125745761257461254a565b5060010190565b6001600160a01b038381168252604060208084018290528451918401829052600092858201929091906060860190855b818110156125c95785518516835294830194918301916001016125ab565b509098975050505050505050565b600060208083850312156125ea57600080fd5b82516001600160401b0381111561260057600080fd5b8301601f8101851361261157600080fd5b805161261f611dcb82611d72565b81815260059190911b8201830190838101908783111561263e57600080fd5b928401925b8284101561265c57835182529284019290840190612643565b979650505050505050565b60008160001904831182151516156126815761268161254a565b500290565b600082198211156126995761269961254a565b500190565b6000826126bb57634e487b7160e01b600052601260045260246000fd5b500490565b600080821280156001600160ff1b03849003851316156126e2576126e261254a565b600160ff1b83900384128116156126fb576126fb61254a565b50500190565b6000828210156127135761271361254a565b500390565b6000816127275761272761254a565b506000190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b60408152600061278d6040830185611ff2565b828103602084015261279f8185611ff2565b95945050505050565b60008083128015600160ff1b8501841216156127c6576127c661254a565b6001600160ff1b03840183138116156127e1576127e161254a565b50500390565b600081518084526127ff81602086016020860161236e565b601f01601f19169290920160200192915050565b60018060a01b038316815260406020820152600082516060604084015261283d60a08401826127e7565b90506020840151606084015260408401516080840152809150509392505050565b634e487b7160e01b600052602160045260246000fd5b828152604060208201526000611e5260408301846127e7565b6000825161289f81846020870161236e565b9190910192915050565b6000602082840312156128bb57600080fd5b81516001600160e01b0319811681146105b057600080fdfea26469706673582212201b12ccb84dc133a9f910d479d0b076293d022c27386084146694193cf41f609b64736f6c634300080c0033608060405234801561001057600080fd5b5061030b806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063778e55f31461003b5780639004134714610065575b600080fd5b610052610049366004610131565b6103e892915050565b6040519081526020015b60405180910390f35b61007861007336600461017a565b610085565b60405161005c9190610252565b60606000825167ffffffffffffffff8111156100a3576100a3610164565b6040519080825280602002602001820160405280156100cc578160200160208202803683370190505b50905060005b835181101561010d576103e88282815181106100f0576100f0610296565b602090810291909101015280610105816102ac565b9150506100d2565b509392505050565b80356001600160a01b038116811461012c57600080fd5b919050565b6000806040838503121561014457600080fd5b61014d83610115565b915061015b60208401610115565b90509250929050565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561018d57600080fd5b61019683610115565b915060208084013567ffffffffffffffff808211156101b457600080fd5b818601915086601f8301126101c857600080fd5b8135818111156101da576101da610164565b8060051b604051601f19603f830116810181811085821117156101ff576101ff610164565b60405291825284820192508381018501918983111561021d57600080fd5b938501935b828510156102425761023385610115565b84529385019392850192610222565b8096505050505050509250929050565b6020808252825182820181905260009190848201906040850190845b8181101561028a5783518352928401929184019160010161026e565b50909695505050505050565b634e487b7160e01b600052603260045260246000fd5b60006000198214156102ce57634e487b7160e01b600052601160045260246000fd5b506001019056fea2646970667358221220044609968e4209af57558f74bde90d5690cc68a4d97bf9dc5f705209c70b0d9064736f6c634300080c0033608060405234801561001057600080fd5b50610229806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80639926ee7d1461003b578063a364f4da1461004f575b600080fd5b61004d6100493660046100ec565b5050565b005b61004d61005d3660046101d1565b50565b80356001600160a01b038116811461007757600080fd5b919050565b634e487b7160e01b600052604160045260246000fd5b6040516060810167ffffffffffffffff811182821017156100b5576100b561007c565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156100e4576100e461007c565b604052919050565b600080604083850312156100ff57600080fd5b61010883610060565b915060208084013567ffffffffffffffff8082111561012657600080fd5b908501906060828803121561013a57600080fd5b610142610092565b82358281111561015157600080fd5b8301601f8101891361016257600080fd5b8035838111156101745761017461007c565b610186601f8201601f191687016100bb565b9350808452898682840101111561019c57600080fd5b808683018786013760008682860101525050818152838301358482015260408301356040820152809450505050509250929050565b6000602082840312156101e357600080fd5b6101ec82610060565b939250505056fea26469706673582212206943cdf3fa94350d7917a12c2d4b4a05065ffd77579c5291937d46934ca9fabd64736f6c634300080c0033a264697066735822122010e49fae1b8359c50bf09fbcb6403d93e07ecbf4bbda7d4ba660023feaa9780664736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x97W\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01\xBFW\x80c\xBAAO\xA6\x14a\x01\xC7W\x80c\xE2\x0C\x9Fq\x14a\x01\xDFW\x80c\xFAv&\xD4\x14a\x01\xE7W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x01dW\x80c\x86w~\x06\x14a\x01yW\x80c\x91j\x17\xC6\x14a\x01\xA4W\x80c\xB5-G*\x14a\x01\xACW`\0\x80\xFD[\x80c>^<#\x11a\0\xD3W\x80c>^<#\x14a\x017W\x80c?r\x86\xF4\x14a\x01?W\x80cf\xD9\xA9\xA0\x14a\x01GW\x80cpz\x92A\x14a\x01\\W`\0\x80\xFD[\x80c\n\x92T\xE4\x14a\0\xFAW\x80c\x1E\xD7\x83\x1C\x14a\x01\x04W\x80c*\xDE8\x80\x14a\x01\"W[`\0\x80\xFD[a\x01\x02a\x01\xF4V[\0[a\x01\x0Ca\x05\x18V[`@Qa\x01\x19\x91\x90a\x18\x7FV[`@Q\x80\x91\x03\x90\xF3[a\x01*a\x05zV[`@Qa\x01\x19\x91\x90a\x19(V[a\x01\x0Ca\x06\xBCV[a\x01\x0Ca\x07\x1CV[a\x01Oa\x07|V[`@Qa\x01\x19\x91\x90a\x19\xE8V[a\x01\x02a\x08bV[a\x01la\x0F\xCCV[`@Qa\x01\x19\x91\x90a\x1A\x9BV[`\x1DTa\x01\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x19V[a\x01Oa\x10\x9CV[`\x1CTa\x01\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01la\x11\x82V[a\x01\xCFa\x12RV[`@Q\x90\x15\x15\x81R` \x01a\x01\x19V[a\x01\x0Ca\x13\x7FV[`\x07Ta\x01\xCF\x90`\xFF\x16\x81V[a\x01\xFCa\x13\xDFV[`\x1CT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x02\x17\x90a\x18XV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x02CW=`\0\x80>=`\0\xFD[P`'\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q`\x01` \x82\x01\x81\x81R``\x83\x01\x84Ra\x124\x93`\0\x93\x92\x83\x92\x91\x83\x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\x85WPP\x90R`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra'\x10` \x82\x01R\x81Q\x80Q\x92\x93P\x90\x91`\0\x90a\x02\xE3Wa\x02\xE3a\x1A\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`'T`\x1DT`@Qc\xAB\x11\x89\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92c\xAB\x11\x89\x95\x92a\x03'\x92\x91\x16\x90`d\x90\x86\x90`\x04\x01a\x1B\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03UW=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@QcX\xC1\xEB\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x92PcX\xC1\xEB\x17\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xB6W=`\0\x80>=`\0\xFD[PP`'T`\x1FT`@QcX\xC1\xEB\x17`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x92PcX\xC1\xEB\x17\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x03W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x17W=`\0\x80>=`\0\xFD[PPPPa\x04B`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x80\x19\x16\x81R` \x01`\0\x81RP\x90V[`'T`\x1ET`@Qc\x050\r\t`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92c\n`\x1A\x12\x92a\x04x\x92\x91\x16\x90\x85\x90`\x04\x01a\x1B\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x92W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xA6W=`\0\x80>=`\0\xFD[PP`'T`\x1FT`@Qc\x050\r\t`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\n`\x1A\x12\x93Pa\x04\xE1\x92\x90\x91\x16\x90\x85\x90`\x04\x01a\x1B\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\x0FW=`\0\x80>=`\0\xFD[PPPPPPPV[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RW[PPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x06\x9CW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\x0F\x90a\x1B\xE6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06;\x90a\x1B\xE6V[\x80\x15a\x06\x88W\x80`\x1F\x10a\x06]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x05\xF0V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05\x9EV[PPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RWPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RWPPPPP\x90P\x90V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08JW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\x0CW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x07\xA0V[`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x08\xE0\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD9\x91\x90a\x1C!V[`\x01a\x15\x0FV[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\t\x1A\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\t\x90\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\teW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x89\x91\x90a\x1C!V[`\x02a\x15\x0FV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-c\x1F{O0a\t\xB5C`\x01a\x1C:V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xD3\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xEDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\x01W=`\0\x80>=`\0\xFD[PP`\x1ET`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x92Pc\xCAf\x9F\xA7\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\nrW=`\0\x80>=`\0\xFD[PPPP`'`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x85}\xC1\x90`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xDAW=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0BY\x94P\x91\x16\x91Pc;$.J\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BR\x91\x90a\x1C!V[`\0a\x15\x0FV[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0B\x93\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\x0B\xDE\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x08\xB5W=`\0\x80>=`\0\xFD[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-c\x1F{O0a\x0C\x03C`\x01a\x1C:V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C!\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C;W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0COW=`\0\x80>=`\0\xFD[PPPPa\x0Cz`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x80\x19\x16\x81R` \x01`\0\x81RP\x90V[`'T`\x1ET`@Qc\x050\r\t`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92c\n`\x1A\x12\x92a\x0C\xB0\x92\x91\x16\x90\x85\x90`\x04\x01a\x1B\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xCAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xDEW=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\r\x1A\x94P\x91\x16\x91Pc;$.J\x90`$\x01a\x08\x98V[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\rT\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\r\x9F\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\teW=`\0\x80>=`\0\xFD[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-c\x1F{O0a\r\xC4C`\x01a\x1C:V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xE2\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x10W=`\0\x80>=`\0\xFD[P`\0\x92P`\x02\x91Pa\x0E \x90PV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EIW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x1ET\x81Q\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x90`\0\x90a\x0EmWa\x0Ema\x1A\xFDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\x1FT\x82Q\x91\x16\x90\x82\x90`\x01\x90\x81\x10a\x0E\x9EWa\x0E\x9Ea\x1A\xFDV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'T`@Qb\xCF*\xB5`\xE0\x1B\x81R\x91\x16\x90b\xCF*\xB5\x90a\x0E\xD9\x90\x84\x90`\x04\x01a\x18\x7FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xF3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x07W=`\0\x80>=`\0\xFD[PP`'T`\x1ET`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0FC\x94P\x91\x16\x91Pc;$.J\x90`$\x01a\x08\x98V[`'T`\x1FT`@Qc\x1D\x92\x17%`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x0F}\x92\x91\x90\x91\x16\x90c;$.J\x90`$\x01a\x08\x98V[`'T`@\x80Qc1O:I`\xE0\x1B\x81R\x90Qa\x0F\xC8\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c1O:I\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\teW=`\0\x80>=`\0\xFD[PPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x10\x0F\x90a\x1B\xE6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10;\x90a\x1B\xE6V[\x80\x15a\x10\x88W\x80`\x1F\x10a\x10]Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x88V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10kW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xF0V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x11jW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11,W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10\xC0V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06\xB3W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x11\xC5\x90a\x1B\xE6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xF1\x90a\x1B\xE6V[\x80\x15a\x12>W\x80`\x1F\x10a\x12\x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12>V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12!W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x11\xA6V[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x12tWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x13zW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x13\x02\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a\x1C`V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x13\x1C\x91a\x1C\x91V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x13YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x13^V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x13v\x91\x90a\x1C\xADV[\x91PP[\x91\x90PV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05pW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05RWPPPPP\x90P\x90V[a\x14\x08`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01gSigner 1`\xC0\x1B\x81RPa\x166V[` \x90\x81U`\x1E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91U`@\x80Q\x80\x82\x01\x90\x91R`\x08\x81Rg)\xB4\xB3\xB72\xB9\x10\x19`\xC1\x1B\x91\x81\x01\x91\x90\x91Ra\x14X\x90a\x166V[`!U`\x1F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x14\x87\x90a\x18eV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x14\xA3W=`\0\x80>=`\0\xFD[P`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x14\xD0\x90a\x18rV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x14\xECW=`\0\x80>=`\0\xFD[P`\x1D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80\x82\x14a\x0F\xC8W\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP`@Qa\x15\x80\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x91\x81\x90\x03`\x80\x01\x90\xA1a\x0F\xC8a\x17LV[`\0\x80\x82`@Q` \x01a\x16J\x91\x90a\x1C\x91V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xD9\x91\x90a\x1C\xD6V[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xC6W\xC7\x18\x90a\x17\x15\x90\x85\x90\x87\x90`\x04\x01a\x1C\xFFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17/W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17CW=`\0\x80>=`\0\xFD[PPPP\x91P\x91V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x18GW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xE6\x92\x91` \x01a\x1C`V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x18\0\x91a\x1C\x91V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x18=W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18BV[``\x91P[PPPP[`\x07\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[a)\x9D\x80a\x1D,\x839\x01\x90V[a\x03+\x80aF\xC9\x839\x01\x90V[a\x02I\x80aI\xF4\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x18\xC0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x18\x9BV[P\x90\x96\x95PPPPPPV[`\0[\x83\x81\x10\x15a\x18\xE7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x18\xCFV[\x83\x81\x11\x15a\x18\xF6W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra\x19\x14\x81` \x86\x01` \x86\x01a\x18\xCCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a\x19\xD8W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a\x19\xC2W`_\x19\x89\x85\x03\x01\x83Ra\x19\xB0\x84\x86Qa\x18\xFCV[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a\x19\x94V[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a\x19OV[P\x91\x9A\x99PPPPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\x1A\x8CW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\x1AwW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a\x1AMV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\x1A\x10V[P\x91\x99\x98PPPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x1A\xF0W`?\x19\x88\x86\x03\x01\x84Ra\x1A\xDE\x85\x83Qa\x18\xFCV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x1A\xC2V[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R` \x80\x83\x01\x85\x90R```@\x80\x85\x01\x82\x90R\x85Q\x91\x85\x01\x83\x90R\x81Q`\x80\x86\x01\x81\x90R`\0\x94\x93\x92\x83\x01\x90\x85\x90`\xA0\x88\x01\x90[\x80\x83\x10\x15a\x1B\x8CW\x83Q\x80Q\x88\x16\x83R\x86\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x83\x01R\x92\x85\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x1BSV[P\x9A\x99PPPPPPPPPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x1B\xC5`\xA0\x84\x01\x82a\x18\xFCV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1B\xFAW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x1C\x1BWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1C3W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15a\x1C[WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P\x01\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a\x1C\x83\x81`\x04\x85\x01` \x87\x01a\x18\xCCV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa\x1C\xA3\x81\x84` \x87\x01a\x18\xCCV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x1C\xBFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1C\xCFW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1C\xE8W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xCFW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x1D#\x90\x83\x01\x84a\x18\xFCV[\x94\x93PPPPV\xFE`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0)\x9D8\x03\x80b\0)\x9D\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0FV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0xV[`\0` \x82\x84\x03\x12\x15b\0\0YW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0qW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa)\tb\0\0\x94`\09`\0a\x06\xFD\x01Ra)\t`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8DW`\x005`\xE0\x1C\x80cm[\xE9&\x11a\0\xDEW\x80c\xAB\x11\x89\x95\x11a\0\x97W\x80c\xE5\xD9\x8F\x94\x11a\0qW\x80c\xE5\xD9\x8F\x94\x14a\x03UW\x80c\xEC\x7F\xBB1\x14a\x03hW\x80c\xF2\xFD\xE3\x8B\x14a\x03\x94W\x80c\xFA\xD8\xB3*\x14a\x03\xA7W`\0\x80\xFD[\x80c\xAB\x11\x89\x95\x14a\x03'W\x80c\xB93\xFAt\x14a\x03:W\x80c\xDE\xC5\xD1\xF6\x14a\x03BW`\0\x80\xFD[\x80cm[\xE9&\x14a\x02\xA3W\x80cqP\x18\xA6\x14a\x02\xD6W\x80c\x85}\xC1\x90\x14a\x02\xDEW\x80c\x8D\xA5\xCB[\x14a\x02\xE6W\x80c\x95_-\x90\x14a\x03\x01W\x80c\x98\xEC\x1A\xC9\x14a\x03\x14W`\0\x80\xFD[\x80c1O:I\x11a\x01KW\x80cQ@\xA5H\x11a\x01%W\x80cQ@\xA5H\x14a\x02WW\x80cX\xC1\xEB\x17\x14a\x02jW\x80c^\xF53)\x14a\x02}W\x80cibU\xBE\x14a\x02\x90W`\0\x80\xFD[\x80c1O:I\x14a\x024W\x80c;$.J\x14a\x02<W\x80c@\xBF/\xB7\x14a\x02OW`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x01\x92W\x80c\n`\x1A\x12\x14a\x01\xA7W\x80c\r\xBA3\x94\x14a\x01\xBAW\x80c\x16&\xBA~\x14a\x01\xE0W\x80c\x17\x03\xA0\x18\x14a\x02\x0CW\x80c\x1EL\xD8^\x14a\x02!W[`\0\x80\xFD[a\x01\xA5a\x01\xA06`\x04a\x1E\x1EV[a\x03\xBAV[\0[a\x01\xA5a\x01\xB56`\x04a\x1E\xD2V[a\x03\xC6V[a\x01\xCDa\x01\xC86`\x04a\x1F\x8FV[a\x03\xD4V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xF3a\x01\xEE6`\x04a\x1F\xACV[a\x03\xF0V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01\xD7V[a\x02\x14a\x04.V[`@Qa\x01\xD7\x91\x90a UV[a\x01\xCDa\x02/6`\x04a\x1F\x8FV[a\x04\xC1V[a\x01\xCDa\x04\xD7V[a\x01\xCDa\x02J6`\x04a hV[a\x04\xE8V[`gTa\x01\xCDV[a\x01\xA5a\x02e6`\x04a \x85V[a\x05\tV[a\x01\xA5a\x02x6`\x04a hV[a\x05,V[a\x01\xA5a\x02\x8B6`\x04a!NV[a\x05=V[a\x01\xA5a\x02\x9E6`\x04a!gV[a\x05NV[a\x02\xC6a\x02\xB16`\x04a hV[`\x98` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xD7V[a\x01\xA5a\x05hV[a\x01\xA5a\x05|V[`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xD7V[a\x01\xCDa\x03\x0F6`\x04a!\xA3V[a\x05\x85V[a\x01\xCDa\x03\"6`\x04a hV[a\x05\xB7V[a\x01\xA5a\x0356`\x04a\"\xBDV[a\x08\x1EV[a\x01\xCDa\t:V[a\x01\xA5a\x03P6`\x04a#\x15V[a\tFV[a\x01\xA5a\x03c6`\x04a hV[a\tWV[a\x02\xC6a\x03v6`\x04a hV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16\x90V[a\x01\xA5a\x03\xA26`\x04a hV[a\thV[a\x01\xA5a\x03\xB56`\x04a hV[a\t\xDEV[a\x03\xC3\x81a\t\xEFV[PV[a\x03\xD0\x82\x82a\nFV[PPV[`\0a\x03\xEA`jc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[\x92\x91PPV[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x04\n\x91\x90a$`V[\x92P\x92P\x92Pa\x04\x1C\x86\x84\x84\x84a\x0B\x9DV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91`\0\x90\x85\x01[\x82\x82\x10\x15a\x04\xB4W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x04eV[PPPP\x81RPP\x90P\x90V[`\0a\x03\xEA`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[`\0a\x04\xE3`ja\x0CNV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`l` R`@\x81 a\x03\xEA\x90a\x0CNV[a\x03\xD0\x82`\0\x81Q\x81\x10a\x05\x1FWa\x05\x1Fa%4V[` \x02` \x01\x01Qa\x0C\xAAV[a\x054a\x0C\xCDV[a\x03\xC3\x81a\r'V[a\x05Ea\x0C\xCDV[a\x03\xC3\x81a\r\xADV[a\x05Va\x0C\xCDV[a\x05_\x82a\r\xF0V[a\x03\xD0\x81a\t\xEFV[a\x05pa\x0C\xCDV[a\x05z`\0a\x0E6V[V[a\x05z3a\x0E\x88V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`l` R`@\x81 a\x05\xB0\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[\x93\x92PPPV[`\0\x80`f`\0\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x06.W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xDFV[PPPP\x90P`\0\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06PWa\x06Pa\x1C\xE2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06yW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x06\xE2W\x83\x81\x81Q\x81\x10a\x06\x9AWa\x06\x9Aa%4V[` \x02` \x01\x01Q`\0\x01Q\x82\x82\x81Q\x81\x10a\x06\xB8Wa\x06\xB8a%4V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x06\xDA\x81a%`V[\x91PPa\x06\x7FV[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x074\x90\x89\x90\x86\x90`\x04\x01a%{V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07y\x91\x90\x81\x01\x90a%\xD7V[\x90P`\0[\x84Q\x81\x10\x15a\x07\xF0W\x84\x81\x81Q\x81\x10a\x07\x99Wa\x07\x99a%4V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07\xC0Wa\x07\xC0a%4V[` \x02` \x01\x01Qa\x07\xD2\x91\x90a&gV[a\x07\xDC\x90\x85a&\x86V[\x93P\x80a\x07\xE8\x81a%`V[\x91PPa\x07~V[Pa\x07\xFDa'\x10\x84a&\x9EV[\x92P`gT\x83\x10a\x08\x12WP\x90\x94\x93PPPPV[P`\0\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08>WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08XWP0;\x15\x80\x15a\x08XWP`\0T`\xFF\x16`\x01\x14[a\x08\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\xE3W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\xEE\x84\x84\x84a\x0F\xABV[\x80\x15a\t4W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`\0a\x04\xE3`ka\x0CNV[a\tNa\x0C\xCDV[a\x05_\x82a\x10\x0CV[a\t_a\x0C\xCDV[a\x03\xC3\x81a\x11kV[a\tpa\x0C\xCDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xB7V[a\x03\xC3\x81a\x0E6V[a\t\xE6a\x0C\xCDV[a\x03\xC3\x81a\x11\xABV[`\0\x80[\x82Q\x81\x10\x15a\n<Wa\n\x1E\x83\x82\x81Q\x81\x10a\n\x11Wa\n\x11a%4V[` \x02` \x01\x01Qa\x12TV[a\n(\x90\x83a&\xC0V[\x91P\x80a\n4\x81a%`V[\x91PPa\t\xF3V[Pa\t4\x81a\x131V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x15\x15`\x01\x14a\n\x84W`@Qc8\x0F\xA2\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xD0\x82\x82a\x13\x9DV[`\0C\x82\x10a\n\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08\xB7V[\x82T`\0[\x81\x81\x10\x15a\x0BDW`\0a\n\xF8\x82\x84a\x14\xCAV[\x90P\x84\x86`\0\x01\x82\x81T\x81\x10a\x0B\x10Wa\x0B\x10a%4V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B0W\x80\x92Pa\x0B>V[a\x0B;\x81`\x01a&\x86V[\x91P[Pa\n\xE4V[\x81\x15a\x0B\x88W\x84a\x0BV`\x01\x84a'\x01V[\x81T\x81\x10a\x0BfWa\x0Bfa%4V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B\x8BV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[`\0\x83Q\x90P`\0\x80a\x0B\xB1\x83\x86Qa\x14\xE5V[`\0[\x83\x81\x10\x15a\x0C:W`\0\x87\x82\x81Q\x81\x10a\x0B\xD0Wa\x0B\xD0a%4V[` \x02` \x01\x01Q\x90Pa\x0B\xE4\x84\x82a\x15&V[a\x0C\x08\x81\x8A\x89\x85\x81Q\x81\x10a\x0B\xFBWa\x0B\xFBa%4V[` \x02` \x01\x01Qa\x15XV[\x80\x93P`\0a\x0C\x17\x82\x88a\x15\x89V[\x90Pa\x0C#\x81\x85a&\x86V[\x93PPP\x80\x80a\x0C2\x90a%`V[\x91PPa\x0B\xB4V[Pa\x0CE\x81\x85a\x15\xECV[PPPPPPPV[\x80T`\0\x90\x80\x15a\x0C\x97W\x82a\x0Ce`\x01\x83a'\x01V[\x81T\x81\x10a\x0CuWa\x0Cua%4V[`\0\x91\x82R` \x90\x91 \x01Td\x01\0\0\0\0\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0C\x9AV[`\0[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`eT\x81Q\x14a\x03\xBAW`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\xB7V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x15a\raW`@Qcx\xF5\xEEa`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xE9\xBC\x8E\xB0\x0C\x07f\xD7\x89\xEC\xBA\0\x0FXT\x06\x07[\x05;\xF1\x84*\xA1\x9DJ\xF5,R\xBCi \x91\x90\xA2PV[a\r\xB8`k\x82a\x16HV[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16a\x0E\xC1W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x0E\xD1\x83a'\x18V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x0E\xFF\x82a\x12TV[\x90Pa\x0F\n\x81a\x131V[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0FSW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0FgW=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x0F\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xB7\x90a'/V[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x0F\xF6\x82a\r\xADV[a\x0F\xFF\x81a\x10\x0CV[a\x10\x07a\x17sV[PPPV[a\x10\x15\x81a\x17\xA2V[a\x102W`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R`\0\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x10\xA5W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x10VV[PPP\x91RP\x90\x91P`f\x90P`\0a\x10\xBE\x82\x82a\x1C\xB4V[PP`\0[\x82QQ\x81\x10\x15a\x119W\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x10\xE5Wa\x10\xE5a%4V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x80a\x111\x81a%`V[\x91PPa\x10\xC3V[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0E*\x92\x91\x90a'zV[a\x11t\x81a\x0E\x88V[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7FD\xCC\x80\x14\x1BGq|\xC6\x0E\xDD:\xD5K8\xB0\x0E\xFE\x9F\xE2;(\x98\xF1[\xCF\x88K\x0F:\xD4\x95\x90`\0\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16a\x11\xE4W`@Qc8\x0F\xA2\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\xA5\xF3\xB7bo\xD8o\xF9\x89\xF1\xD2,\xF3\xD4\x1DtY\x1E\xA6\xEB\x99$\x10y@\x0B\x0C3*\x9A\x8F\x11\x91\x90\xA2`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16\x15a\x03\xC3Wa\x03\xC3\x81a\x11kV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`m` R`@\x81 T\x81\x90\x81\x90\x81\x90`\xFF\x16\x15a\x12\xB3W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`l` R`@\x90 a\x12\x9E\x90`\x01a\x16HV[P\x92Pa\x12\xAC\x83`\x01a'\xA8V[\x90Pa\x12\xE5V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`l` R`@\x81 a\x12\xD4\x91a\x16HV[P\x92Pa\x12\xE2\x83`\0a'\xA8V[\x90P[`@\x80Q\x84\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[`\0\x80a\x13>`ja\x0CNV[\x91P`\0a\x13L\x84\x84a&\xC0V[\x91P\x81\x90Pa\x13\\`j\x82a\x16HV[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`m` R`@\x90 T`\xFF\x16\x15a\x13\xD7W`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90`\0a\x13\xE7\x83a%`V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`m` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x14\x18\x83a\x12TV[\x90Pa\x14#\x81a\x131V[PP`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\x14W\x90\x86\x90\x86\x90`\x04\x01a(\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14qW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x85W=`\0\x80>=`\0\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x86\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90`\0\x90\xA3PPPV[`\0a\x14\xD9`\x02\x84\x84\x18a&\x9EV[a\x05\xB0\x90\x84\x84\x16a&\x86V[\x80\x82\x14a\x15\x08W`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81a\x03\xD0W`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x03\xD0W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x15l`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x18rV[a\x10\x07W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0c\xFF\xFF\xFF\xFF\x82\x81\x16\x14\x15a\x15\xC1W`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`l` R`@\x90 a\x15\xBA\x90a\x0CNV[\x90Pa\x03\xEAV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`l` R`@\x90 a\x15\xBA\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[`\0a\x15\xF7\x82a\x19\xBEV[\x90P\x80\x83\x11\x15a\x16\x1AW`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16%\x83a\x19\xF1V[\x90P\x83\x81\x11\x15a\t4W`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T`\0\x90\x81\x90\x81a\x16Y\x86a\x0CNV[\x90P`\0\x82\x11\x80\x15a\x16\x97WPC\x86a\x16s`\x01\x85a'\x01V[\x81T\x81\x10a\x16\x83Wa\x16\x83a%4V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x16\xF7Wa\x16\xA5\x85a\x1A\x1FV[\x86a\x16\xB1`\x01\x85a'\x01V[\x81T\x81\x10a\x16\xC1Wa\x16\xC1a%4V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x17eV[\x85`\0\x01`@Q\x80`@\x01`@R\x80a\x17\x0FCa\x1A\x8CV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x17#\x88a\x1A\x1FV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16d\x01\0\0\0\0\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x17\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xB7\x90a'/V[a\x05za\x1A\xF1V[\x80Q`\0\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x18PW\x84\x81\x81Q\x81\x10a\x17\xC7Wa\x17\xC7a%4V[` \x02` \x01\x01Q`\0\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x18\x07W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x18\x1CWa\x18\x1Ca%4V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x18<\x91\x90a&\x86V[\x91P\x80a\x18H\x81a%`V[\x91PPa\x17\xACV[Pa'\x10\x81\x14a\x18fWP`\0\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[`\0\x80`\0a\x18\x81\x85\x85a\x1B!V[\x90\x92P\x90P`\0\x81`\x04\x81\x11\x15a\x18\x9AWa\x18\x9Aa(^V[\x14\x80\x15a\x18\xB8WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x18\xC8W`\x01\x92PPPa\x05\xB0V[`\0\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x18\xF0\x92\x91\x90a(tV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x19.\x91\x90a(\x8DV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x19iW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19nV[``\x91P[P\x91P\x91P\x81\x80\x15a\x19\x81WP\x80Q` \x14[\x80\x15a\x19\xB2WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x19\xA6\x90\x83\x01` \x90\x81\x01\x90\x84\x01a(\xA9V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[`\0c\xFF\xFF\xFF\xFF\x82\x81\x16\x14\x15a\x19\xD8Wa\x03\xEA`ja\x0CNV[a\x03\xEA`jc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[\x91\x90PV[`\0c\xFF\xFF\xFF\xFF\x82\x81\x16\x14\x15a\x1A\x0BWa\x03\xEA`ka\x0CNV[a\x03\xEA`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\n\x8E\x16V[`\0`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x1A\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08\xB7V[P\x90V[`\0c\xFF\xFF\xFF\xFF\x82\x11\x15a\x1A\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xB7V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1B\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xB7\x90a'/V[a\x05z3a\x0E6V[`\0\x80\x82Q`A\x14\x15a\x1BXW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1BL\x87\x82\x85\x85a\x1B\x8EV[\x94P\x94PPPPa\x17lV[\x82Q`@\x14\x15a\x1B\x82W` \x83\x01Q`@\x84\x01Qa\x1Bw\x86\x83\x83a\x1C{V[\x93P\x93PPPa\x17lV[P`\0\x90P`\x02a\x17lV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1B\xC5WP`\0\x90P`\x03a\x1CrV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1B\xDDWP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1B\xEEWP`\0\x90P`\x04a\x1CrV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1CBW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1CkW`\0`\x01\x92P\x92PPa\x1CrV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1C\x98`\xFF\x86\x90\x1C`\x1Ba&\x86V[\x90Pa\x1C\xA6\x87\x82\x88\x85a\x1B\x8EV[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90a\x03\xC3\x91\x90[\x80\x82\x11\x15a\x1A\x88W`\0\x81U`\x01\x01a\x1C\xCEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\x1AWa\x1D\x1Aa\x1C\xE2V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\x1AWa\x1D\x1Aa\x1C\xE2V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1DjWa\x1Dja\x1C\xE2V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\x8BWa\x1D\x8Ba\x1C\xE2V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xC3W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\xBBW`\0\x80\xFD[\x815` a\x1D\xD0a\x1D\xCB\x83a\x1DrV[a\x1DBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x1D\xEFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1E\x13W\x805a\x1E\x06\x81a\x1D\x95V[\x83R\x91\x83\x01\x91\x83\x01a\x1D\xF3V[P\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1E0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EFW`\0\x80\xFD[a\x1ER\x84\x82\x85\x01a\x1D\xAAV[\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1EsWa\x1Esa\x1C\xE2V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\x92W`\0\x80\xFD[\x815a\x1E\xA0a\x1D\xCB\x82a\x1EZV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E\xB5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1E\xE5W`\0\x80\xFD[\x825a\x1E\xF0\x81a\x1D\x95V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\x0CW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x1F W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x1F;Wa\x1F;a\x1C\xE2V[`@R\x825\x82\x81\x11\x15a\x1FMW`\0\x80\xFD[a\x1FY\x88\x82\x86\x01a\x1E\x81V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xC3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1F\xA1W`\0\x80\xFD[\x815a\x05\xB0\x81a\x1F}V[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xBFW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xDCW`\0\x80\xFD[a\x1F\xE8\x85\x82\x86\x01a\x1E\x81V[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R`\0\x92`@\x91\x90\x83\x01\x90\x82\x87\x01\x90\x85[\x81\x81\x10\x15a HW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q`\x01`\x01``\x1B\x03\x16\x86\x84\x01R\x92\x85\x01\x92\x91\x84\x01\x91`\x01\x01a \x11V[P\x90\x97\x96PPPPPPPV[` \x81R`\0a\x05\xB0` \x83\x01\x84a\x1F\xF2V[`\0` \x82\x84\x03\x12\x15a zW`\0\x80\xFD[\x815a\x05\xB0\x81a\x1D\x95V[`\0\x80`@\x83\x85\x03\x12\x15a \x98W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \xAFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \xC3W`\0\x80\xFD[\x815` a \xD3a\x1D\xCB\x83a\x1DrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a \xF2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a!*W\x805\x86\x81\x11\x15a!\x0EW`\0\x80\x81\xFD[a!\x1C\x8C\x86\x83\x8B\x01\x01a\x1D\xAAV[\x84RP\x91\x83\x01\x91\x83\x01a \xF6V[P\x96PP\x86\x015\x92PP\x80\x82\x11\x15a!AW`\0\x80\xFD[Pa\x1F\xE8\x85\x82\x86\x01a\x1E\x81V[`\0` \x82\x84\x03\x12\x15a!`W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a!zW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\x97W`\0\x80\xFD[a\x1F\xE8\x85\x82\x86\x01a\x1D\xAAV[`\0\x80`@\x83\x85\x03\x12\x15a!\xB6W`\0\x80\xFD[\x825a!\xC1\x81a\x1D\x95V[\x91P` \x83\x015a!\xD1\x81a\x1F}V[\x80\x91PP\x92P\x92\x90PV[`\0` \x80\x83\x85\x03\x12\x15a!\xEFW`\0\x80\xFD[a!\xF7a\x1C\xF8V[\x91P\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x0FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\" W`\0\x80\xFD[\x805a\".a\x1D\xCB\x82a\x1DrV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\"MW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\"\xB0W`@\x84\x89\x03\x12\x15a\"kW`\0\x80\x81\xFD[a\"sa\x1D V[\x845a\"~\x81a\x1D\x95V[\x81R\x84\x86\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\"\x9AW`\0\x80\x81\xFD[\x81\x87\x01R\x82R`@\x93\x90\x93\x01\x92\x90\x84\x01\x90a\"RV[\x85RP\x92\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\"\xD2W`\0\x80\xFD[\x835a\"\xDD\x81a\x1D\x95V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xFFW`\0\x80\xFD[a#\x0B\x86\x82\x87\x01a!\xDCV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a#(W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#?W`\0\x80\xFD[a#K\x86\x83\x87\x01a!\xDCV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a#aW`\0\x80\xFD[Pa\x1F\xE8\x85\x82\x86\x01a\x1D\xAAV[`\0[\x83\x81\x10\x15a#\x89W\x81\x81\x01Q\x83\x82\x01R` \x01a#qV[\x83\x81\x11\x15a\t4WPP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12a#\xABW`\0\x80\xFD[\x81Q` a#\xBBa\x1D\xCB\x83a\x1DrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a#\xDAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x1E\x13W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xFDW`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x89\x13a$\x0FW`\0\x80\x81\xFD[\x84\x81\x01Q`@a$!a\x1D\xCB\x83a\x1EZV[\x82\x81R\x8B\x82\x84\x86\x01\x01\x11\x15a$6W`\0\x80\x81\xFD[a$E\x83\x89\x83\x01\x84\x87\x01a#nV[\x86RPPP\x91\x83\x01\x91\x83\x01a#\xDEV[\x80Qa\x19\xEC\x81a\x1F}V[`\0\x80`\0``\x84\x86\x03\x12\x15a$uW`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$\x8CW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a$\xA0W`\0\x80\xFD[\x81Q` a$\xB0a\x1D\xCB\x83a\x1DrV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a$\xCFW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a$\xF6W\x85Qa$\xE7\x81a\x1D\x95V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$\xD4V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a%\x0FW`\0\x80\xFD[Pa%\x1C\x86\x82\x87\x01a#\x9AV[\x92PPa%+`@\x85\x01a$UV[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a%tWa%ta%JV[P`\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x84Q\x91\x84\x01\x82\x90R`\0\x92\x85\x82\x01\x92\x90\x91\x90``\x86\x01\x90\x85[\x81\x81\x10\x15a%\xC9W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a%\xABV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a%\xEAW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a&\x11W`\0\x80\xFD[\x80Qa&\x1Fa\x1D\xCB\x82a\x1DrV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a&>W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a&\\W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a&CV[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a&\x81Wa&\x81a%JV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a&\x99Wa&\x99a%JV[P\x01\x90V[`\0\x82a&\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15a&\xE2Wa&\xE2a%JV[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15a&\xFBWa&\xFBa%JV[PP\x01\x90V[`\0\x82\x82\x10\x15a'\x13Wa'\x13a%JV[P\x03\x90V[`\0\x81a''Wa''a%JV[P`\0\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R`\0a'\x8D`@\x83\x01\x85a\x1F\xF2V[\x82\x81\x03` \x84\x01Ra'\x9F\x81\x85a\x1F\xF2V[\x95\x94PPPPPV[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a'\xC6Wa'\xC6a%JV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a'\xE1Wa'\xE1a%JV[PP\x03\x90V[`\0\x81Q\x80\x84Ra'\xFF\x81` \x86\x01` \x86\x01a#nV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra(=`\xA0\x84\x01\x82a'\xE7V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82\x81R`@` \x82\x01R`\0a\x1ER`@\x83\x01\x84a'\xE7V[`\0\x82Qa(\x9F\x81\x84` \x87\x01a#nV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\xBBW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05\xB0W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x1B\x12\xCC\xB8M\xC13\xA9\xF9\x10\xD4y\xD0\xB0v)=\x02,'8`\x84\x14f\x94\x19<\xF4\x1F`\x9BdsolcC\0\x08\x0C\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\x0B\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cw\x8EU\xF3\x14a\0;W\x80c\x90\x04\x13G\x14a\0eW[`\0\x80\xFD[a\0Ra\0I6`\x04a\x011V[a\x03\xE8\x92\x91PPV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0xa\0s6`\x04a\x01zV[a\0\x85V[`@Qa\0\\\x91\x90a\x02RV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\0\xA3Wa\0\xA3a\x01dV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\0\xCCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x01\rWa\x03\xE8\x82\x82\x81Q\x81\x10a\0\xF0Wa\0\xF0a\x02\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x01\x05\x81a\x02\xACV[\x91PPa\0\xD2V[P\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01,W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x01DW`\0\x80\xFD[a\x01M\x83a\x01\x15V[\x91Pa\x01[` \x84\x01a\x01\x15V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x8DW`\0\x80\xFD[a\x01\x96\x83a\x01\x15V[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xB4W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01\xC8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xDAWa\x01\xDAa\x01dV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x01\xFFWa\x01\xFFa\x01dV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x89\x83\x11\x15a\x02\x1DW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x02BWa\x023\x85a\x01\x15V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x02\"V[\x80\x96PPPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x02\x8AW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x02nV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x02\xCEWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x04F\t\x96\x8EB\t\xAFWU\x8Ft\xBD\xE9\rV\x90\xCCh\xA4\xD9{\xF9\xDC_pR\t\xC7\x0B\r\x90dsolcC\0\x08\x0C\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02)\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x99&\xEE}\x14a\0;W\x80c\xA3d\xF4\xDA\x14a\0OW[`\0\x80\xFD[a\0Ma\0I6`\x04a\0\xECV[PPV[\0[a\0Ma\0]6`\x04a\x01\xD1V[PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0wW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xB5Wa\0\xB5a\0|V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xE4Wa\0\xE4a\0|V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0\xFFW`\0\x80\xFD[a\x01\x08\x83a\0`V[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01&W`\0\x80\xFD[\x90\x85\x01\x90``\x82\x88\x03\x12\x15a\x01:W`\0\x80\xFD[a\x01Ba\0\x92V[\x825\x82\x81\x11\x15a\x01QW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x89\x13a\x01bW`\0\x80\xFD[\x805\x83\x81\x11\x15a\x01tWa\x01ta\0|V[a\x01\x86`\x1F\x82\x01`\x1F\x19\x16\x87\x01a\0\xBBV[\x93P\x80\x84R\x89\x86\x82\x84\x01\x01\x11\x15a\x01\x9CW`\0\x80\xFD[\x80\x86\x83\x01\x87\x86\x017`\0\x86\x82\x86\x01\x01RPP\x81\x81R\x83\x83\x015\x84\x82\x01R`@\x83\x015`@\x82\x01R\x80\x94PPPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xE3W`\0\x80\xFD[a\x01\xEC\x82a\0`V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 iC\xCD\xF3\xFA\x945\ry\x17\xA1,-KJ\x05\x06_\xFDwW\x9CR\x91\x93}F\x93L\xA9\xFA\xBDdsolcC\0\x08\x0C\x003\xA2dipfsX\"\x12 \x10\xE4\x9F\xAE\x1B\x83Y\xC5\x0B\xF0\x9F\xBC\xB6@=\x93\xE0~\xCB\xF4\xBB\xDA}K\xA6`\x02?\xEA\xA9x\x06dsolcC\0\x08\x0C\x003",
    );
    /**```solidity
    struct Quorum { StrategyParams[] strategies; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Quorum {
        pub strategies:
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<StrategyParams>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<<StrategyParams as alloy::sol_types::SolType>::RustType>,
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
        impl ::core::convert::From<Quorum> for UnderlyingRustTuple<'_> {
            fn from(value: Quorum) -> Self {
                (value.strategies,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Quorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Quorum {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Quorum {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
        impl alloy_sol_types::SolType for Quorum {
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
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("Quorum(StrategyParams[] strategies)")
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<StrategyParams as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Quorum {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
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
    struct StrategyParams { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
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
        impl alloy_sol_types::SolType for StrategyParams {
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
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
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
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
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
    /**Custom error with signature `InsufficientSignedStake()` and selector `0xe121632f`.
    ```solidity
    error InsufficientSignedStake();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSignedStake {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InsufficientSignedStake> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSignedStake) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSignedStake {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSignedStake {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSignedStake()";
            const SELECTOR: [u8; 4] = [225u8, 33u8, 99u8, 47u8];
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
        }
    };
    /**Custom error with signature `InsufficientWeight()` and selector `0xa8792fd1`.
    ```solidity
    error InsufficientWeight();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWeight {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InsufficientWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWeight()";
            const SELECTOR: [u8; 4] = [168u8, 121u8, 47u8, 209u8];
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
        }
    };
    /**Custom error with signature `InvalidLength()` and selector `0x947d5a84`.
    ```solidity
    error InvalidLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLength {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLength()";
            const SELECTOR: [u8; 4] = [148u8, 125u8, 90u8, 132u8];
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
        }
    };
    /**Custom error with signature `InvalidQuorum()` and selector `0xd1735779`.
    ```solidity
    error InvalidQuorum();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorum {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorum()";
            const SELECTOR: [u8; 4] = [209u8, 115u8, 87u8, 121u8];
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
        }
    };
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
    ```solidity
    error InvalidSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
        }
    };
    /**Custom error with signature `InvalidSignedWeight()` and selector `0x960b41ee`.
    ```solidity
    error InvalidSignedWeight();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignedWeight {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidSignedWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignedWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignedWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignedWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignedWeight()";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 65u8, 238u8];
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
        }
    };
    /**Custom error with signature `InvalidThreshold()` and selector `0xaabd5a09`.
    ```solidity
    error InvalidThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidThreshold {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<InvalidThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidThreshold()";
            const SELECTOR: [u8; 4] = [170u8, 189u8, 90u8, 9u8];
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
        }
    };
    /**Custom error with signature `LengthMismatch()` and selector `0xff633a38`.
    ```solidity
    error LengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LengthMismatch {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<LengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: LengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthMismatch()";
            const SELECTOR: [u8; 4] = [255u8, 99u8, 58u8, 56u8];
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
        }
    };
    /**Custom error with signature `MustUpdateAllOperators()` and selector `0x2d3df6b6`.
    ```solidity
    error MustUpdateAllOperators();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustUpdateAllOperators {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<MustUpdateAllOperators> for UnderlyingRustTuple<'_> {
            fn from(value: MustUpdateAllOperators) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustUpdateAllOperators {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustUpdateAllOperators {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustUpdateAllOperators()";
            const SELECTOR: [u8; 4] = [45u8, 61u8, 246u8, 182u8];
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
        }
    };
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
    ```solidity
    error NotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
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
        }
    };
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
    ```solidity
    error OperatorAlreadyRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<OperatorAlreadyRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
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
        }
    };
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
    ```solidity
    error OperatorNotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
        }
    };
    /**Event with signature `MinimumWeightUpdated(uint256,uint256)` and selector `0x713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f`.
    ```solidity
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumWeightUpdated {
        #[allow(missing_docs)]
        pub _old: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _new: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MinimumWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                    184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                    26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _old: data.0,
                    _new: data.1,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._old,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._new,
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
        impl alloy_sol_types::private::IntoLogData for MinimumWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,address)` and selector `0x31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580`.
    ```solidity
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                    144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8,
                    60u8, 71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
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
                    self._operator.clone(),
                    self._avs.clone(),
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
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
    ```solidity
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                    132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                    113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
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
                    self._operator.clone(),
                    self._avs.clone(),
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
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWeightUpdated(address,uint256,uint256)` and selector `0x88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594`.
    ```solidity
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorWeightUpdated {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8,
                    120u8, 87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8,
                    238u8, 16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    oldWeight: data.0,
                    newWeight: data.1,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newWeight,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone())
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
                    &self._operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumUpdated(((address,uint96)[]),((address,uint96)[]))` and selector `0x23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e`.
    ```solidity
    event QuorumUpdated(Quorum _old, Quorum _new);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumUpdated {
        #[allow(missing_docs)]
        pub _old: <Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _new: <Quorum as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for QuorumUpdated {
            type DataTuple<'a> = (Quorum, Quorum);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                    21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                    137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _old: data.0,
                    _new: data.1,
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
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._old),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._new),
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
        impl alloy_sol_types::private::IntoLogData for QuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ThresholdWeightUpdated(uint256)` and selector `0x9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b`.
    ```solidity
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ThresholdWeightUpdated {
        #[allow(missing_docs)]
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ThresholdWeightUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                    222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8,
                    238u8, 157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _thresholdWeight: data.0,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._thresholdWeight,
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
        impl alloy_sol_types::private::IntoLogData for ThresholdWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ThresholdWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ThresholdWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalWeightUpdated(uint256,uint256)` and selector `0x86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b`.
    ```solidity
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TotalWeightUpdated {
        #[allow(missing_docs)]
        pub oldTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for TotalWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8,
                    48u8, 13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8,
                    23u8, 126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTotalWeight: data.0,
                    newTotalWeight: data.1,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldTotalWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newTotalWeight,
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
        impl alloy_sol_types::private::IntoLogData for TotalWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpdateMinimumWeight(uint256,uint256)` and selector `0x1ea42186b305fa37310450d9fb87ea1e8f0c7f447e771479e3b27634bfe84dc1`.
    ```solidity
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateMinimumWeight {
        #[allow(missing_docs)]
        pub oldMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateMinimumWeight {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8,
                    251u8, 135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8,
                    227u8, 178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldMinimumWeight: data.0,
                    newMinimumWeight: data.1,
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.oldMinimumWeight,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newMinimumWeight,
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
        impl alloy_sol_types::private::IntoLogData for UpdateMinimumWeight {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMinimumWeight> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMinimumWeight) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    /**Function with signature `mockDelegationManager()` and selector `0xb52d472a`.
    ```solidity
    function mockDelegationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mockDelegationManagerCall {}
    ///Container type for the return parameters of the [`mockDelegationManager()`](mockDelegationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mockDelegationManagerReturn {
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
            impl ::core::convert::From<mockDelegationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: mockDelegationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mockDelegationManagerCall {
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
            impl ::core::convert::From<mockDelegationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mockDelegationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mockDelegationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mockDelegationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = mockDelegationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mockDelegationManager()";
            const SELECTOR: [u8; 4] = [181u8, 45u8, 71u8, 42u8];
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
    /**Function with signature `mockServiceManager()` and selector `0x86777e06`.
    ```solidity
    function mockServiceManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mockServiceManagerCall {}
    ///Container type for the return parameters of the [`mockServiceManager()`](mockServiceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mockServiceManagerReturn {
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
            impl ::core::convert::From<mockServiceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: mockServiceManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mockServiceManagerCall {
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
            impl ::core::convert::From<mockServiceManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mockServiceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mockServiceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mockServiceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = mockServiceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mockServiceManager()";
            const SELECTOR: [u8; 4] = [134u8, 119u8, 126u8, 6u8];
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
    /**Function with signature `setUp()` and selector `0x0a9254e4`.
    ```solidity
    function setUp() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpCall {}
    ///Container type for the return parameters of the [`setUp()`](setUpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpReturn {}
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
            impl ::core::convert::From<setUpCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpCall {
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
            impl ::core::convert::From<setUpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUp()";
            const SELECTOR: [u8; 4] = [10u8, 146u8, 84u8, 228u8];
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
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
    ```solidity
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
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
    /**Function with signature `test_FixedStakeUpdates()` and selector `0x707a9241`.
    ```solidity
    function test_FixedStakeUpdates() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_FixedStakeUpdatesCall {}
    ///Container type for the return parameters of the [`test_FixedStakeUpdates()`](test_FixedStakeUpdatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_FixedStakeUpdatesReturn {}
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
            impl ::core::convert::From<test_FixedStakeUpdatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: test_FixedStakeUpdatesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for test_FixedStakeUpdatesCall {
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
            impl ::core::convert::From<test_FixedStakeUpdatesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: test_FixedStakeUpdatesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for test_FixedStakeUpdatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_FixedStakeUpdatesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_FixedStakeUpdatesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_FixedStakeUpdates()";
            const SELECTOR: [u8; 4] = [112u8, 122u8, 146u8, 65u8];
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
    ///Container for all the [`EqualWeightECDSARegistry`](self) function calls.
    pub enum EqualWeightECDSARegistryCalls {
        IS_TEST(IS_TESTCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        mockDelegationManager(mockDelegationManagerCall),
        mockServiceManager(mockServiceManagerCall),
        setUp(setUpCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        test_FixedStakeUpdates(test_FixedStakeUpdatesCall),
    }
    #[automatically_derived]
    impl EqualWeightECDSARegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 146u8, 84u8, 228u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [102u8, 217u8, 169u8, 160u8],
            [112u8, 122u8, 146u8, 65u8],
            [133u8, 34u8, 108u8, 129u8],
            [134u8, 119u8, 126u8, 6u8],
            [145u8, 106u8, 23u8, 198u8],
            [181u8, 45u8, 71u8, 42u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [226u8, 12u8, 159u8, 113u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EqualWeightECDSARegistryCalls {
        const NAME: &'static str = "EqualWeightECDSARegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
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
                Self::mockDelegationManager(_) => {
                    <mockDelegationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mockServiceManager(_) => {
                    <mockServiceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::test_FixedStakeUpdates(_) => {
                    <test_FixedStakeUpdatesCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(EqualWeightECDSARegistryCalls::setUp)
                    }
                    setUp
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn test_FixedStakeUpdates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <test_FixedStakeUpdatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::test_FixedStakeUpdates)
                    }
                    test_FixedStakeUpdates
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn mockServiceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <mockServiceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::mockServiceManager)
                    }
                    mockServiceManager
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn mockDelegationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <mockDelegationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::mockDelegationManager)
                    }
                    mockDelegationManager
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(EqualWeightECDSARegistryCalls::failed)
                    }
                    failed
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryCalls>
                    {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(EqualWeightECDSARegistryCalls::IS_TEST)
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::mockDelegationManager(inner) => {
                    <mockDelegationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mockServiceManager(inner) => {
                    <mockServiceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::test_FixedStakeUpdates(inner) => {
                    <test_FixedStakeUpdatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::mockDelegationManager(inner) => {
                    <mockDelegationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::mockServiceManager(inner) => {
                    <mockServiceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::test_FixedStakeUpdates(inner) => {
                    <test_FixedStakeUpdatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EqualWeightECDSARegistry`](self) custom errors.
    pub enum EqualWeightECDSARegistryErrors {
        InsufficientSignedStake(InsufficientSignedStake),
        InsufficientWeight(InsufficientWeight),
        InvalidLength(InvalidLength),
        InvalidQuorum(InvalidQuorum),
        InvalidSignature(InvalidSignature),
        InvalidSignedWeight(InvalidSignedWeight),
        InvalidThreshold(InvalidThreshold),
        LengthMismatch(LengthMismatch),
        MustUpdateAllOperators(MustUpdateAllOperators),
        NotSorted(NotSorted),
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        OperatorNotRegistered(OperatorNotRegistered),
    }
    #[automatically_derived]
    impl EqualWeightECDSARegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 236u8, 108u8, 31u8],
            [45u8, 61u8, 246u8, 182u8],
            [66u8, 238u8, 104u8, 181u8],
            [139u8, 170u8, 87u8, 159u8],
            [148u8, 125u8, 90u8, 132u8],
            [150u8, 11u8, 65u8, 238u8],
            [168u8, 121u8, 47u8, 209u8],
            [170u8, 189u8, 90u8, 9u8],
            [186u8, 80u8, 249u8, 17u8],
            [209u8, 115u8, 87u8, 121u8],
            [225u8, 33u8, 99u8, 47u8],
            [255u8, 99u8, 58u8, 56u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EqualWeightECDSARegistryErrors {
        const NAME: &'static str = "EqualWeightECDSARegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 12usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InsufficientSignedStake(_) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWeight(_) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLength(_) => <InvalidLength as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidQuorum(_) => <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignedWeight(_) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidThreshold(_) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthMismatch(_) => <LengthMismatch as alloy_sol_types::SolError>::SELECTOR,
                Self::MustUpdateAllOperators(_) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<
                EqualWeightECDSARegistryErrors,
            >] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn MustUpdateAllOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::MustUpdateAllOperators)
                    }
                    MustUpdateAllOperators
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(EqualWeightECDSARegistryErrors::InvalidLength)
                    }
                    InvalidLength
                },
                {
                    fn InvalidSignedWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InvalidSignedWeight as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::InvalidSignedWeight)
                    }
                    InvalidSignedWeight
                },
                {
                    fn InsufficientWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InsufficientWeight as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::InsufficientWeight)
                    }
                    InsufficientWeight
                },
                {
                    fn InvalidThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InvalidThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::InvalidThreshold)
                    }
                    InvalidThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(EqualWeightECDSARegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(EqualWeightECDSARegistryErrors::InvalidQuorum)
                    }
                    InvalidQuorum
                },
                {
                    fn InsufficientSignedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <InsufficientSignedStake as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::InsufficientSignedStake)
                    }
                    InsufficientSignedStake
                },
                {
                    fn LengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EqualWeightECDSARegistryErrors>
                    {
                        <LengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(EqualWeightECDSARegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
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
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`EqualWeightECDSARegistry`](self) events.
    pub enum EqualWeightECDSARegistryEvents {
        MinimumWeightUpdated(MinimumWeightUpdated),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorWeightUpdated(OperatorWeightUpdated),
        QuorumUpdated(QuorumUpdated),
        ThresholdWeightUpdated(ThresholdWeightUpdated),
        TotalWeightUpdated(TotalWeightUpdated),
        UpdateMinimumWeight(UpdateMinimumWeight),
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
    impl EqualWeightECDSARegistryEvents {
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
                30u8, 164u8, 33u8, 134u8, 179u8, 5u8, 250u8, 55u8, 49u8, 4u8, 80u8, 217u8, 251u8,
                135u8, 234u8, 30u8, 143u8, 12u8, 127u8, 68u8, 126u8, 119u8, 20u8, 121u8, 227u8,
                178u8, 118u8, 52u8, 191u8, 232u8, 77u8, 193u8,
            ],
            [
                35u8, 170u8, 212u8, 230u8, 23u8, 68u8, 236u8, 225u8, 100u8, 19u8, 10u8, 164u8,
                21u8, 193u8, 97u8, 110u8, 128u8, 19u8, 107u8, 15u8, 7u8, 112u8, 229u8, 101u8,
                137u8, 67u8, 139u8, 144u8, 178u8, 105u8, 38u8, 94u8,
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
                49u8, 224u8, 173u8, 254u8, 199u8, 27u8, 204u8, 238u8, 55u8, 182u8, 232u8, 58u8,
                144u8, 194u8, 254u8, 219u8, 23u8, 216u8, 241u8, 105u8, 63u8, 238u8, 134u8, 60u8,
                71u8, 113u8, 231u8, 191u8, 226u8, 174u8, 213u8, 128u8,
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
                113u8, 60u8, 165u8, 59u8, 136u8, 214u8, 235u8, 99u8, 245u8, 177u8, 133u8, 76u8,
                184u8, 203u8, 221u8, 115u8, 110u8, 197u8, 30u8, 218u8, 34u8, 94u8, 70u8, 121u8,
                26u8, 169u8, 41u8, 139u8, 1u8, 96u8, 100u8, 143u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8, 71u8,
                177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8, 120u8, 85u8, 214u8,
                126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                134u8, 220u8, 248u8, 107u8, 18u8, 223u8, 238u8, 222u8, 167u8, 74u8, 233u8, 48u8,
                13u8, 189u8, 170u8, 25u8, 59u8, 204u8, 229u8, 128u8, 147u8, 105u8, 200u8, 23u8,
                126u8, 162u8, 244u8, 234u8, 170u8, 101u8, 114u8, 155u8,
            ],
            [
                136u8, 119u8, 13u8, 200u8, 98u8, 228u8, 122u8, 126u8, 213u8, 134u8, 144u8, 120u8,
                87u8, 235u8, 27u8, 117u8, 228u8, 197u8, 255u8, 200u8, 183u8, 7u8, 199u8, 238u8,
                16u8, 235u8, 116u8, 214u8, 136u8, 95u8, 229u8, 148u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8, 237u8,
                155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8, 163u8, 100u8, 124u8,
                33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                147u8, 36u8, 247u8, 229u8, 167u8, 192u8, 40u8, 136u8, 8u8, 166u8, 52u8, 204u8,
                222u8, 68u8, 184u8, 233u8, 121u8, 103u8, 100u8, 116u8, 178u8, 46u8, 41u8, 238u8,
                157u8, 213u8, 105u8, 181u8, 94u8, 121u8, 26u8, 75u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8, 16u8,
                143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8, 67u8, 122u8,
                97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                164u8, 83u8, 219u8, 97u8, 42u8, 245u8, 158u8, 85u8, 33u8, 214u8, 171u8, 146u8,
                132u8, 220u8, 62u8, 45u8, 6u8, 175u8, 40u8, 110u8, 177u8, 177u8, 183u8, 183u8,
                113u8, 252u8, 228u8, 113u8, 108u8, 25u8, 242u8, 193u8,
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
    impl alloy_sol_types::SolEventInterface for EqualWeightECDSARegistryEvents {
        const NAME: &'static str = "EqualWeightECDSARegistryEvents";
        const COUNT: usize = 30usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::MinimumWeightUpdated)
                }
                Some(<OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDeregistered)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorWeightUpdated)
                }
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumUpdated)
                }
                Some(<ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ThresholdWeightUpdated)
                }
                Some(<TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TotalWeightUpdated)
                }
                Some(<UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::UpdateMinimumWeight)
                }
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
    impl alloy_sol_types::private::IntoLogData for EqualWeightECDSARegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
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
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
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
    /**Creates a new wrapper around an on-chain [`EqualWeightECDSARegistry`](self) contract instance.

    See the [wrapper's documentation](`EqualWeightECDSARegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EqualWeightECDSARegistryInstance<T, P, N> {
        EqualWeightECDSARegistryInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<EqualWeightECDSARegistryInstance<T, P, N>>,
    > {
        EqualWeightECDSARegistryInstance::<T, P, N>::deploy(provider)
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
        EqualWeightECDSARegistryInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`EqualWeightECDSARegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`EqualWeightECDSARegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EqualWeightECDSARegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EqualWeightECDSARegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EqualWeightECDSARegistryInstance")
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
        > EqualWeightECDSARegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`EqualWeightECDSARegistry`](self) contract instance.

        See the [wrapper's documentation](`EqualWeightECDSARegistryInstance`) for more details.*/
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
        ) -> alloy_contract::Result<EqualWeightECDSARegistryInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> EqualWeightECDSARegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EqualWeightECDSARegistryInstance<T, P, N> {
            EqualWeightECDSARegistryInstance {
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
        > EqualWeightECDSARegistryInstance<T, P, N>
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
        ///Creates a new call builder for the [`mockDelegationManager`] function.
        pub fn mockDelegationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, mockDelegationManagerCall, N> {
            self.call_builder(&mockDelegationManagerCall {})
        }
        ///Creates a new call builder for the [`mockServiceManager`] function.
        pub fn mockServiceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, mockServiceManagerCall, N> {
            self.call_builder(&mockServiceManagerCall {})
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<T, &P, setUpCall, N> {
            self.call_builder(&setUpCall {})
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
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall {})
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
        ///Creates a new call builder for the [`test_FixedStakeUpdates`] function.
        pub fn test_FixedStakeUpdates(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, test_FixedStakeUpdatesCall, N> {
            self.call_builder(&test_FixedStakeUpdatesCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > EqualWeightECDSARegistryInstance<T, P, N>
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
        ///Creates a new event filter for the [`MinimumWeightUpdated`] event.
        pub fn MinimumWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumWeightUpdated, N> {
            self.event_filter::<MinimumWeightUpdated>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorWeightUpdated`] event.
        pub fn OperatorWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWeightUpdated, N> {
            self.event_filter::<OperatorWeightUpdated>()
        }
        ///Creates a new event filter for the [`QuorumUpdated`] event.
        pub fn QuorumUpdated_filter(&self) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`ThresholdWeightUpdated`] event.
        pub fn ThresholdWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ThresholdWeightUpdated, N> {
            self.event_filter::<ThresholdWeightUpdated>()
        }
        ///Creates a new event filter for the [`TotalWeightUpdated`] event.
        pub fn TotalWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalWeightUpdated, N> {
            self.event_filter::<TotalWeightUpdated>()
        }
        ///Creates a new event filter for the [`UpdateMinimumWeight`] event.
        pub fn UpdateMinimumWeight_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpdateMinimumWeight, N> {
            self.event_filter::<UpdateMinimumWeight>()
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
