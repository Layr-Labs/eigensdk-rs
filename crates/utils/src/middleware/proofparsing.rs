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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
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
    function getBalanceRoot() external view returns (bytes32);
    function getBalanceUpdateSlotProof() external returns (bytes32[] memory);
    function getBeaconStateRoot() external view returns (bytes32);
    function getBlockHeaderProof() external returns (bytes32[18] memory);
    function getBlockRoot() external view returns (bytes32);
    function getBlockRootIndex() external view returns (uint256);
    function getExecutionPayloadProof() external returns (bytes32[7] memory);
    function getExecutionPayloadRoot() external view returns (bytes32);
    function getHistoricalSummaryIndex() external view returns (uint256);
    function getHistoricalSummaryProof() external returns (bytes32[44] memory);
    function getLatestBlockRoot() external view returns (bytes32);
    function getSlot() external view returns (uint256);
    function getSlotProof() external returns (bytes32[3] memory);
    function getSlotRoot() external view returns (bytes32);
    function getStateRootProof() external returns (bytes32[] memory);
    function getTimestampProof() external returns (bytes32[4] memory);
    function getTimestampRoot() external view returns (bytes32);
    function getValidatorBalanceProof() external returns (bytes32[] memory);
    function getValidatorFields() external returns (bytes32[] memory);
    function getValidatorFieldsProof() external returns (bytes32[] memory);
    function getValidatorIndex() external view returns (uint256);
    function getValidatorProof() external returns (bytes32[46] memory);
    function getValidatorPubkeyHash() external view returns (bytes32);
    function getWithdrawalCredentialProof() external returns (bytes32[] memory);
    function getWithdrawalFields() external returns (bytes32[] memory);
    function getWithdrawalIndex() external view returns (uint256);
    function getWithdrawalProof() external returns (bytes32[9] memory);
    function setJSON(string memory path) external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
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
    "name": "getBalanceRoot",
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
    "name": "getBalanceUpdateSlotProof",
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
    "name": "getBeaconStateRoot",
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
    "stateMutability": "view"
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
    "stateMutability": "view"
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
    "stateMutability": "view"
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
    "stateMutability": "view"
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
    "stateMutability": "view"
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
    "stateMutability": "view"
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStateRootProof",
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
    "name": "getTimestampProof",
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
    "name": "getTimestampRoot",
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
    "name": "getValidatorBalanceProof",
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
    "stateMutability": "view"
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getWithdrawalCredentialProof",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getWithdrawalProof",
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
    ///0x608060405260078054600160ff199182168117909255600b8054909116909117905534801561002d57600080fd5b5061345e8061003d6000396000f3fe608060405234801561001057600080fd5b506004361061023d5760003560e01c8063765aa6061161013b578063b5508aa9116100b8578063d944472f1161007c578063d944472f14610411578063db364a4014610419578063e20c9f711461042e578063f148082c14610436578063fa7626d41461043e57600080fd5b8063b5508aa9146103cc578063ba414fa6146103d4578063c3da8ae9146103ec578063d6461cbb146103f4578063d911b683146103fc57600080fd5b80639de4a9b3116100ff5780639de4a9b31461037d578063a507742914610392578063a6b7a848146103a7578063ab72161c146103af578063b38061bf146103c457600080fd5b8063765aa6061461034857806385226c8114610350578063893893ca14610365578063916a17c61461036d578063950bb6821461037557600080fd5b806334e3d90e116101c95780634c20f5101161018d5780634c20f510146103065780634c38f9131461030e57806364f38ed71461032357806366d9a9a01461032b5780636c877c841461034057600080fd5b806334e3d90e146102de5780633e5e3c23146102e65780633f7286f4146102ee57806342864734146102f65780634656fdb5146102fe57600080fd5b80631ed7831c116102105780631ed7831c14610282578063275378b1146102975780632872e20c1461029f5780632ade3880146102b45780632fd1793c146102c957600080fd5b806308a4d71f1461024257806316f0715314610257578063177541fc1461027257806318aadf311461027a575b600080fd5b610255610250366004612aba565b61044b565b005b61025f6104d9565b6040519081526020015b60405180910390f35b61025f610595565b61025f610653565b61028a610717565b6040516102699190612b3a565b61025f610779565b6102a7610837565b6040516102699190612b87565b6102bc610a89565b6040516102699190612c15565b6102d1610bcb565b6040516102699190612cd5565b6102d1610d09565b61028a610e3e565b61028a610e9e565b61025f610efe565b61025f610fc1565b6102d161107b565b6103166111b3565b6040516102699190612d0d565b61025f6112f2565b6103336113b0565b6040516102699190612d36565b61025f611496565b61025f611549565b610358611606565b6040516102699190612de9565b6102d16116d6565b61033361180e565b6102d16118f4565b610385611a2a565b6040516102699190612e4b565b61039a611b68565b6040516102699190612e73565b6102d1611ca7565b6103b7611ddf565b6040516102699190612e9c565b61025f611f1e565b610358611fda565b6103dc6120aa565b6040519015158152602001610269565b6102d16121cb565b61025f612301565b6104046123cc565b6040516102699190612ec5565b61025f61250a565b6104216125cd565b6040516102699190612eed565b61028a61270b565b61025f61276b565b6007546103dc9060ff1681565b6040516360f9bb1160e01b8152600080516020613409833981519152906360f9bb119061047c908490600401612f15565b600060405180830381865afa158015610499573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104c19190810190612f28565b80516104d591601c916020909101906128dc565b5050565b6000610590601c80546104eb90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461051790612f9f565b80156105645780601f1061053957610100808354040283529160200191610564565b820191906000526020600020905b81548152906001019060200180831161054757829003601f168201915b5050505050604051806040016040528060098152602001680b9cdb1bdd149bdbdd60ba1b815250612828565b905090565b6000610590601c80546105a790612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546105d390612f9f565b80156106205780601f106105f557610100808354040283529160200191610620565b820191906000526020600020905b81548152906001019060200180831161060357829003601f168201915b50505050506040518060400160405280601081526020016f0b98995858dbdb94dd185d19549bdbdd60821b815250612828565b6000610590601c805461066590612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461069190612f9f565b80156106de5780601f106106b3576101008083540402835291602001916106de565b820191906000526020600020905b8154815290600101906020018083116106c157829003601f168201915b5050505050604051806040016040528060168152602001750b9b185d195cdd109b1bd8dad2195859195c949bdbdd60521b815250612828565b6060601480548060200260200160405190810160405280929190818152602001828054801561076f57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610751575b5050505050905090565b6000610590601c805461078b90612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546107b790612f9f565b80156108045780601f106107d957610100808354040283529160200191610804565b820191906000526020600020905b8154815290600101906020018083116107e757829003601f168201915b50505050506040518060400160405280601081526020016f0b989b1bd8dad2195859195c949bdbdd60821b815250612828565b61083f612960565b60005b602c811015610a545760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015610895573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526108bd9190810190612f28565b6040516020016108cd9190612fd4565b60408051601f19818403018152908290526108ea91602001612ff9565b604051602081830303815290604052601d908051906020019061090e9291906128dc565b50610a2d601c805461091f90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461094b90612f9f565b80156109985780601f1061096d57610100808354040283529160200191610998565b820191906000526020600020905b81548152906001019060200180831161097b57829003601f168201915b5050505050601d80546109aa90612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546109d690612f9f565b8015610a235780601f106109f857610100808354040283529160200191610a23565b820191906000526020600020905b815481529060010190602001808311610a0657829003601f168201915b5050505050612828565b606a82602c8110610a4057610a4061303e565b015580610a4c81613054565b915050610842565b506040805161058081019182905290606a90602c9082845b815481526020019060010190808311610a6c575050505050905090565b6060601b805480602002602001604051908101604052809291908181526020016000905b82821015610bc257600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610bab578382906000526020600020018054610b1e90612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054610b4a90612f9f565b8015610b975780601f10610b6c57610100808354040283529160200191610b97565b820191906000526020600020905b815481529060010190602001808311610b7a57829003601f168201915b505050505081526020019060010190610aff565b505050508152505081526020019060010190610aad565b50505050905090565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015610c4a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c729190810190612f28565b604051602001610c829190612fd4565b60408051601f1981840301815290829052610c9f9160200161307d565b604051602081830303815290604052601d9080519060200190610cc39291906128dc565b50610cd4601c805461091f90612f9f565b828281518110610ce657610ce661303e565b602090810291909101015280610cfb81613054565b915050610bf7565b50919050565b604080516003808252608082019092526060916000919060208201848036833701905050905060005b6003811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015610d85573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610dad9190810190612f28565b604051602001610dbd9190612fd4565b60408051601f1981840301815290829052610dda916020016130c2565b604051602081830303815290604052601d9080519060200190610dfe9291906128dc565b50610e0f601c805461091f90612f9f565b828281518110610e2157610e2161303e565b602090810291909101015280610e3681613054565b915050610d32565b6060601680548060200260200160405190810160405280929190818152602001828054801561076f576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610751575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561076f576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610751575050505050905090565b6000610590601c8054610f1090612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3c90612f9f565b8015610f895780601f10610f5e57610100808354040283529160200191610f89565b820191906000526020600020905b815481529060010190602001808311610f6c57829003601f168201915b50505050506040518060400160405280601581526020017405cc4d8dec6d690cac2c8cae4a4dedee892dcc8caf605b1b8152506128a6565b6000610590601c8054610fd390612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054610fff90612f9f565b801561104c5780601f106110215761010080835404028352916020019161104c565b820191906000526020600020905b81548152906001019060200180831161102f57829003601f168201915b50505050506040518060400160405280600c81526020016b0b98985b185b98d9549bdbdd60a21b815250612828565b604080516008808252610120820190925260609160009190602082016101008036833701905050905060005b6008811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa1580156110fa573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111229190810190612f28565b6040516020016111329190612fd4565b60408051601f198184030181529082905261114f91602001613118565b604051602081830303815290604052601d90805190602001906111739291906128dc565b50611184601c805461091f90612f9f565b8282815181106111965761119661303e565b6020908102919091010152806111ab81613054565b9150506110a7565b6111bb61297f565b60005b602e8110156112c25760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611211573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526112399190810190612f28565b6040516020016112499190612fd4565b60408051601f198184030181529082905261126691602001613151565b604051602081830303815290604052601d908051906020019061128a9291906128dc565b5061129b601c805461091f90612f9f565b603c82602e81106112ae576112ae61303e565b0155806112ba81613054565b9150506111be565b50604080516105c0810191829052603c805482529091602e90603d60208501808311610a6c575050505050905090565b6000610590601c805461130490612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461133090612f9f565b801561137d5780601f106113525761010080835404028352916020019161137d565b820191906000526020600020905b81548152906001019060200180831161136057829003601f168201915b50505050506040518060400160405280601081526020016f05ceed2e8d0c8e4c2eec2d892dcc8caf60831b8152506128a6565b60606019805480602002602001604051908101604052809291908181526020016000905b82821015610bc25760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561147e57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114405790505b505050505081525050815260200190600101906113d4565b6000610590601c80546114a890612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546114d490612f9f565b80156115215780601f106114f657610100808354040283529160200191611521565b820191906000526020600020905b81548152906001019060200180831161150457829003601f168201915b5050505050604051806040016040528060058152602001640b9cdb1bdd60da1b8152506128a6565b6000610590601c805461155b90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461158790612f9f565b80156115d45780601f106115a9576101008083540402835291602001916115d4565b820191906000526020600020905b8154815290600101906020018083116115b757829003601f168201915b50505050506040518060400160405280600f81526020016e05cecc2d8d2c8c2e8dee492dcc8caf608b1b8152506128a6565b60606018805480602002602001604051908101604052809291908181526020016000905b82821015610bc257838290600052602060002001805461164990612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461167590612f9f565b80156116c25780601f10611697576101008083540402835291602001916116c2565b820191906000526020600020905b8154815290600101906020018083116116a557829003601f168201915b50505050508152602001906001019061162a565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611755573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261177d9190810190612f28565b60405160200161178d9190612fd4565b60408051601f19818403018152908290526117aa91602001613189565b604051602081830303815290604052601d90805190602001906117ce9291906128dc565b506117df601c805461091f90612f9f565b8282815181106117f1576117f161303e565b60209081029190910101528061180681613054565b915050611702565b6060601a805480602002602001604051908101604052809291908181526020016000905b82821015610bc25760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156118dc57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161189e5790505b50505050508152505081526020019060010190611832565b60408051600480825260a08201909252606091600091906020820160808036833701905050905060005b6004811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611971573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119999190810190612f28565b6040516020016119a99190612fd4565b60408051601f19818403018152908290526119c6916020016131c7565b604051602081830303815290604052601d90805190602001906119ea9291906128dc565b506119fb601c805461091f90612f9f565b828281518110611a0d57611a0d61303e565b602090810291909101015280611a2281613054565b91505061191e565b611a3261299e565b60005b6003811015611b395760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611a88573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611ab09190810190612f28565b604051602001611ac09190612fd4565b60408051601f1981840301815290829052611add91602001613201565b604051602081830303815290604052601d9080519060200190611b019291906128dc565b50611b12601c805461091f90612f9f565b60308260038110611b2557611b2561303e565b015580611b3181613054565b915050611a35565b506040805160608101918290526030805482529091600390603160208501808311610a6c575050505050905090565b611b706129bc565b60005b6012811015611c775760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611bc6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611bee9190810190612f28565b604051602001611bfe9190612fd4565b60408051601f1981840301815290829052611c1b91602001613234565b604051602081830303815290604052601d9080519060200190611c3f9291906128dc565b50611c50601c805461091f90612f9f565b601e8260128110611c6357611c6361303e565b015580611c6f81613054565b915050611b73565b5060408051610240810191829052601e805482529091601290601f60208501808311610a6c575050505050905090565b60408051602c8082526105a0820190925260609160009190602082016105808036833701905050905060005b602c811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611d26573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d4e9190810190612f28565b604051602001611d5e9190612fd4565b60408051601f1981840301815290829052611d7b91602001613261565b604051602081830303815290604052601d9080519060200190611d9f9291906128dc565b50611db0601c805461091f90612f9f565b828281518110611dc257611dc261303e565b602090810291909101015280611dd781613054565b915050611cd3565b611de76129db565b60005b6009811015611eee5760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611e3d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611e659190810190612f28565b604051602001611e759190612fd4565b60408051601f1981840301815290829052611e92916020016132a6565b604051602081830303815290604052601d9080519060200190611eb69291906128dc565b50611ec7601c805461091f90612f9f565b60338260098110611eda57611eda61303e565b015580611ee681613054565b915050611dea565b50604080516101208101918290526033805482529091600990603460208501808311610a6c575050505050905090565b6000610590601c8054611f3090612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054611f5c90612f9f565b8015611fa95780601f10611f7e57610100808354040283529160200191611fa9565b820191906000526020600020905b815481529060010190602001808311611f8c57829003601f168201915b50505050506040518060400160405280600e81526020016d0b9d1a5b595cdd185b5c149bdbdd60921b815250612828565b60606017805480602002602001604051908101604052809291908181526020016000905b82821015610bc257838290600052602060002001805461201d90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461204990612f9f565b80156120965780601f1061206b57610100808354040283529160200191612096565b820191906000526020600020905b81548152906001019060200180831161207957829003601f168201915b505050505081526020019060010190611ffe565b600754600090610100900460ff16156120cc5750600754610100900460ff1690565b60006000805160206134098339815191523b156121c65760408051600080516020613409833981519152602082018190526519985a5b195960d21b8284015282518083038401815260608301909352600092909161214e917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016132d2565b60408051601f198184030181529082905261216891613303565b6000604051808303816000865af19150503d80600081146121a5576040519150601f19603f3d011682016040523d82523d6000602084013e6121aa565b606091505b50915050808060200190518101906121c2919061331f565b9150505b919050565b60408051600580825260c08201909252606091600091906020820160a08036833701905050905060005b6005811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015612248573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526122709190810190612f28565b6040516020016122809190612fd4565b60408051601f198184030181529082905261229d91602001613341565b604051602081830303815290604052601d90805190602001906122c19291906128dc565b506122d2601c805461091f90612f9f565b8282815181106122e4576122e461303e565b6020908102919091010152806122f981613054565b9150506121f5565b6000610590601c805461231390612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461233f90612f9f565b801561238c5780601f106123615761010080835404028352916020019161238c565b820191906000526020600020905b81548152906001019060200180831161236f57829003601f168201915b50505050506040518060400160405280601781526020017f2e686973746f726963616c53756d6d617279496e6465780000000000000000008152506128a6565b6123d46129fa565b60005b60048110156124db5760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa15801561242a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526124529190810190612f28565b6040516020016124629190612fd4565b60408051601f198184030181529082905261247f91602001613367565b604051602081830303815290604052601d90805190602001906124a39291906128dc565b506124b4601c805461091f90612f9f565b609d82600481106124c7576124c761303e565b0155806124d381613054565b9150506123d7565b50604080516080810191829052609d805482529091600490609e60208501808311610a6c575050505050905090565b6000610590601c805461251c90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461254890612f9f565b80156125955780601f1061256a57610100808354040283529160200191612595565b820191906000526020600020905b81548152906001019060200180831161257857829003601f168201915b5050505050604051806040016040528060158152602001740b995e1958dd5d1a5bdb94185e5b1bd859149bdbdd605a1b815250612828565b6125d5612a18565b60005b60078110156126dc5760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa15801561262b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526126539190810190612f28565b6040516020016126639190612fd4565b60408051601f198184030181529082905261268091602001613392565b604051602081830303815290604052601d90805190602001906126a49291906128dc565b506126b5601c805461091f90612f9f565b609682600781106126c8576126c861303e565b0155806126d481613054565b9150506125d8565b506040805160e08101918290526096805482529091600790609760208501808311610a6c575050505050905090565b6060601380548060200260200160405190810160405280929190818152602001828054801561076f576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610751575050505050905090565b6000610590601c805461277d90612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546127a990612f9f565b80156127f65780601f106127cb576101008083540402835291602001916127f6565b820191906000526020600020905b8154815290600101906020018083116127d957829003601f168201915b5050505050604051806040016040528060138152602001722e56616c696461746f724669656c64735b305d60681b8152505b604051631777e59d60e01b815260009060008051602061340983398151915290631777e59d9061285e90869086906004016133ca565b602060405180830381865afa15801561287b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061289f91906133ef565b9392505050565b6040516356eef15b60e11b81526000906000805160206134098339815191529063addde2b69061285e90869086906004016133ca565b8280546128e890612f9f565b90600052602060002090601f01602090048101928261290a5760008555612950565b82601f1061292357805160ff1916838001178555612950565b82800160010185558215612950579182015b82811115612950578251825591602001919060010190612935565b5061295c929150612a36565b5090565b604051806105800160405280602c906020820280368337509192915050565b604051806105c00160405280602e906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b6040518061024001604052806012906020820280368337509192915050565b6040518061012001604052806009906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060e001604052806007906020820280368337509192915050565b5b8082111561295c5760008155600101612a37565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612a8a57612a8a612a4b565b604052919050565b600067ffffffffffffffff821115612aac57612aac612a4b565b50601f01601f191660200190565b600060208284031215612acc57600080fd5b813567ffffffffffffffff811115612ae357600080fd5b8201601f81018413612af457600080fd5b8035612b07612b0282612a92565b612a61565b818152856020838501011115612b1c57600080fd5b81602084016020830137600091810160200191909152949350505050565b6020808252825182820181905260009190848201906040850190845b81811015612b7b5783516001600160a01b031683529284019291840191600101612b56565b50909695505050505050565b6105808101818360005b602c811015612bb0578151835260209283019290910190600101612b91565b50505092915050565b60005b83811015612bd4578181015183820152602001612bbc565b83811115612be3576000848401525b50505050565b60008151808452612c01816020860160208601612bb9565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b85811015612cc557603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b81811015612caf57605f19898503018352612c9d848651612be9565b948e01949350918d0191600101612c81565b505050978a019794505091880191600101612c3c565b50919a9950505050505050505050565b6020808252825182820181905260009190848201906040850190845b81811015612b7b57835183529284019291840191600101612cf1565b6105c08101818360005b602e811015612bb0578151835260209283019290910190600101612d17565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015612dda57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015612dc55783516001600160e01b0319168252928b019260019290920191908b0190612d9b565b50978a01979550505091870191600101612d5e565b50919998505050505050505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015612e3e57603f19888603018452612e2c858351612be9565b94509285019290850190600101612e10565b5092979650505050505050565b60608101818360005b6003811015612bb0578151835260209283019290910190600101612e54565b6102408101818360005b6012811015612bb0578151835260209283019290910190600101612e7d565b6101208101818360005b6009811015612bb0578151835260209283019290910190600101612ea6565b60808101818360005b6004811015612bb0578151835260209283019290910190600101612ece565b60e08101818360005b6007811015612bb0578151835260209283019290910190600101612ef6565b60208152600061289f6020830184612be9565b600060208284031215612f3a57600080fd5b815167ffffffffffffffff811115612f5157600080fd5b8201601f81018413612f6257600080fd5b8051612f70612b0282612a92565b818152856020838501011115612f8557600080fd5b612f96826020830160208601612bb9565b95945050505050565b600181811c90821680612fb357607f821691505b60208210811415610d0357634e487b7160e01b600052602260045260246000fd5b60008251612fe6818460208701612bb9565b605d60f81b920191825250600101919050565b7f2e486973746f726963616c53756d6d61727950726f6f665b0000000000000000815260008251613031816018850160208701612bb9565b9190910160180192915050565b634e487b7160e01b600052603260045260246000fd5b600060001982141561307657634e487b7160e01b600052601160045260246000fd5b5060010190565b7f2e5769746864726177616c43726564656e7469616c50726f6f665b00000000008152600082516130b581601b850160208701612bb9565b91909101601b0192915050565b7f2e5374617465526f6f74416761696e73744c6174657374426c6f636b48656164815267657250726f6f665b60c01b60208201526000825161310b816028850160208701612bb9565b9190910160280192915050565b702e56616c696461746f724669656c64735b60781b815260008251613144816011850160208701612bb9565b9190910160110192915050565b6f2e56616c696461746f7250726f6f665b60801b81526000825161317c816010850160208701612bb9565b9190910160100192915050565b752e56616c696461746f724669656c647350726f6f665b60501b8152600082516131ba816016850160208701612bb9565b9190910160160192915050565b712e5769746864726177616c4669656c64735b60701b8152600082516131f4816012850160208701612bb9565b9190910160120192915050565b6a2e536c6f7450726f6f665b60a81b81526000825161322781600b850160208701612bb9565b91909101600b0192915050565b712e426c6f636b48656164657250726f6f665b60701b8152600082516131f4816012850160208701612bb9565b7f2e56616c696461746f7242616c616e636550726f6f665b000000000000000000815260008251613299816017850160208701612bb9565b9190910160170192915050565b702e5769746864726177616c50726f6f665b60781b815260008251613144816011850160208701612bb9565b6001600160e01b03198316815281516000906132f5816004850160208701612bb9565b919091016004019392505050565b60008251613315818460208701612bb9565b9190910192915050565b60006020828403121561333157600080fd5b8151801515811461289f57600080fd5b6a2e736c6f7450726f6f665b60a81b81526000825161322781600b850160208701612bb9565b6f2e54696d657374616d7050726f6f665b60801b81526000825161317c816010850160208701612bb9565b7f2e457865637574696f6e5061796c6f616450726f6f665b000000000000000000815260008251613299816017850160208701612bb9565b6040815260006133dd6040830185612be9565b8281036020840152612f968185612be9565b60006020828403121561340157600080fd5b505191905056fe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da264697066735822122057ae67fee4fb3d1c6d71cd6b6dd37596486c5ab1b5fdae13d9517067b5b9d15c64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x07\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0B\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa4^\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80cvZ\xA6\x06\x11a\x01;W\x80c\xB5P\x8A\xA9\x11a\0\xB8W\x80c\xD9DG/\x11a\0|W\x80c\xD9DG/\x14a\x04\x11W\x80c\xDB6J@\x14a\x04\x19W\x80c\xE2\x0C\x9Fq\x14a\x04.W\x80c\xF1H\x08,\x14a\x046W\x80c\xFAv&\xD4\x14a\x04>W`\0\x80\xFD[\x80c\xB5P\x8A\xA9\x14a\x03\xCCW\x80c\xBAAO\xA6\x14a\x03\xD4W\x80c\xC3\xDA\x8A\xE9\x14a\x03\xECW\x80c\xD6F\x1C\xBB\x14a\x03\xF4W\x80c\xD9\x11\xB6\x83\x14a\x03\xFCW`\0\x80\xFD[\x80c\x9D\xE4\xA9\xB3\x11a\0\xFFW\x80c\x9D\xE4\xA9\xB3\x14a\x03}W\x80c\xA5\x07t)\x14a\x03\x92W\x80c\xA6\xB7\xA8H\x14a\x03\xA7W\x80c\xABr\x16\x1C\x14a\x03\xAFW\x80c\xB3\x80a\xBF\x14a\x03\xC4W`\0\x80\xFD[\x80cvZ\xA6\x06\x14a\x03HW\x80c\x85\"l\x81\x14a\x03PW\x80c\x898\x93\xCA\x14a\x03eW\x80c\x91j\x17\xC6\x14a\x03mW\x80c\x95\x0B\xB6\x82\x14a\x03uW`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x11a\x01\xC9W\x80cL \xF5\x10\x11a\x01\x8DW\x80cL \xF5\x10\x14a\x03\x06W\x80cL8\xF9\x13\x14a\x03\x0EW\x80cd\xF3\x8E\xD7\x14a\x03#W\x80cf\xD9\xA9\xA0\x14a\x03+W\x80cl\x87|\x84\x14a\x03@W`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x14a\x02\xDEW\x80c>^<#\x14a\x02\xE6W\x80c?r\x86\xF4\x14a\x02\xEEW\x80cB\x86G4\x14a\x02\xF6W\x80cFV\xFD\xB5\x14a\x02\xFEW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x02\x10W\x80c\x1E\xD7\x83\x1C\x14a\x02\x82W\x80c'Sx\xB1\x14a\x02\x97W\x80c(r\xE2\x0C\x14a\x02\x9FW\x80c*\xDE8\x80\x14a\x02\xB4W\x80c/\xD1y<\x14a\x02\xC9W`\0\x80\xFD[\x80c\x08\xA4\xD7\x1F\x14a\x02BW\x80c\x16\xF0qS\x14a\x02WW\x80c\x17uA\xFC\x14a\x02rW\x80c\x18\xAA\xDF1\x14a\x02zW[`\0\x80\xFD[a\x02Ua\x02P6`\x04a*\xBAV[a\x04KV[\0[a\x02_a\x04\xD9V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02_a\x05\x95V[a\x02_a\x06SV[a\x02\x8Aa\x07\x17V[`@Qa\x02i\x91\x90a+:V[a\x02_a\x07yV[a\x02\xA7a\x087V[`@Qa\x02i\x91\x90a+\x87V[a\x02\xBCa\n\x89V[`@Qa\x02i\x91\x90a,\x15V[a\x02\xD1a\x0B\xCBV[`@Qa\x02i\x91\x90a,\xD5V[a\x02\xD1a\r\tV[a\x02\x8Aa\x0E>V[a\x02\x8Aa\x0E\x9EV[a\x02_a\x0E\xFEV[a\x02_a\x0F\xC1V[a\x02\xD1a\x10{V[a\x03\x16a\x11\xB3V[`@Qa\x02i\x91\x90a-\rV[a\x02_a\x12\xF2V[a\x033a\x13\xB0V[`@Qa\x02i\x91\x90a-6V[a\x02_a\x14\x96V[a\x02_a\x15IV[a\x03Xa\x16\x06V[`@Qa\x02i\x91\x90a-\xE9V[a\x02\xD1a\x16\xD6V[a\x033a\x18\x0EV[a\x02\xD1a\x18\xF4V[a\x03\x85a\x1A*V[`@Qa\x02i\x91\x90a.KV[a\x03\x9Aa\x1BhV[`@Qa\x02i\x91\x90a.sV[a\x02\xD1a\x1C\xA7V[a\x03\xB7a\x1D\xDFV[`@Qa\x02i\x91\x90a.\x9CV[a\x02_a\x1F\x1EV[a\x03Xa\x1F\xDAV[a\x03\xDCa \xAAV[`@Q\x90\x15\x15\x81R` \x01a\x02iV[a\x02\xD1a!\xCBV[a\x02_a#\x01V[a\x04\x04a#\xCCV[`@Qa\x02i\x91\x90a.\xC5V[a\x02_a%\nV[a\x04!a%\xCDV[`@Qa\x02i\x91\x90a.\xEDV[a\x02\x8Aa'\x0BV[a\x02_a'kV[`\x07Ta\x03\xDC\x90`\xFF\x16\x81V[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R`\0\x80Q` a4\t\x839\x81Q\x91R\x90c`\xF9\xBB\x11\x90a\x04|\x90\x84\x90`\x04\x01a/\x15V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xC1\x91\x90\x81\x01\x90a/(V[\x80Qa\x04\xD5\x91`\x1C\x91` \x90\x91\x01\x90a(\xDCV[PPV[`\0a\x05\x90`\x1C\x80Ta\x04\xEB\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x17\x90a/\x9FV[\x80\x15a\x05dW\x80`\x1F\x10a\x059Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x0B\x9C\xDB\x1B\xDD\x14\x9B\xDB\xDD`\xBA\x1B\x81RPa((V[\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x05\xA7\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD3\x90a/\x9FV[\x80\x15a\x06 W\x80`\x1F\x10a\x05\xF5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06 V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x99XX\xDB\xDB\x94\xDD\x18]\x19T\x9B\xDB\xDD`\x82\x1B\x81RPa((V[`\0a\x05\x90`\x1C\x80Ta\x06e\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x91\x90a/\x9FV[\x80\x15a\x06\xDEW\x80`\x1F\x10a\x06\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u\x0B\x9B\x18]\x19\\\xDD\x10\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`R\x1B\x81RPa((V[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QW[PPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x07\x8B\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xB7\x90a/\x9FV[\x80\x15a\x08\x04W\x80`\x1F\x10a\x07\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`\x82\x1B\x81RPa((V[a\x08?a)`V[`\0[`,\x81\x10\x15a\nTW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xBD\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x08\xCD\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x08\xEA\x91` \x01a/\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\t\x0E\x92\x91\x90a(\xDCV[Pa\n-`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tK\x90a/\x9FV[\x80\x15a\t\x98W\x80`\x1F\x10a\tmWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x98V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t{W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\x1D\x80Ta\t\xAA\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xD6\x90a/\x9FV[\x80\x15a\n#W\x80`\x1F\x10a\t\xF8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n#V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa((V[`j\x82`,\x81\x10a\n@Wa\n@a0>V[\x01U\x80a\nL\x81a0TV[\x91PPa\x08BV[P`@\x80Qa\x05\x80\x81\x01\x91\x82\x90R\x90`j\x90`,\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\nlWPPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x0B\xABW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0B\x1E\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0BJ\x90a/\x9FV[\x80\x15a\x0B\x97W\x80`\x1F\x10a\x0BlWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BzW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\xFFV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\xADV[PPPP\x90P\x90V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Cr\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x0C\x82\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\x9F\x91` \x01a0}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x0C\xC3\x92\x91\x90a(\xDCV[Pa\x0C\xD4`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x0C\xE6Wa\x0C\xE6a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0C\xFB\x81a0TV[\x91PPa\x0B\xF7V[P\x91\x90PV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01\x84\x806\x837\x01\x90PP\x90P`\0[`\x03\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xAD\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\r\xBD\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xDA\x91` \x01a0\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\r\xFE\x92\x91\x90a(\xDCV[Pa\x0E\x0F`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x0E!Wa\x0E!a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0E6\x81a0TV[\x91PPa\r2V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QWPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x0F\x10\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F<\x90a/\x9FV[\x80\x15a\x0F\x89W\x80`\x1F\x10a\x0F^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FlW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x05\xCCM\x8D\xECmi\x0C\xAC,\x8C\xAEJM\xED\xEE\x89-\xCC\x8C\xAF`[\x1B\x81RPa(\xA6V[`\0a\x05\x90`\x1C\x80Ta\x0F\xD3\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xFF\x90a/\x9FV[\x80\x15a\x10LW\x80`\x1F\x10a\x10!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10LV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x0B\x98\x98[\x18[\x98\xD9T\x9B\xDB\xDD`\xA2\x1B\x81RPa((V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P`\0[`\x08\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\"\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x112\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11O\x91` \x01a1\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x11s\x92\x91\x90a(\xDCV[Pa\x11\x84`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x11\x96Wa\x11\x96a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x11\xAB\x81a0TV[\x91PPa\x10\xA7V[a\x11\xBBa)\x7FV[`\0[`.\x81\x10\x15a\x12\xC2W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x129\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x12I\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x12f\x91` \x01a1QV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x12\x8A\x92\x91\x90a(\xDCV[Pa\x12\x9B`\x1C\x80Ta\t\x1F\x90a/\x9FV[`<\x82`.\x81\x10a\x12\xAEWa\x12\xAEa0>V[\x01U\x80a\x12\xBA\x81a0TV[\x91PPa\x11\xBEV[P`@\x80Qa\x05\xC0\x81\x01\x91\x82\x90R`<\x80T\x82R\x90\x91`.\x90`=` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x13\x04\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x130\x90a/\x9FV[\x80\x15a\x13}W\x80`\x1F\x10a\x13RWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13}V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xCE\xED.\x8D\x0C\x8EL.\xEC-\x89-\xCC\x8C\xAF`\x83\x1B\x81RPa(\xA6V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14@W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x13\xD4V[`\0a\x05\x90`\x1C\x80Ta\x14\xA8\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xD4\x90a/\x9FV[\x80\x15a\x15!W\x80`\x1F\x10a\x14\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x04W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x0B\x9C\xDB\x1B\xDD`\xDA\x1B\x81RPa(\xA6V[`\0a\x05\x90`\x1C\x80Ta\x15[\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x87\x90a/\x9FV[\x80\x15a\x15\xD4W\x80`\x1F\x10a\x15\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xD4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xB7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x05\xCE\xCC-\x8D,\x8C.\x8D\xEEI-\xCC\x8C\xAF`\x8B\x1B\x81RPa(\xA6V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x16I\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16u\x90a/\x9FV[\x80\x15a\x16\xC2W\x80`\x1F\x10a\x16\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16*V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17}\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x17\x8D\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xAA\x91` \x01a1\x89V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x17\xCE\x92\x91\x90a(\xDCV[Pa\x17\xDF`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x17\xF1Wa\x17\xF1a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x18\x06\x81a0TV[\x91PPa\x17\x02V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x18\xDCW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18\x9EW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x182V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\0[`\x04\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\x99\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x19\xA9\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19\xC6\x91` \x01a1\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x19\xEA\x92\x91\x90a(\xDCV[Pa\x19\xFB`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x1A\rWa\x1A\ra0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x1A\"\x81a0TV[\x91PPa\x19\x1EV[a\x1A2a)\x9EV[`\0[`\x03\x81\x10\x15a\x1B9W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\xB0\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1A\xC0\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1A\xDD\x91` \x01a2\x01V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1B\x01\x92\x91\x90a(\xDCV[Pa\x1B\x12`\x1C\x80Ta\t\x1F\x90a/\x9FV[`0\x82`\x03\x81\x10a\x1B%Wa\x1B%a0>V[\x01U\x80a\x1B1\x81a0TV[\x91PPa\x1A5V[P`@\x80Q``\x81\x01\x91\x82\x90R`0\x80T\x82R\x90\x91`\x03\x90`1` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[a\x1Bpa)\xBCV[`\0[`\x12\x81\x10\x15a\x1CwW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xEE\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1B\xFE\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C\x1B\x91` \x01a24V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1C?\x92\x91\x90a(\xDCV[Pa\x1CP`\x1C\x80Ta\t\x1F\x90a/\x9FV[`\x1E\x82`\x12\x81\x10a\x1CcWa\x1Cca0>V[\x01U\x80a\x1Co\x81a0TV[\x91PPa\x1BsV[P`@\x80Qa\x02@\x81\x01\x91\x82\x90R`\x1E\x80T\x82R\x90\x91`\x12\x90`\x1F` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`@\x80Q`,\x80\x82Ra\x05\xA0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\x80\x806\x837\x01\x90PP\x90P`\0[`,\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1DN\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1D^\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1D{\x91` \x01a2aV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1D\x9F\x92\x91\x90a(\xDCV[Pa\x1D\xB0`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x1D\xC2Wa\x1D\xC2a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x1D\xD7\x81a0TV[\x91PPa\x1C\xD3V[a\x1D\xE7a)\xDBV[`\0[`\t\x81\x10\x15a\x1E\xEEW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Ee\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1Eu\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\x92\x91` \x01a2\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1E\xB6\x92\x91\x90a(\xDCV[Pa\x1E\xC7`\x1C\x80Ta\t\x1F\x90a/\x9FV[`3\x82`\t\x81\x10a\x1E\xDAWa\x1E\xDAa0>V[\x01U\x80a\x1E\xE6\x81a0TV[\x91PPa\x1D\xEAV[P`@\x80Qa\x01 \x81\x01\x91\x82\x90R`3\x80T\x82R\x90\x91`\t\x90`4` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x1F0\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\\\x90a/\x9FV[\x80\x15a\x1F\xA9W\x80`\x1F\x10a\x1F~Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xA9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x0B\x9D\x1A[Y\\\xDD\x18[\\\x14\x9B\xDB\xDD`\x92\x1B\x81RPa((V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W\x83\x82\x90`\0R` `\0 \x01\x80Ta \x1D\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta I\x90a/\x9FV[\x80\x15a \x96W\x80`\x1F\x10a kWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \x96V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a yW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xFEV[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a \xCCWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a4\t\x839\x81Q\x91R;\x15a!\xC6W`@\x80Q`\0\x80Q` a4\t\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a!N\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a2\xD2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!h\x91a3\x03V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a!\xA5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a!\xAAV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a!\xC2\x91\x90a3\x1FV[\x91PP[\x91\x90PV[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P`\0[`\x05\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"HW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"p\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\"\x80\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"\x9D\x91` \x01a3AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\"\xC1\x92\x91\x90a(\xDCV[Pa\"\xD2`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\"\xE4Wa\"\xE4a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\"\xF9\x81a0TV[\x91PPa!\xF5V[`\0a\x05\x90`\x1C\x80Ta#\x13\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#?\x90a/\x9FV[\x80\x15a#\x8CW\x80`\x1F\x10a#aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x8CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F.historicalSummaryIndex\0\0\0\0\0\0\0\0\0\x81RPa(\xA6V[a#\xD4a)\xFAV[`\0[`\x04\x81\x10\x15a$\xDBW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$R\x91\x90\x81\x01\x90a/(V[`@Q` \x01a$b\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra$\x7F\x91` \x01a3gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a$\xA3\x92\x91\x90a(\xDCV[Pa$\xB4`\x1C\x80Ta\t\x1F\x90a/\x9FV[`\x9D\x82`\x04\x81\x10a$\xC7Wa$\xC7a0>V[\x01U\x80a$\xD3\x81a0TV[\x91PPa#\xD7V[P`@\x80Q`\x80\x81\x01\x91\x82\x90R`\x9D\x80T\x82R\x90\x91`\x04\x90`\x9E` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta%\x1C\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%H\x90a/\x9FV[\x80\x15a%\x95W\x80`\x1F\x10a%jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\x95V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x0B\x99^\x19X\xDD]\x1A[\xDB\x94\x18^[\x1B\xD8Y\x14\x9B\xDB\xDD`Z\x1B\x81RPa((V[a%\xD5a*\x18V[`\0[`\x07\x81\x10\x15a&\xDCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&S\x91\x90\x81\x01\x90a/(V[`@Q` \x01a&c\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra&\x80\x91` \x01a3\x92V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a&\xA4\x92\x91\x90a(\xDCV[Pa&\xB5`\x1C\x80Ta\t\x1F\x90a/\x9FV[`\x96\x82`\x07\x81\x10a&\xC8Wa&\xC8a0>V[\x01U\x80a&\xD4\x81a0TV[\x91PPa%\xD8V[P`@\x80Q`\xE0\x81\x01\x91\x82\x90R`\x96\x80T\x82R\x90\x91`\x07\x90`\x97` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta'}\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'\xA9\x90a/\x9FV[\x80\x15a'\xF6W\x80`\x1F\x10a'\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\xF6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r.ValidatorFields[0]`h\x1B\x81RP[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R`\0\x90`\0\x80Q` a4\t\x839\x81Q\x91R\x90c\x17w\xE5\x9D\x90a(^\x90\x86\x90\x86\x90`\x04\x01a3\xCAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a({W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x9F\x91\x90a3\xEFV[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R`\0\x90`\0\x80Q` a4\t\x839\x81Q\x91R\x90c\xAD\xDD\xE2\xB6\x90a(^\x90\x86\x90\x86\x90`\x04\x01a3\xCAV[\x82\x80Ta(\xE8\x90a/\x9FV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a)\nW`\0\x85Ua)PV[\x82`\x1F\x10a)#W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua)PV[\x82\x80\x01`\x01\x01\x85U\x82\x15a)PW\x91\x82\x01[\x82\x81\x11\x15a)PW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a)5V[Pa)\\\x92\x91Pa*6V[P\x90V[`@Q\x80a\x05\x80\x01`@R\x80`,\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\xC0\x01`@R\x80`.\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02@\x01`@R\x80`\x12\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80`\x07\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a)\\W`\0\x81U`\x01\x01a*7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\x8AWa*\x8Aa*KV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*\xACWa*\xACa*KV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a*\xCCW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xE3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a*\xF4W`\0\x80\xFD[\x805a+\x07a+\x02\x82a*\x92V[a*aV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a+\x1CW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a+{W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a+VV[P\x90\x96\x95PPPPPPV[a\x05\x80\x81\x01\x81\x83`\0[`,\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\x91V[PPP\x92\x91PPV[`\0[\x83\x81\x10\x15a+\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01a+\xBCV[\x83\x81\x11\x15a+\xE3W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra,\x01\x81` \x86\x01` \x86\x01a+\xB9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a,\xC5W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a,\xAFW`_\x19\x89\x85\x03\x01\x83Ra,\x9D\x84\x86Qa+\xE9V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a,\x81V[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a,<V[P\x91\x9A\x99PPPPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a+{W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a,\xF1V[a\x05\xC0\x81\x01\x81\x83`\0[`.\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\x17V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a-\xDAW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a-\xC5W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a-\x9BV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a-^V[P\x91\x99\x98PPPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a.>W`?\x19\x88\x86\x03\x01\x84Ra.,\x85\x83Qa+\xE9V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a.\x10V[P\x92\x97\x96PPPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.TV[a\x02@\x81\x01\x81\x83`\0[`\x12\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.}V[a\x01 \x81\x01\x81\x83`\0[`\t\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xA6V[`\x80\x81\x01\x81\x83`\0[`\x04\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xCEV[`\xE0\x81\x01\x81\x83`\0[`\x07\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xF6V[` \x81R`\0a(\x9F` \x83\x01\x84a+\xE9V[`\0` \x82\x84\x03\x12\x15a/:W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/QW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/bW`\0\x80\xFD[\x80Qa/pa+\x02\x82a*\x92V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\x85W`\0\x80\xFD[a/\x96\x82` \x83\x01` \x86\x01a+\xB9V[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a/\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\r\x03WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x82Qa/\xE6\x81\x84` \x87\x01a+\xB9V[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[\x7F.HistoricalSummaryProof[\0\0\0\0\0\0\0\0\x81R`\0\x82Qa01\x81`\x18\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x18\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a0vWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x7F.WithdrawalCredentialProof[\0\0\0\0\0\x81R`\0\x82Qa0\xB5\x81`\x1B\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x1B\x01\x92\x91PPV[\x7F.StateRootAgainstLatestBlockHead\x81RgerProof[`\xC0\x1B` \x82\x01R`\0\x82Qa1\x0B\x81`(\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`(\x01\x92\x91PPV[p.ValidatorFields[`x\x1B\x81R`\0\x82Qa1D\x81`\x11\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x11\x01\x92\x91PPV[o.ValidatorProof[`\x80\x1B\x81R`\0\x82Qa1|\x81`\x10\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x10\x01\x92\x91PPV[u.ValidatorFieldsProof[`P\x1B\x81R`\0\x82Qa1\xBA\x81`\x16\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x16\x01\x92\x91PPV[q.WithdrawalFields[`p\x1B\x81R`\0\x82Qa1\xF4\x81`\x12\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x12\x01\x92\x91PPV[j.SlotProof[`\xA8\x1B\x81R`\0\x82Qa2'\x81`\x0B\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x0B\x01\x92\x91PPV[q.BlockHeaderProof[`p\x1B\x81R`\0\x82Qa1\xF4\x81`\x12\x85\x01` \x87\x01a+\xB9V[\x7F.ValidatorBalanceProof[\0\0\0\0\0\0\0\0\0\x81R`\0\x82Qa2\x99\x81`\x17\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x17\x01\x92\x91PPV[p.WithdrawalProof[`x\x1B\x81R`\0\x82Qa1D\x81`\x11\x85\x01` \x87\x01a+\xB9V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a2\xF5\x81`\x04\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa3\x15\x81\x84` \x87\x01a+\xB9V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a31W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a(\x9FW`\0\x80\xFD[j.slotProof[`\xA8\x1B\x81R`\0\x82Qa2'\x81`\x0B\x85\x01` \x87\x01a+\xB9V[o.TimestampProof[`\x80\x1B\x81R`\0\x82Qa1|\x81`\x10\x85\x01` \x87\x01a+\xB9V[\x7F.ExecutionPayloadProof[\0\0\0\0\0\0\0\0\0\x81R`\0\x82Qa2\x99\x81`\x17\x85\x01` \x87\x01a+\xB9V[`@\x81R`\0a3\xDD`@\x83\x01\x85a+\xE9V[\x82\x81\x03` \x84\x01Ra/\x96\x81\x85a+\xE9V[`\0` \x82\x84\x03\x12\x15a4\x01W`\0\x80\xFD[PQ\x91\x90PV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 W\xAEg\xFE\xE4\xFB=\x1Cmq\xCDkm\xD3u\x96HlZ\xB1\xB5\xFD\xAE\x13\xD9Qpg\xB5\xB9\xD1\\dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061023d5760003560e01c8063765aa6061161013b578063b5508aa9116100b8578063d944472f1161007c578063d944472f14610411578063db364a4014610419578063e20c9f711461042e578063f148082c14610436578063fa7626d41461043e57600080fd5b8063b5508aa9146103cc578063ba414fa6146103d4578063c3da8ae9146103ec578063d6461cbb146103f4578063d911b683146103fc57600080fd5b80639de4a9b3116100ff5780639de4a9b31461037d578063a507742914610392578063a6b7a848146103a7578063ab72161c146103af578063b38061bf146103c457600080fd5b8063765aa6061461034857806385226c8114610350578063893893ca14610365578063916a17c61461036d578063950bb6821461037557600080fd5b806334e3d90e116101c95780634c20f5101161018d5780634c20f510146103065780634c38f9131461030e57806364f38ed71461032357806366d9a9a01461032b5780636c877c841461034057600080fd5b806334e3d90e146102de5780633e5e3c23146102e65780633f7286f4146102ee57806342864734146102f65780634656fdb5146102fe57600080fd5b80631ed7831c116102105780631ed7831c14610282578063275378b1146102975780632872e20c1461029f5780632ade3880146102b45780632fd1793c146102c957600080fd5b806308a4d71f1461024257806316f0715314610257578063177541fc1461027257806318aadf311461027a575b600080fd5b610255610250366004612aba565b61044b565b005b61025f6104d9565b6040519081526020015b60405180910390f35b61025f610595565b61025f610653565b61028a610717565b6040516102699190612b3a565b61025f610779565b6102a7610837565b6040516102699190612b87565b6102bc610a89565b6040516102699190612c15565b6102d1610bcb565b6040516102699190612cd5565b6102d1610d09565b61028a610e3e565b61028a610e9e565b61025f610efe565b61025f610fc1565b6102d161107b565b6103166111b3565b6040516102699190612d0d565b61025f6112f2565b6103336113b0565b6040516102699190612d36565b61025f611496565b61025f611549565b610358611606565b6040516102699190612de9565b6102d16116d6565b61033361180e565b6102d16118f4565b610385611a2a565b6040516102699190612e4b565b61039a611b68565b6040516102699190612e73565b6102d1611ca7565b6103b7611ddf565b6040516102699190612e9c565b61025f611f1e565b610358611fda565b6103dc6120aa565b6040519015158152602001610269565b6102d16121cb565b61025f612301565b6104046123cc565b6040516102699190612ec5565b61025f61250a565b6104216125cd565b6040516102699190612eed565b61028a61270b565b61025f61276b565b6007546103dc9060ff1681565b6040516360f9bb1160e01b8152600080516020613409833981519152906360f9bb119061047c908490600401612f15565b600060405180830381865afa158015610499573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104c19190810190612f28565b80516104d591601c916020909101906128dc565b5050565b6000610590601c80546104eb90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461051790612f9f565b80156105645780601f1061053957610100808354040283529160200191610564565b820191906000526020600020905b81548152906001019060200180831161054757829003601f168201915b5050505050604051806040016040528060098152602001680b9cdb1bdd149bdbdd60ba1b815250612828565b905090565b6000610590601c80546105a790612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546105d390612f9f565b80156106205780601f106105f557610100808354040283529160200191610620565b820191906000526020600020905b81548152906001019060200180831161060357829003601f168201915b50505050506040518060400160405280601081526020016f0b98995858dbdb94dd185d19549bdbdd60821b815250612828565b6000610590601c805461066590612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461069190612f9f565b80156106de5780601f106106b3576101008083540402835291602001916106de565b820191906000526020600020905b8154815290600101906020018083116106c157829003601f168201915b5050505050604051806040016040528060168152602001750b9b185d195cdd109b1bd8dad2195859195c949bdbdd60521b815250612828565b6060601480548060200260200160405190810160405280929190818152602001828054801561076f57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610751575b5050505050905090565b6000610590601c805461078b90612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546107b790612f9f565b80156108045780601f106107d957610100808354040283529160200191610804565b820191906000526020600020905b8154815290600101906020018083116107e757829003601f168201915b50505050506040518060400160405280601081526020016f0b989b1bd8dad2195859195c949bdbdd60821b815250612828565b61083f612960565b60005b602c811015610a545760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015610895573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526108bd9190810190612f28565b6040516020016108cd9190612fd4565b60408051601f19818403018152908290526108ea91602001612ff9565b604051602081830303815290604052601d908051906020019061090e9291906128dc565b50610a2d601c805461091f90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461094b90612f9f565b80156109985780601f1061096d57610100808354040283529160200191610998565b820191906000526020600020905b81548152906001019060200180831161097b57829003601f168201915b5050505050601d80546109aa90612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546109d690612f9f565b8015610a235780601f106109f857610100808354040283529160200191610a23565b820191906000526020600020905b815481529060010190602001808311610a0657829003601f168201915b5050505050612828565b606a82602c8110610a4057610a4061303e565b015580610a4c81613054565b915050610842565b506040805161058081019182905290606a90602c9082845b815481526020019060010190808311610a6c575050505050905090565b6060601b805480602002602001604051908101604052809291908181526020016000905b82821015610bc257600084815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610bab578382906000526020600020018054610b1e90612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054610b4a90612f9f565b8015610b975780601f10610b6c57610100808354040283529160200191610b97565b820191906000526020600020905b815481529060010190602001808311610b7a57829003601f168201915b505050505081526020019060010190610aff565b505050508152505081526020019060010190610aad565b50505050905090565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015610c4a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610c729190810190612f28565b604051602001610c829190612fd4565b60408051601f1981840301815290829052610c9f9160200161307d565b604051602081830303815290604052601d9080519060200190610cc39291906128dc565b50610cd4601c805461091f90612f9f565b828281518110610ce657610ce661303e565b602090810291909101015280610cfb81613054565b915050610bf7565b50919050565b604080516003808252608082019092526060916000919060208201848036833701905050905060005b6003811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015610d85573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610dad9190810190612f28565b604051602001610dbd9190612fd4565b60408051601f1981840301815290829052610dda916020016130c2565b604051602081830303815290604052601d9080519060200190610dfe9291906128dc565b50610e0f601c805461091f90612f9f565b828281518110610e2157610e2161303e565b602090810291909101015280610e3681613054565b915050610d32565b6060601680548060200260200160405190810160405280929190818152602001828054801561076f576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610751575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561076f576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610751575050505050905090565b6000610590601c8054610f1090612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054610f3c90612f9f565b8015610f895780601f10610f5e57610100808354040283529160200191610f89565b820191906000526020600020905b815481529060010190602001808311610f6c57829003601f168201915b50505050506040518060400160405280601581526020017405cc4d8dec6d690cac2c8cae4a4dedee892dcc8caf605b1b8152506128a6565b6000610590601c8054610fd390612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054610fff90612f9f565b801561104c5780601f106110215761010080835404028352916020019161104c565b820191906000526020600020905b81548152906001019060200180831161102f57829003601f168201915b50505050506040518060400160405280600c81526020016b0b98985b185b98d9549bdbdd60a21b815250612828565b604080516008808252610120820190925260609160009190602082016101008036833701905050905060005b6008811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa1580156110fa573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111229190810190612f28565b6040516020016111329190612fd4565b60408051601f198184030181529082905261114f91602001613118565b604051602081830303815290604052601d90805190602001906111739291906128dc565b50611184601c805461091f90612f9f565b8282815181106111965761119661303e565b6020908102919091010152806111ab81613054565b9150506110a7565b6111bb61297f565b60005b602e8110156112c25760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611211573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526112399190810190612f28565b6040516020016112499190612fd4565b60408051601f198184030181529082905261126691602001613151565b604051602081830303815290604052601d908051906020019061128a9291906128dc565b5061129b601c805461091f90612f9f565b603c82602e81106112ae576112ae61303e565b0155806112ba81613054565b9150506111be565b50604080516105c0810191829052603c805482529091602e90603d60208501808311610a6c575050505050905090565b6000610590601c805461130490612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461133090612f9f565b801561137d5780601f106113525761010080835404028352916020019161137d565b820191906000526020600020905b81548152906001019060200180831161136057829003601f168201915b50505050506040518060400160405280601081526020016f05ceed2e8d0c8e4c2eec2d892dcc8caf60831b8152506128a6565b60606019805480602002602001604051908101604052809291908181526020016000905b82821015610bc25760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561147e57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114405790505b505050505081525050815260200190600101906113d4565b6000610590601c80546114a890612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546114d490612f9f565b80156115215780601f106114f657610100808354040283529160200191611521565b820191906000526020600020905b81548152906001019060200180831161150457829003601f168201915b5050505050604051806040016040528060058152602001640b9cdb1bdd60da1b8152506128a6565b6000610590601c805461155b90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461158790612f9f565b80156115d45780601f106115a9576101008083540402835291602001916115d4565b820191906000526020600020905b8154815290600101906020018083116115b757829003601f168201915b50505050506040518060400160405280600f81526020016e05cecc2d8d2c8c2e8dee492dcc8caf608b1b8152506128a6565b60606018805480602002602001604051908101604052809291908181526020016000905b82821015610bc257838290600052602060002001805461164990612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461167590612f9f565b80156116c25780601f10611697576101008083540402835291602001916116c2565b820191906000526020600020905b8154815290600101906020018083116116a557829003601f168201915b50505050508152602001906001019061162a565b60408051602e8082526105e0820190925260609160009190602082016105c08036833701905050905060005b602e811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611755573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261177d9190810190612f28565b60405160200161178d9190612fd4565b60408051601f19818403018152908290526117aa91602001613189565b604051602081830303815290604052601d90805190602001906117ce9291906128dc565b506117df601c805461091f90612f9f565b8282815181106117f1576117f161303e565b60209081029190910101528061180681613054565b915050611702565b6060601a805480602002602001604051908101604052809291908181526020016000905b82821015610bc25760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156118dc57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161189e5790505b50505050508152505081526020019060010190611832565b60408051600480825260a08201909252606091600091906020820160808036833701905050905060005b6004811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611971573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526119999190810190612f28565b6040516020016119a99190612fd4565b60408051601f19818403018152908290526119c6916020016131c7565b604051602081830303815290604052601d90805190602001906119ea9291906128dc565b506119fb601c805461091f90612f9f565b828281518110611a0d57611a0d61303e565b602090810291909101015280611a2281613054565b91505061191e565b611a3261299e565b60005b6003811015611b395760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611a88573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611ab09190810190612f28565b604051602001611ac09190612fd4565b60408051601f1981840301815290829052611add91602001613201565b604051602081830303815290604052601d9080519060200190611b019291906128dc565b50611b12601c805461091f90612f9f565b60308260038110611b2557611b2561303e565b015580611b3181613054565b915050611a35565b506040805160608101918290526030805482529091600390603160208501808311610a6c575050505050905090565b611b706129bc565b60005b6012811015611c775760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611bc6573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611bee9190810190612f28565b604051602001611bfe9190612fd4565b60408051601f1981840301815290829052611c1b91602001613234565b604051602081830303815290604052601d9080519060200190611c3f9291906128dc565b50611c50601c805461091f90612f9f565b601e8260128110611c6357611c6361303e565b015580611c6f81613054565b915050611b73565b5060408051610240810191829052601e805482529091601290601f60208501808311610a6c575050505050905090565b60408051602c8082526105a0820190925260609160009190602082016105808036833701905050905060005b602c811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611d26573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d4e9190810190612f28565b604051602001611d5e9190612fd4565b60408051601f1981840301815290829052611d7b91602001613261565b604051602081830303815290604052601d9080519060200190611d9f9291906128dc565b50611db0601c805461091f90612f9f565b828281518110611dc257611dc261303e565b602090810291909101015280611dd781613054565b915050611cd3565b611de76129db565b60005b6009811015611eee5760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015611e3d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611e659190810190612f28565b604051602001611e759190612fd4565b60408051601f1981840301815290829052611e92916020016132a6565b604051602081830303815290604052601d9080519060200190611eb69291906128dc565b50611ec7601c805461091f90612f9f565b60338260098110611eda57611eda61303e565b015580611ee681613054565b915050611dea565b50604080516101208101918290526033805482529091600990603460208501808311610a6c575050505050905090565b6000610590601c8054611f3090612f9f565b80601f0160208091040260200160405190810160405280929190818152602001828054611f5c90612f9f565b8015611fa95780601f10611f7e57610100808354040283529160200191611fa9565b820191906000526020600020905b815481529060010190602001808311611f8c57829003601f168201915b50505050506040518060400160405280600e81526020016d0b9d1a5b595cdd185b5c149bdbdd60921b815250612828565b60606017805480602002602001604051908101604052809291908181526020016000905b82821015610bc257838290600052602060002001805461201d90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461204990612f9f565b80156120965780601f1061206b57610100808354040283529160200191612096565b820191906000526020600020905b81548152906001019060200180831161207957829003601f168201915b505050505081526020019060010190611ffe565b600754600090610100900460ff16156120cc5750600754610100900460ff1690565b60006000805160206134098339815191523b156121c65760408051600080516020613409833981519152602082018190526519985a5b195960d21b8284015282518083038401815260608301909352600092909161214e917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016132d2565b60408051601f198184030181529082905261216891613303565b6000604051808303816000865af19150503d80600081146121a5576040519150601f19603f3d011682016040523d82523d6000602084013e6121aa565b606091505b50915050808060200190518101906121c2919061331f565b9150505b919050565b60408051600580825260c08201909252606091600091906020820160a08036833701905050905060005b6005811015610d035760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa158015612248573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526122709190810190612f28565b6040516020016122809190612fd4565b60408051601f198184030181529082905261229d91602001613341565b604051602081830303815290604052601d90805190602001906122c19291906128dc565b506122d2601c805461091f90612f9f565b8282815181106122e4576122e461303e565b6020908102919091010152806122f981613054565b9150506121f5565b6000610590601c805461231390612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461233f90612f9f565b801561238c5780601f106123615761010080835404028352916020019161238c565b820191906000526020600020905b81548152906001019060200180831161236f57829003601f168201915b50505050506040518060400160405280601781526020017f2e686973746f726963616c53756d6d617279496e6465780000000000000000008152506128a6565b6123d46129fa565b60005b60048110156124db5760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa15801561242a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526124529190810190612f28565b6040516020016124629190612fd4565b60408051601f198184030181529082905261247f91602001613367565b604051602081830303815290604052601d90805190602001906124a39291906128dc565b506124b4601c805461091f90612f9f565b609d82600481106124c7576124c761303e565b0155806124d381613054565b9150506123d7565b50604080516080810191829052609d805482529091600490609e60208501808311610a6c575050505050905090565b6000610590601c805461251c90612f9f565b80601f016020809104026020016040519081016040528092919081815260200182805461254890612f9f565b80156125955780601f1061256a57610100808354040283529160200191612595565b820191906000526020600020905b81548152906001019060200180831161257857829003601f168201915b5050505050604051806040016040528060158152602001740b995e1958dd5d1a5bdb94185e5b1bd859149bdbdd605a1b815250612828565b6125d5612a18565b60005b60078110156126dc5760405163348051d760e11b81526004810182905260008051602061340983398151915290636900a3ae90602401600060405180830381865afa15801561262b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526126539190810190612f28565b6040516020016126639190612fd4565b60408051601f198184030181529082905261268091602001613392565b604051602081830303815290604052601d90805190602001906126a49291906128dc565b506126b5601c805461091f90612f9f565b609682600781106126c8576126c861303e565b0155806126d481613054565b9150506125d8565b506040805160e08101918290526096805482529091600790609760208501808311610a6c575050505050905090565b6060601380548060200260200160405190810160405280929190818152602001828054801561076f576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610751575050505050905090565b6000610590601c805461277d90612f9f565b80601f01602080910402602001604051908101604052809291908181526020018280546127a990612f9f565b80156127f65780601f106127cb576101008083540402835291602001916127f6565b820191906000526020600020905b8154815290600101906020018083116127d957829003601f168201915b5050505050604051806040016040528060138152602001722e56616c696461746f724669656c64735b305d60681b8152505b604051631777e59d60e01b815260009060008051602061340983398151915290631777e59d9061285e90869086906004016133ca565b602060405180830381865afa15801561287b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061289f91906133ef565b9392505050565b6040516356eef15b60e11b81526000906000805160206134098339815191529063addde2b69061285e90869086906004016133ca565b8280546128e890612f9f565b90600052602060002090601f01602090048101928261290a5760008555612950565b82601f1061292357805160ff1916838001178555612950565b82800160010185558215612950579182015b82811115612950578251825591602001919060010190612935565b5061295c929150612a36565b5090565b604051806105800160405280602c906020820280368337509192915050565b604051806105c00160405280602e906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b6040518061024001604052806012906020820280368337509192915050565b6040518061012001604052806009906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060e001604052806007906020820280368337509192915050565b5b8082111561295c5760008155600101612a37565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612a8a57612a8a612a4b565b604052919050565b600067ffffffffffffffff821115612aac57612aac612a4b565b50601f01601f191660200190565b600060208284031215612acc57600080fd5b813567ffffffffffffffff811115612ae357600080fd5b8201601f81018413612af457600080fd5b8035612b07612b0282612a92565b612a61565b818152856020838501011115612b1c57600080fd5b81602084016020830137600091810160200191909152949350505050565b6020808252825182820181905260009190848201906040850190845b81811015612b7b5783516001600160a01b031683529284019291840191600101612b56565b50909695505050505050565b6105808101818360005b602c811015612bb0578151835260209283019290910190600101612b91565b50505092915050565b60005b83811015612bd4578181015183820152602001612bbc565b83811115612be3576000848401525b50505050565b60008151808452612c01816020860160208601612bb9565b601f01601f19169290920160200192915050565b602080825282518282018190526000919060409081850190600581811b8701840188860187805b85811015612cc557603f198b8503018752825180516001600160a01b031685528901518985018990528051898601819052908a0190606081881b870181019190870190855b81811015612caf57605f19898503018352612c9d848651612be9565b948e01949350918d0191600101612c81565b505050978a019794505091880191600101612c3c565b50919a9950505050505050505050565b6020808252825182820181905260009190848201906040850190845b81811015612b7b57835183529284019291840191600101612cf1565b6105c08101818360005b602e811015612bb0578151835260209283019290910190600101612d17565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015612dda57898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015612dc55783516001600160e01b0319168252928b019260019290920191908b0190612d9b565b50978a01979550505091870191600101612d5e565b50919998505050505050505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015612e3e57603f19888603018452612e2c858351612be9565b94509285019290850190600101612e10565b5092979650505050505050565b60608101818360005b6003811015612bb0578151835260209283019290910190600101612e54565b6102408101818360005b6012811015612bb0578151835260209283019290910190600101612e7d565b6101208101818360005b6009811015612bb0578151835260209283019290910190600101612ea6565b60808101818360005b6004811015612bb0578151835260209283019290910190600101612ece565b60e08101818360005b6007811015612bb0578151835260209283019290910190600101612ef6565b60208152600061289f6020830184612be9565b600060208284031215612f3a57600080fd5b815167ffffffffffffffff811115612f5157600080fd5b8201601f81018413612f6257600080fd5b8051612f70612b0282612a92565b818152856020838501011115612f8557600080fd5b612f96826020830160208601612bb9565b95945050505050565b600181811c90821680612fb357607f821691505b60208210811415610d0357634e487b7160e01b600052602260045260246000fd5b60008251612fe6818460208701612bb9565b605d60f81b920191825250600101919050565b7f2e486973746f726963616c53756d6d61727950726f6f665b0000000000000000815260008251613031816018850160208701612bb9565b9190910160180192915050565b634e487b7160e01b600052603260045260246000fd5b600060001982141561307657634e487b7160e01b600052601160045260246000fd5b5060010190565b7f2e5769746864726177616c43726564656e7469616c50726f6f665b00000000008152600082516130b581601b850160208701612bb9565b91909101601b0192915050565b7f2e5374617465526f6f74416761696e73744c6174657374426c6f636b48656164815267657250726f6f665b60c01b60208201526000825161310b816028850160208701612bb9565b9190910160280192915050565b702e56616c696461746f724669656c64735b60781b815260008251613144816011850160208701612bb9565b9190910160110192915050565b6f2e56616c696461746f7250726f6f665b60801b81526000825161317c816010850160208701612bb9565b9190910160100192915050565b752e56616c696461746f724669656c647350726f6f665b60501b8152600082516131ba816016850160208701612bb9565b9190910160160192915050565b712e5769746864726177616c4669656c64735b60701b8152600082516131f4816012850160208701612bb9565b9190910160120192915050565b6a2e536c6f7450726f6f665b60a81b81526000825161322781600b850160208701612bb9565b91909101600b0192915050565b712e426c6f636b48656164657250726f6f665b60701b8152600082516131f4816012850160208701612bb9565b7f2e56616c696461746f7242616c616e636550726f6f665b000000000000000000815260008251613299816017850160208701612bb9565b9190910160170192915050565b702e5769746864726177616c50726f6f665b60781b815260008251613144816011850160208701612bb9565b6001600160e01b03198316815281516000906132f5816004850160208701612bb9565b919091016004019392505050565b60008251613315818460208701612bb9565b9190910192915050565b60006020828403121561333157600080fd5b8151801515811461289f57600080fd5b6a2e736c6f7450726f6f665b60a81b81526000825161322781600b850160208701612bb9565b6f2e54696d657374616d7050726f6f665b60801b81526000825161317c816010850160208701612bb9565b7f2e457865637574696f6e5061796c6f616450726f6f665b000000000000000000815260008251613299816017850160208701612bb9565b6040815260006133dd6040830185612be9565b8281036020840152612f968185612be9565b60006020828403121561340157600080fd5b505191905056fe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da264697066735822122057ae67fee4fb3d1c6d71cd6b6dd37596486c5ab1b5fdae13d9517067b5b9d15c64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80cvZ\xA6\x06\x11a\x01;W\x80c\xB5P\x8A\xA9\x11a\0\xB8W\x80c\xD9DG/\x11a\0|W\x80c\xD9DG/\x14a\x04\x11W\x80c\xDB6J@\x14a\x04\x19W\x80c\xE2\x0C\x9Fq\x14a\x04.W\x80c\xF1H\x08,\x14a\x046W\x80c\xFAv&\xD4\x14a\x04>W`\0\x80\xFD[\x80c\xB5P\x8A\xA9\x14a\x03\xCCW\x80c\xBAAO\xA6\x14a\x03\xD4W\x80c\xC3\xDA\x8A\xE9\x14a\x03\xECW\x80c\xD6F\x1C\xBB\x14a\x03\xF4W\x80c\xD9\x11\xB6\x83\x14a\x03\xFCW`\0\x80\xFD[\x80c\x9D\xE4\xA9\xB3\x11a\0\xFFW\x80c\x9D\xE4\xA9\xB3\x14a\x03}W\x80c\xA5\x07t)\x14a\x03\x92W\x80c\xA6\xB7\xA8H\x14a\x03\xA7W\x80c\xABr\x16\x1C\x14a\x03\xAFW\x80c\xB3\x80a\xBF\x14a\x03\xC4W`\0\x80\xFD[\x80cvZ\xA6\x06\x14a\x03HW\x80c\x85\"l\x81\x14a\x03PW\x80c\x898\x93\xCA\x14a\x03eW\x80c\x91j\x17\xC6\x14a\x03mW\x80c\x95\x0B\xB6\x82\x14a\x03uW`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x11a\x01\xC9W\x80cL \xF5\x10\x11a\x01\x8DW\x80cL \xF5\x10\x14a\x03\x06W\x80cL8\xF9\x13\x14a\x03\x0EW\x80cd\xF3\x8E\xD7\x14a\x03#W\x80cf\xD9\xA9\xA0\x14a\x03+W\x80cl\x87|\x84\x14a\x03@W`\0\x80\xFD[\x80c4\xE3\xD9\x0E\x14a\x02\xDEW\x80c>^<#\x14a\x02\xE6W\x80c?r\x86\xF4\x14a\x02\xEEW\x80cB\x86G4\x14a\x02\xF6W\x80cFV\xFD\xB5\x14a\x02\xFEW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x02\x10W\x80c\x1E\xD7\x83\x1C\x14a\x02\x82W\x80c'Sx\xB1\x14a\x02\x97W\x80c(r\xE2\x0C\x14a\x02\x9FW\x80c*\xDE8\x80\x14a\x02\xB4W\x80c/\xD1y<\x14a\x02\xC9W`\0\x80\xFD[\x80c\x08\xA4\xD7\x1F\x14a\x02BW\x80c\x16\xF0qS\x14a\x02WW\x80c\x17uA\xFC\x14a\x02rW\x80c\x18\xAA\xDF1\x14a\x02zW[`\0\x80\xFD[a\x02Ua\x02P6`\x04a*\xBAV[a\x04KV[\0[a\x02_a\x04\xD9V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02_a\x05\x95V[a\x02_a\x06SV[a\x02\x8Aa\x07\x17V[`@Qa\x02i\x91\x90a+:V[a\x02_a\x07yV[a\x02\xA7a\x087V[`@Qa\x02i\x91\x90a+\x87V[a\x02\xBCa\n\x89V[`@Qa\x02i\x91\x90a,\x15V[a\x02\xD1a\x0B\xCBV[`@Qa\x02i\x91\x90a,\xD5V[a\x02\xD1a\r\tV[a\x02\x8Aa\x0E>V[a\x02\x8Aa\x0E\x9EV[a\x02_a\x0E\xFEV[a\x02_a\x0F\xC1V[a\x02\xD1a\x10{V[a\x03\x16a\x11\xB3V[`@Qa\x02i\x91\x90a-\rV[a\x02_a\x12\xF2V[a\x033a\x13\xB0V[`@Qa\x02i\x91\x90a-6V[a\x02_a\x14\x96V[a\x02_a\x15IV[a\x03Xa\x16\x06V[`@Qa\x02i\x91\x90a-\xE9V[a\x02\xD1a\x16\xD6V[a\x033a\x18\x0EV[a\x02\xD1a\x18\xF4V[a\x03\x85a\x1A*V[`@Qa\x02i\x91\x90a.KV[a\x03\x9Aa\x1BhV[`@Qa\x02i\x91\x90a.sV[a\x02\xD1a\x1C\xA7V[a\x03\xB7a\x1D\xDFV[`@Qa\x02i\x91\x90a.\x9CV[a\x02_a\x1F\x1EV[a\x03Xa\x1F\xDAV[a\x03\xDCa \xAAV[`@Q\x90\x15\x15\x81R` \x01a\x02iV[a\x02\xD1a!\xCBV[a\x02_a#\x01V[a\x04\x04a#\xCCV[`@Qa\x02i\x91\x90a.\xC5V[a\x02_a%\nV[a\x04!a%\xCDV[`@Qa\x02i\x91\x90a.\xEDV[a\x02\x8Aa'\x0BV[a\x02_a'kV[`\x07Ta\x03\xDC\x90`\xFF\x16\x81V[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R`\0\x80Q` a4\t\x839\x81Q\x91R\x90c`\xF9\xBB\x11\x90a\x04|\x90\x84\x90`\x04\x01a/\x15V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xC1\x91\x90\x81\x01\x90a/(V[\x80Qa\x04\xD5\x91`\x1C\x91` \x90\x91\x01\x90a(\xDCV[PPV[`\0a\x05\x90`\x1C\x80Ta\x04\xEB\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x17\x90a/\x9FV[\x80\x15a\x05dW\x80`\x1F\x10a\x059Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05dV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x0B\x9C\xDB\x1B\xDD\x14\x9B\xDB\xDD`\xBA\x1B\x81RPa((V[\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x05\xA7\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xD3\x90a/\x9FV[\x80\x15a\x06 W\x80`\x1F\x10a\x05\xF5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06 V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x99XX\xDB\xDB\x94\xDD\x18]\x19T\x9B\xDB\xDD`\x82\x1B\x81RPa((V[`\0a\x05\x90`\x1C\x80Ta\x06e\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x91\x90a/\x9FV[\x80\x15a\x06\xDEW\x80`\x1F\x10a\x06\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xDEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u\x0B\x9B\x18]\x19\\\xDD\x10\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`R\x1B\x81RPa((V[```\x14\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QW[PPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x07\x8B\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xB7\x90a/\x9FV[\x80\x15a\x08\x04W\x80`\x1F\x10a\x07\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`\x82\x1B\x81RPa((V[a\x08?a)`V[`\0[`,\x81\x10\x15a\nTW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xBD\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x08\xCD\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x08\xEA\x91` \x01a/\xF9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\t\x0E\x92\x91\x90a(\xDCV[Pa\n-`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tK\x90a/\x9FV[\x80\x15a\t\x98W\x80`\x1F\x10a\tmWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x98V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t{W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\x1D\x80Ta\t\xAA\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xD6\x90a/\x9FV[\x80\x15a\n#W\x80`\x1F\x10a\t\xF8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n#V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa((V[`j\x82`,\x81\x10a\n@Wa\n@a0>V[\x01U\x80a\nL\x81a0TV[\x91PPa\x08BV[P`@\x80Qa\x05\x80\x81\x01\x91\x82\x90R\x90`j\x90`,\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\nlWPPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x0B\xABW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x0B\x1E\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0BJ\x90a/\x9FV[\x80\x15a\x0B\x97W\x80`\x1F\x10a\x0BlWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BzW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\n\xFFV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\n\xADV[PPPP\x90P\x90V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CJW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Cr\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x0C\x82\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0C\x9F\x91` \x01a0}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x0C\xC3\x92\x91\x90a(\xDCV[Pa\x0C\xD4`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x0C\xE6Wa\x0C\xE6a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0C\xFB\x81a0TV[\x91PPa\x0B\xF7V[P\x91\x90PV[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01\x84\x806\x837\x01\x90PP\x90P`\0[`\x03\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\xAD\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\r\xBD\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xDA\x91` \x01a0\xC2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\r\xFE\x92\x91\x90a(\xDCV[Pa\x0E\x0F`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x0E!Wa\x0E!a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0E6\x81a0TV[\x91PPa\r2V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QWPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x0F\x10\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F<\x90a/\x9FV[\x80\x15a\x0F\x89W\x80`\x1F\x10a\x0F^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0FlW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x05\xCCM\x8D\xECmi\x0C\xAC,\x8C\xAEJM\xED\xEE\x89-\xCC\x8C\xAF`[\x1B\x81RPa(\xA6V[`\0a\x05\x90`\x1C\x80Ta\x0F\xD3\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xFF\x90a/\x9FV[\x80\x15a\x10LW\x80`\x1F\x10a\x10!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10LV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x0B\x98\x98[\x18[\x98\xD9T\x9B\xDB\xDD`\xA2\x1B\x81RPa((V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P`\0[`\x08\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\"\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x112\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11O\x91` \x01a1\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x11s\x92\x91\x90a(\xDCV[Pa\x11\x84`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x11\x96Wa\x11\x96a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x11\xAB\x81a0TV[\x91PPa\x10\xA7V[a\x11\xBBa)\x7FV[`\0[`.\x81\x10\x15a\x12\xC2W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x129\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x12I\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x12f\x91` \x01a1QV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x12\x8A\x92\x91\x90a(\xDCV[Pa\x12\x9B`\x1C\x80Ta\t\x1F\x90a/\x9FV[`<\x82`.\x81\x10a\x12\xAEWa\x12\xAEa0>V[\x01U\x80a\x12\xBA\x81a0TV[\x91PPa\x11\xBEV[P`@\x80Qa\x05\xC0\x81\x01\x91\x82\x90R`<\x80T\x82R\x90\x91`.\x90`=` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x13\x04\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x130\x90a/\x9FV[\x80\x15a\x13}W\x80`\x1F\x10a\x13RWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13}V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xCE\xED.\x8D\x0C\x8EL.\xEC-\x89-\xCC\x8C\xAF`\x83\x1B\x81RPa(\xA6V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14~W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14@W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x13\xD4V[`\0a\x05\x90`\x1C\x80Ta\x14\xA8\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xD4\x90a/\x9FV[\x80\x15a\x15!W\x80`\x1F\x10a\x14\xF6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15!V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x04W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x0B\x9C\xDB\x1B\xDD`\xDA\x1B\x81RPa(\xA6V[`\0a\x05\x90`\x1C\x80Ta\x15[\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x87\x90a/\x9FV[\x80\x15a\x15\xD4W\x80`\x1F\x10a\x15\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xD4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xB7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x05\xCE\xCC-\x8D,\x8C.\x8D\xEEI-\xCC\x8C\xAF`\x8B\x1B\x81RPa(\xA6V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x16I\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16u\x90a/\x9FV[\x80\x15a\x16\xC2W\x80`\x1F\x10a\x16\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x16*V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P`\0[`.\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17}\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x17\x8D\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x17\xAA\x91` \x01a1\x89V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x17\xCE\x92\x91\x90a(\xDCV[Pa\x17\xDF`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x17\xF1Wa\x17\xF1a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x18\x06\x81a0TV[\x91PPa\x17\x02V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x18\xDCW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18\x9EW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x182V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\x80\x806\x837\x01\x90PP\x90P`\0[`\x04\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19\x99\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x19\xA9\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19\xC6\x91` \x01a1\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x19\xEA\x92\x91\x90a(\xDCV[Pa\x19\xFB`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x1A\rWa\x1A\ra0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x1A\"\x81a0TV[\x91PPa\x19\x1EV[a\x1A2a)\x9EV[`\0[`\x03\x81\x10\x15a\x1B9W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\xB0\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1A\xC0\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1A\xDD\x91` \x01a2\x01V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1B\x01\x92\x91\x90a(\xDCV[Pa\x1B\x12`\x1C\x80Ta\t\x1F\x90a/\x9FV[`0\x82`\x03\x81\x10a\x1B%Wa\x1B%a0>V[\x01U\x80a\x1B1\x81a0TV[\x91PPa\x1A5V[P`@\x80Q``\x81\x01\x91\x82\x90R`0\x80T\x82R\x90\x91`\x03\x90`1` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[a\x1Bpa)\xBCV[`\0[`\x12\x81\x10\x15a\x1CwW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xEE\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1B\xFE\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1C\x1B\x91` \x01a24V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1C?\x92\x91\x90a(\xDCV[Pa\x1CP`\x1C\x80Ta\t\x1F\x90a/\x9FV[`\x1E\x82`\x12\x81\x10a\x1CcWa\x1Cca0>V[\x01U\x80a\x1Co\x81a0TV[\x91PPa\x1BsV[P`@\x80Qa\x02@\x81\x01\x91\x82\x90R`\x1E\x80T\x82R\x90\x91`\x12\x90`\x1F` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`@\x80Q`,\x80\x82Ra\x05\xA0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x05\x80\x806\x837\x01\x90PP\x90P`\0[`,\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1DN\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1D^\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1D{\x91` \x01a2aV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1D\x9F\x92\x91\x90a(\xDCV[Pa\x1D\xB0`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\x1D\xC2Wa\x1D\xC2a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x1D\xD7\x81a0TV[\x91PPa\x1C\xD3V[a\x1D\xE7a)\xDBV[`\0[`\t\x81\x10\x15a\x1E\xEEW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Ee\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\x1Eu\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\x92\x91` \x01a2\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\x1E\xB6\x92\x91\x90a(\xDCV[Pa\x1E\xC7`\x1C\x80Ta\t\x1F\x90a/\x9FV[`3\x82`\t\x81\x10a\x1E\xDAWa\x1E\xDAa0>V[\x01U\x80a\x1E\xE6\x81a0TV[\x91PPa\x1D\xEAV[P`@\x80Qa\x01 \x81\x01\x91\x82\x90R`3\x80T\x82R\x90\x91`\t\x90`4` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta\x1F0\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\\\x90a/\x9FV[\x80\x15a\x1F\xA9W\x80`\x1F\x10a\x1F~Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xA9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x0B\x9D\x1A[Y\\\xDD\x18[\\\x14\x9B\xDB\xDD`\x92\x1B\x81RPa((V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xC2W\x83\x82\x90`\0R` `\0 \x01\x80Ta \x1D\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta I\x90a/\x9FV[\x80\x15a \x96W\x80`\x1F\x10a kWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \x96V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a yW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xFEV[`\x07T`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a \xCCWP`\x07Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0`\0\x80Q` a4\t\x839\x81Q\x91R;\x15a!\xC6W`@\x80Q`\0\x80Q` a4\t\x839\x81Q\x91R` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a!N\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01a2\xD2V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra!h\x91a3\x03V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a!\xA5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a!\xAAV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a!\xC2\x91\x90a3\x1FV[\x91PP[\x91\x90PV[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P`\0[`\x05\x81\x10\x15a\r\x03W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"HW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"p\x91\x90\x81\x01\x90a/(V[`@Q` \x01a\"\x80\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"\x9D\x91` \x01a3AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a\"\xC1\x92\x91\x90a(\xDCV[Pa\"\xD2`\x1C\x80Ta\t\x1F\x90a/\x9FV[\x82\x82\x81Q\x81\x10a\"\xE4Wa\"\xE4a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\"\xF9\x81a0TV[\x91PPa!\xF5V[`\0a\x05\x90`\x1C\x80Ta#\x13\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#?\x90a/\x9FV[\x80\x15a#\x8CW\x80`\x1F\x10a#aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x8CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F.historicalSummaryIndex\0\0\0\0\0\0\0\0\0\x81RPa(\xA6V[a#\xD4a)\xFAV[`\0[`\x04\x81\x10\x15a$\xDBW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$R\x91\x90\x81\x01\x90a/(V[`@Q` \x01a$b\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra$\x7F\x91` \x01a3gV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a$\xA3\x92\x91\x90a(\xDCV[Pa$\xB4`\x1C\x80Ta\t\x1F\x90a/\x9FV[`\x9D\x82`\x04\x81\x10a$\xC7Wa$\xC7a0>V[\x01U\x80a$\xD3\x81a0TV[\x91PPa#\xD7V[P`@\x80Q`\x80\x81\x01\x91\x82\x90R`\x9D\x80T\x82R\x90\x91`\x04\x90`\x9E` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta%\x1C\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%H\x90a/\x9FV[\x80\x15a%\x95W\x80`\x1F\x10a%jWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\x95V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x0B\x99^\x19X\xDD]\x1A[\xDB\x94\x18^[\x1B\xD8Y\x14\x9B\xDB\xDD`Z\x1B\x81RPa((V[a%\xD5a*\x18V[`\0[`\x07\x81\x10\x15a&\xDCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x80Q` a4\t\x839\x81Q\x91R\x90ci\0\xA3\xAE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&S\x91\x90\x81\x01\x90a/(V[`@Q` \x01a&c\x91\x90a/\xD4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra&\x80\x91` \x01a3\x92V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`\x1D\x90\x80Q\x90` \x01\x90a&\xA4\x92\x91\x90a(\xDCV[Pa&\xB5`\x1C\x80Ta\t\x1F\x90a/\x9FV[`\x96\x82`\x07\x81\x10a&\xC8Wa&\xC8a0>V[\x01U\x80a&\xD4\x81a0TV[\x91PPa%\xD8V[P`@\x80Q`\xE0\x81\x01\x91\x82\x90R`\x96\x80T\x82R\x90\x91`\x07\x90`\x97` \x85\x01\x80\x83\x11a\nlWPPPPP\x90P\x90V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07oW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07QWPPPPP\x90P\x90V[`\0a\x05\x90`\x1C\x80Ta'}\x90a/\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'\xA9\x90a/\x9FV[\x80\x15a'\xF6W\x80`\x1F\x10a'\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\xF6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r.ValidatorFields[0]`h\x1B\x81RP[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R`\0\x90`\0\x80Q` a4\t\x839\x81Q\x91R\x90c\x17w\xE5\x9D\x90a(^\x90\x86\x90\x86\x90`\x04\x01a3\xCAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a({W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x9F\x91\x90a3\xEFV[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R`\0\x90`\0\x80Q` a4\t\x839\x81Q\x91R\x90c\xAD\xDD\xE2\xB6\x90a(^\x90\x86\x90\x86\x90`\x04\x01a3\xCAV[\x82\x80Ta(\xE8\x90a/\x9FV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a)\nW`\0\x85Ua)PV[\x82`\x1F\x10a)#W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ua)PV[\x82\x80\x01`\x01\x01\x85U\x82\x15a)PW\x91\x82\x01[\x82\x81\x11\x15a)PW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a)5V[Pa)\\\x92\x91Pa*6V[P\x90V[`@Q\x80a\x05\x80\x01`@R\x80`,\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\xC0\x01`@R\x80`.\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02@\x01`@R\x80`\x12\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80`\x07\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a)\\W`\0\x81U`\x01\x01a*7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a*\x8AWa*\x8Aa*KV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*\xACWa*\xACa*KV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15a*\xCCW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xE3W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a*\xF4W`\0\x80\xFD[\x805a+\x07a+\x02\x82a*\x92V[a*aV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a+\x1CW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a+{W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a+VV[P\x90\x96\x95PPPPPPV[a\x05\x80\x81\x01\x81\x83`\0[`,\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\x91V[PPP\x92\x91PPV[`\0[\x83\x81\x10\x15a+\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01a+\xBCV[\x83\x81\x11\x15a+\xE3W`\0\x84\x84\x01R[PPPPV[`\0\x81Q\x80\x84Ra,\x01\x81` \x86\x01` \x86\x01a+\xB9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a,\xC5W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a,\xAFW`_\x19\x89\x85\x03\x01\x83Ra,\x9D\x84\x86Qa+\xE9V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a,\x81V[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a,<V[P\x91\x9A\x99PPPPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a+{W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a,\xF1V[a\x05\xC0\x81\x01\x81\x83`\0[`.\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\x17V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a-\xDAW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a-\xC5W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a-\x9BV[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a-^V[P\x91\x99\x98PPPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a.>W`?\x19\x88\x86\x03\x01\x84Ra.,\x85\x83Qa+\xE9V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a.\x10V[P\x92\x97\x96PPPPPPPV[``\x81\x01\x81\x83`\0[`\x03\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.TV[a\x02@\x81\x01\x81\x83`\0[`\x12\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.}V[a\x01 \x81\x01\x81\x83`\0[`\t\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xA6V[`\x80\x81\x01\x81\x83`\0[`\x04\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xCEV[`\xE0\x81\x01\x81\x83`\0[`\x07\x81\x10\x15a+\xB0W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\xF6V[` \x81R`\0a(\x9F` \x83\x01\x84a+\xE9V[`\0` \x82\x84\x03\x12\x15a/:W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/QW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a/bW`\0\x80\xFD[\x80Qa/pa+\x02\x82a*\x92V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a/\x85W`\0\x80\xFD[a/\x96\x82` \x83\x01` \x86\x01a+\xB9V[\x95\x94PPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a/\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\r\x03WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x82Qa/\xE6\x81\x84` \x87\x01a+\xB9V[`]`\xF8\x1B\x92\x01\x91\x82RP`\x01\x01\x91\x90PV[\x7F.HistoricalSummaryProof[\0\0\0\0\0\0\0\0\x81R`\0\x82Qa01\x81`\x18\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x18\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a0vWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[\x7F.WithdrawalCredentialProof[\0\0\0\0\0\x81R`\0\x82Qa0\xB5\x81`\x1B\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x1B\x01\x92\x91PPV[\x7F.StateRootAgainstLatestBlockHead\x81RgerProof[`\xC0\x1B` \x82\x01R`\0\x82Qa1\x0B\x81`(\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`(\x01\x92\x91PPV[p.ValidatorFields[`x\x1B\x81R`\0\x82Qa1D\x81`\x11\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x11\x01\x92\x91PPV[o.ValidatorProof[`\x80\x1B\x81R`\0\x82Qa1|\x81`\x10\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x10\x01\x92\x91PPV[u.ValidatorFieldsProof[`P\x1B\x81R`\0\x82Qa1\xBA\x81`\x16\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x16\x01\x92\x91PPV[q.WithdrawalFields[`p\x1B\x81R`\0\x82Qa1\xF4\x81`\x12\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x12\x01\x92\x91PPV[j.SlotProof[`\xA8\x1B\x81R`\0\x82Qa2'\x81`\x0B\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x0B\x01\x92\x91PPV[q.BlockHeaderProof[`p\x1B\x81R`\0\x82Qa1\xF4\x81`\x12\x85\x01` \x87\x01a+\xB9V[\x7F.ValidatorBalanceProof[\0\0\0\0\0\0\0\0\0\x81R`\0\x82Qa2\x99\x81`\x17\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x17\x01\x92\x91PPV[p.WithdrawalProof[`x\x1B\x81R`\0\x82Qa1D\x81`\x11\x85\x01` \x87\x01a+\xB9V[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90a2\xF5\x81`\x04\x85\x01` \x87\x01a+\xB9V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qa3\x15\x81\x84` \x87\x01a+\xB9V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a31W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a(\x9FW`\0\x80\xFD[j.slotProof[`\xA8\x1B\x81R`\0\x82Qa2'\x81`\x0B\x85\x01` \x87\x01a+\xB9V[o.TimestampProof[`\x80\x1B\x81R`\0\x82Qa1|\x81`\x10\x85\x01` \x87\x01a+\xB9V[\x7F.ExecutionPayloadProof[\0\0\0\0\0\0\0\0\0\x81R`\0\x82Qa2\x99\x81`\x17\x85\x01` \x87\x01a+\xB9V[`@\x81R`\0a3\xDD`@\x83\x01\x85a+\xE9V[\x82\x81\x03` \x84\x01Ra/\x96\x81\x85a+\xE9V[`\0` \x82\x84\x03\x12\x15a4\x01W`\0\x80\xFD[PQ\x91\x90PV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 W\xAEg\xFE\xE4\xFB=\x1Cmq\xCDkm\xD3u\x96HlZ\xB1\xB5\xFD\xAE\x13\xD9Qpg\xB5\xB9\xD1\\dsolcC\0\x08\x0C\x003",
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
    /**Function with signature `getBalanceRoot()` and selector `0x4656fdb5`.
```solidity
function getBalanceRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceRootCall {}
    ///Container type for the return parameters of the [`getBalanceRoot()`](getBalanceRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceRootReturn {
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
            impl ::core::convert::From<getBalanceRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceRootCall {
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
            impl ::core::convert::From<getBalanceRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalanceRoot()";
            const SELECTOR: [u8; 4] = [70u8, 86u8, 253u8, 181u8];
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
    /**Function with signature `getBalanceUpdateSlotProof()` and selector `0xc3da8ae9`.
```solidity
function getBalanceUpdateSlotProof() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceUpdateSlotProofCall {}
    ///Container type for the return parameters of the [`getBalanceUpdateSlotProof()`](getBalanceUpdateSlotProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceUpdateSlotProofReturn {
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
            impl ::core::convert::From<getBalanceUpdateSlotProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceUpdateSlotProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceUpdateSlotProofCall {
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
            impl ::core::convert::From<getBalanceUpdateSlotProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceUpdateSlotProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceUpdateSlotProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceUpdateSlotProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceUpdateSlotProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalanceUpdateSlotProof()";
            const SELECTOR: [u8; 4] = [195u8, 218u8, 138u8, 233u8];
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
function getBeaconStateRoot() external view returns (bytes32);
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
function getBlockRoot() external view returns (bytes32);
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
function getBlockRootIndex() external view returns (uint256);
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
function getExecutionPayloadRoot() external view returns (bytes32);
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
function getHistoricalSummaryIndex() external view returns (uint256);
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
function getLatestBlockRoot() external view returns (bytes32);
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
function getSlot() external view returns (uint256);
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
function getSlotRoot() external view returns (bytes32);
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
function getStateRootProof() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateRootProofCall {}
    ///Container type for the return parameters of the [`getStateRootProof()`](getStateRootProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateRootProofReturn {
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
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
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
    /**Function with signature `getTimestampProof()` and selector `0xd911b683`.
```solidity
function getTimestampProof() external returns (bytes32[4] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofCall {}
    ///Container type for the return parameters of the [`getTimestampProof()`](getTimestampProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofReturn {
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
            impl ::core::convert::From<getTimestampProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofCall {
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
            impl ::core::convert::From<getTimestampProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    4usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampProof()";
            const SELECTOR: [u8; 4] = [217u8, 17u8, 182u8, 131u8];
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
function getTimestampRoot() external view returns (bytes32);
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
    /**Function with signature `getValidatorBalanceProof()` and selector `0xa6b7a848`.
```solidity
function getValidatorBalanceProof() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorBalanceProofCall {}
    ///Container type for the return parameters of the [`getValidatorBalanceProof()`](getValidatorBalanceProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorBalanceProofReturn {
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
            impl ::core::convert::From<getValidatorBalanceProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorBalanceProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorBalanceProofCall {
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
            impl ::core::convert::From<getValidatorBalanceProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorBalanceProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorBalanceProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorBalanceProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorBalanceProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorBalanceProof()";
            const SELECTOR: [u8; 4] = [166u8, 183u8, 168u8, 72u8];
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
function getValidatorIndex() external view returns (uint256);
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
function getValidatorPubkeyHash() external view returns (bytes32);
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
function getWithdrawalCredentialProof() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalCredentialProofCall {}
    ///Container type for the return parameters of the [`getWithdrawalCredentialProof()`](getWithdrawalCredentialProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalCredentialProofReturn {
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
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
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
function getWithdrawalIndex() external view returns (uint256);
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
    /**Function with signature `getWithdrawalProof()` and selector `0xab72161c`.
```solidity
function getWithdrawalProof() external returns (bytes32[9] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofCall {}
    ///Container type for the return parameters of the [`getWithdrawalProof()`](getWithdrawalProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofReturn {
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
            impl ::core::convert::From<getWithdrawalProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofCall {
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
            impl ::core::convert::From<getWithdrawalProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    9usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalProof()";
            const SELECTOR: [u8; 4] = [171u8, 114u8, 22u8, 28u8];
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        getBalanceRoot(getBalanceRootCall),
        getBalanceUpdateSlotProof(getBalanceUpdateSlotProofCall),
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
        getTimestampProof(getTimestampProofCall),
        getTimestampRoot(getTimestampRootCall),
        getValidatorBalanceProof(getValidatorBalanceProofCall),
        getValidatorFields(getValidatorFieldsCall),
        getValidatorFieldsProof(getValidatorFieldsProofCall),
        getValidatorIndex(getValidatorIndexCall),
        getValidatorProof(getValidatorProofCall),
        getValidatorPubkeyHash(getValidatorPubkeyHashCall),
        getWithdrawalCredentialProof(getWithdrawalCredentialProofCall),
        getWithdrawalFields(getWithdrawalFieldsCall),
        getWithdrawalIndex(getWithdrawalIndexCall),
        getWithdrawalProof(getWithdrawalProofCall),
        setJSON(setJSONCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
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
            [30u8, 215u8, 131u8, 28u8],
            [39u8, 83u8, 120u8, 177u8],
            [40u8, 114u8, 226u8, 12u8],
            [42u8, 222u8, 56u8, 128u8],
            [47u8, 209u8, 121u8, 60u8],
            [52u8, 227u8, 217u8, 14u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [66u8, 134u8, 71u8, 52u8],
            [70u8, 86u8, 253u8, 181u8],
            [76u8, 32u8, 245u8, 16u8],
            [76u8, 56u8, 249u8, 19u8],
            [100u8, 243u8, 142u8, 215u8],
            [102u8, 217u8, 169u8, 160u8],
            [108u8, 135u8, 124u8, 132u8],
            [118u8, 90u8, 166u8, 6u8],
            [133u8, 34u8, 108u8, 129u8],
            [137u8, 56u8, 147u8, 202u8],
            [145u8, 106u8, 23u8, 198u8],
            [149u8, 11u8, 182u8, 130u8],
            [157u8, 228u8, 169u8, 179u8],
            [165u8, 7u8, 116u8, 41u8],
            [166u8, 183u8, 168u8, 72u8],
            [171u8, 114u8, 22u8, 28u8],
            [179u8, 128u8, 97u8, 191u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [195u8, 218u8, 138u8, 233u8],
            [214u8, 70u8, 28u8, 187u8],
            [217u8, 17u8, 182u8, 131u8],
            [217u8, 68u8, 71u8, 47u8],
            [219u8, 54u8, 74u8, 64u8],
            [226u8, 12u8, 159u8, 113u8],
            [241u8, 72u8, 8u8, 44u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProofParsingCalls {
        const NAME: &'static str = "ProofParsingCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 39usize;
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
                Self::getBalanceRoot(_) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBalanceUpdateSlotProof(_) => {
                    <getBalanceUpdateSlotProofCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getTimestampProof(_) => {
                    <getTimestampProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampRoot(_) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorBalanceProof(_) => {
                    <getValidatorBalanceProofCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getWithdrawalProof(_) => {
                    <getWithdrawalProofCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetInterfaces)
                    }
                    targetInterfaces
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
                    fn getBalanceRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBalanceRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBalanceRoot)
                    }
                    getBalanceRoot
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
                    fn getValidatorBalanceProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorBalanceProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorBalanceProof)
                    }
                    getValidatorBalanceProof
                },
                {
                    fn getWithdrawalProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalProof)
                    }
                    getWithdrawalProof
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
                    fn getBalanceUpdateSlotProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBalanceUpdateSlotProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBalanceUpdateSlotProof)
                    }
                    getBalanceUpdateSlotProof
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
                    fn getTimestampProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampProof)
                    }
                    getTimestampProof
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
                Self::getBalanceRoot(inner) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBalanceUpdateSlotProof(inner) => {
                    <getBalanceUpdateSlotProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getTimestampProof(inner) => {
                    <getTimestampProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampRoot(inner) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorBalanceProof(inner) => {
                    <getValidatorBalanceProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getWithdrawalProof(inner) => {
                    <getWithdrawalProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getBalanceRoot(inner) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBalanceUpdateSlotProof(inner) => {
                    <getBalanceUpdateSlotProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getTimestampProof(inner) => {
                    <getTimestampProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getValidatorBalanceProof(inner) => {
                    <getValidatorBalanceProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getWithdrawalProof(inner) => {
                    <getWithdrawalProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        ///Creates a new call builder for the [`getBalanceRoot`] function.
        pub fn getBalanceRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBalanceRootCall, N> {
            self.call_builder(&getBalanceRootCall {})
        }
        ///Creates a new call builder for the [`getBalanceUpdateSlotProof`] function.
        pub fn getBalanceUpdateSlotProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBalanceUpdateSlotProofCall, N> {
            self.call_builder(&getBalanceUpdateSlotProofCall {})
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
        ///Creates a new call builder for the [`getTimestampProof`] function.
        pub fn getTimestampProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampProofCall, N> {
            self.call_builder(&getTimestampProofCall {})
        }
        ///Creates a new call builder for the [`getTimestampRoot`] function.
        pub fn getTimestampRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampRootCall, N> {
            self.call_builder(&getTimestampRootCall {})
        }
        ///Creates a new call builder for the [`getValidatorBalanceProof`] function.
        pub fn getValidatorBalanceProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorBalanceProofCall, N> {
            self.call_builder(&getValidatorBalanceProofCall {})
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
        ///Creates a new call builder for the [`getWithdrawalProof`] function.
        pub fn getWithdrawalProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalProofCall, N> {
            self.call_builder(&getWithdrawalProofCall {})
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
