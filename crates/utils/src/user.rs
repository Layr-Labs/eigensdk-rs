///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManagerTypes {
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] scaledShares; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IDelegationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] scaledShares; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub staker: alloy::sol_types::private::Address,
        pub delegatedTo: alloy::sol_types::private::Address,
        pub withdrawer: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub startBlock: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub scaledShares: alloy::sol_types::private::Vec<
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<Withdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: Withdrawal) -> Self {
                (
                    value.staker,
                    value.delegatedTo,
                    value.withdrawer,
                    value.nonce,
                    value.startBlock,
                    value.strategies,
                    value.scaledShares,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Withdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    staker: tuple.0,
                    delegatedTo: tuple.1,
                    withdrawer: tuple.2,
                    nonce: tuple.3,
                    startBlock: tuple.4,
                    strategies: tuple.5,
                    scaledShares: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Withdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Withdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatedTo,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.scaledShares),
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
        impl alloy_sol_types::SolType for Withdrawal {
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
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] scaledShares)",
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
                            &self.staker,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatedTo,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startBlock)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.scaledShares)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Withdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.staker,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatedTo,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startBlock,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.scaledShares,
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
                    &rust.staker,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatedTo,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startBlock,
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
                    &rust.scaledShares,
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
    /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDelegationManagerTypesInstance<T, P, N> {
        IDelegationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDelegationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDelegationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDelegationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDelegationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDelegationManagerTypesInstance")
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
    > IDelegationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IDelegationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDelegationManagerTypesInstance<T, P, N> {
            IDelegationManagerTypesInstance {
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
    > IDelegationManagerTypesInstance<T, P, N> {
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
    > IDelegationManagerTypesInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
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
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        pub artifact: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
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
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
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
library IDelegationManagerTypes {
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startBlock;
        address[] strategies;
        uint256[] scaledShares;
    }
}

library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface User {
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

    constructor(string name);

    receive() external payable;

    function IS_TEST() external view returns (bool);
    function NAME() external view returns (string memory);
    function completeCheckpoint() external;
    function completeWithdrawalAsShares(IDelegationManagerTypes.Withdrawal memory withdrawal) external returns (address[] memory);
    function completeWithdrawalAsTokens(IDelegationManagerTypes.Withdrawal memory withdrawal) external returns (address[] memory);
    function completeWithdrawalsAsShares(IDelegationManagerTypes.Withdrawal[] memory withdrawals) external returns (address[][] memory);
    function completeWithdrawalsAsTokens(IDelegationManagerTypes.Withdrawal[] memory withdrawals) external returns (address[][] memory);
    function delegateTo(address operator) external;
    function depositIntoEigenlayer(address[] memory strategies, uint256[] memory tokenBalances) external;
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function exitValidators(uint40[] memory _validators) external returns (uint64 exitedBalanceGwei);
    function failed() external view returns (bool);
    function forceUndelegate(address staker) external returns (IDelegationManagerTypes.Withdrawal[] memory);
    function getActiveValidators() external view returns (uint40[] memory);
    function pod() external view returns (address);
    function queueWithdrawals(address[] memory strategies, uint256[] memory depositShares) external returns (IDelegationManagerTypes.Withdrawal[] memory);
    function registerAsOperator() external;
    function startCheckpoint() external;
    function startValidators() external returns (uint40[] memory, uint64);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function undelegate() external returns (IDelegationManagerTypes.Withdrawal[] memory);
    function updateBalances(address[] memory strategies, int256[] memory tokenDeltas) external;
    function verifyStaleBalance(uint40 validatorIndex) external;
    function verifyWithdrawalCredentials(uint40[] memory _validators) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
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
    "name": "NAME",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "completeCheckpoint",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeWithdrawalAsShares",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeWithdrawalAsTokens",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManagerTypes.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeWithdrawalsAsShares",
    "inputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[][]",
        "internalType": "contract IERC20[][]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeWithdrawalsAsTokens",
    "inputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[][]",
        "internalType": "contract IERC20[][]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateTo",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "contract User"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "depositIntoEigenlayer",
    "inputs": [
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "tokenBalances",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
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
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
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
    "name": "exitValidators",
    "inputs": [
      {
        "name": "_validators",
        "type": "uint40[]",
        "internalType": "uint40[]"
      }
    ],
    "outputs": [
      {
        "name": "exitedBalanceGwei",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "forceUndelegate",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "contract User"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getActiveValidators",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint40[]",
        "internalType": "uint40[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pod",
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
    "name": "queueWithdrawals",
    "inputs": [
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "depositShares",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerAsOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startCheckpoint",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "startValidators",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint40[]",
        "internalType": "uint40[]"
      },
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
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
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
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
    "name": "undelegate",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IDelegationManagerTypes.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "scaledShares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateBalances",
    "inputs": [
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "tokenDeltas",
        "type": "int256[]",
        "internalType": "int256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyStaleBalance",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyWithdrawalCredentials",
    "inputs": [
      {
        "name": "_validators",
        "type": "uint40[]",
        "internalType": "uint40[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod User {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052600c8054600160ff199091168117909155601f80546001600160a81b031916747109709ecfa91a80626ff3989d68f67f5b1dd12d011790556028805463ffffffff19169091179055348015610057575f5ffd5b50604051615c15380380615c15833981016040819052610076916103ca565b5f339050806001600160a01b031663ea4d3c9b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100da9190610491565b602080546001600160a01b0319166001600160a01b0392831617815560408051630736e1c760e31b81529051928416926339b70e38926004808401939192918290030181865afa158015610130573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101549190610491565b60215f6101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316634665bcda6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101b5573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101d99190610491565b60225f6101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316633dfb40e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561023a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061025e9190610491565b60235f6101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b03166322c0350b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102bf573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102e39190610491565b602480546001600160a01b0319166001600160a01b039290921691909117905561030b61031f565b60256103178382610537565b5050506105f1565b60225f9054906101000a90046001600160a01b03166001600160a01b03166384d810626040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610370573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103949190610491565b602680546001600160a01b0319166001600160a01b0392909216919091179055565b634e487b7160e01b5f52604160045260245ffd5b5f602082840312156103da575f5ffd5b81516001600160401b038111156103ef575f5ffd5b8201601f810184136103ff575f5ffd5b80516001600160401b03811115610418576104186103b6565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610446576104466103b6565b60405281815282820160200186101561045d575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b6001600160a01b038116811461048e575f5ffd5b50565b5f602082840312156104a1575f5ffd5b81516104ac8161047a565b9392505050565b600181811c908216806104c757607f821691505b6020821081036104e557634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561053257805f5260205f20601f840160051c810160208510156105105750805b601f840160051c820191505b8181101561052f575f815560010161051c565b50505b505050565b81516001600160401b03811115610550576105506103b6565b6105648161055e84546104b3565b846104eb565b6020601f821160018114610596575f831561057f5750848201515b5f19600385901b1c1916600184901b17845561052f565b5f84815260208120601f198516915b828110156105c557878501518255602094850194600190920191016105a5565b50848210156105e257868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b615617806105fe5f395ff3fe6080604052600436106101de575f3560e01c8063841c1299116100fd578063ac637c7a11610092578063d6c10daf11610062578063d6c10daf14610575578063e20c9f7114610589578063f234c1bd1461059d578063fa7626d4146105bf575f5ffd5b8063ac637c7a1461050a578063b0464fdc14610529578063b5508aa91461053d578063ba414fa614610551575f5ffd5b806392ab89bb116100cd57806392ab89bb1461047d5780639de7025814610491578063a3f4df7e146104b2578063a88dbb36146104d3575f5ffd5b8063841c12991461040857806385226c811461042757806390b5162514610448578063916a17c61461045c575f5ffd5b80633d8c08d41161017357806346a5be0d1161014357806346a5be0d1461038a57806366d9a9a0146103a9578063695f4ae1146103ca5780636d336f58146103e9575f5ffd5b80633d8c08d4146102ff5780633e5e3c23146103365780633f7286f41461034a578063401be65e1461035e575f5ffd5b80632a34ade8116101ae5780632a34ade81461027f5780632ade388014610293578063344e1383146102b4578063391cc9f6146102e0575f5ffd5b8063071c25b7146101e95780631ed7831c1461020a57806320a545d91461023457806323e4817514610253575f5ffd5b366101e557005b5f5ffd5b3480156101f4575f5ffd5b50610208610203366004613ccd565b6105d8565b005b348015610215575f5ffd5b5061021e61079e565b60405161022b9190613cef565b60405180910390f35b34801561023f575f5ffd5b5061020861024e366004613ed7565b6107fe565b34801561025e575f5ffd5b5061027261026d366004613fee565b610b09565b60405161022b919061414b565b34801561028a575f5ffd5b50610208610e59565b34801561029e575f5ffd5b506102a7610fc0565b60405161022b919061422a565b3480156102bf575f5ffd5b506102d36102ce36600461437b565b6110fc565b60405161022b919061442a565b3480156102eb575f5ffd5b506102726102fa366004614481565b611255565b34801561030a575f5ffd5b5061031e61031936600461449c565b6113d0565b6040516001600160401b03909116815260200161022b565b348015610341575f5ffd5b5061021e611475565b348015610355575f5ffd5b5061021e6114d3565b348015610369575f5ffd5b5061037d610378366004614535565b611531565b60405161022b919061456e565b348015610395575f5ffd5b506102d36103a436600461437b565b6115e8565b3480156103b4575f5ffd5b506103bd611737565b60405161022b91906145ba565b3480156103d5575f5ffd5b5061037d6103e4366004614535565b61189b565b3480156103f4575f5ffd5b50610208610403366004613fee565b611951565b348015610413575f5ffd5b5061020861042236600461449c565b611c4d565b348015610432575f5ffd5b5061043b611d0d565b60405161022b9190614638565b348015610453575f5ffd5b50610208611dd8565b348015610467575f5ffd5b50610470611e88565b60405161022b919061464a565b348015610488575f5ffd5b50610272611f69565b34801561049c575f5ffd5b506104a5612284565b60405161022b91906146f8565b3480156104bd575f5ffd5b506104c6612426565b60405161022b919061470a565b3480156104de575f5ffd5b506026546104f2906001600160a01b031681565b6040516001600160a01b03909116815260200161022b565b348015610515575f5ffd5b50610208610524366004614481565b6124ad565b348015610534575f5ffd5b506104706125fa565b348015610548575f5ffd5b5061043b6126db565b34801561055c575f5ffd5b506105656127a6565b604051901515815260200161022b565b348015610580575f5ffd5b50610208612846565b348015610594575f5ffd5b5061021e6128f7565b3480156105a8575f5ffd5b506105b1612955565b60405161022b92919061471c565b3480156105ca575f5ffd5b50601f546105659060ff1681565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610629573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061064d9190614746565b50610681604051806040016040528060128152602001717665726966795374616c6542616c616e636560701b815250612a0e565b602480546040516308fa0b1360e21b815264ffffffffff841660048201525f926001600160a01b03909216916323e82c4c91015f60405180830381865afa1580156106ce573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526106f59190810190614893565b6026548151602083015160408085015190516301c8abe960e11b81529495506001600160a01b039093169363039157d2936107349392916004016149b3565b5f604051808303815f87803b15801561074b575f5ffd5b505af192505050801561075c575060015b61079a573d808015610789576040519150601f19603f3d011682016040523d82523d5f602084013e61078e565b606091505b5061079881612a6b565b505b5050565b606060168054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116107d6575b5050505050905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af115801561084f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108739190614746565b506108a36040518060400160405280600e81526020016d75706461746542616c616e63657360901b815250612a0e565b5f5b8251811015610798575f8382815181106108c1576108c1614a14565b602002602001015190505f8383815181106108de576108de614a14565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b0316036109a35761091d612ac7565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa15801561096d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109919190614746565b1561099e5761099e612b56565b610aff565b5f8190505f836001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109e4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a089190614a28565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015610a5a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a7e9190614a43565b506021546040516373d0285560e11b81526001600160a01b0386811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303815f875af1158015610ad7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610afb9190614746565b5050505b50506001016108a5565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015610b52573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b769190614746565b50610ba86040518060400160405280601081526020016f71756575655769746864726177616c7360801b815250612a0e565b602054604051631976849960e21b81523060048201525f916001600160a01b0316906365da126490602401602060405180830381865afa158015610bee573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c129190614a28565b60205460405163285e212160e21b815230600482018190529293505f916001600160a01b03169063a178848490602401602060405180830381865afa158015610c5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c819190614746565b6040805160018082528183019092529192505f9190816020015b604080516060808201835280825260208201525f91810191909152815260200190600190039081610c9b5790505090506040518060600160405280888152602001878152602001846001600160a01b0316815250815f81518110610d0157610d01614a14565b60209081029190910101526040805160018082528183019092525f91816020015b610d2a613c62565b815260200190600190039081610d225790505090506040518060e00160405280306001600160a01b03168152602001866001600160a01b03168152602001856001600160a01b031681526020018481526020014363ffffffff16815260200189815260200188815250815f81518110610da557610da5614a14565b602090810291909101810191909152546040516306ec6e8160e11b81525f916001600160a01b031690630dd8dd0290610de2908690600401614a62565b5f604051808303815f875af1158015610dfd573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610e249190810190614af4565b9050610e4b825182516040518060600160405280602681526020016155bc60269139612e50565b509450505050505b92915050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610eaa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ece9190614746565b50610f02604051806040016040528060128152602001713932b3b4b9ba32b920b9a7b832b930ba37b960711b815250612a0e565b604080516060810182523081525f60208083018281528385019283529054602854945163024b980360e51b815284516001600160a01b039081166004830152925183166024820152925163ffffffff9081166044850152909416606483015260a06084830152600860a4830152676d6574616461746160c01b60c48301529192919091169063497300609060e4015f604051808303815f87803b158015610fa7575f5ffd5b505af1158015610fb9573d5f5f3e3d5ffd5b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b828210156110f3575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b828210156110dc578382905f5260205f2001805461105190614b25565b80601f016020809104026020016040519081016040528092919081815260200182805461107d90614b25565b80156110c85780601f1061109f576101008083540402835291602001916110c8565b820191905f5260205f20905b8154815290600101906020018083116110ab57829003601f168201915b505050505081526020019060010190611034565b505050508152505081526020019060010190610fe3565b50505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015611145573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111699190614746565b506111a86040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612a0e565b5f82516001600160401b038111156111c2576111c2613d3a565b6040519080825280602002602001820160405280156111f557816020015b60608152602001906001900390816111e05790505b5090505f5b835181101561124c5761122784828151811061121857611218614a14565b60200260200101516001612ebc565b82828151811061123957611239614a14565b60209081029190910101526001016111fa565b5090505b919050565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561129e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112c29190614746565b506113556040518060400160405280600f81526020016e666f726365556e64656c656761746560881b815250836001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611329573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526113509190810190614b5d565b6131d6565b5f61135f83613231565b6020546040516336a2fa1960e21b81526001600160a01b03868116600483015292935091169063da8be864906024015f604051808303815f875af11580156113a9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261124c9190810190614af4565b602354604080516301504d8f60e41b815290515f926001600160a01b031691631504d8f0916004808301926020929190829003018187875af1158015611418573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061143c9190614746565b5061146c6040518060400160405280600e81526020016d6578697456616c696461746f727360901b815250612a0e565b610e5382613546565b606060188054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116107d6575050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116107d6575050505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561157a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061159e9190614746565b506115dd6040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612a0e565b610e53826001612ebc565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015611631573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116559190614746565b506116946040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612a0e565b5f82516001600160401b038111156116ae576116ae613d3a565b6040519080825280602002602001820160405280156116e157816020015b60608152602001906001900390816116cc5790505b5090505f5b835181101561124c5761171284828151811061170457611704614a14565b60200260200101515f612ebc565b82828151811061172457611724614a14565b60209081029190910101526001016116e6565b6060601b805480602002602001604051908101604052809291908181526020015f905b828210156110f3578382905f5260205f2090600202016040518060400160405290815f8201805461178a90614b25565b80601f01602080910402602001604051908101604052809291908181526020018280546117b690614b25565b80156118015780601f106117d857610100808354040283529160200191611801565b820191905f5260205f20905b8154815290600101906020018083116117e457829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561188357602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116118455790505b5050505050815250508152602001906001019061175a565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af11580156118e4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119089190614746565b506119476040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612a0e565b610e53825f612ebc565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156119a2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119c69190614746565b506119fd604051806040016040528060158152602001743232b837b9b4ba24b73a37a2b4b3b2b73630bcb2b960591b815250612a0e565b5f5b8251811015610798575f838281518110611a1b57611a1b614a14565b602002602001015190505f838381518110611a3857611a38614a14565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603611aec575f611a78613689565b50905060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611ac7575f5ffd5b505af1158015611ad9573d5f5f3e3d5ffd5b50505050611ae681613a7e565b50611c43565b5f826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b29573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b4d9190614a28565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015611b9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bc39190614a43565b506021546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303815f875af1158015611c1c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c409190614746565b50505b50506001016119ff565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611c9e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cc29190614746565b50611d016040518060400160405280601b81526020017f7665726966795769746864726177616c43726564656e7469616c730000000000815250612a0e565b611d0a81613a7e565b50565b6060601a805480602002602001604051908101604052809291908181526020015f905b828210156110f3578382905f5260205f20018054611d4d90614b25565b80601f0160208091040260200160405190810160405280929190818152602001828054611d7990614b25565b8015611dc45780601f10611d9b57610100808354040283529160200191611dc4565b820191905f5260205f20905b815481529060010190602001808311611da757829003601f168201915b505050505081526020019060010190611d30565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611e29573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e4d9190614746565b50611e7e6040518060400160405280600f81526020016e1cdd185c9d10da1958dadc1bda5b9d608a1b815250612a0e565b611e86612ac7565b565b6060601d805480602002602001604051908101604052809291908181526020015f905b828210156110f3575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611f5157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611f135790505b50505050508152505081526020019060010190611eab565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015611fb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fd69190614746565b506120026040518060400160405280600a815260200169756e64656c656761746560b01b815250612a0e565b5f61200c30613231565b6020546040516336a2fa1960e21b81523060048201529192506001600160a01b03169063da8be864906024015f604051808303815f875af1158015612053573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261207a9190810190614af4565b505f5b815181101561227e575f51602061551d5f395f51905f526040516120ca9060208082526015908201527432bc3832b1ba34b733903bb4ba34323930bbb0b61d60591b604082015260600190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882828151811061210557612105614a14565b602002602001015160600151604051612142919060408082526007908201526603737b731b29d160cd1b6060820152602081019190915260800190565b60405180910390a17f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f82828151811061217d5761217d614a14565b602002602001015160a001515f8151811061219a5761219a614a14565b60200260200101516040516121dc9190604080825260079082015266039ba3930ba1d160cd1b60608201526001600160a01b0391909116602082015260800190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882828151811061221757612217614a14565b602002602001015160c001515f8151811061223457612234614a14565b602002602001015160405161226e9190604080825260089082015267039b430b932b99d160c51b6060820152602081019190915260800190565b60405180910390a160010161207d565b50905090565b6027546060905f906001600160401b038111156122a3576122a3613d3a565b6040519080825280602002602001820160405280156122cc578160200160208202803683370190505b5090505f80805b60275481101561241d57602454602780546001600160a01b039092169163aa47389c91908490811061230757612307614a14565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff166040518263ffffffff1660e01b8152600401612355919064ffffffffff91909116815260200190565b602060405180830381865afa158015612370573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123949190614a43565b1561241557602781815481106123ac576123ac614a14565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff168483815181106123e3576123e3614a14565b64ffffffffff909216602092830291909101909101528261240381614bb5565b935050818061241190614bb5565b9250505b6001016122d3565b50508152919050565b60606025805461243590614b25565b80601f016020809104026020016040519081016040528092919081815260200182805461246190614b25565b80156107f45780601f10612483576101008083540402835291602001916107f4565b820191905f5260205f20905b81548152906001019060200180831161248f57509395945050505050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156124fe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125229190614746565b506125846040518060400160405280600a81526020016964656c6567617465546f60b01b815250826001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611329573d5f5f3e3d5ffd5b604080518082018252606081525f602080830182905254925163eea9064b60e01b815291926001600160a01b03169163eea9064b916125c99186918691600401614bcd565b5f604051808303815f87803b1580156125e0575f5ffd5b505af11580156125f2573d5f5f3e3d5ffd5b505050505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b828210156110f3575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156126c357602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116126855790505b5050505050815250508152602001906001019061261d565b60606019805480602002602001604051908101604052809291908181526020015f905b828210156110f3578382905f5260205f2001805461271b90614b25565b80601f016020809104026020016040519081016040528092919081815260200182805461274790614b25565b80156127925780601f1061276957610100808354040283529160200191612792565b820191905f5260205f20905b81548152906001019060200180831161277557829003601f168201915b5050505050815260200190600101906126fe565b6008545f9060ff16156127bd575060085460ff1690565b604051630667f9d760e41b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa15801561281b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061283f9190614746565b1415905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612897573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128bb9190614746565b506128ef6040518060400160405280601281526020017118dbdb5c1b195d1950da1958dadc1bda5b9d60721b815250612a0e565b611e86612b56565b606060158054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116107d6575050505050905090565b60605f60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156129a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129cd9190614746565b506129fe6040518060400160405280600f81526020016e737461727456616c696461746f727360881b815250612a0e565b612a06613689565b915091509091565b5f51602061551d5f395f51905f52612a2c612a27612426565b613b35565b612a3583613b5e565b604051602001612a46929190614c24565b60408051601f1981840301815290829052612a609161470a565b60405180910390a150565b805115612a7a57805181602001fd5b60405162461bcd60e51b815260206004820152601b60248201527f7265766572746564207769746820756e6b6e6f776e206572726f72000000000060448201526064015b60405180910390fd5b6026546040516388676cad60e01b81525f60048201526001600160a01b03909116906388676cad906024015f604051808303815f87803b158015612b09575f5ffd5b505af1925050508015612b1a575060015b611e86573d808015612b47576040519150601f19603f3d011682016040523d82523d5f602084013e612b4c565b606091505b50611d0a81612a6b565b604080518082018252601881527f2d206163746976652076616c696461746f7220636f756e7400000000000000006020808301919091526026548351632340e8d360e01b81529351612bfb946001600160a01b0390921692632340e8d392600480820193918290030181865afa158015612bd2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bf69190614746565b613b86565b60408051808201825260128152712d2070726f6f66732072656d61696e696e6760701b602082015260265482516323e941b960e11b81529251612c9e936001600160a01b03909216916347d283729160048083019260a09291908290030181865afa158015612c6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c909190614c43565b6020015162ffffff16613b86565b602654604080516321767f9560e11b815290515f926001600160a01b0316916342ecff2a9160048083019260209291908290030181865afa158015612ce5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d099190614cbc565b9050806001600160401b03165f03612d7c5760405162461bcd60e51b815260206004820152603060248201527f557365722e5f636f6d706c657465436865636b706f696e743a206e6f2065786960448201526f1cdd1a5b99c818da1958dadc1bda5b9d60821b6064820152608401612abe565b60245460405163b1b6f6a160e01b81525f916001600160a01b03169063b1b6f6a190612daf906027908690600401614cd5565b5f60405180830381865afa158015612dc9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612df09190810190614e3a565b9050612e1960405180606001604052806022815260200161559a60229139826020015151613b86565b6026548151602083015160405163783a5d3160e11b81526001600160a01b039093169263f074ba62926125c9929091600401614f94565b6040516388b44c8560e01b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d906388b44c8590612e8b9086908690869060040161502a565b5f6040518083038186803b158015612ea1575f5ffd5b505afa158015612eb3573d5f5f3e3d5ffd5b50505050505050565b60605f8360a00151516001600160401b03811115612edc57612edc613d3a565b604051908082528060200260200182016040528015612f05578160200160208202803683370190505b5090505f5b815181101561316c575f8560a001518281518110612f2a57612f2a614a14565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316816001600160a01b0316036130d05773beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0838381518110612f8857612f88614a14565b60200260200101906001600160a01b031690816001600160a01b03168152505084156130cb57612fcf60405180606001604052806032815260200161556860329139613bb7565b612fdf612fda612284565b613546565b5060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561302c575f5ffd5b505af115801561303e573d5f5f3e3d5ffd5b5050505061304a612ac7565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa15801561309a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130be9190614746565b156130cb576130cb612b56565b613163565b806001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa15801561310c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131309190614a28565b83838151811061314257613142614a14565b60200260200101906001600160a01b031690816001600160a01b0316815250505b50600101612f0a565b50602054604051630e4cc3f960e41b81526001600160a01b039091169063e4cc3f90906131a190879085908890600401615048565b5f604051808303815f87803b1580156131b8575f5ffd5b505af11580156131ca573d5f5f3e3d5ffd5b50929695505050505050565b5f51602061551d5f395f51905f526131ef612a27612426565b6131f884613b5e565b8360405160200161320b9392919061507f565b60408051601f19818403018152908290526132259161470a565b60405180910390a15050565b6020546040516366d5ba9360e01b81526001600160a01b0383811660048301526060925f928392909116906366d5ba93906024015f60405180830381865afa15801561327f573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526132a691908101906150b4565b915091505f82516001600160401b038111156132c4576132c4613d3a565b6040519080825280602002602001820160405280156132fd57816020015b6132ea613c62565b8152602001906001900390816132e25790505b50602054604051631976849960e21b81526001600160a01b0388811660048301529293505f92909116906365da126490602401602060405180830381865afa15801561334b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061336f9190614a28565b60205460405163285e212160e21b81526001600160a01b0389811660048301529293505f929091169063a178848490602401602060405180830381865afa1580156133bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e09190614746565b90505f5b855181101561353a576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f9291506020808301908036833701905050905087838151811061344257613442614a14565b6020026020010151825f8151811061345c5761345c614a14565b60200260200101906001600160a01b031690816001600160a01b03168152505086838151811061348e5761348e614a14565b6020026020010151815f815181106134a8576134a8614a14565b6020026020010181815250506040518060e001604052808b6001600160a01b03168152602001866001600160a01b031681526020018b6001600160a01b0316815260200184866134f8919061516f565b81526020014363ffffffff1681526020018381526020018281525086848151811061352557613525614a14565b602090810291909101015250506001016133e4565b50919695505050505050565b5f6135876040518060400160405280601881526020017f2d2065786974696e67206e756d2076616c696461746f727300000000000000008152508351613b86565b5f5b82518110156136405760245483516001600160a01b039091169063f8f98a4e908590849081106135bb576135bb614a14565b60200260200101516040518263ffffffff1660e01b81526004016135ec919064ffffffffff91909116815260200190565b6020604051808303815f875af1158015613608573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061362c9190614cbc565b6136369083615182565b9150600101613589565b506112506040518060400160405280601e81526020017f2d206578697465642062616c616e636520746f20706f64202867776569290000815250826001600160401b0316613b86565b60605f47816136a16801bc16d674ec800000836151b5565b90506136b6816801bc16d674ec8000006151c8565b6136c090836151df565b91505f81670de0b6b3a76400008410613707576136e1633b9aca00856151f2565b6136eb90856151df565b91506136f782856151df565b93508061370381614bb5565b9150505b805f036137735760405162461bcd60e51b815260206004820152603460248201527f737461727456616c696461746f72733a206e6f7420656e6f75676820455448206044820152733a379039ba30b93a1030903b30b634b230ba37b960611b6064820152608401612abe565b5f816001600160401b0381111561378c5761378c613d3a565b6040519080825280602002602001820160405280156137b5578160200160208202803683370190505b5090505f633b9aca006137c887476151df565b6137d291906151b5565b90506138146040518060400160405280601981526020017f2d206372656174696e67206e65772076616c696461746f7273000000000000008152508351613b86565b61383f6040518060600160405280602b815260200161553d602b9139826001600160401b0316613b86565b5f5b85811015613955576024545f906001600160a01b031663ed3c16056801bc16d674ec80000061386e613bd3565b6040518363ffffffff1660e01b815260040161388a919061470a565b60206040518083038185885af11580156138a6573d5f5f3e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906138cb9190615205565b9050808483815181106138e0576138e0614a14565b64ffffffffff928316602091820292909201015260278054600181810183555f9290925260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054958516600592909306919091026101000a918202919093021990931692909217905501613841565b5061396185600161516f565b8303613a71576024545f906001600160a01b031663ed3c160586613983613bd3565b6040518363ffffffff1660e01b815260040161399f919061470a565b60206040518083038185885af11580156139bb573d5f5f3e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906139e09190615205565b90508083600185516139f291906151df565b81518110613a0257613a02614a14565b64ffffffffff9283166020918202929092010152602780546001810182555f9190915260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054948416600592909306919091026101000a91820291909202199092169190911790555b9097909650945050505050565b6024546040516352851d0d60e11b81525f916001600160a01b03169063a50a3a1a90613aae9085906004016146f8565b5f60405180830381865afa158015613ac8573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613aef919081019061529e565b6026548151602083015160408085015160608601519151633f65cf1960e01b81529596506001600160a01b0390941694633f65cf19946107349493928992600401615431565b6060610e53604051806040016040528060058152602001641b5b39366d60d81b81525083613c18565b6060610e53604051806040016040528060048152602001631b5b336d60e01b81525083613c18565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882826040516132259291906154de565b5f51602061551d5f395f51905f5281604051612a60919061470a565b60265460408051600160f81b60208201525f60218201526bffffffffffffffffffffffff19606093841b16602c82015201604051602081830303815290604052905090565b60608282604051806040016040528060048152602001631b5b306d60e01b815250604051602001613c4b939291906154ff565b604051602081830303815290604052905092915050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b64ffffffffff81168114611d0a575f5ffd5b5f60208284031215613cdd575f5ffd5b8135613ce881613cbb565b9392505050565b602080825282518282018190525f918401906040840190835b81811015613d2f5783516001600160a01b0316835260209384019390920191600101613d08565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715613d7057613d70613d3a565b60405290565b604080519081016001600160401b0381118282101715613d7057613d70613d3a565b604051606081016001600160401b0381118282101715613d7057613d70613d3a565b60405160a081016001600160401b0381118282101715613d7057613d70613d3a565b604051608081016001600160401b0381118282101715613d7057613d70613d3a565b604051601f8201601f191681016001600160401b0381118282101715613e2657613e26613d3a565b604052919050565b5f6001600160401b03821115613e4657613e46613d3a565b5060051b60200190565b6001600160a01b0381168114611d0a575f5ffd5b5f82601f830112613e73575f5ffd5b8135613e86613e8182613e2e565b613dfe565b8082825260208201915060208360051b860101925085831115613ea7575f5ffd5b602085015b83811015613ecd578035613ebf81613e50565b835260209283019201613eac565b5095945050505050565b5f5f60408385031215613ee8575f5ffd5b82356001600160401b03811115613efd575f5ffd5b613f0985828601613e64565b92505060208301356001600160401b03811115613f24575f5ffd5b8301601f81018513613f34575f5ffd5b8035613f42613e8182613e2e565b8082825260208201915060208360051b850101925087831115613f63575f5ffd5b6020840193505b82841015613f85578335825260209384019390910190613f6a565b809450505050509250929050565b5f82601f830112613fa2575f5ffd5b8135613fb0613e8182613e2e565b8082825260208201915060208360051b860101925085831115613fd1575f5ffd5b602085015b83811015613ecd578035835260209283019201613fd6565b5f5f60408385031215613fff575f5ffd5b82356001600160401b03811115614014575f5ffd5b61402085828601613e64565b92505060208301356001600160401b0381111561403b575f5ffd5b61404785828601613f93565b9150509250929050565b5f8151808452602084019350602083015f5b8281101561408a5781516001600160a01b0316865260209586019590910190600101614063565b5093949350505050565b5f8151808452602084019350602083015f5b8281101561408a5781518652602095860195909101906001016140a6565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f9161410f9085018263ffffffff169052565b5060a082015160e060a085015261412960e0850182614051565b905060c083015184820360c08601526141428282614094565b95945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f1987860301845261418d8583516140c4565b94506020938401939190910190600101614171565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561421e57601f198584030188526142088383516141a2565b60209889019890935091909101906001016141ec565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57868503603f19018452815180516001600160a01b0316865260209081015160409187018290529061428b908701826141d0565b9550506020938401939190910190600101614250565b803561125081613e50565b803563ffffffff81168114611250575f5ffd5b5f60e082840312156142cf575f5ffd5b6142d7613d4e565b90506142e2826142a1565b81526142f0602083016142a1565b6020820152614301604083016142a1565b60408201526060828101359082015261431c608083016142ac565b608082015260a08201356001600160401b03811115614339575f5ffd5b61434584828501613e64565b60a08301525060c08201356001600160401b03811115614363575f5ffd5b61436f84828501613f93565b60c08301525092915050565b5f6020828403121561438b575f5ffd5b81356001600160401b038111156143a0575f5ffd5b8201601f810184136143b0575f5ffd5b80356143be613e8182613e2e565b8082825260208201915060208360051b8501019250868311156143df575f5ffd5b602084015b8381101561441f5780356001600160401b03811115614401575f5ffd5b614410896020838901016142bf565b845250602092830192016143e4565b509695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f1987860301845261446c858351614051565b94506020938401939190910190600101614450565b5f60208284031215614491575f5ffd5b8135613ce881613e50565b5f602082840312156144ac575f5ffd5b81356001600160401b038111156144c1575f5ffd5b8201601f810184136144d1575f5ffd5b80356144df613e8182613e2e565b8082825260208201915060208360051b850101925086831115614500575f5ffd5b6020840193505b8284101561452b57833561451a81613cbb565b825260209384019390910190614507565b9695505050505050565b5f60208284031215614545575f5ffd5b81356001600160401b0381111561455a575f5ffd5b614566848285016142bf565b949350505050565b602081525f613ce86020830184614051565b5f8151808452602084019350602083015f5b8281101561408a5781516001600160e01b031916865260209586019590910190600101614592565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f19878603018452815180516040875261460660408801826141a2565b90506020820151915086810360208801526146218183614580565b9650505060209384019391909101906001016145e0565b602081525f613ce860208301846141d0565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57868503603f19018452815180516001600160a01b031686526020908101516040918701829052906146ab90870182614580565b9550506020938401939190910190600101614670565b5f8151808452602084019350602083015f5b8281101561408a57815164ffffffffff168652602095860195909101906001016146d3565b602081525f613ce860208301846146c1565b602081525f613ce860208301846141a2565b604081525f61472e60408301856146c1565b90506001600160401b03831660208301529392505050565b5f60208284031215614756575f5ffd5b5051919050565b80516001600160401b0381168114611250575f5ffd5b5f5f6001600160401b0384111561478c5761478c613d3a565b50601f8301601f19166020016147a181613dfe565b9150508281528383830111156147b5575f5ffd5b8282602083015e5f602084830101529392505050565b5f82601f8301126147da575f5ffd5b613ce883835160208501614773565b5f604082840312156147f9575f5ffd5b614801613d76565b8251815260208301519091506001600160401b03811115614820575f5ffd5b61482c848285016147cb565b60208301525092915050565b5f82601f830112614847575f5ffd5b8151614855613e8182613e2e565b8082825260208201915060208360051b860101925085831115614876575f5ffd5b602085015b83811015613ecd57805183526020928301920161487b565b5f602082840312156148a3575f5ffd5b81516001600160401b038111156148b8575f5ffd5b8201606081850312156148c9575f5ffd5b6148d1613d98565b6148da8261475d565b815260208201516001600160401b038111156148f4575f5ffd5b614900868285016147e9565b60208301525060408201516001600160401b0381111561491e575f5ffd5b919091019060408286031215614932575f5ffd5b61493a613d76565b82516001600160401b0381111561494f575f5ffd5b61495b87828601614838565b82525060208301516001600160401b03811115614976575f5ffd5b614982878286016147cb565b6020830152506040820152949350505050565b805182525f60208201516040602085015261456660408501826141a2565b6001600160401b0384168152606060208201525f6149d46060830185614995565b82810360408401528351604082526149ef6040830182614094565b905060208501518282036020840152614a0882826141a2565b98975050505050505050565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614a38575f5ffd5b8151613ce881613e50565b5f60208284031215614a53575f5ffd5b81518015158114613ce8575f5ffd5b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f198786030184528151805160608752614aae6060880182614051565b905060208201518782036020890152614ac78282614094565b6040938401516001600160a01b031698909301979097525094506020938401939190910190600101614a88565b5f60208284031215614b04575f5ffd5b81516001600160401b03811115614b19575f5ffd5b61456684828501614838565b600181811c90821680614b3957607f821691505b602082108103614b5757634e487b7160e01b5f52602260045260245ffd5b50919050565b5f60208284031215614b6d575f5ffd5b81516001600160401b03811115614b82575f5ffd5b8201601f81018413614b92575f5ffd5b61456684825160208401614773565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614bc657614bc6614ba1565b5060010190565b60018060a01b0384168152606060208201525f835160406060840152614bf660a08401826141a2565b602095909501516080840152505060400152919050565b5f81518060208401855e5f93019283525090919050565b5f614c2f8285614c0d565b601760f91b81526141426001820185614c0d565b5f60a0828403128015614c54575f5ffd5b50614c5d613dba565b82518152602083015162ffffff81168114614c76575f5ffd5b6020820152614c876040840161475d565b604082015260608301518060070b8114614c9f575f5ffd5b6060820152614cb06080840161475d565b60808201529392505050565b5f60208284031215614ccc575f5ffd5b613ce88261475d565b5f6040820160408352808554614cef818490815260200190565b5f8881526020812094509092505b81600582011015614d6457835464ffffffffff8082168552602882901c81166020860152605082901c81166040860152607882901c8116606086015260a082811c8216608087015260c89290921c169084015260019093019260c090920191600601614cfd565b92549281811015614d835764ffffffffff841683526020909201916001015b81811015614da35764ffffffffff602885901c1683526020909201916001015b81811015614dc35764ffffffffff605085901c1683526020909201916001015b81811015614de35764ffffffffff607885901c1683526020909201916001015b81811015614e035764ffffffffff60a085901c1683526020909201916001015b81811015614e205760c884901c64ffffffffff1683526020830192505b50506001600160401b03851660208501529150613ce89050565b5f60208284031215614e4a575f5ffd5b81516001600160401b03811115614e5f575f5ffd5b820160408185031215614e70575f5ffd5b614e78613d76565b81516001600160401b03811115614e8d575f5ffd5b614e99868285016147e9565b82525060208201516001600160401b03811115614eb4575f5ffd5b80830192505084601f830112614ec8575f5ffd5b8151614ed6613e8182613e2e565b8082825260208201915060208360051b860101925087831115614ef7575f5ffd5b602085015b83811015614f835780516001600160401b03811115614f19575f5ffd5b86016060818b03601f19011215614f2e575f5ffd5b614f36613d98565b602082810151825260408301519082015260608201516001600160401b03811115614f5f575f5ffd5b614f6e8c6020838601016147cb565b60408301525084525060209283019201614efc565b506020840152509095945050505050565b604081525f614fa66040830185614995565b828103602084015280845180835260208301915060208160051b840101602087015f5b8381101561501c57601f1986840301855281518051845260208101516020850152604081015190506060604085015261500560608501826141a2565b602096870196909450929092019150600101614fc9565b509098975050505050505050565b838152826020820152606060408201525f61414260608301846141a2565b606081525f61505a60608301866140c4565b828103602084015261506c8186614051565b9150508215156040830152949350505050565b5f61508a8286614c0d565b601760f91b815261509e6001820186614c0d565b9050601d60f91b815261452b6001820185614c0d565b5f5f604083850312156150c5575f5ffd5b82516001600160401b038111156150da575f5ffd5b8301601f810185136150ea575f5ffd5b80516150f8613e8182613e2e565b8082825260208201915060208360051b850101925087831115615119575f5ffd5b6020840193505b8284101561514457835161513381613e50565b825260209384019390910190615120565b8095505050505060208301516001600160401b03811115615163575f5ffd5b61404785828601614838565b80820180821115610e5357610e53614ba1565b6001600160401b038181168382160190811115610e5357610e53614ba1565b634e487b7160e01b5f52601260045260245ffd5b5f826151c3576151c36151a1565b500490565b8082028115828204841417610e5357610e53614ba1565b81810381811115610e5357610e53614ba1565b5f82615200576152006151a1565b500690565b5f60208284031215615215575f5ffd5b8151613ce881613cbb565b5f82601f83011261522f575f5ffd5b815161523d613e8182613e2e565b8082825260208201915060208360051b86010192508583111561525e575f5ffd5b602085015b83811015613ecd5780516001600160401b03811115615280575f5ffd5b61528f886020838a0101614838565b84525060209283019201615263565b5f602082840312156152ae575f5ffd5b81516001600160401b038111156152c3575f5ffd5b8201608081850312156152d4575f5ffd5b6152dc613ddc565b6152e58261475d565b815260208201516001600160401b038111156152ff575f5ffd5b61530b868285016147e9565b60208301525060408201516001600160401b03811115615329575f5ffd5b8201601f81018613615339575f5ffd5b8051615347613e8182613e2e565b8082825260208201915060208360051b850101925088831115615368575f5ffd5b602084015b838110156153a85780516001600160401b0381111561538a575f5ffd5b6153998b6020838901016147cb565b8452506020928301920161536d565b50604085015250505060608201516001600160401b038111156153c9575f5ffd5b6153d586828501615220565b606083015250949350505050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561421e57601f1985840301885261541b838351614094565b60209889019890935091909101906001016153ff565b6001600160401b038616815260a060208201525f61545260a0830187614995565b828103604084015261546481876146c1565b9050828103606084015280855180835260208301915060208160051b840101602088015f5b838110156154bb57601f198684030185526154a58383516141a2565b6020958601959093509190910190600101615489565b505085810360808701526154cf81886153e3565b9b9a5050505050505050505050565b604081525f6154f060408301856141a2565b90508260208301529392505050565b5f6141426155166155108488614c0d565b86614c0d565b84614c0d56fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f502d206465706f736974696e672062616c616e636520746f20626561636f6e20636861696e202867776569292d2065786974696e6720616c6c2076616c696461746f727320616e6420636f6d706c6574696e6720636865636b706f696e742d207375626d697474696e67206e756d20636865636b706f696e742070726f6f6673557365722e71756575655769746864726177616c733a206c656e677468206d69736d61746368a26469706673582212200452283de03c7bf2dc409eb68bbb8b9544f447688532742a90fb63bac9e9710464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x0C\x80T`\x01`\xFF\x19\x90\x91\x16\x81\x17\x90\x91U`\x1F\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16tq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x01\x17\x90U`(\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90U4\x80\x15a\0WW__\xFD[P`@Qa\\\x158\x03\x80a\\\x15\x839\x81\x01`@\x81\x90Ra\0v\x91a\x03\xCAV[_3\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xEAM<\x9B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xDA\x91\x90a\x04\x91V[` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x81U`@\x80Qc\x076\xE1\xC7`\xE3\x1B\x81R\x90Q\x92\x84\x16\x92c9\xB7\x0E8\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x010W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01T\x91\x90a\x04\x91V[`!_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16cFe\xBC\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xD9\x91\x90a\x04\x91V[`\"_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c=\xFB@\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02^\x91\x90a\x04\x91V[`#_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c\"\xC05\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE3\x91\x90a\x04\x91V[`$\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x03\x0Ba\x03\x1FV[`%a\x03\x17\x83\x82a\x057V[PPPa\x05\xF1V[`\"_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x84\xD8\x10b`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03pW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x94\x91\x90a\x04\x91V[`&\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x03\xDAW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xEFW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x03\xFFW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\x18Wa\x04\x18a\x03\xB6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04FWa\x04Fa\x03\xB6V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x04]W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x8EW__\xFD[PV[_` \x82\x84\x03\x12\x15a\x04\xA1W__\xFD[\x81Qa\x04\xAC\x81a\x04zV[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\xC7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\xE5WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x052W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x05\x10WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x05/W_\x81U`\x01\x01a\x05\x1CV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05PWa\x05Pa\x03\xB6V[a\x05d\x81a\x05^\x84Ta\x04\xB3V[\x84a\x04\xEBV[` `\x1F\x82\x11`\x01\x81\x14a\x05\x96W_\x83\x15a\x05\x7FWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x05/V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x05\xC5W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x05\xA5V[P\x84\x82\x10\x15a\x05\xE2W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[aV\x17\x80a\x05\xFE_9_\xF3\xFE`\x80`@R`\x046\x10a\x01\xDEW_5`\xE0\x1C\x80c\x84\x1C\x12\x99\x11a\0\xFDW\x80c\xACc|z\x11a\0\x92W\x80c\xD6\xC1\r\xAF\x11a\0bW\x80c\xD6\xC1\r\xAF\x14a\x05uW\x80c\xE2\x0C\x9Fq\x14a\x05\x89W\x80c\xF24\xC1\xBD\x14a\x05\x9DW\x80c\xFAv&\xD4\x14a\x05\xBFW__\xFD[\x80c\xACc|z\x14a\x05\nW\x80c\xB0FO\xDC\x14a\x05)W\x80c\xB5P\x8A\xA9\x14a\x05=W\x80c\xBAAO\xA6\x14a\x05QW__\xFD[\x80c\x92\xAB\x89\xBB\x11a\0\xCDW\x80c\x92\xAB\x89\xBB\x14a\x04}W\x80c\x9D\xE7\x02X\x14a\x04\x91W\x80c\xA3\xF4\xDF~\x14a\x04\xB2W\x80c\xA8\x8D\xBB6\x14a\x04\xD3W__\xFD[\x80c\x84\x1C\x12\x99\x14a\x04\x08W\x80c\x85\"l\x81\x14a\x04'W\x80c\x90\xB5\x16%\x14a\x04HW\x80c\x91j\x17\xC6\x14a\x04\\W__\xFD[\x80c=\x8C\x08\xD4\x11a\x01sW\x80cF\xA5\xBE\r\x11a\x01CW\x80cF\xA5\xBE\r\x14a\x03\x8AW\x80cf\xD9\xA9\xA0\x14a\x03\xA9W\x80ci_J\xE1\x14a\x03\xCAW\x80cm3oX\x14a\x03\xE9W__\xFD[\x80c=\x8C\x08\xD4\x14a\x02\xFFW\x80c>^<#\x14a\x036W\x80c?r\x86\xF4\x14a\x03JW\x80c@\x1B\xE6^\x14a\x03^W__\xFD[\x80c*4\xAD\xE8\x11a\x01\xAEW\x80c*4\xAD\xE8\x14a\x02\x7FW\x80c*\xDE8\x80\x14a\x02\x93W\x80c4N\x13\x83\x14a\x02\xB4W\x80c9\x1C\xC9\xF6\x14a\x02\xE0W__\xFD[\x80c\x07\x1C%\xB7\x14a\x01\xE9W\x80c\x1E\xD7\x83\x1C\x14a\x02\nW\x80c \xA5E\xD9\x14a\x024W\x80c#\xE4\x81u\x14a\x02SW__\xFD[6a\x01\xE5W\0[__\xFD[4\x80\x15a\x01\xF4W__\xFD[Pa\x02\x08a\x02\x036`\x04a<\xCDV[a\x05\xD8V[\0[4\x80\x15a\x02\x15W__\xFD[Pa\x02\x1Ea\x07\x9EV[`@Qa\x02+\x91\x90a<\xEFV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02?W__\xFD[Pa\x02\x08a\x02N6`\x04a>\xD7V[a\x07\xFEV[4\x80\x15a\x02^W__\xFD[Pa\x02ra\x02m6`\x04a?\xEEV[a\x0B\tV[`@Qa\x02+\x91\x90aAKV[4\x80\x15a\x02\x8AW__\xFD[Pa\x02\x08a\x0EYV[4\x80\x15a\x02\x9EW__\xFD[Pa\x02\xA7a\x0F\xC0V[`@Qa\x02+\x91\x90aB*V[4\x80\x15a\x02\xBFW__\xFD[Pa\x02\xD3a\x02\xCE6`\x04aC{V[a\x10\xFCV[`@Qa\x02+\x91\x90aD*V[4\x80\x15a\x02\xEBW__\xFD[Pa\x02ra\x02\xFA6`\x04aD\x81V[a\x12UV[4\x80\x15a\x03\nW__\xFD[Pa\x03\x1Ea\x03\x196`\x04aD\x9CV[a\x13\xD0V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x03AW__\xFD[Pa\x02\x1Ea\x14uV[4\x80\x15a\x03UW__\xFD[Pa\x02\x1Ea\x14\xD3V[4\x80\x15a\x03iW__\xFD[Pa\x03}a\x03x6`\x04aE5V[a\x151V[`@Qa\x02+\x91\x90aEnV[4\x80\x15a\x03\x95W__\xFD[Pa\x02\xD3a\x03\xA46`\x04aC{V[a\x15\xE8V[4\x80\x15a\x03\xB4W__\xFD[Pa\x03\xBDa\x177V[`@Qa\x02+\x91\x90aE\xBAV[4\x80\x15a\x03\xD5W__\xFD[Pa\x03}a\x03\xE46`\x04aE5V[a\x18\x9BV[4\x80\x15a\x03\xF4W__\xFD[Pa\x02\x08a\x04\x036`\x04a?\xEEV[a\x19QV[4\x80\x15a\x04\x13W__\xFD[Pa\x02\x08a\x04\"6`\x04aD\x9CV[a\x1CMV[4\x80\x15a\x042W__\xFD[Pa\x04;a\x1D\rV[`@Qa\x02+\x91\x90aF8V[4\x80\x15a\x04SW__\xFD[Pa\x02\x08a\x1D\xD8V[4\x80\x15a\x04gW__\xFD[Pa\x04pa\x1E\x88V[`@Qa\x02+\x91\x90aFJV[4\x80\x15a\x04\x88W__\xFD[Pa\x02ra\x1FiV[4\x80\x15a\x04\x9CW__\xFD[Pa\x04\xA5a\"\x84V[`@Qa\x02+\x91\x90aF\xF8V[4\x80\x15a\x04\xBDW__\xFD[Pa\x04\xC6a$&V[`@Qa\x02+\x91\x90aG\nV[4\x80\x15a\x04\xDEW__\xFD[P`&Ta\x04\xF2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x05\x15W__\xFD[Pa\x02\x08a\x05$6`\x04aD\x81V[a$\xADV[4\x80\x15a\x054W__\xFD[Pa\x04pa%\xFAV[4\x80\x15a\x05HW__\xFD[Pa\x04;a&\xDBV[4\x80\x15a\x05\\W__\xFD[Pa\x05ea'\xA6V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[4\x80\x15a\x05\x80W__\xFD[Pa\x02\x08a(FV[4\x80\x15a\x05\x94W__\xFD[Pa\x02\x1Ea(\xF7V[4\x80\x15a\x05\xA8W__\xFD[Pa\x05\xB1a)UV[`@Qa\x02+\x92\x91\x90aG\x1CV[4\x80\x15a\x05\xCAW__\xFD[P`\x1FTa\x05e\x90`\xFF\x16\x81V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06M\x91\x90aGFV[Pa\x06\x81`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qverifyStaleBalance`p\x1B\x81RPa*\x0EV[`$\x80T`@Qc\x08\xFA\x0B\x13`\xE2\x1B\x81Rd\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c#\xE8,L\x91\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xCEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\xF5\x91\x90\x81\x01\x90aH\x93V[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x01\xC8\xAB\xE9`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93c\x03\x91W\xD2\x93a\x074\x93\x92\x91`\x04\x01aI\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07KW__\xFD[PZ\xF1\x92PPP\x80\x15a\x07\\WP`\x01[a\x07\x9AW=\x80\x80\x15a\x07\x89W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\x8EV[``\x91P[Pa\x07\x98\x81a*kV[P[PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6W[PPPPP\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08s\x91\x90aGFV[Pa\x08\xA3`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mupdateBalances`\x90\x1B\x81RPa*\x0EV[_[\x82Q\x81\x10\x15a\x07\x98W_\x83\x82\x81Q\x81\x10a\x08\xC1Wa\x08\xC1aJ\x14V[` \x02` \x01\x01Q\x90P_\x83\x83\x81Q\x81\x10a\x08\xDEWa\x08\xDEaJ\x14V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\xA3Wa\t\x1Da*\xC7V[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tmW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x91\x91\x90aGFV[\x15a\t\x9EWa\t\x9Ea+VV[a\n\xFFV[_\x81\x90P_\x83`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x08\x91\x90aJ(V[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\nZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n~\x91\x90aJCV[P`!T`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xFB\x91\x90aGFV[PPP[PP`\x01\x01a\x08\xA5V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x0BRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bv\x91\x90aGFV[Pa\x0B\xA8`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oqueueWithdrawals`\x80\x1B\x81RPa*\x0EV[` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R0`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x12\x91\x90aJ(V[` T`@Qc(^!!`\xE2\x1B\x81R0`\x04\x82\x01\x81\x90R\x92\x93P_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x81\x91\x90aGFV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R_\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x9BW\x90PP\x90P`@Q\x80``\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81_\x81Q\x81\x10a\r\x01Wa\r\x01aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[a\r*a<bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\"W\x90PP\x90P`@Q\x80`\xE0\x01`@R\x800`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x88\x81RP\x81_\x81Q\x81\x10a\r\xA5Wa\r\xA5aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91RT`@Qc\x06\xECn\x81`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\r\xD8\xDD\x02\x90a\r\xE2\x90\x86\x90`\x04\x01aJbV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xFDW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E$\x91\x90\x81\x01\x90aJ\xF4V[\x90Pa\x0EK\x82Q\x82Q`@Q\x80``\x01`@R\x80`&\x81R` \x01aU\xBC`&\x919a.PV[P\x94PPPPP[\x92\x91PPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCE\x91\x90aGFV[Pa\x0F\x02`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q92\xB3\xB4\xB9\xBA2\xB9 \xB9\xA7\xB82\xB90\xBA7\xB9`q\x1B\x81RPa*\x0EV[`@\x80Q``\x81\x01\x82R0\x81R_` \x80\x83\x01\x82\x81R\x83\x85\x01\x92\x83R\x90T`(T\x94Qc\x02K\x98\x03`\xE5\x1B\x81R\x84Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x92Q\x83\x16`$\x82\x01R\x92Qc\xFF\xFF\xFF\xFF\x90\x81\x16`D\x85\x01R\x90\x94\x16`d\x83\x01R`\xA0`\x84\x83\x01R`\x08`\xA4\x83\x01Rgmetadata`\xC0\x1B`\xC4\x83\x01R\x91\x92\x91\x90\x91\x16\x90cIs\0`\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xA7W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xB9W=__>=_\xFD[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x10\xDCW\x83\x82\x90_R` _ \x01\x80Ta\x10Q\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10}\x90aK%V[\x80\x15a\x10\xC8W\x80`\x1F\x10a\x10\x9FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xC8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x104V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xE3V[PPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x11EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11i\x91\x90aGFV[Pa\x11\xA8`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa*\x0EV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xC2Wa\x11\xC2a=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xF5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\xE0W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12LWa\x12'\x84\x82\x81Q\x81\x10a\x12\x18Wa\x12\x18aJ\x14V[` \x02` \x01\x01Q`\x01a.\xBCV[\x82\x82\x81Q\x81\x10a\x129Wa\x129aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x11\xFAV[P\x90P[\x91\x90PV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x12\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC2\x91\x90aGFV[Pa\x13U`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nforceUndelegate`\x88\x1B\x81RP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13)W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13P\x91\x90\x81\x01\x90aK]V[a1\xD6V[_a\x13_\x83a21V[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xA9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12L\x91\x90\x81\x01\x90aJ\xF4V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\x14\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14<\x91\x90aGFV[Pa\x14l`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mexitValidators`\x90\x1B\x81RPa*\x0EV[a\x0ES\x82a5FV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6WPPPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x15zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9E\x91\x90aGFV[Pa\x15\xDD`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa*\x0EV[a\x0ES\x82`\x01a.\xBCV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x161W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16U\x91\x90aGFV[Pa\x16\x94`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa*\x0EV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xAEWa\x16\xAEa=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xE1W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\xCCW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12LWa\x17\x12\x84\x82\x81Q\x81\x10a\x17\x04Wa\x17\x04aJ\x14V[` \x02` \x01\x01Q_a.\xBCV[\x82\x82\x81Q\x81\x10a\x17$Wa\x17$aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x16\xE6V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x17\x8A\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xB6\x90aK%V[\x80\x15a\x18\x01W\x80`\x1F\x10a\x17\xD8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x01V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18\x83W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18EW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x17ZV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x18\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x08\x91\x90aGFV[Pa\x19G`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa*\x0EV[a\x0ES\x82_a.\xBCV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC6\x91\x90aGFV[Pa\x19\xFD`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t22\xB87\xB9\xB4\xBA$\xB7:7\xA2\xB4\xB3\xB2\xB760\xBC\xB2\xB9`Y\x1B\x81RPa*\x0EV[_[\x82Q\x81\x10\x15a\x07\x98W_\x83\x82\x81Q\x81\x10a\x1A\x1BWa\x1A\x1BaJ\x14V[` \x02` \x01\x01Q\x90P_\x83\x83\x81Q\x81\x10a\x1A8Wa\x1A8aJ\x14V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1A\xECW_a\x1Axa6\x89V[P\x90P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xC7W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xD9W=__>=_\xFD[PPPPa\x1A\xE6\x81a:~V[Pa\x1CCV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BM\x91\x90aJ(V[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC3\x91\x90aJCV[P`!T`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C@\x91\x90aGFV[PP[PP`\x01\x01a\x19\xFFV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xC2\x91\x90aGFV[Pa\x1D\x01`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FverifyWithdrawalCredentials\0\0\0\0\0\x81RPa*\x0EV[a\x1D\n\x81a:~V[PV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W\x83\x82\x90_R` _ \x01\x80Ta\x1DM\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Dy\x90aK%V[\x80\x15a\x1D\xC4W\x80`\x1F\x10a\x1D\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1D0V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EM\x91\x90aGFV[Pa\x1E~`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x1C\xDD\x18\\\x9D\x10\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x8A\x1B\x81RPa*\x0EV[a\x1E\x86a*\xC7V[V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1FQW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1F\x13W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xABV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x1F\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xD6\x91\x90aGFV[Pa \x02`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iundelegate`\xB0\x1B\x81RPa*\x0EV[_a \x0C0a21V[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a SW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra z\x91\x90\x81\x01\x90aJ\xF4V[P_[\x81Q\x81\x10\x15a\"~W_Q` aU\x1D_9_Q\x90_R`@Qa \xCA\x90` \x80\x82R`\x15\x90\x82\x01Rt2\xBC82\xB1\xBA4\xB73\x90;\xB4\xBA4290\xBB\xB0\xB6\x1D`Y\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a!\x05Wa!\x05aJ\x14V[` \x02` \x01\x01Q``\x01Q`@Qa!B\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03s{s\x1B)\xD1`\xCD\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x82\x82\x81Q\x81\x10a!}Wa!}aJ\x14V[` \x02` \x01\x01Q`\xA0\x01Q_\x81Q\x81\x10a!\x9AWa!\x9AaJ\x14V[` \x02` \x01\x01Q`@Qa!\xDC\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03\x9B\xA3\x93\x0B\xA1\xD1`\xCD\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a\"\x17Wa\"\x17aJ\x14V[` \x02` \x01\x01Q`\xC0\x01Q_\x81Q\x81\x10a\"4Wa\"4aJ\x14V[` \x02` \x01\x01Q`@Qa\"n\x91\x90`@\x80\x82R`\x08\x90\x82\x01Rg\x03\x9BC\x0B\x93+\x99\xD1`\xC5\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a }V[P\x90P\x90V[`'T``\x90_\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xA3Wa\"\xA3a=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xCCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80\x80[`'T\x81\x10\x15a$\x1DW`$T`'\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xAAG8\x9C\x91\x90\x84\x90\x81\x10a#\x07Wa#\x07aJ\x14V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#U\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#pW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x94\x91\x90aJCV[\x15a$\x15W`'\x81\x81T\x81\x10a#\xACWa#\xACaJ\x14V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10a#\xE3Wa#\xE3aJ\x14V[d\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a$\x03\x81aK\xB5V[\x93PP\x81\x80a$\x11\x90aK\xB5V[\x92PP[`\x01\x01a\"\xD3V[PP\x81R\x91\x90PV[```%\x80Ta$5\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$a\x90aK%V[\x80\x15a\x07\xF4W\x80`\x1F\x10a$\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xF4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x8FWP\x93\x95\x94PPPPPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\"\x91\x90aGFV[Pa%\x84`@Q\x80`@\x01`@R\x80`\n\x81R` \x01idelegateTo`\xB0\x1B\x81RP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13)W=__>=_\xFD[`@\x80Q\x80\x82\x01\x82R``\x81R_` \x80\x83\x01\x82\x90RT\x92Qc\xEE\xA9\x06K`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE\xA9\x06K\x91a%\xC9\x91\x86\x91\x86\x91`\x04\x01aK\xCDV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%\xE0W__\xFD[PZ\xF1\x15\x80\x15a%\xF2W=__>=_\xFD[PPPPPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a&\xC3W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a&\x85W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a&\x1DV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W\x83\x82\x90_R` _ \x01\x80Ta'\x1B\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'G\x90aK%V[\x80\x15a'\x92W\x80`\x1F\x10a'iWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\x92V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'uW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a&\xFEV[`\x08T_\x90`\xFF\x16\x15a'\xBDWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(?\x91\x90aGFV[\x14\x15\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xBB\x91\x90aGFV[Pa(\xEF`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\xDB\xDB\\\x1B\x19]\x19P\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`r\x1B\x81RPa*\x0EV[a\x1E\x86a+VV[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6WPPPPP\x90P\x90V[``_`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xCD\x91\x90aGFV[Pa)\xFE`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nstartValidators`\x88\x1B\x81RPa*\x0EV[a*\x06a6\x89V[\x91P\x91P\x90\x91V[_Q` aU\x1D_9_Q\x90_Ra*,a*'a$&V[a;5V[a*5\x83a;^V[`@Q` \x01a*F\x92\x91\x90aL$V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra*`\x91aG\nV[`@Q\x80\x91\x03\x90\xA1PV[\x80Q\x15a*zW\x80Q\x81` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Freverted with unknown error\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`&T`@Qc\x88gl\xAD`\xE0\x1B\x81R_`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x88gl\xAD\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+\tW__\xFD[PZ\xF1\x92PPP\x80\x15a+\x1AWP`\x01[a\x1E\x86W=\x80\x80\x15a+GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a+LV[``\x91P[Pa\x1D\n\x81a*kV[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F- active validator count\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R`&T\x83Qc#@\xE8\xD3`\xE0\x1B\x81R\x93Qa+\xFB\x94`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c#@\xE8\xD3\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xF6\x91\x90aGFV[a;\x86V[`@\x80Q\x80\x82\x01\x82R`\x12\x81Rq- proofs remaining`p\x1B` \x82\x01R`&T\x82Qc#\xE9A\xB9`\xE1\x1B\x81R\x92Qa,\x9E\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cG\xD2\x83r\x91`\x04\x80\x83\x01\x92`\xA0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,lW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x90\x91\x90aLCV[` \x01Qb\xFF\xFF\xFF\x16a;\x86V[`&T`@\x80Qc!v\x7F\x95`\xE1\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cB\xEC\xFF*\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xE5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\t\x91\x90aL\xBCV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a-|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FUser._completeCheckpoint: no exi`D\x82\x01Ro\x1C\xDD\x1A[\x99\xC8\x18\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x82\x1B`d\x82\x01R`\x84\x01a*\xBEV[`$T`@Qc\xB1\xB6\xF6\xA1`\xE0\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xB6\xF6\xA1\x90a-\xAF\x90`'\x90\x86\x90`\x04\x01aL\xD5V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xC9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\xF0\x91\x90\x81\x01\x90aN:V[\x90Pa.\x19`@Q\x80``\x01`@R\x80`\"\x81R` \x01aU\x9A`\"\x919\x82` \x01QQa;\x86V[`&T\x81Q` \x83\x01Q`@Qcx:]1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF0t\xBAb\x92a%\xC9\x92\x90\x91`\x04\x01aO\x94V[`@Qc\x88\xB4L\x85`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x88\xB4L\x85\x90a.\x8B\x90\x86\x90\x86\x90\x86\x90`\x04\x01aP*V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a.\xA1W__\xFD[PZ\xFA\x15\x80\x15a.\xB3W=__>=_\xFD[PPPPPPPV[``_\x83`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xDCWa.\xDCa=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a/\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a1lW_\x85`\xA0\x01Q\x82\x81Q\x81\x10a/*Wa/*aJ\x14V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a0\xD0Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x83\x83\x81Q\x81\x10a/\x88Wa/\x88aJ\x14V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x15a0\xCBWa/\xCF`@Q\x80``\x01`@R\x80`2\x81R` \x01aUh`2\x919a;\xB7V[a/\xDFa/\xDAa\"\x84V[a5FV[P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0,W__\xFD[PZ\xF1\x15\x80\x15a0>W=__>=_\xFD[PPPPa0Ja*\xC7V[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xBE\x91\x90aGFV[\x15a0\xCBWa0\xCBa+VV[a1cV[\x80`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a10\x91\x90aJ(V[\x83\x83\x81Q\x81\x10a1BWa1BaJ\x14V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP[P`\x01\x01a/\nV[P` T`@Qc\x0EL\xC3\xF9`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\xCC?\x90\x90a1\xA1\x90\x87\x90\x85\x90\x88\x90`\x04\x01aPHV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xB8W__\xFD[PZ\xF1\x15\x80\x15a1\xCAW=__>=_\xFD[P\x92\x96\x95PPPPPPV[_Q` aU\x1D_9_Q\x90_Ra1\xEFa*'a$&V[a1\xF8\x84a;^V[\x83`@Q` \x01a2\x0B\x93\x92\x91\x90aP\x7FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra2%\x91aG\nV[`@Q\x80\x91\x03\x90\xA1PPV[` T`@Qcf\xD5\xBA\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R``\x92_\x92\x83\x92\x90\x91\x16\x90cf\xD5\xBA\x93\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x7FW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra2\xA6\x91\x90\x81\x01\x90aP\xB4V[\x91P\x91P_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xC4Wa2\xC4a=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xFDW\x81` \x01[a2\xEAa<bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a2\xE2W\x90P[P` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3o\x91\x90aJ(V[` T`@Qc(^!!`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE0\x91\x90aGFV[\x90P_[\x85Q\x81\x10\x15a5:W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a4BWa4BaJ\x14V[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a4\\Wa4\\aJ\x14V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x86\x83\x81Q\x81\x10a4\x8EWa4\x8EaJ\x14V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a4\xA8Wa4\xA8aJ\x14V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x86a4\xF8\x91\x90aQoV[\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81RP\x86\x84\x81Q\x81\x10a5%Wa5%aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a3\xE4V[P\x91\x96\x95PPPPPPV[_a5\x87`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F- exiting num validators\0\0\0\0\0\0\0\0\x81RP\x83Qa;\x86V[_[\x82Q\x81\x10\x15a6@W`$T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xF9\x8AN\x90\x85\x90\x84\x90\x81\x10a5\xBBWa5\xBBaJ\x14V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xEC\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6,\x91\x90aL\xBCV[a66\x90\x83aQ\x82V[\x91P`\x01\x01a5\x89V[Pa\x12P`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7F- exited balance to pod (gwei)\0\0\x81RP\x82`\x01`\x01`@\x1B\x03\x16a;\x86V[``_G\x81a6\xA1h\x01\xBC\x16\xD6t\xEC\x80\0\0\x83aQ\xB5V[\x90Pa6\xB6\x81h\x01\xBC\x16\xD6t\xEC\x80\0\0aQ\xC8V[a6\xC0\x90\x83aQ\xDFV[\x91P_\x81g\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a7\x07Wa6\xE1c;\x9A\xCA\0\x85aQ\xF2V[a6\xEB\x90\x85aQ\xDFV[\x91Pa6\xF7\x82\x85aQ\xDFV[\x93P\x80a7\x03\x81aK\xB5V[\x91PP[\x80_\x03a7sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FstartValidators: not enough ETH `D\x82\x01Rs:7\x909\xBA0\xB9:\x100\x90;0\xB64\xB20\xBA7\xB9`a\x1B`d\x82\x01R`\x84\x01a*\xBEV[_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x8CWa7\x8Ca=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\xB5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_c;\x9A\xCA\0a7\xC8\x87GaQ\xDFV[a7\xD2\x91\x90aQ\xB5V[\x90Pa8\x14`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- creating new validators\0\0\0\0\0\0\0\x81RP\x83Qa;\x86V[a8?`@Q\x80``\x01`@R\x80`+\x81R` \x01aU=`+\x919\x82`\x01`\x01`@\x1B\x03\x16a;\x86V[_[\x85\x81\x10\x15a9UW`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05h\x01\xBC\x16\xD6t\xEC\x80\0\0a8na;\xD3V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\x8A\x91\x90aG\nV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a8\xA6W=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xCB\x91\x90aR\x05V[\x90P\x80\x84\x83\x81Q\x81\x10a8\xE0Wa8\xE0aJ\x14V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x81\x01\x83U_\x92\x90\x92R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x95\x85\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x93\x02\x19\x90\x93\x16\x92\x90\x92\x17\x90U\x01a8AV[Pa9a\x85`\x01aQoV[\x83\x03a:qW`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05\x86a9\x83a;\xD3V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\x9F\x91\x90aG\nV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a9\xBBW=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xE0\x91\x90aR\x05V[\x90P\x80\x83`\x01\x85Qa9\xF2\x91\x90aQ\xDFV[\x81Q\x81\x10a:\x02Wa:\x02aJ\x14V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x01\x82U_\x91\x90\x91R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x94\x84\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x92\x02\x19\x90\x92\x16\x91\x90\x91\x17\x90U[\x90\x97\x90\x96P\x94PPPPPV[`$T`@QcR\x85\x1D\r`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA5\n:\x1A\x90a:\xAE\x90\x85\x90`\x04\x01aF\xF8V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xC8W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra:\xEF\x91\x90\x81\x01\x90aR\x9EV[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc?e\xCF\x19`\xE0\x1B\x81R\x95\x96P`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x94c?e\xCF\x19\x94a\x074\x94\x93\x92\x89\x92`\x04\x01aT1V[``a\x0ES`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83a<\x18V[``a\x0ES`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83a<\x18V[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa2%\x92\x91\x90aT\xDEV[_Q` aU\x1D_9_Q\x90_R\x81`@Qa*`\x91\x90aG\nV[`&T`@\x80Q`\x01`\xF8\x1B` \x82\x01R_`!\x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16`,\x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01a<K\x93\x92\x91\x90aT\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1D\nW__\xFD[_` \x82\x84\x03\x12\x15a<\xDDW__\xFD[\x815a<\xE8\x81a<\xBBV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a=/W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a=\x08V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>&Wa>&a=:V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a>FWa>Fa=:V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1D\nW__\xFD[_\x82`\x1F\x83\x01\x12a>sW__\xFD[\x815a>\x86a>\x81\x82a>.V[a=\xFEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a>\xA7W__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x805a>\xBF\x81a>PV[\x83R` \x92\x83\x01\x92\x01a>\xACV[P\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a>\xE8W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xFDW__\xFD[a?\t\x85\x82\x86\x01a>dV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?$W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a?4W__\xFD[\x805a?Ba>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a?cW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a?\x85W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a?jV[\x80\x94PPPPP\x92P\x92\x90PV[_\x82`\x1F\x83\x01\x12a?\xA2W__\xFD[\x815a?\xB0a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a?\xD1W__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x805\x83R` \x92\x83\x01\x92\x01a?\xD6V[__`@\x83\x85\x03\x12\x15a?\xFFW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x14W__\xFD[a@ \x85\x82\x86\x01a>dV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@;W__\xFD[a@G\x85\x82\x86\x01a?\x93V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a@cV[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a@\xA6V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aA\x0F\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaA)`\xE0\x85\x01\x82a@QV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaAB\x82\x82a@\x94V[\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84RaA\x8D\x85\x83Qa@\xC4V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aAqV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aB\x1EW`\x1F\x19\x85\x84\x03\x01\x88RaB\x08\x83\x83QaA\xA2V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aA\xECV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aB\x8B\x90\x87\x01\x82aA\xD0V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aBPV[\x805a\x12P\x81a>PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12PW__\xFD[_`\xE0\x82\x84\x03\x12\x15aB\xCFW__\xFD[aB\xD7a=NV[\x90PaB\xE2\x82aB\xA1V[\x81RaB\xF0` \x83\x01aB\xA1V[` \x82\x01RaC\x01`@\x83\x01aB\xA1V[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaC\x1C`\x80\x83\x01aB\xACV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aC9W__\xFD[aCE\x84\x82\x85\x01a>dV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCcW__\xFD[aCo\x84\x82\x85\x01a?\x93V[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aC\x8BW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xA0W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aC\xB0W__\xFD[\x805aC\xBEa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aC\xDFW__\xFD[` \x84\x01[\x83\x81\x10\x15aD\x1FW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x01W__\xFD[aD\x10\x89` \x83\x89\x01\x01aB\xBFV[\x84RP` \x92\x83\x01\x92\x01aC\xE4V[P\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84RaDl\x85\x83Qa@QV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aDPV[_` \x82\x84\x03\x12\x15aD\x91W__\xFD[\x815a<\xE8\x81a>PV[_` \x82\x84\x03\x12\x15aD\xACW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xC1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aD\xD1W__\xFD[\x805aD\xDFa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aE\0W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aE+W\x835aE\x1A\x81a<\xBBV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aE\x07V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aEEW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aEZW__\xFD[aEf\x84\x82\x85\x01aB\xBFV[\x94\x93PPPPV[` \x81R_a<\xE8` \x83\x01\x84a@QV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aE\x92V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87RaF\x06`@\x88\x01\x82aA\xA2V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01RaF!\x81\x83aE\x80V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aE\xE0V[` \x81R_a<\xE8` \x83\x01\x84aA\xD0V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aF\xAB\x90\x87\x01\x82aE\x80V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aFpV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Qd\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aF\xD3V[` \x81R_a<\xE8` \x83\x01\x84aF\xC1V[` \x81R_a<\xE8` \x83\x01\x84aA\xA2V[`@\x81R_aG.`@\x83\x01\x85aF\xC1V[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aGVW__\xFD[PQ\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x12PW__\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15aG\x8CWaG\x8Ca=:V[P`\x1F\x83\x01`\x1F\x19\x16` \x01aG\xA1\x81a=\xFEV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15aG\xB5W__\xFD[\x82\x82` \x83\x01^_` \x84\x83\x01\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aG\xDAW__\xFD[a<\xE8\x83\x83Q` \x85\x01aGsV[_`@\x82\x84\x03\x12\x15aG\xF9W__\xFD[aH\x01a=vV[\x82Q\x81R` \x83\x01Q\x90\x91P`\x01`\x01`@\x1B\x03\x81\x11\x15aH W__\xFD[aH,\x84\x82\x85\x01aG\xCBV[` \x83\x01RP\x92\x91PPV[_\x82`\x1F\x83\x01\x12aHGW__\xFD[\x81QaHUa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aHvW__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x80Q\x83R` \x92\x83\x01\x92\x01aH{V[_` \x82\x84\x03\x12\x15aH\xA3W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xB8W__\xFD[\x82\x01``\x81\x85\x03\x12\x15aH\xC9W__\xFD[aH\xD1a=\x98V[aH\xDA\x82aG]V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xF4W__\xFD[aI\0\x86\x82\x85\x01aG\xE9V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x1EW__\xFD[\x91\x90\x91\x01\x90`@\x82\x86\x03\x12\x15aI2W__\xFD[aI:a=vV[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aIOW__\xFD[aI[\x87\x82\x86\x01aH8V[\x82RP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aIvW__\xFD[aI\x82\x87\x82\x86\x01aG\xCBV[` \x83\x01RP`@\x82\x01R\x94\x93PPPPV[\x80Q\x82R_` \x82\x01Q`@` \x85\x01RaEf`@\x85\x01\x82aA\xA2V[`\x01`\x01`@\x1B\x03\x84\x16\x81R``` \x82\x01R_aI\xD4``\x83\x01\x85aI\x95V[\x82\x81\x03`@\x84\x01R\x83Q`@\x82RaI\xEF`@\x83\x01\x82a@\x94V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaJ\x08\x82\x82aA\xA2V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aJ8W__\xFD[\x81Qa<\xE8\x81a>PV[_` \x82\x84\x03\x12\x15aJSW__\xFD[\x81Q\x80\x15\x15\x81\x14a<\xE8W__\xFD[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q``\x87RaJ\xAE``\x88\x01\x82a@QV[\x90P` \x82\x01Q\x87\x82\x03` \x89\x01RaJ\xC7\x82\x82a@\x94V[`@\x93\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x98\x90\x93\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aJ\x88V[_` \x82\x84\x03\x12\x15aK\x04W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x19W__\xFD[aEf\x84\x82\x85\x01aH8V[`\x01\x81\x81\x1C\x90\x82\x16\x80aK9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aKWWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aKmW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x82W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aK\x92W__\xFD[aEf\x84\x82Q` \x84\x01aGsV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aK\xC6WaK\xC6aK\xA1V[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R_\x83Q`@``\x84\x01RaK\xF6`\xA0\x84\x01\x82aA\xA2V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aL/\x82\x85aL\rV[`\x17`\xF9\x1B\x81RaAB`\x01\x82\x01\x85aL\rV[_`\xA0\x82\x84\x03\x12\x80\x15aLTW__\xFD[PaL]a=\xBAV[\x82Q\x81R` \x83\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14aLvW__\xFD[` \x82\x01RaL\x87`@\x84\x01aG]V[`@\x82\x01R``\x83\x01Q\x80`\x07\x0B\x81\x14aL\x9FW__\xFD[``\x82\x01RaL\xB0`\x80\x84\x01aG]V[`\x80\x82\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aL\xCCW__\xFD[a<\xE8\x82aG]V[_`@\x82\x01`@\x83R\x80\x85TaL\xEF\x81\x84\x90\x81R` \x01\x90V[_\x88\x81R` \x81 \x94P\x90\x92P[\x81`\x05\x82\x01\x10\x15aMdW\x83Td\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x85R`(\x82\x90\x1C\x81\x16` \x86\x01R`P\x82\x90\x1C\x81\x16`@\x86\x01R`x\x82\x90\x1C\x81\x16``\x86\x01R`\xA0\x82\x81\x1C\x82\x16`\x80\x87\x01R`\xC8\x92\x90\x92\x1C\x16\x90\x84\x01R`\x01\x90\x93\x01\x92`\xC0\x90\x92\x01\x91`\x06\x01aL\xFDV[\x92T\x92\x81\x81\x10\x15aM\x83Wd\xFF\xFF\xFF\xFF\xFF\x84\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aM\xA3Wd\xFF\xFF\xFF\xFF\xFF`(\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aM\xC3Wd\xFF\xFF\xFF\xFF\xFF`P\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aM\xE3Wd\xFF\xFF\xFF\xFF\xFF`x\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aN\x03Wd\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aN W`\xC8\x84\x90\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x83\x01\x92P[PP`\x01`\x01`@\x1B\x03\x85\x16` \x85\x01R\x91Pa<\xE8\x90PV[_` \x82\x84\x03\x12\x15aNJW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN_W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15aNpW__\xFD[aNxa=vV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x8DW__\xFD[aN\x99\x86\x82\x85\x01aG\xE9V[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xB4W__\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12aN\xC8W__\xFD[\x81QaN\xD6a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x87\x83\x11\x15aN\xF7W__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x83W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x19W__\xFD[\x86\x01``\x81\x8B\x03`\x1F\x19\x01\x12\x15aO.W__\xFD[aO6a=\x98V[` \x82\x81\x01Q\x82R`@\x83\x01Q\x90\x82\x01R``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO_W__\xFD[aOn\x8C` \x83\x86\x01\x01aG\xCBV[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01aN\xFCV[P` \x84\x01RP\x90\x95\x94PPPPPV[`@\x81R_aO\xA6`@\x83\x01\x85aI\x95V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01_[\x83\x81\x10\x15aP\x1CW`\x1F\x19\x86\x84\x03\x01\x85R\x81Q\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q\x90P```@\x85\x01RaP\x05``\x85\x01\x82aA\xA2V[` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01aO\xC9V[P\x90\x98\x97PPPPPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R_aAB``\x83\x01\x84aA\xA2V[``\x81R_aPZ``\x83\x01\x86a@\xC4V[\x82\x81\x03` \x84\x01RaPl\x81\x86a@QV[\x91PP\x82\x15\x15`@\x83\x01R\x94\x93PPPPV[_aP\x8A\x82\x86aL\rV[`\x17`\xF9\x1B\x81RaP\x9E`\x01\x82\x01\x86aL\rV[\x90P`\x1D`\xF9\x1B\x81RaE+`\x01\x82\x01\x85aL\rV[__`@\x83\x85\x03\x12\x15aP\xC5W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xDAW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aP\xEAW__\xFD[\x80QaP\xF8a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aQ\x19W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aQDW\x83QaQ3\x81a>PV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aQ V[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQcW__\xFD[a@G\x85\x82\x86\x01aH8V[\x80\x82\x01\x80\x82\x11\x15a\x0ESWa\x0ESaK\xA1V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0ESWa\x0ESaK\xA1V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aQ\xC3WaQ\xC3aQ\xA1V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0ESWa\x0ESaK\xA1V[\x81\x81\x03\x81\x81\x11\x15a\x0ESWa\x0ESaK\xA1V[_\x82aR\0WaR\0aQ\xA1V[P\x06\x90V[_` \x82\x84\x03\x12\x15aR\x15W__\xFD[\x81Qa<\xE8\x81a<\xBBV[_\x82`\x1F\x83\x01\x12aR/W__\xFD[\x81QaR=a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aR^W__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x80W__\xFD[aR\x8F\x88` \x83\x8A\x01\x01aH8V[\x84RP` \x92\x83\x01\x92\x01aRcV[_` \x82\x84\x03\x12\x15aR\xAEW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xC3W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15aR\xD4W__\xFD[aR\xDCa=\xDCV[aR\xE5\x82aG]V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xFFW__\xFD[aS\x0B\x86\x82\x85\x01aG\xE9V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS)W__\xFD[\x82\x01`\x1F\x81\x01\x86\x13aS9W__\xFD[\x80QaSGa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aShW__\xFD[` \x84\x01[\x83\x81\x10\x15aS\xA8W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x8AW__\xFD[aS\x99\x8B` \x83\x89\x01\x01aG\xCBV[\x84RP` \x92\x83\x01\x92\x01aSmV[P`@\x85\x01RPPP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xC9W__\xFD[aS\xD5\x86\x82\x85\x01aR V[``\x83\x01RP\x94\x93PPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aB\x1EW`\x1F\x19\x85\x84\x03\x01\x88RaT\x1B\x83\x83Qa@\x94V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aS\xFFV[`\x01`\x01`@\x1B\x03\x86\x16\x81R`\xA0` \x82\x01R_aTR`\xA0\x83\x01\x87aI\x95V[\x82\x81\x03`@\x84\x01RaTd\x81\x87aF\xC1V[\x90P\x82\x81\x03``\x84\x01R\x80\x85Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x88\x01_[\x83\x81\x10\x15aT\xBBW`\x1F\x19\x86\x84\x03\x01\x85RaT\xA5\x83\x83QaA\xA2V[` \x95\x86\x01\x95\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aT\x89V[PP\x85\x81\x03`\x80\x87\x01RaT\xCF\x81\x88aS\xE3V[\x9B\x9APPPPPPPPPPPV[`@\x81R_aT\xF0`@\x83\x01\x85aA\xA2V[\x90P\x82` \x83\x01R\x93\x92PPPV[_aABaU\x16aU\x10\x84\x88aL\rV[\x86aL\rV[\x84aL\rV\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP- depositing balance to beacon chain (gwei)- exiting all validators and completing checkpoint- submitting num checkpoint proofsUser.queueWithdrawals: length mismatch\xA2dipfsX\"\x12 \x04R(=\xE0<{\xF2\xDC@\x9E\xB6\x8B\xBB\x8B\x95D\xF4Gh\x852t*\x90\xFBc\xBA\xC9\xE9q\x04dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101de575f3560e01c8063841c1299116100fd578063ac637c7a11610092578063d6c10daf11610062578063d6c10daf14610575578063e20c9f7114610589578063f234c1bd1461059d578063fa7626d4146105bf575f5ffd5b8063ac637c7a1461050a578063b0464fdc14610529578063b5508aa91461053d578063ba414fa614610551575f5ffd5b806392ab89bb116100cd57806392ab89bb1461047d5780639de7025814610491578063a3f4df7e146104b2578063a88dbb36146104d3575f5ffd5b8063841c12991461040857806385226c811461042757806390b5162514610448578063916a17c61461045c575f5ffd5b80633d8c08d41161017357806346a5be0d1161014357806346a5be0d1461038a57806366d9a9a0146103a9578063695f4ae1146103ca5780636d336f58146103e9575f5ffd5b80633d8c08d4146102ff5780633e5e3c23146103365780633f7286f41461034a578063401be65e1461035e575f5ffd5b80632a34ade8116101ae5780632a34ade81461027f5780632ade388014610293578063344e1383146102b4578063391cc9f6146102e0575f5ffd5b8063071c25b7146101e95780631ed7831c1461020a57806320a545d91461023457806323e4817514610253575f5ffd5b366101e557005b5f5ffd5b3480156101f4575f5ffd5b50610208610203366004613ccd565b6105d8565b005b348015610215575f5ffd5b5061021e61079e565b60405161022b9190613cef565b60405180910390f35b34801561023f575f5ffd5b5061020861024e366004613ed7565b6107fe565b34801561025e575f5ffd5b5061027261026d366004613fee565b610b09565b60405161022b919061414b565b34801561028a575f5ffd5b50610208610e59565b34801561029e575f5ffd5b506102a7610fc0565b60405161022b919061422a565b3480156102bf575f5ffd5b506102d36102ce36600461437b565b6110fc565b60405161022b919061442a565b3480156102eb575f5ffd5b506102726102fa366004614481565b611255565b34801561030a575f5ffd5b5061031e61031936600461449c565b6113d0565b6040516001600160401b03909116815260200161022b565b348015610341575f5ffd5b5061021e611475565b348015610355575f5ffd5b5061021e6114d3565b348015610369575f5ffd5b5061037d610378366004614535565b611531565b60405161022b919061456e565b348015610395575f5ffd5b506102d36103a436600461437b565b6115e8565b3480156103b4575f5ffd5b506103bd611737565b60405161022b91906145ba565b3480156103d5575f5ffd5b5061037d6103e4366004614535565b61189b565b3480156103f4575f5ffd5b50610208610403366004613fee565b611951565b348015610413575f5ffd5b5061020861042236600461449c565b611c4d565b348015610432575f5ffd5b5061043b611d0d565b60405161022b9190614638565b348015610453575f5ffd5b50610208611dd8565b348015610467575f5ffd5b50610470611e88565b60405161022b919061464a565b348015610488575f5ffd5b50610272611f69565b34801561049c575f5ffd5b506104a5612284565b60405161022b91906146f8565b3480156104bd575f5ffd5b506104c6612426565b60405161022b919061470a565b3480156104de575f5ffd5b506026546104f2906001600160a01b031681565b6040516001600160a01b03909116815260200161022b565b348015610515575f5ffd5b50610208610524366004614481565b6124ad565b348015610534575f5ffd5b506104706125fa565b348015610548575f5ffd5b5061043b6126db565b34801561055c575f5ffd5b506105656127a6565b604051901515815260200161022b565b348015610580575f5ffd5b50610208612846565b348015610594575f5ffd5b5061021e6128f7565b3480156105a8575f5ffd5b506105b1612955565b60405161022b92919061471c565b3480156105ca575f5ffd5b50601f546105659060ff1681565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610629573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061064d9190614746565b50610681604051806040016040528060128152602001717665726966795374616c6542616c616e636560701b815250612a0e565b602480546040516308fa0b1360e21b815264ffffffffff841660048201525f926001600160a01b03909216916323e82c4c91015f60405180830381865afa1580156106ce573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526106f59190810190614893565b6026548151602083015160408085015190516301c8abe960e11b81529495506001600160a01b039093169363039157d2936107349392916004016149b3565b5f604051808303815f87803b15801561074b575f5ffd5b505af192505050801561075c575060015b61079a573d808015610789576040519150601f19603f3d011682016040523d82523d5f602084013e61078e565b606091505b5061079881612a6b565b505b5050565b606060168054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116107d6575b5050505050905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af115801561084f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108739190614746565b506108a36040518060400160405280600e81526020016d75706461746542616c616e63657360901b815250612a0e565b5f5b8251811015610798575f8382815181106108c1576108c1614a14565b602002602001015190505f8383815181106108de576108de614a14565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b0316036109a35761091d612ac7565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa15801561096d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109919190614746565b1561099e5761099e612b56565b610aff565b5f8190505f836001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109e4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a089190614a28565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015610a5a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a7e9190614a43565b506021546040516373d0285560e11b81526001600160a01b0386811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303815f875af1158015610ad7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610afb9190614746565b5050505b50506001016108a5565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015610b52573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b769190614746565b50610ba86040518060400160405280601081526020016f71756575655769746864726177616c7360801b815250612a0e565b602054604051631976849960e21b81523060048201525f916001600160a01b0316906365da126490602401602060405180830381865afa158015610bee573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c129190614a28565b60205460405163285e212160e21b815230600482018190529293505f916001600160a01b03169063a178848490602401602060405180830381865afa158015610c5d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c819190614746565b6040805160018082528183019092529192505f9190816020015b604080516060808201835280825260208201525f91810191909152815260200190600190039081610c9b5790505090506040518060600160405280888152602001878152602001846001600160a01b0316815250815f81518110610d0157610d01614a14565b60209081029190910101526040805160018082528183019092525f91816020015b610d2a613c62565b815260200190600190039081610d225790505090506040518060e00160405280306001600160a01b03168152602001866001600160a01b03168152602001856001600160a01b031681526020018481526020014363ffffffff16815260200189815260200188815250815f81518110610da557610da5614a14565b602090810291909101810191909152546040516306ec6e8160e11b81525f916001600160a01b031690630dd8dd0290610de2908690600401614a62565b5f604051808303815f875af1158015610dfd573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610e249190810190614af4565b9050610e4b825182516040518060600160405280602681526020016155bc60269139612e50565b509450505050505b92915050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610eaa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ece9190614746565b50610f02604051806040016040528060128152602001713932b3b4b9ba32b920b9a7b832b930ba37b960711b815250612a0e565b604080516060810182523081525f60208083018281528385019283529054602854945163024b980360e51b815284516001600160a01b039081166004830152925183166024820152925163ffffffff9081166044850152909416606483015260a06084830152600860a4830152676d6574616461746160c01b60c48301529192919091169063497300609060e4015f604051808303815f87803b158015610fa7575f5ffd5b505af1158015610fb9573d5f5f3e3d5ffd5b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b828210156110f3575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b828210156110dc578382905f5260205f2001805461105190614b25565b80601f016020809104026020016040519081016040528092919081815260200182805461107d90614b25565b80156110c85780601f1061109f576101008083540402835291602001916110c8565b820191905f5260205f20905b8154815290600101906020018083116110ab57829003601f168201915b505050505081526020019060010190611034565b505050508152505081526020019060010190610fe3565b50505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015611145573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111699190614746565b506111a86040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612a0e565b5f82516001600160401b038111156111c2576111c2613d3a565b6040519080825280602002602001820160405280156111f557816020015b60608152602001906001900390816111e05790505b5090505f5b835181101561124c5761122784828151811061121857611218614a14565b60200260200101516001612ebc565b82828151811061123957611239614a14565b60209081029190910101526001016111fa565b5090505b919050565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561129e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112c29190614746565b506113556040518060400160405280600f81526020016e666f726365556e64656c656761746560881b815250836001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611329573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526113509190810190614b5d565b6131d6565b5f61135f83613231565b6020546040516336a2fa1960e21b81526001600160a01b03868116600483015292935091169063da8be864906024015f604051808303815f875af11580156113a9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261124c9190810190614af4565b602354604080516301504d8f60e41b815290515f926001600160a01b031691631504d8f0916004808301926020929190829003018187875af1158015611418573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061143c9190614746565b5061146c6040518060400160405280600e81526020016d6578697456616c696461746f727360901b815250612a0e565b610e5382613546565b606060188054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116107d6575050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116107d6575050505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561157a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061159e9190614746565b506115dd6040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612a0e565b610e53826001612ebc565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015611631573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116559190614746565b506116946040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612a0e565b5f82516001600160401b038111156116ae576116ae613d3a565b6040519080825280602002602001820160405280156116e157816020015b60608152602001906001900390816116cc5790505b5090505f5b835181101561124c5761171284828151811061170457611704614a14565b60200260200101515f612ebc565b82828151811061172457611724614a14565b60209081029190910101526001016116e6565b6060601b805480602002602001604051908101604052809291908181526020015f905b828210156110f3578382905f5260205f2090600202016040518060400160405290815f8201805461178a90614b25565b80601f01602080910402602001604051908101604052809291908181526020018280546117b690614b25565b80156118015780601f106117d857610100808354040283529160200191611801565b820191905f5260205f20905b8154815290600101906020018083116117e457829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561188357602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116118455790505b5050505050815250508152602001906001019061175a565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af11580156118e4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119089190614746565b506119476040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612a0e565b610e53825f612ebc565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156119a2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119c69190614746565b506119fd604051806040016040528060158152602001743232b837b9b4ba24b73a37a2b4b3b2b73630bcb2b960591b815250612a0e565b5f5b8251811015610798575f838281518110611a1b57611a1b614a14565b602002602001015190505f838381518110611a3857611a38614a14565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603611aec575f611a78613689565b50905060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611ac7575f5ffd5b505af1158015611ad9573d5f5f3e3d5ffd5b50505050611ae681613a7e565b50611c43565b5f826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b29573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b4d9190614a28565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015611b9f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bc39190614a43565b506021546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303815f875af1158015611c1c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c409190614746565b50505b50506001016119ff565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611c9e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cc29190614746565b50611d016040518060400160405280601b81526020017f7665726966795769746864726177616c43726564656e7469616c730000000000815250612a0e565b611d0a81613a7e565b50565b6060601a805480602002602001604051908101604052809291908181526020015f905b828210156110f3578382905f5260205f20018054611d4d90614b25565b80601f0160208091040260200160405190810160405280929190818152602001828054611d7990614b25565b8015611dc45780601f10611d9b57610100808354040283529160200191611dc4565b820191905f5260205f20905b815481529060010190602001808311611da757829003601f168201915b505050505081526020019060010190611d30565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611e29573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e4d9190614746565b50611e7e6040518060400160405280600f81526020016e1cdd185c9d10da1958dadc1bda5b9d608a1b815250612a0e565b611e86612ac7565b565b6060601d805480602002602001604051908101604052809291908181526020015f905b828210156110f3575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611f5157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611f135790505b50505050508152505081526020019060010190611eab565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015611fb2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fd69190614746565b506120026040518060400160405280600a815260200169756e64656c656761746560b01b815250612a0e565b5f61200c30613231565b6020546040516336a2fa1960e21b81523060048201529192506001600160a01b03169063da8be864906024015f604051808303815f875af1158015612053573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261207a9190810190614af4565b505f5b815181101561227e575f51602061551d5f395f51905f526040516120ca9060208082526015908201527432bc3832b1ba34b733903bb4ba34323930bbb0b61d60591b604082015260600190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882828151811061210557612105614a14565b602002602001015160600151604051612142919060408082526007908201526603737b731b29d160cd1b6060820152602081019190915260800190565b60405180910390a17f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f82828151811061217d5761217d614a14565b602002602001015160a001515f8151811061219a5761219a614a14565b60200260200101516040516121dc9190604080825260079082015266039ba3930ba1d160cd1b60608201526001600160a01b0391909116602082015260800190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882828151811061221757612217614a14565b602002602001015160c001515f8151811061223457612234614a14565b602002602001015160405161226e9190604080825260089082015267039b430b932b99d160c51b6060820152602081019190915260800190565b60405180910390a160010161207d565b50905090565b6027546060905f906001600160401b038111156122a3576122a3613d3a565b6040519080825280602002602001820160405280156122cc578160200160208202803683370190505b5090505f80805b60275481101561241d57602454602780546001600160a01b039092169163aa47389c91908490811061230757612307614a14565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff166040518263ffffffff1660e01b8152600401612355919064ffffffffff91909116815260200190565b602060405180830381865afa158015612370573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123949190614a43565b1561241557602781815481106123ac576123ac614a14565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff168483815181106123e3576123e3614a14565b64ffffffffff909216602092830291909101909101528261240381614bb5565b935050818061241190614bb5565b9250505b6001016122d3565b50508152919050565b60606025805461243590614b25565b80601f016020809104026020016040519081016040528092919081815260200182805461246190614b25565b80156107f45780601f10612483576101008083540402835291602001916107f4565b820191905f5260205f20905b81548152906001019060200180831161248f57509395945050505050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156124fe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125229190614746565b506125846040518060400160405280600a81526020016964656c6567617465546f60b01b815250826001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611329573d5f5f3e3d5ffd5b604080518082018252606081525f602080830182905254925163eea9064b60e01b815291926001600160a01b03169163eea9064b916125c99186918691600401614bcd565b5f604051808303815f87803b1580156125e0575f5ffd5b505af11580156125f2573d5f5f3e3d5ffd5b505050505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b828210156110f3575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156126c357602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116126855790505b5050505050815250508152602001906001019061261d565b60606019805480602002602001604051908101604052809291908181526020015f905b828210156110f3578382905f5260205f2001805461271b90614b25565b80601f016020809104026020016040519081016040528092919081815260200182805461274790614b25565b80156127925780601f1061276957610100808354040283529160200191612792565b820191905f5260205f20905b81548152906001019060200180831161277557829003601f168201915b5050505050815260200190600101906126fe565b6008545f9060ff16156127bd575060085460ff1690565b604051630667f9d760e41b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa15801561281b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061283f9190614746565b1415905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612897573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128bb9190614746565b506128ef6040518060400160405280601281526020017118dbdb5c1b195d1950da1958dadc1bda5b9d60721b815250612a0e565b611e86612b56565b606060158054806020026020016040519081016040528092919081815260200182805480156107f457602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116107d6575050505050905090565b60605f60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156129a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129cd9190614746565b506129fe6040518060400160405280600f81526020016e737461727456616c696461746f727360881b815250612a0e565b612a06613689565b915091509091565b5f51602061551d5f395f51905f52612a2c612a27612426565b613b35565b612a3583613b5e565b604051602001612a46929190614c24565b60408051601f1981840301815290829052612a609161470a565b60405180910390a150565b805115612a7a57805181602001fd5b60405162461bcd60e51b815260206004820152601b60248201527f7265766572746564207769746820756e6b6e6f776e206572726f72000000000060448201526064015b60405180910390fd5b6026546040516388676cad60e01b81525f60048201526001600160a01b03909116906388676cad906024015f604051808303815f87803b158015612b09575f5ffd5b505af1925050508015612b1a575060015b611e86573d808015612b47576040519150601f19603f3d011682016040523d82523d5f602084013e612b4c565b606091505b50611d0a81612a6b565b604080518082018252601881527f2d206163746976652076616c696461746f7220636f756e7400000000000000006020808301919091526026548351632340e8d360e01b81529351612bfb946001600160a01b0390921692632340e8d392600480820193918290030181865afa158015612bd2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612bf69190614746565b613b86565b60408051808201825260128152712d2070726f6f66732072656d61696e696e6760701b602082015260265482516323e941b960e11b81529251612c9e936001600160a01b03909216916347d283729160048083019260a09291908290030181865afa158015612c6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c909190614c43565b6020015162ffffff16613b86565b602654604080516321767f9560e11b815290515f926001600160a01b0316916342ecff2a9160048083019260209291908290030181865afa158015612ce5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d099190614cbc565b9050806001600160401b03165f03612d7c5760405162461bcd60e51b815260206004820152603060248201527f557365722e5f636f6d706c657465436865636b706f696e743a206e6f2065786960448201526f1cdd1a5b99c818da1958dadc1bda5b9d60821b6064820152608401612abe565b60245460405163b1b6f6a160e01b81525f916001600160a01b03169063b1b6f6a190612daf906027908690600401614cd5565b5f60405180830381865afa158015612dc9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612df09190810190614e3a565b9050612e1960405180606001604052806022815260200161559a60229139826020015151613b86565b6026548151602083015160405163783a5d3160e11b81526001600160a01b039093169263f074ba62926125c9929091600401614f94565b6040516388b44c8560e01b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d906388b44c8590612e8b9086908690869060040161502a565b5f6040518083038186803b158015612ea1575f5ffd5b505afa158015612eb3573d5f5f3e3d5ffd5b50505050505050565b60605f8360a00151516001600160401b03811115612edc57612edc613d3a565b604051908082528060200260200182016040528015612f05578160200160208202803683370190505b5090505f5b815181101561316c575f8560a001518281518110612f2a57612f2a614a14565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316816001600160a01b0316036130d05773beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0838381518110612f8857612f88614a14565b60200260200101906001600160a01b031690816001600160a01b03168152505084156130cb57612fcf60405180606001604052806032815260200161556860329139613bb7565b612fdf612fda612284565b613546565b5060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561302c575f5ffd5b505af115801561303e573d5f5f3e3d5ffd5b5050505061304a612ac7565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa15801561309a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130be9190614746565b156130cb576130cb612b56565b613163565b806001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa15801561310c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131309190614a28565b83838151811061314257613142614a14565b60200260200101906001600160a01b031690816001600160a01b0316815250505b50600101612f0a565b50602054604051630e4cc3f960e41b81526001600160a01b039091169063e4cc3f90906131a190879085908890600401615048565b5f604051808303815f87803b1580156131b8575f5ffd5b505af11580156131ca573d5f5f3e3d5ffd5b50929695505050505050565b5f51602061551d5f395f51905f526131ef612a27612426565b6131f884613b5e565b8360405160200161320b9392919061507f565b60408051601f19818403018152908290526132259161470a565b60405180910390a15050565b6020546040516366d5ba9360e01b81526001600160a01b0383811660048301526060925f928392909116906366d5ba93906024015f60405180830381865afa15801561327f573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526132a691908101906150b4565b915091505f82516001600160401b038111156132c4576132c4613d3a565b6040519080825280602002602001820160405280156132fd57816020015b6132ea613c62565b8152602001906001900390816132e25790505b50602054604051631976849960e21b81526001600160a01b0388811660048301529293505f92909116906365da126490602401602060405180830381865afa15801561334b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061336f9190614a28565b60205460405163285e212160e21b81526001600160a01b0389811660048301529293505f929091169063a178848490602401602060405180830381865afa1580156133bc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133e09190614746565b90505f5b855181101561353a576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f9291506020808301908036833701905050905087838151811061344257613442614a14565b6020026020010151825f8151811061345c5761345c614a14565b60200260200101906001600160a01b031690816001600160a01b03168152505086838151811061348e5761348e614a14565b6020026020010151815f815181106134a8576134a8614a14565b6020026020010181815250506040518060e001604052808b6001600160a01b03168152602001866001600160a01b031681526020018b6001600160a01b0316815260200184866134f8919061516f565b81526020014363ffffffff1681526020018381526020018281525086848151811061352557613525614a14565b602090810291909101015250506001016133e4565b50919695505050505050565b5f6135876040518060400160405280601881526020017f2d2065786974696e67206e756d2076616c696461746f727300000000000000008152508351613b86565b5f5b82518110156136405760245483516001600160a01b039091169063f8f98a4e908590849081106135bb576135bb614a14565b60200260200101516040518263ffffffff1660e01b81526004016135ec919064ffffffffff91909116815260200190565b6020604051808303815f875af1158015613608573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061362c9190614cbc565b6136369083615182565b9150600101613589565b506112506040518060400160405280601e81526020017f2d206578697465642062616c616e636520746f20706f64202867776569290000815250826001600160401b0316613b86565b60605f47816136a16801bc16d674ec800000836151b5565b90506136b6816801bc16d674ec8000006151c8565b6136c090836151df565b91505f81670de0b6b3a76400008410613707576136e1633b9aca00856151f2565b6136eb90856151df565b91506136f782856151df565b93508061370381614bb5565b9150505b805f036137735760405162461bcd60e51b815260206004820152603460248201527f737461727456616c696461746f72733a206e6f7420656e6f75676820455448206044820152733a379039ba30b93a1030903b30b634b230ba37b960611b6064820152608401612abe565b5f816001600160401b0381111561378c5761378c613d3a565b6040519080825280602002602001820160405280156137b5578160200160208202803683370190505b5090505f633b9aca006137c887476151df565b6137d291906151b5565b90506138146040518060400160405280601981526020017f2d206372656174696e67206e65772076616c696461746f7273000000000000008152508351613b86565b61383f6040518060600160405280602b815260200161553d602b9139826001600160401b0316613b86565b5f5b85811015613955576024545f906001600160a01b031663ed3c16056801bc16d674ec80000061386e613bd3565b6040518363ffffffff1660e01b815260040161388a919061470a565b60206040518083038185885af11580156138a6573d5f5f3e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906138cb9190615205565b9050808483815181106138e0576138e0614a14565b64ffffffffff928316602091820292909201015260278054600181810183555f9290925260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054958516600592909306919091026101000a918202919093021990931692909217905501613841565b5061396185600161516f565b8303613a71576024545f906001600160a01b031663ed3c160586613983613bd3565b6040518363ffffffff1660e01b815260040161399f919061470a565b60206040518083038185885af11580156139bb573d5f5f3e3d5ffd5b50505050506040513d601f19601f820116820180604052508101906139e09190615205565b90508083600185516139f291906151df565b81518110613a0257613a02614a14565b64ffffffffff9283166020918202929092010152602780546001810182555f9190915260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054948416600592909306919091026101000a91820291909202199092169190911790555b9097909650945050505050565b6024546040516352851d0d60e11b81525f916001600160a01b03169063a50a3a1a90613aae9085906004016146f8565b5f60405180830381865afa158015613ac8573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613aef919081019061529e565b6026548151602083015160408085015160608601519151633f65cf1960e01b81529596506001600160a01b0390941694633f65cf19946107349493928992600401615431565b6060610e53604051806040016040528060058152602001641b5b39366d60d81b81525083613c18565b6060610e53604051806040016040528060048152602001631b5b336d60e01b81525083613c18565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882826040516132259291906154de565b5f51602061551d5f395f51905f5281604051612a60919061470a565b60265460408051600160f81b60208201525f60218201526bffffffffffffffffffffffff19606093841b16602c82015201604051602081830303815290604052905090565b60608282604051806040016040528060048152602001631b5b306d60e01b815250604051602001613c4b939291906154ff565b604051602081830303815290604052905092915050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b64ffffffffff81168114611d0a575f5ffd5b5f60208284031215613cdd575f5ffd5b8135613ce881613cbb565b9392505050565b602080825282518282018190525f918401906040840190835b81811015613d2f5783516001600160a01b0316835260209384019390920191600101613d08565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715613d7057613d70613d3a565b60405290565b604080519081016001600160401b0381118282101715613d7057613d70613d3a565b604051606081016001600160401b0381118282101715613d7057613d70613d3a565b60405160a081016001600160401b0381118282101715613d7057613d70613d3a565b604051608081016001600160401b0381118282101715613d7057613d70613d3a565b604051601f8201601f191681016001600160401b0381118282101715613e2657613e26613d3a565b604052919050565b5f6001600160401b03821115613e4657613e46613d3a565b5060051b60200190565b6001600160a01b0381168114611d0a575f5ffd5b5f82601f830112613e73575f5ffd5b8135613e86613e8182613e2e565b613dfe565b8082825260208201915060208360051b860101925085831115613ea7575f5ffd5b602085015b83811015613ecd578035613ebf81613e50565b835260209283019201613eac565b5095945050505050565b5f5f60408385031215613ee8575f5ffd5b82356001600160401b03811115613efd575f5ffd5b613f0985828601613e64565b92505060208301356001600160401b03811115613f24575f5ffd5b8301601f81018513613f34575f5ffd5b8035613f42613e8182613e2e565b8082825260208201915060208360051b850101925087831115613f63575f5ffd5b6020840193505b82841015613f85578335825260209384019390910190613f6a565b809450505050509250929050565b5f82601f830112613fa2575f5ffd5b8135613fb0613e8182613e2e565b8082825260208201915060208360051b860101925085831115613fd1575f5ffd5b602085015b83811015613ecd578035835260209283019201613fd6565b5f5f60408385031215613fff575f5ffd5b82356001600160401b03811115614014575f5ffd5b61402085828601613e64565b92505060208301356001600160401b0381111561403b575f5ffd5b61404785828601613f93565b9150509250929050565b5f8151808452602084019350602083015f5b8281101561408a5781516001600160a01b0316865260209586019590910190600101614063565b5093949350505050565b5f8151808452602084019350602083015f5b8281101561408a5781518652602095860195909101906001016140a6565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f9161410f9085018263ffffffff169052565b5060a082015160e060a085015261412960e0850182614051565b905060c083015184820360c08601526141428282614094565b95945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f1987860301845261418d8583516140c4565b94506020938401939190910190600101614171565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561421e57601f198584030188526142088383516141a2565b60209889019890935091909101906001016141ec565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57868503603f19018452815180516001600160a01b0316865260209081015160409187018290529061428b908701826141d0565b9550506020938401939190910190600101614250565b803561125081613e50565b803563ffffffff81168114611250575f5ffd5b5f60e082840312156142cf575f5ffd5b6142d7613d4e565b90506142e2826142a1565b81526142f0602083016142a1565b6020820152614301604083016142a1565b60408201526060828101359082015261431c608083016142ac565b608082015260a08201356001600160401b03811115614339575f5ffd5b61434584828501613e64565b60a08301525060c08201356001600160401b03811115614363575f5ffd5b61436f84828501613f93565b60c08301525092915050565b5f6020828403121561438b575f5ffd5b81356001600160401b038111156143a0575f5ffd5b8201601f810184136143b0575f5ffd5b80356143be613e8182613e2e565b8082825260208201915060208360051b8501019250868311156143df575f5ffd5b602084015b8381101561441f5780356001600160401b03811115614401575f5ffd5b614410896020838901016142bf565b845250602092830192016143e4565b509695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f1987860301845261446c858351614051565b94506020938401939190910190600101614450565b5f60208284031215614491575f5ffd5b8135613ce881613e50565b5f602082840312156144ac575f5ffd5b81356001600160401b038111156144c1575f5ffd5b8201601f810184136144d1575f5ffd5b80356144df613e8182613e2e565b8082825260208201915060208360051b850101925086831115614500575f5ffd5b6020840193505b8284101561452b57833561451a81613cbb565b825260209384019390910190614507565b9695505050505050565b5f60208284031215614545575f5ffd5b81356001600160401b0381111561455a575f5ffd5b614566848285016142bf565b949350505050565b602081525f613ce86020830184614051565b5f8151808452602084019350602083015f5b8281101561408a5781516001600160e01b031916865260209586019590910190600101614592565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f19878603018452815180516040875261460660408801826141a2565b90506020820151915086810360208801526146218183614580565b9650505060209384019391909101906001016145e0565b602081525f613ce860208301846141d0565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57868503603f19018452815180516001600160a01b031686526020908101516040918701829052906146ab90870182614580565b9550506020938401939190910190600101614670565b5f8151808452602084019350602083015f5b8281101561408a57815164ffffffffff168652602095860195909101906001016146d3565b602081525f613ce860208301846146c1565b602081525f613ce860208301846141a2565b604081525f61472e60408301856146c1565b90506001600160401b03831660208301529392505050565b5f60208284031215614756575f5ffd5b5051919050565b80516001600160401b0381168114611250575f5ffd5b5f5f6001600160401b0384111561478c5761478c613d3a565b50601f8301601f19166020016147a181613dfe565b9150508281528383830111156147b5575f5ffd5b8282602083015e5f602084830101529392505050565b5f82601f8301126147da575f5ffd5b613ce883835160208501614773565b5f604082840312156147f9575f5ffd5b614801613d76565b8251815260208301519091506001600160401b03811115614820575f5ffd5b61482c848285016147cb565b60208301525092915050565b5f82601f830112614847575f5ffd5b8151614855613e8182613e2e565b8082825260208201915060208360051b860101925085831115614876575f5ffd5b602085015b83811015613ecd57805183526020928301920161487b565b5f602082840312156148a3575f5ffd5b81516001600160401b038111156148b8575f5ffd5b8201606081850312156148c9575f5ffd5b6148d1613d98565b6148da8261475d565b815260208201516001600160401b038111156148f4575f5ffd5b614900868285016147e9565b60208301525060408201516001600160401b0381111561491e575f5ffd5b919091019060408286031215614932575f5ffd5b61493a613d76565b82516001600160401b0381111561494f575f5ffd5b61495b87828601614838565b82525060208301516001600160401b03811115614976575f5ffd5b614982878286016147cb565b6020830152506040820152949350505050565b805182525f60208201516040602085015261456660408501826141a2565b6001600160401b0384168152606060208201525f6149d46060830185614995565b82810360408401528351604082526149ef6040830182614094565b905060208501518282036020840152614a0882826141a2565b98975050505050505050565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614a38575f5ffd5b8151613ce881613e50565b5f60208284031215614a53575f5ffd5b81518015158114613ce8575f5ffd5b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156131ca57603f198786030184528151805160608752614aae6060880182614051565b905060208201518782036020890152614ac78282614094565b6040938401516001600160a01b031698909301979097525094506020938401939190910190600101614a88565b5f60208284031215614b04575f5ffd5b81516001600160401b03811115614b19575f5ffd5b61456684828501614838565b600181811c90821680614b3957607f821691505b602082108103614b5757634e487b7160e01b5f52602260045260245ffd5b50919050565b5f60208284031215614b6d575f5ffd5b81516001600160401b03811115614b82575f5ffd5b8201601f81018413614b92575f5ffd5b61456684825160208401614773565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614bc657614bc6614ba1565b5060010190565b60018060a01b0384168152606060208201525f835160406060840152614bf660a08401826141a2565b602095909501516080840152505060400152919050565b5f81518060208401855e5f93019283525090919050565b5f614c2f8285614c0d565b601760f91b81526141426001820185614c0d565b5f60a0828403128015614c54575f5ffd5b50614c5d613dba565b82518152602083015162ffffff81168114614c76575f5ffd5b6020820152614c876040840161475d565b604082015260608301518060070b8114614c9f575f5ffd5b6060820152614cb06080840161475d565b60808201529392505050565b5f60208284031215614ccc575f5ffd5b613ce88261475d565b5f6040820160408352808554614cef818490815260200190565b5f8881526020812094509092505b81600582011015614d6457835464ffffffffff8082168552602882901c81166020860152605082901c81166040860152607882901c8116606086015260a082811c8216608087015260c89290921c169084015260019093019260c090920191600601614cfd565b92549281811015614d835764ffffffffff841683526020909201916001015b81811015614da35764ffffffffff602885901c1683526020909201916001015b81811015614dc35764ffffffffff605085901c1683526020909201916001015b81811015614de35764ffffffffff607885901c1683526020909201916001015b81811015614e035764ffffffffff60a085901c1683526020909201916001015b81811015614e205760c884901c64ffffffffff1683526020830192505b50506001600160401b03851660208501529150613ce89050565b5f60208284031215614e4a575f5ffd5b81516001600160401b03811115614e5f575f5ffd5b820160408185031215614e70575f5ffd5b614e78613d76565b81516001600160401b03811115614e8d575f5ffd5b614e99868285016147e9565b82525060208201516001600160401b03811115614eb4575f5ffd5b80830192505084601f830112614ec8575f5ffd5b8151614ed6613e8182613e2e565b8082825260208201915060208360051b860101925087831115614ef7575f5ffd5b602085015b83811015614f835780516001600160401b03811115614f19575f5ffd5b86016060818b03601f19011215614f2e575f5ffd5b614f36613d98565b602082810151825260408301519082015260608201516001600160401b03811115614f5f575f5ffd5b614f6e8c6020838601016147cb565b60408301525084525060209283019201614efc565b506020840152509095945050505050565b604081525f614fa66040830185614995565b828103602084015280845180835260208301915060208160051b840101602087015f5b8381101561501c57601f1986840301855281518051845260208101516020850152604081015190506060604085015261500560608501826141a2565b602096870196909450929092019150600101614fc9565b509098975050505050505050565b838152826020820152606060408201525f61414260608301846141a2565b606081525f61505a60608301866140c4565b828103602084015261506c8186614051565b9150508215156040830152949350505050565b5f61508a8286614c0d565b601760f91b815261509e6001820186614c0d565b9050601d60f91b815261452b6001820185614c0d565b5f5f604083850312156150c5575f5ffd5b82516001600160401b038111156150da575f5ffd5b8301601f810185136150ea575f5ffd5b80516150f8613e8182613e2e565b8082825260208201915060208360051b850101925087831115615119575f5ffd5b6020840193505b8284101561514457835161513381613e50565b825260209384019390910190615120565b8095505050505060208301516001600160401b03811115615163575f5ffd5b61404785828601614838565b80820180821115610e5357610e53614ba1565b6001600160401b038181168382160190811115610e5357610e53614ba1565b634e487b7160e01b5f52601260045260245ffd5b5f826151c3576151c36151a1565b500490565b8082028115828204841417610e5357610e53614ba1565b81810381811115610e5357610e53614ba1565b5f82615200576152006151a1565b500690565b5f60208284031215615215575f5ffd5b8151613ce881613cbb565b5f82601f83011261522f575f5ffd5b815161523d613e8182613e2e565b8082825260208201915060208360051b86010192508583111561525e575f5ffd5b602085015b83811015613ecd5780516001600160401b03811115615280575f5ffd5b61528f886020838a0101614838565b84525060209283019201615263565b5f602082840312156152ae575f5ffd5b81516001600160401b038111156152c3575f5ffd5b8201608081850312156152d4575f5ffd5b6152dc613ddc565b6152e58261475d565b815260208201516001600160401b038111156152ff575f5ffd5b61530b868285016147e9565b60208301525060408201516001600160401b03811115615329575f5ffd5b8201601f81018613615339575f5ffd5b8051615347613e8182613e2e565b8082825260208201915060208360051b850101925088831115615368575f5ffd5b602084015b838110156153a85780516001600160401b0381111561538a575f5ffd5b6153998b6020838901016147cb565b8452506020928301920161536d565b50604085015250505060608201516001600160401b038111156153c9575f5ffd5b6153d586828501615220565b606083015250949350505050565b5f82825180855260208501945060208160051b830101602085015f5b8381101561421e57601f1985840301885261541b838351614094565b60209889019890935091909101906001016153ff565b6001600160401b038616815260a060208201525f61545260a0830187614995565b828103604084015261546481876146c1565b9050828103606084015280855180835260208301915060208160051b840101602088015f5b838110156154bb57601f198684030185526154a58383516141a2565b6020958601959093509190910190600101615489565b505085810360808701526154cf81886153e3565b9b9a5050505050505050505050565b604081525f6154f060408301856141a2565b90508260208301529392505050565b5f6141426155166155108488614c0d565b86614c0d565b84614c0d56fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f502d206465706f736974696e672062616c616e636520746f20626561636f6e20636861696e202867776569292d2065786974696e6720616c6c2076616c696461746f727320616e6420636f6d706c6574696e6720636865636b706f696e742d207375626d697474696e67206e756d20636865636b706f696e742070726f6f6673557365722e71756575655769746864726177616c733a206c656e677468206d69736d61746368a26469706673582212200452283de03c7bf2dc409eb68bbb8b9544f447688532742a90fb63bac9e9710464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xDEW_5`\xE0\x1C\x80c\x84\x1C\x12\x99\x11a\0\xFDW\x80c\xACc|z\x11a\0\x92W\x80c\xD6\xC1\r\xAF\x11a\0bW\x80c\xD6\xC1\r\xAF\x14a\x05uW\x80c\xE2\x0C\x9Fq\x14a\x05\x89W\x80c\xF24\xC1\xBD\x14a\x05\x9DW\x80c\xFAv&\xD4\x14a\x05\xBFW__\xFD[\x80c\xACc|z\x14a\x05\nW\x80c\xB0FO\xDC\x14a\x05)W\x80c\xB5P\x8A\xA9\x14a\x05=W\x80c\xBAAO\xA6\x14a\x05QW__\xFD[\x80c\x92\xAB\x89\xBB\x11a\0\xCDW\x80c\x92\xAB\x89\xBB\x14a\x04}W\x80c\x9D\xE7\x02X\x14a\x04\x91W\x80c\xA3\xF4\xDF~\x14a\x04\xB2W\x80c\xA8\x8D\xBB6\x14a\x04\xD3W__\xFD[\x80c\x84\x1C\x12\x99\x14a\x04\x08W\x80c\x85\"l\x81\x14a\x04'W\x80c\x90\xB5\x16%\x14a\x04HW\x80c\x91j\x17\xC6\x14a\x04\\W__\xFD[\x80c=\x8C\x08\xD4\x11a\x01sW\x80cF\xA5\xBE\r\x11a\x01CW\x80cF\xA5\xBE\r\x14a\x03\x8AW\x80cf\xD9\xA9\xA0\x14a\x03\xA9W\x80ci_J\xE1\x14a\x03\xCAW\x80cm3oX\x14a\x03\xE9W__\xFD[\x80c=\x8C\x08\xD4\x14a\x02\xFFW\x80c>^<#\x14a\x036W\x80c?r\x86\xF4\x14a\x03JW\x80c@\x1B\xE6^\x14a\x03^W__\xFD[\x80c*4\xAD\xE8\x11a\x01\xAEW\x80c*4\xAD\xE8\x14a\x02\x7FW\x80c*\xDE8\x80\x14a\x02\x93W\x80c4N\x13\x83\x14a\x02\xB4W\x80c9\x1C\xC9\xF6\x14a\x02\xE0W__\xFD[\x80c\x07\x1C%\xB7\x14a\x01\xE9W\x80c\x1E\xD7\x83\x1C\x14a\x02\nW\x80c \xA5E\xD9\x14a\x024W\x80c#\xE4\x81u\x14a\x02SW__\xFD[6a\x01\xE5W\0[__\xFD[4\x80\x15a\x01\xF4W__\xFD[Pa\x02\x08a\x02\x036`\x04a<\xCDV[a\x05\xD8V[\0[4\x80\x15a\x02\x15W__\xFD[Pa\x02\x1Ea\x07\x9EV[`@Qa\x02+\x91\x90a<\xEFV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02?W__\xFD[Pa\x02\x08a\x02N6`\x04a>\xD7V[a\x07\xFEV[4\x80\x15a\x02^W__\xFD[Pa\x02ra\x02m6`\x04a?\xEEV[a\x0B\tV[`@Qa\x02+\x91\x90aAKV[4\x80\x15a\x02\x8AW__\xFD[Pa\x02\x08a\x0EYV[4\x80\x15a\x02\x9EW__\xFD[Pa\x02\xA7a\x0F\xC0V[`@Qa\x02+\x91\x90aB*V[4\x80\x15a\x02\xBFW__\xFD[Pa\x02\xD3a\x02\xCE6`\x04aC{V[a\x10\xFCV[`@Qa\x02+\x91\x90aD*V[4\x80\x15a\x02\xEBW__\xFD[Pa\x02ra\x02\xFA6`\x04aD\x81V[a\x12UV[4\x80\x15a\x03\nW__\xFD[Pa\x03\x1Ea\x03\x196`\x04aD\x9CV[a\x13\xD0V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x03AW__\xFD[Pa\x02\x1Ea\x14uV[4\x80\x15a\x03UW__\xFD[Pa\x02\x1Ea\x14\xD3V[4\x80\x15a\x03iW__\xFD[Pa\x03}a\x03x6`\x04aE5V[a\x151V[`@Qa\x02+\x91\x90aEnV[4\x80\x15a\x03\x95W__\xFD[Pa\x02\xD3a\x03\xA46`\x04aC{V[a\x15\xE8V[4\x80\x15a\x03\xB4W__\xFD[Pa\x03\xBDa\x177V[`@Qa\x02+\x91\x90aE\xBAV[4\x80\x15a\x03\xD5W__\xFD[Pa\x03}a\x03\xE46`\x04aE5V[a\x18\x9BV[4\x80\x15a\x03\xF4W__\xFD[Pa\x02\x08a\x04\x036`\x04a?\xEEV[a\x19QV[4\x80\x15a\x04\x13W__\xFD[Pa\x02\x08a\x04\"6`\x04aD\x9CV[a\x1CMV[4\x80\x15a\x042W__\xFD[Pa\x04;a\x1D\rV[`@Qa\x02+\x91\x90aF8V[4\x80\x15a\x04SW__\xFD[Pa\x02\x08a\x1D\xD8V[4\x80\x15a\x04gW__\xFD[Pa\x04pa\x1E\x88V[`@Qa\x02+\x91\x90aFJV[4\x80\x15a\x04\x88W__\xFD[Pa\x02ra\x1FiV[4\x80\x15a\x04\x9CW__\xFD[Pa\x04\xA5a\"\x84V[`@Qa\x02+\x91\x90aF\xF8V[4\x80\x15a\x04\xBDW__\xFD[Pa\x04\xC6a$&V[`@Qa\x02+\x91\x90aG\nV[4\x80\x15a\x04\xDEW__\xFD[P`&Ta\x04\xF2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02+V[4\x80\x15a\x05\x15W__\xFD[Pa\x02\x08a\x05$6`\x04aD\x81V[a$\xADV[4\x80\x15a\x054W__\xFD[Pa\x04pa%\xFAV[4\x80\x15a\x05HW__\xFD[Pa\x04;a&\xDBV[4\x80\x15a\x05\\W__\xFD[Pa\x05ea'\xA6V[`@Q\x90\x15\x15\x81R` \x01a\x02+V[4\x80\x15a\x05\x80W__\xFD[Pa\x02\x08a(FV[4\x80\x15a\x05\x94W__\xFD[Pa\x02\x1Ea(\xF7V[4\x80\x15a\x05\xA8W__\xFD[Pa\x05\xB1a)UV[`@Qa\x02+\x92\x91\x90aG\x1CV[4\x80\x15a\x05\xCAW__\xFD[P`\x1FTa\x05e\x90`\xFF\x16\x81V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06M\x91\x90aGFV[Pa\x06\x81`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qverifyStaleBalance`p\x1B\x81RPa*\x0EV[`$\x80T`@Qc\x08\xFA\x0B\x13`\xE2\x1B\x81Rd\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c#\xE8,L\x91\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xCEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x06\xF5\x91\x90\x81\x01\x90aH\x93V[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x01\xC8\xAB\xE9`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93c\x03\x91W\xD2\x93a\x074\x93\x92\x91`\x04\x01aI\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07KW__\xFD[PZ\xF1\x92PPP\x80\x15a\x07\\WP`\x01[a\x07\x9AW=\x80\x80\x15a\x07\x89W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\x8EV[``\x91P[Pa\x07\x98\x81a*kV[P[PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6W[PPPPP\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08s\x91\x90aGFV[Pa\x08\xA3`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mupdateBalances`\x90\x1B\x81RPa*\x0EV[_[\x82Q\x81\x10\x15a\x07\x98W_\x83\x82\x81Q\x81\x10a\x08\xC1Wa\x08\xC1aJ\x14V[` \x02` \x01\x01Q\x90P_\x83\x83\x81Q\x81\x10a\x08\xDEWa\x08\xDEaJ\x14V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\xA3Wa\t\x1Da*\xC7V[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tmW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x91\x91\x90aGFV[\x15a\t\x9EWa\t\x9Ea+VV[a\n\xFFV[_\x81\x90P_\x83`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x08\x91\x90aJ(V[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\nZW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n~\x91\x90aJCV[P`!T`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xFB\x91\x90aGFV[PPP[PP`\x01\x01a\x08\xA5V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x0BRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bv\x91\x90aGFV[Pa\x0B\xA8`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oqueueWithdrawals`\x80\x1B\x81RPa*\x0EV[` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R0`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x12\x91\x90aJ(V[` T`@Qc(^!!`\xE2\x1B\x81R0`\x04\x82\x01\x81\x90R\x92\x93P_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C]W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x81\x91\x90aGFV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R_\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x9BW\x90PP\x90P`@Q\x80``\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81_\x81Q\x81\x10a\r\x01Wa\r\x01aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[a\r*a<bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\"W\x90PP\x90P`@Q\x80`\xE0\x01`@R\x800`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x88\x81RP\x81_\x81Q\x81\x10a\r\xA5Wa\r\xA5aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91RT`@Qc\x06\xECn\x81`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\r\xD8\xDD\x02\x90a\r\xE2\x90\x86\x90`\x04\x01aJbV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xFDW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E$\x91\x90\x81\x01\x90aJ\xF4V[\x90Pa\x0EK\x82Q\x82Q`@Q\x80``\x01`@R\x80`&\x81R` \x01aU\xBC`&\x919a.PV[P\x94PPPPP[\x92\x91PPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xAAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCE\x91\x90aGFV[Pa\x0F\x02`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q92\xB3\xB4\xB9\xBA2\xB9 \xB9\xA7\xB82\xB90\xBA7\xB9`q\x1B\x81RPa*\x0EV[`@\x80Q``\x81\x01\x82R0\x81R_` \x80\x83\x01\x82\x81R\x83\x85\x01\x92\x83R\x90T`(T\x94Qc\x02K\x98\x03`\xE5\x1B\x81R\x84Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x92Q\x83\x16`$\x82\x01R\x92Qc\xFF\xFF\xFF\xFF\x90\x81\x16`D\x85\x01R\x90\x94\x16`d\x83\x01R`\xA0`\x84\x83\x01R`\x08`\xA4\x83\x01Rgmetadata`\xC0\x1B`\xC4\x83\x01R\x91\x92\x91\x90\x91\x16\x90cIs\0`\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xA7W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xB9W=__>=_\xFD[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x10\xDCW\x83\x82\x90_R` _ \x01\x80Ta\x10Q\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10}\x90aK%V[\x80\x15a\x10\xC8W\x80`\x1F\x10a\x10\x9FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\xC8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x104V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xE3V[PPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x11EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11i\x91\x90aGFV[Pa\x11\xA8`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa*\x0EV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xC2Wa\x11\xC2a=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xF5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\xE0W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12LWa\x12'\x84\x82\x81Q\x81\x10a\x12\x18Wa\x12\x18aJ\x14V[` \x02` \x01\x01Q`\x01a.\xBCV[\x82\x82\x81Q\x81\x10a\x129Wa\x129aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x11\xFAV[P\x90P[\x91\x90PV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x12\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xC2\x91\x90aGFV[Pa\x13U`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nforceUndelegate`\x88\x1B\x81RP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13)W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13P\x91\x90\x81\x01\x90aK]V[a1\xD6V[_a\x13_\x83a21V[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xA9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12L\x91\x90\x81\x01\x90aJ\xF4V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\x14\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14<\x91\x90aGFV[Pa\x14l`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mexitValidators`\x90\x1B\x81RPa*\x0EV[a\x0ES\x82a5FV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6WPPPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x15zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9E\x91\x90aGFV[Pa\x15\xDD`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa*\x0EV[a\x0ES\x82`\x01a.\xBCV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x161W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16U\x91\x90aGFV[Pa\x16\x94`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa*\x0EV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xAEWa\x16\xAEa=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xE1W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\xCCW\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12LWa\x17\x12\x84\x82\x81Q\x81\x10a\x17\x04Wa\x17\x04aJ\x14V[` \x02` \x01\x01Q_a.\xBCV[\x82\x82\x81Q\x81\x10a\x17$Wa\x17$aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x16\xE6V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x17\x8A\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xB6\x90aK%V[\x80\x15a\x18\x01W\x80`\x1F\x10a\x17\xD8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x01V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18\x83W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18EW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x17ZV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x18\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x08\x91\x90aGFV[Pa\x19G`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa*\x0EV[a\x0ES\x82_a.\xBCV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC6\x91\x90aGFV[Pa\x19\xFD`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t22\xB87\xB9\xB4\xBA$\xB7:7\xA2\xB4\xB3\xB2\xB760\xBC\xB2\xB9`Y\x1B\x81RPa*\x0EV[_[\x82Q\x81\x10\x15a\x07\x98W_\x83\x82\x81Q\x81\x10a\x1A\x1BWa\x1A\x1BaJ\x14V[` \x02` \x01\x01Q\x90P_\x83\x83\x81Q\x81\x10a\x1A8Wa\x1A8aJ\x14V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1A\xECW_a\x1Axa6\x89V[P\x90P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xC7W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xD9W=__>=_\xFD[PPPPa\x1A\xE6\x81a:~V[Pa\x1CCV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BM\x91\x90aJ(V[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC3\x91\x90aJCV[P`!T`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C@\x91\x90aGFV[PP[PP`\x01\x01a\x19\xFFV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x9EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xC2\x91\x90aGFV[Pa\x1D\x01`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FverifyWithdrawalCredentials\0\0\0\0\0\x81RPa*\x0EV[a\x1D\n\x81a:~V[PV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W\x83\x82\x90_R` _ \x01\x80Ta\x1DM\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Dy\x90aK%V[\x80\x15a\x1D\xC4W\x80`\x1F\x10a\x1D\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1D0V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EM\x91\x90aGFV[Pa\x1E~`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x1C\xDD\x18\\\x9D\x10\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x8A\x1B\x81RPa*\x0EV[a\x1E\x86a*\xC7V[V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1FQW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1F\x13W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xABV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x1F\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xD6\x91\x90aGFV[Pa \x02`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iundelegate`\xB0\x1B\x81RPa*\x0EV[_a \x0C0a21V[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a SW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra z\x91\x90\x81\x01\x90aJ\xF4V[P_[\x81Q\x81\x10\x15a\"~W_Q` aU\x1D_9_Q\x90_R`@Qa \xCA\x90` \x80\x82R`\x15\x90\x82\x01Rt2\xBC82\xB1\xBA4\xB73\x90;\xB4\xBA4290\xBB\xB0\xB6\x1D`Y\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a!\x05Wa!\x05aJ\x14V[` \x02` \x01\x01Q``\x01Q`@Qa!B\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03s{s\x1B)\xD1`\xCD\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x82\x82\x81Q\x81\x10a!}Wa!}aJ\x14V[` \x02` \x01\x01Q`\xA0\x01Q_\x81Q\x81\x10a!\x9AWa!\x9AaJ\x14V[` \x02` \x01\x01Q`@Qa!\xDC\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03\x9B\xA3\x93\x0B\xA1\xD1`\xCD\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a\"\x17Wa\"\x17aJ\x14V[` \x02` \x01\x01Q`\xC0\x01Q_\x81Q\x81\x10a\"4Wa\"4aJ\x14V[` \x02` \x01\x01Q`@Qa\"n\x91\x90`@\x80\x82R`\x08\x90\x82\x01Rg\x03\x9BC\x0B\x93+\x99\xD1`\xC5\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a }V[P\x90P\x90V[`'T``\x90_\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xA3Wa\"\xA3a=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xCCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80\x80[`'T\x81\x10\x15a$\x1DW`$T`'\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xAAG8\x9C\x91\x90\x84\x90\x81\x10a#\x07Wa#\x07aJ\x14V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#U\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#pW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x94\x91\x90aJCV[\x15a$\x15W`'\x81\x81T\x81\x10a#\xACWa#\xACaJ\x14V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10a#\xE3Wa#\xE3aJ\x14V[d\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a$\x03\x81aK\xB5V[\x93PP\x81\x80a$\x11\x90aK\xB5V[\x92PP[`\x01\x01a\"\xD3V[PP\x81R\x91\x90PV[```%\x80Ta$5\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$a\x90aK%V[\x80\x15a\x07\xF4W\x80`\x1F\x10a$\x83Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\xF4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x8FWP\x93\x95\x94PPPPPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\"\x91\x90aGFV[Pa%\x84`@Q\x80`@\x01`@R\x80`\n\x81R` \x01idelegateTo`\xB0\x1B\x81RP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13)W=__>=_\xFD[`@\x80Q\x80\x82\x01\x82R``\x81R_` \x80\x83\x01\x82\x90RT\x92Qc\xEE\xA9\x06K`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE\xA9\x06K\x91a%\xC9\x91\x86\x91\x86\x91`\x04\x01aK\xCDV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a%\xE0W__\xFD[PZ\xF1\x15\x80\x15a%\xF2W=__>=_\xFD[PPPPPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a&\xC3W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a&\x85W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a&\x1DV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10\xF3W\x83\x82\x90_R` _ \x01\x80Ta'\x1B\x90aK%V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta'G\x90aK%V[\x80\x15a'\x92W\x80`\x1F\x10a'iWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\x92V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'uW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a&\xFEV[`\x08T_\x90`\xFF\x16\x15a'\xBDWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(?\x91\x90aGFV[\x14\x15\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\x97W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xBB\x91\x90aGFV[Pa(\xEF`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\xDB\xDB\\\x1B\x19]\x19P\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`r\x1B\x81RPa*\x0EV[a\x1E\x86a+VV[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\xF4W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\xD6WPPPPP\x90P\x90V[``_`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xCD\x91\x90aGFV[Pa)\xFE`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nstartValidators`\x88\x1B\x81RPa*\x0EV[a*\x06a6\x89V[\x91P\x91P\x90\x91V[_Q` aU\x1D_9_Q\x90_Ra*,a*'a$&V[a;5V[a*5\x83a;^V[`@Q` \x01a*F\x92\x91\x90aL$V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra*`\x91aG\nV[`@Q\x80\x91\x03\x90\xA1PV[\x80Q\x15a*zW\x80Q\x81` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Freverted with unknown error\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`&T`@Qc\x88gl\xAD`\xE0\x1B\x81R_`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x88gl\xAD\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+\tW__\xFD[PZ\xF1\x92PPP\x80\x15a+\x1AWP`\x01[a\x1E\x86W=\x80\x80\x15a+GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a+LV[``\x91P[Pa\x1D\n\x81a*kV[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F- active validator count\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R`&T\x83Qc#@\xE8\xD3`\xE0\x1B\x81R\x93Qa+\xFB\x94`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c#@\xE8\xD3\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+\xD2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xF6\x91\x90aGFV[a;\x86V[`@\x80Q\x80\x82\x01\x82R`\x12\x81Rq- proofs remaining`p\x1B` \x82\x01R`&T\x82Qc#\xE9A\xB9`\xE1\x1B\x81R\x92Qa,\x9E\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cG\xD2\x83r\x91`\x04\x80\x83\x01\x92`\xA0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,lW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x90\x91\x90aLCV[` \x01Qb\xFF\xFF\xFF\x16a;\x86V[`&T`@\x80Qc!v\x7F\x95`\xE1\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cB\xEC\xFF*\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\xE5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\t\x91\x90aL\xBCV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a-|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FUser._completeCheckpoint: no exi`D\x82\x01Ro\x1C\xDD\x1A[\x99\xC8\x18\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x82\x1B`d\x82\x01R`\x84\x01a*\xBEV[`$T`@Qc\xB1\xB6\xF6\xA1`\xE0\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xB6\xF6\xA1\x90a-\xAF\x90`'\x90\x86\x90`\x04\x01aL\xD5V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xC9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\xF0\x91\x90\x81\x01\x90aN:V[\x90Pa.\x19`@Q\x80``\x01`@R\x80`\"\x81R` \x01aU\x9A`\"\x919\x82` \x01QQa;\x86V[`&T\x81Q` \x83\x01Q`@Qcx:]1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF0t\xBAb\x92a%\xC9\x92\x90\x91`\x04\x01aO\x94V[`@Qc\x88\xB4L\x85`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x88\xB4L\x85\x90a.\x8B\x90\x86\x90\x86\x90\x86\x90`\x04\x01aP*V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a.\xA1W__\xFD[PZ\xFA\x15\x80\x15a.\xB3W=__>=_\xFD[PPPPPPPV[``_\x83`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xDCWa.\xDCa=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a/\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a1lW_\x85`\xA0\x01Q\x82\x81Q\x81\x10a/*Wa/*aJ\x14V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a0\xD0Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x83\x83\x81Q\x81\x10a/\x88Wa/\x88aJ\x14V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x15a0\xCBWa/\xCF`@Q\x80``\x01`@R\x80`2\x81R` \x01aUh`2\x919a;\xB7V[a/\xDFa/\xDAa\"\x84V[a5FV[P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0,W__\xFD[PZ\xF1\x15\x80\x15a0>W=__>=_\xFD[PPPPa0Ja*\xC7V[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xBE\x91\x90aGFV[\x15a0\xCBWa0\xCBa+VV[a1cV[\x80`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x0CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a10\x91\x90aJ(V[\x83\x83\x81Q\x81\x10a1BWa1BaJ\x14V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP[P`\x01\x01a/\nV[P` T`@Qc\x0EL\xC3\xF9`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\xCC?\x90\x90a1\xA1\x90\x87\x90\x85\x90\x88\x90`\x04\x01aPHV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xB8W__\xFD[PZ\xF1\x15\x80\x15a1\xCAW=__>=_\xFD[P\x92\x96\x95PPPPPPV[_Q` aU\x1D_9_Q\x90_Ra1\xEFa*'a$&V[a1\xF8\x84a;^V[\x83`@Q` \x01a2\x0B\x93\x92\x91\x90aP\x7FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra2%\x91aG\nV[`@Q\x80\x91\x03\x90\xA1PPV[` T`@Qcf\xD5\xBA\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R``\x92_\x92\x83\x92\x90\x91\x16\x90cf\xD5\xBA\x93\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x7FW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra2\xA6\x91\x90\x81\x01\x90aP\xB4V[\x91P\x91P_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xC4Wa2\xC4a=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xFDW\x81` \x01[a2\xEAa<bV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a2\xE2W\x90P[P` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3KW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3o\x91\x90aJ(V[` T`@Qc(^!!`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xE0\x91\x90aGFV[\x90P_[\x85Q\x81\x10\x15a5:W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a4BWa4BaJ\x14V[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a4\\Wa4\\aJ\x14V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x86\x83\x81Q\x81\x10a4\x8EWa4\x8EaJ\x14V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a4\xA8Wa4\xA8aJ\x14V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x86a4\xF8\x91\x90aQoV[\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81RP\x86\x84\x81Q\x81\x10a5%Wa5%aJ\x14V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a3\xE4V[P\x91\x96\x95PPPPPPV[_a5\x87`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F- exiting num validators\0\0\0\0\0\0\0\0\x81RP\x83Qa;\x86V[_[\x82Q\x81\x10\x15a6@W`$T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xF9\x8AN\x90\x85\x90\x84\x90\x81\x10a5\xBBWa5\xBBaJ\x14V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a5\xEC\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a6\x08W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6,\x91\x90aL\xBCV[a66\x90\x83aQ\x82V[\x91P`\x01\x01a5\x89V[Pa\x12P`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7F- exited balance to pod (gwei)\0\0\x81RP\x82`\x01`\x01`@\x1B\x03\x16a;\x86V[``_G\x81a6\xA1h\x01\xBC\x16\xD6t\xEC\x80\0\0\x83aQ\xB5V[\x90Pa6\xB6\x81h\x01\xBC\x16\xD6t\xEC\x80\0\0aQ\xC8V[a6\xC0\x90\x83aQ\xDFV[\x91P_\x81g\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a7\x07Wa6\xE1c;\x9A\xCA\0\x85aQ\xF2V[a6\xEB\x90\x85aQ\xDFV[\x91Pa6\xF7\x82\x85aQ\xDFV[\x93P\x80a7\x03\x81aK\xB5V[\x91PP[\x80_\x03a7sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FstartValidators: not enough ETH `D\x82\x01Rs:7\x909\xBA0\xB9:\x100\x90;0\xB64\xB20\xBA7\xB9`a\x1B`d\x82\x01R`\x84\x01a*\xBEV[_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x8CWa7\x8Ca=:V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\xB5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_c;\x9A\xCA\0a7\xC8\x87GaQ\xDFV[a7\xD2\x91\x90aQ\xB5V[\x90Pa8\x14`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- creating new validators\0\0\0\0\0\0\0\x81RP\x83Qa;\x86V[a8?`@Q\x80``\x01`@R\x80`+\x81R` \x01aU=`+\x919\x82`\x01`\x01`@\x1B\x03\x16a;\x86V[_[\x85\x81\x10\x15a9UW`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05h\x01\xBC\x16\xD6t\xEC\x80\0\0a8na;\xD3V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8\x8A\x91\x90aG\nV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a8\xA6W=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xCB\x91\x90aR\x05V[\x90P\x80\x84\x83\x81Q\x81\x10a8\xE0Wa8\xE0aJ\x14V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x81\x01\x83U_\x92\x90\x92R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x95\x85\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x93\x02\x19\x90\x93\x16\x92\x90\x92\x17\x90U\x01a8AV[Pa9a\x85`\x01aQoV[\x83\x03a:qW`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05\x86a9\x83a;\xD3V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\x9F\x91\x90aG\nV[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a9\xBBW=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xE0\x91\x90aR\x05V[\x90P\x80\x83`\x01\x85Qa9\xF2\x91\x90aQ\xDFV[\x81Q\x81\x10a:\x02Wa:\x02aJ\x14V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x01\x82U_\x91\x90\x91R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x94\x84\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x92\x02\x19\x90\x92\x16\x91\x90\x91\x17\x90U[\x90\x97\x90\x96P\x94PPPPPV[`$T`@QcR\x85\x1D\r`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA5\n:\x1A\x90a:\xAE\x90\x85\x90`\x04\x01aF\xF8V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xC8W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra:\xEF\x91\x90\x81\x01\x90aR\x9EV[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc?e\xCF\x19`\xE0\x1B\x81R\x95\x96P`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x94c?e\xCF\x19\x94a\x074\x94\x93\x92\x89\x92`\x04\x01aT1V[``a\x0ES`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83a<\x18V[``a\x0ES`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83a<\x18V[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa2%\x92\x91\x90aT\xDEV[_Q` aU\x1D_9_Q\x90_R\x81`@Qa*`\x91\x90aG\nV[`&T`@\x80Q`\x01`\xF8\x1B` \x82\x01R_`!\x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16`,\x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01a<K\x93\x92\x91\x90aT\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1D\nW__\xFD[_` \x82\x84\x03\x12\x15a<\xDDW__\xFD[\x815a<\xE8\x81a<\xBBV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a=/W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a=\x08V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a=pWa=pa=:V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>&Wa>&a=:V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a>FWa>Fa=:V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1D\nW__\xFD[_\x82`\x1F\x83\x01\x12a>sW__\xFD[\x815a>\x86a>\x81\x82a>.V[a=\xFEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a>\xA7W__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x805a>\xBF\x81a>PV[\x83R` \x92\x83\x01\x92\x01a>\xACV[P\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a>\xE8W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xFDW__\xFD[a?\t\x85\x82\x86\x01a>dV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?$W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a?4W__\xFD[\x805a?Ba>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a?cW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a?\x85W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a?jV[\x80\x94PPPPP\x92P\x92\x90PV[_\x82`\x1F\x83\x01\x12a?\xA2W__\xFD[\x815a?\xB0a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a?\xD1W__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x805\x83R` \x92\x83\x01\x92\x01a?\xD6V[__`@\x83\x85\x03\x12\x15a?\xFFW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x14W__\xFD[a@ \x85\x82\x86\x01a>dV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@;W__\xFD[a@G\x85\x82\x86\x01a?\x93V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a@cV[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a@\xA6V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aA\x0F\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaA)`\xE0\x85\x01\x82a@QV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaAB\x82\x82a@\x94V[\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84RaA\x8D\x85\x83Qa@\xC4V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aAqV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aB\x1EW`\x1F\x19\x85\x84\x03\x01\x88RaB\x08\x83\x83QaA\xA2V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aA\xECV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aB\x8B\x90\x87\x01\x82aA\xD0V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aBPV[\x805a\x12P\x81a>PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12PW__\xFD[_`\xE0\x82\x84\x03\x12\x15aB\xCFW__\xFD[aB\xD7a=NV[\x90PaB\xE2\x82aB\xA1V[\x81RaB\xF0` \x83\x01aB\xA1V[` \x82\x01RaC\x01`@\x83\x01aB\xA1V[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaC\x1C`\x80\x83\x01aB\xACV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aC9W__\xFD[aCE\x84\x82\x85\x01a>dV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aCcW__\xFD[aCo\x84\x82\x85\x01a?\x93V[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aC\x8BW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xA0W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aC\xB0W__\xFD[\x805aC\xBEa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aC\xDFW__\xFD[` \x84\x01[\x83\x81\x10\x15aD\x1FW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x01W__\xFD[aD\x10\x89` \x83\x89\x01\x01aB\xBFV[\x84RP` \x92\x83\x01\x92\x01aC\xE4V[P\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84RaDl\x85\x83Qa@QV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aDPV[_` \x82\x84\x03\x12\x15aD\x91W__\xFD[\x815a<\xE8\x81a>PV[_` \x82\x84\x03\x12\x15aD\xACW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xC1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aD\xD1W__\xFD[\x805aD\xDFa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aE\0W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aE+W\x835aE\x1A\x81a<\xBBV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aE\x07V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aEEW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aEZW__\xFD[aEf\x84\x82\x85\x01aB\xBFV[\x94\x93PPPPV[` \x81R_a<\xE8` \x83\x01\x84a@QV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aE\x92V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87RaF\x06`@\x88\x01\x82aA\xA2V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01RaF!\x81\x83aE\x80V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aE\xE0V[` \x81R_a<\xE8` \x83\x01\x84aA\xD0V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aF\xAB\x90\x87\x01\x82aE\x80V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aFpV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a@\x8AW\x81Qd\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aF\xD3V[` \x81R_a<\xE8` \x83\x01\x84aF\xC1V[` \x81R_a<\xE8` \x83\x01\x84aA\xA2V[`@\x81R_aG.`@\x83\x01\x85aF\xC1V[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aGVW__\xFD[PQ\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x12PW__\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15aG\x8CWaG\x8Ca=:V[P`\x1F\x83\x01`\x1F\x19\x16` \x01aG\xA1\x81a=\xFEV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15aG\xB5W__\xFD[\x82\x82` \x83\x01^_` \x84\x83\x01\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aG\xDAW__\xFD[a<\xE8\x83\x83Q` \x85\x01aGsV[_`@\x82\x84\x03\x12\x15aG\xF9W__\xFD[aH\x01a=vV[\x82Q\x81R` \x83\x01Q\x90\x91P`\x01`\x01`@\x1B\x03\x81\x11\x15aH W__\xFD[aH,\x84\x82\x85\x01aG\xCBV[` \x83\x01RP\x92\x91PPV[_\x82`\x1F\x83\x01\x12aHGW__\xFD[\x81QaHUa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aHvW__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x80Q\x83R` \x92\x83\x01\x92\x01aH{V[_` \x82\x84\x03\x12\x15aH\xA3W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xB8W__\xFD[\x82\x01``\x81\x85\x03\x12\x15aH\xC9W__\xFD[aH\xD1a=\x98V[aH\xDA\x82aG]V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xF4W__\xFD[aI\0\x86\x82\x85\x01aG\xE9V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x1EW__\xFD[\x91\x90\x91\x01\x90`@\x82\x86\x03\x12\x15aI2W__\xFD[aI:a=vV[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aIOW__\xFD[aI[\x87\x82\x86\x01aH8V[\x82RP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aIvW__\xFD[aI\x82\x87\x82\x86\x01aG\xCBV[` \x83\x01RP`@\x82\x01R\x94\x93PPPPV[\x80Q\x82R_` \x82\x01Q`@` \x85\x01RaEf`@\x85\x01\x82aA\xA2V[`\x01`\x01`@\x1B\x03\x84\x16\x81R``` \x82\x01R_aI\xD4``\x83\x01\x85aI\x95V[\x82\x81\x03`@\x84\x01R\x83Q`@\x82RaI\xEF`@\x83\x01\x82a@\x94V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaJ\x08\x82\x82aA\xA2V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aJ8W__\xFD[\x81Qa<\xE8\x81a>PV[_` \x82\x84\x03\x12\x15aJSW__\xFD[\x81Q\x80\x15\x15\x81\x14a<\xE8W__\xFD[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a1\xCAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q``\x87RaJ\xAE``\x88\x01\x82a@QV[\x90P` \x82\x01Q\x87\x82\x03` \x89\x01RaJ\xC7\x82\x82a@\x94V[`@\x93\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x98\x90\x93\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aJ\x88V[_` \x82\x84\x03\x12\x15aK\x04W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x19W__\xFD[aEf\x84\x82\x85\x01aH8V[`\x01\x81\x81\x1C\x90\x82\x16\x80aK9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aKWWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aKmW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x82W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aK\x92W__\xFD[aEf\x84\x82Q` \x84\x01aGsV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aK\xC6WaK\xC6aK\xA1V[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R_\x83Q`@``\x84\x01RaK\xF6`\xA0\x84\x01\x82aA\xA2V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aL/\x82\x85aL\rV[`\x17`\xF9\x1B\x81RaAB`\x01\x82\x01\x85aL\rV[_`\xA0\x82\x84\x03\x12\x80\x15aLTW__\xFD[PaL]a=\xBAV[\x82Q\x81R` \x83\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14aLvW__\xFD[` \x82\x01RaL\x87`@\x84\x01aG]V[`@\x82\x01R``\x83\x01Q\x80`\x07\x0B\x81\x14aL\x9FW__\xFD[``\x82\x01RaL\xB0`\x80\x84\x01aG]V[`\x80\x82\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aL\xCCW__\xFD[a<\xE8\x82aG]V[_`@\x82\x01`@\x83R\x80\x85TaL\xEF\x81\x84\x90\x81R` \x01\x90V[_\x88\x81R` \x81 \x94P\x90\x92P[\x81`\x05\x82\x01\x10\x15aMdW\x83Td\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x85R`(\x82\x90\x1C\x81\x16` \x86\x01R`P\x82\x90\x1C\x81\x16`@\x86\x01R`x\x82\x90\x1C\x81\x16``\x86\x01R`\xA0\x82\x81\x1C\x82\x16`\x80\x87\x01R`\xC8\x92\x90\x92\x1C\x16\x90\x84\x01R`\x01\x90\x93\x01\x92`\xC0\x90\x92\x01\x91`\x06\x01aL\xFDV[\x92T\x92\x81\x81\x10\x15aM\x83Wd\xFF\xFF\xFF\xFF\xFF\x84\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aM\xA3Wd\xFF\xFF\xFF\xFF\xFF`(\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aM\xC3Wd\xFF\xFF\xFF\xFF\xFF`P\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aM\xE3Wd\xFF\xFF\xFF\xFF\xFF`x\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aN\x03Wd\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aN W`\xC8\x84\x90\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x83\x01\x92P[PP`\x01`\x01`@\x1B\x03\x85\x16` \x85\x01R\x91Pa<\xE8\x90PV[_` \x82\x84\x03\x12\x15aNJW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN_W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15aNpW__\xFD[aNxa=vV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x8DW__\xFD[aN\x99\x86\x82\x85\x01aG\xE9V[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xB4W__\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12aN\xC8W__\xFD[\x81QaN\xD6a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x87\x83\x11\x15aN\xF7W__\xFD[` \x85\x01[\x83\x81\x10\x15aO\x83W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x19W__\xFD[\x86\x01``\x81\x8B\x03`\x1F\x19\x01\x12\x15aO.W__\xFD[aO6a=\x98V[` \x82\x81\x01Q\x82R`@\x83\x01Q\x90\x82\x01R``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO_W__\xFD[aOn\x8C` \x83\x86\x01\x01aG\xCBV[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01aN\xFCV[P` \x84\x01RP\x90\x95\x94PPPPPV[`@\x81R_aO\xA6`@\x83\x01\x85aI\x95V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01_[\x83\x81\x10\x15aP\x1CW`\x1F\x19\x86\x84\x03\x01\x85R\x81Q\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q\x90P```@\x85\x01RaP\x05``\x85\x01\x82aA\xA2V[` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01aO\xC9V[P\x90\x98\x97PPPPPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R_aAB``\x83\x01\x84aA\xA2V[``\x81R_aPZ``\x83\x01\x86a@\xC4V[\x82\x81\x03` \x84\x01RaPl\x81\x86a@QV[\x91PP\x82\x15\x15`@\x83\x01R\x94\x93PPPPV[_aP\x8A\x82\x86aL\rV[`\x17`\xF9\x1B\x81RaP\x9E`\x01\x82\x01\x86aL\rV[\x90P`\x1D`\xF9\x1B\x81RaE+`\x01\x82\x01\x85aL\rV[__`@\x83\x85\x03\x12\x15aP\xC5W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xDAW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aP\xEAW__\xFD[\x80QaP\xF8a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aQ\x19W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aQDW\x83QaQ3\x81a>PV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aQ V[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQcW__\xFD[a@G\x85\x82\x86\x01aH8V[\x80\x82\x01\x80\x82\x11\x15a\x0ESWa\x0ESaK\xA1V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0ESWa\x0ESaK\xA1V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aQ\xC3WaQ\xC3aQ\xA1V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0ESWa\x0ESaK\xA1V[\x81\x81\x03\x81\x81\x11\x15a\x0ESWa\x0ESaK\xA1V[_\x82aR\0WaR\0aQ\xA1V[P\x06\x90V[_` \x82\x84\x03\x12\x15aR\x15W__\xFD[\x81Qa<\xE8\x81a<\xBBV[_\x82`\x1F\x83\x01\x12aR/W__\xFD[\x81QaR=a>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aR^W__\xFD[` \x85\x01[\x83\x81\x10\x15a>\xCDW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x80W__\xFD[aR\x8F\x88` \x83\x8A\x01\x01aH8V[\x84RP` \x92\x83\x01\x92\x01aRcV[_` \x82\x84\x03\x12\x15aR\xAEW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xC3W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15aR\xD4W__\xFD[aR\xDCa=\xDCV[aR\xE5\x82aG]V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xFFW__\xFD[aS\x0B\x86\x82\x85\x01aG\xE9V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS)W__\xFD[\x82\x01`\x1F\x81\x01\x86\x13aS9W__\xFD[\x80QaSGa>\x81\x82a>.V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aShW__\xFD[` \x84\x01[\x83\x81\x10\x15aS\xA8W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x8AW__\xFD[aS\x99\x8B` \x83\x89\x01\x01aG\xCBV[\x84RP` \x92\x83\x01\x92\x01aSmV[P`@\x85\x01RPPP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xC9W__\xFD[aS\xD5\x86\x82\x85\x01aR V[``\x83\x01RP\x94\x93PPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aB\x1EW`\x1F\x19\x85\x84\x03\x01\x88RaT\x1B\x83\x83Qa@\x94V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aS\xFFV[`\x01`\x01`@\x1B\x03\x86\x16\x81R`\xA0` \x82\x01R_aTR`\xA0\x83\x01\x87aI\x95V[\x82\x81\x03`@\x84\x01RaTd\x81\x87aF\xC1V[\x90P\x82\x81\x03``\x84\x01R\x80\x85Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x88\x01_[\x83\x81\x10\x15aT\xBBW`\x1F\x19\x86\x84\x03\x01\x85RaT\xA5\x83\x83QaA\xA2V[` \x95\x86\x01\x95\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aT\x89V[PP\x85\x81\x03`\x80\x87\x01RaT\xCF\x81\x88aS\xE3V[\x9B\x9APPPPPPPPPPPV[`@\x81R_aT\xF0`@\x83\x01\x85aA\xA2V[\x90P\x82` \x83\x01R\x93\x92PPPV[_aABaU\x16aU\x10\x84\x88aL\rV[\x86aL\rV[\x84aL\rV\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP- depositing balance to beacon chain (gwei)- exiting all validators and completing checkpoint- submitting num checkpoint proofsUser.queueWithdrawals: length mismatch\xA2dipfsX\"\x12 \x04R(=\xE0<{\xF2\xDC@\x9E\xB6\x8B\xBB\x8B\x95D\xF4Gh\x852t*\x90\xFBc\xBA\xC9\xE9q\x04dsolcC\0\x08\x1B\x003",
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
    /**Constructor`.
```solidity
constructor(string name);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub name: alloy::sol_types::private::String,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.name,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { name: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                )
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
    /**Function with signature `NAME()` and selector `0xa3f4df7e`.
```solidity
function NAME() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NAMECall {}
    ///Container type for the return parameters of the [`NAME()`](NAMECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NAMEReturn {
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
            impl ::core::convert::From<NAMECall> for UnderlyingRustTuple<'_> {
                fn from(value: NAMECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NAMECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<NAMEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: NAMEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NAMEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for NAMECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = NAMEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NAME()";
            const SELECTOR: [u8; 4] = [163u8, 244u8, 223u8, 126u8];
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
    /**Function with signature `completeCheckpoint()` and selector `0xd6c10daf`.
```solidity
function completeCheckpoint() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeCheckpointCall {}
    ///Container type for the return parameters of the [`completeCheckpoint()`](completeCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeCheckpointReturn {}
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
            impl ::core::convert::From<completeCheckpointCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeCheckpointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeCheckpointCall {
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
            impl ::core::convert::From<completeCheckpointReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeCheckpoint()";
            const SELECTOR: [u8; 4] = [214u8, 193u8, 13u8, 175u8];
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
    /**Function with signature `completeWithdrawalAsShares((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x695f4ae1`.
```solidity
function completeWithdrawalAsShares(IDelegationManagerTypes.Withdrawal memory withdrawal) external returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalAsSharesCall {
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`completeWithdrawalAsShares((address,address,address,uint256,uint32,address[],uint256[]))`](completeWithdrawalAsSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalAsSharesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<completeWithdrawalAsSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsSharesCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalAsSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawal: tuple.0 }
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
            impl ::core::convert::From<completeWithdrawalAsSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalAsSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalAsSharesCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalAsSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeWithdrawalAsShares((address,address,address,uint256,uint32,address[],uint256[]))";
            const SELECTOR: [u8; 4] = [105u8, 95u8, 74u8, 225u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
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
    /**Function with signature `completeWithdrawalAsTokens((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x401be65e`.
```solidity
function completeWithdrawalAsTokens(IDelegationManagerTypes.Withdrawal memory withdrawal) external returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalAsTokensCall {
        pub withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`completeWithdrawalAsTokens((address,address,address,uint256,uint32,address[],uint256[]))`](completeWithdrawalAsTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalAsTokensReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (IDelegationManagerTypes::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<completeWithdrawalAsTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsTokensCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalAsTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawal: tuple.0 }
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
            impl ::core::convert::From<completeWithdrawalAsTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalAsTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalAsTokensCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalAsTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeWithdrawalAsTokens((address,address,address,uint256,uint32,address[],uint256[]))";
            const SELECTOR: [u8; 4] = [64u8, 27u8, 230u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManagerTypes::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
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
    /**Function with signature `completeWithdrawalsAsShares((address,address,address,uint256,uint32,address[],uint256[])[])` and selector `0x46a5be0d`.
```solidity
function completeWithdrawalsAsShares(IDelegationManagerTypes.Withdrawal[] memory withdrawals) external returns (address[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalsAsSharesCall {
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`completeWithdrawalsAsShares((address,address,address,uint256,uint32,address[],uint256[])[])`](completeWithdrawalsAsSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalsAsSharesReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<completeWithdrawalsAsSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsSharesCall) -> Self {
                    (value.withdrawals,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalsAsSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawals: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<completeWithdrawalsAsSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalsAsSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalsAsSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalsAsSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeWithdrawalsAsShares((address,address,address,uint256,uint32,address[],uint256[])[])";
            const SELECTOR: [u8; 4] = [70u8, 165u8, 190u8, 13u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IDelegationManagerTypes::Withdrawal,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawals),
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
    /**Function with signature `completeWithdrawalsAsTokens((address,address,address,uint256,uint32,address[],uint256[])[])` and selector `0x344e1383`.
```solidity
function completeWithdrawalsAsTokens(IDelegationManagerTypes.Withdrawal[] memory withdrawals) external returns (address[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalsAsTokensCall {
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`completeWithdrawalsAsTokens((address,address,address,uint256,uint32,address[],uint256[])[])`](completeWithdrawalsAsTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeWithdrawalsAsTokensReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<completeWithdrawalsAsTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsTokensCall) -> Self {
                    (value.withdrawals,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalsAsTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawals: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<completeWithdrawalsAsTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for completeWithdrawalsAsTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalsAsTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalsAsTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeWithdrawalsAsTokens((address,address,address,uint256,uint32,address[],uint256[])[])";
            const SELECTOR: [u8; 4] = [52u8, 78u8, 19u8, 131u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IDelegationManagerTypes::Withdrawal,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawals),
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
    /**Function with signature `delegateTo(address)` and selector `0xac637c7a`.
```solidity
function delegateTo(address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegateTo(address)`](delegateToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToReturn {}
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
            impl ::core::convert::From<delegateToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<delegateToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateToCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateTo(address)";
            const SELECTOR: [u8; 4] = [172u8, 99u8, 124u8, 122u8];
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
                        &self.operator,
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
    /**Function with signature `depositIntoEigenlayer(address[],uint256[])` and selector `0x6d336f58`.
```solidity
function depositIntoEigenlayer(address[] memory strategies, uint256[] memory tokenBalances) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositIntoEigenlayerCall {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub tokenBalances: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`depositIntoEigenlayer(address[],uint256[])`](depositIntoEigenlayerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositIntoEigenlayerReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<depositIntoEigenlayerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoEigenlayerCall) -> Self {
                    (value.strategies, value.tokenBalances)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositIntoEigenlayerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategies: tuple.0,
                        tokenBalances: tuple.1,
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
            impl ::core::convert::From<depositIntoEigenlayerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoEigenlayerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositIntoEigenlayerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositIntoEigenlayerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositIntoEigenlayerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositIntoEigenlayer(address[],uint256[])";
            const SELECTOR: [u8; 4] = [109u8, 51u8, 111u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenBalances),
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
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall {}
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        pub excludedSelectors_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
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
    /**Function with signature `exitValidators(uint40[])` and selector `0x3d8c08d4`.
```solidity
function exitValidators(uint40[] memory _validators) external returns (uint64 exitedBalanceGwei);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitValidatorsCall {
        pub _validators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
    }
    ///Container type for the return parameters of the [`exitValidators(uint40[])`](exitValidatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitValidatorsReturn {
        pub exitedBalanceGwei: u64,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<exitValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _validators: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<exitValidatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorsReturn) -> Self {
                    (value.exitedBalanceGwei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for exitValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { exitedBalanceGwei: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exitValidatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitValidatorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exitValidators(uint40[])";
            const SELECTOR: [u8; 4] = [61u8, 140u8, 8u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self._validators),
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
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
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
    /**Function with signature `forceUndelegate(address)` and selector `0x391cc9f6`.
```solidity
function forceUndelegate(address staker) external returns (IDelegationManagerTypes.Withdrawal[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceUndelegateCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`forceUndelegate(address)`](forceUndelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceUndelegateReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<forceUndelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: forceUndelegateCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for forceUndelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<forceUndelegateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: forceUndelegateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceUndelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for forceUndelegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = forceUndelegateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "forceUndelegate(address)";
            const SELECTOR: [u8; 4] = [57u8, 28u8, 201u8, 246u8];
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
                        &self.staker,
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
    /**Function with signature `getActiveValidators()` and selector `0x9de70258`.
```solidity
function getActiveValidators() external view returns (uint40[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveValidatorsCall {}
    ///Container type for the return parameters of the [`getActiveValidators()`](getActiveValidatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveValidatorsReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<getActiveValidatorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveValidatorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<getActiveValidatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveValidatorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveValidatorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getActiveValidatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveValidators()";
            const SELECTOR: [u8; 4] = [157u8, 231u8, 2u8, 88u8];
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
    /**Function with signature `pod()` and selector `0xa88dbb36`.
```solidity
function pod() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct podCall {}
    ///Container type for the return parameters of the [`pod()`](podCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct podReturn {
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
            impl ::core::convert::From<podCall> for UnderlyingRustTuple<'_> {
                fn from(value: podCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for podCall {
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
            impl ::core::convert::From<podReturn> for UnderlyingRustTuple<'_> {
                fn from(value: podReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for podReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for podCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = podReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pod()";
            const SELECTOR: [u8; 4] = [168u8, 141u8, 187u8, 54u8];
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
    /**Function with signature `queueWithdrawals(address[],uint256[])` and selector `0x23e48175`.
```solidity
function queueWithdrawals(address[] memory strategies, uint256[] memory depositShares) external returns (IDelegationManagerTypes.Withdrawal[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsCall {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub depositShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`queueWithdrawals(address[],uint256[])`](queueWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<queueWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsCall) -> Self {
                    (value.strategies, value.depositShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategies: tuple.0,
                        depositShares: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queueWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueWithdrawals(address[],uint256[])";
            const SELECTOR: [u8; 4] = [35u8, 228u8, 129u8, 117u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.depositShares),
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
    /**Function with signature `registerAsOperator()` and selector `0x2a34ade8`.
```solidity
function registerAsOperator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorCall {}
    ///Container type for the return parameters of the [`registerAsOperator()`](registerAsOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorReturn {}
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
            impl ::core::convert::From<registerAsOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAsOperatorCall {
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
            impl ::core::convert::From<registerAsOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerAsOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAsOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAsOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerAsOperator()";
            const SELECTOR: [u8; 4] = [42u8, 52u8, 173u8, 232u8];
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
    /**Function with signature `startCheckpoint()` and selector `0x90b51625`.
```solidity
function startCheckpoint() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startCheckpointCall {}
    ///Container type for the return parameters of the [`startCheckpoint()`](startCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startCheckpointReturn {}
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
            impl ::core::convert::From<startCheckpointCall> for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startCheckpointCall {
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
            impl ::core::convert::From<startCheckpointReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startCheckpoint()";
            const SELECTOR: [u8; 4] = [144u8, 181u8, 22u8, 37u8];
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
    /**Function with signature `startValidators()` and selector `0xf234c1bd`.
```solidity
function startValidators() external returns (uint40[] memory, uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startValidatorsCall {}
    ///Container type for the return parameters of the [`startValidators()`](startValidatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startValidatorsReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
        pub _1: u64,
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
            impl ::core::convert::From<startValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: startValidatorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
                >,
                u64,
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
            impl ::core::convert::From<startValidatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startValidatorsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startValidatorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startValidatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startValidators()";
            const SELECTOR: [u8; 4] = [242u8, 52u8, 193u8, 189u8];
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
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
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
    /**Function with signature `undelegate()` and selector `0x92ab89bb`.
```solidity
function undelegate() external returns (IDelegationManagerTypes.Withdrawal[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateCall {}
    ///Container type for the return parameters of the [`undelegate()`](undelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<undelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<undelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for undelegateCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = undelegateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "undelegate()";
            const SELECTOR: [u8; 4] = [146u8, 171u8, 137u8, 187u8];
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
    /**Function with signature `updateBalances(address[],int256[])` and selector `0x20a545d9`.
```solidity
function updateBalances(address[] memory strategies, int256[] memory tokenDeltas) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateBalancesCall {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub tokenDeltas: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
    }
    ///Container type for the return parameters of the [`updateBalances(address[],int256[])`](updateBalancesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateBalancesReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<updateBalancesCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateBalancesCall) -> Self {
                    (value.strategies, value.tokenDeltas)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateBalancesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategies: tuple.0,
                        tokenDeltas: tuple.1,
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
            impl ::core::convert::From<updateBalancesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateBalancesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateBalancesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateBalancesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateBalancesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateBalances(address[],int256[])";
            const SELECTOR: [u8; 4] = [32u8, 165u8, 69u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenDeltas),
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
    /**Function with signature `verifyStaleBalance(uint40)` and selector `0x071c25b7`.
```solidity
function verifyStaleBalance(uint40 validatorIndex) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStaleBalanceCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`verifyStaleBalance(uint40)`](verifyStaleBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStaleBalanceReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<verifyStaleBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyStaleBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<verifyStaleBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyStaleBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyStaleBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyStaleBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyStaleBalance(uint40)";
            const SELECTOR: [u8; 4] = [7u8, 28u8, 37u8, 183u8];
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
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
    /**Function with signature `verifyWithdrawalCredentials(uint40[])` and selector `0x841c1299`.
```solidity
function verifyWithdrawalCredentials(uint40[] memory _validators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsCall {
        pub _validators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
    }
    ///Container type for the return parameters of the [`verifyWithdrawalCredentials(uint40[])`](verifyWithdrawalCredentialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<verifyWithdrawalCredentialsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _validators: tuple.0 }
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
            impl ::core::convert::From<verifyWithdrawalCredentialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyWithdrawalCredentialsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyWithdrawalCredentialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyWithdrawalCredentials(uint40[])";
            const SELECTOR: [u8; 4] = [132u8, 28u8, 18u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self._validators),
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
    ///Container for all the [`User`](self) function calls.
    pub enum UserCalls {
        IS_TEST(IS_TESTCall),
        NAME(NAMECall),
        completeCheckpoint(completeCheckpointCall),
        completeWithdrawalAsShares(completeWithdrawalAsSharesCall),
        completeWithdrawalAsTokens(completeWithdrawalAsTokensCall),
        completeWithdrawalsAsShares(completeWithdrawalsAsSharesCall),
        completeWithdrawalsAsTokens(completeWithdrawalsAsTokensCall),
        delegateTo(delegateToCall),
        depositIntoEigenlayer(depositIntoEigenlayerCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSelectors(excludeSelectorsCall),
        excludeSenders(excludeSendersCall),
        exitValidators(exitValidatorsCall),
        failed(failedCall),
        forceUndelegate(forceUndelegateCall),
        getActiveValidators(getActiveValidatorsCall),
        pod(podCall),
        queueWithdrawals(queueWithdrawalsCall),
        registerAsOperator(registerAsOperatorCall),
        startCheckpoint(startCheckpointCall),
        startValidators(startValidatorsCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        undelegate(undelegateCall),
        updateBalances(updateBalancesCall),
        verifyStaleBalance(verifyStaleBalanceCall),
        verifyWithdrawalCredentials(verifyWithdrawalCredentialsCall),
    }
    #[automatically_derived]
    impl UserCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 28u8, 37u8, 183u8],
            [30u8, 215u8, 131u8, 28u8],
            [32u8, 165u8, 69u8, 217u8],
            [35u8, 228u8, 129u8, 117u8],
            [42u8, 52u8, 173u8, 232u8],
            [42u8, 222u8, 56u8, 128u8],
            [52u8, 78u8, 19u8, 131u8],
            [57u8, 28u8, 201u8, 246u8],
            [61u8, 140u8, 8u8, 212u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [64u8, 27u8, 230u8, 94u8],
            [70u8, 165u8, 190u8, 13u8],
            [102u8, 217u8, 169u8, 160u8],
            [105u8, 95u8, 74u8, 225u8],
            [109u8, 51u8, 111u8, 88u8],
            [132u8, 28u8, 18u8, 153u8],
            [133u8, 34u8, 108u8, 129u8],
            [144u8, 181u8, 22u8, 37u8],
            [145u8, 106u8, 23u8, 198u8],
            [146u8, 171u8, 137u8, 187u8],
            [157u8, 231u8, 2u8, 88u8],
            [163u8, 244u8, 223u8, 126u8],
            [168u8, 141u8, 187u8, 54u8],
            [172u8, 99u8, 124u8, 122u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [214u8, 193u8, 13u8, 175u8],
            [226u8, 12u8, 159u8, 113u8],
            [242u8, 52u8, 193u8, 189u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for UserCalls {
        const NAME: &'static str = "UserCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::NAME(_) => <NAMECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::completeCheckpoint(_) => {
                    <completeCheckpointCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeWithdrawalAsShares(_) => {
                    <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeWithdrawalAsTokens(_) => {
                    <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeWithdrawalsAsShares(_) => {
                    <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeWithdrawalsAsTokens(_) => {
                    <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositIntoEigenlayer(_) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exitValidators(_) => {
                    <exitValidatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::forceUndelegate(_) => {
                    <forceUndelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveValidators(_) => {
                    <getActiveValidatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pod(_) => <podCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::queueWithdrawals(_) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startCheckpoint(_) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::startValidators(_) => {
                    <startValidatorsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::undelegate(_) => {
                    <undelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateBalances(_) => {
                    <updateBalancesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyStaleBalance(_) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyWithdrawalCredentials(_) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<UserCalls>] = &[
                {
                    fn verifyStaleBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::verifyStaleBalance)
                    }
                    verifyStaleBalance
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn updateBalances(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <updateBalancesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::updateBalances)
                    }
                    updateBalances
                },
                {
                    fn queueWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::queueWithdrawals)
                    }
                    queueWithdrawals
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::registerAsOperator)
                    }
                    registerAsOperator
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn completeWithdrawalsAsTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::completeWithdrawalsAsTokens)
                    }
                    completeWithdrawalsAsTokens
                },
                {
                    fn forceUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <forceUndelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::forceUndelegate)
                    }
                    forceUndelegate
                },
                {
                    fn exitValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <exitValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::exitValidators)
                    }
                    exitValidators
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn completeWithdrawalAsTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::completeWithdrawalAsTokens)
                    }
                    completeWithdrawalAsTokens
                },
                {
                    fn completeWithdrawalsAsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::completeWithdrawalsAsShares)
                    }
                    completeWithdrawalsAsShares
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn completeWithdrawalAsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::completeWithdrawalAsShares)
                    }
                    completeWithdrawalAsShares
                },
                {
                    fn depositIntoEigenlayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::depositIntoEigenlayer)
                    }
                    depositIntoEigenlayer
                },
                {
                    fn verifyWithdrawalCredentials(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::verifyWithdrawalCredentials)
                    }
                    verifyWithdrawalCredentials
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn startCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <startCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::startCheckpoint)
                    }
                    startCheckpoint
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::undelegate)
                    }
                    undelegate
                },
                {
                    fn getActiveValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::getActiveValidators)
                    }
                    getActiveValidators
                },
                {
                    fn NAME(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::NAME)
                    }
                    NAME
                },
                {
                    fn pod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <podCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::pod)
                    }
                    pod
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::failed)
                    }
                    failed
                },
                {
                    fn completeCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <completeCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::completeCheckpoint)
                    }
                    completeCheckpoint
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn startValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <startValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::startValidators)
                    }
                    startValidators
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<UserCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(UserCalls::IS_TEST)
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
                Self::NAME(inner) => {
                    <NAMECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::completeCheckpoint(inner) => {
                    <completeCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeWithdrawalAsShares(inner) => {
                    <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeWithdrawalAsTokens(inner) => {
                    <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeWithdrawalsAsShares(inner) => {
                    <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeWithdrawalsAsTokens(inner) => {
                    <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::depositIntoEigenlayer(inner) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::exitValidators(inner) => {
                    <exitValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::forceUndelegate(inner) => {
                    <forceUndelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveValidators(inner) => {
                    <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pod(inner) => {
                    <podCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::startValidators(inner) => {
                    <startValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateBalances(inner) => {
                    <updateBalancesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::NAME(inner) => {
                    <NAMECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::completeCheckpoint(inner) => {
                    <completeCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeWithdrawalAsShares(inner) => {
                    <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeWithdrawalAsTokens(inner) => {
                    <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeWithdrawalsAsShares(inner) => {
                    <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeWithdrawalsAsTokens(inner) => {
                    <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::depositIntoEigenlayer(inner) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::exitValidators(inner) => {
                    <exitValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::forceUndelegate(inner) => {
                    <forceUndelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveValidators(inner) => {
                    <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pod(inner) => {
                    <podCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::startValidators(inner) => {
                    <startValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateBalances(inner) => {
                    <updateBalancesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`User`](self) events.
    pub enum UserEvents {
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
    impl UserEvents {
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
    impl alloy_sol_types::SolEventInterface for UserEvents {
        const NAME: &'static str = "UserEvents";
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
    impl alloy_sol_types::private::IntoLogData for UserEvents {
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
    /**Creates a new wrapper around an on-chain [`User`](self) contract instance.

See the [wrapper's documentation](`UserInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, provider: P) -> UserInstance<T, P, N> {
        UserInstance::<T, P, N>::new(address, provider)
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
        name: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<UserInstance<T, P, N>>,
    > {
        UserInstance::<T, P, N>::deploy(provider, name)
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
        name: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        UserInstance::<T, P, N>::deploy_builder(provider, name)
    }
    /**A [`User`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`User`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct UserInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for UserInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("UserInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > UserInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`User`](self) contract instance.

See the [wrapper's documentation](`UserInstance`) for more details.*/
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
            name: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<UserInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, name);
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
            name: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { name },
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
    impl<T, P: ::core::clone::Clone, N> UserInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> UserInstance<T, P, N> {
            UserInstance {
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
    > UserInstance<T, P, N> {
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
        ///Creates a new call builder for the [`NAME`] function.
        pub fn NAME(&self) -> alloy_contract::SolCallBuilder<T, &P, NAMECall, N> {
            self.call_builder(&NAMECall {})
        }
        ///Creates a new call builder for the [`completeCheckpoint`] function.
        pub fn completeCheckpoint(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeCheckpointCall, N> {
            self.call_builder(&completeCheckpointCall {})
        }
        ///Creates a new call builder for the [`completeWithdrawalAsShares`] function.
        pub fn completeWithdrawalAsShares(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalAsSharesCall, N> {
            self.call_builder(
                &completeWithdrawalAsSharesCall {
                    withdrawal,
                },
            )
        }
        ///Creates a new call builder for the [`completeWithdrawalAsTokens`] function.
        pub fn completeWithdrawalAsTokens(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalAsTokensCall, N> {
            self.call_builder(
                &completeWithdrawalAsTokensCall {
                    withdrawal,
                },
            )
        }
        ///Creates a new call builder for the [`completeWithdrawalsAsShares`] function.
        pub fn completeWithdrawalsAsShares(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalsAsSharesCall, N> {
            self.call_builder(
                &completeWithdrawalsAsSharesCall {
                    withdrawals,
                },
            )
        }
        ///Creates a new call builder for the [`completeWithdrawalsAsTokens`] function.
        pub fn completeWithdrawalsAsTokens(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalsAsTokensCall, N> {
            self.call_builder(
                &completeWithdrawalsAsTokensCall {
                    withdrawals,
                },
            )
        }
        ///Creates a new call builder for the [`delegateTo`] function.
        pub fn delegateTo(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToCall, N> {
            self.call_builder(&delegateToCall { operator })
        }
        ///Creates a new call builder for the [`depositIntoEigenlayer`] function.
        pub fn depositIntoEigenlayer(
            &self,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            tokenBalances: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositIntoEigenlayerCall, N> {
            self.call_builder(
                &depositIntoEigenlayerCall {
                    strategies,
                    tokenBalances,
                },
            )
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
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`exitValidators`] function.
        pub fn exitValidators(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, exitValidatorsCall, N> {
            self.call_builder(&exitValidatorsCall { _validators })
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`forceUndelegate`] function.
        pub fn forceUndelegate(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, forceUndelegateCall, N> {
            self.call_builder(&forceUndelegateCall { staker })
        }
        ///Creates a new call builder for the [`getActiveValidators`] function.
        pub fn getActiveValidators(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getActiveValidatorsCall, N> {
            self.call_builder(&getActiveValidatorsCall {})
        }
        ///Creates a new call builder for the [`pod`] function.
        pub fn pod(&self) -> alloy_contract::SolCallBuilder<T, &P, podCall, N> {
            self.call_builder(&podCall {})
        }
        ///Creates a new call builder for the [`queueWithdrawals`] function.
        pub fn queueWithdrawals(
            &self,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            depositShares: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalsCall, N> {
            self.call_builder(
                &queueWithdrawalsCall {
                    strategies,
                    depositShares,
                },
            )
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(&registerAsOperatorCall {})
        }
        ///Creates a new call builder for the [`startCheckpoint`] function.
        pub fn startCheckpoint(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, startCheckpointCall, N> {
            self.call_builder(&startCheckpointCall {})
        }
        ///Creates a new call builder for the [`startValidators`] function.
        pub fn startValidators(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, startValidatorsCall, N> {
            self.call_builder(&startValidatorsCall {})
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
        ///Creates a new call builder for the [`undelegate`] function.
        pub fn undelegate(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, undelegateCall, N> {
            self.call_builder(&undelegateCall {})
        }
        ///Creates a new call builder for the [`updateBalances`] function.
        pub fn updateBalances(
            &self,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            tokenDeltas: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::I256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateBalancesCall, N> {
            self.call_builder(
                &updateBalancesCall {
                    strategies,
                    tokenDeltas,
                },
            )
        }
        ///Creates a new call builder for the [`verifyStaleBalance`] function.
        pub fn verifyStaleBalance(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyStaleBalanceCall, N> {
            self.call_builder(
                &verifyStaleBalanceCall {
                    validatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials`] function.
        pub fn verifyWithdrawalCredentials(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyWithdrawalCredentialsCall, N> {
            self.call_builder(
                &verifyWithdrawalCredentialsCall {
                    _validators,
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
    > UserInstance<T, P, N> {
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