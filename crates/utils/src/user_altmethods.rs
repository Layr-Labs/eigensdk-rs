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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub scaledShares:
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
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
        impl alloy_sol_types::SolType for Withdrawal {
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
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] scaledShares)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IDelegationManagerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IDelegationManagerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IDelegationManagerTypes`](self) contract instance.

        See the [wrapper's documentation](`IDelegationManagerTypesInstance`) for more details.*/
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
        > IDelegationManagerTypesInstance<T, P, N>
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
        > IDelegationManagerTypesInstance<T, P, N>
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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

interface User_AltMethods {
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
    function signedHashes(bytes32) external view returns (bool);
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
    "name": "signedHashes",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
pub mod User_AltMethods {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052600c8054600160ff199091168117909155601f80546001600160a81b031916747109709ecfa91a80626ff3989d68f67f5b1dd12d011790556028805463ffffffff19169091179055348015610057575f5ffd5b50604051615efd380380615efd833981016040819052610076916103cc565b805f339050806001600160a01b031663ea4d3c9b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906100db9190610493565b602080546001600160a01b0319166001600160a01b0392831617815560408051630736e1c760e31b81529051928416926339b70e38926004808401939192918290030181865afa158015610131573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101559190610493565b60215f6101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316634665bcda6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101b6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101da9190610493565b60225f6101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316633dfb40e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561023b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061025f9190610493565b60235f6101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b03166322c0350b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102c0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102e49190610493565b602480546001600160a01b0319166001600160a01b039290921691909117905561030c610321565b60256103188382610539565b505050506105f3565b60225f9054906101000a90046001600160a01b03166001600160a01b03166384d810626040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610372573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103969190610493565b602680546001600160a01b0319166001600160a01b0392909216919091179055565b634e487b7160e01b5f52604160045260245ffd5b5f602082840312156103dc575f5ffd5b81516001600160401b038111156103f1575f5ffd5b8201601f81018413610401575f5ffd5b80516001600160401b0381111561041a5761041a6103b8565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610448576104486103b8565b60405281815282820160200186101561045f575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b6001600160a01b0381168114610490575f5ffd5b50565b5f602082840312156104a3575f5ffd5b81516104ae8161047c565b9392505050565b600181811c908216806104c957607f821691505b6020821081036104e757634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561053457805f5260205f20601f840160051c810160208510156105125750805b601f840160051c820191505b81811015610531575f815560010161051e565b50505b505050565b81516001600160401b03811115610552576105526103b8565b6105668161056084546104b5565b846104ed565b6020601f821160018114610598575f83156105815750848201515b5f19600385901b1c1916600184901b178455610531565b5f84815260208120601f198516915b828110156105c757878501518255602094850194600190920191016105a7565b50848210156105e457868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b6158fd806106005f395ff3fe6080604052600436106101e9575f3560e01c8063841c129911610108578063ac637c7a1161009d578063ba414fa61161006d578063ba414fa61461059a578063d6c10daf146105ae578063e20c9f71146105c2578063f234c1bd146105d6578063fa7626d4146105f8575f5ffd5b8063ac637c7a14610515578063b0464fdc14610534578063b5508aa914610548578063b71723551461055c575f5ffd5b806392ab89bb116100d857806392ab89bb146104885780639de702581461049c578063a3f4df7e146104bd578063a88dbb36146104de575f5ffd5b8063841c12991461041357806385226c811461043257806390b5162514610453578063916a17c614610467575f5ffd5b80633d8c08d41161017e57806346a5be0d1161014e57806346a5be0d1461039557806366d9a9a0146103b4578063695f4ae1146103d55780636d336f58146103f4575f5ffd5b80633d8c08d41461030a5780633e5e3c23146103415780633f7286f414610355578063401be65e14610369575f5ffd5b80632a34ade8116101b95780632a34ade81461028a5780632ade38801461029e578063344e1383146102bf578063391cc9f6146102eb575f5ffd5b8063071c25b7146101f45780631ed7831c1461021557806320a545d91461023f57806323e481751461025e575f5ffd5b366101f057005b5f5ffd5b3480156101ff575f5ffd5b5061021361020e366004613f5b565b610611565b005b348015610220575f5ffd5b506102296107d7565b6040516102369190613f7d565b60405180910390f35b34801561024a575f5ffd5b50610213610259366004614165565b610837565b348015610269575f5ffd5b5061027d61027836600461427c565b610b42565b60405161023691906143d9565b348015610295575f5ffd5b50610213610e92565b3480156102a9575f5ffd5b506102b2610ff9565b60405161023691906144b8565b3480156102ca575f5ffd5b506102de6102d9366004614609565b611135565b60405161023691906146b8565b3480156102f6575f5ffd5b5061027d61030536600461470f565b61128e565b348015610315575f5ffd5b5061032961032436600461472a565b611409565b6040516001600160401b039091168152602001610236565b34801561034c575f5ffd5b506102296114ae565b348015610360575f5ffd5b5061022961150c565b348015610374575f5ffd5b506103886103833660046147c3565b61156a565b60405161023691906147fc565b3480156103a0575f5ffd5b506102de6103af366004614609565b611621565b3480156103bf575f5ffd5b506103c8611770565b6040516102369190614848565b3480156103e0575f5ffd5b506103886103ef3660046147c3565b6118d4565b3480156103ff575f5ffd5b5061021361040e36600461427c565b61198a565b34801561041e575f5ffd5b5061021361042d36600461472a565b611edb565b34801561043d575f5ffd5b50610446611f9b565b60405161023691906148c6565b34801561045e575f5ffd5b50610213612066565b348015610472575f5ffd5b5061047b612116565b60405161023691906148d8565b348015610493575f5ffd5b5061027d6121f7565b3480156104a7575f5ffd5b506104b0612512565b6040516102369190614986565b3480156104c8575f5ffd5b506104d16126b4565b6040516102369190614998565b3480156104e9575f5ffd5b506026546104fd906001600160a01b031681565b6040516001600160a01b039091168152602001610236565b348015610520575f5ffd5b5061021361052f36600461470f565b61273b565b34801561053f575f5ffd5b5061047b612888565b348015610553575f5ffd5b50610446612969565b348015610567575f5ffd5b5061058a6105763660046149aa565b60296020525f908152604090205460ff1681565b6040519015158152602001610236565b3480156105a5575f5ffd5b5061058a612a34565b3480156105b9575f5ffd5b50610213612ad4565b3480156105cd575f5ffd5b50610229612b85565b3480156105e1575f5ffd5b506105ea612be3565b6040516102369291906149c1565b348015610603575f5ffd5b50601f5461058a9060ff1681565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610662573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061068691906149eb565b506106ba604051806040016040528060128152602001717665726966795374616c6542616c616e636560701b815250612c9c565b602480546040516308fa0b1360e21b815264ffffffffff841660048201525f926001600160a01b03909216916323e82c4c91015f60405180830381865afa158015610707573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261072e9190810190614b38565b6026548151602083015160408085015190516301c8abe960e11b81529495506001600160a01b039093169363039157d29361076d939291600401614c58565b5f604051808303815f87803b158015610784575f5ffd5b505af1925050508015610795575060015b6107d3573d8080156107c2576040519150601f19603f3d011682016040523d82523d5f602084013e6107c7565b606091505b506107d181612cf9565b505b5050565b6060601680548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f20905b81546001600160a01b0316815260019091019060200180831161080f575b5050505050905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610888573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108ac91906149eb565b506108dc6040518060400160405280600e81526020016d75706461746542616c616e63657360901b815250612c9c565b5f5b82518110156107d1575f8382815181106108fa576108fa614cb9565b602002602001015190505f83838151811061091757610917614cb9565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b0316036109dc57610956612d55565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109ca91906149eb565b156109d7576109d7612de4565b610b38565b5f8190505f836001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a1d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a419190614ccd565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015610a93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ab79190614ce8565b506021546040516373d0285560e11b81526001600160a01b0386811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303815f875af1158015610b10573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b3491906149eb565b5050505b50506001016108de565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015610b8b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610baf91906149eb565b50610be16040518060400160405280601081526020016f71756575655769746864726177616c7360801b815250612c9c565b602054604051631976849960e21b81523060048201525f916001600160a01b0316906365da126490602401602060405180830381865afa158015610c27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c4b9190614ccd565b60205460405163285e212160e21b815230600482018190529293505f916001600160a01b03169063a178848490602401602060405180830381865afa158015610c96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cba91906149eb565b6040805160018082528183019092529192505f9190816020015b604080516060808201835280825260208201525f91810191909152815260200190600190039081610cd45790505090506040518060600160405280888152602001878152602001846001600160a01b0316815250815f81518110610d3a57610d3a614cb9565b60209081029190910101526040805160018082528183019092525f91816020015b610d63613ef0565b815260200190600190039081610d5b5790505090506040518060e00160405280306001600160a01b03168152602001866001600160a01b03168152602001856001600160a01b031681526020018481526020014363ffffffff16815260200189815260200188815250815f81518110610dde57610dde614cb9565b602090810291909101810191909152546040516306ec6e8160e11b81525f916001600160a01b031690630dd8dd0290610e1b908690600401614d07565b5f604051808303815f875af1158015610e36573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610e5d9190810190614d99565b9050610e84825182516040518060600160405280602681526020016158a2602691396130de565b509450505050505b92915050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610ee3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f0791906149eb565b50610f3b604051806040016040528060128152602001713932b3b4b9ba32b920b9a7b832b930ba37b960711b815250612c9c565b604080516060810182523081525f60208083018281528385019283529054602854945163024b980360e51b815284516001600160a01b039081166004830152925183166024820152925163ffffffff9081166044850152909416606483015260a06084830152600860a4830152676d6574616461746160c01b60c48301529192919091169063497300609060e4015f604051808303815f87803b158015610fe0575f5ffd5b505af1158015610ff2573d5f5f3e3d5ffd5b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b8282101561112c575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015611115578382905f5260205f2001805461108a90614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546110b690614dca565b80156111015780601f106110d857610100808354040283529160200191611101565b820191905f5260205f20905b8154815290600101906020018083116110e457829003601f168201915b50505050508152602001906001019061106d565b50505050815250508152602001906001019061101c565b50505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561117e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111a291906149eb565b506111e16040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612c9c565b5f82516001600160401b038111156111fb576111fb613fc8565b60405190808252806020026020018201604052801561122e57816020015b60608152602001906001900390816112195790505b5090505f5b83518110156112855761126084828151811061125157611251614cb9565b6020026020010151600161314a565b82828151811061127257611272614cb9565b6020908102919091010152600101611233565b5090505b919050565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af11580156112d7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112fb91906149eb565b5061138e6040518060400160405280600f81526020016e666f726365556e64656c656761746560881b815250836001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611362573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526113899190810190614e02565b613464565b5f611398836134bf565b6020546040516336a2fa1960e21b81526001600160a01b03868116600483015292935091169063da8be864906024015f604051808303815f875af11580156113e2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526112859190810190614d99565b602354604080516301504d8f60e41b815290515f926001600160a01b031691631504d8f0916004808301926020929190829003018187875af1158015611451573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061147591906149eb565b506114a56040518060400160405280600e81526020016d6578697456616c696461746f727360901b815250612c9c565b610e8c826137d4565b6060601880548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161080f575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161080f575050505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af11580156115b3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115d791906149eb565b506116166040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612c9c565b610e8c82600161314a565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561166a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061168e91906149eb565b506116cd6040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612c9c565b5f82516001600160401b038111156116e7576116e7613fc8565b60405190808252806020026020018201604052801561171a57816020015b60608152602001906001900390816117055790505b5090505f5b83518110156112855761174b84828151811061173d5761173d614cb9565b60200260200101515f61314a565b82828151811061175d5761175d614cb9565b602090810291909101015260010161171f565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561112c578382905f5260205f2090600202016040518060400160405290815f820180546117c390614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546117ef90614dca565b801561183a5780601f106118115761010080835404028352916020019161183a565b820191905f5260205f20905b81548152906001019060200180831161181d57829003601f168201915b50505050508152602001600182018054806020026020016040519081016040528092919081815260200182805480156118bc57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161187e5790505b50505050508152505081526020019060010190611793565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561191d573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061194191906149eb565b506119806040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612c9c565b610e8c825f61314a565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156119db573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119ff91906149eb565b50611a3e6040518060400160405280601981526020017f6465706f736974496e746f456967656e6c617965725f414c5400000000000000815250612c9c565b5f195f5b8351811015611ed5575f848281518110611a5e57611a5e614cb9565b602002602001015190505f848381518110611a7b57611a7b614cb9565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603611b2f575f611abb613917565b50905060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611b0a575f5ffd5b505af1158015611b1c573d5f5f3e3d5ffd5b50505050611b2981613d0c565b50611ecb565b5f826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b909190614ccd565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015611be2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c069190614ce8565b50602154604051623f675f60e91b81523060048201525f916001600160a01b031690637ecebe0090602401602060405180830381865afa158015611c4c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c7091906149eb565b90505f60215f9054906101000a90046001600160a01b03166001600160a01b03166348825e946040518163ffffffff1660e01b8152600401602060405180830381865afa158015611cc3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ce791906149eb565b60408051602081019290925230908201526001600160a01b0380871660608301528416608082015260a0810185905260c0810183905260e081018890526101000160408051601f19818403018152828252805160209182012060215463f698da2560e01b855292519094505f936001600160a01b039093169263f698da259260048083019391928290030181865afa158015611d85573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611da991906149eb565b60405161190160f01b602082015260228101919091526042810183905260620160408051601f19818403018152828252805160209182012090830181905292505f91016040516020818303038152906040529050600160295f8481526020019081526020015f205f6101000a81548160ff02191690831515021790555060215f9054906101000a90046001600160a01b03166001600160a01b03166332e89ace888789308e876040518763ffffffff1660e01b8152600401611e7096959493929190614e46565b6020604051808303815f875af1158015611e8c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611eb091906149eb565b50505f908152602960205260409020805460ff191690555050505b5050600101611a42565b50505050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611f2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5091906149eb565b50611f8f6040518060400160405280601b81526020017f7665726966795769746864726177616c43726564656e7469616c730000000000815250612c9c565b611f9881613d0c565b50565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561112c578382905f5260205f20018054611fdb90614dca565b80601f016020809104026020016040519081016040528092919081815260200182805461200790614dca565b80156120525780601f1061202957610100808354040283529160200191612052565b820191905f5260205f20905b81548152906001019060200180831161203557829003601f168201915b505050505081526020019060010190611fbe565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156120b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120db91906149eb565b5061210c6040518060400160405280600f81526020016e1cdd185c9d10da1958dadc1bda5b9d608a1b815250612c9c565b612114612d55565b565b6060601d805480602002602001604051908101604052809291908181526020015f905b8282101561112c575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156121df57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116121a15790505b50505050508152505081526020019060010190612139565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015612240573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061226491906149eb565b506122906040518060400160405280600a815260200169756e64656c656761746560b01b815250612c9c565b5f61229a306134bf565b6020546040516336a2fa1960e21b81523060048201529192506001600160a01b03169063da8be864906024015f604051808303815f875af11580156122e1573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526123089190810190614d99565b505f5b815181101561250c575f5160206158035f395f51905f526040516123589060208082526015908201527432bc3832b1ba34b733903bb4ba34323930bbb0b61d60591b604082015260600190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882828151811061239357612393614cb9565b6020026020010151606001516040516123d0919060408082526007908201526603737b731b29d160cd1b6060820152602081019190915260800190565b60405180910390a17f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f82828151811061240b5761240b614cb9565b602002602001015160a001515f8151811061242857612428614cb9565b602002602001015160405161246a9190604080825260079082015266039ba3930ba1d160cd1b60608201526001600160a01b0391909116602082015260800190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a88282815181106124a5576124a5614cb9565b602002602001015160c001515f815181106124c2576124c2614cb9565b60200260200101516040516124fc9190604080825260089082015267039b430b932b99d160c51b6060820152602081019190915260800190565b60405180910390a160010161230b565b50905090565b6027546060905f906001600160401b0381111561253157612531613fc8565b60405190808252806020026020018201604052801561255a578160200160208202803683370190505b5090505f80805b6027548110156126ab57602454602780546001600160a01b039092169163aa47389c91908490811061259557612595614cb9565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff166040518263ffffffff1660e01b81526004016125e3919064ffffffffff91909116815260200190565b602060405180830381865afa1580156125fe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126229190614ce8565b156126a3576027818154811061263a5761263a614cb9565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff1684838151811061267157612671614cb9565b64ffffffffff909216602092830291909101909101528261269181614e9b565b935050818061269f90614e9b565b9250505b600101612561565b50508152919050565b6060602580546126c390614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546126ef90614dca565b801561082d5780601f106127115761010080835404028352916020019161082d565b820191905f5260205f20905b81548152906001019060200180831161271d57509395945050505050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af115801561278c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127b091906149eb565b506128126040518060400160405280600a81526020016964656c6567617465546f60b01b815250826001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611362573d5f5f3e3d5ffd5b604080518082018252606081525f602080830182905254925163eea9064b60e01b815291926001600160a01b03169163eea9064b916128579186918691600401614eb3565b5f604051808303815f87803b15801561286e575f5ffd5b505af1158015612880573d5f5f3e3d5ffd5b505050505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561112c575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561295157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116129135790505b505050505081525050815260200190600101906128ab565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561112c578382905f5260205f200180546129a990614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546129d590614dca565b8015612a205780601f106129f757610100808354040283529160200191612a20565b820191905f5260205f20905b815481529060010190602001808311612a0357829003601f168201915b50505050508152602001906001019061298c565b6008545f9060ff1615612a4b575060085460ff1690565b604051630667f9d760e41b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa158015612aa9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612acd91906149eb565b1415905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612b25573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b4991906149eb565b50612b7d6040518060400160405280601281526020017118dbdb5c1b195d1950da1958dadc1bda5b9d60721b815250612c9c565b612114612de4565b6060601580548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161080f575050505050905090565b60605f60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612c37573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c5b91906149eb565b50612c8c6040518060400160405280600f81526020016e737461727456616c696461746f727360881b815250612c9c565b612c94613917565b915091509091565b5f5160206158035f395f51905f52612cba612cb56126b4565b613dc3565b612cc383613dec565b604051602001612cd4929190614f0a565b60408051601f1981840301815290829052612cee91614998565b60405180910390a150565b805115612d0857805181602001fd5b60405162461bcd60e51b815260206004820152601b60248201527f7265766572746564207769746820756e6b6e6f776e206572726f72000000000060448201526064015b60405180910390fd5b6026546040516388676cad60e01b81525f60048201526001600160a01b03909116906388676cad906024015f604051808303815f87803b158015612d97575f5ffd5b505af1925050508015612da8575060015b612114573d808015612dd5576040519150601f19603f3d011682016040523d82523d5f602084013e612dda565b606091505b50611f9881612cf9565b604080518082018252601881527f2d206163746976652076616c696461746f7220636f756e7400000000000000006020808301919091526026548351632340e8d360e01b81529351612e89946001600160a01b0390921692632340e8d392600480820193918290030181865afa158015612e60573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e8491906149eb565b613e14565b60408051808201825260128152712d2070726f6f66732072656d61696e696e6760701b602082015260265482516323e941b960e11b81529251612f2c936001600160a01b03909216916347d283729160048083019260a09291908290030181865afa158015612efa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f1e9190614f29565b6020015162ffffff16613e14565b602654604080516321767f9560e11b815290515f926001600160a01b0316916342ecff2a9160048083019260209291908290030181865afa158015612f73573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f979190614fa2565b9050806001600160401b03165f0361300a5760405162461bcd60e51b815260206004820152603060248201527f557365722e5f636f6d706c657465436865636b706f696e743a206e6f2065786960448201526f1cdd1a5b99c818da1958dadc1bda5b9d60821b6064820152608401612d4c565b60245460405163b1b6f6a160e01b81525f916001600160a01b03169063b1b6f6a19061303d906027908690600401614fbb565b5f60405180830381865afa158015613057573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261307e9190810190615120565b90506130a760405180606001604052806022815260200161588060229139826020015151613e14565b6026548151602083015160405163783a5d3160e11b81526001600160a01b039093169263f074ba629261285792909160040161527a565b6040516388b44c8560e01b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d906388b44c859061311990869086908690600401615310565b5f6040518083038186803b15801561312f575f5ffd5b505afa158015613141573d5f5f3e3d5ffd5b50505050505050565b60605f8360a00151516001600160401b0381111561316a5761316a613fc8565b604051908082528060200260200182016040528015613193578160200160208202803683370190505b5090505f5b81518110156133fa575f8560a0015182815181106131b8576131b8614cb9565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316816001600160a01b03160361335e5773beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac083838151811061321657613216614cb9565b60200260200101906001600160a01b031690816001600160a01b03168152505084156133595761325d60405180606001604052806032815260200161584e60329139613e45565b61326d613268612512565b6137d4565b5060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156132ba575f5ffd5b505af11580156132cc573d5f5f3e3d5ffd5b505050506132d8612d55565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa158015613328573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061334c91906149eb565b1561335957613359612de4565b6133f1565b806001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa15801561339a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133be9190614ccd565b8383815181106133d0576133d0614cb9565b60200260200101906001600160a01b031690816001600160a01b0316815250505b50600101613198565b50602054604051630e4cc3f960e41b81526001600160a01b039091169063e4cc3f909061342f9087908590889060040161532e565b5f604051808303815f87803b158015613446575f5ffd5b505af1158015613458573d5f5f3e3d5ffd5b50929695505050505050565b5f5160206158035f395f51905f5261347d612cb56126b4565b61348684613dec565b8360405160200161349993929190615365565b60408051601f19818403018152908290526134b391614998565b60405180910390a15050565b6020546040516366d5ba9360e01b81526001600160a01b0383811660048301526060925f928392909116906366d5ba93906024015f60405180830381865afa15801561350d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613534919081019061539a565b915091505f82516001600160401b0381111561355257613552613fc8565b60405190808252806020026020018201604052801561358b57816020015b613578613ef0565b8152602001906001900390816135705790505b50602054604051631976849960e21b81526001600160a01b0388811660048301529293505f92909116906365da126490602401602060405180830381865afa1580156135d9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135fd9190614ccd565b60205460405163285e212160e21b81526001600160a01b0389811660048301529293505f929091169063a178848490602401602060405180830381865afa15801561364a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061366e91906149eb565b90505f5b85518110156137c8576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f929150602080830190803683370190505090508783815181106136d0576136d0614cb9565b6020026020010151825f815181106136ea576136ea614cb9565b60200260200101906001600160a01b031690816001600160a01b03168152505086838151811061371c5761371c614cb9565b6020026020010151815f8151811061373657613736614cb9565b6020026020010181815250506040518060e001604052808b6001600160a01b03168152602001866001600160a01b031681526020018b6001600160a01b0316815260200184866137869190615455565b81526020014363ffffffff168152602001838152602001828152508684815181106137b3576137b3614cb9565b60209081029190910101525050600101613672565b50919695505050505050565b5f6138156040518060400160405280601881526020017f2d2065786974696e67206e756d2076616c696461746f727300000000000000008152508351613e14565b5f5b82518110156138ce5760245483516001600160a01b039091169063f8f98a4e9085908490811061384957613849614cb9565b60200260200101516040518263ffffffff1660e01b815260040161387a919064ffffffffff91909116815260200190565b6020604051808303815f875af1158015613896573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138ba9190614fa2565b6138c49083615468565b9150600101613817565b506112896040518060400160405280601e81526020017f2d206578697465642062616c616e636520746f20706f64202867776569290000815250826001600160401b0316613e14565b60605f478161392f6801bc16d674ec8000008361549b565b9050613944816801bc16d674ec8000006154ae565b61394e90836154c5565b91505f81670de0b6b3a764000084106139955761396f633b9aca00856154d8565b61397990856154c5565b915061398582856154c5565b93508061399181614e9b565b9150505b805f03613a015760405162461bcd60e51b815260206004820152603460248201527f737461727456616c696461746f72733a206e6f7420656e6f75676820455448206044820152733a379039ba30b93a1030903b30b634b230ba37b960611b6064820152608401612d4c565b5f816001600160401b03811115613a1a57613a1a613fc8565b604051908082528060200260200182016040528015613a43578160200160208202803683370190505b5090505f633b9aca00613a5687476154c5565b613a60919061549b565b9050613aa26040518060400160405280601981526020017f2d206372656174696e67206e65772076616c696461746f7273000000000000008152508351613e14565b613acd6040518060600160405280602b8152602001615823602b9139826001600160401b0316613e14565b5f5b85811015613be3576024545f906001600160a01b031663ed3c16056801bc16d674ec800000613afc613e61565b6040518363ffffffff1660e01b8152600401613b189190614998565b60206040518083038185885af1158015613b34573d5f5f3e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190613b5991906154eb565b905080848381518110613b6e57613b6e614cb9565b64ffffffffff928316602091820292909201015260278054600181810183555f9290925260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054958516600592909306919091026101000a918202919093021990931692909217905501613acf565b50613bef856001615455565b8303613cff576024545f906001600160a01b031663ed3c160586613c11613e61565b6040518363ffffffff1660e01b8152600401613c2d9190614998565b60206040518083038185885af1158015613c49573d5f5f3e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190613c6e91906154eb565b9050808360018551613c8091906154c5565b81518110613c9057613c90614cb9565b64ffffffffff9283166020918202929092010152602780546001810182555f9190915260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054948416600592909306919091026101000a91820291909202199092169190911790555b9097909650945050505050565b6024546040516352851d0d60e11b81525f916001600160a01b03169063a50a3a1a90613d3c908590600401614986565b5f60405180830381865afa158015613d56573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613d7d9190810190615584565b6026548151602083015160408085015160608601519151633f65cf1960e01b81529596506001600160a01b0390941694633f65cf199461076d9493928992600401615717565b6060610e8c604051806040016040528060058152602001641b5b39366d60d81b81525083613ea6565b6060610e8c604051806040016040528060048152602001631b5b336d60e01b81525083613ea6565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882826040516134b39291906157c4565b5f5160206158035f395f51905f5281604051612cee9190614998565b60265460408051600160f81b60208201525f60218201526bffffffffffffffffffffffff19606093841b16602c82015201604051602081830303815290604052905090565b60608282604051806040016040528060048152602001631b5b306d60e01b815250604051602001613ed9939291906157e5565b604051602081830303815290604052905092915050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b64ffffffffff81168114611f98575f5ffd5b5f60208284031215613f6b575f5ffd5b8135613f7681613f49565b9392505050565b602080825282518282018190525f918401906040840190835b81811015613fbd5783516001600160a01b0316835260209384019390920191600101613f96565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715613ffe57613ffe613fc8565b60405290565b604080519081016001600160401b0381118282101715613ffe57613ffe613fc8565b604051606081016001600160401b0381118282101715613ffe57613ffe613fc8565b60405160a081016001600160401b0381118282101715613ffe57613ffe613fc8565b604051608081016001600160401b0381118282101715613ffe57613ffe613fc8565b604051601f8201601f191681016001600160401b03811182821017156140b4576140b4613fc8565b604052919050565b5f6001600160401b038211156140d4576140d4613fc8565b5060051b60200190565b6001600160a01b0381168114611f98575f5ffd5b5f82601f830112614101575f5ffd5b813561411461410f826140bc565b61408c565b8082825260208201915060208360051b860101925085831115614135575f5ffd5b602085015b8381101561415b57803561414d816140de565b83526020928301920161413a565b5095945050505050565b5f5f60408385031215614176575f5ffd5b82356001600160401b0381111561418b575f5ffd5b614197858286016140f2565b92505060208301356001600160401b038111156141b2575f5ffd5b8301601f810185136141c2575f5ffd5b80356141d061410f826140bc565b8082825260208201915060208360051b8501019250878311156141f1575f5ffd5b6020840193505b828410156142135783358252602093840193909101906141f8565b809450505050509250929050565b5f82601f830112614230575f5ffd5b813561423e61410f826140bc565b8082825260208201915060208360051b86010192508583111561425f575f5ffd5b602085015b8381101561415b578035835260209283019201614264565b5f5f6040838503121561428d575f5ffd5b82356001600160401b038111156142a2575f5ffd5b6142ae858286016140f2565b92505060208301356001600160401b038111156142c9575f5ffd5b6142d585828601614221565b9150509250929050565b5f8151808452602084019350602083015f5b828110156143185781516001600160a01b03168652602095860195909101906001016142f1565b5093949350505050565b5f8151808452602084019350602083015f5b82811015614318578151865260209586019590910190600101614334565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f9161439d9085018263ffffffff169052565b5060a082015160e060a08501526143b760e08501826142df565b905060c083015184820360c08601526143d08282614322565b95945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f1987860301845261441b858351614352565b945060209384019391909101906001016143ff565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b838110156144ac57601f19858403018852614496838351614430565b602098890198909350919091019060010161447a565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857868503603f19018452815180516001600160a01b031686526020908101516040918701829052906145199087018261445e565b95505060209384019391909101906001016144de565b8035611289816140de565b803563ffffffff81168114611289575f5ffd5b5f60e0828403121561455d575f5ffd5b614565613fdc565b90506145708261452f565b815261457e6020830161452f565b602082015261458f6040830161452f565b6040820152606082810135908201526145aa6080830161453a565b608082015260a08201356001600160401b038111156145c7575f5ffd5b6145d3848285016140f2565b60a08301525060c08201356001600160401b038111156145f1575f5ffd5b6145fd84828501614221565b60c08301525092915050565b5f60208284031215614619575f5ffd5b81356001600160401b0381111561462e575f5ffd5b8201601f8101841361463e575f5ffd5b803561464c61410f826140bc565b8082825260208201915060208360051b85010192508683111561466d575f5ffd5b602084015b838110156146ad5780356001600160401b0381111561468f575f5ffd5b61469e8960208389010161454d565b84525060209283019201614672565b509695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f198786030184526146fa8583516142df565b945060209384019391909101906001016146de565b5f6020828403121561471f575f5ffd5b8135613f76816140de565b5f6020828403121561473a575f5ffd5b81356001600160401b0381111561474f575f5ffd5b8201601f8101841361475f575f5ffd5b803561476d61410f826140bc565b8082825260208201915060208360051b85010192508683111561478e575f5ffd5b6020840193505b828410156147b95783356147a881613f49565b825260209384019390910190614795565b9695505050505050565b5f602082840312156147d3575f5ffd5b81356001600160401b038111156147e8575f5ffd5b6147f48482850161454d565b949350505050565b602081525f613f7660208301846142df565b5f8151808452602084019350602083015f5b828110156143185781516001600160e01b031916865260209586019590910190600101614820565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f1987860301845281518051604087526148946040880182614430565b90506020820151915086810360208801526148af818361480e565b96505050602093840193919091019060010161486e565b602081525f613f76602083018461445e565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857868503603f19018452815180516001600160a01b031686526020908101516040918701829052906149399087018261480e565b95505060209384019391909101906001016148fe565b5f8151808452602084019350602083015f5b8281101561431857815164ffffffffff16865260209586019590910190600101614961565b602081525f613f76602083018461494f565b602081525f613f766020830184614430565b5f602082840312156149ba575f5ffd5b5035919050565b604081525f6149d3604083018561494f565b90506001600160401b03831660208301529392505050565b5f602082840312156149fb575f5ffd5b5051919050565b80516001600160401b0381168114611289575f5ffd5b5f5f6001600160401b03841115614a3157614a31613fc8565b50601f8301601f1916602001614a468161408c565b915050828152838383011115614a5a575f5ffd5b8282602083015e5f602084830101529392505050565b5f82601f830112614a7f575f5ffd5b613f7683835160208501614a18565b5f60408284031215614a9e575f5ffd5b614aa6614004565b8251815260208301519091506001600160401b03811115614ac5575f5ffd5b614ad184828501614a70565b60208301525092915050565b5f82601f830112614aec575f5ffd5b8151614afa61410f826140bc565b8082825260208201915060208360051b860101925085831115614b1b575f5ffd5b602085015b8381101561415b578051835260209283019201614b20565b5f60208284031215614b48575f5ffd5b81516001600160401b03811115614b5d575f5ffd5b820160608185031215614b6e575f5ffd5b614b76614026565b614b7f82614a02565b815260208201516001600160401b03811115614b99575f5ffd5b614ba586828501614a8e565b60208301525060408201516001600160401b03811115614bc3575f5ffd5b919091019060408286031215614bd7575f5ffd5b614bdf614004565b82516001600160401b03811115614bf4575f5ffd5b614c0087828601614add565b82525060208301516001600160401b03811115614c1b575f5ffd5b614c2787828601614a70565b6020830152506040820152949350505050565b805182525f6020820151604060208501526147f46040850182614430565b6001600160401b0384168152606060208201525f614c796060830185614c3a565b8281036040840152835160408252614c946040830182614322565b905060208501518282036020840152614cad8282614430565b98975050505050505050565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614cdd575f5ffd5b8151613f76816140de565b5f60208284031215614cf8575f5ffd5b81518015158114613f76575f5ffd5b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f198786030184528151805160608752614d5360608801826142df565b905060208201518782036020890152614d6c8282614322565b6040938401516001600160a01b031698909301979097525094506020938401939190910190600101614d2d565b5f60208284031215614da9575f5ffd5b81516001600160401b03811115614dbe575f5ffd5b6147f484828501614add565b600181811c90821680614dde57607f821691505b602082108103614dfc57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f60208284031215614e12575f5ffd5b81516001600160401b03811115614e27575f5ffd5b8201601f81018413614e37575f5ffd5b6147f484825160208401614a18565b6001600160a01b038781168252868116602083015260408201869052841660608201526080810183905260c060a082018190525f90614cad90830184614430565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614eac57614eac614e87565b5060010190565b60018060a01b0384168152606060208201525f835160406060840152614edc60a0840182614430565b602095909501516080840152505060400152919050565b5f81518060208401855e5f93019283525090919050565b5f614f158285614ef3565b601760f91b81526143d06001820185614ef3565b5f60a0828403128015614f3a575f5ffd5b50614f43614048565b82518152602083015162ffffff81168114614f5c575f5ffd5b6020820152614f6d60408401614a02565b604082015260608301518060070b8114614f85575f5ffd5b6060820152614f9660808401614a02565b60808201529392505050565b5f60208284031215614fb2575f5ffd5b613f7682614a02565b5f6040820160408352808554614fd5818490815260200190565b5f8881526020812094509092505b8160058201101561504a57835464ffffffffff8082168552602882901c81166020860152605082901c81166040860152607882901c8116606086015260a082811c8216608087015260c89290921c169084015260019093019260c090920191600601614fe3565b925492818110156150695764ffffffffff841683526020909201916001015b818110156150895764ffffffffff602885901c1683526020909201916001015b818110156150a95764ffffffffff605085901c1683526020909201916001015b818110156150c95764ffffffffff607885901c1683526020909201916001015b818110156150e95764ffffffffff60a085901c1683526020909201916001015b818110156151065760c884901c64ffffffffff1683526020830192505b50506001600160401b03851660208501529150613f769050565b5f60208284031215615130575f5ffd5b81516001600160401b03811115615145575f5ffd5b820160408185031215615156575f5ffd5b61515e614004565b81516001600160401b03811115615173575f5ffd5b61517f86828501614a8e565b82525060208201516001600160401b0381111561519a575f5ffd5b80830192505084601f8301126151ae575f5ffd5b81516151bc61410f826140bc565b8082825260208201915060208360051b8601019250878311156151dd575f5ffd5b602085015b838110156152695780516001600160401b038111156151ff575f5ffd5b86016060818b03601f19011215615214575f5ffd5b61521c614026565b602082810151825260408301519082015260608201516001600160401b03811115615245575f5ffd5b6152548c602083860101614a70565b604083015250845250602092830192016151e2565b506020840152509095945050505050565b604081525f61528c6040830185614c3a565b828103602084015280845180835260208301915060208160051b840101602087015f5b8381101561530257601f198684030185528151805184526020810151602085015260408101519050606060408501526152eb6060850182614430565b6020968701969094509290920191506001016152af565b509098975050505050505050565b838152826020820152606060408201525f6143d06060830184614430565b606081525f6153406060830186614352565b828103602084015261535281866142df565b9150508215156040830152949350505050565b5f6153708286614ef3565b601760f91b81526153846001820186614ef3565b9050601d60f91b81526147b96001820185614ef3565b5f5f604083850312156153ab575f5ffd5b82516001600160401b038111156153c0575f5ffd5b8301601f810185136153d0575f5ffd5b80516153de61410f826140bc565b8082825260208201915060208360051b8501019250878311156153ff575f5ffd5b6020840193505b8284101561542a578351615419816140de565b825260209384019390910190615406565b8095505050505060208301516001600160401b03811115615449575f5ffd5b6142d585828601614add565b80820180821115610e8c57610e8c614e87565b6001600160401b038181168382160190811115610e8c57610e8c614e87565b634e487b7160e01b5f52601260045260245ffd5b5f826154a9576154a9615487565b500490565b8082028115828204841417610e8c57610e8c614e87565b81810381811115610e8c57610e8c614e87565b5f826154e6576154e6615487565b500690565b5f602082840312156154fb575f5ffd5b8151613f7681613f49565b5f82601f830112615515575f5ffd5b815161552361410f826140bc565b8082825260208201915060208360051b860101925085831115615544575f5ffd5b602085015b8381101561415b5780516001600160401b03811115615566575f5ffd5b615575886020838a0101614add565b84525060209283019201615549565b5f60208284031215615594575f5ffd5b81516001600160401b038111156155a9575f5ffd5b8201608081850312156155ba575f5ffd5b6155c261406a565b6155cb82614a02565b815260208201516001600160401b038111156155e5575f5ffd5b6155f186828501614a8e565b60208301525060408201516001600160401b0381111561560f575f5ffd5b8201601f8101861361561f575f5ffd5b805161562d61410f826140bc565b8082825260208201915060208360051b85010192508883111561564e575f5ffd5b602084015b8381101561568e5780516001600160401b03811115615670575f5ffd5b61567f8b602083890101614a70565b84525060209283019201615653565b50604085015250505060608201516001600160401b038111156156af575f5ffd5b6156bb86828501615506565b606083015250949350505050565b5f82825180855260208501945060208160051b830101602085015f5b838110156144ac57601f19858403018852615701838351614322565b60209889019890935091909101906001016156e5565b6001600160401b038616815260a060208201525f61573860a0830187614c3a565b828103604084015261574a818761494f565b9050828103606084015280855180835260208301915060208160051b840101602088015f5b838110156157a157601f1986840301855261578b838351614430565b602095860195909350919091019060010161576f565b505085810360808701526157b581886156c9565b9b9a5050505050505050505050565b604081525f6157d66040830185614430565b90508260208301529392505050565b5f6143d06157fc6157f68488614ef3565b86614ef3565b84614ef356fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f502d206465706f736974696e672062616c616e636520746f20626561636f6e20636861696e202867776569292d2065786974696e6720616c6c2076616c696461746f727320616e6420636f6d706c6574696e6720636865636b706f696e742d207375626d697474696e67206e756d20636865636b706f696e742070726f6f6673557365722e71756575655769746864726177616c733a206c656e677468206d69736d61746368a26469706673582212202e274a6220d7e1f20eeac1e5c441420b61660ad31d07fe0db048283f62125f9864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x0C\x80T`\x01`\xFF\x19\x90\x91\x16\x81\x17\x90\x91U`\x1F\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16tq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x01\x17\x90U`(\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90U4\x80\x15a\0WW__\xFD[P`@Qa^\xFD8\x03\x80a^\xFD\x839\x81\x01`@\x81\x90Ra\0v\x91a\x03\xCCV[\x80_3\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xEAM<\x9B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xDB\x91\x90a\x04\x93V[` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x81U`@\x80Qc\x076\xE1\xC7`\xE3\x1B\x81R\x90Q\x92\x84\x16\x92c9\xB7\x0E8\x92`\x04\x80\x84\x01\x93\x91\x92\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x011W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01U\x91\x90a\x04\x93V[`!_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16cFe\xBC\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xB6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDA\x91\x90a\x04\x93V[`\"_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c=\xFB@\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02_\x91\x90a\x04\x93V[`#_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c\"\xC05\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xC0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE4\x91\x90a\x04\x93V[`$\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x03\x0Ca\x03!V[`%a\x03\x18\x83\x82a\x059V[PPPPa\x05\xF3V[`\"_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x84\xD8\x10b`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x96\x91\x90a\x04\x93V[`&\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x03\xDCW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\xF1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x04\x01W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\x1AWa\x04\x1Aa\x03\xB8V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04HWa\x04Ha\x03\xB8V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x04_W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x90W__\xFD[PV[_` \x82\x84\x03\x12\x15a\x04\xA3W__\xFD[\x81Qa\x04\xAE\x81a\x04|V[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04\xC9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x04\xE7WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x054W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x05\x12WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x051W_\x81U`\x01\x01a\x05\x1EV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05RWa\x05Ra\x03\xB8V[a\x05f\x81a\x05`\x84Ta\x04\xB5V[\x84a\x04\xEDV[` `\x1F\x82\x11`\x01\x81\x14a\x05\x98W_\x83\x15a\x05\x81WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x051V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x05\xC7W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x05\xA7V[P\x84\x82\x10\x15a\x05\xE4W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[aX\xFD\x80a\x06\0_9_\xF3\xFE`\x80`@R`\x046\x10a\x01\xE9W_5`\xE0\x1C\x80c\x84\x1C\x12\x99\x11a\x01\x08W\x80c\xACc|z\x11a\0\x9DW\x80c\xBAAO\xA6\x11a\0mW\x80c\xBAAO\xA6\x14a\x05\x9AW\x80c\xD6\xC1\r\xAF\x14a\x05\xAEW\x80c\xE2\x0C\x9Fq\x14a\x05\xC2W\x80c\xF24\xC1\xBD\x14a\x05\xD6W\x80c\xFAv&\xD4\x14a\x05\xF8W__\xFD[\x80c\xACc|z\x14a\x05\x15W\x80c\xB0FO\xDC\x14a\x054W\x80c\xB5P\x8A\xA9\x14a\x05HW\x80c\xB7\x17#U\x14a\x05\\W__\xFD[\x80c\x92\xAB\x89\xBB\x11a\0\xD8W\x80c\x92\xAB\x89\xBB\x14a\x04\x88W\x80c\x9D\xE7\x02X\x14a\x04\x9CW\x80c\xA3\xF4\xDF~\x14a\x04\xBDW\x80c\xA8\x8D\xBB6\x14a\x04\xDEW__\xFD[\x80c\x84\x1C\x12\x99\x14a\x04\x13W\x80c\x85\"l\x81\x14a\x042W\x80c\x90\xB5\x16%\x14a\x04SW\x80c\x91j\x17\xC6\x14a\x04gW__\xFD[\x80c=\x8C\x08\xD4\x11a\x01~W\x80cF\xA5\xBE\r\x11a\x01NW\x80cF\xA5\xBE\r\x14a\x03\x95W\x80cf\xD9\xA9\xA0\x14a\x03\xB4W\x80ci_J\xE1\x14a\x03\xD5W\x80cm3oX\x14a\x03\xF4W__\xFD[\x80c=\x8C\x08\xD4\x14a\x03\nW\x80c>^<#\x14a\x03AW\x80c?r\x86\xF4\x14a\x03UW\x80c@\x1B\xE6^\x14a\x03iW__\xFD[\x80c*4\xAD\xE8\x11a\x01\xB9W\x80c*4\xAD\xE8\x14a\x02\x8AW\x80c*\xDE8\x80\x14a\x02\x9EW\x80c4N\x13\x83\x14a\x02\xBFW\x80c9\x1C\xC9\xF6\x14a\x02\xEBW__\xFD[\x80c\x07\x1C%\xB7\x14a\x01\xF4W\x80c\x1E\xD7\x83\x1C\x14a\x02\x15W\x80c \xA5E\xD9\x14a\x02?W\x80c#\xE4\x81u\x14a\x02^W__\xFD[6a\x01\xF0W\0[__\xFD[4\x80\x15a\x01\xFFW__\xFD[Pa\x02\x13a\x02\x0E6`\x04a?[V[a\x06\x11V[\0[4\x80\x15a\x02 W__\xFD[Pa\x02)a\x07\xD7V[`@Qa\x026\x91\x90a?}V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02JW__\xFD[Pa\x02\x13a\x02Y6`\x04aAeV[a\x087V[4\x80\x15a\x02iW__\xFD[Pa\x02}a\x02x6`\x04aB|V[a\x0BBV[`@Qa\x026\x91\x90aC\xD9V[4\x80\x15a\x02\x95W__\xFD[Pa\x02\x13a\x0E\x92V[4\x80\x15a\x02\xA9W__\xFD[Pa\x02\xB2a\x0F\xF9V[`@Qa\x026\x91\x90aD\xB8V[4\x80\x15a\x02\xCAW__\xFD[Pa\x02\xDEa\x02\xD96`\x04aF\tV[a\x115V[`@Qa\x026\x91\x90aF\xB8V[4\x80\x15a\x02\xF6W__\xFD[Pa\x02}a\x03\x056`\x04aG\x0FV[a\x12\x8EV[4\x80\x15a\x03\x15W__\xFD[Pa\x03)a\x03$6`\x04aG*V[a\x14\tV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x026V[4\x80\x15a\x03LW__\xFD[Pa\x02)a\x14\xAEV[4\x80\x15a\x03`W__\xFD[Pa\x02)a\x15\x0CV[4\x80\x15a\x03tW__\xFD[Pa\x03\x88a\x03\x836`\x04aG\xC3V[a\x15jV[`@Qa\x026\x91\x90aG\xFCV[4\x80\x15a\x03\xA0W__\xFD[Pa\x02\xDEa\x03\xAF6`\x04aF\tV[a\x16!V[4\x80\x15a\x03\xBFW__\xFD[Pa\x03\xC8a\x17pV[`@Qa\x026\x91\x90aHHV[4\x80\x15a\x03\xE0W__\xFD[Pa\x03\x88a\x03\xEF6`\x04aG\xC3V[a\x18\xD4V[4\x80\x15a\x03\xFFW__\xFD[Pa\x02\x13a\x04\x0E6`\x04aB|V[a\x19\x8AV[4\x80\x15a\x04\x1EW__\xFD[Pa\x02\x13a\x04-6`\x04aG*V[a\x1E\xDBV[4\x80\x15a\x04=W__\xFD[Pa\x04Fa\x1F\x9BV[`@Qa\x026\x91\x90aH\xC6V[4\x80\x15a\x04^W__\xFD[Pa\x02\x13a fV[4\x80\x15a\x04rW__\xFD[Pa\x04{a!\x16V[`@Qa\x026\x91\x90aH\xD8V[4\x80\x15a\x04\x93W__\xFD[Pa\x02}a!\xF7V[4\x80\x15a\x04\xA7W__\xFD[Pa\x04\xB0a%\x12V[`@Qa\x026\x91\x90aI\x86V[4\x80\x15a\x04\xC8W__\xFD[Pa\x04\xD1a&\xB4V[`@Qa\x026\x91\x90aI\x98V[4\x80\x15a\x04\xE9W__\xFD[P`&Ta\x04\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x026V[4\x80\x15a\x05 W__\xFD[Pa\x02\x13a\x05/6`\x04aG\x0FV[a';V[4\x80\x15a\x05?W__\xFD[Pa\x04{a(\x88V[4\x80\x15a\x05SW__\xFD[Pa\x04Fa)iV[4\x80\x15a\x05gW__\xFD[Pa\x05\x8Aa\x05v6`\x04aI\xAAV[`)` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x026V[4\x80\x15a\x05\xA5W__\xFD[Pa\x05\x8Aa*4V[4\x80\x15a\x05\xB9W__\xFD[Pa\x02\x13a*\xD4V[4\x80\x15a\x05\xCDW__\xFD[Pa\x02)a+\x85V[4\x80\x15a\x05\xE1W__\xFD[Pa\x05\xEAa+\xE3V[`@Qa\x026\x92\x91\x90aI\xC1V[4\x80\x15a\x06\x03W__\xFD[P`\x1FTa\x05\x8A\x90`\xFF\x16\x81V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x86\x91\x90aI\xEBV[Pa\x06\xBA`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qverifyStaleBalance`p\x1B\x81RPa,\x9CV[`$\x80T`@Qc\x08\xFA\x0B\x13`\xE2\x1B\x81Rd\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c#\xE8,L\x91\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x07W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07.\x91\x90\x81\x01\x90aK8V[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x01\xC8\xAB\xE9`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93c\x03\x91W\xD2\x93a\x07m\x93\x92\x91`\x04\x01aLXV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x84W__\xFD[PZ\xF1\x92PPP\x80\x15a\x07\x95WP`\x01[a\x07\xD3W=\x80\x80\x15a\x07\xC2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xC7V[``\x91P[Pa\x07\xD1\x81a,\xF9V[P[PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FW[PPPPP\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xAC\x91\x90aI\xEBV[Pa\x08\xDC`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mupdateBalances`\x90\x1B\x81RPa,\x9CV[_[\x82Q\x81\x10\x15a\x07\xD1W_\x83\x82\x81Q\x81\x10a\x08\xFAWa\x08\xFAaL\xB9V[` \x02` \x01\x01Q\x90P_\x83\x83\x81Q\x81\x10a\t\x17Wa\t\x17aL\xB9V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\xDCWa\tVa-UV[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xCA\x91\x90aI\xEBV[\x15a\t\xD7Wa\t\xD7a-\xE4V[a\x0B8V[_\x81\x90P_\x83`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nA\x91\x90aL\xCDV[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB7\x91\x90aL\xE8V[P`!T`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B4\x91\x90aI\xEBV[PPP[PP`\x01\x01a\x08\xDEV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x0B\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xAF\x91\x90aI\xEBV[Pa\x0B\xE1`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oqueueWithdrawals`\x80\x1B\x81RPa,\x9CV[` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R0`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CK\x91\x90aL\xCDV[` T`@Qc(^!!`\xE2\x1B\x81R0`\x04\x82\x01\x81\x90R\x92\x93P_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBA\x91\x90aI\xEBV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R_\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xD4W\x90PP\x90P`@Q\x80``\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81_\x81Q\x81\x10a\r:Wa\r:aL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[a\rca>\xF0V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r[W\x90PP\x90P`@Q\x80`\xE0\x01`@R\x800`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x88\x81RP\x81_\x81Q\x81\x10a\r\xDEWa\r\xDEaL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91RT`@Qc\x06\xECn\x81`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\r\xD8\xDD\x02\x90a\x0E\x1B\x90\x86\x90`\x04\x01aM\x07V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E6W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E]\x91\x90\x81\x01\x90aM\x99V[\x90Pa\x0E\x84\x82Q\x82Q`@Q\x80``\x01`@R\x80`&\x81R` \x01aX\xA2`&\x919a0\xDEV[P\x94PPPPP[\x92\x91PPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x07\x91\x90aI\xEBV[Pa\x0F;`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q92\xB3\xB4\xB9\xBA2\xB9 \xB9\xA7\xB82\xB90\xBA7\xB9`q\x1B\x81RPa,\x9CV[`@\x80Q``\x81\x01\x82R0\x81R_` \x80\x83\x01\x82\x81R\x83\x85\x01\x92\x83R\x90T`(T\x94Qc\x02K\x98\x03`\xE5\x1B\x81R\x84Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x92Q\x83\x16`$\x82\x01R\x92Qc\xFF\xFF\xFF\xFF\x90\x81\x16`D\x85\x01R\x90\x94\x16`d\x83\x01R`\xA0`\x84\x83\x01R`\x08`\xA4\x83\x01Rgmetadata`\xC0\x1B`\xC4\x83\x01R\x91\x92\x91\x90\x91\x16\x90cIs\0`\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xE0W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xF2W=__>=_\xFD[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x11\x15W\x83\x82\x90_R` _ \x01\x80Ta\x10\x8A\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xB6\x90aM\xCAV[\x80\x15a\x11\x01W\x80`\x1F\x10a\x10\xD8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\x01V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x10mV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10\x1CV[PPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x11~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA2\x91\x90aI\xEBV[Pa\x11\xE1`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa,\x9CV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xFBWa\x11\xFBa?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12.W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\x19W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12\x85Wa\x12`\x84\x82\x81Q\x81\x10a\x12QWa\x12QaL\xB9V[` \x02` \x01\x01Q`\x01a1JV[\x82\x82\x81Q\x81\x10a\x12rWa\x12raL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x123V[P\x90P[\x91\x90PV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x12\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFB\x91\x90aI\xEBV[Pa\x13\x8E`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nforceUndelegate`\x88\x1B\x81RP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13bW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\x89\x91\x90\x81\x01\x90aN\x02V[a4dV[_a\x13\x98\x83a4\xBFV[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xE2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\x85\x91\x90\x81\x01\x90aM\x99V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\x14QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14u\x91\x90aI\xEBV[Pa\x14\xA5`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mexitValidators`\x90\x1B\x81RPa,\x9CV[a\x0E\x8C\x82a7\xD4V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FWPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FWPPPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x15\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD7\x91\x90aI\xEBV[Pa\x16\x16`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa,\x9CV[a\x0E\x8C\x82`\x01a1JV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x16jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x8E\x91\x90aI\xEBV[Pa\x16\xCD`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa,\x9CV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE7Wa\x16\xE7a?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x1AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17\x05W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12\x85Wa\x17K\x84\x82\x81Q\x81\x10a\x17=Wa\x17=aL\xB9V[` \x02` \x01\x01Q_a1JV[\x82\x82\x81Q\x81\x10a\x17]Wa\x17]aL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17\x1FV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x17\xC3\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xEF\x90aM\xCAV[\x80\x15a\x18:W\x80`\x1F\x10a\x18\x11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18:V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18\xBCW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18~W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x17\x93V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x19\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19A\x91\x90aI\xEBV[Pa\x19\x80`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa,\x9CV[a\x0E\x8C\x82_a1JV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\xDBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xFF\x91\x90aI\xEBV[Pa\x1A>`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FdepositIntoEigenlayer_ALT\0\0\0\0\0\0\0\x81RPa,\x9CV[_\x19_[\x83Q\x81\x10\x15a\x1E\xD5W_\x84\x82\x81Q\x81\x10a\x1A^Wa\x1A^aL\xB9V[` \x02` \x01\x01Q\x90P_\x84\x83\x81Q\x81\x10a\x1A{Wa\x1A{aL\xB9V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B/W_a\x1A\xBBa9\x17V[P\x90P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\nW__\xFD[PZ\xF1\x15\x80\x15a\x1B\x1CW=__>=_\xFD[PPPPa\x1B)\x81a=\x0CV[Pa\x1E\xCBV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x90\x91\x90aL\xCDV[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x06\x91\x90aL\xE8V[P`!T`@Qb?g_`\xE9\x1B\x81R0`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c~\xCE\xBE\0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cp\x91\x90aI\xEBV[\x90P_`!_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cH\x82^\x94`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE7\x91\x90aI\xEBV[`@\x80Q` \x81\x01\x92\x90\x92R0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16``\x83\x01R\x84\x16`\x80\x82\x01R`\xA0\x81\x01\x85\x90R`\xC0\x81\x01\x83\x90R`\xE0\x81\x01\x88\x90Ra\x01\0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `!Tc\xF6\x98\xDA%`\xE0\x1B\x85R\x92Q\x90\x94P_\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF6\x98\xDA%\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1D\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA9\x91\x90aI\xEBV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92P_\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\x01`)_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`!_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\xE8\x9A\xCE\x88\x87\x890\x8E\x87`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Ep\x96\x95\x94\x93\x92\x91\x90aNFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xB0\x91\x90aI\xEBV[PP_\x90\x81R`)` R`@\x90 \x80T`\xFF\x19\x16\x90UPPP[PP`\x01\x01a\x1ABV[PPPPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FP\x91\x90aI\xEBV[Pa\x1F\x8F`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FverifyWithdrawalCredentials\0\0\0\0\0\x81RPa,\x9CV[a\x1F\x98\x81a=\x0CV[PV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W\x83\x82\x90_R` _ \x01\x80Ta\x1F\xDB\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x07\x90aM\xCAV[\x80\x15a RW\x80`\x1F\x10a )Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a RV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a 5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xBEV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xDB\x91\x90aI\xEBV[Pa!\x0C`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x1C\xDD\x18\\\x9D\x10\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x8A\x1B\x81RPa,\x9CV[a!\x14a-UV[V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a!\xDFW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a!\xA1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!9V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\"@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"d\x91\x90aI\xEBV[Pa\"\x90`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iundelegate`\xB0\x1B\x81RPa,\x9CV[_a\"\x9A0a4\xBFV[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xE1W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\x08\x91\x90\x81\x01\x90aM\x99V[P_[\x81Q\x81\x10\x15a%\x0CW_Q` aX\x03_9_Q\x90_R`@Qa#X\x90` \x80\x82R`\x15\x90\x82\x01Rt2\xBC82\xB1\xBA4\xB73\x90;\xB4\xBA4290\xBB\xB0\xB6\x1D`Y\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a#\x93Wa#\x93aL\xB9V[` \x02` \x01\x01Q``\x01Q`@Qa#\xD0\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03s{s\x1B)\xD1`\xCD\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x82\x82\x81Q\x81\x10a$\x0BWa$\x0BaL\xB9V[` \x02` \x01\x01Q`\xA0\x01Q_\x81Q\x81\x10a$(Wa$(aL\xB9V[` \x02` \x01\x01Q`@Qa$j\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03\x9B\xA3\x93\x0B\xA1\xD1`\xCD\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a$\xA5Wa$\xA5aL\xB9V[` \x02` \x01\x01Q`\xC0\x01Q_\x81Q\x81\x10a$\xC2Wa$\xC2aL\xB9V[` \x02` \x01\x01Q`@Qa$\xFC\x91\x90`@\x80\x82R`\x08\x90\x82\x01Rg\x03\x9BC\x0B\x93+\x99\xD1`\xC5\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a#\x0BV[P\x90P\x90V[`'T``\x90_\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a%1Wa%1a?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%ZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80\x80[`'T\x81\x10\x15a&\xABW`$T`'\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xAAG8\x9C\x91\x90\x84\x90\x81\x10a%\x95Wa%\x95aL\xB9V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a%\xE3\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\"\x91\x90aL\xE8V[\x15a&\xA3W`'\x81\x81T\x81\x10a&:Wa&:aL\xB9V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10a&qWa&qaL\xB9V[d\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a&\x91\x81aN\x9BV[\x93PP\x81\x80a&\x9F\x90aN\x9BV[\x92PP[`\x01\x01a%aV[PP\x81R\x91\x90PV[```%\x80Ta&\xC3\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xEF\x90aM\xCAV[\x80\x15a\x08-W\x80`\x1F\x10a'\x11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08-V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\x1DWP\x93\x95\x94PPPPPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xB0\x91\x90aI\xEBV[Pa(\x12`@Q\x80`@\x01`@R\x80`\n\x81R` \x01idelegateTo`\xB0\x1B\x81RP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13bW=__>=_\xFD[`@\x80Q\x80\x82\x01\x82R``\x81R_` \x80\x83\x01\x82\x90RT\x92Qc\xEE\xA9\x06K`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE\xA9\x06K\x91a(W\x91\x86\x91\x86\x91`\x04\x01aN\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(nW__\xFD[PZ\xF1\x15\x80\x15a(\x80W=__>=_\xFD[PPPPPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a)QW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a)\x13W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a(\xABV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W\x83\x82\x90_R` _ \x01\x80Ta)\xA9\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta)\xD5\x90aM\xCAV[\x80\x15a* W\x80`\x1F\x10a)\xF7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a* V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a)\x8CV[`\x08T_\x90`\xFF\x16\x15a*KWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xCD\x91\x90aI\xEBV[\x14\x15\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+I\x91\x90aI\xEBV[Pa+}`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\xDB\xDB\\\x1B\x19]\x19P\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`r\x1B\x81RPa,\x9CV[a!\x14a-\xE4V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FWPPPPP\x90P\x90V[``_`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,[\x91\x90aI\xEBV[Pa,\x8C`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nstartValidators`\x88\x1B\x81RPa,\x9CV[a,\x94a9\x17V[\x91P\x91P\x90\x91V[_Q` aX\x03_9_Q\x90_Ra,\xBAa,\xB5a&\xB4V[a=\xC3V[a,\xC3\x83a=\xECV[`@Q` \x01a,\xD4\x92\x91\x90aO\nV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra,\xEE\x91aI\x98V[`@Q\x80\x91\x03\x90\xA1PV[\x80Q\x15a-\x08W\x80Q\x81` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Freverted with unknown error\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`&T`@Qc\x88gl\xAD`\xE0\x1B\x81R_`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x88gl\xAD\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\x97W__\xFD[PZ\xF1\x92PPP\x80\x15a-\xA8WP`\x01[a!\x14W=\x80\x80\x15a-\xD5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a-\xDAV[``\x91P[Pa\x1F\x98\x81a,\xF9V[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F- active validator count\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R`&T\x83Qc#@\xE8\xD3`\xE0\x1B\x81R\x93Qa.\x89\x94`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c#@\xE8\xD3\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a.`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x84\x91\x90aI\xEBV[a>\x14V[`@\x80Q\x80\x82\x01\x82R`\x12\x81Rq- proofs remaining`p\x1B` \x82\x01R`&T\x82Qc#\xE9A\xB9`\xE1\x1B\x81R\x92Qa/,\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cG\xD2\x83r\x91`\x04\x80\x83\x01\x92`\xA0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a.\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x1E\x91\x90aO)V[` \x01Qb\xFF\xFF\xFF\x16a>\x14V[`&T`@\x80Qc!v\x7F\x95`\xE1\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cB\xEC\xFF*\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a/sW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x97\x91\x90aO\xA2V[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a0\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FUser._completeCheckpoint: no exi`D\x82\x01Ro\x1C\xDD\x1A[\x99\xC8\x18\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x82\x1B`d\x82\x01R`\x84\x01a-LV[`$T`@Qc\xB1\xB6\xF6\xA1`\xE0\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xB6\xF6\xA1\x90a0=\x90`'\x90\x86\x90`\x04\x01aO\xBBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0WW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra0~\x91\x90\x81\x01\x90aQ V[\x90Pa0\xA7`@Q\x80``\x01`@R\x80`\"\x81R` \x01aX\x80`\"\x919\x82` \x01QQa>\x14V[`&T\x81Q` \x83\x01Q`@Qcx:]1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF0t\xBAb\x92a(W\x92\x90\x91`\x04\x01aRzV[`@Qc\x88\xB4L\x85`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x88\xB4L\x85\x90a1\x19\x90\x86\x90\x86\x90\x86\x90`\x04\x01aS\x10V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a1/W__\xFD[PZ\xFA\x15\x80\x15a1AW=__>=_\xFD[PPPPPPPV[``_\x83`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a1jWa1ja?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\x93W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a3\xFAW_\x85`\xA0\x01Q\x82\x81Q\x81\x10a1\xB8Wa1\xB8aL\xB9V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a3^Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x83\x83\x81Q\x81\x10a2\x16Wa2\x16aL\xB9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x15a3YWa2]`@Q\x80``\x01`@R\x80`2\x81R` \x01aXN`2\x919a>EV[a2ma2ha%\x12V[a7\xD4V[P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\xBAW__\xFD[PZ\xF1\x15\x80\x15a2\xCCW=__>=_\xFD[PPPPa2\xD8a-UV[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3L\x91\x90aI\xEBV[\x15a3YWa3Ya-\xE4V[a3\xF1V[\x80`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xBE\x91\x90aL\xCDV[\x83\x83\x81Q\x81\x10a3\xD0Wa3\xD0aL\xB9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP[P`\x01\x01a1\x98V[P` T`@Qc\x0EL\xC3\xF9`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\xCC?\x90\x90a4/\x90\x87\x90\x85\x90\x88\x90`\x04\x01aS.V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4FW__\xFD[PZ\xF1\x15\x80\x15a4XW=__>=_\xFD[P\x92\x96\x95PPPPPPV[_Q` aX\x03_9_Q\x90_Ra4}a,\xB5a&\xB4V[a4\x86\x84a=\xECV[\x83`@Q` \x01a4\x99\x93\x92\x91\x90aSeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra4\xB3\x91aI\x98V[`@Q\x80\x91\x03\x90\xA1PPV[` T`@Qcf\xD5\xBA\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R``\x92_\x92\x83\x92\x90\x91\x16\x90cf\xD5\xBA\x93\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\rW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra54\x91\x90\x81\x01\x90aS\x9AV[\x91P\x91P_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a5RWa5Ra?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\x8BW\x81` \x01[a5xa>\xF0V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a5pW\x90P[P` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xFD\x91\x90aL\xCDV[` T`@Qc(^!!`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6n\x91\x90aI\xEBV[\x90P_[\x85Q\x81\x10\x15a7\xC8W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a6\xD0Wa6\xD0aL\xB9V[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a6\xEAWa6\xEAaL\xB9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x86\x83\x81Q\x81\x10a7\x1CWa7\x1CaL\xB9V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a76Wa76aL\xB9V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x86a7\x86\x91\x90aTUV[\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81RP\x86\x84\x81Q\x81\x10a7\xB3Wa7\xB3aL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a6rV[P\x91\x96\x95PPPPPPV[_a8\x15`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F- exiting num validators\0\0\0\0\0\0\0\0\x81RP\x83Qa>\x14V[_[\x82Q\x81\x10\x15a8\xCEW`$T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xF9\x8AN\x90\x85\x90\x84\x90\x81\x10a8IWa8IaL\xB9V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8z\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xBA\x91\x90aO\xA2V[a8\xC4\x90\x83aThV[\x91P`\x01\x01a8\x17V[Pa\x12\x89`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7F- exited balance to pod (gwei)\0\0\x81RP\x82`\x01`\x01`@\x1B\x03\x16a>\x14V[``_G\x81a9/h\x01\xBC\x16\xD6t\xEC\x80\0\0\x83aT\x9BV[\x90Pa9D\x81h\x01\xBC\x16\xD6t\xEC\x80\0\0aT\xAEV[a9N\x90\x83aT\xC5V[\x91P_\x81g\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a9\x95Wa9oc;\x9A\xCA\0\x85aT\xD8V[a9y\x90\x85aT\xC5V[\x91Pa9\x85\x82\x85aT\xC5V[\x93P\x80a9\x91\x81aN\x9BV[\x91PP[\x80_\x03a:\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FstartValidators: not enough ETH `D\x82\x01Rs:7\x909\xBA0\xB9:\x100\x90;0\xB64\xB20\xBA7\xB9`a\x1B`d\x82\x01R`\x84\x01a-LV[_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x1AWa:\x1Aa?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a:CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_c;\x9A\xCA\0a:V\x87GaT\xC5V[a:`\x91\x90aT\x9BV[\x90Pa:\xA2`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- creating new validators\0\0\0\0\0\0\0\x81RP\x83Qa>\x14V[a:\xCD`@Q\x80``\x01`@R\x80`+\x81R` \x01aX#`+\x919\x82`\x01`\x01`@\x1B\x03\x16a>\x14V[_[\x85\x81\x10\x15a;\xE3W`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05h\x01\xBC\x16\xD6t\xEC\x80\0\0a:\xFCa>aV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x18\x91\x90aI\x98V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a;4W=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;Y\x91\x90aT\xEBV[\x90P\x80\x84\x83\x81Q\x81\x10a;nWa;naL\xB9V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x81\x01\x83U_\x92\x90\x92R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x95\x85\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x93\x02\x19\x90\x93\x16\x92\x90\x92\x17\x90U\x01a:\xCFV[Pa;\xEF\x85`\x01aTUV[\x83\x03a<\xFFW`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05\x86a<\x11a>aV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<-\x91\x90aI\x98V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a<IW=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<n\x91\x90aT\xEBV[\x90P\x80\x83`\x01\x85Qa<\x80\x91\x90aT\xC5V[\x81Q\x81\x10a<\x90Wa<\x90aL\xB9V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x01\x82U_\x91\x90\x91R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x94\x84\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x92\x02\x19\x90\x92\x16\x91\x90\x91\x17\x90U[\x90\x97\x90\x96P\x94PPPPPV[`$T`@QcR\x85\x1D\r`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA5\n:\x1A\x90a=<\x90\x85\x90`\x04\x01aI\x86V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=VW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra=}\x91\x90\x81\x01\x90aU\x84V[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc?e\xCF\x19`\xE0\x1B\x81R\x95\x96P`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x94c?e\xCF\x19\x94a\x07m\x94\x93\x92\x89\x92`\x04\x01aW\x17V[``a\x0E\x8C`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83a>\xA6V[``a\x0E\x8C`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83a>\xA6V[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa4\xB3\x92\x91\x90aW\xC4V[_Q` aX\x03_9_Q\x90_R\x81`@Qa,\xEE\x91\x90aI\x98V[`&T`@\x80Q`\x01`\xF8\x1B` \x82\x01R_`!\x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16`,\x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01a>\xD9\x93\x92\x91\x90aW\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F\x98W__\xFD[_` \x82\x84\x03\x12\x15a?kW__\xFD[\x815a?v\x81a?IV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a?\xBDW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a?\x96V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xB4Wa@\xB4a?\xC8V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a@\xD4Wa@\xD4a?\xC8V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\x98W__\xFD[_\x82`\x1F\x83\x01\x12aA\x01W__\xFD[\x815aA\x14aA\x0F\x82a@\xBCV[a@\x8CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aA5W__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x805aAM\x81a@\xDEV[\x83R` \x92\x83\x01\x92\x01aA:V[P\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15aAvW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x8BW__\xFD[aA\x97\x85\x82\x86\x01a@\xF2V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xB2W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aA\xC2W__\xFD[\x805aA\xD0aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aA\xF1W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aB\x13W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aA\xF8V[\x80\x94PPPPP\x92P\x92\x90PV[_\x82`\x1F\x83\x01\x12aB0W__\xFD[\x815aB>aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aB_W__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x805\x83R` \x92\x83\x01\x92\x01aBdV[__`@\x83\x85\x03\x12\x15aB\x8DW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xA2W__\xFD[aB\xAE\x85\x82\x86\x01a@\xF2V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xC9W__\xFD[aB\xD5\x85\x82\x86\x01aB!V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aB\xF1V[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aC4V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aC\x9D\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaC\xB7`\xE0\x85\x01\x82aB\xDFV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaC\xD0\x82\x82aC\"V[\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84RaD\x1B\x85\x83QaCRV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aC\xFFV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aD\xACW`\x1F\x19\x85\x84\x03\x01\x88RaD\x96\x83\x83QaD0V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aDzV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aE\x19\x90\x87\x01\x82aD^V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aD\xDEV[\x805a\x12\x89\x81a@\xDEV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\x89W__\xFD[_`\xE0\x82\x84\x03\x12\x15aE]W__\xFD[aEea?\xDCV[\x90PaEp\x82aE/V[\x81RaE~` \x83\x01aE/V[` \x82\x01RaE\x8F`@\x83\x01aE/V[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaE\xAA`\x80\x83\x01aE:V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xC7W__\xFD[aE\xD3\x84\x82\x85\x01a@\xF2V[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xF1W__\xFD[aE\xFD\x84\x82\x85\x01aB!V[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aF\x19W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aF.W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aF>W__\xFD[\x805aFLaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aFmW__\xFD[` \x84\x01[\x83\x81\x10\x15aF\xADW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x8FW__\xFD[aF\x9E\x89` \x83\x89\x01\x01aEMV[\x84RP` \x92\x83\x01\x92\x01aFrV[P\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84RaF\xFA\x85\x83QaB\xDFV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aF\xDEV[_` \x82\x84\x03\x12\x15aG\x1FW__\xFD[\x815a?v\x81a@\xDEV[_` \x82\x84\x03\x12\x15aG:W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aGOW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aG_W__\xFD[\x805aGmaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aG\x8EW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aG\xB9W\x835aG\xA8\x81a?IV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aG\x95V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aG\xD3W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xE8W__\xFD[aG\xF4\x84\x82\x85\x01aEMV[\x94\x93PPPPV[` \x81R_a?v` \x83\x01\x84aB\xDFV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aH V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87RaH\x94`@\x88\x01\x82aD0V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01RaH\xAF\x81\x83aH\x0EV[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aHnV[` \x81R_a?v` \x83\x01\x84aD^V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aI9\x90\x87\x01\x82aH\x0EV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aH\xFEV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Qd\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aIaV[` \x81R_a?v` \x83\x01\x84aIOV[` \x81R_a?v` \x83\x01\x84aD0V[_` \x82\x84\x03\x12\x15aI\xBAW__\xFD[P5\x91\x90PV[`@\x81R_aI\xD3`@\x83\x01\x85aIOV[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aI\xFBW__\xFD[PQ\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x12\x89W__\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15aJ1WaJ1a?\xC8V[P`\x1F\x83\x01`\x1F\x19\x16` \x01aJF\x81a@\x8CV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15aJZW__\xFD[\x82\x82` \x83\x01^_` \x84\x83\x01\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aJ\x7FW__\xFD[a?v\x83\x83Q` \x85\x01aJ\x18V[_`@\x82\x84\x03\x12\x15aJ\x9EW__\xFD[aJ\xA6a@\x04V[\x82Q\x81R` \x83\x01Q\x90\x91P`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xC5W__\xFD[aJ\xD1\x84\x82\x85\x01aJpV[` \x83\x01RP\x92\x91PPV[_\x82`\x1F\x83\x01\x12aJ\xECW__\xFD[\x81QaJ\xFAaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aK\x1BW__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x80Q\x83R` \x92\x83\x01\x92\x01aK V[_` \x82\x84\x03\x12\x15aKHW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK]W__\xFD[\x82\x01``\x81\x85\x03\x12\x15aKnW__\xFD[aKva@&V[aK\x7F\x82aJ\x02V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x99W__\xFD[aK\xA5\x86\x82\x85\x01aJ\x8EV[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xC3W__\xFD[\x91\x90\x91\x01\x90`@\x82\x86\x03\x12\x15aK\xD7W__\xFD[aK\xDFa@\x04V[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xF4W__\xFD[aL\0\x87\x82\x86\x01aJ\xDDV[\x82RP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x1BW__\xFD[aL'\x87\x82\x86\x01aJpV[` \x83\x01RP`@\x82\x01R\x94\x93PPPPV[\x80Q\x82R_` \x82\x01Q`@` \x85\x01RaG\xF4`@\x85\x01\x82aD0V[`\x01`\x01`@\x1B\x03\x84\x16\x81R``` \x82\x01R_aLy``\x83\x01\x85aL:V[\x82\x81\x03`@\x84\x01R\x83Q`@\x82RaL\x94`@\x83\x01\x82aC\"V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaL\xAD\x82\x82aD0V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aL\xDDW__\xFD[\x81Qa?v\x81a@\xDEV[_` \x82\x84\x03\x12\x15aL\xF8W__\xFD[\x81Q\x80\x15\x15\x81\x14a?vW__\xFD[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q``\x87RaMS``\x88\x01\x82aB\xDFV[\x90P` \x82\x01Q\x87\x82\x03` \x89\x01RaMl\x82\x82aC\"V[`@\x93\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x98\x90\x93\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aM-V[_` \x82\x84\x03\x12\x15aM\xA9W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xBEW__\xFD[aG\xF4\x84\x82\x85\x01aJ\xDDV[`\x01\x81\x81\x1C\x90\x82\x16\x80aM\xDEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aM\xFCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aN\x12W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN'W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aN7W__\xFD[aG\xF4\x84\x82Q` \x84\x01aJ\x18V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R`@\x82\x01\x86\x90R\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xC0`\xA0\x82\x01\x81\x90R_\x90aL\xAD\x90\x83\x01\x84aD0V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aN\xACWaN\xACaN\x87V[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R_\x83Q`@``\x84\x01RaN\xDC`\xA0\x84\x01\x82aD0V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aO\x15\x82\x85aN\xF3V[`\x17`\xF9\x1B\x81RaC\xD0`\x01\x82\x01\x85aN\xF3V[_`\xA0\x82\x84\x03\x12\x80\x15aO:W__\xFD[PaOCa@HV[\x82Q\x81R` \x83\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14aO\\W__\xFD[` \x82\x01RaOm`@\x84\x01aJ\x02V[`@\x82\x01R``\x83\x01Q\x80`\x07\x0B\x81\x14aO\x85W__\xFD[``\x82\x01RaO\x96`\x80\x84\x01aJ\x02V[`\x80\x82\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aO\xB2W__\xFD[a?v\x82aJ\x02V[_`@\x82\x01`@\x83R\x80\x85TaO\xD5\x81\x84\x90\x81R` \x01\x90V[_\x88\x81R` \x81 \x94P\x90\x92P[\x81`\x05\x82\x01\x10\x15aPJW\x83Td\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x85R`(\x82\x90\x1C\x81\x16` \x86\x01R`P\x82\x90\x1C\x81\x16`@\x86\x01R`x\x82\x90\x1C\x81\x16``\x86\x01R`\xA0\x82\x81\x1C\x82\x16`\x80\x87\x01R`\xC8\x92\x90\x92\x1C\x16\x90\x84\x01R`\x01\x90\x93\x01\x92`\xC0\x90\x92\x01\x91`\x06\x01aO\xE3V[\x92T\x92\x81\x81\x10\x15aPiWd\xFF\xFF\xFF\xFF\xFF\x84\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\x89Wd\xFF\xFF\xFF\xFF\xFF`(\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\xA9Wd\xFF\xFF\xFF\xFF\xFF`P\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\xC9Wd\xFF\xFF\xFF\xFF\xFF`x\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\xE9Wd\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aQ\x06W`\xC8\x84\x90\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x83\x01\x92P[PP`\x01`\x01`@\x1B\x03\x85\x16` \x85\x01R\x91Pa?v\x90PV[_` \x82\x84\x03\x12\x15aQ0W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQEW__\xFD[\x82\x01`@\x81\x85\x03\x12\x15aQVW__\xFD[aQ^a@\x04V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQsW__\xFD[aQ\x7F\x86\x82\x85\x01aJ\x8EV[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\x9AW__\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12aQ\xAEW__\xFD[\x81QaQ\xBCaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x87\x83\x11\x15aQ\xDDW__\xFD[` \x85\x01[\x83\x81\x10\x15aRiW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xFFW__\xFD[\x86\x01``\x81\x8B\x03`\x1F\x19\x01\x12\x15aR\x14W__\xFD[aR\x1Ca@&V[` \x82\x81\x01Q\x82R`@\x83\x01Q\x90\x82\x01R``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aREW__\xFD[aRT\x8C` \x83\x86\x01\x01aJpV[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01aQ\xE2V[P` \x84\x01RP\x90\x95\x94PPPPPV[`@\x81R_aR\x8C`@\x83\x01\x85aL:V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01_[\x83\x81\x10\x15aS\x02W`\x1F\x19\x86\x84\x03\x01\x85R\x81Q\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q\x90P```@\x85\x01RaR\xEB``\x85\x01\x82aD0V[` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01aR\xAFV[P\x90\x98\x97PPPPPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R_aC\xD0``\x83\x01\x84aD0V[``\x81R_aS@``\x83\x01\x86aCRV[\x82\x81\x03` \x84\x01RaSR\x81\x86aB\xDFV[\x91PP\x82\x15\x15`@\x83\x01R\x94\x93PPPPV[_aSp\x82\x86aN\xF3V[`\x17`\xF9\x1B\x81RaS\x84`\x01\x82\x01\x86aN\xF3V[\x90P`\x1D`\xF9\x1B\x81RaG\xB9`\x01\x82\x01\x85aN\xF3V[__`@\x83\x85\x03\x12\x15aS\xABW__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xC0W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aS\xD0W__\xFD[\x80QaS\xDEaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aS\xFFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aT*W\x83QaT\x19\x81a@\xDEV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aT\x06V[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTIW__\xFD[aB\xD5\x85\x82\x86\x01aJ\xDDV[\x80\x82\x01\x80\x82\x11\x15a\x0E\x8CWa\x0E\x8CaN\x87V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0E\x8CWa\x0E\x8CaN\x87V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aT\xA9WaT\xA9aT\x87V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\x8CWa\x0E\x8CaN\x87V[\x81\x81\x03\x81\x81\x11\x15a\x0E\x8CWa\x0E\x8CaN\x87V[_\x82aT\xE6WaT\xE6aT\x87V[P\x06\x90V[_` \x82\x84\x03\x12\x15aT\xFBW__\xFD[\x81Qa?v\x81a?IV[_\x82`\x1F\x83\x01\x12aU\x15W__\xFD[\x81QaU#aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aUDW__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aUfW__\xFD[aUu\x88` \x83\x8A\x01\x01aJ\xDDV[\x84RP` \x92\x83\x01\x92\x01aUIV[_` \x82\x84\x03\x12\x15aU\x94W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xA9W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15aU\xBAW__\xFD[aU\xC2a@jV[aU\xCB\x82aJ\x02V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xE5W__\xFD[aU\xF1\x86\x82\x85\x01aJ\x8EV[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x0FW__\xFD[\x82\x01`\x1F\x81\x01\x86\x13aV\x1FW__\xFD[\x80QaV-aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aVNW__\xFD[` \x84\x01[\x83\x81\x10\x15aV\x8EW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aVpW__\xFD[aV\x7F\x8B` \x83\x89\x01\x01aJpV[\x84RP` \x92\x83\x01\x92\x01aVSV[P`@\x85\x01RPPP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xAFW__\xFD[aV\xBB\x86\x82\x85\x01aU\x06V[``\x83\x01RP\x94\x93PPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aD\xACW`\x1F\x19\x85\x84\x03\x01\x88RaW\x01\x83\x83QaC\"V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aV\xE5V[`\x01`\x01`@\x1B\x03\x86\x16\x81R`\xA0` \x82\x01R_aW8`\xA0\x83\x01\x87aL:V[\x82\x81\x03`@\x84\x01RaWJ\x81\x87aIOV[\x90P\x82\x81\x03``\x84\x01R\x80\x85Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x88\x01_[\x83\x81\x10\x15aW\xA1W`\x1F\x19\x86\x84\x03\x01\x85RaW\x8B\x83\x83QaD0V[` \x95\x86\x01\x95\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aWoV[PP\x85\x81\x03`\x80\x87\x01RaW\xB5\x81\x88aV\xC9V[\x9B\x9APPPPPPPPPPPV[`@\x81R_aW\xD6`@\x83\x01\x85aD0V[\x90P\x82` \x83\x01R\x93\x92PPPV[_aC\xD0aW\xFCaW\xF6\x84\x88aN\xF3V[\x86aN\xF3V[\x84aN\xF3V\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP- depositing balance to beacon chain (gwei)- exiting all validators and completing checkpoint- submitting num checkpoint proofsUser.queueWithdrawals: length mismatch\xA2dipfsX\"\x12 .'Jb \xD7\xE1\xF2\x0E\xEA\xC1\xE5\xC4AB\x0Baf\n\xD3\x1D\x07\xFE\r\xB0H(?b\x12_\x98dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101e9575f3560e01c8063841c129911610108578063ac637c7a1161009d578063ba414fa61161006d578063ba414fa61461059a578063d6c10daf146105ae578063e20c9f71146105c2578063f234c1bd146105d6578063fa7626d4146105f8575f5ffd5b8063ac637c7a14610515578063b0464fdc14610534578063b5508aa914610548578063b71723551461055c575f5ffd5b806392ab89bb116100d857806392ab89bb146104885780639de702581461049c578063a3f4df7e146104bd578063a88dbb36146104de575f5ffd5b8063841c12991461041357806385226c811461043257806390b5162514610453578063916a17c614610467575f5ffd5b80633d8c08d41161017e57806346a5be0d1161014e57806346a5be0d1461039557806366d9a9a0146103b4578063695f4ae1146103d55780636d336f58146103f4575f5ffd5b80633d8c08d41461030a5780633e5e3c23146103415780633f7286f414610355578063401be65e14610369575f5ffd5b80632a34ade8116101b95780632a34ade81461028a5780632ade38801461029e578063344e1383146102bf578063391cc9f6146102eb575f5ffd5b8063071c25b7146101f45780631ed7831c1461021557806320a545d91461023f57806323e481751461025e575f5ffd5b366101f057005b5f5ffd5b3480156101ff575f5ffd5b5061021361020e366004613f5b565b610611565b005b348015610220575f5ffd5b506102296107d7565b6040516102369190613f7d565b60405180910390f35b34801561024a575f5ffd5b50610213610259366004614165565b610837565b348015610269575f5ffd5b5061027d61027836600461427c565b610b42565b60405161023691906143d9565b348015610295575f5ffd5b50610213610e92565b3480156102a9575f5ffd5b506102b2610ff9565b60405161023691906144b8565b3480156102ca575f5ffd5b506102de6102d9366004614609565b611135565b60405161023691906146b8565b3480156102f6575f5ffd5b5061027d61030536600461470f565b61128e565b348015610315575f5ffd5b5061032961032436600461472a565b611409565b6040516001600160401b039091168152602001610236565b34801561034c575f5ffd5b506102296114ae565b348015610360575f5ffd5b5061022961150c565b348015610374575f5ffd5b506103886103833660046147c3565b61156a565b60405161023691906147fc565b3480156103a0575f5ffd5b506102de6103af366004614609565b611621565b3480156103bf575f5ffd5b506103c8611770565b6040516102369190614848565b3480156103e0575f5ffd5b506103886103ef3660046147c3565b6118d4565b3480156103ff575f5ffd5b5061021361040e36600461427c565b61198a565b34801561041e575f5ffd5b5061021361042d36600461472a565b611edb565b34801561043d575f5ffd5b50610446611f9b565b60405161023691906148c6565b34801561045e575f5ffd5b50610213612066565b348015610472575f5ffd5b5061047b612116565b60405161023691906148d8565b348015610493575f5ffd5b5061027d6121f7565b3480156104a7575f5ffd5b506104b0612512565b6040516102369190614986565b3480156104c8575f5ffd5b506104d16126b4565b6040516102369190614998565b3480156104e9575f5ffd5b506026546104fd906001600160a01b031681565b6040516001600160a01b039091168152602001610236565b348015610520575f5ffd5b5061021361052f36600461470f565b61273b565b34801561053f575f5ffd5b5061047b612888565b348015610553575f5ffd5b50610446612969565b348015610567575f5ffd5b5061058a6105763660046149aa565b60296020525f908152604090205460ff1681565b6040519015158152602001610236565b3480156105a5575f5ffd5b5061058a612a34565b3480156105b9575f5ffd5b50610213612ad4565b3480156105cd575f5ffd5b50610229612b85565b3480156105e1575f5ffd5b506105ea612be3565b6040516102369291906149c1565b348015610603575f5ffd5b50601f5461058a9060ff1681565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610662573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061068691906149eb565b506106ba604051806040016040528060128152602001717665726966795374616c6542616c616e636560701b815250612c9c565b602480546040516308fa0b1360e21b815264ffffffffff841660048201525f926001600160a01b03909216916323e82c4c91015f60405180830381865afa158015610707573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261072e9190810190614b38565b6026548151602083015160408085015190516301c8abe960e11b81529495506001600160a01b039093169363039157d29361076d939291600401614c58565b5f604051808303815f87803b158015610784575f5ffd5b505af1925050508015610795575060015b6107d3573d8080156107c2576040519150601f19603f3d011682016040523d82523d5f602084013e6107c7565b606091505b506107d181612cf9565b505b5050565b6060601680548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f20905b81546001600160a01b0316815260019091019060200180831161080f575b5050505050905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610888573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108ac91906149eb565b506108dc6040518060400160405280600e81526020016d75706461746542616c616e63657360901b815250612c9c565b5f5b82518110156107d1575f8382815181106108fa576108fa614cb9565b602002602001015190505f83838151811061091757610917614cb9565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b0316036109dc57610956612d55565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109ca91906149eb565b156109d7576109d7612de4565b610b38565b5f8190505f836001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a1d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a419190614ccd565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015610a93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ab79190614ce8565b506021546040516373d0285560e11b81526001600160a01b0386811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303815f875af1158015610b10573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b3491906149eb565b5050505b50506001016108de565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015610b8b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610baf91906149eb565b50610be16040518060400160405280601081526020016f71756575655769746864726177616c7360801b815250612c9c565b602054604051631976849960e21b81523060048201525f916001600160a01b0316906365da126490602401602060405180830381865afa158015610c27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c4b9190614ccd565b60205460405163285e212160e21b815230600482018190529293505f916001600160a01b03169063a178848490602401602060405180830381865afa158015610c96573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cba91906149eb565b6040805160018082528183019092529192505f9190816020015b604080516060808201835280825260208201525f91810191909152815260200190600190039081610cd45790505090506040518060600160405280888152602001878152602001846001600160a01b0316815250815f81518110610d3a57610d3a614cb9565b60209081029190910101526040805160018082528183019092525f91816020015b610d63613ef0565b815260200190600190039081610d5b5790505090506040518060e00160405280306001600160a01b03168152602001866001600160a01b03168152602001856001600160a01b031681526020018481526020014363ffffffff16815260200189815260200188815250815f81518110610dde57610dde614cb9565b602090810291909101810191909152546040516306ec6e8160e11b81525f916001600160a01b031690630dd8dd0290610e1b908690600401614d07565b5f604051808303815f875af1158015610e36573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610e5d9190810190614d99565b9050610e84825182516040518060600160405280602681526020016158a2602691396130de565b509450505050505b92915050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610ee3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f0791906149eb565b50610f3b604051806040016040528060128152602001713932b3b4b9ba32b920b9a7b832b930ba37b960711b815250612c9c565b604080516060810182523081525f60208083018281528385019283529054602854945163024b980360e51b815284516001600160a01b039081166004830152925183166024820152925163ffffffff9081166044850152909416606483015260a06084830152600860a4830152676d6574616461746160c01b60c48301529192919091169063497300609060e4015f604051808303815f87803b158015610fe0575f5ffd5b505af1158015610ff2573d5f5f3e3d5ffd5b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b8282101561112c575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015611115578382905f5260205f2001805461108a90614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546110b690614dca565b80156111015780601f106110d857610100808354040283529160200191611101565b820191905f5260205f20905b8154815290600101906020018083116110e457829003601f168201915b50505050508152602001906001019061106d565b50505050815250508152602001906001019061101c565b50505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561117e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111a291906149eb565b506111e16040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612c9c565b5f82516001600160401b038111156111fb576111fb613fc8565b60405190808252806020026020018201604052801561122e57816020015b60608152602001906001900390816112195790505b5090505f5b83518110156112855761126084828151811061125157611251614cb9565b6020026020010151600161314a565b82828151811061127257611272614cb9565b6020908102919091010152600101611233565b5090505b919050565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af11580156112d7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112fb91906149eb565b5061138e6040518060400160405280600f81526020016e666f726365556e64656c656761746560881b815250836001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611362573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526113899190810190614e02565b613464565b5f611398836134bf565b6020546040516336a2fa1960e21b81526001600160a01b03868116600483015292935091169063da8be864906024015f604051808303815f875af11580156113e2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526112859190810190614d99565b602354604080516301504d8f60e41b815290515f926001600160a01b031691631504d8f0916004808301926020929190829003018187875af1158015611451573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061147591906149eb565b506114a56040518060400160405280600e81526020016d6578697456616c696461746f727360901b815250612c9c565b610e8c826137d4565b6060601880548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161080f575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161080f575050505050905090565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af11580156115b3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115d791906149eb565b506116166040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612c9c565b610e8c82600161314a565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561166a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061168e91906149eb565b506116cd6040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612c9c565b5f82516001600160401b038111156116e7576116e7613fc8565b60405190808252806020026020018201604052801561171a57816020015b60608152602001906001900390816117055790505b5090505f5b83518110156112855761174b84828151811061173d5761173d614cb9565b60200260200101515f61314a565b82828151811061175d5761175d614cb9565b602090810291909101015260010161171f565b6060601b805480602002602001604051908101604052809291908181526020015f905b8282101561112c578382905f5260205f2090600202016040518060400160405290815f820180546117c390614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546117ef90614dca565b801561183a5780601f106118115761010080835404028352916020019161183a565b820191905f5260205f20905b81548152906001019060200180831161181d57829003601f168201915b50505050508152602001600182018054806020026020016040519081016040528092919081815260200182805480156118bc57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161187e5790505b50505050508152505081526020019060010190611793565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af115801561191d573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061194191906149eb565b506119806040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612c9c565b610e8c825f61314a565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156119db573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119ff91906149eb565b50611a3e6040518060400160405280601981526020017f6465706f736974496e746f456967656e6c617965725f414c5400000000000000815250612c9c565b5f195f5b8351811015611ed5575f848281518110611a5e57611a5e614cb9565b602002602001015190505f848381518110611a7b57611a7b614cb9565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603611b2f575f611abb613917565b50905060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611b0a575f5ffd5b505af1158015611b1c573d5f5f3e3d5ffd5b50505050611b2981613d0c565b50611ecb565b5f826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b6c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b909190614ccd565b60215460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303815f875af1158015611be2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c069190614ce8565b50602154604051623f675f60e91b81523060048201525f916001600160a01b031690637ecebe0090602401602060405180830381865afa158015611c4c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c7091906149eb565b90505f60215f9054906101000a90046001600160a01b03166001600160a01b03166348825e946040518163ffffffff1660e01b8152600401602060405180830381865afa158015611cc3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ce791906149eb565b60408051602081019290925230908201526001600160a01b0380871660608301528416608082015260a0810185905260c0810183905260e081018890526101000160408051601f19818403018152828252805160209182012060215463f698da2560e01b855292519094505f936001600160a01b039093169263f698da259260048083019391928290030181865afa158015611d85573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611da991906149eb565b60405161190160f01b602082015260228101919091526042810183905260620160408051601f19818403018152828252805160209182012090830181905292505f91016040516020818303038152906040529050600160295f8481526020019081526020015f205f6101000a81548160ff02191690831515021790555060215f9054906101000a90046001600160a01b03166001600160a01b03166332e89ace888789308e876040518763ffffffff1660e01b8152600401611e7096959493929190614e46565b6020604051808303815f875af1158015611e8c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611eb091906149eb565b50505f908152602960205260409020805460ff191690555050505b5050600101611a42565b50505050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611f2c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5091906149eb565b50611f8f6040518060400160405280601b81526020017f7665726966795769746864726177616c43726564656e7469616c730000000000815250612c9c565b611f9881613d0c565b50565b6060601a805480602002602001604051908101604052809291908181526020015f905b8282101561112c578382905f5260205f20018054611fdb90614dca565b80601f016020809104026020016040519081016040528092919081815260200182805461200790614dca565b80156120525780601f1061202957610100808354040283529160200191612052565b820191905f5260205f20905b81548152906001019060200180831161203557829003601f168201915b505050505081526020019060010190611fbe565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af11580156120b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120db91906149eb565b5061210c6040518060400160405280600f81526020016e1cdd185c9d10da1958dadc1bda5b9d608a1b815250612c9c565b612114612d55565b565b6060601d805480602002602001604051908101604052809291908181526020015f905b8282101561112c575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156121df57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116121a15790505b50505050508152505081526020019060010190612139565b602354604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301815f875af1158015612240573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061226491906149eb565b506122906040518060400160405280600a815260200169756e64656c656761746560b01b815250612c9c565b5f61229a306134bf565b6020546040516336a2fa1960e21b81523060048201529192506001600160a01b03169063da8be864906024015f604051808303815f875af11580156122e1573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526123089190810190614d99565b505f5b815181101561250c575f5160206158035f395f51905f526040516123589060208082526015908201527432bc3832b1ba34b733903bb4ba34323930bbb0b61d60591b604082015260600190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882828151811061239357612393614cb9565b6020026020010151606001516040516123d0919060408082526007908201526603737b731b29d160cd1b6060820152602081019190915260800190565b60405180910390a17f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f82828151811061240b5761240b614cb9565b602002602001015160a001515f8151811061242857612428614cb9565b602002602001015160405161246a9190604080825260079082015266039ba3930ba1d160cd1b60608201526001600160a01b0391909116602082015260800190565b60405180910390a17fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a88282815181106124a5576124a5614cb9565b602002602001015160c001515f815181106124c2576124c2614cb9565b60200260200101516040516124fc9190604080825260089082015267039b430b932b99d160c51b6060820152602081019190915260800190565b60405180910390a160010161230b565b50905090565b6027546060905f906001600160401b0381111561253157612531613fc8565b60405190808252806020026020018201604052801561255a578160200160208202803683370190505b5090505f80805b6027548110156126ab57602454602780546001600160a01b039092169163aa47389c91908490811061259557612595614cb9565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff166040518263ffffffff1660e01b81526004016125e3919064ffffffffff91909116815260200190565b602060405180830381865afa1580156125fe573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126229190614ce8565b156126a3576027818154811061263a5761263a614cb9565b905f5260205f2090600691828204019190066005029054906101000a900464ffffffffff1684838151811061267157612671614cb9565b64ffffffffff909216602092830291909101909101528261269181614e9b565b935050818061269f90614e9b565b9250505b600101612561565b50508152919050565b6060602580546126c390614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546126ef90614dca565b801561082d5780601f106127115761010080835404028352916020019161082d565b820191905f5260205f20905b81548152906001019060200180831161271d57509395945050505050565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af115801561278c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127b091906149eb565b506128126040518060400160405280600a81526020016964656c6567617465546f60b01b815250826001600160a01b031663a3f4df7e6040518163ffffffff1660e01b81526004015f60405180830381865afa158015611362573d5f5f3e3d5ffd5b604080518082018252606081525f602080830182905254925163eea9064b60e01b815291926001600160a01b03169163eea9064b916128579186918691600401614eb3565b5f604051808303815f87803b15801561286e575f5ffd5b505af1158015612880573d5f5f3e3d5ffd5b505050505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b8282101561112c575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561295157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116129135790505b505050505081525050815260200190600101906128ab565b60606019805480602002602001604051908101604052809291908181526020015f905b8282101561112c578382905f5260205f200180546129a990614dca565b80601f01602080910402602001604051908101604052809291908181526020018280546129d590614dca565b8015612a205780601f106129f757610100808354040283529160200191612a20565b820191905f5260205f20905b815481529060010190602001808311612a0357829003601f168201915b50505050508152602001906001019061298c565b6008545f9060ff1615612a4b575060085460ff1690565b604051630667f9d760e41b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa158015612aa9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612acd91906149eb565b1415905090565b60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612b25573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b4991906149eb565b50612b7d6040518060400160405280601281526020017118dbdb5c1b195d1950da1958dadc1bda5b9d60721b815250612c9c565b612114612de4565b6060601580548060200260200160405190810160405280929190818152602001828054801561082d57602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161080f575050505050905090565b60605f60235f9054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612c37573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c5b91906149eb565b50612c8c6040518060400160405280600f81526020016e737461727456616c696461746f727360881b815250612c9c565b612c94613917565b915091509091565b5f5160206158035f395f51905f52612cba612cb56126b4565b613dc3565b612cc383613dec565b604051602001612cd4929190614f0a565b60408051601f1981840301815290829052612cee91614998565b60405180910390a150565b805115612d0857805181602001fd5b60405162461bcd60e51b815260206004820152601b60248201527f7265766572746564207769746820756e6b6e6f776e206572726f72000000000060448201526064015b60405180910390fd5b6026546040516388676cad60e01b81525f60048201526001600160a01b03909116906388676cad906024015f604051808303815f87803b158015612d97575f5ffd5b505af1925050508015612da8575060015b612114573d808015612dd5576040519150601f19603f3d011682016040523d82523d5f602084013e612dda565b606091505b50611f9881612cf9565b604080518082018252601881527f2d206163746976652076616c696461746f7220636f756e7400000000000000006020808301919091526026548351632340e8d360e01b81529351612e89946001600160a01b0390921692632340e8d392600480820193918290030181865afa158015612e60573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e8491906149eb565b613e14565b60408051808201825260128152712d2070726f6f66732072656d61696e696e6760701b602082015260265482516323e941b960e11b81529251612f2c936001600160a01b03909216916347d283729160048083019260a09291908290030181865afa158015612efa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f1e9190614f29565b6020015162ffffff16613e14565b602654604080516321767f9560e11b815290515f926001600160a01b0316916342ecff2a9160048083019260209291908290030181865afa158015612f73573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f979190614fa2565b9050806001600160401b03165f0361300a5760405162461bcd60e51b815260206004820152603060248201527f557365722e5f636f6d706c657465436865636b706f696e743a206e6f2065786960448201526f1cdd1a5b99c818da1958dadc1bda5b9d60821b6064820152608401612d4c565b60245460405163b1b6f6a160e01b81525f916001600160a01b03169063b1b6f6a19061303d906027908690600401614fbb565b5f60405180830381865afa158015613057573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261307e9190810190615120565b90506130a760405180606001604052806022815260200161588060229139826020015151613e14565b6026548151602083015160405163783a5d3160e11b81526001600160a01b039093169263f074ba629261285792909160040161527a565b6040516388b44c8560e01b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d906388b44c859061311990869086908690600401615310565b5f6040518083038186803b15801561312f575f5ffd5b505afa158015613141573d5f5f3e3d5ffd5b50505050505050565b60605f8360a00151516001600160401b0381111561316a5761316a613fc8565b604051908082528060200260200182016040528015613193578160200160208202803683370190505b5090505f5b81518110156133fa575f8560a0015182815181106131b8576131b8614cb9565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316816001600160a01b03160361335e5773beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac083838151811061321657613216614cb9565b60200260200101906001600160a01b031690816001600160a01b03168152505084156133595761325d60405180606001604052806032815260200161584e60329139613e45565b61326d613268612512565b6137d4565b5060245f9054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156132ba575f5ffd5b505af11580156132cc573d5f5f3e3d5ffd5b505050506132d8612d55565b60265f9054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa158015613328573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061334c91906149eb565b1561335957613359612de4565b6133f1565b806001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa15801561339a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133be9190614ccd565b8383815181106133d0576133d0614cb9565b60200260200101906001600160a01b031690816001600160a01b0316815250505b50600101613198565b50602054604051630e4cc3f960e41b81526001600160a01b039091169063e4cc3f909061342f9087908590889060040161532e565b5f604051808303815f87803b158015613446575f5ffd5b505af1158015613458573d5f5f3e3d5ffd5b50929695505050505050565b5f5160206158035f395f51905f5261347d612cb56126b4565b61348684613dec565b8360405160200161349993929190615365565b60408051601f19818403018152908290526134b391614998565b60405180910390a15050565b6020546040516366d5ba9360e01b81526001600160a01b0383811660048301526060925f928392909116906366d5ba93906024015f60405180830381865afa15801561350d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613534919081019061539a565b915091505f82516001600160401b0381111561355257613552613fc8565b60405190808252806020026020018201604052801561358b57816020015b613578613ef0565b8152602001906001900390816135705790505b50602054604051631976849960e21b81526001600160a01b0388811660048301529293505f92909116906365da126490602401602060405180830381865afa1580156135d9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135fd9190614ccd565b60205460405163285e212160e21b81526001600160a01b0389811660048301529293505f929091169063a178848490602401602060405180830381865afa15801561364a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061366e91906149eb565b90505f5b85518110156137c8576040805160018082528183019092525f916020808301908036833750506040805160018082528183019092529293505f929150602080830190803683370190505090508783815181106136d0576136d0614cb9565b6020026020010151825f815181106136ea576136ea614cb9565b60200260200101906001600160a01b031690816001600160a01b03168152505086838151811061371c5761371c614cb9565b6020026020010151815f8151811061373657613736614cb9565b6020026020010181815250506040518060e001604052808b6001600160a01b03168152602001866001600160a01b031681526020018b6001600160a01b0316815260200184866137869190615455565b81526020014363ffffffff168152602001838152602001828152508684815181106137b3576137b3614cb9565b60209081029190910101525050600101613672565b50919695505050505050565b5f6138156040518060400160405280601881526020017f2d2065786974696e67206e756d2076616c696461746f727300000000000000008152508351613e14565b5f5b82518110156138ce5760245483516001600160a01b039091169063f8f98a4e9085908490811061384957613849614cb9565b60200260200101516040518263ffffffff1660e01b815260040161387a919064ffffffffff91909116815260200190565b6020604051808303815f875af1158015613896573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906138ba9190614fa2565b6138c49083615468565b9150600101613817565b506112896040518060400160405280601e81526020017f2d206578697465642062616c616e636520746f20706f64202867776569290000815250826001600160401b0316613e14565b60605f478161392f6801bc16d674ec8000008361549b565b9050613944816801bc16d674ec8000006154ae565b61394e90836154c5565b91505f81670de0b6b3a764000084106139955761396f633b9aca00856154d8565b61397990856154c5565b915061398582856154c5565b93508061399181614e9b565b9150505b805f03613a015760405162461bcd60e51b815260206004820152603460248201527f737461727456616c696461746f72733a206e6f7420656e6f75676820455448206044820152733a379039ba30b93a1030903b30b634b230ba37b960611b6064820152608401612d4c565b5f816001600160401b03811115613a1a57613a1a613fc8565b604051908082528060200260200182016040528015613a43578160200160208202803683370190505b5090505f633b9aca00613a5687476154c5565b613a60919061549b565b9050613aa26040518060400160405280601981526020017f2d206372656174696e67206e65772076616c696461746f7273000000000000008152508351613e14565b613acd6040518060600160405280602b8152602001615823602b9139826001600160401b0316613e14565b5f5b85811015613be3576024545f906001600160a01b031663ed3c16056801bc16d674ec800000613afc613e61565b6040518363ffffffff1660e01b8152600401613b189190614998565b60206040518083038185885af1158015613b34573d5f5f3e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190613b5991906154eb565b905080848381518110613b6e57613b6e614cb9565b64ffffffffff928316602091820292909201015260278054600181810183555f9290925260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054958516600592909306919091026101000a918202919093021990931692909217905501613acf565b50613bef856001615455565b8303613cff576024545f906001600160a01b031663ed3c160586613c11613e61565b6040518363ffffffff1660e01b8152600401613c2d9190614998565b60206040518083038185885af1158015613c49573d5f5f3e3d5ffd5b50505050506040513d601f19601f82011682018060405250810190613c6e91906154eb565b9050808360018551613c8091906154c5565b81518110613c9057613c90614cb9565b64ffffffffff9283166020918202929092010152602780546001810182555f9190915260068082047f98a476f1687bc3d60a2da2adbcba2c46958e61fa2fb4042cd7bc5816a710195b018054948416600592909306919091026101000a91820291909202199092169190911790555b9097909650945050505050565b6024546040516352851d0d60e11b81525f916001600160a01b03169063a50a3a1a90613d3c908590600401614986565b5f60405180830381865afa158015613d56573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613d7d9190810190615584565b6026548151602083015160408085015160608601519151633f65cf1960e01b81529596506001600160a01b0390941694633f65cf199461076d9493928992600401615717565b6060610e8c604051806040016040528060058152602001641b5b39366d60d81b81525083613ea6565b6060610e8c604051806040016040528060048152602001631b5b336d60e01b81525083613ea6565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882826040516134b39291906157c4565b5f5160206158035f395f51905f5281604051612cee9190614998565b60265460408051600160f81b60208201525f60218201526bffffffffffffffffffffffff19606093841b16602c82015201604051602081830303815290604052905090565b60608282604051806040016040528060048152602001631b5b306d60e01b815250604051602001613ed9939291906157e5565b604051602081830303815290604052905092915050565b6040518060e001604052805f6001600160a01b031681526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f81526020015f63ffffffff16815260200160608152602001606081525090565b64ffffffffff81168114611f98575f5ffd5b5f60208284031215613f6b575f5ffd5b8135613f7681613f49565b9392505050565b602080825282518282018190525f918401906040840190835b81811015613fbd5783516001600160a01b0316835260209384019390920191600101613f96565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b60405160e081016001600160401b0381118282101715613ffe57613ffe613fc8565b60405290565b604080519081016001600160401b0381118282101715613ffe57613ffe613fc8565b604051606081016001600160401b0381118282101715613ffe57613ffe613fc8565b60405160a081016001600160401b0381118282101715613ffe57613ffe613fc8565b604051608081016001600160401b0381118282101715613ffe57613ffe613fc8565b604051601f8201601f191681016001600160401b03811182821017156140b4576140b4613fc8565b604052919050565b5f6001600160401b038211156140d4576140d4613fc8565b5060051b60200190565b6001600160a01b0381168114611f98575f5ffd5b5f82601f830112614101575f5ffd5b813561411461410f826140bc565b61408c565b8082825260208201915060208360051b860101925085831115614135575f5ffd5b602085015b8381101561415b57803561414d816140de565b83526020928301920161413a565b5095945050505050565b5f5f60408385031215614176575f5ffd5b82356001600160401b0381111561418b575f5ffd5b614197858286016140f2565b92505060208301356001600160401b038111156141b2575f5ffd5b8301601f810185136141c2575f5ffd5b80356141d061410f826140bc565b8082825260208201915060208360051b8501019250878311156141f1575f5ffd5b6020840193505b828410156142135783358252602093840193909101906141f8565b809450505050509250929050565b5f82601f830112614230575f5ffd5b813561423e61410f826140bc565b8082825260208201915060208360051b86010192508583111561425f575f5ffd5b602085015b8381101561415b578035835260209283019201614264565b5f5f6040838503121561428d575f5ffd5b82356001600160401b038111156142a2575f5ffd5b6142ae858286016140f2565b92505060208301356001600160401b038111156142c9575f5ffd5b6142d585828601614221565b9150509250929050565b5f8151808452602084019350602083015f5b828110156143185781516001600160a01b03168652602095860195909101906001016142f1565b5093949350505050565b5f8151808452602084019350602083015f5b82811015614318578151865260209586019590910190600101614334565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201515f9161439d9085018263ffffffff169052565b5060a082015160e060a08501526143b760e08501826142df565b905060c083015184820360c08601526143d08282614322565b95945050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f1987860301845261441b858351614352565b945060209384019391909101906001016143ff565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b838110156144ac57601f19858403018852614496838351614430565b602098890198909350919091019060010161447a565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857868503603f19018452815180516001600160a01b031686526020908101516040918701829052906145199087018261445e565b95505060209384019391909101906001016144de565b8035611289816140de565b803563ffffffff81168114611289575f5ffd5b5f60e0828403121561455d575f5ffd5b614565613fdc565b90506145708261452f565b815261457e6020830161452f565b602082015261458f6040830161452f565b6040820152606082810135908201526145aa6080830161453a565b608082015260a08201356001600160401b038111156145c7575f5ffd5b6145d3848285016140f2565b60a08301525060c08201356001600160401b038111156145f1575f5ffd5b6145fd84828501614221565b60c08301525092915050565b5f60208284031215614619575f5ffd5b81356001600160401b0381111561462e575f5ffd5b8201601f8101841361463e575f5ffd5b803561464c61410f826140bc565b8082825260208201915060208360051b85010192508683111561466d575f5ffd5b602084015b838110156146ad5780356001600160401b0381111561468f575f5ffd5b61469e8960208389010161454d565b84525060209283019201614672565b509695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f198786030184526146fa8583516142df565b945060209384019391909101906001016146de565b5f6020828403121561471f575f5ffd5b8135613f76816140de565b5f6020828403121561473a575f5ffd5b81356001600160401b0381111561474f575f5ffd5b8201601f8101841361475f575f5ffd5b803561476d61410f826140bc565b8082825260208201915060208360051b85010192508683111561478e575f5ffd5b6020840193505b828410156147b95783356147a881613f49565b825260209384019390910190614795565b9695505050505050565b5f602082840312156147d3575f5ffd5b81356001600160401b038111156147e8575f5ffd5b6147f48482850161454d565b949350505050565b602081525f613f7660208301846142df565b5f8151808452602084019350602083015f5b828110156143185781516001600160e01b031916865260209586019590910190600101614820565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f1987860301845281518051604087526148946040880182614430565b90506020820151915086810360208801526148af818361480e565b96505050602093840193919091019060010161486e565b602081525f613f76602083018461445e565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857868503603f19018452815180516001600160a01b031686526020908101516040918701829052906149399087018261480e565b95505060209384019391909101906001016148fe565b5f8151808452602084019350602083015f5b8281101561431857815164ffffffffff16865260209586019590910190600101614961565b602081525f613f76602083018461494f565b602081525f613f766020830184614430565b5f602082840312156149ba575f5ffd5b5035919050565b604081525f6149d3604083018561494f565b90506001600160401b03831660208301529392505050565b5f602082840312156149fb575f5ffd5b5051919050565b80516001600160401b0381168114611289575f5ffd5b5f5f6001600160401b03841115614a3157614a31613fc8565b50601f8301601f1916602001614a468161408c565b915050828152838383011115614a5a575f5ffd5b8282602083015e5f602084830101529392505050565b5f82601f830112614a7f575f5ffd5b613f7683835160208501614a18565b5f60408284031215614a9e575f5ffd5b614aa6614004565b8251815260208301519091506001600160401b03811115614ac5575f5ffd5b614ad184828501614a70565b60208301525092915050565b5f82601f830112614aec575f5ffd5b8151614afa61410f826140bc565b8082825260208201915060208360051b860101925085831115614b1b575f5ffd5b602085015b8381101561415b578051835260209283019201614b20565b5f60208284031215614b48575f5ffd5b81516001600160401b03811115614b5d575f5ffd5b820160608185031215614b6e575f5ffd5b614b76614026565b614b7f82614a02565b815260208201516001600160401b03811115614b99575f5ffd5b614ba586828501614a8e565b60208301525060408201516001600160401b03811115614bc3575f5ffd5b919091019060408286031215614bd7575f5ffd5b614bdf614004565b82516001600160401b03811115614bf4575f5ffd5b614c0087828601614add565b82525060208301516001600160401b03811115614c1b575f5ffd5b614c2787828601614a70565b6020830152506040820152949350505050565b805182525f6020820151604060208501526147f46040850182614430565b6001600160401b0384168152606060208201525f614c796060830185614c3a565b8281036040840152835160408252614c946040830182614322565b905060208501518282036020840152614cad8282614430565b98975050505050505050565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614cdd575f5ffd5b8151613f76816140de565b5f60208284031215614cf8575f5ffd5b81518015158114613f76575f5ffd5b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561345857603f198786030184528151805160608752614d5360608801826142df565b905060208201518782036020890152614d6c8282614322565b6040938401516001600160a01b031698909301979097525094506020938401939190910190600101614d2d565b5f60208284031215614da9575f5ffd5b81516001600160401b03811115614dbe575f5ffd5b6147f484828501614add565b600181811c90821680614dde57607f821691505b602082108103614dfc57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f60208284031215614e12575f5ffd5b81516001600160401b03811115614e27575f5ffd5b8201601f81018413614e37575f5ffd5b6147f484825160208401614a18565b6001600160a01b038781168252868116602083015260408201869052841660608201526080810183905260c060a082018190525f90614cad90830184614430565b634e487b7160e01b5f52601160045260245ffd5b5f60018201614eac57614eac614e87565b5060010190565b60018060a01b0384168152606060208201525f835160406060840152614edc60a0840182614430565b602095909501516080840152505060400152919050565b5f81518060208401855e5f93019283525090919050565b5f614f158285614ef3565b601760f91b81526143d06001820185614ef3565b5f60a0828403128015614f3a575f5ffd5b50614f43614048565b82518152602083015162ffffff81168114614f5c575f5ffd5b6020820152614f6d60408401614a02565b604082015260608301518060070b8114614f85575f5ffd5b6060820152614f9660808401614a02565b60808201529392505050565b5f60208284031215614fb2575f5ffd5b613f7682614a02565b5f6040820160408352808554614fd5818490815260200190565b5f8881526020812094509092505b8160058201101561504a57835464ffffffffff8082168552602882901c81166020860152605082901c81166040860152607882901c8116606086015260a082811c8216608087015260c89290921c169084015260019093019260c090920191600601614fe3565b925492818110156150695764ffffffffff841683526020909201916001015b818110156150895764ffffffffff602885901c1683526020909201916001015b818110156150a95764ffffffffff605085901c1683526020909201916001015b818110156150c95764ffffffffff607885901c1683526020909201916001015b818110156150e95764ffffffffff60a085901c1683526020909201916001015b818110156151065760c884901c64ffffffffff1683526020830192505b50506001600160401b03851660208501529150613f769050565b5f60208284031215615130575f5ffd5b81516001600160401b03811115615145575f5ffd5b820160408185031215615156575f5ffd5b61515e614004565b81516001600160401b03811115615173575f5ffd5b61517f86828501614a8e565b82525060208201516001600160401b0381111561519a575f5ffd5b80830192505084601f8301126151ae575f5ffd5b81516151bc61410f826140bc565b8082825260208201915060208360051b8601019250878311156151dd575f5ffd5b602085015b838110156152695780516001600160401b038111156151ff575f5ffd5b86016060818b03601f19011215615214575f5ffd5b61521c614026565b602082810151825260408301519082015260608201516001600160401b03811115615245575f5ffd5b6152548c602083860101614a70565b604083015250845250602092830192016151e2565b506020840152509095945050505050565b604081525f61528c6040830185614c3a565b828103602084015280845180835260208301915060208160051b840101602087015f5b8381101561530257601f198684030185528151805184526020810151602085015260408101519050606060408501526152eb6060850182614430565b6020968701969094509290920191506001016152af565b509098975050505050505050565b838152826020820152606060408201525f6143d06060830184614430565b606081525f6153406060830186614352565b828103602084015261535281866142df565b9150508215156040830152949350505050565b5f6153708286614ef3565b601760f91b81526153846001820186614ef3565b9050601d60f91b81526147b96001820185614ef3565b5f5f604083850312156153ab575f5ffd5b82516001600160401b038111156153c0575f5ffd5b8301601f810185136153d0575f5ffd5b80516153de61410f826140bc565b8082825260208201915060208360051b8501019250878311156153ff575f5ffd5b6020840193505b8284101561542a578351615419816140de565b825260209384019390910190615406565b8095505050505060208301516001600160401b03811115615449575f5ffd5b6142d585828601614add565b80820180821115610e8c57610e8c614e87565b6001600160401b038181168382160190811115610e8c57610e8c614e87565b634e487b7160e01b5f52601260045260245ffd5b5f826154a9576154a9615487565b500490565b8082028115828204841417610e8c57610e8c614e87565b81810381811115610e8c57610e8c614e87565b5f826154e6576154e6615487565b500690565b5f602082840312156154fb575f5ffd5b8151613f7681613f49565b5f82601f830112615515575f5ffd5b815161552361410f826140bc565b8082825260208201915060208360051b860101925085831115615544575f5ffd5b602085015b8381101561415b5780516001600160401b03811115615566575f5ffd5b615575886020838a0101614add565b84525060209283019201615549565b5f60208284031215615594575f5ffd5b81516001600160401b038111156155a9575f5ffd5b8201608081850312156155ba575f5ffd5b6155c261406a565b6155cb82614a02565b815260208201516001600160401b038111156155e5575f5ffd5b6155f186828501614a8e565b60208301525060408201516001600160401b0381111561560f575f5ffd5b8201601f8101861361561f575f5ffd5b805161562d61410f826140bc565b8082825260208201915060208360051b85010192508883111561564e575f5ffd5b602084015b8381101561568e5780516001600160401b03811115615670575f5ffd5b61567f8b602083890101614a70565b84525060209283019201615653565b50604085015250505060608201516001600160401b038111156156af575f5ffd5b6156bb86828501615506565b606083015250949350505050565b5f82825180855260208501945060208160051b830101602085015f5b838110156144ac57601f19858403018852615701838351614322565b60209889019890935091909101906001016156e5565b6001600160401b038616815260a060208201525f61573860a0830187614c3a565b828103604084015261574a818761494f565b9050828103606084015280855180835260208301915060208160051b840101602088015f5b838110156157a157601f1986840301855261578b838351614430565b602095860195909350919091019060010161576f565b505085810360808701526157b581886156c9565b9b9a5050505050505050505050565b604081525f6157d66040830185614430565b90508260208301529392505050565b5f6143d06157fc6157f68488614ef3565b86614ef3565b84614ef356fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f502d206465706f736974696e672062616c616e636520746f20626561636f6e20636861696e202867776569292d2065786974696e6720616c6c2076616c696461746f727320616e6420636f6d706c6574696e6720636865636b706f696e742d207375626d697474696e67206e756d20636865636b706f696e742070726f6f6673557365722e71756575655769746864726177616c733a206c656e677468206d69736d61746368a26469706673582212202e274a6220d7e1f20eeac1e5c441420b61660ad31d07fe0db048283f62125f9864736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xE9W_5`\xE0\x1C\x80c\x84\x1C\x12\x99\x11a\x01\x08W\x80c\xACc|z\x11a\0\x9DW\x80c\xBAAO\xA6\x11a\0mW\x80c\xBAAO\xA6\x14a\x05\x9AW\x80c\xD6\xC1\r\xAF\x14a\x05\xAEW\x80c\xE2\x0C\x9Fq\x14a\x05\xC2W\x80c\xF24\xC1\xBD\x14a\x05\xD6W\x80c\xFAv&\xD4\x14a\x05\xF8W__\xFD[\x80c\xACc|z\x14a\x05\x15W\x80c\xB0FO\xDC\x14a\x054W\x80c\xB5P\x8A\xA9\x14a\x05HW\x80c\xB7\x17#U\x14a\x05\\W__\xFD[\x80c\x92\xAB\x89\xBB\x11a\0\xD8W\x80c\x92\xAB\x89\xBB\x14a\x04\x88W\x80c\x9D\xE7\x02X\x14a\x04\x9CW\x80c\xA3\xF4\xDF~\x14a\x04\xBDW\x80c\xA8\x8D\xBB6\x14a\x04\xDEW__\xFD[\x80c\x84\x1C\x12\x99\x14a\x04\x13W\x80c\x85\"l\x81\x14a\x042W\x80c\x90\xB5\x16%\x14a\x04SW\x80c\x91j\x17\xC6\x14a\x04gW__\xFD[\x80c=\x8C\x08\xD4\x11a\x01~W\x80cF\xA5\xBE\r\x11a\x01NW\x80cF\xA5\xBE\r\x14a\x03\x95W\x80cf\xD9\xA9\xA0\x14a\x03\xB4W\x80ci_J\xE1\x14a\x03\xD5W\x80cm3oX\x14a\x03\xF4W__\xFD[\x80c=\x8C\x08\xD4\x14a\x03\nW\x80c>^<#\x14a\x03AW\x80c?r\x86\xF4\x14a\x03UW\x80c@\x1B\xE6^\x14a\x03iW__\xFD[\x80c*4\xAD\xE8\x11a\x01\xB9W\x80c*4\xAD\xE8\x14a\x02\x8AW\x80c*\xDE8\x80\x14a\x02\x9EW\x80c4N\x13\x83\x14a\x02\xBFW\x80c9\x1C\xC9\xF6\x14a\x02\xEBW__\xFD[\x80c\x07\x1C%\xB7\x14a\x01\xF4W\x80c\x1E\xD7\x83\x1C\x14a\x02\x15W\x80c \xA5E\xD9\x14a\x02?W\x80c#\xE4\x81u\x14a\x02^W__\xFD[6a\x01\xF0W\0[__\xFD[4\x80\x15a\x01\xFFW__\xFD[Pa\x02\x13a\x02\x0E6`\x04a?[V[a\x06\x11V[\0[4\x80\x15a\x02 W__\xFD[Pa\x02)a\x07\xD7V[`@Qa\x026\x91\x90a?}V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02JW__\xFD[Pa\x02\x13a\x02Y6`\x04aAeV[a\x087V[4\x80\x15a\x02iW__\xFD[Pa\x02}a\x02x6`\x04aB|V[a\x0BBV[`@Qa\x026\x91\x90aC\xD9V[4\x80\x15a\x02\x95W__\xFD[Pa\x02\x13a\x0E\x92V[4\x80\x15a\x02\xA9W__\xFD[Pa\x02\xB2a\x0F\xF9V[`@Qa\x026\x91\x90aD\xB8V[4\x80\x15a\x02\xCAW__\xFD[Pa\x02\xDEa\x02\xD96`\x04aF\tV[a\x115V[`@Qa\x026\x91\x90aF\xB8V[4\x80\x15a\x02\xF6W__\xFD[Pa\x02}a\x03\x056`\x04aG\x0FV[a\x12\x8EV[4\x80\x15a\x03\x15W__\xFD[Pa\x03)a\x03$6`\x04aG*V[a\x14\tV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x026V[4\x80\x15a\x03LW__\xFD[Pa\x02)a\x14\xAEV[4\x80\x15a\x03`W__\xFD[Pa\x02)a\x15\x0CV[4\x80\x15a\x03tW__\xFD[Pa\x03\x88a\x03\x836`\x04aG\xC3V[a\x15jV[`@Qa\x026\x91\x90aG\xFCV[4\x80\x15a\x03\xA0W__\xFD[Pa\x02\xDEa\x03\xAF6`\x04aF\tV[a\x16!V[4\x80\x15a\x03\xBFW__\xFD[Pa\x03\xC8a\x17pV[`@Qa\x026\x91\x90aHHV[4\x80\x15a\x03\xE0W__\xFD[Pa\x03\x88a\x03\xEF6`\x04aG\xC3V[a\x18\xD4V[4\x80\x15a\x03\xFFW__\xFD[Pa\x02\x13a\x04\x0E6`\x04aB|V[a\x19\x8AV[4\x80\x15a\x04\x1EW__\xFD[Pa\x02\x13a\x04-6`\x04aG*V[a\x1E\xDBV[4\x80\x15a\x04=W__\xFD[Pa\x04Fa\x1F\x9BV[`@Qa\x026\x91\x90aH\xC6V[4\x80\x15a\x04^W__\xFD[Pa\x02\x13a fV[4\x80\x15a\x04rW__\xFD[Pa\x04{a!\x16V[`@Qa\x026\x91\x90aH\xD8V[4\x80\x15a\x04\x93W__\xFD[Pa\x02}a!\xF7V[4\x80\x15a\x04\xA7W__\xFD[Pa\x04\xB0a%\x12V[`@Qa\x026\x91\x90aI\x86V[4\x80\x15a\x04\xC8W__\xFD[Pa\x04\xD1a&\xB4V[`@Qa\x026\x91\x90aI\x98V[4\x80\x15a\x04\xE9W__\xFD[P`&Ta\x04\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x026V[4\x80\x15a\x05 W__\xFD[Pa\x02\x13a\x05/6`\x04aG\x0FV[a';V[4\x80\x15a\x05?W__\xFD[Pa\x04{a(\x88V[4\x80\x15a\x05SW__\xFD[Pa\x04Fa)iV[4\x80\x15a\x05gW__\xFD[Pa\x05\x8Aa\x05v6`\x04aI\xAAV[`)` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x026V[4\x80\x15a\x05\xA5W__\xFD[Pa\x05\x8Aa*4V[4\x80\x15a\x05\xB9W__\xFD[Pa\x02\x13a*\xD4V[4\x80\x15a\x05\xCDW__\xFD[Pa\x02)a+\x85V[4\x80\x15a\x05\xE1W__\xFD[Pa\x05\xEAa+\xE3V[`@Qa\x026\x92\x91\x90aI\xC1V[4\x80\x15a\x06\x03W__\xFD[P`\x1FTa\x05\x8A\x90`\xFF\x16\x81V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06bW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x86\x91\x90aI\xEBV[Pa\x06\xBA`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qverifyStaleBalance`p\x1B\x81RPa,\x9CV[`$\x80T`@Qc\x08\xFA\x0B\x13`\xE2\x1B\x81Rd\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c#\xE8,L\x91\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x07W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07.\x91\x90\x81\x01\x90aK8V[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x01\xC8\xAB\xE9`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93c\x03\x91W\xD2\x93a\x07m\x93\x92\x91`\x04\x01aLXV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x84W__\xFD[PZ\xF1\x92PPP\x80\x15a\x07\x95WP`\x01[a\x07\xD3W=\x80\x80\x15a\x07\xC2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07\xC7V[``\x91P[Pa\x07\xD1\x81a,\xF9V[P[PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FW[PPPPP\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xAC\x91\x90aI\xEBV[Pa\x08\xDC`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mupdateBalances`\x90\x1B\x81RPa,\x9CV[_[\x82Q\x81\x10\x15a\x07\xD1W_\x83\x82\x81Q\x81\x10a\x08\xFAWa\x08\xFAaL\xB9V[` \x02` \x01\x01Q\x90P_\x83\x83\x81Q\x81\x10a\t\x17Wa\t\x17aL\xB9V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\t\xDCWa\tVa-UV[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xCA\x91\x90aI\xEBV[\x15a\t\xD7Wa\t\xD7a-\xE4V[a\x0B8V[_\x81\x90P_\x83`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nA\x91\x90aL\xCDV[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB7\x91\x90aL\xE8V[P`!T`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\x10W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B4\x91\x90aI\xEBV[PPP[PP`\x01\x01a\x08\xDEV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x0B\x8BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xAF\x91\x90aI\xEBV[Pa\x0B\xE1`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oqueueWithdrawals`\x80\x1B\x81RPa,\x9CV[` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R0`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CK\x91\x90aL\xCDV[` T`@Qc(^!!`\xE2\x1B\x81R0`\x04\x82\x01\x81\x90R\x92\x93P_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xBA\x91\x90aI\xEBV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R_\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xD4W\x90PP\x90P`@Q\x80``\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81_\x81Q\x81\x10a\r:Wa\r:aL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01[a\rca>\xF0V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r[W\x90PP\x90P`@Q\x80`\xE0\x01`@R\x800`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x88\x81RP\x81_\x81Q\x81\x10a\r\xDEWa\r\xDEaL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91RT`@Qc\x06\xECn\x81`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\r\xD8\xDD\x02\x90a\x0E\x1B\x90\x86\x90`\x04\x01aM\x07V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E6W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E]\x91\x90\x81\x01\x90aM\x99V[\x90Pa\x0E\x84\x82Q\x82Q`@Q\x80``\x01`@R\x80`&\x81R` \x01aX\xA2`&\x919a0\xDEV[P\x94PPPPP[\x92\x91PPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x07\x91\x90aI\xEBV[Pa\x0F;`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q92\xB3\xB4\xB9\xBA2\xB9 \xB9\xA7\xB82\xB90\xBA7\xB9`q\x1B\x81RPa,\x9CV[`@\x80Q``\x81\x01\x82R0\x81R_` \x80\x83\x01\x82\x81R\x83\x85\x01\x92\x83R\x90T`(T\x94Qc\x02K\x98\x03`\xE5\x1B\x81R\x84Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x92Q\x83\x16`$\x82\x01R\x92Qc\xFF\xFF\xFF\xFF\x90\x81\x16`D\x85\x01R\x90\x94\x16`d\x83\x01R`\xA0`\x84\x83\x01R`\x08`\xA4\x83\x01Rgmetadata`\xC0\x1B`\xC4\x83\x01R\x91\x92\x91\x90\x91\x16\x90cIs\0`\x90`\xE4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xE0W__\xFD[PZ\xF1\x15\x80\x15a\x0F\xF2W=__>=_\xFD[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x11\x15W\x83\x82\x90_R` _ \x01\x80Ta\x10\x8A\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xB6\x90aM\xCAV[\x80\x15a\x11\x01W\x80`\x1F\x10a\x10\xD8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\x01V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x10mV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10\x1CV[PPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x11~W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA2\x91\x90aI\xEBV[Pa\x11\xE1`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa,\x9CV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xFBWa\x11\xFBa?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12.W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\x19W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12\x85Wa\x12`\x84\x82\x81Q\x81\x10a\x12QWa\x12QaL\xB9V[` \x02` \x01\x01Q`\x01a1JV[\x82\x82\x81Q\x81\x10a\x12rWa\x12raL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x123V[P\x90P[\x91\x90PV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x12\xD7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFB\x91\x90aI\xEBV[Pa\x13\x8E`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nforceUndelegate`\x88\x1B\x81RP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13bW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\x89\x91\x90\x81\x01\x90aN\x02V[a4dV[_a\x13\x98\x83a4\xBFV[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xE2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\x85\x91\x90\x81\x01\x90aM\x99V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\x14QW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14u\x91\x90aI\xEBV[Pa\x14\xA5`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mexitValidators`\x90\x1B\x81RPa,\x9CV[a\x0E\x8C\x82a7\xD4V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FWPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FWPPPPP\x90P\x90V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x15\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD7\x91\x90aI\xEBV[Pa\x16\x16`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa,\x9CV[a\x0E\x8C\x82`\x01a1JV[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x16jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x8E\x91\x90aI\xEBV[Pa\x16\xCD`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa,\x9CV[_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE7Wa\x16\xE7a?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x1AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17\x05W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x12\x85Wa\x17K\x84\x82\x81Q\x81\x10a\x17=Wa\x17=aL\xB9V[` \x02` \x01\x01Q_a1JV[\x82\x82\x81Q\x81\x10a\x17]Wa\x17]aL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x17\x1FV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x17\xC3\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xEF\x90aM\xCAV[\x80\x15a\x18:W\x80`\x1F\x10a\x18\x11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18:V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18\xBCW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x18~W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x17\x93V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\x19\x1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19A\x91\x90aI\xEBV[Pa\x19\x80`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa,\x9CV[a\x0E\x8C\x82_a1JV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\xDBW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xFF\x91\x90aI\xEBV[Pa\x1A>`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FdepositIntoEigenlayer_ALT\0\0\0\0\0\0\0\x81RPa,\x9CV[_\x19_[\x83Q\x81\x10\x15a\x1E\xD5W_\x84\x82\x81Q\x81\x10a\x1A^Wa\x1A^aL\xB9V[` \x02` \x01\x01Q\x90P_\x84\x83\x81Q\x81\x10a\x1A{Wa\x1A{aL\xB9V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B/W_a\x1A\xBBa9\x17V[P\x90P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\nW__\xFD[PZ\xF1\x15\x80\x15a\x1B\x1CW=__>=_\xFD[PPPPa\x1B)\x81a=\x0CV[Pa\x1E\xCBV[_\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BlW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x90\x91\x90aL\xCDV[`!T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x06\x91\x90aL\xE8V[P`!T`@Qb?g_`\xE9\x1B\x81R0`\x04\x82\x01R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c~\xCE\xBE\0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CLW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cp\x91\x90aI\xEBV[\x90P_`!_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cH\x82^\x94`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE7\x91\x90aI\xEBV[`@\x80Q` \x81\x01\x92\x90\x92R0\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16``\x83\x01R\x84\x16`\x80\x82\x01R`\xA0\x81\x01\x85\x90R`\xC0\x81\x01\x83\x90R`\xE0\x81\x01\x88\x90Ra\x01\0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `!Tc\xF6\x98\xDA%`\xE0\x1B\x85R\x92Q\x90\x94P_\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF6\x98\xDA%\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1D\x85W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xA9\x91\x90aI\xEBV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92P_\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\x01`)_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`!_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\xE8\x9A\xCE\x88\x87\x890\x8E\x87`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Ep\x96\x95\x94\x93\x92\x91\x90aNFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xB0\x91\x90aI\xEBV[PP_\x90\x81R`)` R`@\x90 \x80T`\xFF\x19\x16\x90UPPP[PP`\x01\x01a\x1ABV[PPPPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F,W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FP\x91\x90aI\xEBV[Pa\x1F\x8F`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FverifyWithdrawalCredentials\0\0\0\0\0\x81RPa,\x9CV[a\x1F\x98\x81a=\x0CV[PV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W\x83\x82\x90_R` _ \x01\x80Ta\x1F\xDB\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x07\x90aM\xCAV[\x80\x15a RW\x80`\x1F\x10a )Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a RV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a 5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xBEV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xDB\x91\x90aI\xEBV[Pa!\x0C`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x1C\xDD\x18\\\x9D\x10\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x8A\x1B\x81RPa,\x9CV[a!\x14a-UV[V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a!\xDFW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a!\xA1W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a!9V[`#T`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81_\x87Z\xF1\x15\x80\x15a\"@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"d\x91\x90aI\xEBV[Pa\"\x90`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iundelegate`\xB0\x1B\x81RPa,\x9CV[_a\"\x9A0a4\xBFV[` T`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x8B\xE8d\x90`$\x01_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xE1W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\x08\x91\x90\x81\x01\x90aM\x99V[P_[\x81Q\x81\x10\x15a%\x0CW_Q` aX\x03_9_Q\x90_R`@Qa#X\x90` \x80\x82R`\x15\x90\x82\x01Rt2\xBC82\xB1\xBA4\xB73\x90;\xB4\xBA4290\xBB\xB0\xB6\x1D`Y\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a#\x93Wa#\x93aL\xB9V[` \x02` \x01\x01Q``\x01Q`@Qa#\xD0\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03s{s\x1B)\xD1`\xCD\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x82\x82\x81Q\x81\x10a$\x0BWa$\x0BaL\xB9V[` \x02` \x01\x01Q`\xA0\x01Q_\x81Q\x81\x10a$(Wa$(aL\xB9V[` \x02` \x01\x01Q`@Qa$j\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03\x9B\xA3\x93\x0B\xA1\xD1`\xCD\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82\x81Q\x81\x10a$\xA5Wa$\xA5aL\xB9V[` \x02` \x01\x01Q`\xC0\x01Q_\x81Q\x81\x10a$\xC2Wa$\xC2aL\xB9V[` \x02` \x01\x01Q`@Qa$\xFC\x91\x90`@\x80\x82R`\x08\x90\x82\x01Rg\x03\x9BC\x0B\x93+\x99\xD1`\xC5\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a#\x0BV[P\x90P\x90V[`'T``\x90_\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a%1Wa%1a?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%ZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80\x80[`'T\x81\x10\x15a&\xABW`$T`'\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xAAG8\x9C\x91\x90\x84\x90\x81\x10a%\x95Wa%\x95aL\xB9V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a%\xE3\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xFEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\"\x91\x90aL\xE8V[\x15a&\xA3W`'\x81\x81T\x81\x10a&:Wa&:aL\xB9V[\x90_R` _ \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10a&qWa&qaL\xB9V[d\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a&\x91\x81aN\x9BV[\x93PP\x81\x80a&\x9F\x90aN\x9BV[\x92PP[`\x01\x01a%aV[PP\x81R\x91\x90PV[```%\x80Ta&\xC3\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xEF\x90aM\xCAV[\x80\x15a\x08-W\x80`\x1F\x10a'\x11Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08-V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\x1DWP\x93\x95\x94PPPPPV[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a'\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xB0\x91\x90aI\xEBV[Pa(\x12`@Q\x80`@\x01`@R\x80`\n\x81R` \x01idelegateTo`\xB0\x1B\x81RP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13bW=__>=_\xFD[`@\x80Q\x80\x82\x01\x82R``\x81R_` \x80\x83\x01\x82\x90RT\x92Qc\xEE\xA9\x06K`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE\xA9\x06K\x91a(W\x91\x86\x91\x86\x91`\x04\x01aN\xB3V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(nW__\xFD[PZ\xF1\x15\x80\x15a(\x80W=__>=_\xFD[PPPPPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a)QW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a)\x13W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a(\xABV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x11,W\x83\x82\x90_R` _ \x01\x80Ta)\xA9\x90aM\xCAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta)\xD5\x90aM\xCAV[\x80\x15a* W\x80`\x1F\x10a)\xF7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a* V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\x03W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a)\x8CV[`\x08T_\x90`\xFF\x16\x15a*KWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xCD\x91\x90aI\xEBV[\x14\x15\x90P\x90V[`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a+%W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+I\x91\x90aI\xEBV[Pa+}`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\xDB\xDB\\\x1B\x19]\x19P\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`r\x1B\x81RPa,\x9CV[a!\x14a-\xE4V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08-W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x0FWPPPPP\x90P\x90V[``_`#_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a,7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,[\x91\x90aI\xEBV[Pa,\x8C`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nstartValidators`\x88\x1B\x81RPa,\x9CV[a,\x94a9\x17V[\x91P\x91P\x90\x91V[_Q` aX\x03_9_Q\x90_Ra,\xBAa,\xB5a&\xB4V[a=\xC3V[a,\xC3\x83a=\xECV[`@Q` \x01a,\xD4\x92\x91\x90aO\nV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra,\xEE\x91aI\x98V[`@Q\x80\x91\x03\x90\xA1PV[\x80Q\x15a-\x08W\x80Q\x81` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Freverted with unknown error\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`&T`@Qc\x88gl\xAD`\xE0\x1B\x81R_`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x88gl\xAD\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a-\x97W__\xFD[PZ\xF1\x92PPP\x80\x15a-\xA8WP`\x01[a!\x14W=\x80\x80\x15a-\xD5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a-\xDAV[``\x91P[Pa\x1F\x98\x81a,\xF9V[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F- active validator count\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R`&T\x83Qc#@\xE8\xD3`\xE0\x1B\x81R\x93Qa.\x89\x94`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c#@\xE8\xD3\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a.`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x84\x91\x90aI\xEBV[a>\x14V[`@\x80Q\x80\x82\x01\x82R`\x12\x81Rq- proofs remaining`p\x1B` \x82\x01R`&T\x82Qc#\xE9A\xB9`\xE1\x1B\x81R\x92Qa/,\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cG\xD2\x83r\x91`\x04\x80\x83\x01\x92`\xA0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a.\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x1E\x91\x90aO)V[` \x01Qb\xFF\xFF\xFF\x16a>\x14V[`&T`@\x80Qc!v\x7F\x95`\xE1\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cB\xEC\xFF*\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a/sW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x97\x91\x90aO\xA2V[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a0\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FUser._completeCheckpoint: no exi`D\x82\x01Ro\x1C\xDD\x1A[\x99\xC8\x18\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x82\x1B`d\x82\x01R`\x84\x01a-LV[`$T`@Qc\xB1\xB6\xF6\xA1`\xE0\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xB6\xF6\xA1\x90a0=\x90`'\x90\x86\x90`\x04\x01aO\xBBV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0WW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra0~\x91\x90\x81\x01\x90aQ V[\x90Pa0\xA7`@Q\x80``\x01`@R\x80`\"\x81R` \x01aX\x80`\"\x919\x82` \x01QQa>\x14V[`&T\x81Q` \x83\x01Q`@Qcx:]1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF0t\xBAb\x92a(W\x92\x90\x91`\x04\x01aRzV[`@Qc\x88\xB4L\x85`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x88\xB4L\x85\x90a1\x19\x90\x86\x90\x86\x90\x86\x90`\x04\x01aS\x10V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a1/W__\xFD[PZ\xFA\x15\x80\x15a1AW=__>=_\xFD[PPPPPPPV[``_\x83`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a1jWa1ja?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\x93W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a3\xFAW_\x85`\xA0\x01Q\x82\x81Q\x81\x10a1\xB8Wa1\xB8aL\xB9V[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a3^Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x83\x83\x81Q\x81\x10a2\x16Wa2\x16aL\xB9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x15a3YWa2]`@Q\x80``\x01`@R\x80`2\x81R` \x01aXN`2\x919a>EV[a2ma2ha%\x12V[a7\xD4V[P`$_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\xBAW__\xFD[PZ\xF1\x15\x80\x15a2\xCCW=__>=_\xFD[PPPPa2\xD8a-UV[`&_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3L\x91\x90aI\xEBV[\x15a3YWa3Ya-\xE4V[a3\xF1V[\x80`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xBE\x91\x90aL\xCDV[\x83\x83\x81Q\x81\x10a3\xD0Wa3\xD0aL\xB9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP[P`\x01\x01a1\x98V[P` T`@Qc\x0EL\xC3\xF9`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\xCC?\x90\x90a4/\x90\x87\x90\x85\x90\x88\x90`\x04\x01aS.V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4FW__\xFD[PZ\xF1\x15\x80\x15a4XW=__>=_\xFD[P\x92\x96\x95PPPPPPV[_Q` aX\x03_9_Q\x90_Ra4}a,\xB5a&\xB4V[a4\x86\x84a=\xECV[\x83`@Q` \x01a4\x99\x93\x92\x91\x90aSeV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra4\xB3\x91aI\x98V[`@Q\x80\x91\x03\x90\xA1PPV[` T`@Qcf\xD5\xBA\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R``\x92_\x92\x83\x92\x90\x91\x16\x90cf\xD5\xBA\x93\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\rW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra54\x91\x90\x81\x01\x90aS\x9AV[\x91P\x91P_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a5RWa5Ra?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\x8BW\x81` \x01[a5xa>\xF0V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a5pW\x90P[P` T`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xD9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xFD\x91\x90aL\xCDV[` T`@Qc(^!!`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P_\x92\x90\x91\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6JW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6n\x91\x90aI\xEBV[\x90P_[\x85Q\x81\x10\x15a7\xC8W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P_\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a6\xD0Wa6\xD0aL\xB9V[` \x02` \x01\x01Q\x82_\x81Q\x81\x10a6\xEAWa6\xEAaL\xB9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x86\x83\x81Q\x81\x10a7\x1CWa7\x1CaL\xB9V[` \x02` \x01\x01Q\x81_\x81Q\x81\x10a76Wa76aL\xB9V[` \x02` \x01\x01\x81\x81RPP`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x86a7\x86\x91\x90aTUV[\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81RP\x86\x84\x81Q\x81\x10a7\xB3Wa7\xB3aL\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a6rV[P\x91\x96\x95PPPPPPV[_a8\x15`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F- exiting num validators\0\0\0\0\0\0\0\0\x81RP\x83Qa>\x14V[_[\x82Q\x81\x10\x15a8\xCEW`$T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xF9\x8AN\x90\x85\x90\x84\x90\x81\x10a8IWa8IaL\xB9V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a8z\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xBA\x91\x90aO\xA2V[a8\xC4\x90\x83aThV[\x91P`\x01\x01a8\x17V[Pa\x12\x89`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7F- exited balance to pod (gwei)\0\0\x81RP\x82`\x01`\x01`@\x1B\x03\x16a>\x14V[``_G\x81a9/h\x01\xBC\x16\xD6t\xEC\x80\0\0\x83aT\x9BV[\x90Pa9D\x81h\x01\xBC\x16\xD6t\xEC\x80\0\0aT\xAEV[a9N\x90\x83aT\xC5V[\x91P_\x81g\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a9\x95Wa9oc;\x9A\xCA\0\x85aT\xD8V[a9y\x90\x85aT\xC5V[\x91Pa9\x85\x82\x85aT\xC5V[\x93P\x80a9\x91\x81aN\x9BV[\x91PP[\x80_\x03a:\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FstartValidators: not enough ETH `D\x82\x01Rs:7\x909\xBA0\xB9:\x100\x90;0\xB64\xB20\xBA7\xB9`a\x1B`d\x82\x01R`\x84\x01a-LV[_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x1AWa:\x1Aa?\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a:CW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_c;\x9A\xCA\0a:V\x87GaT\xC5V[a:`\x91\x90aT\x9BV[\x90Pa:\xA2`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- creating new validators\0\0\0\0\0\0\0\x81RP\x83Qa>\x14V[a:\xCD`@Q\x80``\x01`@R\x80`+\x81R` \x01aX#`+\x919\x82`\x01`\x01`@\x1B\x03\x16a>\x14V[_[\x85\x81\x10\x15a;\xE3W`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05h\x01\xBC\x16\xD6t\xEC\x80\0\0a:\xFCa>aV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;\x18\x91\x90aI\x98V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a;4W=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;Y\x91\x90aT\xEBV[\x90P\x80\x84\x83\x81Q\x81\x10a;nWa;naL\xB9V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x81\x01\x83U_\x92\x90\x92R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x95\x85\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x93\x02\x19\x90\x93\x16\x92\x90\x92\x17\x90U\x01a:\xCFV[Pa;\xEF\x85`\x01aTUV[\x83\x03a<\xFFW`$T_\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05\x86a<\x11a>aV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<-\x91\x90aI\x98V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a<IW=__>=_\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<n\x91\x90aT\xEBV[\x90P\x80\x83`\x01\x85Qa<\x80\x91\x90aT\xC5V[\x81Q\x81\x10a<\x90Wa<\x90aL\xB9V[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`'\x80T`\x01\x81\x01\x82U_\x91\x90\x91R`\x06\x80\x82\x04\x7F\x98\xA4v\xF1h{\xC3\xD6\n-\xA2\xAD\xBC\xBA,F\x95\x8Ea\xFA/\xB4\x04,\xD7\xBCX\x16\xA7\x10\x19[\x01\x80T\x94\x84\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x92\x02\x19\x90\x92\x16\x91\x90\x91\x17\x90U[\x90\x97\x90\x96P\x94PPPPPV[`$T`@QcR\x85\x1D\r`\xE1\x1B\x81R_\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA5\n:\x1A\x90a=<\x90\x85\x90`\x04\x01aI\x86V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=VW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra=}\x91\x90\x81\x01\x90aU\x84V[`&T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc?e\xCF\x19`\xE0\x1B\x81R\x95\x96P`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x94c?e\xCF\x19\x94a\x07m\x94\x93\x92\x89\x92`\x04\x01aW\x17V[``a\x0E\x8C`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83a>\xA6V[``a\x0E\x8C`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83a>\xA6V[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa4\xB3\x92\x91\x90aW\xC4V[_Q` aX\x03_9_Q\x90_R\x81`@Qa,\xEE\x91\x90aI\x98V[`&T`@\x80Q`\x01`\xF8\x1B` \x82\x01R_`!\x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16`,\x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01a>\xD9\x93\x92\x91\x90aW\xE5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F\x98W__\xFD[_` \x82\x84\x03\x12\x15a?kW__\xFD[\x815a?v\x81a?IV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a?\xBDW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a?\x96V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xFEWa?\xFEa?\xC8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xB4Wa@\xB4a?\xC8V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a@\xD4Wa@\xD4a?\xC8V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\x98W__\xFD[_\x82`\x1F\x83\x01\x12aA\x01W__\xFD[\x815aA\x14aA\x0F\x82a@\xBCV[a@\x8CV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aA5W__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x805aAM\x81a@\xDEV[\x83R` \x92\x83\x01\x92\x01aA:V[P\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15aAvW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x8BW__\xFD[aA\x97\x85\x82\x86\x01a@\xF2V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xB2W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aA\xC2W__\xFD[\x805aA\xD0aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aA\xF1W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aB\x13W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aA\xF8V[\x80\x94PPPPP\x92P\x92\x90PV[_\x82`\x1F\x83\x01\x12aB0W__\xFD[\x815aB>aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aB_W__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x805\x83R` \x92\x83\x01\x92\x01aBdV[__`@\x83\x85\x03\x12\x15aB\x8DW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xA2W__\xFD[aB\xAE\x85\x82\x86\x01a@\xF2V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xC9W__\xFD[aB\xD5\x85\x82\x86\x01aB!V[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aB\xF1V[P\x93\x94\x93PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aC4V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q_\x91aC\x9D\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaC\xB7`\xE0\x85\x01\x82aB\xDFV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaC\xD0\x82\x82aC\"V[\x95\x94PPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84RaD\x1B\x85\x83QaCRV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aC\xFFV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aD\xACW`\x1F\x19\x85\x84\x03\x01\x88RaD\x96\x83\x83QaD0V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aDzV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aE\x19\x90\x87\x01\x82aD^V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aD\xDEV[\x805a\x12\x89\x81a@\xDEV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\x89W__\xFD[_`\xE0\x82\x84\x03\x12\x15aE]W__\xFD[aEea?\xDCV[\x90PaEp\x82aE/V[\x81RaE~` \x83\x01aE/V[` \x82\x01RaE\x8F`@\x83\x01aE/V[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaE\xAA`\x80\x83\x01aE:V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xC7W__\xFD[aE\xD3\x84\x82\x85\x01a@\xF2V[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xF1W__\xFD[aE\xFD\x84\x82\x85\x01aB!V[`\xC0\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aF\x19W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aF.W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aF>W__\xFD[\x805aFLaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aFmW__\xFD[` \x84\x01[\x83\x81\x10\x15aF\xADW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x8FW__\xFD[aF\x9E\x89` \x83\x89\x01\x01aEMV[\x84RP` \x92\x83\x01\x92\x01aFrV[P\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84RaF\xFA\x85\x83QaB\xDFV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aF\xDEV[_` \x82\x84\x03\x12\x15aG\x1FW__\xFD[\x815a?v\x81a@\xDEV[_` \x82\x84\x03\x12\x15aG:W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aGOW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aG_W__\xFD[\x805aGmaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aG\x8EW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aG\xB9W\x835aG\xA8\x81a?IV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aG\x95V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aG\xD3W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xE8W__\xFD[aG\xF4\x84\x82\x85\x01aEMV[\x94\x93PPPPV[` \x81R_a?v` \x83\x01\x84aB\xDFV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aH V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87RaH\x94`@\x88\x01\x82aD0V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01RaH\xAF\x81\x83aH\x0EV[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aHnV[` \x81R_a?v` \x83\x01\x84aD^V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aI9\x90\x87\x01\x82aH\x0EV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aH\xFEV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aC\x18W\x81Qd\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aIaV[` \x81R_a?v` \x83\x01\x84aIOV[` \x81R_a?v` \x83\x01\x84aD0V[_` \x82\x84\x03\x12\x15aI\xBAW__\xFD[P5\x91\x90PV[`@\x81R_aI\xD3`@\x83\x01\x85aIOV[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aI\xFBW__\xFD[PQ\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x12\x89W__\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15aJ1WaJ1a?\xC8V[P`\x1F\x83\x01`\x1F\x19\x16` \x01aJF\x81a@\x8CV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15aJZW__\xFD[\x82\x82` \x83\x01^_` \x84\x83\x01\x01R\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aJ\x7FW__\xFD[a?v\x83\x83Q` \x85\x01aJ\x18V[_`@\x82\x84\x03\x12\x15aJ\x9EW__\xFD[aJ\xA6a@\x04V[\x82Q\x81R` \x83\x01Q\x90\x91P`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xC5W__\xFD[aJ\xD1\x84\x82\x85\x01aJpV[` \x83\x01RP\x92\x91PPV[_\x82`\x1F\x83\x01\x12aJ\xECW__\xFD[\x81QaJ\xFAaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aK\x1BW__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x80Q\x83R` \x92\x83\x01\x92\x01aK V[_` \x82\x84\x03\x12\x15aKHW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK]W__\xFD[\x82\x01``\x81\x85\x03\x12\x15aKnW__\xFD[aKva@&V[aK\x7F\x82aJ\x02V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\x99W__\xFD[aK\xA5\x86\x82\x85\x01aJ\x8EV[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xC3W__\xFD[\x91\x90\x91\x01\x90`@\x82\x86\x03\x12\x15aK\xD7W__\xFD[aK\xDFa@\x04V[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xF4W__\xFD[aL\0\x87\x82\x86\x01aJ\xDDV[\x82RP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x1BW__\xFD[aL'\x87\x82\x86\x01aJpV[` \x83\x01RP`@\x82\x01R\x94\x93PPPPV[\x80Q\x82R_` \x82\x01Q`@` \x85\x01RaG\xF4`@\x85\x01\x82aD0V[`\x01`\x01`@\x1B\x03\x84\x16\x81R``` \x82\x01R_aLy``\x83\x01\x85aL:V[\x82\x81\x03`@\x84\x01R\x83Q`@\x82RaL\x94`@\x83\x01\x82aC\"V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaL\xAD\x82\x82aD0V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aL\xDDW__\xFD[\x81Qa?v\x81a@\xDEV[_` \x82\x84\x03\x12\x15aL\xF8W__\xFD[\x81Q\x80\x15\x15\x81\x14a?vW__\xFD[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a4XW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q``\x87RaMS``\x88\x01\x82aB\xDFV[\x90P` \x82\x01Q\x87\x82\x03` \x89\x01RaMl\x82\x82aC\"V[`@\x93\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x98\x90\x93\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aM-V[_` \x82\x84\x03\x12\x15aM\xA9W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xBEW__\xFD[aG\xF4\x84\x82\x85\x01aJ\xDDV[`\x01\x81\x81\x1C\x90\x82\x16\x80aM\xDEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aM\xFCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aN\x12W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN'W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aN7W__\xFD[aG\xF4\x84\x82Q` \x84\x01aJ\x18V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R`@\x82\x01\x86\x90R\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xC0`\xA0\x82\x01\x81\x90R_\x90aL\xAD\x90\x83\x01\x84aD0V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01aN\xACWaN\xACaN\x87V[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R_\x83Q`@``\x84\x01RaN\xDC`\xA0\x84\x01\x82aD0V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aO\x15\x82\x85aN\xF3V[`\x17`\xF9\x1B\x81RaC\xD0`\x01\x82\x01\x85aN\xF3V[_`\xA0\x82\x84\x03\x12\x80\x15aO:W__\xFD[PaOCa@HV[\x82Q\x81R` \x83\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14aO\\W__\xFD[` \x82\x01RaOm`@\x84\x01aJ\x02V[`@\x82\x01R``\x83\x01Q\x80`\x07\x0B\x81\x14aO\x85W__\xFD[``\x82\x01RaO\x96`\x80\x84\x01aJ\x02V[`\x80\x82\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15aO\xB2W__\xFD[a?v\x82aJ\x02V[_`@\x82\x01`@\x83R\x80\x85TaO\xD5\x81\x84\x90\x81R` \x01\x90V[_\x88\x81R` \x81 \x94P\x90\x92P[\x81`\x05\x82\x01\x10\x15aPJW\x83Td\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x85R`(\x82\x90\x1C\x81\x16` \x86\x01R`P\x82\x90\x1C\x81\x16`@\x86\x01R`x\x82\x90\x1C\x81\x16``\x86\x01R`\xA0\x82\x81\x1C\x82\x16`\x80\x87\x01R`\xC8\x92\x90\x92\x1C\x16\x90\x84\x01R`\x01\x90\x93\x01\x92`\xC0\x90\x92\x01\x91`\x06\x01aO\xE3V[\x92T\x92\x81\x81\x10\x15aPiWd\xFF\xFF\xFF\xFF\xFF\x84\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\x89Wd\xFF\xFF\xFF\xFF\xFF`(\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\xA9Wd\xFF\xFF\xFF\xFF\xFF`P\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\xC9Wd\xFF\xFF\xFF\xFF\xFF`x\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aP\xE9Wd\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aQ\x06W`\xC8\x84\x90\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x83\x01\x92P[PP`\x01`\x01`@\x1B\x03\x85\x16` \x85\x01R\x91Pa?v\x90PV[_` \x82\x84\x03\x12\x15aQ0W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQEW__\xFD[\x82\x01`@\x81\x85\x03\x12\x15aQVW__\xFD[aQ^a@\x04V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQsW__\xFD[aQ\x7F\x86\x82\x85\x01aJ\x8EV[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\x9AW__\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12aQ\xAEW__\xFD[\x81QaQ\xBCaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x87\x83\x11\x15aQ\xDDW__\xFD[` \x85\x01[\x83\x81\x10\x15aRiW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xFFW__\xFD[\x86\x01``\x81\x8B\x03`\x1F\x19\x01\x12\x15aR\x14W__\xFD[aR\x1Ca@&V[` \x82\x81\x01Q\x82R`@\x83\x01Q\x90\x82\x01R``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aREW__\xFD[aRT\x8C` \x83\x86\x01\x01aJpV[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01aQ\xE2V[P` \x84\x01RP\x90\x95\x94PPPPPV[`@\x81R_aR\x8C`@\x83\x01\x85aL:V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01_[\x83\x81\x10\x15aS\x02W`\x1F\x19\x86\x84\x03\x01\x85R\x81Q\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q\x90P```@\x85\x01RaR\xEB``\x85\x01\x82aD0V[` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01aR\xAFV[P\x90\x98\x97PPPPPPPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R_aC\xD0``\x83\x01\x84aD0V[``\x81R_aS@``\x83\x01\x86aCRV[\x82\x81\x03` \x84\x01RaSR\x81\x86aB\xDFV[\x91PP\x82\x15\x15`@\x83\x01R\x94\x93PPPPV[_aSp\x82\x86aN\xF3V[`\x17`\xF9\x1B\x81RaS\x84`\x01\x82\x01\x86aN\xF3V[\x90P`\x1D`\xF9\x1B\x81RaG\xB9`\x01\x82\x01\x85aN\xF3V[__`@\x83\x85\x03\x12\x15aS\xABW__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xC0W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aS\xD0W__\xFD[\x80QaS\xDEaA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aS\xFFW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aT*W\x83QaT\x19\x81a@\xDEV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aT\x06V[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTIW__\xFD[aB\xD5\x85\x82\x86\x01aJ\xDDV[\x80\x82\x01\x80\x82\x11\x15a\x0E\x8CWa\x0E\x8CaN\x87V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0E\x8CWa\x0E\x8CaN\x87V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aT\xA9WaT\xA9aT\x87V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\x8CWa\x0E\x8CaN\x87V[\x81\x81\x03\x81\x81\x11\x15a\x0E\x8CWa\x0E\x8CaN\x87V[_\x82aT\xE6WaT\xE6aT\x87V[P\x06\x90V[_` \x82\x84\x03\x12\x15aT\xFBW__\xFD[\x81Qa?v\x81a?IV[_\x82`\x1F\x83\x01\x12aU\x15W__\xFD[\x81QaU#aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aUDW__\xFD[` \x85\x01[\x83\x81\x10\x15aA[W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aUfW__\xFD[aUu\x88` \x83\x8A\x01\x01aJ\xDDV[\x84RP` \x92\x83\x01\x92\x01aUIV[_` \x82\x84\x03\x12\x15aU\x94W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xA9W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15aU\xBAW__\xFD[aU\xC2a@jV[aU\xCB\x82aJ\x02V[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xE5W__\xFD[aU\xF1\x86\x82\x85\x01aJ\x8EV[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x0FW__\xFD[\x82\x01`\x1F\x81\x01\x86\x13aV\x1FW__\xFD[\x80QaV-aA\x0F\x82a@\xBCV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15aVNW__\xFD[` \x84\x01[\x83\x81\x10\x15aV\x8EW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aVpW__\xFD[aV\x7F\x8B` \x83\x89\x01\x01aJpV[\x84RP` \x92\x83\x01\x92\x01aVSV[P`@\x85\x01RPPP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xAFW__\xFD[aV\xBB\x86\x82\x85\x01aU\x06V[``\x83\x01RP\x94\x93PPPPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aD\xACW`\x1F\x19\x85\x84\x03\x01\x88RaW\x01\x83\x83QaC\"V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aV\xE5V[`\x01`\x01`@\x1B\x03\x86\x16\x81R`\xA0` \x82\x01R_aW8`\xA0\x83\x01\x87aL:V[\x82\x81\x03`@\x84\x01RaWJ\x81\x87aIOV[\x90P\x82\x81\x03``\x84\x01R\x80\x85Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x88\x01_[\x83\x81\x10\x15aW\xA1W`\x1F\x19\x86\x84\x03\x01\x85RaW\x8B\x83\x83QaD0V[` \x95\x86\x01\x95\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aWoV[PP\x85\x81\x03`\x80\x87\x01RaW\xB5\x81\x88aV\xC9V[\x9B\x9APPPPPPPPPPPV[`@\x81R_aW\xD6`@\x83\x01\x85aD0V[\x90P\x82` \x83\x01R\x93\x92PPPV[_aC\xD0aW\xFCaW\xF6\x84\x88aN\xF3V[\x86aN\xF3V[\x84aN\xF3V\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP- depositing balance to beacon chain (gwei)- exiting all validators and completing checkpoint- submitting num checkpoint proofsUser.queueWithdrawals: length mismatch\xA2dipfsX\"\x12 .'Jb \xD7\xE1\xF2\x0E\xEA\xC1\xE5\xC4AB\x0Baf\n\xD3\x1D\x07\xFE\r\xB0H(?b\x12_\x98dsolcC\0\x08\x1B\x003",
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = NAMEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeCheckpointCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeCheckpointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeCheckpointCall {
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
            impl ::core::convert::From<completeCheckpointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub withdrawal:
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingRustTuple<'a> =
                (<IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<completeWithdrawalAsSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsSharesCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalAsSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                    }
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
            impl ::core::convert::From<completeWithdrawalAsSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalAsSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalAsSharesCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalAsSharesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub withdrawal:
            <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingRustTuple<'a> =
                (<IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<completeWithdrawalAsTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsTokensCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalAsTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                    }
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
            impl ::core::convert::From<completeWithdrawalAsTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalAsTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalAsTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalAsTokensCall {
            type Parameters<'a> = (IDelegationManagerTypes::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalAsTokensReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<completeWithdrawalsAsSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsSharesCall) -> Self {
                    (value.withdrawals,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalsAsSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeWithdrawalsAsSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalsAsSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalsAsSharesCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalsAsSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::Withdrawal,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.withdrawals
                ),)
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<completeWithdrawalsAsTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsTokensCall) -> Self {
                    (value.withdrawals,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalsAsTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeWithdrawalsAsTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeWithdrawalsAsTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeWithdrawalsAsTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeWithdrawalsAsTokensCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeWithdrawalsAsTokensReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    IDelegationManagerTypes::Withdrawal,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.withdrawals
                ),)
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub tokenBalances:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositIntoEigenlayerCall> for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoEigenlayerCall) -> Self {
                    (value.strategies, value.tokenBalances)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositIntoEigenlayerCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<depositIntoEigenlayerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoEigenlayerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for depositIntoEigenlayerReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositIntoEigenlayerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSelectorsCall {
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
            impl ::core::convert::From<excludeSelectorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSelectorsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `exitValidators(uint40[])` and selector `0x3d8c08d4`.
    ```solidity
    function exitValidators(uint40[] memory _validators) external returns (uint64 exitedBalanceGwei);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitValidatorsCall {
        pub _validators:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<exitValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _validators: tuple.0,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<exitValidatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorsReturn) -> Self {
                    (value.exitedBalanceGwei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        exitedBalanceGwei: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exitValidatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitValidatorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<40>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._validators
                ),)
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<forceUndelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: forceUndelegateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for forceUndelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for forceUndelegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = forceUndelegateReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<getActiveValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveValidatorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<getActiveValidatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveValidatorsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveValidatorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getActiveValidatorsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = podReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub depositShares:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queueWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsCall) -> Self {
                    (value.strategies, value.depositShares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalsCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<queueWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAsOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAsOperatorCall {
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
            impl ::core::convert::From<registerAsOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAsOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAsOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAsOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `signedHashes(bytes32)` and selector `0xb7172355`.
    ```solidity
    function signedHashes(bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signedHashesCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`signedHashes(bytes32)`](signedHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signedHashesReturn {
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
            impl ::core::convert::From<signedHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: signedHashesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for signedHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<signedHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: signedHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for signedHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for signedHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = signedHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "signedHashes(bytes32)";
            const SELECTOR: [u8; 4] = [183u8, 23u8, 35u8, 85u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startCheckpointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = startCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
                u64,
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
            impl ::core::convert::From<startValidatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startValidatorsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startValidatorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = startValidatorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = undelegateReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManagerTypes::Withdrawal>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub tokenDeltas:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::I256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateBalancesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateBalancesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateBalancesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateBalancesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
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
            impl ::core::convert::From<verifyStaleBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStaleBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorIndex: tuple.0,
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
            impl ::core::convert::From<verifyStaleBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStaleBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyStaleBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyStaleBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
    /**Function with signature `verifyWithdrawalCredentials(uint40[])` and selector `0x841c1299`.
    ```solidity
    function verifyWithdrawalCredentials(uint40[] memory _validators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsCall {
        pub _validators:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<verifyWithdrawalCredentialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyWithdrawalCredentialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _validators: tuple.0,
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
            impl ::core::convert::From<verifyWithdrawalCredentialsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyWithdrawalCredentialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyWithdrawalCredentialsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyWithdrawalCredentialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<40>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._validators
                ),)
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
    ///Container for all the [`User_AltMethods`](self) function calls.
    pub enum User_AltMethodsCalls {
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
        signedHashes(signedHashesCall),
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
    impl User_AltMethodsCalls {
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
            [183u8, 23u8, 35u8, 85u8],
            [186u8, 65u8, 79u8, 166u8],
            [214u8, 193u8, 13u8, 175u8],
            [226u8, 12u8, 159u8, 113u8],
            [242u8, 52u8, 193u8, 189u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for User_AltMethodsCalls {
        const NAME: &'static str = "User_AltMethodsCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
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
                Self::delegateTo(_) => <delegateToCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::signedHashes(_) => <signedHashesCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::targetSenders(_) => <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::undelegate(_) => <undelegateCall as alloy_sol_types::SolCall>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<User_AltMethodsCalls>] = &[
                {
                    fn verifyStaleBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::verifyStaleBalance)
                    }
                    verifyStaleBalance
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn updateBalances(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <updateBalancesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::updateBalances)
                    }
                    updateBalances
                },
                {
                    fn queueWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::queueWithdrawals)
                    }
                    queueWithdrawals
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::registerAsOperator)
                    }
                    registerAsOperator
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn completeWithdrawalsAsTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_AltMethodsCalls::completeWithdrawalsAsTokens)
                    }
                    completeWithdrawalsAsTokens
                },
                {
                    fn forceUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <forceUndelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::forceUndelegate)
                    }
                    forceUndelegate
                },
                {
                    fn exitValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <exitValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::exitValidators)
                    }
                    exitValidators
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn completeWithdrawalAsTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_AltMethodsCalls::completeWithdrawalAsTokens)
                    }
                    completeWithdrawalAsTokens
                },
                {
                    fn completeWithdrawalsAsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_AltMethodsCalls::completeWithdrawalsAsShares)
                    }
                    completeWithdrawalsAsShares
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn completeWithdrawalAsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_AltMethodsCalls::completeWithdrawalAsShares)
                    }
                    completeWithdrawalAsShares
                },
                {
                    fn depositIntoEigenlayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::depositIntoEigenlayer)
                    }
                    depositIntoEigenlayer
                },
                {
                    fn verifyWithdrawalCredentials(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_AltMethodsCalls::verifyWithdrawalCredentials)
                    }
                    verifyWithdrawalCredentials
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn startCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <startCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::startCheckpoint)
                    }
                    startCheckpoint
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::undelegate)
                    }
                    undelegate
                },
                {
                    fn getActiveValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::getActiveValidators)
                    }
                    getActiveValidators
                },
                {
                    fn NAME(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::NAME)
                    }
                    NAME
                },
                {
                    fn pod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <podCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::pod)
                    }
                    pod
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn signedHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <signedHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::signedHashes)
                    }
                    signedHashes
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::failed)
                    }
                    failed
                },
                {
                    fn completeCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <completeCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::completeCheckpoint)
                    }
                    completeCheckpoint
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn startValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <startValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(User_AltMethodsCalls::startValidators)
                    }
                    startValidators
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_AltMethodsCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(User_AltMethodsCalls::IS_TEST)
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
                Self::NAME(inner) => {
                    <NAMECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::completeCheckpoint(inner) => {
                    <completeCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exitValidators(inner) => {
                    <exitValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::forceUndelegate(inner) => {
                    <forceUndelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getActiveValidators(inner) => {
                    <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pod(inner) => <podCall as alloy_sol_types::SolCall>::abi_encoded_size(inner),
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::signedHashes(inner) => {
                    <signedHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startValidators(inner) => {
                    <startValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateBalances(inner) => {
                    <updateBalancesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <completeCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::completeWithdrawalAsShares(inner) => {
                    <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::completeWithdrawalAsTokens(inner) => {
                    <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::completeWithdrawalsAsShares(inner) => {
                    <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::completeWithdrawalsAsTokens(inner) => {
                    <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::depositIntoEigenlayer(inner) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::exitValidators(inner) => {
                    <exitValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::forceUndelegate(inner) => {
                    <forceUndelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getActiveValidators(inner) => {
                    <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::pod(inner) => {
                    <podCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::signedHashes(inner) => {
                    <signedHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::startValidators(inner) => {
                    <startValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateBalances(inner) => {
                    <updateBalancesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
            }
        }
    }
    ///Container for all the [`User_AltMethods`](self) events.
    pub enum User_AltMethodsEvents {
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
    impl User_AltMethodsEvents {
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
    impl alloy_sol_types::SolEventInterface for User_AltMethodsEvents {
        const NAME: &'static str = "User_AltMethodsEvents";
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
    impl alloy_sol_types::private::IntoLogData for User_AltMethodsEvents {
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
    /**Creates a new wrapper around an on-chain [`User_AltMethods`](self) contract instance.

    See the [wrapper's documentation](`User_AltMethodsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> User_AltMethodsInstance<T, P, N> {
        User_AltMethodsInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<User_AltMethodsInstance<T, P, N>>>
    {
        User_AltMethodsInstance::<T, P, N>::deploy(provider, name)
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
        User_AltMethodsInstance::<T, P, N>::deploy_builder(provider, name)
    }
    /**A [`User_AltMethods`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`User_AltMethods`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct User_AltMethodsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for User_AltMethodsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("User_AltMethodsInstance")
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
        > User_AltMethodsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`User_AltMethods`](self) contract instance.

        See the [wrapper's documentation](`User_AltMethodsInstance`) for more details.*/
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
            name: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<User_AltMethodsInstance<T, P, N>> {
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall { name })[..],
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
    impl<T, P: ::core::clone::Clone, N> User_AltMethodsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> User_AltMethodsInstance<T, P, N> {
            User_AltMethodsInstance {
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
        > User_AltMethodsInstance<T, P, N>
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
            self.call_builder(&completeWithdrawalAsSharesCall { withdrawal })
        }
        ///Creates a new call builder for the [`completeWithdrawalAsTokens`] function.
        pub fn completeWithdrawalAsTokens(
            &self,
            withdrawal: <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalAsTokensCall, N> {
            self.call_builder(&completeWithdrawalAsTokensCall { withdrawal })
        }
        ///Creates a new call builder for the [`completeWithdrawalsAsShares`] function.
        pub fn completeWithdrawalsAsShares(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalsAsSharesCall, N> {
            self.call_builder(&completeWithdrawalsAsSharesCall { withdrawals })
        }
        ///Creates a new call builder for the [`completeWithdrawalsAsTokens`] function.
        pub fn completeWithdrawalsAsTokens(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManagerTypes::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeWithdrawalsAsTokensCall, N> {
            self.call_builder(&completeWithdrawalsAsTokensCall { withdrawals })
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
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            tokenBalances: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositIntoEigenlayerCall, N> {
            self.call_builder(&depositIntoEigenlayerCall {
                strategies,
                tokenBalances,
            })
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
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            depositShares: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalsCall, N> {
            self.call_builder(&queueWithdrawalsCall {
                strategies,
                depositShares,
            })
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(&registerAsOperatorCall {})
        }
        ///Creates a new call builder for the [`signedHashes`] function.
        pub fn signedHashes(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, signedHashesCall, N> {
            self.call_builder(&signedHashesCall { _0 })
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
        pub fn targetSenders(&self) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`undelegate`] function.
        pub fn undelegate(&self) -> alloy_contract::SolCallBuilder<T, &P, undelegateCall, N> {
            self.call_builder(&undelegateCall {})
        }
        ///Creates a new call builder for the [`updateBalances`] function.
        pub fn updateBalances(
            &self,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            tokenDeltas: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::I256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateBalancesCall, N> {
            self.call_builder(&updateBalancesCall {
                strategies,
                tokenDeltas,
            })
        }
        ///Creates a new call builder for the [`verifyStaleBalance`] function.
        pub fn verifyStaleBalance(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyStaleBalanceCall, N> {
            self.call_builder(&verifyStaleBalanceCall { validatorIndex })
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials`] function.
        pub fn verifyWithdrawalCredentials(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyWithdrawalCredentialsCall, N> {
            self.call_builder(&verifyWithdrawalCredentialsCall { _validators })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > User_AltMethodsInstance<T, P, N>
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
