///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManagerTypes {
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startTimestamp; address[] strategies; uint256[] scaledShares; }
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
struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startTimestamp; address[] strategies; uint256[] scaledShares; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub staker: alloy::sol_types::private::Address,
        pub delegatedTo: alloy::sol_types::private::Address,
        pub withdrawer: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub startTimestamp: u32,
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
                    value.startTimestamp,
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
                    startTimestamp: tuple.4,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
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
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startTimestamp,address[] strategies,uint256[] scaledShares)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
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
                        &rust.startTimestamp,
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
                    &rust.startTimestamp,
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
library IDelegationManagerTypes {
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startTimestamp;
        address[] strategies;
        uint256[] scaledShares;
    }
}

library StdInvariant {
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface User_M1_AltMethods {
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
    function depositIntoEigenlayer_M1(address[] memory strategies, uint256[] memory tokenBalances) external;
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function exitValidators(uint40[] memory _validators) external returns (uint64 exitedBalanceGwei);
    function failed() external returns (bool);
    function forceUndelegate(address staker) external returns (IDelegationManagerTypes.Withdrawal[] memory);
    function getActiveValidators() external view returns (uint40[] memory);
    function isValidSignature(bytes32 hash, bytes memory) external view returns (bytes4);
    function pod() external view returns (address);
    function queueWithdrawals(address[] memory strategies, uint256[] memory shares) external returns (IDelegationManagerTypes.Withdrawal[] memory);
    function registerAsOperator() external;
    function signedHashes(bytes32) external view returns (bool);
    function startCheckpoint() external;
    function startValidators() external returns (uint40[] memory, uint64);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
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
            "name": "startTimestamp",
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
            "name": "startTimestamp",
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
            "name": "startTimestamp",
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
            "name": "startTimestamp",
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
    "name": "depositIntoEigenlayer_M1",
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
    "stateMutability": "nonpayable"
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
            "name": "startTimestamp",
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
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "hash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
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
        "name": "shares",
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
            "name": "startTimestamp",
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
            "name": "startTimestamp",
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
pub mod User_M1_AltMethods {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405260008054600160ff1991821681179092556004805490911682179055601b80546001600160a01b031916737109709ecfa91a80626ff3989d68f67f5b1dd12d1790556024805463ffffffff1916909117905534801561006257600080fd5b5060405161662038038061662083398101604081905261008191610565565b80806000339050806001600160a01b031663ea4d3c9b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156100c6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906100ea9190610649565b601c60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b03166339b70e386040518163ffffffff1660e01b8152600401602060405180830381865afa15801561014e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101729190610649565b601d60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316634665bcda6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156101d6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906101fa9190610649565b601e60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316633dfb40e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561025e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906102829190610649565b601f60006101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b03166322c0350b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156102e6573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061030a9190610649565b602080546001600160a01b0319166001600160a01b0392909216919091179055610332610459565b602161033e83826106f6565b5050506000339050806001600160a01b03166339b70e386040518163ffffffff1660e01b8152600401602060405180830381865afa158015610384573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906103a89190610649565b602460046101000a8154816001600160a01b0302191690836001600160a01b03160217905550806001600160a01b0316634665bcda6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561040c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104309190610649565b602580546001600160a01b0319166001600160a01b0392909216919091179055506107b4915050565b601e60009054906101000a90046001600160a01b03166001600160a01b03166384d810626040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156104a957600080fd5b505af11580156104bd573d6000803e3d6000fd5b5050601e54604051639ba0627560e01b81523060048201526001600160a01b039091169250639ba062759150602401602060405180830381865afa158015610509573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061052d9190610649565b602280546001600160a01b0319166001600160a01b0392909216919091179055565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561057757600080fd5b81516001600160401b0381111561058d57600080fd5b8201601f8101841361059e57600080fd5b80516001600160401b038111156105b7576105b761054f565b604051601f8201601f19908116603f011681016001600160401b03811182821017156105e5576105e561054f565b6040528181528282016020018610156105fd57600080fd5b60005b8281101561061c57602081850181015183830182015201610600565b50600091810160200191909152949350505050565b6001600160a01b038116811461064657600080fd5b50565b60006020828403121561065b57600080fd5b815161066681610631565b9392505050565b600181811c9082168061068157607f821691505b6020821081036106a157634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156106f157806000526020600020601f840160051c810160208510156106ce5750805b601f840160051c820191505b818110156106ee57600081556001016106da565b50505b505050565b81516001600160401b0381111561070f5761070f61054f565b6107238161071d845461066d565b846106a7565b6020601f821160018114610757576000831561073f5750848201515b600019600385901b1c1916600184901b1784556106ee565b600084815260208120601f198516915b828110156107875787850151825560209485019460019092019101610767565b50848210156107a55786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b615e5d806107c36000396000f3fe6080604052600436106101f25760003560e01c80636d336f581161010d578063a88dbb36116100a0578063ba414fa61161006f578063ba414fa6146105d2578063d6c10daf146105e7578063e20c9f71146105fc578063f234c1bd14610611578063fa7626d41461063457600080fd5b8063a88dbb3614610525578063ac637c7a1461055d578063b5508aa91461057d578063b71723551461059257600080fd5b8063916a17c6116100dc578063916a17c6146104b757806392ab89bb146104cc5780639de70258146104e1578063a3f4df7e1461050357600080fd5b80636d336f5814610440578063841c12991461046057806385226c811461048057806390b51625146104a257600080fd5b80633d8c08d41161018557806346a5be0d1161015457806346a5be0d146103be578063654bb5d9146103de57806366d9a9a0146103fe578063695f4ae11461042057600080fd5b80633d8c08d41461032f5780633e5e3c23146103675780633f7286f41461037c578063401be65e1461039157600080fd5b806323e48175116101c157806323e48175146102a05780632a34ade8146102cd578063344e1383146102e2578063391cc9f61461030f57600080fd5b8063071c25b7146101fe5780631626ba7e146102205780631ed7831c1461025e57806320a545d91461028057600080fd5b366101f957005b600080fd5b34801561020a57600080fd5b5061021e6102193660046143a8565b61064e565b005b34801561022c57600080fd5b5061024061023b3660046144e2565b610820565b6040516001600160e01b031990911681526020015b60405180910390f35b34801561026a57600080fd5b50610273610856565b604051610255919061456b565b34801561028c57600080fd5b5061021e61029b366004614660565b6108b8565b3480156102ac57600080fd5b506102c06102bb366004614780565b610bd7565b60405161025591906148e6565b3480156102d957600080fd5b5061021e610f37565b3480156102ee57600080fd5b506103026102fd366004614a1e565b6110ab565b6040516102559190614ad3565b34801561031b57600080fd5b506102c061032a366004614b2c565b611209565b34801561033b57600080fd5b5061034f61034a366004614b49565b611391565b6040516001600160401b039091168152602001610255565b34801561037357600080fd5b50610273611439565b34801561038857600080fd5b50610273611499565b34801561039d57600080fd5b506103b16103ac366004614be7565b6114f9565b6040516102559190614c23565b3480156103ca57600080fd5b506103026103d9366004614a1e565b6115b3565b3480156103ea57600080fd5b5061021e6103f9366004614780565b611708565b34801561040a57600080fd5b50610413611c86565b6040516102559190614c36565b34801561042c57600080fd5b506103b161043b366004614be7565b611d75565b34801561044c57600080fd5b5061021e61045b366004614780565b611e2f565b34801561046c57600080fd5b5061021e61047b366004614b49565b612142565b34801561048c57600080fd5b50610495612206565b6040516102559190614d34565b3480156104ae57600080fd5b5061021e6122d6565b3480156104c357600080fd5b5061041361238a565b3480156104d857600080fd5b506102c0612470565b3480156104ed57600080fd5b506104f6612774565b6040516102559190614dc6565b34801561050f57600080fd5b5061051861291f565b6040516102559190614dd9565b34801561053157600080fd5b50602254610545906001600160a01b031681565b6040516001600160a01b039091168152602001610255565b34801561056957600080fd5b5061021e610578366004614b2c565b6129a8565b34801561058957600080fd5b50610495612b03565b34801561059e57600080fd5b506105c26105ad366004614dec565b60266020526000908152604090205460ff1681565b6040519015158152602001610255565b3480156105de57600080fd5b506105c2612bd3565b3480156105f357600080fd5b5061021e612cfc565b34801561060857600080fd5b50610273612db1565b34801561061d57600080fd5b50610626612e11565b604051610255929190614e05565b34801561064057600080fd5b506000546105c29060ff1681565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156106a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106c79190614e30565b506106fb604051806040016040528060128152602001717665726966795374616c6542616c616e636560701b815250612ecf565b6020546040516308fa0b1360e21b815264ffffffffff831660048201526000916001600160a01b0316906323e82c4c90602401600060405180830381865afa15801561074b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526107739190810190614f60565b6022548151602083015160408085015190516301c8abe960e11b81529495506001600160a01b039093169363039157d2936107b293929160040161508a565b600060405180830381600087803b1580156107cc57600080fd5b505af19250505080156107dd575060015b61081c573d80801561080b576040519150601f19603f3d011682016040523d82523d6000602084013e610810565b606091505b5061081a81612f2d565b505b5050565b60008281526026602052604081205460ff16156108455750630b135d3f60e11b610850565b506001600160e01b03195b92915050565b6060600d8054806020026020016040519081016040528092919081815260200182805480156108ae57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610890575b5050505050905090565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af115801561090d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109319190614e30565b506109616040518060400160405280600e81526020016d75706461746542616c616e63657360901b815250612ecf565b60005b825181101561081a576000838281518110610981576109816150ec565b60200260200101519050600083838151811061099f5761099f6150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603610a67576109de612f84565b602260009054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a31573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a559190614e30565b15610a6257610a62613018565b610bcd565b60008190506000836001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015610aac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ad09190615102565b601d5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015610b25573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b49919061511f565b50601d546040516373d0285560e11b81526001600160a01b0386811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af1158015610ba5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bc99190614e30565b5050505b5050600101610964565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015610c23573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c479190614e30565b50610c796040518060400160405280601081526020016f71756575655769746864726177616c7360801b815250612ecf565b601c54604051631976849960e21b81523060048201526000916001600160a01b0316906365da126490602401602060405180830381865afa158015610cc2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ce69190615102565b601c5460405163285e212160e21b815230600482018190529293506000916001600160a01b03169063a178848490602401602060405180830381865afa158015610d34573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d589190614e30565b60408051600180825281830190925291925060009190816020015b60408051606080820183528082526020820152600091810191909152815260200190600190039081610d735790505090506040518060600160405280888152602001878152602001846001600160a01b031681525081600081518110610ddb57610ddb6150ec565b6020908102919091010152604080516001808252818301909252600091816020015b610e05614337565b815260200190600190039081610dfd5790505090506040518060e00160405280306001600160a01b03168152602001866001600160a01b03168152602001856001600160a01b031681526020018481526020014263ffffffff1681526020018981526020018881525081600081518110610e8157610e816150ec565b6020908102919091010152601c546040516306ec6e8160e11b81526000916001600160a01b031690630dd8dd0290610ebd908690600401615141565b6000604051808303816000875af1158015610edc573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610f0491908101906151d5565b9050610f2b82518251604051806060016040528060268152602001615e026026913961331f565b50979650505050505050565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610f8c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fb09190614e30565b50610fe4604051806040016040528060128152602001713932b3b4b9ba32b920b9a7b832b930ba37b960711b815250612ecf565b60408051606081018252308152600060208201818152828401918252601c5460248054955163024b980360e51b815285516001600160a01b0390811660048301529351841691810191909152925163ffffffff9081166044850152909416606483015260a06084830152600860a4830152676d6574616461746160c01b60c48301529192919091169063497300609060e401600060405180830381600087803b15801561109057600080fd5b505af11580156110a4573d6000803e3d6000fd5b5050505050565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af11580156110f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061111b9190614e30565b5061115a6040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612ecf565b600082516001600160401b03811115611175576111756143c5565b6040519080825280602002602001820160405280156111a857816020015b60608152602001906001900390816111935790505b50905060005b8351811015611200576111db8482815181106111cc576111cc6150ec565b60200260200101516001613367565b8282815181106111ed576111ed6150ec565b60209081029190910101526001016111ae565b5090505b919050565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015611255573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112799190614e30565b506113106040518060400160405280600f81526020016e666f726365556e64656c656761746560881b815250836001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156112e3573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261130b9190810190615209565b613694565b600061131b836136f0565b601c546040516336a2fa1960e21b81526001600160a01b03868116600483015292935091169063da8be864906024016000604051808303816000875af1158015611369573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261120091908101906151d5565b601f54604080516301504d8f60e41b815290516000926001600160a01b031691631504d8f0916004808301926020929190829003018187875af11580156113dc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114009190614e30565b506114306040518060400160405280600e81526020016d6578697456616c696461746f727360901b815250612ecf565b61085082613a16565b6060600f8054806020026020016040519081016040528092919081815260200182805480156108ae576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610890575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156108ae576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610890575050505050905090565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015611545573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115699190614e30565b506115a86040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612ecf565b610850826001613367565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af11580156115ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116239190614e30565b506116626040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612ecf565b600082516001600160401b0381111561167d5761167d6143c5565b6040519080825280602002602001820160405280156116b057816020015b606081526020019060019003908161169b5790505b50905060005b8351811015611200576116e38482815181106116d4576116d46150ec565b60200260200101516000613367565b8282815181106116f5576116f56150ec565b60209081029190910101526001016116b6565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af115801561175d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117819190614e30565b506117c06040518060400160405280601d81526020017f2e6465706f736974496e746f456967656e6c617965725f4d315f414c54000000815250612ecf565b60001960005b8351811015611c805760008482815181106117e3576117e36150ec565b602002602001015190506000848381518110611801576118016150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b0316036118bb5760405162461bcd60e51b815260206004820152604760248201527f53686f756c64206e6f74206265206465706f736974696e67207769746820424560448201527f41434f4e434841494e5f4554485f535452415420666f72204d312d6d61696e6e60648201526632ba102ab9b2b960c91b608482015260a4015b60405180910390fd5b6000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118fb573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061191f9190615102565b601d5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015611974573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611998919061511f565b50601d54604051623f675f60e91b81523060048201526000916001600160a01b031690637ecebe0090602401602060405180830381865afa1580156119e1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a059190614e30565b90506000601d60009054906101000a90046001600160a01b03166001600160a01b03166348825e946040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a5c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a809190614e30565b6040805160208101929092526001600160a01b0380881691830191909152841660608201526080810185905260a0810183905260c0810188905260e00160408051601f198184030181528282528051602091820120601d5463f698da2560e01b855292519094506000936001600160a01b039093169263f698da259260048083019391928290030181865afa158015611b1d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b419190614e30565b60405161190160f01b6020820152602281018290526042810184905290915060009060620160408051601f198184030181528282528051602091820120908301819052925060009101604051602081830303815290604052905060016026600084815260200190815260200160002060006101000a81548160ff021916908315150217905550601d60009054906101000a90046001600160a01b03166001600160a01b03166332e89ace89888a308f876040518763ffffffff1660e01b8152600401611c1296959493929190615251565b6020604051808303816000875af1158015611c31573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c559190614e30565b50506000908152602660205260409020805460ff1916905550505060019390930192506117c6915050565b50505050565b60606012805480602002602001604051908101604052809291908181526020016000905b82821015611d6c5760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611d5457602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611d165790505b50505050508152505081526020019060010190611caa565b50505050905090565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015611dc1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611de59190614e30565b50611e246040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612ecf565b610850826000613367565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611e84573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ea89190614e30565b50611edf604051806040016040528060158152602001743232b837b9b4ba24b73a37a2b4b3b2b73630bcb2b960591b815250612ecf565b60005b825181101561081a576000838281518110611eff57611eff6150ec565b602002602001015190506000838381518110611f1d57611f1d6150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603611fd8576000611f5e613b5e565b509050602060009054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b8152600401600060405180830381600087803b158015611fb157600080fd5b505af1158015611fc5573d6000803e3d6000fd5b50505050611fd281613f61565b50612138565b6000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015612018573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061203c9190615102565b601d5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015612091573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120b5919061511f565b50601d546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af1158015612111573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121359190614e30565b50505b5050600101611ee2565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015612197573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121bb9190614e30565b506121fa6040518060400160405280601b81526020017f7665726966795769746864726177616c43726564656e7469616c730000000000815250612ecf565b61220381613f61565b50565b60606011805480602002602001604051908101604052809291908181526020016000905b82821015611d6c57838290600052602060002001805461224990615293565b80601f016020809104026020016040519081016040528092919081815260200182805461227590615293565b80156122c25780601f10612297576101008083540402835291602001916122c2565b820191906000526020600020905b8154815290600101906020018083116122a557829003601f168201915b50505050508152602001906001019061222a565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af115801561232b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061234f9190614e30565b506123806040518060400160405280600f81526020016e1cdd185c9d10da1958dadc1bda5b9d608a1b815250612ecf565b612388612f84565b565b60606013805480602002602001604051908101604052809291908181526020016000905b82821015611d6c5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561245857602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161241a5790505b505050505081525050815260200190600101906123ae565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af11580156124bc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124e09190614e30565b5061250c6040518060400160405280600a815260200169756e64656c656761746560b01b815250612ecf565b6000612517306136f0565b601c546040516336a2fa1960e21b81523060048201529192506001600160a01b03169063da8be864906024016000604051808303816000875af1158015612562573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261258a91908101906151d5565b5060005b815181101561276e57600080516020615d438339815191526040516125dc9060208082526015908201527432bc3832b1ba34b733903bb4ba34323930bbb0b61d60591b604082015260600190565b60405180910390a1600080516020615de2833981519152828281518110612605576126056150ec565b602002602001015160600151604051612642919060408082526007908201526603737b731b29d160cd1b6060820152602081019190915260800190565b60405180910390a17f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f82828151811061267d5761267d6150ec565b602002602001015160a0015160008151811061269b5761269b6150ec565b60200260200101516040516126dd9190604080825260079082015266039ba3930ba1d160cd1b60608201526001600160a01b0391909116602082015260800190565b60405180910390a1600080516020615de2833981519152828281518110612706576127066150ec565b602002602001015160c00151600081518110612724576127246150ec565b602002602001015160405161275e9190604080825260089082015267039b430b932b99d160c51b6060820152602081019190915260800190565b60405180910390a160010161258e565b50905090565b6023546060906000906001600160401b03811115612794576127946143c5565b6040519080825280602002602001820160405280156127bd578160200160208202803683370190505b50905060008060005b60235481101561291657602054602380546001600160a01b039092169163aa47389c9190849081106127fa576127fa6150ec565b90600052602060002090600691828204019190066005029054906101000a900464ffffffffff166040518263ffffffff1660e01b815260040161284a919064ffffffffff91909116815260200190565b602060405180830381865afa158015612867573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061288b919061511f565b1561290e57602381815481106128a3576128a36150ec565b90600052602060002090600691828204019190066005029054906101000a900464ffffffffff168483815181106128dc576128dc6150ec565b64ffffffffff90921660209283029190910190910152826128fc816152e3565b935050818061290a906152e3565b9250505b6001016127c6565b50508152919050565b60606021805461292e90615293565b80601f016020809104026020016040519081016040528092919081815260200182805461295a90615293565b80156108ae5780601f1061297c576101008083540402835291602001916108ae565b820191906000526020600020905b81548152906001019060200180831161298a57509395945050505050565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156129fd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a219190614e30565b50612a866040518060400160405280600a81526020016964656c6567617465546f60b01b815250826001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156112e3573d6000803e3d6000fd5b60408051808201825260608152600060208201819052601c54925163eea9064b60e01b815291926001600160a01b03169163eea9064b91612acd91869186916004016152fc565b600060405180830381600087803b158015612ae757600080fd5b505af1158015612afb573d6000803e3d6000fd5b505050505050565b60606010805480602002602001604051908101604052809291908181526020016000905b82821015611d6c578382906000526020600020018054612b4690615293565b80601f0160208091040260200160405190810160405280929190818152602001828054612b7290615293565b8015612bbf5780601f10612b9457610100808354040283529160200191612bbf565b820191906000526020600020905b815481529060010190602001808311612ba257829003601f168201915b505050505081526020019060010190612b27565b60008054610100900460ff1615612bf35750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156112045760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091612c81917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc49160800161533d565b60408051601f1981840301815290829052612c9b9161536e565b6000604051808303816000865af19150503d8060008114612cd8576040519150601f19603f3d011682016040523d82523d6000602084013e612cdd565b606091505b5091505080806020019051810190612cf5919061511f565b9392505050565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015612d51573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d759190614e30565b50612da96040518060400160405280601281526020017118dbdb5c1b195d1950da1958dadc1bda5b9d60721b815250612ecf565b612388613018565b6060600c8054806020026020016040519081016040528092919081815260200182805480156108ae576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610890575050505050905090565b60606000601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015612e6a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e8e9190614e30565b50612ebf6040518060400160405280600f81526020016e737461727456616c696461746f727360881b815250612ecf565b612ec7613b5e565b915091509091565b600080516020615d43833981519152612eee612ee961291f565b61401d565b612ef783614046565b604051602001612f0892919061538a565b60408051601f1981840301815290829052612f2291614dd9565b60405180910390a150565b805115612f3c57805181602001fd5b60405162461bcd60e51b815260206004820152601b60248201527f7265766572746564207769746820756e6b6e6f776e206572726f72000000000060448201526064016118b2565b6022546040516388676cad60e01b8152600060048201526001600160a01b03909116906388676cad90602401600060405180830381600087803b158015612fca57600080fd5b505af1925050508015612fdb575060015b612388573d808015613009576040519150601f19603f3d011682016040523d82523d6000602084013e61300e565b606091505b5061220381612f2d565b604080518082018252601881527f2d206163746976652076616c696461746f7220636f756e7400000000000000006020808301919091526022548351632340e8d360e01b815293516130bf946001600160a01b0390921692632340e8d392600480820193918290030181865afa158015613096573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906130ba9190614e30565b61406e565b60408051808201825260128152712d2070726f6f66732072656d61696e696e6760701b602082015260225482516323e941b960e11b81529251613164936001600160a01b03909216916347d283729160048083019260a09291908290030181865afa158015613132573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061315691906153c6565b6020015162ffffff1661406e565b602254604080516321767f9560e11b815290516000926001600160a01b0316916342ecff2a9160048083019260209291908290030181865afa1580156131ae573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131d29190615444565b9050806001600160401b03166000036132465760405162461bcd60e51b815260206004820152603060248201527f557365722e5f636f6d706c657465436865636b706f696e743a206e6f2065786960448201526f1cdd1a5b99c818da1958dadc1bda5b9d60821b60648201526084016118b2565b60205460405163b1b6f6a160e01b81526000916001600160a01b03169063b1b6f6a19061327a90602390869060040161545f565b600060405180830381865afa158015613297573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526132bf91908101906155c6565b90506132e8604051806060016040528060228152602001615dc06022913982602001515161406e565b6022548151602083015160405163783a5d3160e11b81526001600160a01b039093169263f074ba6292612acd92909160040161572b565b81831461081a577f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf35838160405161335591906157c3565b60405180910390a161081a838361408d565b606060008360a00151516001600160401b03811115613388576133886143c5565b6040519080825280602002602001820160405280156133b1578160200160208202803683370190505b50905060005b81518110156136255760008560a0015182815181106133d8576133d86150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316816001600160a01b0316036135875773beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0838381518110613436576134366150ec565b60200260200101906001600160a01b031690816001600160a01b03168152505084156135825761347d604051806060016040528060328152602001615d8e6032913961417e565b61348d613488612774565b613a16565b50602060009054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156134de57600080fd5b505af11580156134f2573d6000803e3d6000fd5b505050506134fe612f84565b602260009054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa158015613551573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906135759190614e30565b1561358257613582613018565b61361c565b806001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156135c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906135e99190615102565b8383815181106135fb576135fb6150ec565b60200260200101906001600160a01b031690816001600160a01b0316815250505b506001016133b7565b50601c54604051630e4cc3f960e41b81526001600160a01b039091169063e4cc3f909061365a908790859088906004016157f2565b600060405180830381600087803b15801561367457600080fd5b505af1158015613688573d6000803e3d6000fd5b50929695505050505050565b600080516020615d438339815191526136ae612ee961291f565b6136b784614046565b836040516020016136ca9392919061582a565b60408051601f19818403018152908290526136e491614dd9565b60405180910390a15050565b601c546040516366d5ba9360e01b81526001600160a01b0383811660048301526060926000928392909116906366d5ba9390602401600060405180830381865afa158015613742573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261376a9190810190615889565b91509150600082516001600160401b03811115613789576137896143c5565b6040519080825280602002602001820160405280156137c257816020015b6137af614337565b8152602001906001900390816137a75790505b50601c54604051631976849960e21b81526001600160a01b038881166004830152929350600092909116906365da126490602401602060405180830381865afa158015613813573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906138379190615102565b601c5460405163285e212160e21b81526001600160a01b0389811660048301529293506000929091169063a178848490602401602060405180830381865afa158015613887573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906138ab9190614e30565b905060005b8551811015613a0a57604080516001808252818301909252600091602080830190803683375050604080516001808252818301909252929350600092915060208083019080368337019050509050878381518110613910576139106150ec565b60200260200101518260008151811061392b5761392b6150ec565b60200260200101906001600160a01b031690816001600160a01b03168152505086838151811061395d5761395d6150ec565b602002602001015181600081518110613978576139786150ec565b6020026020010181815250506040518060e001604052808b6001600160a01b03168152602001866001600160a01b031681526020018b6001600160a01b0316815260200184866139c8919061594a565b81526020014263ffffffff168152602001838152602001828152508684815181106139f5576139f56150ec565b602090810291909101015250506001016138b0565b50919695505050505050565b6000613a586040518060400160405280601881526020017f2d2065786974696e67206e756d2076616c696461746f72730000000000000000815250835161406e565b60005b8251811015613b155760205483516001600160a01b039091169063f8f98a4e90859084908110613a8d57613a8d6150ec565b60200260200101516040518263ffffffff1660e01b8152600401613abe919064ffffffffff91909116815260200190565b6020604051808303816000875af1158015613add573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613b019190615444565b613b0b908361595d565b9150600101613a5b565b506112046040518060400160405280601e81526020017f2d206578697465642062616c616e636520746f20706f64202867776569290000815250826001600160401b031661406e565b606060004781613b776801bc16d674ec80000083615992565b9050613b8c816801bc16d674ec8000006159a6565b613b9690836159bd565b9150600081670de0b6b3a76400008410613bde57613bb8633b9aca00856159d0565b613bc290856159bd565b9150613bce82856159bd565b935080613bda816152e3565b9150505b80600003613c4b5760405162461bcd60e51b815260206004820152603460248201527f737461727456616c696461746f72733a206e6f7420656e6f75676820455448206044820152733a379039ba30b93a1030903b30b634b230ba37b960611b60648201526084016118b2565b6000816001600160401b03811115613c6557613c656143c5565b604051908082528060200260200182016040528015613c8e578160200160208202803683370190505b5090506000633b9aca00613ca287476159bd565b613cac9190615992565b9050613cee6040518060400160405280601981526020017f2d206372656174696e67206e65772076616c696461746f727300000000000000815250835161406e565b613d196040518060600160405280602b8152602001615d63602b9139826001600160401b031661406e565b60005b85811015613e34576020546000906001600160a01b031663ed3c16056801bc16d674ec800000613d4a61419b565b6040518363ffffffff1660e01b8152600401613d669190614dd9565b60206040518083038185885af1158015613d84573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190613da991906159e4565b905080848381518110613dbe57613dbe6150ec565b64ffffffffff9283166020918202929092010152602380546001818101835560009290925260068082047fd57b2b5166478fd4318d2acc6cc2c704584312bdd8781b32d5d06abda57f4230018054958516600592909306919091026101000a918202919093021990931692909217905501613d1c565b50613e4085600161594a565b8303613f54576020546000906001600160a01b031663ed3c160586613e6361419b565b6040518363ffffffff1660e01b8152600401613e7f9190614dd9565b60206040518083038185885af1158015613e9d573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190613ec291906159e4565b9050808360018551613ed491906159bd565b81518110613ee457613ee46150ec565b64ffffffffff92831660209182029290920101526023805460018101825560009190915260068082047fd57b2b5166478fd4318d2acc6cc2c704584312bdd8781b32d5d06abda57f4230018054948416600592909306919091026101000a91820291909202199092169190911790555b9097909650945050505050565b6020546040516352851d0d60e11b81526000916001600160a01b03169063a50a3a1a90613f92908590600401614dc6565b600060405180830381865afa158015613faf573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052613fd79190810190615a83565b6022548151602083015160408085015160608601519151633f65cf1960e01b81529596506001600160a01b0390941694633f65cf19946107b29493928992600401615c2e565b6060610850604051806040016040528060058152602001641b5b39366d60d81b815250836141e1565b6060610850604051806040016040528060048152602001631b5b336d60e01b815250836141e1565b600080516020615de283398151915282826040516136e4929190615cdd565b80821461081c57600080516020615d438339815191526040516140ec9060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b6060820152602081018490529051600080516020615de28339815191529181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b6060820152602081018390529051600080516020615de28339815191529181900360800190a161081c61422b565b600080516020615d4383398151915281604051612f229190614dd9565b60225460408051600160f81b6020820152600060218201526bffffffffffffffffffffffff19606093841b16602c82015201604051602081830303815290604052905090565b60608282604051806040016040528060048152602001631b5b306d60e01b81525060405160200161421493929190615cff565b604051602081830303815290604052905092915050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156143265760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526142c5929160200161533d565b60408051601f19818403018152908290526142df9161536e565b6000604051808303816000865af19150503d806000811461431c576040519150601f19603f3d011682016040523d82523d6000602084013e614321565b606091505b505050505b6000805461ff001916610100179055565b6040518060e0016040528060006001600160a01b0316815260200160006001600160a01b0316815260200160006001600160a01b0316815260200160008152602001600063ffffffff16815260200160608152602001606081525090565b64ffffffffff8116811461220357600080fd5b6000602082840312156143ba57600080fd5b8135612cf581614395565b634e487b7160e01b600052604160045260246000fd5b60405160e081016001600160401b03811182821017156143fd576143fd6143c5565b60405290565b604080519081016001600160401b03811182821017156143fd576143fd6143c5565b604051606081016001600160401b03811182821017156143fd576143fd6143c5565b60405160a081016001600160401b03811182821017156143fd576143fd6143c5565b604051608081016001600160401b03811182821017156143fd576143fd6143c5565b604051601f8201601f191681016001600160401b03811182821017156144b3576144b36143c5565b604052919050565b60006001600160401b038211156144d4576144d46143c5565b50601f01601f191660200190565b600080604083850312156144f557600080fd5b8235915060208301356001600160401b0381111561451257600080fd5b8301601f8101851361452357600080fd5b8035614536614531826144bb565b61448b565b81815286602083850101111561454b57600080fd5b816020840160208301376000602083830101528093505050509250929050565b602080825282518282018190526000918401906040840190835b818110156145ac5783516001600160a01b0316835260209384019390920191600101614585565b509095945050505050565b60006001600160401b038211156145d0576145d06143c5565b5060051b60200190565b6001600160a01b038116811461220357600080fd5b600082601f83011261460057600080fd5b813561460e614531826145b7565b8082825260208201915060208360051b86010192508583111561463057600080fd5b602085015b83811015614656578035614648816145da565b835260209283019201614635565b5095945050505050565b6000806040838503121561467357600080fd5b82356001600160401b0381111561468957600080fd5b614695858286016145ef565b92505060208301356001600160401b038111156146b157600080fd5b8301601f810185136146c257600080fd5b80356146d0614531826145b7565b8082825260208201915060208360051b8501019250878311156146f257600080fd5b6020840193505b828410156147145783358252602093840193909101906146f9565b809450505050509250929050565b600082601f83011261473357600080fd5b8135614741614531826145b7565b8082825260208201915060208360051b86010192508583111561476357600080fd5b602085015b83811015614656578035835260209283019201614768565b6000806040838503121561479357600080fd5b82356001600160401b038111156147a957600080fd5b6147b5858286016145ef565b92505060208301356001600160401b038111156147d157600080fd5b6147dd85828601614722565b9150509250929050565b600081518084526020840193506020830160005b828110156148225781516001600160a01b03168652602095860195909101906001016147fb565b5093949350505050565b600081518084526020840193506020830160005b82811015614822578151865260209586019590910190600101614840565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201516000916148aa9085018263ffffffff169052565b5060a082015160e060a08501526148c460e08501826147e7565b905060c083015184820360c08601526148dd828261482c565b95945050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f1987860301845261492a85835161485e565b9450602093840193919091019060010161490e565b8035611204816145da565b803563ffffffff8116811461120457600080fd5b600060e0828403121561497057600080fd5b6149786143db565b90506149838261493f565b81526149916020830161493f565b60208201526149a26040830161493f565b6040820152606082810135908201526149bd6080830161494a565b608082015260a08201356001600160401b038111156149db57600080fd5b6149e7848285016145ef565b60a08301525060c08201356001600160401b03811115614a0657600080fd5b614a1284828501614722565b60c08301525092915050565b600060208284031215614a3057600080fd5b81356001600160401b03811115614a4657600080fd5b8201601f81018413614a5757600080fd5b8035614a65614531826145b7565b8082825260208201915060208360051b850101925086831115614a8757600080fd5b602084015b83811015614ac85780356001600160401b03811115614aaa57600080fd5b614ab98960208389010161495e565b84525060209283019201614a8c565b509695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f19878603018452614b178583516147e7565b94506020938401939190910190600101614afb565b600060208284031215614b3e57600080fd5b8135612cf5816145da565b600060208284031215614b5b57600080fd5b81356001600160401b03811115614b7157600080fd5b8201601f81018413614b8257600080fd5b8035614b90614531826145b7565b8082825260208201915060208360051b850101925086831115614bb257600080fd5b6020840193505b82841015614bdd578335614bcc81614395565b825260209384019390910190614bb9565b9695505050505050565b600060208284031215614bf957600080fd5b81356001600160401b03811115614c0f57600080fd5b614c1b8482850161495e565b949350505050565b602081526000612cf560208301846147e7565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b80831015614ccc5783516001600160e01b03191682526020938401936001939093019290910190614ca0565b50965050506020938401939190910190600101614c5e565b60005b83811015614cff578181015183820152602001614ce7565b50506000910152565b60008151808452614d20816020860160208601614ce4565b601f01601f19169290920160200192915050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f19878603018452614d78858351614d08565b94506020938401939190910190600101614d5c565b600081518084526020840193506020830160005b8281101561482257815164ffffffffff16865260209586019590910190600101614da1565b602081526000612cf56020830184614d8d565b602081526000612cf56020830184614d08565b600060208284031215614dfe57600080fd5b5035919050565b604081526000614e186040830185614d8d565b90506001600160401b03831660208301529392505050565b600060208284031215614e4257600080fd5b5051919050565b80516001600160401b038116811461120457600080fd5b6000614e6e614531846144bb565b9050828152838383011115614e8257600080fd5b612cf5836020830184614ce4565b600082601f830112614ea157600080fd5b612cf583835160208501614e60565b600060408284031215614ec257600080fd5b614eca614403565b8251815260208301519091506001600160401b03811115614eea57600080fd5b614ef684828501614e90565b60208301525092915050565b600082601f830112614f1357600080fd5b8151614f21614531826145b7565b8082825260208201915060208360051b860101925085831115614f4357600080fd5b602085015b83811015614656578051835260209283019201614f48565b600060208284031215614f7257600080fd5b81516001600160401b03811115614f8857600080fd5b820160608185031215614f9a57600080fd5b614fa2614425565b614fab82614e49565b815260208201516001600160401b03811115614fc657600080fd5b614fd286828501614eb0565b60208301525060408201516001600160401b03811115614ff157600080fd5b91909101906040828603121561500657600080fd5b61500e614403565b82516001600160401b0381111561502457600080fd5b61503087828601614f02565b82525060208301516001600160401b0381111561504c57600080fd5b61505887828601614e90565b6020830152506040820152949350505050565b805182526000602082015160406020850152614c1b6040850182614d08565b6001600160401b03841681526060602082015260006150ac606083018561506b565b82810360408401528351604082526150c7604083018261482c565b9050602085015182820360208401526150e08282614d08565b98975050505050505050565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561511457600080fd5b8151612cf5816145da565b60006020828403121561513157600080fd5b81518015158114612cf557600080fd5b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f19878603018452815180516060875261518f60608801826147e7565b9050602082015187820360208901526151a8828261482c565b6040938401516001600160a01b031698909301979097525094506020938401939190910190600101615169565b6000602082840312156151e757600080fd5b81516001600160401b038111156151fd57600080fd5b614c1b84828501614f02565b60006020828403121561521b57600080fd5b81516001600160401b0381111561523157600080fd5b8201601f8101841361524257600080fd5b614c1b84825160208401614e60565b6001600160a01b038781168252868116602083015260408201869052841660608201526080810183905260c060a082018190526000906150e090830184614d08565b600181811c908216806152a757607f821691505b6020821081036152c757634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b6000600182016152f5576152f56152cd565b5060010190565b60018060a01b038416815260606020820152600083516040606084015261532660a0840182614d08565b602095909501516080840152505060400152919050565b6001600160e01b0319831681528151600090615360816004850160208701614ce4565b919091016004019392505050565b60008251615380818460208701614ce4565b9190910192915050565b6000835161539c818460208801614ce4565b601760f91b90830190815283516153ba816001840160208801614ce4565b01600101949350505050565b600060a08284031280156153d957600080fd5b5060006153e4614447565b83518152602084015162ffffff811681146153fd578283fd5b602082015261540e60408501614e49565b604082015260608401518060070b8114615426578283fd5b606082015261543760808501614e49565b6080820152949350505050565b60006020828403121561545657600080fd5b612cf582614e49565b6000604082016040835280855461547a818490815260200190565b60008881526020812094509092505b816005820110156154f057835464ffffffffff8082168552602882901c81166020860152605082901c81166040860152607882901c8116606086015260a082811c8216608087015260c89290921c169084015260019093019260c090920191600601615489565b9254928181101561550f5764ffffffffff841683526020909201916001015b8181101561552f5764ffffffffff602885901c1683526020909201916001015b8181101561554f5764ffffffffff605085901c1683526020909201916001015b8181101561556f5764ffffffffff607885901c1683526020909201916001015b8181101561558f5764ffffffffff60a085901c1683526020909201916001015b818110156155ac5760c884901c64ffffffffff1683526020830192505b50506001600160401b03851660208501529150612cf59050565b6000602082840312156155d857600080fd5b81516001600160401b038111156155ee57600080fd5b82016040818503121561560057600080fd5b615608614403565b81516001600160401b0381111561561e57600080fd5b61562a86828501614eb0565b82525060208201516001600160401b0381111561564657600080fd5b80830192505084601f83011261565b57600080fd5b8151615669614531826145b7565b8082825260208201915060208360051b86010192508783111561568b57600080fd5b602085015b8381101561571a5780516001600160401b038111156156ae57600080fd5b86016060818b03601f190112156156c457600080fd5b6156cc614425565b602082810151825260408301519082015260608201516001600160401b038111156156f657600080fd5b6157058c602083860101614e90565b60408301525084525060209283019201615690565b506020840152509095945050505050565b60408152600061573e604083018561506b565b828103602084015280845180835260208301915060208160051b8401016020870160005b838110156157b557601f1986840301855281518051845260208101516020850152604081015190506060604085015261579e6060850182614d08565b602096870196909450929092019150600101615762565b509098975050505050505050565b60408152600560408201526422b93937b960d91b6060820152608060208201526000612cf56080830184614d08565b606081526000615805606083018661485e565b828103602084015261581781866147e7565b9150508215156040830152949350505050565b6000845161583c818460208901614ce4565b601760f91b908301908152845161585a816001840160208901614ce4565b601d60f91b60019290910191820152835161587c816002840160208801614ce4565b0160020195945050505050565b6000806040838503121561589c57600080fd5b82516001600160401b038111156158b257600080fd5b8301601f810185136158c357600080fd5b80516158d1614531826145b7565b8082825260208201915060208360051b8501019250878311156158f357600080fd5b6020840193505b8284101561591e57835161590d816145da565b8252602093840193909101906158fa565b8095505050505060208301516001600160401b0381111561593e57600080fd5b6147dd85828601614f02565b80820180821115610850576108506152cd565b6001600160401b038181168382160190811115610850576108506152cd565b634e487b7160e01b600052601260045260246000fd5b6000826159a1576159a161597c565b500490565b8082028115828204841417610850576108506152cd565b81810381811115610850576108506152cd565b6000826159df576159df61597c565b500690565b6000602082840312156159f657600080fd5b8151612cf581614395565b600082601f830112615a1257600080fd5b8151615a20614531826145b7565b8082825260208201915060208360051b860101925085831115615a4257600080fd5b602085015b838110156146565780516001600160401b03811115615a6557600080fd5b615a74886020838a0101614f02565b84525060209283019201615a47565b600060208284031215615a9557600080fd5b81516001600160401b03811115615aab57600080fd5b820160808185031215615abd57600080fd5b615ac5614469565b615ace82614e49565b815260208201516001600160401b03811115615ae957600080fd5b615af586828501614eb0565b60208301525060408201516001600160401b03811115615b1457600080fd5b8201601f81018613615b2557600080fd5b8051615b33614531826145b7565b8082825260208201915060208360051b850101925088831115615b5557600080fd5b602084015b83811015615b965780516001600160401b03811115615b7857600080fd5b615b878b602083890101614e90565b84525060209283019201615b5a565b50604085015250505060608201516001600160401b03811115615bb857600080fd5b615bc486828501615a01565b606083015250949350505050565b600082825180855260208501945060208160051b8301016020850160005b83811015615c2257601f19858403018852615c0c83835161482c565b6020988901989093509190910190600101615bf0565b50909695505050505050565b6001600160401b038616815260a060208201526000615c5060a083018761506b565b8281036040840152615c628187614d8d565b9050828103606084015280855180835260208301915060208160051b8401016020880160005b83811015615cba57601f19868403018552615ca4838351614d08565b6020958601959093509190910190600101615c88565b50508581036080870152615cce8188615bd2565b9b9a5050505050505050505050565b604081526000615cf06040830185614d08565b90508260208301529392505050565b60008451615d11818460208901614ce4565b845190830190615d25818360208901614ce4565b8451910190615d38818360208801614ce4565b019594505050505056fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f502d206465706f736974696e672062616c616e636520746f20626561636f6e20636861696e202867776569292d2065786974696e6720616c6c2076616c696461746f727320616e6420636f6d706c6574696e6720636865636b706f696e742d207375626d697474696e67206e756d20636865636b706f696e742070726f6f6673b2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8557365722e71756575655769746864726177616c733a206c656e677468206d69736d61746368a2646970667358221220bc4f9e60413b69f34b8d56f3337e68c7bbb931fa94fbc14959d7c5aee9e988fd64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x82\x17\x90U`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x17\x90U`$\x80Tc\xFF\xFF\xFF\xFF\x19\x16\x90\x91\x17\x90U4\x80\x15a\0bW`\0\x80\xFD[P`@Qaf 8\x03\x80af \x839\x81\x01`@\x81\x90Ra\0\x81\x91a\x05eV[\x80\x80`\x003\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xEAM<\x9B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xEA\x91\x90a\x06IV[`\x1C`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c9\xB7\x0E8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01r\x91\x90a\x06IV[`\x1D`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16cFe\xBC\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFA\x91\x90a\x06IV[`\x1E`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c=\xFB@\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x82\x91\x90a\x06IV[`\x1F`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16c\"\xC05\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\n\x91\x90a\x06IV[` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x032a\x04YV[`!a\x03>\x83\x82a\x06\xF6V[PPP`\x003\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c9\xB7\x0E8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xA8\x91\x90a\x06IV[`$`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16cFe\xBC\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x040\x91\x90a\x06IV[`%\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa\x07\xB4\x91PPV[`\x1E`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x84\xD8\x10b`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xBDW=`\0\x80>=`\0\xFD[PP`\x1ET`@Qc\x9B\xA0bu`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\x9B\xA0bu\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05-\x91\x90a\x06IV[`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x05wW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\x8DW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x05\x9EW`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\xB7Wa\x05\xB7a\x05OV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x05\xE5Wa\x05\xE5a\x05OV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x05\xFDW`\0\x80\xFD[`\0[\x82\x81\x10\x15a\x06\x1CW` \x81\x85\x01\x81\x01Q\x83\x83\x01\x82\x01R\x01a\x06\0V[P`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06FW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x06[W`\0\x80\xFD[\x81Qa\x06f\x81a\x061V[\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\x81W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x06\xA1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x06\xF1W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x06\xCEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x06\xEEW`\0\x81U`\x01\x01a\x06\xDAV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\x0FWa\x07\x0Fa\x05OV[a\x07#\x81a\x07\x1D\x84Ta\x06mV[\x84a\x06\xA7V[` `\x1F\x82\x11`\x01\x81\x14a\x07WW`\0\x83\x15a\x07?WP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x06\xEEV[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x07\x87W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x07gV[P\x84\x82\x10\x15a\x07\xA5W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[a^]\x80a\x07\xC3`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x01\xF2W`\x005`\xE0\x1C\x80cm3oX\x11a\x01\rW\x80c\xA8\x8D\xBB6\x11a\0\xA0W\x80c\xBAAO\xA6\x11a\0oW\x80c\xBAAO\xA6\x14a\x05\xD2W\x80c\xD6\xC1\r\xAF\x14a\x05\xE7W\x80c\xE2\x0C\x9Fq\x14a\x05\xFCW\x80c\xF24\xC1\xBD\x14a\x06\x11W\x80c\xFAv&\xD4\x14a\x064W`\0\x80\xFD[\x80c\xA8\x8D\xBB6\x14a\x05%W\x80c\xACc|z\x14a\x05]W\x80c\xB5P\x8A\xA9\x14a\x05}W\x80c\xB7\x17#U\x14a\x05\x92W`\0\x80\xFD[\x80c\x91j\x17\xC6\x11a\0\xDCW\x80c\x91j\x17\xC6\x14a\x04\xB7W\x80c\x92\xAB\x89\xBB\x14a\x04\xCCW\x80c\x9D\xE7\x02X\x14a\x04\xE1W\x80c\xA3\xF4\xDF~\x14a\x05\x03W`\0\x80\xFD[\x80cm3oX\x14a\x04@W\x80c\x84\x1C\x12\x99\x14a\x04`W\x80c\x85\"l\x81\x14a\x04\x80W\x80c\x90\xB5\x16%\x14a\x04\xA2W`\0\x80\xFD[\x80c=\x8C\x08\xD4\x11a\x01\x85W\x80cF\xA5\xBE\r\x11a\x01TW\x80cF\xA5\xBE\r\x14a\x03\xBEW\x80ceK\xB5\xD9\x14a\x03\xDEW\x80cf\xD9\xA9\xA0\x14a\x03\xFEW\x80ci_J\xE1\x14a\x04 W`\0\x80\xFD[\x80c=\x8C\x08\xD4\x14a\x03/W\x80c>^<#\x14a\x03gW\x80c?r\x86\xF4\x14a\x03|W\x80c@\x1B\xE6^\x14a\x03\x91W`\0\x80\xFD[\x80c#\xE4\x81u\x11a\x01\xC1W\x80c#\xE4\x81u\x14a\x02\xA0W\x80c*4\xAD\xE8\x14a\x02\xCDW\x80c4N\x13\x83\x14a\x02\xE2W\x80c9\x1C\xC9\xF6\x14a\x03\x0FW`\0\x80\xFD[\x80c\x07\x1C%\xB7\x14a\x01\xFEW\x80c\x16&\xBA~\x14a\x02 W\x80c\x1E\xD7\x83\x1C\x14a\x02^W\x80c \xA5E\xD9\x14a\x02\x80W`\0\x80\xFD[6a\x01\xF9W\0[`\0\x80\xFD[4\x80\x15a\x02\nW`\0\x80\xFD[Pa\x02\x1Ea\x02\x196`\x04aC\xA8V[a\x06NV[\0[4\x80\x15a\x02,W`\0\x80\xFD[Pa\x02@a\x02;6`\x04aD\xE2V[a\x08 V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW`\0\x80\xFD[Pa\x02sa\x08VV[`@Qa\x02U\x91\x90aEkV[4\x80\x15a\x02\x8CW`\0\x80\xFD[Pa\x02\x1Ea\x02\x9B6`\x04aF`V[a\x08\xB8V[4\x80\x15a\x02\xACW`\0\x80\xFD[Pa\x02\xC0a\x02\xBB6`\x04aG\x80V[a\x0B\xD7V[`@Qa\x02U\x91\x90aH\xE6V[4\x80\x15a\x02\xD9W`\0\x80\xFD[Pa\x02\x1Ea\x0F7V[4\x80\x15a\x02\xEEW`\0\x80\xFD[Pa\x03\x02a\x02\xFD6`\x04aJ\x1EV[a\x10\xABV[`@Qa\x02U\x91\x90aJ\xD3V[4\x80\x15a\x03\x1BW`\0\x80\xFD[Pa\x02\xC0a\x03*6`\x04aK,V[a\x12\tV[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x03Oa\x03J6`\x04aKIV[a\x13\x91V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x03sW`\0\x80\xFD[Pa\x02sa\x149V[4\x80\x15a\x03\x88W`\0\x80\xFD[Pa\x02sa\x14\x99V[4\x80\x15a\x03\x9DW`\0\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04aK\xE7V[a\x14\xF9V[`@Qa\x02U\x91\x90aL#V[4\x80\x15a\x03\xCAW`\0\x80\xFD[Pa\x03\x02a\x03\xD96`\x04aJ\x1EV[a\x15\xB3V[4\x80\x15a\x03\xEAW`\0\x80\xFD[Pa\x02\x1Ea\x03\xF96`\x04aG\x80V[a\x17\x08V[4\x80\x15a\x04\nW`\0\x80\xFD[Pa\x04\x13a\x1C\x86V[`@Qa\x02U\x91\x90aL6V[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x03\xB1a\x04;6`\x04aK\xE7V[a\x1DuV[4\x80\x15a\x04LW`\0\x80\xFD[Pa\x02\x1Ea\x04[6`\x04aG\x80V[a\x1E/V[4\x80\x15a\x04lW`\0\x80\xFD[Pa\x02\x1Ea\x04{6`\x04aKIV[a!BV[4\x80\x15a\x04\x8CW`\0\x80\xFD[Pa\x04\x95a\"\x06V[`@Qa\x02U\x91\x90aM4V[4\x80\x15a\x04\xAEW`\0\x80\xFD[Pa\x02\x1Ea\"\xD6V[4\x80\x15a\x04\xC3W`\0\x80\xFD[Pa\x04\x13a#\x8AV[4\x80\x15a\x04\xD8W`\0\x80\xFD[Pa\x02\xC0a$pV[4\x80\x15a\x04\xEDW`\0\x80\xFD[Pa\x04\xF6a'tV[`@Qa\x02U\x91\x90aM\xC6V[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x05\x18a)\x1FV[`@Qa\x02U\x91\x90aM\xD9V[4\x80\x15a\x051W`\0\x80\xFD[P`\"Ta\x05E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x05iW`\0\x80\xFD[Pa\x02\x1Ea\x05x6`\x04aK,V[a)\xA8V[4\x80\x15a\x05\x89W`\0\x80\xFD[Pa\x04\x95a+\x03V[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x05\xC2a\x05\xAD6`\x04aM\xECV[`&` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02UV[4\x80\x15a\x05\xDEW`\0\x80\xFD[Pa\x05\xC2a+\xD3V[4\x80\x15a\x05\xF3W`\0\x80\xFD[Pa\x02\x1Ea,\xFCV[4\x80\x15a\x06\x08W`\0\x80\xFD[Pa\x02sa-\xB1V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x06&a.\x11V[`@Qa\x02U\x92\x91\x90aN\x05V[4\x80\x15a\x06@W`\0\x80\xFD[P`\0Ta\x05\xC2\x90`\xFF\x16\x81V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90aN0V[Pa\x06\xFB`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qverifyStaleBalance`p\x1B\x81RPa.\xCFV[` T`@Qc\x08\xFA\x0B\x13`\xE2\x1B\x81Rd\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xE8,L\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07s\x91\x90\x81\x01\x90aO`V[`\"T\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x01\xC8\xAB\xE9`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93c\x03\x91W\xD2\x93a\x07\xB2\x93\x92\x91`\x04\x01aP\x8AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xCCW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x07\xDDWP`\x01[a\x08\x1CW=\x80\x80\x15a\x08\x0BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x10V[``\x91P[Pa\x08\x1A\x81a/-V[P[PPV[`\0\x82\x81R`&` R`@\x81 T`\xFF\x16\x15a\x08EWPc\x0B\x13]?`\xE1\x1Ba\x08PV[P`\x01`\x01`\xE0\x1B\x03\x19[\x92\x91PPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90W[PPPPP\x90P\x90V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t1\x91\x90aN0V[Pa\ta`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mupdateBalances`\x90\x1B\x81RPa.\xCFV[`\0[\x82Q\x81\x10\x15a\x08\x1AW`\0\x83\x82\x81Q\x81\x10a\t\x81Wa\t\x81aP\xECV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\t\x9FWa\t\x9FaP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\ngWa\t\xDEa/\x84V[`\"`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nU\x91\x90aN0V[\x15a\nbWa\nba0\x18V[a\x0B\xCDV[`\0\x81\x90P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD0\x91\x90aQ\x02V[`\x1DT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BI\x91\x90aQ\x1FV[P`\x1DT`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC9\x91\x90aN0V[PPP[PP`\x01\x01a\tdV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x0C#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CG\x91\x90aN0V[Pa\x0Cy`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oqueueWithdrawals`\x80\x1B\x81RPa.\xCFV[`\x1CT`@Qc\x19v\x84\x99`\xE2\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE6\x91\x90aQ\x02V[`\x1CT`@Qc(^!!`\xE2\x1B\x81R0`\x04\x82\x01\x81\x90R\x92\x93P`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rX\x91\x90aN0V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rsW\x90PP\x90P`@Q\x80``\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81`\0\x81Q\x81\x10a\r\xDBWa\r\xDBaP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[a\x0E\x05aC7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xFDW\x90PP\x90P`@Q\x80`\xE0\x01`@R\x800`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81R` \x01Bc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x88\x81RP\x81`\0\x81Q\x81\x10a\x0E\x81Wa\x0E\x81aP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1CT`@Qc\x06\xECn\x81`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\r\xD8\xDD\x02\x90a\x0E\xBD\x90\x86\x90`\x04\x01aQAV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x04\x91\x90\x81\x01\x90aQ\xD5V[\x90Pa\x0F+\x82Q\x82Q`@Q\x80``\x01`@R\x80`&\x81R` \x01a^\x02`&\x919a3\x1FV[P\x97\x96PPPPPPPV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB0\x91\x90aN0V[Pa\x0F\xE4`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q92\xB3\xB4\xB9\xBA2\xB9 \xB9\xA7\xB82\xB90\xBA7\xB9`q\x1B\x81RPa.\xCFV[`@\x80Q``\x81\x01\x82R0\x81R`\0` \x82\x01\x81\x81R\x82\x84\x01\x91\x82R`\x1CT`$\x80T\x95Qc\x02K\x98\x03`\xE5\x1B\x81R\x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x93Q\x84\x16\x91\x81\x01\x91\x90\x91R\x92Qc\xFF\xFF\xFF\xFF\x90\x81\x16`D\x85\x01R\x90\x94\x16`d\x83\x01R`\xA0`\x84\x83\x01R`\x08`\xA4\x83\x01Rgmetadata`\xC0\x1B`\xC4\x83\x01R\x91\x92\x91\x90\x91\x16\x90cIs\0`\x90`\xE4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x90W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xA4W=`\0\x80>=`\0\xFD[PPPPPV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x10\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x1B\x91\x90aN0V[Pa\x11Z`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa.\xCFV[`\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11uWa\x11uaC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xA8W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\x93W\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12\0Wa\x11\xDB\x84\x82\x81Q\x81\x10a\x11\xCCWa\x11\xCCaP\xECV[` \x02` \x01\x01Q`\x01a3gV[\x82\x82\x81Q\x81\x10a\x11\xEDWa\x11\xEDaP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x11\xAEV[P\x90P[\x91\x90PV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x12UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12y\x91\x90aN0V[Pa\x13\x10`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nforceUndelegate`\x88\x1B\x81RP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\x0B\x91\x90\x81\x01\x90aR\tV[a6\x94V[`\0a\x13\x1B\x83a6\xF0V[`\x1CT`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xDA\x8B\xE8d\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\0\x91\x90\x81\x01\x90aQ\xD5V[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\x13\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\0\x91\x90aN0V[Pa\x140`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mexitValidators`\x90\x1B\x81RPa.\xCFV[a\x08P\x82a:\x16V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90WPPPPP\x90P\x90V[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x15EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15i\x91\x90aN0V[Pa\x15\xA8`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa.\xCFV[a\x08P\x82`\x01a3gV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16#\x91\x90aN0V[Pa\x16b`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa.\xCFV[`\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16}Wa\x16}aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xB0W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\x9BW\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12\0Wa\x16\xE3\x84\x82\x81Q\x81\x10a\x16\xD4Wa\x16\xD4aP\xECV[` \x02` \x01\x01Q`\0a3gV[\x82\x82\x81Q\x81\x10a\x16\xF5Wa\x16\xF5aP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x16\xB6V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x81\x91\x90aN0V[Pa\x17\xC0`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F.depositIntoEigenlayer_M1_ALT\0\0\0\x81RPa.\xCFV[`\0\x19`\0[\x83Q\x81\x10\x15a\x1C\x80W`\0\x84\x82\x81Q\x81\x10a\x17\xE3Wa\x17\xE3aP\xECV[` \x02` \x01\x01Q\x90P`\0\x84\x83\x81Q\x81\x10a\x18\x01Wa\x18\x01aP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x18\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FShould not be depositing with BE`D\x82\x01R\x7FACONCHAIN_ETH_STRAT for M1-mainn`d\x82\x01Rf2\xBA\x10*\xB9\xB2\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x1F\x91\x90aQ\x02V[`\x1DT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x98\x91\x90aQ\x1FV[P`\x1DT`@Qb?g_`\xE9\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c~\xCE\xBE\0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x05\x91\x90aN0V[\x90P`\0`\x1D`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cH\x82^\x94`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x80\x91\x90aN0V[`@\x80Q` \x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x91\x83\x01\x91\x90\x91R\x84\x16``\x82\x01R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x88\x90R`\xE0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x1DTc\xF6\x98\xDA%`\xE0\x1B\x85R\x92Q\x90\x94P`\0\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF6\x98\xDA%\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BA\x91\x90aN0V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x82\x90R`B\x81\x01\x84\x90R\x90\x91P`\0\x90`b\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92P`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\x01`&`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x1D`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\xE8\x9A\xCE\x89\x88\x8A0\x8F\x87`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x12\x96\x95\x94\x93\x92\x91\x90aRQV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1C1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CU\x91\x90aN0V[PP`\0\x90\x81R`&` R`@\x90 \x80T`\xFF\x19\x16\x90UPPP`\x01\x93\x90\x93\x01\x92Pa\x17\xC6\x91PPV[PPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1DTW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1D\x16W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1C\xAAV[PPPP\x90P\x90V[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x1D\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xE5\x91\x90aN0V[Pa\x1E$`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa.\xCFV[a\x08P\x82`\0a3gV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1E\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xA8\x91\x90aN0V[Pa\x1E\xDF`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t22\xB87\xB9\xB4\xBA$\xB7:7\xA2\xB4\xB3\xB2\xB760\xBC\xB2\xB9`Y\x1B\x81RPa.\xCFV[`\0[\x82Q\x81\x10\x15a\x08\x1AW`\0\x83\x82\x81Q\x81\x10a\x1E\xFFWa\x1E\xFFaP\xECV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x1F\x1DWa\x1F\x1DaP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1F\xD8W`\0a\x1F^a;^V[P\x90P` `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xC5W=`\0\x80>=`\0\xFD[PPPPa\x1F\xD2\x81a?aV[Pa!8V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a <\x91\x90aQ\x02V[`\x1DT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xB5\x91\x90aQ\x1FV[P`\x1DT`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!5\x91\x90aN0V[PP[PP`\x01\x01a\x1E\xE2V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xBB\x91\x90aN0V[Pa!\xFA`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FverifyWithdrawalCredentials\0\0\0\0\0\x81RPa.\xCFV[a\"\x03\x81a?aV[PV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW\x83\x82\x90`\0R` `\0 \x01\x80Ta\"I\x90aR\x93V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"u\x90aR\x93V[\x80\x15a\"\xC2W\x80`\x1F\x10a\"\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\"*V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a#+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#O\x91\x90aN0V[Pa#\x80`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x1C\xDD\x18\\\x9D\x10\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x8A\x1B\x81RPa.\xCFV[a#\x88a/\x84V[V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a$XW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a$\x1AW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a#\xAEV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a$\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xE0\x91\x90aN0V[Pa%\x0C`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iundelegate`\xB0\x1B\x81RPa.\xCFV[`\0a%\x170a6\xF0V[`\x1CT`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x8B\xE8d\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra%\x8A\x91\x90\x81\x01\x90aQ\xD5V[P`\0[\x81Q\x81\x10\x15a'nW`\0\x80Q` a]C\x839\x81Q\x91R`@Qa%\xDC\x90` \x80\x82R`\x15\x90\x82\x01Rt2\xBC82\xB1\xBA4\xB73\x90;\xB4\xBA4290\xBB\xB0\xB6\x1D`Y\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a]\xE2\x839\x81Q\x91R\x82\x82\x81Q\x81\x10a&\x05Wa&\x05aP\xECV[` \x02` \x01\x01Q``\x01Q`@Qa&B\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03s{s\x1B)\xD1`\xCD\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x82\x82\x81Q\x81\x10a&}Wa&}aP\xECV[` \x02` \x01\x01Q`\xA0\x01Q`\0\x81Q\x81\x10a&\x9BWa&\x9BaP\xECV[` \x02` \x01\x01Q`@Qa&\xDD\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03\x9B\xA3\x93\x0B\xA1\xD1`\xCD\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a]\xE2\x839\x81Q\x91R\x82\x82\x81Q\x81\x10a'\x06Wa'\x06aP\xECV[` \x02` \x01\x01Q`\xC0\x01Q`\0\x81Q\x81\x10a'$Wa'$aP\xECV[` \x02` \x01\x01Q`@Qa'^\x91\x90`@\x80\x82R`\x08\x90\x82\x01Rg\x03\x9BC\x0B\x93+\x99\xD1`\xC5\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a%\x8EV[P\x90P\x90V[`#T``\x90`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x94Wa'\x94aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\xBDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\0[`#T\x81\x10\x15a)\x16W` T`#\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xAAG8\x9C\x91\x90\x84\x90\x81\x10a'\xFAWa'\xFAaP\xECV[\x90`\0R` `\0 \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(J\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x8B\x91\x90aQ\x1FV[\x15a)\x0EW`#\x81\x81T\x81\x10a(\xA3Wa(\xA3aP\xECV[\x90`\0R` `\0 \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10a(\xDCWa(\xDCaP\xECV[d\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a(\xFC\x81aR\xE3V[\x93PP\x81\x80a)\n\x90aR\xE3V[\x92PP[`\x01\x01a'\xC6V[PP\x81R\x91\x90PV[```!\x80Ta).\x90aR\x93V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta)Z\x90aR\x93V[\x80\x15a\x08\xAEW\x80`\x1F\x10a)|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a)\x8AWP\x93\x95\x94PPPPPV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a)\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*!\x91\x90aN0V[Pa*\x86`@Q\x80`@\x01`@R\x80`\n\x81R` \x01idelegateTo`\xB0\x1B\x81RP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE3W=`\0\x80>=`\0\xFD[`@\x80Q\x80\x82\x01\x82R``\x81R`\0` \x82\x01\x81\x90R`\x1CT\x92Qc\xEE\xA9\x06K`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE\xA9\x06K\x91a*\xCD\x91\x86\x91\x86\x91`\x04\x01aR\xFCV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xFBW=`\0\x80>=`\0\xFD[PPPPPPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW\x83\x82\x90`\0R` `\0 \x01\x80Ta+F\x90aR\x93V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta+r\x90aR\x93V[\x80\x15a+\xBFW\x80`\x1F\x10a+\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+\xBFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+\xA2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a+'V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a+\xF3WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12\x04W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a,\x81\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01aS=V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra,\x9B\x91aSnV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a,\xD8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a,\xDDV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a,\xF5\x91\x90aQ\x1FV[\x93\x92PPPV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a-QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-u\x91\x90aN0V[Pa-\xA9`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\xDB\xDB\\\x1B\x19]\x19P\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`r\x1B\x81RPa.\xCFV[a#\x88a0\x18V[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90WPPPPP\x90P\x90V[```\0`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a.jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x8E\x91\x90aN0V[Pa.\xBF`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nstartValidators`\x88\x1B\x81RPa.\xCFV[a.\xC7a;^V[\x91P\x91P\x90\x91V[`\0\x80Q` a]C\x839\x81Q\x91Ra.\xEEa.\xE9a)\x1FV[a@\x1DV[a.\xF7\x83a@FV[`@Q` \x01a/\x08\x92\x91\x90aS\x8AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra/\"\x91aM\xD9V[`@Q\x80\x91\x03\x90\xA1PV[\x80Q\x15a/<W\x80Q\x81` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Freverted with unknown error\0\0\0\0\0`D\x82\x01R`d\x01a\x18\xB2V[`\"T`@Qc\x88gl\xAD`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x88gl\xAD\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\xCAW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a/\xDBWP`\x01[a#\x88W=\x80\x80\x15a0\tW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0\x0EV[``\x91P[Pa\"\x03\x81a/-V[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F- active validator count\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R`\"T\x83Qc#@\xE8\xD3`\xE0\x1B\x81R\x93Qa0\xBF\x94`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c#@\xE8\xD3\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a0\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xBA\x91\x90aN0V[a@nV[`@\x80Q\x80\x82\x01\x82R`\x12\x81Rq- proofs remaining`p\x1B` \x82\x01R`\"T\x82Qc#\xE9A\xB9`\xE1\x1B\x81R\x92Qa1d\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cG\xD2\x83r\x91`\x04\x80\x83\x01\x92`\xA0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1V\x91\x90aS\xC6V[` \x01Qb\xFF\xFF\xFF\x16a@nV[`\"T`@\x80Qc!v\x7F\x95`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cB\xEC\xFF*\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a1\xAEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xD2\x91\x90aTDV[\x90P\x80`\x01`\x01`@\x1B\x03\x16`\0\x03a2FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FUser._completeCheckpoint: no exi`D\x82\x01Ro\x1C\xDD\x1A[\x99\xC8\x18\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x82\x1B`d\x82\x01R`\x84\x01a\x18\xB2V[` T`@Qc\xB1\xB6\xF6\xA1`\xE0\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xB6\xF6\xA1\x90a2z\x90`#\x90\x86\x90`\x04\x01aT_V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra2\xBF\x91\x90\x81\x01\x90aU\xC6V[\x90Pa2\xE8`@Q\x80``\x01`@R\x80`\"\x81R` \x01a]\xC0`\"\x919\x82` \x01QQa@nV[`\"T\x81Q` \x83\x01Q`@Qcx:]1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF0t\xBAb\x92a*\xCD\x92\x90\x91`\x04\x01aW+V[\x81\x83\x14a\x08\x1AW\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x81`@Qa3U\x91\x90aW\xC3V[`@Q\x80\x91\x03\x90\xA1a\x08\x1A\x83\x83a@\x8DV[```\0\x83`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x88Wa3\x88aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xB1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x81Q\x81\x10\x15a6%W`\0\x85`\xA0\x01Q\x82\x81Q\x81\x10a3\xD8Wa3\xD8aP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a5\x87Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x83\x83\x81Q\x81\x10a46Wa46aP\xECV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x15a5\x82Wa4}`@Q\x80``\x01`@R\x80`2\x81R` \x01a]\x8E`2\x919aA~V[a4\x8Da4\x88a'tV[a:\x16V[P` `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xDEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xF2W=`\0\x80>=`\0\xFD[PPPPa4\xFEa/\x84V[`\"`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5u\x91\x90aN0V[\x15a5\x82Wa5\x82a0\x18V[a6\x1CV[\x80`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xE9\x91\x90aQ\x02V[\x83\x83\x81Q\x81\x10a5\xFBWa5\xFBaP\xECV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP[P`\x01\x01a3\xB7V[P`\x1CT`@Qc\x0EL\xC3\xF9`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\xCC?\x90\x90a6Z\x90\x87\x90\x85\x90\x88\x90`\x04\x01aW\xF2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a6tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a6\x88W=`\0\x80>=`\0\xFD[P\x92\x96\x95PPPPPPV[`\0\x80Q` a]C\x839\x81Q\x91Ra6\xAEa.\xE9a)\x1FV[a6\xB7\x84a@FV[\x83`@Q` \x01a6\xCA\x93\x92\x91\x90aX*V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra6\xE4\x91aM\xD9V[`@Q\x80\x91\x03\x90\xA1PPV[`\x1CT`@Qcf\xD5\xBA\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R``\x92`\0\x92\x83\x92\x90\x91\x16\x90cf\xD5\xBA\x93\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7j\x91\x90\x81\x01\x90aX\x89V[\x91P\x91P`\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x89Wa7\x89aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\xC2W\x81` \x01[a7\xAFaC7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a7\xA7W\x90P[P`\x1CT`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a87\x91\x90aQ\x02V[`\x1CT`@Qc(^!!`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xAB\x91\x90aN0V[\x90P`\0[\x85Q\x81\x10\x15a:\nW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a9\x10Wa9\x10aP\xECV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a9+Wa9+aP\xECV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x86\x83\x81Q\x81\x10a9]Wa9]aP\xECV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a9xWa9xaP\xECV[` \x02` \x01\x01\x81\x81RPP`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x86a9\xC8\x91\x90aYJV[\x81R` \x01Bc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81RP\x86\x84\x81Q\x81\x10a9\xF5Wa9\xF5aP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a8\xB0V[P\x91\x96\x95PPPPPPV[`\0a:X`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F- exiting num validators\0\0\0\0\0\0\0\0\x81RP\x83Qa@nV[`\0[\x82Q\x81\x10\x15a;\x15W` T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xF9\x8AN\x90\x85\x90\x84\x90\x81\x10a:\x8DWa:\x8DaP\xECV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xBE\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a:\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\x01\x91\x90aTDV[a;\x0B\x90\x83aY]V[\x91P`\x01\x01a:[V[Pa\x12\x04`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7F- exited balance to pod (gwei)\0\0\x81RP\x82`\x01`\x01`@\x1B\x03\x16a@nV[```\0G\x81a;wh\x01\xBC\x16\xD6t\xEC\x80\0\0\x83aY\x92V[\x90Pa;\x8C\x81h\x01\xBC\x16\xD6t\xEC\x80\0\0aY\xA6V[a;\x96\x90\x83aY\xBDV[\x91P`\0\x81g\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a;\xDEWa;\xB8c;\x9A\xCA\0\x85aY\xD0V[a;\xC2\x90\x85aY\xBDV[\x91Pa;\xCE\x82\x85aY\xBDV[\x93P\x80a;\xDA\x81aR\xE3V[\x91PP[\x80`\0\x03a<KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FstartValidators: not enough ETH `D\x82\x01Rs:7\x909\xBA0\xB9:\x100\x90;0\xB64\xB20\xBA7\xB9`a\x1B`d\x82\x01R`\x84\x01a\x18\xB2V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a<eWa<eaC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\x8EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0c;\x9A\xCA\0a<\xA2\x87GaY\xBDV[a<\xAC\x91\x90aY\x92V[\x90Pa<\xEE`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- creating new validators\0\0\0\0\0\0\0\x81RP\x83Qa@nV[a=\x19`@Q\x80``\x01`@R\x80`+\x81R` \x01a]c`+\x919\x82`\x01`\x01`@\x1B\x03\x16a@nV[`\0[\x85\x81\x10\x15a>4W` T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05h\x01\xBC\x16\xD6t\xEC\x80\0\0a=JaA\x9BV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=f\x91\x90aM\xD9V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a=\x84W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xA9\x91\x90aY\xE4V[\x90P\x80\x84\x83\x81Q\x81\x10a=\xBEWa=\xBEaP\xECV[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`#\x80T`\x01\x81\x81\x01\x83U`\0\x92\x90\x92R`\x06\x80\x82\x04\x7F\xD5{+QfG\x8F\xD41\x8D*\xCCl\xC2\xC7\x04XC\x12\xBD\xD8x\x1B2\xD5\xD0j\xBD\xA5\x7FB0\x01\x80T\x95\x85\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x93\x02\x19\x90\x93\x16\x92\x90\x92\x17\x90U\x01a=\x1CV[Pa>@\x85`\x01aYJV[\x83\x03a?TW` T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05\x86a>caA\x9BV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x7F\x91\x90aM\xD9V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a>\x9DW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xC2\x91\x90aY\xE4V[\x90P\x80\x83`\x01\x85Qa>\xD4\x91\x90aY\xBDV[\x81Q\x81\x10a>\xE4Wa>\xE4aP\xECV[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`#\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R`\x06\x80\x82\x04\x7F\xD5{+QfG\x8F\xD41\x8D*\xCCl\xC2\xC7\x04XC\x12\xBD\xD8x\x1B2\xD5\xD0j\xBD\xA5\x7FB0\x01\x80T\x94\x84\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x92\x02\x19\x90\x92\x16\x91\x90\x91\x17\x90U[\x90\x97\x90\x96P\x94PPPPPV[` T`@QcR\x85\x1D\r`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA5\n:\x1A\x90a?\x92\x90\x85\x90`\x04\x01aM\xC6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra?\xD7\x91\x90\x81\x01\x90aZ\x83V[`\"T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc?e\xCF\x19`\xE0\x1B\x81R\x95\x96P`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x94c?e\xCF\x19\x94a\x07\xB2\x94\x93\x92\x89\x92`\x04\x01a\\.V[``a\x08P`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83aA\xE1V[``a\x08P`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83aA\xE1V[`\0\x80Q` a]\xE2\x839\x81Q\x91R\x82\x82`@Qa6\xE4\x92\x91\x90a\\\xDDV[\x80\x82\x14a\x08\x1CW`\0\x80Q` a]C\x839\x81Q\x91R`@Qa@\xEC\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q`\0\x80Q` a]\xE2\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q`\0\x80Q` a]\xE2\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1a\x08\x1CaB+V[`\0\x80Q` a]C\x839\x81Q\x91R\x81`@Qa/\"\x91\x90aM\xD9V[`\"T`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16`,\x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01aB\x14\x93\x92\x91\x90a\\\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aC&W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaB\xC5\x92\x91` \x01aS=V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaB\xDF\x91aSnV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14aC\x1CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aC!V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"\x03W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\xBAW`\0\x80\xFD[\x815a,\xF5\x81aC\x95V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\xB3WaD\xB3aC\xC5V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xD4WaD\xD4aC\xC5V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aD\xF5W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x12W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aE#W`\0\x80\xFD[\x805aE6aE1\x82aD\xBBV[aD\x8BV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aEKW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aE\xACW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aE\x85V[P\x90\x95\x94PPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aE\xD0WaE\xD0aC\xC5V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\x03W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aF\0W`\0\x80\xFD[\x815aF\x0EaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aF0W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x805aFH\x81aE\xDAV[\x83R` \x92\x83\x01\x92\x01aF5V[P\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aFsW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x89W`\0\x80\xFD[aF\x95\x85\x82\x86\x01aE\xEFV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xB1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aF\xC2W`\0\x80\xFD[\x805aF\xD0aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aF\xF2W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aG\x14W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aF\xF9V[\x80\x94PPPPP\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12aG3W`\0\x80\xFD[\x815aGAaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aGcW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x805\x83R` \x92\x83\x01\x92\x01aGhV[`\0\x80`@\x83\x85\x03\x12\x15aG\x93W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xA9W`\0\x80\xFD[aG\xB5\x85\x82\x86\x01aE\xEFV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xD1W`\0\x80\xFD[aG\xDD\x85\x82\x86\x01aG\"V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aH\"W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aG\xFBV[P\x93\x94\x93PPPPV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aH\"W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aH@V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q`\0\x91aH\xAA\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaH\xC4`\xE0\x85\x01\x82aG\xE7V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaH\xDD\x82\x82aH,V[\x95\x94PPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84RaI*\x85\x83QaH^V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aI\x0EV[\x805a\x12\x04\x81aE\xDAV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\x04W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15aIpW`\0\x80\xFD[aIxaC\xDBV[\x90PaI\x83\x82aI?V[\x81RaI\x91` \x83\x01aI?V[` \x82\x01RaI\xA2`@\x83\x01aI?V[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaI\xBD`\x80\x83\x01aIJV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xDBW`\0\x80\xFD[aI\xE7\x84\x82\x85\x01aE\xEFV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x06W`\0\x80\xFD[aJ\x12\x84\x82\x85\x01aG\"V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aJ0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aJWW`\0\x80\xFD[\x805aJeaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aJ\x87W`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15aJ\xC8W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xAAW`\0\x80\xFD[aJ\xB9\x89` \x83\x89\x01\x01aI^V[\x84RP` \x92\x83\x01\x92\x01aJ\x8CV[P\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84RaK\x17\x85\x83QaG\xE7V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aJ\xFBV[`\0` \x82\x84\x03\x12\x15aK>W`\0\x80\xFD[\x815a,\xF5\x81aE\xDAV[`\0` \x82\x84\x03\x12\x15aK[W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKqW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aK\x82W`\0\x80\xFD[\x805aK\x90aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aK\xB2W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aK\xDDW\x835aK\xCC\x81aC\x95V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aK\xB9V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aK\xF9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x0FW`\0\x80\xFD[aL\x1B\x84\x82\x85\x01aI^V[\x94\x93PPPPV[` \x81R`\0a,\xF5` \x83\x01\x84aG\xE7V[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15aL\xCCW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aL\xA0V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aL^V[`\0[\x83\x81\x10\x15aL\xFFW\x81\x81\x01Q\x83\x82\x01R` \x01aL\xE7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaM \x81` \x86\x01` \x86\x01aL\xE4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84RaMx\x85\x83QaM\x08V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aM\\V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aH\"W\x81Qd\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aM\xA1V[` \x81R`\0a,\xF5` \x83\x01\x84aM\x8DV[` \x81R`\0a,\xF5` \x83\x01\x84aM\x08V[`\0` \x82\x84\x03\x12\x15aM\xFEW`\0\x80\xFD[P5\x91\x90PV[`@\x81R`\0aN\x18`@\x83\x01\x85aM\x8DV[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aNBW`\0\x80\xFD[PQ\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x12\x04W`\0\x80\xFD[`\0aNnaE1\x84aD\xBBV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15aN\x82W`\0\x80\xFD[a,\xF5\x83` \x83\x01\x84aL\xE4V[`\0\x82`\x1F\x83\x01\x12aN\xA1W`\0\x80\xFD[a,\xF5\x83\x83Q` \x85\x01aN`V[`\0`@\x82\x84\x03\x12\x15aN\xC2W`\0\x80\xFD[aN\xCAaD\x03V[\x82Q\x81R` \x83\x01Q\x90\x91P`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xEAW`\0\x80\xFD[aN\xF6\x84\x82\x85\x01aN\x90V[` \x83\x01RP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aO\x13W`\0\x80\xFD[\x81QaO!aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aOCW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x80Q\x83R` \x92\x83\x01\x92\x01aOHV[`\0` \x82\x84\x03\x12\x15aOrW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x88W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15aO\x9AW`\0\x80\xFD[aO\xA2aD%V[aO\xAB\x82aNIV[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xC6W`\0\x80\xFD[aO\xD2\x86\x82\x85\x01aN\xB0V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xF1W`\0\x80\xFD[\x91\x90\x91\x01\x90`@\x82\x86\x03\x12\x15aP\x06W`\0\x80\xFD[aP\x0EaD\x03V[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aP$W`\0\x80\xFD[aP0\x87\x82\x86\x01aO\x02V[\x82RP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aPLW`\0\x80\xFD[aPX\x87\x82\x86\x01aN\x90V[` \x83\x01RP`@\x82\x01R\x94\x93PPPPV[\x80Q\x82R`\0` \x82\x01Q`@` \x85\x01RaL\x1B`@\x85\x01\x82aM\x08V[`\x01`\x01`@\x1B\x03\x84\x16\x81R``` \x82\x01R`\0aP\xAC``\x83\x01\x85aPkV[\x82\x81\x03`@\x84\x01R\x83Q`@\x82RaP\xC7`@\x83\x01\x82aH,V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaP\xE0\x82\x82aM\x08V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aQ\x14W`\0\x80\xFD[\x81Qa,\xF5\x81aE\xDAV[`\0` \x82\x84\x03\x12\x15aQ1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a,\xF5W`\0\x80\xFD[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q``\x87RaQ\x8F``\x88\x01\x82aG\xE7V[\x90P` \x82\x01Q\x87\x82\x03` \x89\x01RaQ\xA8\x82\x82aH,V[`@\x93\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x98\x90\x93\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aQiV[`\0` \x82\x84\x03\x12\x15aQ\xE7W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xFDW`\0\x80\xFD[aL\x1B\x84\x82\x85\x01aO\x02V[`\0` \x82\x84\x03\x12\x15aR\x1BW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR1W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aRBW`\0\x80\xFD[aL\x1B\x84\x82Q` \x84\x01aN`V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R`@\x82\x01\x86\x90R\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xC0`\xA0\x82\x01\x81\x90R`\0\x90aP\xE0\x90\x83\x01\x84aM\x08V[`\x01\x81\x81\x1C\x90\x82\x16\x80aR\xA7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aR\xC7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aR\xF5WaR\xF5aR\xCDV[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R`\0\x83Q`@``\x84\x01RaS&`\xA0\x84\x01\x82aM\x08V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90aS`\x81`\x04\x85\x01` \x87\x01aL\xE4V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82QaS\x80\x81\x84` \x87\x01aL\xE4V[\x91\x90\x91\x01\x92\x91PPV[`\0\x83QaS\x9C\x81\x84` \x88\x01aL\xE4V[`\x17`\xF9\x1B\x90\x83\x01\x90\x81R\x83QaS\xBA\x81`\x01\x84\x01` \x88\x01aL\xE4V[\x01`\x01\x01\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x80\x15aS\xD9W`\0\x80\xFD[P`\0aS\xE4aDGV[\x83Q\x81R` \x84\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14aS\xFDW\x82\x83\xFD[` \x82\x01RaT\x0E`@\x85\x01aNIV[`@\x82\x01R``\x84\x01Q\x80`\x07\x0B\x81\x14aT&W\x82\x83\xFD[``\x82\x01RaT7`\x80\x85\x01aNIV[`\x80\x82\x01R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aTVW`\0\x80\xFD[a,\xF5\x82aNIV[`\0`@\x82\x01`@\x83R\x80\x85TaTz\x81\x84\x90\x81R` \x01\x90V[`\0\x88\x81R` \x81 \x94P\x90\x92P[\x81`\x05\x82\x01\x10\x15aT\xF0W\x83Td\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x85R`(\x82\x90\x1C\x81\x16` \x86\x01R`P\x82\x90\x1C\x81\x16`@\x86\x01R`x\x82\x90\x1C\x81\x16``\x86\x01R`\xA0\x82\x81\x1C\x82\x16`\x80\x87\x01R`\xC8\x92\x90\x92\x1C\x16\x90\x84\x01R`\x01\x90\x93\x01\x92`\xC0\x90\x92\x01\x91`\x06\x01aT\x89V[\x92T\x92\x81\x81\x10\x15aU\x0FWd\xFF\xFF\xFF\xFF\xFF\x84\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aU/Wd\xFF\xFF\xFF\xFF\xFF`(\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aUOWd\xFF\xFF\xFF\xFF\xFF`P\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aUoWd\xFF\xFF\xFF\xFF\xFF`x\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aU\x8FWd\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aU\xACW`\xC8\x84\x90\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x83\x01\x92P[PP`\x01`\x01`@\x1B\x03\x85\x16` \x85\x01R\x91Pa,\xF5\x90PV[`\0` \x82\x84\x03\x12\x15aU\xD8W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xEEW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15aV\0W`\0\x80\xFD[aV\x08aD\x03V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x1EW`\0\x80\xFD[aV*\x86\x82\x85\x01aN\xB0V[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aVFW`\0\x80\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12aV[W`\0\x80\xFD[\x81QaViaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x87\x83\x11\x15aV\x8BW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aW\x1AW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xAEW`\0\x80\xFD[\x86\x01``\x81\x8B\x03`\x1F\x19\x01\x12\x15aV\xC4W`\0\x80\xFD[aV\xCCaD%V[` \x82\x81\x01Q\x82R`@\x83\x01Q\x90\x82\x01R``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xF6W`\0\x80\xFD[aW\x05\x8C` \x83\x86\x01\x01aN\x90V[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01aV\x90V[P` \x84\x01RP\x90\x95\x94PPPPPV[`@\x81R`\0aW>`@\x83\x01\x85aPkV[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01`\0[\x83\x81\x10\x15aW\xB5W`\x1F\x19\x86\x84\x03\x01\x85R\x81Q\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q\x90P```@\x85\x01RaW\x9E``\x85\x01\x82aM\x08V[` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01aWbV[P\x90\x98\x97PPPPPPPPV[`@\x81R`\x05`@\x82\x01Rd\"\xB997\xB9`\xD9\x1B``\x82\x01R`\x80` \x82\x01R`\0a,\xF5`\x80\x83\x01\x84aM\x08V[``\x81R`\0aX\x05``\x83\x01\x86aH^V[\x82\x81\x03` \x84\x01RaX\x17\x81\x86aG\xE7V[\x91PP\x82\x15\x15`@\x83\x01R\x94\x93PPPPV[`\0\x84QaX<\x81\x84` \x89\x01aL\xE4V[`\x17`\xF9\x1B\x90\x83\x01\x90\x81R\x84QaXZ\x81`\x01\x84\x01` \x89\x01aL\xE4V[`\x1D`\xF9\x1B`\x01\x92\x90\x91\x01\x91\x82\x01R\x83QaX|\x81`\x02\x84\x01` \x88\x01aL\xE4V[\x01`\x02\x01\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aX\x9CW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aX\xB2W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aX\xC3W`\0\x80\xFD[\x80QaX\xD1aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aX\xF3W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aY\x1EW\x83QaY\r\x81aE\xDAV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aX\xFAV[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY>W`\0\x80\xFD[aG\xDD\x85\x82\x86\x01aO\x02V[\x80\x82\x01\x80\x82\x11\x15a\x08PWa\x08PaR\xCDV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x08PWa\x08PaR\xCDV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aY\xA1WaY\xA1aY|V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08PWa\x08PaR\xCDV[\x81\x81\x03\x81\x81\x11\x15a\x08PWa\x08PaR\xCDV[`\0\x82aY\xDFWaY\xDFaY|V[P\x06\x90V[`\0` \x82\x84\x03\x12\x15aY\xF6W`\0\x80\xFD[\x81Qa,\xF5\x81aC\x95V[`\0\x82`\x1F\x83\x01\x12aZ\x12W`\0\x80\xFD[\x81QaZ aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aZBW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZeW`\0\x80\xFD[aZt\x88` \x83\x8A\x01\x01aO\x02V[\x84RP` \x92\x83\x01\x92\x01aZGV[`\0` \x82\x84\x03\x12\x15aZ\x95W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xABW`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15aZ\xBDW`\0\x80\xFD[aZ\xC5aDiV[aZ\xCE\x82aNIV[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xE9W`\0\x80\xFD[aZ\xF5\x86\x82\x85\x01aN\xB0V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[\x14W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a[%W`\0\x80\xFD[\x80Qa[3aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a[UW`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a[\x96W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[xW`\0\x80\xFD[a[\x87\x8B` \x83\x89\x01\x01aN\x90V[\x84RP` \x92\x83\x01\x92\x01a[ZV[P`@\x85\x01RPPP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[\xB8W`\0\x80\xFD[a[\xC4\x86\x82\x85\x01aZ\x01V[``\x83\x01RP\x94\x93PPPPV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15a\\\"W`\x1F\x19\x85\x84\x03\x01\x88Ra\\\x0C\x83\x83QaH,V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a[\xF0V[P\x90\x96\x95PPPPPPV[`\x01`\x01`@\x1B\x03\x86\x16\x81R`\xA0` \x82\x01R`\0a\\P`\xA0\x83\x01\x87aPkV[\x82\x81\x03`@\x84\x01Ra\\b\x81\x87aM\x8DV[\x90P\x82\x81\x03``\x84\x01R\x80\x85Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x88\x01`\0[\x83\x81\x10\x15a\\\xBAW`\x1F\x19\x86\x84\x03\x01\x85Ra\\\xA4\x83\x83QaM\x08V[` \x95\x86\x01\x95\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a\\\x88V[PP\x85\x81\x03`\x80\x87\x01Ra\\\xCE\x81\x88a[\xD2V[\x9B\x9APPPPPPPPPPPV[`@\x81R`\0a\\\xF0`@\x83\x01\x85aM\x08V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x84Qa]\x11\x81\x84` \x89\x01aL\xE4V[\x84Q\x90\x83\x01\x90a]%\x81\x83` \x89\x01aL\xE4V[\x84Q\x91\x01\x90a]8\x81\x83` \x88\x01aL\xE4V[\x01\x95\x94PPPPPV\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP- depositing balance to beacon chain (gwei)- exiting all validators and completing checkpoint- submitting num checkpoint proofs\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8User.queueWithdrawals: length mismatch\xA2dipfsX\"\x12 \xBCO\x9E`A;i\xF3K\x8DV\xF33~h\xC7\xBB\xB91\xFA\x94\xFB\xC1IY\xD7\xC5\xAE\xE9\xE9\x88\xFDdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101f25760003560e01c80636d336f581161010d578063a88dbb36116100a0578063ba414fa61161006f578063ba414fa6146105d2578063d6c10daf146105e7578063e20c9f71146105fc578063f234c1bd14610611578063fa7626d41461063457600080fd5b8063a88dbb3614610525578063ac637c7a1461055d578063b5508aa91461057d578063b71723551461059257600080fd5b8063916a17c6116100dc578063916a17c6146104b757806392ab89bb146104cc5780639de70258146104e1578063a3f4df7e1461050357600080fd5b80636d336f5814610440578063841c12991461046057806385226c811461048057806390b51625146104a257600080fd5b80633d8c08d41161018557806346a5be0d1161015457806346a5be0d146103be578063654bb5d9146103de57806366d9a9a0146103fe578063695f4ae11461042057600080fd5b80633d8c08d41461032f5780633e5e3c23146103675780633f7286f41461037c578063401be65e1461039157600080fd5b806323e48175116101c157806323e48175146102a05780632a34ade8146102cd578063344e1383146102e2578063391cc9f61461030f57600080fd5b8063071c25b7146101fe5780631626ba7e146102205780631ed7831c1461025e57806320a545d91461028057600080fd5b366101f957005b600080fd5b34801561020a57600080fd5b5061021e6102193660046143a8565b61064e565b005b34801561022c57600080fd5b5061024061023b3660046144e2565b610820565b6040516001600160e01b031990911681526020015b60405180910390f35b34801561026a57600080fd5b50610273610856565b604051610255919061456b565b34801561028c57600080fd5b5061021e61029b366004614660565b6108b8565b3480156102ac57600080fd5b506102c06102bb366004614780565b610bd7565b60405161025591906148e6565b3480156102d957600080fd5b5061021e610f37565b3480156102ee57600080fd5b506103026102fd366004614a1e565b6110ab565b6040516102559190614ad3565b34801561031b57600080fd5b506102c061032a366004614b2c565b611209565b34801561033b57600080fd5b5061034f61034a366004614b49565b611391565b6040516001600160401b039091168152602001610255565b34801561037357600080fd5b50610273611439565b34801561038857600080fd5b50610273611499565b34801561039d57600080fd5b506103b16103ac366004614be7565b6114f9565b6040516102559190614c23565b3480156103ca57600080fd5b506103026103d9366004614a1e565b6115b3565b3480156103ea57600080fd5b5061021e6103f9366004614780565b611708565b34801561040a57600080fd5b50610413611c86565b6040516102559190614c36565b34801561042c57600080fd5b506103b161043b366004614be7565b611d75565b34801561044c57600080fd5b5061021e61045b366004614780565b611e2f565b34801561046c57600080fd5b5061021e61047b366004614b49565b612142565b34801561048c57600080fd5b50610495612206565b6040516102559190614d34565b3480156104ae57600080fd5b5061021e6122d6565b3480156104c357600080fd5b5061041361238a565b3480156104d857600080fd5b506102c0612470565b3480156104ed57600080fd5b506104f6612774565b6040516102559190614dc6565b34801561050f57600080fd5b5061051861291f565b6040516102559190614dd9565b34801561053157600080fd5b50602254610545906001600160a01b031681565b6040516001600160a01b039091168152602001610255565b34801561056957600080fd5b5061021e610578366004614b2c565b6129a8565b34801561058957600080fd5b50610495612b03565b34801561059e57600080fd5b506105c26105ad366004614dec565b60266020526000908152604090205460ff1681565b6040519015158152602001610255565b3480156105de57600080fd5b506105c2612bd3565b3480156105f357600080fd5b5061021e612cfc565b34801561060857600080fd5b50610273612db1565b34801561061d57600080fd5b50610626612e11565b604051610255929190614e05565b34801561064057600080fd5b506000546105c29060ff1681565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156106a3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106c79190614e30565b506106fb604051806040016040528060128152602001717665726966795374616c6542616c616e636560701b815250612ecf565b6020546040516308fa0b1360e21b815264ffffffffff831660048201526000916001600160a01b0316906323e82c4c90602401600060405180830381865afa15801561074b573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526107739190810190614f60565b6022548151602083015160408085015190516301c8abe960e11b81529495506001600160a01b039093169363039157d2936107b293929160040161508a565b600060405180830381600087803b1580156107cc57600080fd5b505af19250505080156107dd575060015b61081c573d80801561080b576040519150601f19603f3d011682016040523d82523d6000602084013e610810565b606091505b5061081a81612f2d565b505b5050565b60008281526026602052604081205460ff16156108455750630b135d3f60e11b610850565b506001600160e01b03195b92915050565b6060600d8054806020026020016040519081016040528092919081815260200182805480156108ae57602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610890575b5050505050905090565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af115801561090d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109319190614e30565b506109616040518060400160405280600e81526020016d75706461746542616c616e63657360901b815250612ecf565b60005b825181101561081a576000838281518110610981576109816150ec565b60200260200101519050600083838151811061099f5761099f6150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603610a67576109de612f84565b602260009054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a31573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a559190614e30565b15610a6257610a62613018565b610bcd565b60008190506000836001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015610aac573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ad09190615102565b601d5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015610b25573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b49919061511f565b50601d546040516373d0285560e11b81526001600160a01b0386811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af1158015610ba5573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610bc99190614e30565b5050505b5050600101610964565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015610c23573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c479190614e30565b50610c796040518060400160405280601081526020016f71756575655769746864726177616c7360801b815250612ecf565b601c54604051631976849960e21b81523060048201526000916001600160a01b0316906365da126490602401602060405180830381865afa158015610cc2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ce69190615102565b601c5460405163285e212160e21b815230600482018190529293506000916001600160a01b03169063a178848490602401602060405180830381865afa158015610d34573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d589190614e30565b60408051600180825281830190925291925060009190816020015b60408051606080820183528082526020820152600091810191909152815260200190600190039081610d735790505090506040518060600160405280888152602001878152602001846001600160a01b031681525081600081518110610ddb57610ddb6150ec565b6020908102919091010152604080516001808252818301909252600091816020015b610e05614337565b815260200190600190039081610dfd5790505090506040518060e00160405280306001600160a01b03168152602001866001600160a01b03168152602001856001600160a01b031681526020018481526020014263ffffffff1681526020018981526020018881525081600081518110610e8157610e816150ec565b6020908102919091010152601c546040516306ec6e8160e11b81526000916001600160a01b031690630dd8dd0290610ebd908690600401615141565b6000604051808303816000875af1158015610edc573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052610f0491908101906151d5565b9050610f2b82518251604051806060016040528060268152602001615e026026913961331f565b50979650505050505050565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015610f8c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fb09190614e30565b50610fe4604051806040016040528060128152602001713932b3b4b9ba32b920b9a7b832b930ba37b960711b815250612ecf565b60408051606081018252308152600060208201818152828401918252601c5460248054955163024b980360e51b815285516001600160a01b0390811660048301529351841691810191909152925163ffffffff9081166044850152909416606483015260a06084830152600860a4830152676d6574616461746160c01b60c48301529192919091169063497300609060e401600060405180830381600087803b15801561109057600080fd5b505af11580156110a4573d6000803e3d6000fd5b5050505050565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af11580156110f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061111b9190614e30565b5061115a6040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612ecf565b600082516001600160401b03811115611175576111756143c5565b6040519080825280602002602001820160405280156111a857816020015b60608152602001906001900390816111935790505b50905060005b8351811015611200576111db8482815181106111cc576111cc6150ec565b60200260200101516001613367565b8282815181106111ed576111ed6150ec565b60209081029190910101526001016111ae565b5090505b919050565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015611255573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112799190614e30565b506113106040518060400160405280600f81526020016e666f726365556e64656c656761746560881b815250836001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156112e3573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261130b9190810190615209565b613694565b600061131b836136f0565b601c546040516336a2fa1960e21b81526001600160a01b03868116600483015292935091169063da8be864906024016000604051808303816000875af1158015611369573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261120091908101906151d5565b601f54604080516301504d8f60e41b815290516000926001600160a01b031691631504d8f0916004808301926020929190829003018187875af11580156113dc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906114009190614e30565b506114306040518060400160405280600e81526020016d6578697456616c696461746f727360901b815250612ecf565b61085082613a16565b6060600f8054806020026020016040519081016040528092919081815260200182805480156108ae576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610890575050505050905090565b6060600e8054806020026020016040519081016040528092919081815260200182805480156108ae576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610890575050505050905090565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015611545573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115699190614e30565b506115a86040518060400160405280601b81526020017f636f6d706c6574655769746864726177616c734173546f6b656e730000000000815250612ecf565b610850826001613367565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af11580156115ff573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116239190614e30565b506116626040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612ecf565b600082516001600160401b0381111561167d5761167d6143c5565b6040519080825280602002602001820160405280156116b057816020015b606081526020019060019003908161169b5790505b50905060005b8351811015611200576116e38482815181106116d4576116d46150ec565b60200260200101516000613367565b8282815181106116f5576116f56150ec565b60209081029190910101526001016116b6565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af115801561175d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117819190614e30565b506117c06040518060400160405280601d81526020017f2e6465706f736974496e746f456967656e6c617965725f4d315f414c54000000815250612ecf565b60001960005b8351811015611c805760008482815181106117e3576117e36150ec565b602002602001015190506000848381518110611801576118016150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b0316036118bb5760405162461bcd60e51b815260206004820152604760248201527f53686f756c64206e6f74206265206465706f736974696e67207769746820424560448201527f41434f4e434841494e5f4554485f535452415420666f72204d312d6d61696e6e60648201526632ba102ab9b2b960c91b608482015260a4015b60405180910390fd5b6000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118fb573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061191f9190615102565b601d5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015611974573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611998919061511f565b50601d54604051623f675f60e91b81523060048201526000916001600160a01b031690637ecebe0090602401602060405180830381865afa1580156119e1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a059190614e30565b90506000601d60009054906101000a90046001600160a01b03166001600160a01b03166348825e946040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a5c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a809190614e30565b6040805160208101929092526001600160a01b0380881691830191909152841660608201526080810185905260a0810183905260c0810188905260e00160408051601f198184030181528282528051602091820120601d5463f698da2560e01b855292519094506000936001600160a01b039093169263f698da259260048083019391928290030181865afa158015611b1d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611b419190614e30565b60405161190160f01b6020820152602281018290526042810184905290915060009060620160408051601f198184030181528282528051602091820120908301819052925060009101604051602081830303815290604052905060016026600084815260200190815260200160002060006101000a81548160ff021916908315150217905550601d60009054906101000a90046001600160a01b03166001600160a01b03166332e89ace89888a308f876040518763ffffffff1660e01b8152600401611c1296959493929190615251565b6020604051808303816000875af1158015611c31573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c559190614e30565b50506000908152602660205260409020805460ff1916905550505060019390930192506117c6915050565b50505050565b60606012805480602002602001604051908101604052809291908181526020016000905b82821015611d6c5760008481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611d5457602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611d165790505b50505050508152505081526020019060010190611caa565b50505050905090565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af1158015611dc1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611de59190614e30565b50611e246040518060400160405280601a81526020017f636f6d706c6574655769746864726177616c4173536861726573000000000000815250612ecf565b610850826000613367565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015611e84573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611ea89190614e30565b50611edf604051806040016040528060158152602001743232b837b9b4ba24b73a37a2b4b3b2b73630bcb2b960591b815250612ecf565b60005b825181101561081a576000838281518110611eff57611eff6150ec565b602002602001015190506000838381518110611f1d57611f1d6150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316826001600160a01b031603611fd8576000611f5e613b5e565b509050602060009054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b8152600401600060405180830381600087803b158015611fb157600080fd5b505af1158015611fc5573d6000803e3d6000fd5b50505050611fd281613f61565b50612138565b6000826001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa158015612018573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061203c9190615102565b601d5460405163095ea7b360e01b81526001600160a01b0391821660048201526024810185905291925082169063095ea7b3906044016020604051808303816000875af1158015612091573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906120b5919061511f565b50601d546040516373d0285560e11b81526001600160a01b0385811660048301528381166024830152604482018590529091169063e7a050aa906064016020604051808303816000875af1158015612111573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121359190614e30565b50505b5050600101611ee2565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015612197573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906121bb9190614e30565b506121fa6040518060400160405280601b81526020017f7665726966795769746864726177616c43726564656e7469616c730000000000815250612ecf565b61220381613f61565b50565b60606011805480602002602001604051908101604052809291908181526020016000905b82821015611d6c57838290600052602060002001805461224990615293565b80601f016020809104026020016040519081016040528092919081815260200182805461227590615293565b80156122c25780601f10612297576101008083540402835291602001916122c2565b820191906000526020600020905b8154815290600101906020018083116122a557829003601f168201915b50505050508152602001906001019061222a565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af115801561232b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061234f9190614e30565b506123806040518060400160405280600f81526020016e1cdd185c9d10da1958dadc1bda5b9d608a1b815250612ecf565b612388612f84565b565b60606013805480602002602001604051908101604052809291908181526020016000905b82821015611d6c5760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561245857602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161241a5790505b505050505081525050815260200190600101906123ae565b601f54604080516301504d8f60e41b815290516060926001600160a01b031691631504d8f091600480830192602092919082900301816000875af11580156124bc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906124e09190614e30565b5061250c6040518060400160405280600a815260200169756e64656c656761746560b01b815250612ecf565b6000612517306136f0565b601c546040516336a2fa1960e21b81523060048201529192506001600160a01b03169063da8be864906024016000604051808303816000875af1158015612562573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261258a91908101906151d5565b5060005b815181101561276e57600080516020615d438339815191526040516125dc9060208082526015908201527432bc3832b1ba34b733903bb4ba34323930bbb0b61d60591b604082015260600190565b60405180910390a1600080516020615de2833981519152828281518110612605576126056150ec565b602002602001015160600151604051612642919060408082526007908201526603737b731b29d160cd1b6060820152602081019190915260800190565b60405180910390a17f9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f82828151811061267d5761267d6150ec565b602002602001015160a0015160008151811061269b5761269b6150ec565b60200260200101516040516126dd9190604080825260079082015266039ba3930ba1d160cd1b60608201526001600160a01b0391909116602082015260800190565b60405180910390a1600080516020615de2833981519152828281518110612706576127066150ec565b602002602001015160c00151600081518110612724576127246150ec565b602002602001015160405161275e9190604080825260089082015267039b430b932b99d160c51b6060820152602081019190915260800190565b60405180910390a160010161258e565b50905090565b6023546060906000906001600160401b03811115612794576127946143c5565b6040519080825280602002602001820160405280156127bd578160200160208202803683370190505b50905060008060005b60235481101561291657602054602380546001600160a01b039092169163aa47389c9190849081106127fa576127fa6150ec565b90600052602060002090600691828204019190066005029054906101000a900464ffffffffff166040518263ffffffff1660e01b815260040161284a919064ffffffffff91909116815260200190565b602060405180830381865afa158015612867573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061288b919061511f565b1561290e57602381815481106128a3576128a36150ec565b90600052602060002090600691828204019190066005029054906101000a900464ffffffffff168483815181106128dc576128dc6150ec565b64ffffffffff90921660209283029190910190910152826128fc816152e3565b935050818061290a906152e3565b9250505b6001016127c6565b50508152919050565b60606021805461292e90615293565b80601f016020809104026020016040519081016040528092919081815260200182805461295a90615293565b80156108ae5780601f1061297c576101008083540402835291602001916108ae565b820191906000526020600020905b81548152906001019060200180831161298a57509395945050505050565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af11580156129fd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612a219190614e30565b50612a866040518060400160405280600a81526020016964656c6567617465546f60b01b815250826001600160a01b031663a3f4df7e6040518163ffffffff1660e01b8152600401600060405180830381865afa1580156112e3573d6000803e3d6000fd5b60408051808201825260608152600060208201819052601c54925163eea9064b60e01b815291926001600160a01b03169163eea9064b91612acd91869186916004016152fc565b600060405180830381600087803b158015612ae757600080fd5b505af1158015612afb573d6000803e3d6000fd5b505050505050565b60606010805480602002602001604051908101604052809291908181526020016000905b82821015611d6c578382906000526020600020018054612b4690615293565b80601f0160208091040260200160405190810160405280929190818152602001828054612b7290615293565b8015612bbf5780601f10612b9457610100808354040283529160200191612bbf565b820191906000526020600020905b815481529060010190602001808311612ba257829003601f168201915b505050505081526020019060010190612b27565b60008054610100900460ff1615612bf35750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156112045760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091612c81917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc49160800161533d565b60408051601f1981840301815290829052612c9b9161536e565b6000604051808303816000865af19150503d8060008114612cd8576040519150601f19603f3d011682016040523d82523d6000602084013e612cdd565b606091505b5091505080806020019051810190612cf5919061511f565b9392505050565b601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015612d51573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612d759190614e30565b50612da96040518060400160405280601281526020017118dbdb5c1b195d1950da1958dadc1bda5b9d60721b815250612ecf565b612388613018565b6060600c8054806020026020016040519081016040528092919081815260200182805480156108ae576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610890575050505050905090565b60606000601f60009054906101000a90046001600160a01b03166001600160a01b0316631504d8f06040518163ffffffff1660e01b81526004016020604051808303816000875af1158015612e6a573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612e8e9190614e30565b50612ebf6040518060400160405280600f81526020016e737461727456616c696461746f727360881b815250612ecf565b612ec7613b5e565b915091509091565b600080516020615d43833981519152612eee612ee961291f565b61401d565b612ef783614046565b604051602001612f0892919061538a565b60408051601f1981840301815290829052612f2291614dd9565b60405180910390a150565b805115612f3c57805181602001fd5b60405162461bcd60e51b815260206004820152601b60248201527f7265766572746564207769746820756e6b6e6f776e206572726f72000000000060448201526064016118b2565b6022546040516388676cad60e01b8152600060048201526001600160a01b03909116906388676cad90602401600060405180830381600087803b158015612fca57600080fd5b505af1925050508015612fdb575060015b612388573d808015613009576040519150601f19603f3d011682016040523d82523d6000602084013e61300e565b606091505b5061220381612f2d565b604080518082018252601881527f2d206163746976652076616c696461746f7220636f756e7400000000000000006020808301919091526022548351632340e8d360e01b815293516130bf946001600160a01b0390921692632340e8d392600480820193918290030181865afa158015613096573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906130ba9190614e30565b61406e565b60408051808201825260128152712d2070726f6f66732072656d61696e696e6760701b602082015260225482516323e941b960e11b81529251613164936001600160a01b03909216916347d283729160048083019260a09291908290030181865afa158015613132573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061315691906153c6565b6020015162ffffff1661406e565b602254604080516321767f9560e11b815290516000926001600160a01b0316916342ecff2a9160048083019260209291908290030181865afa1580156131ae573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131d29190615444565b9050806001600160401b03166000036132465760405162461bcd60e51b815260206004820152603060248201527f557365722e5f636f6d706c657465436865636b706f696e743a206e6f2065786960448201526f1cdd1a5b99c818da1958dadc1bda5b9d60821b60648201526084016118b2565b60205460405163b1b6f6a160e01b81526000916001600160a01b03169063b1b6f6a19061327a90602390869060040161545f565b600060405180830381865afa158015613297573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526132bf91908101906155c6565b90506132e8604051806060016040528060228152602001615dc06022913982602001515161406e565b6022548151602083015160405163783a5d3160e11b81526001600160a01b039093169263f074ba6292612acd92909160040161572b565b81831461081a577f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf35838160405161335591906157c3565b60405180910390a161081a838361408d565b606060008360a00151516001600160401b03811115613388576133886143c5565b6040519080825280602002602001820160405280156133b1578160200160208202803683370190505b50905060005b81518110156136255760008560a0015182815181106133d8576133d86150ec565b6020026020010151905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b0316816001600160a01b0316036135875773beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0838381518110613436576134366150ec565b60200260200101906001600160a01b031690816001600160a01b03168152505084156135825761347d604051806060016040528060328152602001615d8e6032913961417e565b61348d613488612774565b613a16565b50602060009054906101000a90046001600160a01b03166001600160a01b03166359d095dd6040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156134de57600080fd5b505af11580156134f2573d6000803e3d6000fd5b505050506134fe612f84565b602260009054906101000a90046001600160a01b03166001600160a01b0316632340e8d36040518163ffffffff1660e01b8152600401602060405180830381865afa158015613551573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906135759190614e30565b1561358257613582613018565b61361c565b806001600160a01b0316632495a5996040518163ffffffff1660e01b8152600401602060405180830381865afa1580156135c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906135e99190615102565b8383815181106135fb576135fb6150ec565b60200260200101906001600160a01b031690816001600160a01b0316815250505b506001016133b7565b50601c54604051630e4cc3f960e41b81526001600160a01b039091169063e4cc3f909061365a908790859088906004016157f2565b600060405180830381600087803b15801561367457600080fd5b505af1158015613688573d6000803e3d6000fd5b50929695505050505050565b600080516020615d438339815191526136ae612ee961291f565b6136b784614046565b836040516020016136ca9392919061582a565b60408051601f19818403018152908290526136e491614dd9565b60405180910390a15050565b601c546040516366d5ba9360e01b81526001600160a01b0383811660048301526060926000928392909116906366d5ba9390602401600060405180830381865afa158015613742573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261376a9190810190615889565b91509150600082516001600160401b03811115613789576137896143c5565b6040519080825280602002602001820160405280156137c257816020015b6137af614337565b8152602001906001900390816137a75790505b50601c54604051631976849960e21b81526001600160a01b038881166004830152929350600092909116906365da126490602401602060405180830381865afa158015613813573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906138379190615102565b601c5460405163285e212160e21b81526001600160a01b0389811660048301529293506000929091169063a178848490602401602060405180830381865afa158015613887573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906138ab9190614e30565b905060005b8551811015613a0a57604080516001808252818301909252600091602080830190803683375050604080516001808252818301909252929350600092915060208083019080368337019050509050878381518110613910576139106150ec565b60200260200101518260008151811061392b5761392b6150ec565b60200260200101906001600160a01b031690816001600160a01b03168152505086838151811061395d5761395d6150ec565b602002602001015181600081518110613978576139786150ec565b6020026020010181815250506040518060e001604052808b6001600160a01b03168152602001866001600160a01b031681526020018b6001600160a01b0316815260200184866139c8919061594a565b81526020014263ffffffff168152602001838152602001828152508684815181106139f5576139f56150ec565b602090810291909101015250506001016138b0565b50919695505050505050565b6000613a586040518060400160405280601881526020017f2d2065786974696e67206e756d2076616c696461746f72730000000000000000815250835161406e565b60005b8251811015613b155760205483516001600160a01b039091169063f8f98a4e90859084908110613a8d57613a8d6150ec565b60200260200101516040518263ffffffff1660e01b8152600401613abe919064ffffffffff91909116815260200190565b6020604051808303816000875af1158015613add573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613b019190615444565b613b0b908361595d565b9150600101613a5b565b506112046040518060400160405280601e81526020017f2d206578697465642062616c616e636520746f20706f64202867776569290000815250826001600160401b031661406e565b606060004781613b776801bc16d674ec80000083615992565b9050613b8c816801bc16d674ec8000006159a6565b613b9690836159bd565b9150600081670de0b6b3a76400008410613bde57613bb8633b9aca00856159d0565b613bc290856159bd565b9150613bce82856159bd565b935080613bda816152e3565b9150505b80600003613c4b5760405162461bcd60e51b815260206004820152603460248201527f737461727456616c696461746f72733a206e6f7420656e6f75676820455448206044820152733a379039ba30b93a1030903b30b634b230ba37b960611b60648201526084016118b2565b6000816001600160401b03811115613c6557613c656143c5565b604051908082528060200260200182016040528015613c8e578160200160208202803683370190505b5090506000633b9aca00613ca287476159bd565b613cac9190615992565b9050613cee6040518060400160405280601981526020017f2d206372656174696e67206e65772076616c696461746f727300000000000000815250835161406e565b613d196040518060600160405280602b8152602001615d63602b9139826001600160401b031661406e565b60005b85811015613e34576020546000906001600160a01b031663ed3c16056801bc16d674ec800000613d4a61419b565b6040518363ffffffff1660e01b8152600401613d669190614dd9565b60206040518083038185885af1158015613d84573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190613da991906159e4565b905080848381518110613dbe57613dbe6150ec565b64ffffffffff9283166020918202929092010152602380546001818101835560009290925260068082047fd57b2b5166478fd4318d2acc6cc2c704584312bdd8781b32d5d06abda57f4230018054958516600592909306919091026101000a918202919093021990931692909217905501613d1c565b50613e4085600161594a565b8303613f54576020546000906001600160a01b031663ed3c160586613e6361419b565b6040518363ffffffff1660e01b8152600401613e7f9190614dd9565b60206040518083038185885af1158015613e9d573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190613ec291906159e4565b9050808360018551613ed491906159bd565b81518110613ee457613ee46150ec565b64ffffffffff92831660209182029290920101526023805460018101825560009190915260068082047fd57b2b5166478fd4318d2acc6cc2c704584312bdd8781b32d5d06abda57f4230018054948416600592909306919091026101000a91820291909202199092169190911790555b9097909650945050505050565b6020546040516352851d0d60e11b81526000916001600160a01b03169063a50a3a1a90613f92908590600401614dc6565b600060405180830381865afa158015613faf573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052613fd79190810190615a83565b6022548151602083015160408085015160608601519151633f65cf1960e01b81529596506001600160a01b0390941694633f65cf19946107b29493928992600401615c2e565b6060610850604051806040016040528060058152602001641b5b39366d60d81b815250836141e1565b6060610850604051806040016040528060048152602001631b5b336d60e01b815250836141e1565b600080516020615de283398151915282826040516136e4929190615cdd565b80821461081c57600080516020615d438339815191526040516140ec9060208082526022908201527f4572726f723a2061203d3d2062206e6f7420736174697366696564205b75696e604082015261745d60f01b606082015260800190565b60405180910390a160408051818152600a81830152690808080808081319599d60b21b6060820152602081018490529051600080516020615de28339815191529181900360800190a160408051818152600a81830152690808080808149a59da1d60b21b6060820152602081018390529051600080516020615de28339815191529181900360800190a161081c61422b565b600080516020615d4383398151915281604051612f229190614dd9565b60225460408051600160f81b6020820152600060218201526bffffffffffffffffffffffff19606093841b16602c82015201604051602081830303815290604052905090565b60608282604051806040016040528060048152602001631b5b306d60e01b81525060405160200161421493929190615cff565b604051602081830303815290604052905092915050565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156143265760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b9282019290925260016060820152600091907f70ca10bbd0dbfd9020a9f4b13402c16cb120705e0d1c0aeab10fa353ae586fc49060800160408051601f19818403018152908290526142c5929160200161533d565b60408051601f19818403018152908290526142df9161536e565b6000604051808303816000865af19150503d806000811461431c576040519150601f19603f3d011682016040523d82523d6000602084013e614321565b606091505b505050505b6000805461ff001916610100179055565b6040518060e0016040528060006001600160a01b0316815260200160006001600160a01b0316815260200160006001600160a01b0316815260200160008152602001600063ffffffff16815260200160608152602001606081525090565b64ffffffffff8116811461220357600080fd5b6000602082840312156143ba57600080fd5b8135612cf581614395565b634e487b7160e01b600052604160045260246000fd5b60405160e081016001600160401b03811182821017156143fd576143fd6143c5565b60405290565b604080519081016001600160401b03811182821017156143fd576143fd6143c5565b604051606081016001600160401b03811182821017156143fd576143fd6143c5565b60405160a081016001600160401b03811182821017156143fd576143fd6143c5565b604051608081016001600160401b03811182821017156143fd576143fd6143c5565b604051601f8201601f191681016001600160401b03811182821017156144b3576144b36143c5565b604052919050565b60006001600160401b038211156144d4576144d46143c5565b50601f01601f191660200190565b600080604083850312156144f557600080fd5b8235915060208301356001600160401b0381111561451257600080fd5b8301601f8101851361452357600080fd5b8035614536614531826144bb565b61448b565b81815286602083850101111561454b57600080fd5b816020840160208301376000602083830101528093505050509250929050565b602080825282518282018190526000918401906040840190835b818110156145ac5783516001600160a01b0316835260209384019390920191600101614585565b509095945050505050565b60006001600160401b038211156145d0576145d06143c5565b5060051b60200190565b6001600160a01b038116811461220357600080fd5b600082601f83011261460057600080fd5b813561460e614531826145b7565b8082825260208201915060208360051b86010192508583111561463057600080fd5b602085015b83811015614656578035614648816145da565b835260209283019201614635565b5095945050505050565b6000806040838503121561467357600080fd5b82356001600160401b0381111561468957600080fd5b614695858286016145ef565b92505060208301356001600160401b038111156146b157600080fd5b8301601f810185136146c257600080fd5b80356146d0614531826145b7565b8082825260208201915060208360051b8501019250878311156146f257600080fd5b6020840193505b828410156147145783358252602093840193909101906146f9565b809450505050509250929050565b600082601f83011261473357600080fd5b8135614741614531826145b7565b8082825260208201915060208360051b86010192508583111561476357600080fd5b602085015b83811015614656578035835260209283019201614768565b6000806040838503121561479357600080fd5b82356001600160401b038111156147a957600080fd5b6147b5858286016145ef565b92505060208301356001600160401b038111156147d157600080fd5b6147dd85828601614722565b9150509250929050565b600081518084526020840193506020830160005b828110156148225781516001600160a01b03168652602095860195909101906001016147fb565b5093949350505050565b600081518084526020840193506020830160005b82811015614822578151865260209586019590910190600101614840565b80516001600160a01b03908116835260208083015182169084015260408083015190911690830152606080820151908301526080808201516000916148aa9085018263ffffffff169052565b5060a082015160e060a08501526148c460e08501826147e7565b905060c083015184820360c08601526148dd828261482c565b95945050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f1987860301845261492a85835161485e565b9450602093840193919091019060010161490e565b8035611204816145da565b803563ffffffff8116811461120457600080fd5b600060e0828403121561497057600080fd5b6149786143db565b90506149838261493f565b81526149916020830161493f565b60208201526149a26040830161493f565b6040820152606082810135908201526149bd6080830161494a565b608082015260a08201356001600160401b038111156149db57600080fd5b6149e7848285016145ef565b60a08301525060c08201356001600160401b03811115614a0657600080fd5b614a1284828501614722565b60c08301525092915050565b600060208284031215614a3057600080fd5b81356001600160401b03811115614a4657600080fd5b8201601f81018413614a5757600080fd5b8035614a65614531826145b7565b8082825260208201915060208360051b850101925086831115614a8757600080fd5b602084015b83811015614ac85780356001600160401b03811115614aaa57600080fd5b614ab98960208389010161495e565b84525060209283019201614a8c565b509695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f19878603018452614b178583516147e7565b94506020938401939190910190600101614afb565b600060208284031215614b3e57600080fd5b8135612cf5816145da565b600060208284031215614b5b57600080fd5b81356001600160401b03811115614b7157600080fd5b8201601f81018413614b8257600080fd5b8035614b90614531826145b7565b8082825260208201915060208360051b850101925086831115614bb257600080fd5b6020840193505b82841015614bdd578335614bcc81614395565b825260209384019390910190614bb9565b9695505050505050565b600060208284031215614bf957600080fd5b81356001600160401b03811115614c0f57600080fd5b614c1b8482850161495e565b949350505050565b602081526000612cf560208301846147e7565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b80831015614ccc5783516001600160e01b03191682526020938401936001939093019290910190614ca0565b50965050506020938401939190910190600101614c5e565b60005b83811015614cff578181015183820152602001614ce7565b50506000910152565b60008151808452614d20816020860160208601614ce4565b601f01601f19169290920160200192915050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f19878603018452614d78858351614d08565b94506020938401939190910190600101614d5c565b600081518084526020840193506020830160005b8281101561482257815164ffffffffff16865260209586019590910190600101614da1565b602081526000612cf56020830184614d8d565b602081526000612cf56020830184614d08565b600060208284031215614dfe57600080fd5b5035919050565b604081526000614e186040830185614d8d565b90506001600160401b03831660208301529392505050565b600060208284031215614e4257600080fd5b5051919050565b80516001600160401b038116811461120457600080fd5b6000614e6e614531846144bb565b9050828152838383011115614e8257600080fd5b612cf5836020830184614ce4565b600082601f830112614ea157600080fd5b612cf583835160208501614e60565b600060408284031215614ec257600080fd5b614eca614403565b8251815260208301519091506001600160401b03811115614eea57600080fd5b614ef684828501614e90565b60208301525092915050565b600082601f830112614f1357600080fd5b8151614f21614531826145b7565b8082825260208201915060208360051b860101925085831115614f4357600080fd5b602085015b83811015614656578051835260209283019201614f48565b600060208284031215614f7257600080fd5b81516001600160401b03811115614f8857600080fd5b820160608185031215614f9a57600080fd5b614fa2614425565b614fab82614e49565b815260208201516001600160401b03811115614fc657600080fd5b614fd286828501614eb0565b60208301525060408201516001600160401b03811115614ff157600080fd5b91909101906040828603121561500657600080fd5b61500e614403565b82516001600160401b0381111561502457600080fd5b61503087828601614f02565b82525060208301516001600160401b0381111561504c57600080fd5b61505887828601614e90565b6020830152506040820152949350505050565b805182526000602082015160406020850152614c1b6040850182614d08565b6001600160401b03841681526060602082015260006150ac606083018561506b565b82810360408401528351604082526150c7604083018261482c565b9050602085015182820360208401526150e08282614d08565b98975050505050505050565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561511457600080fd5b8151612cf5816145da565b60006020828403121561513157600080fd5b81518015158114612cf557600080fd5b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b8281101561368857603f19878603018452815180516060875261518f60608801826147e7565b9050602082015187820360208901526151a8828261482c565b6040938401516001600160a01b031698909301979097525094506020938401939190910190600101615169565b6000602082840312156151e757600080fd5b81516001600160401b038111156151fd57600080fd5b614c1b84828501614f02565b60006020828403121561521b57600080fd5b81516001600160401b0381111561523157600080fd5b8201601f8101841361524257600080fd5b614c1b84825160208401614e60565b6001600160a01b038781168252868116602083015260408201869052841660608201526080810183905260c060a082018190526000906150e090830184614d08565b600181811c908216806152a757607f821691505b6020821081036152c757634e487b7160e01b600052602260045260246000fd5b50919050565b634e487b7160e01b600052601160045260246000fd5b6000600182016152f5576152f56152cd565b5060010190565b60018060a01b038416815260606020820152600083516040606084015261532660a0840182614d08565b602095909501516080840152505060400152919050565b6001600160e01b0319831681528151600090615360816004850160208701614ce4565b919091016004019392505050565b60008251615380818460208701614ce4565b9190910192915050565b6000835161539c818460208801614ce4565b601760f91b90830190815283516153ba816001840160208801614ce4565b01600101949350505050565b600060a08284031280156153d957600080fd5b5060006153e4614447565b83518152602084015162ffffff811681146153fd578283fd5b602082015261540e60408501614e49565b604082015260608401518060070b8114615426578283fd5b606082015261543760808501614e49565b6080820152949350505050565b60006020828403121561545657600080fd5b612cf582614e49565b6000604082016040835280855461547a818490815260200190565b60008881526020812094509092505b816005820110156154f057835464ffffffffff8082168552602882901c81166020860152605082901c81166040860152607882901c8116606086015260a082811c8216608087015260c89290921c169084015260019093019260c090920191600601615489565b9254928181101561550f5764ffffffffff841683526020909201916001015b8181101561552f5764ffffffffff602885901c1683526020909201916001015b8181101561554f5764ffffffffff605085901c1683526020909201916001015b8181101561556f5764ffffffffff607885901c1683526020909201916001015b8181101561558f5764ffffffffff60a085901c1683526020909201916001015b818110156155ac5760c884901c64ffffffffff1683526020830192505b50506001600160401b03851660208501529150612cf59050565b6000602082840312156155d857600080fd5b81516001600160401b038111156155ee57600080fd5b82016040818503121561560057600080fd5b615608614403565b81516001600160401b0381111561561e57600080fd5b61562a86828501614eb0565b82525060208201516001600160401b0381111561564657600080fd5b80830192505084601f83011261565b57600080fd5b8151615669614531826145b7565b8082825260208201915060208360051b86010192508783111561568b57600080fd5b602085015b8381101561571a5780516001600160401b038111156156ae57600080fd5b86016060818b03601f190112156156c457600080fd5b6156cc614425565b602082810151825260408301519082015260608201516001600160401b038111156156f657600080fd5b6157058c602083860101614e90565b60408301525084525060209283019201615690565b506020840152509095945050505050565b60408152600061573e604083018561506b565b828103602084015280845180835260208301915060208160051b8401016020870160005b838110156157b557601f1986840301855281518051845260208101516020850152604081015190506060604085015261579e6060850182614d08565b602096870196909450929092019150600101615762565b509098975050505050505050565b60408152600560408201526422b93937b960d91b6060820152608060208201526000612cf56080830184614d08565b606081526000615805606083018661485e565b828103602084015261581781866147e7565b9150508215156040830152949350505050565b6000845161583c818460208901614ce4565b601760f91b908301908152845161585a816001840160208901614ce4565b601d60f91b60019290910191820152835161587c816002840160208801614ce4565b0160020195945050505050565b6000806040838503121561589c57600080fd5b82516001600160401b038111156158b257600080fd5b8301601f810185136158c357600080fd5b80516158d1614531826145b7565b8082825260208201915060208360051b8501019250878311156158f357600080fd5b6020840193505b8284101561591e57835161590d816145da565b8252602093840193909101906158fa565b8095505050505060208301516001600160401b0381111561593e57600080fd5b6147dd85828601614f02565b80820180821115610850576108506152cd565b6001600160401b038181168382160190811115610850576108506152cd565b634e487b7160e01b600052601260045260246000fd5b6000826159a1576159a161597c565b500490565b8082028115828204841417610850576108506152cd565b81810381811115610850576108506152cd565b6000826159df576159df61597c565b500690565b6000602082840312156159f657600080fd5b8151612cf581614395565b600082601f830112615a1257600080fd5b8151615a20614531826145b7565b8082825260208201915060208360051b860101925085831115615a4257600080fd5b602085015b838110156146565780516001600160401b03811115615a6557600080fd5b615a74886020838a0101614f02565b84525060209283019201615a47565b600060208284031215615a9557600080fd5b81516001600160401b03811115615aab57600080fd5b820160808185031215615abd57600080fd5b615ac5614469565b615ace82614e49565b815260208201516001600160401b03811115615ae957600080fd5b615af586828501614eb0565b60208301525060408201516001600160401b03811115615b1457600080fd5b8201601f81018613615b2557600080fd5b8051615b33614531826145b7565b8082825260208201915060208360051b850101925088831115615b5557600080fd5b602084015b83811015615b965780516001600160401b03811115615b7857600080fd5b615b878b602083890101614e90565b84525060209283019201615b5a565b50604085015250505060608201516001600160401b03811115615bb857600080fd5b615bc486828501615a01565b606083015250949350505050565b600082825180855260208501945060208160051b8301016020850160005b83811015615c2257601f19858403018852615c0c83835161482c565b6020988901989093509190910190600101615bf0565b50909695505050505050565b6001600160401b038616815260a060208201526000615c5060a083018761506b565b8281036040840152615c628187614d8d565b9050828103606084015280855180835260208301915060208160051b8401016020880160005b83811015615cba57601f19868403018552615ca4838351614d08565b6020958601959093509190910190600101615c88565b50508581036080870152615cce8188615bd2565b9b9a5050505050505050505050565b604081526000615cf06040830185614d08565b90508260208301529392505050565b60008451615d11818460208901614ce4565b845190830190615d25818360208901614ce4565b8451910190615d38818360208801614ce4565b019594505050505056fe41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f502d206465706f736974696e672062616c616e636520746f20626561636f6e20636861696e202867776569292d2065786974696e6720616c6c2076616c696461746f727320616e6420636f6d706c6574696e6720636865636b706f696e742d207375626d697474696e67206e756d20636865636b706f696e742070726f6f6673b2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8557365722e71756575655769746864726177616c733a206c656e677468206d69736d61746368a2646970667358221220bc4f9e60413b69f34b8d56f3337e68c7bbb931fa94fbc14959d7c5aee9e988fd64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xF2W`\x005`\xE0\x1C\x80cm3oX\x11a\x01\rW\x80c\xA8\x8D\xBB6\x11a\0\xA0W\x80c\xBAAO\xA6\x11a\0oW\x80c\xBAAO\xA6\x14a\x05\xD2W\x80c\xD6\xC1\r\xAF\x14a\x05\xE7W\x80c\xE2\x0C\x9Fq\x14a\x05\xFCW\x80c\xF24\xC1\xBD\x14a\x06\x11W\x80c\xFAv&\xD4\x14a\x064W`\0\x80\xFD[\x80c\xA8\x8D\xBB6\x14a\x05%W\x80c\xACc|z\x14a\x05]W\x80c\xB5P\x8A\xA9\x14a\x05}W\x80c\xB7\x17#U\x14a\x05\x92W`\0\x80\xFD[\x80c\x91j\x17\xC6\x11a\0\xDCW\x80c\x91j\x17\xC6\x14a\x04\xB7W\x80c\x92\xAB\x89\xBB\x14a\x04\xCCW\x80c\x9D\xE7\x02X\x14a\x04\xE1W\x80c\xA3\xF4\xDF~\x14a\x05\x03W`\0\x80\xFD[\x80cm3oX\x14a\x04@W\x80c\x84\x1C\x12\x99\x14a\x04`W\x80c\x85\"l\x81\x14a\x04\x80W\x80c\x90\xB5\x16%\x14a\x04\xA2W`\0\x80\xFD[\x80c=\x8C\x08\xD4\x11a\x01\x85W\x80cF\xA5\xBE\r\x11a\x01TW\x80cF\xA5\xBE\r\x14a\x03\xBEW\x80ceK\xB5\xD9\x14a\x03\xDEW\x80cf\xD9\xA9\xA0\x14a\x03\xFEW\x80ci_J\xE1\x14a\x04 W`\0\x80\xFD[\x80c=\x8C\x08\xD4\x14a\x03/W\x80c>^<#\x14a\x03gW\x80c?r\x86\xF4\x14a\x03|W\x80c@\x1B\xE6^\x14a\x03\x91W`\0\x80\xFD[\x80c#\xE4\x81u\x11a\x01\xC1W\x80c#\xE4\x81u\x14a\x02\xA0W\x80c*4\xAD\xE8\x14a\x02\xCDW\x80c4N\x13\x83\x14a\x02\xE2W\x80c9\x1C\xC9\xF6\x14a\x03\x0FW`\0\x80\xFD[\x80c\x07\x1C%\xB7\x14a\x01\xFEW\x80c\x16&\xBA~\x14a\x02 W\x80c\x1E\xD7\x83\x1C\x14a\x02^W\x80c \xA5E\xD9\x14a\x02\x80W`\0\x80\xFD[6a\x01\xF9W\0[`\0\x80\xFD[4\x80\x15a\x02\nW`\0\x80\xFD[Pa\x02\x1Ea\x02\x196`\x04aC\xA8V[a\x06NV[\0[4\x80\x15a\x02,W`\0\x80\xFD[Pa\x02@a\x02;6`\x04aD\xE2V[a\x08 V[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02jW`\0\x80\xFD[Pa\x02sa\x08VV[`@Qa\x02U\x91\x90aEkV[4\x80\x15a\x02\x8CW`\0\x80\xFD[Pa\x02\x1Ea\x02\x9B6`\x04aF`V[a\x08\xB8V[4\x80\x15a\x02\xACW`\0\x80\xFD[Pa\x02\xC0a\x02\xBB6`\x04aG\x80V[a\x0B\xD7V[`@Qa\x02U\x91\x90aH\xE6V[4\x80\x15a\x02\xD9W`\0\x80\xFD[Pa\x02\x1Ea\x0F7V[4\x80\x15a\x02\xEEW`\0\x80\xFD[Pa\x03\x02a\x02\xFD6`\x04aJ\x1EV[a\x10\xABV[`@Qa\x02U\x91\x90aJ\xD3V[4\x80\x15a\x03\x1BW`\0\x80\xFD[Pa\x02\xC0a\x03*6`\x04aK,V[a\x12\tV[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x03Oa\x03J6`\x04aKIV[a\x13\x91V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x03sW`\0\x80\xFD[Pa\x02sa\x149V[4\x80\x15a\x03\x88W`\0\x80\xFD[Pa\x02sa\x14\x99V[4\x80\x15a\x03\x9DW`\0\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04aK\xE7V[a\x14\xF9V[`@Qa\x02U\x91\x90aL#V[4\x80\x15a\x03\xCAW`\0\x80\xFD[Pa\x03\x02a\x03\xD96`\x04aJ\x1EV[a\x15\xB3V[4\x80\x15a\x03\xEAW`\0\x80\xFD[Pa\x02\x1Ea\x03\xF96`\x04aG\x80V[a\x17\x08V[4\x80\x15a\x04\nW`\0\x80\xFD[Pa\x04\x13a\x1C\x86V[`@Qa\x02U\x91\x90aL6V[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x03\xB1a\x04;6`\x04aK\xE7V[a\x1DuV[4\x80\x15a\x04LW`\0\x80\xFD[Pa\x02\x1Ea\x04[6`\x04aG\x80V[a\x1E/V[4\x80\x15a\x04lW`\0\x80\xFD[Pa\x02\x1Ea\x04{6`\x04aKIV[a!BV[4\x80\x15a\x04\x8CW`\0\x80\xFD[Pa\x04\x95a\"\x06V[`@Qa\x02U\x91\x90aM4V[4\x80\x15a\x04\xAEW`\0\x80\xFD[Pa\x02\x1Ea\"\xD6V[4\x80\x15a\x04\xC3W`\0\x80\xFD[Pa\x04\x13a#\x8AV[4\x80\x15a\x04\xD8W`\0\x80\xFD[Pa\x02\xC0a$pV[4\x80\x15a\x04\xEDW`\0\x80\xFD[Pa\x04\xF6a'tV[`@Qa\x02U\x91\x90aM\xC6V[4\x80\x15a\x05\x0FW`\0\x80\xFD[Pa\x05\x18a)\x1FV[`@Qa\x02U\x91\x90aM\xD9V[4\x80\x15a\x051W`\0\x80\xFD[P`\"Ta\x05E\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02UV[4\x80\x15a\x05iW`\0\x80\xFD[Pa\x02\x1Ea\x05x6`\x04aK,V[a)\xA8V[4\x80\x15a\x05\x89W`\0\x80\xFD[Pa\x04\x95a+\x03V[4\x80\x15a\x05\x9EW`\0\x80\xFD[Pa\x05\xC2a\x05\xAD6`\x04aM\xECV[`&` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02UV[4\x80\x15a\x05\xDEW`\0\x80\xFD[Pa\x05\xC2a+\xD3V[4\x80\x15a\x05\xF3W`\0\x80\xFD[Pa\x02\x1Ea,\xFCV[4\x80\x15a\x06\x08W`\0\x80\xFD[Pa\x02sa-\xB1V[4\x80\x15a\x06\x1DW`\0\x80\xFD[Pa\x06&a.\x11V[`@Qa\x02U\x92\x91\x90aN\x05V[4\x80\x15a\x06@W`\0\x80\xFD[P`\0Ta\x05\xC2\x90`\xFF\x16\x81V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC7\x91\x90aN0V[Pa\x06\xFB`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qverifyStaleBalance`p\x1B\x81RPa.\xCFV[` T`@Qc\x08\xFA\x0B\x13`\xE2\x1B\x81Rd\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xE8,L\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07s\x91\x90\x81\x01\x90aO`V[`\"T\x81Q` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x01\xC8\xAB\xE9`\xE1\x1B\x81R\x94\x95P`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93c\x03\x91W\xD2\x93a\x07\xB2\x93\x92\x91`\x04\x01aP\x8AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xCCW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x07\xDDWP`\x01[a\x08\x1CW=\x80\x80\x15a\x08\x0BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x10V[``\x91P[Pa\x08\x1A\x81a/-V[P[PPV[`\0\x82\x81R`&` R`@\x81 T`\xFF\x16\x15a\x08EWPc\x0B\x13]?`\xE1\x1Ba\x08PV[P`\x01`\x01`\xE0\x1B\x03\x19[\x92\x91PPV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90W[PPPPP\x90P\x90V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t1\x91\x90aN0V[Pa\ta`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mupdateBalances`\x90\x1B\x81RPa.\xCFV[`\0[\x82Q\x81\x10\x15a\x08\x1AW`\0\x83\x82\x81Q\x81\x10a\t\x81Wa\t\x81aP\xECV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\t\x9FWa\t\x9FaP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\ngWa\t\xDEa/\x84V[`\"`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nU\x91\x90aN0V[\x15a\nbWa\nba0\x18V[a\x0B\xCDV[`\0\x81\x90P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD0\x91\x90aQ\x02V[`\x1DT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BI\x91\x90aQ\x1FV[P`\x1DT`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC9\x91\x90aN0V[PPP[PP`\x01\x01a\tdV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x0C#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CG\x91\x90aN0V[Pa\x0Cy`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01oqueueWithdrawals`\x80\x1B\x81RPa.\xCFV[`\x1CT`@Qc\x19v\x84\x99`\xE2\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE6\x91\x90aQ\x02V[`\x1CT`@Qc(^!!`\xE2\x1B\x81R0`\x04\x82\x01\x81\x90R\x92\x93P`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rX\x91\x90aN0V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90\x81` \x01[`@\x80Q``\x80\x82\x01\x83R\x80\x82R` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rsW\x90PP\x90P`@Q\x80``\x01`@R\x80\x88\x81R` \x01\x87\x81R` \x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x81`\0\x81Q\x81\x10a\r\xDBWa\r\xDBaP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[a\x0E\x05aC7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xFDW\x90PP\x90P`@Q\x80`\xE0\x01`@R\x800`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81R` \x01Bc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89\x81R` \x01\x88\x81RP\x81`\0\x81Q\x81\x10a\x0E\x81Wa\x0E\x81aP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x1CT`@Qc\x06\xECn\x81`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\r\xD8\xDD\x02\x90a\x0E\xBD\x90\x86\x90`\x04\x01aQAV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\x04\x91\x90\x81\x01\x90aQ\xD5V[\x90Pa\x0F+\x82Q\x82Q`@Q\x80``\x01`@R\x80`&\x81R` \x01a^\x02`&\x919a3\x1FV[P\x97\x96PPPPPPPV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0F\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB0\x91\x90aN0V[Pa\x0F\xE4`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q92\xB3\xB4\xB9\xBA2\xB9 \xB9\xA7\xB82\xB90\xBA7\xB9`q\x1B\x81RPa.\xCFV[`@\x80Q``\x81\x01\x82R0\x81R`\0` \x82\x01\x81\x81R\x82\x84\x01\x91\x82R`\x1CT`$\x80T\x95Qc\x02K\x98\x03`\xE5\x1B\x81R\x85Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x93Q\x84\x16\x91\x81\x01\x91\x90\x91R\x92Qc\xFF\xFF\xFF\xFF\x90\x81\x16`D\x85\x01R\x90\x94\x16`d\x83\x01R`\xA0`\x84\x83\x01R`\x08`\xA4\x83\x01Rgmetadata`\xC0\x1B`\xC4\x83\x01R\x91\x92\x91\x90\x91\x16\x90cIs\0`\x90`\xE4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\x90W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xA4W=`\0\x80>=`\0\xFD[PPPPPV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x10\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x1B\x91\x90aN0V[Pa\x11Z`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa.\xCFV[`\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11uWa\x11uaC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xA8W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x11\x93W\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12\0Wa\x11\xDB\x84\x82\x81Q\x81\x10a\x11\xCCWa\x11\xCCaP\xECV[` \x02` \x01\x01Q`\x01a3gV[\x82\x82\x81Q\x81\x10a\x11\xEDWa\x11\xEDaP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x11\xAEV[P\x90P[\x91\x90PV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x12UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12y\x91\x90aN0V[Pa\x13\x10`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nforceUndelegate`\x88\x1B\x81RP\x83`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\x0B\x91\x90\x81\x01\x90aR\tV[a6\x94V[`\0a\x13\x1B\x83a6\xF0V[`\x1CT`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\xDA\x8B\xE8d\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x13iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\0\x91\x90\x81\x01\x90aQ\xD5V[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x87\x87Z\xF1\x15\x80\x15a\x13\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\0\x91\x90aN0V[Pa\x140`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01mexitValidators`\x90\x1B\x81RPa.\xCFV[a\x08P\x82a:\x16V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90WPPPPP\x90P\x90V[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x15EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15i\x91\x90aN0V[Pa\x15\xA8`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FcompleteWithdrawalsAsTokens\0\0\0\0\0\x81RPa.\xCFV[a\x08P\x82`\x01a3gV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x15\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16#\x91\x90aN0V[Pa\x16b`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa.\xCFV[`\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16}Wa\x16}aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xB0W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\x9BW\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12\0Wa\x16\xE3\x84\x82\x81Q\x81\x10a\x16\xD4Wa\x16\xD4aP\xECV[` \x02` \x01\x01Q`\0a3gV[\x82\x82\x81Q\x81\x10a\x16\xF5Wa\x16\xF5aP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x16\xB6V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x81\x91\x90aN0V[Pa\x17\xC0`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F.depositIntoEigenlayer_M1_ALT\0\0\0\x81RPa.\xCFV[`\0\x19`\0[\x83Q\x81\x10\x15a\x1C\x80W`\0\x84\x82\x81Q\x81\x10a\x17\xE3Wa\x17\xE3aP\xECV[` \x02` \x01\x01Q\x90P`\0\x84\x83\x81Q\x81\x10a\x18\x01Wa\x18\x01aP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x18\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FShould not be depositing with BE`D\x82\x01R\x7FACONCHAIN_ETH_STRAT for M1-mainn`d\x82\x01Rf2\xBA\x10*\xB9\xB2\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x1F\x91\x90aQ\x02V[`\x1DT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x19tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x98\x91\x90aQ\x1FV[P`\x1DT`@Qb?g_`\xE9\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c~\xCE\xBE\0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x05\x91\x90aN0V[\x90P`\0`\x1D`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cH\x82^\x94`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x80\x91\x90aN0V[`@\x80Q` \x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x91\x83\x01\x91\x90\x91R\x84\x16``\x82\x01R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x83\x90R`\xC0\x81\x01\x88\x90R`\xE0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x1DTc\xF6\x98\xDA%`\xE0\x1B\x85R\x92Q\x90\x94P`\0\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF6\x98\xDA%\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BA\x91\x90aN0V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x82\x90R`B\x81\x01\x84\x90R\x90\x91P`\0\x90`b\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x81\x90R\x92P`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\x01`&`\0\x84\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x1D`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c2\xE8\x9A\xCE\x89\x88\x8A0\x8F\x87`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\x12\x96\x95\x94\x93\x92\x91\x90aRQV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1C1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CU\x91\x90aN0V[PP`\0\x90\x81R`&` R`@\x90 \x80T`\xFF\x19\x16\x90UPPP`\x01\x93\x90\x93\x01\x92Pa\x17\xC6\x91PPV[PPPPV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1DTW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1D\x16W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1C\xAAV[PPPP\x90P\x90V[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a\x1D\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xE5\x91\x90aN0V[Pa\x1E$`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FcompleteWithdrawalAsShares\0\0\0\0\0\0\x81RPa.\xCFV[a\x08P\x82`\0a3gV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1E\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xA8\x91\x90aN0V[Pa\x1E\xDF`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t22\xB87\xB9\xB4\xBA$\xB7:7\xA2\xB4\xB3\xB2\xB760\xBC\xB2\xB9`Y\x1B\x81RPa.\xCFV[`\0[\x82Q\x81\x10\x15a\x08\x1AW`\0\x83\x82\x81Q\x81\x10a\x1E\xFFWa\x1E\xFFaP\xECV[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x1F\x1DWa\x1F\x1DaP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1F\xD8W`\0a\x1F^a;^V[P\x90P` `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xB1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\xC5W=`\0\x80>=`\0\xFD[PPPPa\x1F\xD2\x81a?aV[Pa!8V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a <\x91\x90aQ\x02V[`\x1DT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a \x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xB5\x91\x90aQ\x1FV[P`\x1DT`@Qcs\xD0(U`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x83\x81\x16`$\x83\x01R`D\x82\x01\x85\x90R\x90\x91\x16\x90c\xE7\xA0P\xAA\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!5\x91\x90aN0V[PP[PP`\x01\x01a\x1E\xE2V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a!\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xBB\x91\x90aN0V[Pa!\xFA`@Q\x80`@\x01`@R\x80`\x1B\x81R` \x01\x7FverifyWithdrawalCredentials\0\0\0\0\0\x81RPa.\xCFV[a\"\x03\x81a?aV[PV[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW\x83\x82\x90`\0R` `\0 \x01\x80Ta\"I\x90aR\x93V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"u\x90aR\x93V[\x80\x15a\"\xC2W\x80`\x1F\x10a\"\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\"*V[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a#+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#O\x91\x90aN0V[Pa#\x80`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x1C\xDD\x18\\\x9D\x10\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x8A\x1B\x81RPa.\xCFV[a#\x88a/\x84V[V[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a$XW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a$\x1AW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a#\xAEV[`\x1FT`@\x80Qc\x01PM\x8F`\xE4\x1B\x81R\x90Q``\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x15\x04\xD8\xF0\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81`\0\x87Z\xF1\x15\x80\x15a$\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xE0\x91\x90aN0V[Pa%\x0C`@Q\x80`@\x01`@R\x80`\n\x81R` \x01iundelegate`\xB0\x1B\x81RPa.\xCFV[`\0a%\x170a6\xF0V[`\x1CT`@Qc6\xA2\xFA\x19`\xE2\x1B\x81R0`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDA\x8B\xE8d\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra%\x8A\x91\x90\x81\x01\x90aQ\xD5V[P`\0[\x81Q\x81\x10\x15a'nW`\0\x80Q` a]C\x839\x81Q\x91R`@Qa%\xDC\x90` \x80\x82R`\x15\x90\x82\x01Rt2\xBC82\xB1\xBA4\xB73\x90;\xB4\xBA4290\xBB\xB0\xB6\x1D`Y\x1B`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a]\xE2\x839\x81Q\x91R\x82\x82\x81Q\x81\x10a&\x05Wa&\x05aP\xECV[` \x02` \x01\x01Q``\x01Q`@Qa&B\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03s{s\x1B)\xD1`\xCD\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1\x7F\x9CN\x85A\xCA\x8F\r\xC1\xC4\x13\xF9\x10\x8Ff\xD8-<\xEC\xB1\xBD\xDB\xCECza\xCA\xA3\x17\\L\xC9o\x82\x82\x81Q\x81\x10a&}Wa&}aP\xECV[` \x02` \x01\x01Q`\xA0\x01Q`\0\x81Q\x81\x10a&\x9BWa&\x9BaP\xECV[` \x02` \x01\x01Q`@Qa&\xDD\x91\x90`@\x80\x82R`\x07\x90\x82\x01Rf\x03\x9B\xA3\x93\x0B\xA1\xD1`\xCD\x1B``\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\0\x80Q` a]\xE2\x839\x81Q\x91R\x82\x82\x81Q\x81\x10a'\x06Wa'\x06aP\xECV[` \x02` \x01\x01Q`\xC0\x01Q`\0\x81Q\x81\x10a'$Wa'$aP\xECV[` \x02` \x01\x01Q`@Qa'^\x91\x90`@\x80\x82R`\x08\x90\x82\x01Rg\x03\x9BC\x0B\x93+\x99\xD1`\xC5\x1B``\x82\x01R` \x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a%\x8EV[P\x90P\x90V[`#T``\x90`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x94Wa'\x94aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\xBDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80`\0[`#T\x81\x10\x15a)\x16W` T`#\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xAAG8\x9C\x91\x90\x84\x90\x81\x10a'\xFAWa'\xFAaP\xECV[\x90`\0R` `\0 \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(J\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x8B\x91\x90aQ\x1FV[\x15a)\x0EW`#\x81\x81T\x81\x10a(\xA3Wa(\xA3aP\xECV[\x90`\0R` `\0 \x90`\x06\x91\x82\x82\x04\x01\x91\x90\x06`\x05\x02\x90T\x90a\x01\0\n\x90\x04d\xFF\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10a(\xDCWa(\xDCaP\xECV[d\xFF\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a(\xFC\x81aR\xE3V[\x93PP\x81\x80a)\n\x90aR\xE3V[\x92PP[`\x01\x01a'\xC6V[PP\x81R\x91\x90PV[```!\x80Ta).\x90aR\x93V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta)Z\x90aR\x93V[\x80\x15a\x08\xAEW\x80`\x1F\x10a)|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xAEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a)\x8AWP\x93\x95\x94PPPPPV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a)\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*!\x91\x90aN0V[Pa*\x86`@Q\x80`@\x01`@R\x80`\n\x81R` \x01idelegateTo`\xB0\x1B\x81RP\x82`\x01`\x01`\xA0\x1B\x03\x16c\xA3\xF4\xDF~`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xE3W=`\0\x80>=`\0\xFD[`@\x80Q\x80\x82\x01\x82R``\x81R`\0` \x82\x01\x81\x90R`\x1CT\x92Qc\xEE\xA9\x06K`\xE0\x1B\x81R\x91\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEE\xA9\x06K\x91a*\xCD\x91\x86\x91\x86\x91`\x04\x01aR\xFCV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\xE7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xFBW=`\0\x80>=`\0\xFD[PPPPPPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x1DlW\x83\x82\x90`\0R` `\0 \x01\x80Ta+F\x90aR\x93V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta+r\x90aR\x93V[\x80\x15a+\xBFW\x80`\x1F\x10a+\x94Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a+\xBFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a+\xA2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a+'V[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a+\xF3WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x12\x04W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a,\x81\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01aS=V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra,\x9B\x91aSnV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a,\xD8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a,\xDDV[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a,\xF5\x91\x90aQ\x1FV[\x93\x92PPPV[`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a-QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-u\x91\x90aN0V[Pa-\xA9`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x18\xDB\xDB\\\x1B\x19]\x19P\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`r\x1B\x81RPa.\xCFV[a#\x88a0\x18V[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xAEW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x90WPPPPP\x90P\x90V[```\0`\x1F`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x15\x04\xD8\xF0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a.jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x8E\x91\x90aN0V[Pa.\xBF`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nstartValidators`\x88\x1B\x81RPa.\xCFV[a.\xC7a;^V[\x91P\x91P\x90\x91V[`\0\x80Q` a]C\x839\x81Q\x91Ra.\xEEa.\xE9a)\x1FV[a@\x1DV[a.\xF7\x83a@FV[`@Q` \x01a/\x08\x92\x91\x90aS\x8AV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra/\"\x91aM\xD9V[`@Q\x80\x91\x03\x90\xA1PV[\x80Q\x15a/<W\x80Q\x81` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7Freverted with unknown error\0\0\0\0\0`D\x82\x01R`d\x01a\x18\xB2V[`\"T`@Qc\x88gl\xAD`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x88gl\xAD\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\xCAW`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a/\xDBWP`\x01[a#\x88W=\x80\x80\x15a0\tW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0\x0EV[``\x91P[Pa\"\x03\x81a/-V[`@\x80Q\x80\x82\x01\x82R`\x18\x81R\x7F- active validator count\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R`\"T\x83Qc#@\xE8\xD3`\xE0\x1B\x81R\x93Qa0\xBF\x94`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c#@\xE8\xD3\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a0\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xBA\x91\x90aN0V[a@nV[`@\x80Q\x80\x82\x01\x82R`\x12\x81Rq- proofs remaining`p\x1B` \x82\x01R`\"T\x82Qc#\xE9A\xB9`\xE1\x1B\x81R\x92Qa1d\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cG\xD2\x83r\x91`\x04\x80\x83\x01\x92`\xA0\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1V\x91\x90aS\xC6V[` \x01Qb\xFF\xFF\xFF\x16a@nV[`\"T`@\x80Qc!v\x7F\x95`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cB\xEC\xFF*\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a1\xAEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xD2\x91\x90aTDV[\x90P\x80`\x01`\x01`@\x1B\x03\x16`\0\x03a2FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FUser._completeCheckpoint: no exi`D\x82\x01Ro\x1C\xDD\x1A[\x99\xC8\x18\xDA\x19X\xDA\xDC\x1B\xDA[\x9D`\x82\x1B`d\x82\x01R`\x84\x01a\x18\xB2V[` T`@Qc\xB1\xB6\xF6\xA1`\xE0\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB1\xB6\xF6\xA1\x90a2z\x90`#\x90\x86\x90`\x04\x01aT_V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra2\xBF\x91\x90\x81\x01\x90aU\xC6V[\x90Pa2\xE8`@Q\x80``\x01`@R\x80`\"\x81R` \x01a]\xC0`\"\x919\x82` \x01QQa@nV[`\"T\x81Q` \x83\x01Q`@Qcx:]1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xF0t\xBAb\x92a*\xCD\x92\x90\x91`\x04\x01aW+V[\x81\x83\x14a\x08\x1AW\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x81`@Qa3U\x91\x90aW\xC3V[`@Q\x80\x91\x03\x90\xA1a\x08\x1A\x83\x83a@\x8DV[```\0\x83`\xA0\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x88Wa3\x88aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xB1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x81Q\x81\x10\x15a6%W`\0\x85`\xA0\x01Q\x82\x81Q\x81\x10a3\xD8Wa3\xD8aP\xECV[` \x02` \x01\x01Q\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a5\x87Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x83\x83\x81Q\x81\x10a46Wa46aP\xECV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x15a5\x82Wa4}`@Q\x80``\x01`@R\x80`2\x81R` \x01a]\x8E`2\x919aA~V[a4\x8Da4\x88a'tV[a:\x16V[P` `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cY\xD0\x95\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xDEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xF2W=`\0\x80>=`\0\xFD[PPPPa4\xFEa/\x84V[`\"`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c#@\xE8\xD3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5u\x91\x90aN0V[\x15a5\x82Wa5\x82a0\x18V[a6\x1CV[\x80`\x01`\x01`\xA0\x1B\x03\x16c$\x95\xA5\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xE9\x91\x90aQ\x02V[\x83\x83\x81Q\x81\x10a5\xFBWa5\xFBaP\xECV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP[P`\x01\x01a3\xB7V[P`\x1CT`@Qc\x0EL\xC3\xF9`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE4\xCC?\x90\x90a6Z\x90\x87\x90\x85\x90\x88\x90`\x04\x01aW\xF2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a6tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a6\x88W=`\0\x80>=`\0\xFD[P\x92\x96\x95PPPPPPV[`\0\x80Q` a]C\x839\x81Q\x91Ra6\xAEa.\xE9a)\x1FV[a6\xB7\x84a@FV[\x83`@Q` \x01a6\xCA\x93\x92\x91\x90aX*V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra6\xE4\x91aM\xD9V[`@Q\x80\x91\x03\x90\xA1PPV[`\x1CT`@Qcf\xD5\xBA\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R``\x92`\0\x92\x83\x92\x90\x91\x16\x90cf\xD5\xBA\x93\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7j\x91\x90\x81\x01\x90aX\x89V[\x91P\x91P`\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x89Wa7\x89aC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7\xC2W\x81` \x01[a7\xAFaC7V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a7\xA7W\x90P[P`\x1CT`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a87\x91\x90aQ\x02V[`\x1CT`@Qc(^!!`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\x04\x83\x01R\x92\x93P`\0\x92\x90\x91\x16\x90c\xA1x\x84\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xAB\x91\x90aN0V[\x90P`\0[\x85Q\x81\x10\x15a:\nW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x83\x81Q\x81\x10a9\x10Wa9\x10aP\xECV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a9+Wa9+aP\xECV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x86\x83\x81Q\x81\x10a9]Wa9]aP\xECV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a9xWa9xaP\xECV[` \x02` \x01\x01\x81\x81RPP`@Q\x80`\xE0\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x86a9\xC8\x91\x90aYJV[\x81R` \x01Bc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83\x81R` \x01\x82\x81RP\x86\x84\x81Q\x81\x10a9\xF5Wa9\xF5aP\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a8\xB0V[P\x91\x96\x95PPPPPPV[`\0a:X`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7F- exiting num validators\0\0\0\0\0\0\0\0\x81RP\x83Qa@nV[`\0[\x82Q\x81\x10\x15a;\x15W` T\x83Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xF9\x8AN\x90\x85\x90\x84\x90\x81\x10a:\x8DWa:\x8DaP\xECV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a:\xBE\x91\x90d\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a:\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\x01\x91\x90aTDV[a;\x0B\x90\x83aY]V[\x91P`\x01\x01a:[V[Pa\x12\x04`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7F- exited balance to pod (gwei)\0\0\x81RP\x82`\x01`\x01`@\x1B\x03\x16a@nV[```\0G\x81a;wh\x01\xBC\x16\xD6t\xEC\x80\0\0\x83aY\x92V[\x90Pa;\x8C\x81h\x01\xBC\x16\xD6t\xEC\x80\0\0aY\xA6V[a;\x96\x90\x83aY\xBDV[\x91P`\0\x81g\r\xE0\xB6\xB3\xA7d\0\0\x84\x10a;\xDEWa;\xB8c;\x9A\xCA\0\x85aY\xD0V[a;\xC2\x90\x85aY\xBDV[\x91Pa;\xCE\x82\x85aY\xBDV[\x93P\x80a;\xDA\x81aR\xE3V[\x91PP[\x80`\0\x03a<KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FstartValidators: not enough ETH `D\x82\x01Rs:7\x909\xBA0\xB9:\x100\x90;0\xB64\xB20\xBA7\xB9`a\x1B`d\x82\x01R`\x84\x01a\x18\xB2V[`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a<eWa<eaC\xC5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\x8EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0c;\x9A\xCA\0a<\xA2\x87GaY\xBDV[a<\xAC\x91\x90aY\x92V[\x90Pa<\xEE`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- creating new validators\0\0\0\0\0\0\0\x81RP\x83Qa@nV[a=\x19`@Q\x80``\x01`@R\x80`+\x81R` \x01a]c`+\x919\x82`\x01`\x01`@\x1B\x03\x16a@nV[`\0[\x85\x81\x10\x15a>4W` T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05h\x01\xBC\x16\xD6t\xEC\x80\0\0a=JaA\x9BV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=f\x91\x90aM\xD9V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a=\x84W=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xA9\x91\x90aY\xE4V[\x90P\x80\x84\x83\x81Q\x81\x10a=\xBEWa=\xBEaP\xECV[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`#\x80T`\x01\x81\x81\x01\x83U`\0\x92\x90\x92R`\x06\x80\x82\x04\x7F\xD5{+QfG\x8F\xD41\x8D*\xCCl\xC2\xC7\x04XC\x12\xBD\xD8x\x1B2\xD5\xD0j\xBD\xA5\x7FB0\x01\x80T\x95\x85\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x93\x02\x19\x90\x93\x16\x92\x90\x92\x17\x90U\x01a=\x1CV[Pa>@\x85`\x01aYJV[\x83\x03a?TW` T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\xED<\x16\x05\x86a>caA\x9BV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\x7F\x91\x90aM\xD9V[` `@Q\x80\x83\x03\x81\x85\x88Z\xF1\x15\x80\x15a>\x9DW=`\0\x80>=`\0\xFD[PPPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xC2\x91\x90aY\xE4V[\x90P\x80\x83`\x01\x85Qa>\xD4\x91\x90aY\xBDV[\x81Q\x81\x10a>\xE4Wa>\xE4aP\xECV[d\xFF\xFF\xFF\xFF\xFF\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`#\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R`\x06\x80\x82\x04\x7F\xD5{+QfG\x8F\xD41\x8D*\xCCl\xC2\xC7\x04XC\x12\xBD\xD8x\x1B2\xD5\xD0j\xBD\xA5\x7FB0\x01\x80T\x94\x84\x16`\x05\x92\x90\x93\x06\x91\x90\x91\x02a\x01\0\n\x91\x82\x02\x91\x90\x92\x02\x19\x90\x92\x16\x91\x90\x91\x17\x90U[\x90\x97\x90\x96P\x94PPPPPV[` T`@QcR\x85\x1D\r`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA5\n:\x1A\x90a?\x92\x90\x85\x90`\x04\x01aM\xC6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra?\xD7\x91\x90\x81\x01\x90aZ\x83V[`\"T\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc?e\xCF\x19`\xE0\x1B\x81R\x95\x96P`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x94c?e\xCF\x19\x94a\x07\xB2\x94\x93\x92\x89\x92`\x04\x01a\\.V[``a\x08P`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83aA\xE1V[``a\x08P`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83aA\xE1V[`\0\x80Q` a]\xE2\x839\x81Q\x91R\x82\x82`@Qa6\xE4\x92\x91\x90a\\\xDDV[\x80\x82\x14a\x08\x1CW`\0\x80Q` a]C\x839\x81Q\x91R`@Qa@\xEC\x90` \x80\x82R`\"\x90\x82\x01R\x7FError: a == b not satisfied [uin`@\x82\x01Rat]`\xF0\x1B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x08\x13\x19Y\x9D`\xB2\x1B``\x82\x01R` \x81\x01\x84\x90R\x90Q`\0\x80Q` a]\xE2\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1`@\x80Q\x81\x81R`\n\x81\x83\x01Ri\x08\x08\x08\x08\x08\x14\x9AY\xDA\x1D`\xB2\x1B``\x82\x01R` \x81\x01\x83\x90R\x90Q`\0\x80Q` a]\xE2\x839\x81Q\x91R\x91\x81\x90\x03`\x80\x01\x90\xA1a\x08\x1CaB+V[`\0\x80Q` a]C\x839\x81Q\x91R\x81`@Qa/\"\x91\x90aM\xD9V[`\"T`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x93\x84\x1B\x16`,\x82\x01R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01aB\x14\x93\x92\x91\x90a\\\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15aC&W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x92\x82\x01\x92\x90\x92R`\x01``\x82\x01R`\0\x91\x90\x7Fp\xCA\x10\xBB\xD0\xDB\xFD\x90 \xA9\xF4\xB14\x02\xC1l\xB1 p^\r\x1C\n\xEA\xB1\x0F\xA3S\xAEXo\xC4\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaB\xC5\x92\x91` \x01aS=V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaB\xDF\x91aSnV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14aC\x1CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aC!V[``\x91P[PPPP[`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90UV[`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"\x03W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\xBAW`\0\x80\xFD[\x815a,\xF5\x81aC\x95V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xFDWaC\xFDaC\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\xB3WaD\xB3aC\xC5V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xD4WaD\xD4aC\xC5V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aD\xF5W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x12W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aE#W`\0\x80\xFD[\x805aE6aE1\x82aD\xBBV[aD\x8BV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aEKW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aE\xACW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aE\x85V[P\x90\x95\x94PPPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aE\xD0WaE\xD0aC\xC5V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\x03W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aF\0W`\0\x80\xFD[\x815aF\x0EaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aF0W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x805aFH\x81aE\xDAV[\x83R` \x92\x83\x01\x92\x01aF5V[P\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aFsW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aF\x89W`\0\x80\xFD[aF\x95\x85\x82\x86\x01aE\xEFV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xB1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aF\xC2W`\0\x80\xFD[\x805aF\xD0aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aF\xF2W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aG\x14W\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aF\xF9V[\x80\x94PPPPP\x92P\x92\x90PV[`\0\x82`\x1F\x83\x01\x12aG3W`\0\x80\xFD[\x815aGAaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aGcW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x805\x83R` \x92\x83\x01\x92\x01aGhV[`\0\x80`@\x83\x85\x03\x12\x15aG\x93W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xA9W`\0\x80\xFD[aG\xB5\x85\x82\x86\x01aE\xEFV[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xD1W`\0\x80\xFD[aG\xDD\x85\x82\x86\x01aG\"V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aH\"W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aG\xFBV[P\x93\x94\x93PPPPV[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aH\"W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aH@V[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x80\x82\x01Q`\0\x91aH\xAA\x90\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaH\xC4`\xE0\x85\x01\x82aG\xE7V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaH\xDD\x82\x82aH,V[\x95\x94PPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84RaI*\x85\x83QaH^V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aI\x0EV[\x805a\x12\x04\x81aE\xDAV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\x04W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15aIpW`\0\x80\xFD[aIxaC\xDBV[\x90PaI\x83\x82aI?V[\x81RaI\x91` \x83\x01aI?V[` \x82\x01RaI\xA2`@\x83\x01aI?V[`@\x82\x01R``\x82\x81\x015\x90\x82\x01RaI\xBD`\x80\x83\x01aIJV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xDBW`\0\x80\xFD[aI\xE7\x84\x82\x85\x01aE\xEFV[`\xA0\x83\x01RP`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x06W`\0\x80\xFD[aJ\x12\x84\x82\x85\x01aG\"V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aJ0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aJWW`\0\x80\xFD[\x805aJeaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aJ\x87W`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15aJ\xC8W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xAAW`\0\x80\xFD[aJ\xB9\x89` \x83\x89\x01\x01aI^V[\x84RP` \x92\x83\x01\x92\x01aJ\x8CV[P\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84RaK\x17\x85\x83QaG\xE7V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aJ\xFBV[`\0` \x82\x84\x03\x12\x15aK>W`\0\x80\xFD[\x815a,\xF5\x81aE\xDAV[`\0` \x82\x84\x03\x12\x15aK[W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKqW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aK\x82W`\0\x80\xFD[\x805aK\x90aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aK\xB2W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aK\xDDW\x835aK\xCC\x81aC\x95V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aK\xB9V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aK\xF9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x0FW`\0\x80\xFD[aL\x1B\x84\x82\x85\x01aI^V[\x94\x93PPPPV[` \x81R`\0a,\xF5` \x83\x01\x84aG\xE7V[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15aL\xCCW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aL\xA0V[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aL^V[`\0[\x83\x81\x10\x15aL\xFFW\x81\x81\x01Q\x83\x82\x01R` \x01aL\xE7V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaM \x81` \x86\x01` \x86\x01aL\xE4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84RaMx\x85\x83QaM\x08V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aM\\V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aH\"W\x81Qd\xFF\xFF\xFF\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aM\xA1V[` \x81R`\0a,\xF5` \x83\x01\x84aM\x8DV[` \x81R`\0a,\xF5` \x83\x01\x84aM\x08V[`\0` \x82\x84\x03\x12\x15aM\xFEW`\0\x80\xFD[P5\x91\x90PV[`@\x81R`\0aN\x18`@\x83\x01\x85aM\x8DV[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aNBW`\0\x80\xFD[PQ\x91\x90PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x12\x04W`\0\x80\xFD[`\0aNnaE1\x84aD\xBBV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15aN\x82W`\0\x80\xFD[a,\xF5\x83` \x83\x01\x84aL\xE4V[`\0\x82`\x1F\x83\x01\x12aN\xA1W`\0\x80\xFD[a,\xF5\x83\x83Q` \x85\x01aN`V[`\0`@\x82\x84\x03\x12\x15aN\xC2W`\0\x80\xFD[aN\xCAaD\x03V[\x82Q\x81R` \x83\x01Q\x90\x91P`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xEAW`\0\x80\xFD[aN\xF6\x84\x82\x85\x01aN\x90V[` \x83\x01RP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aO\x13W`\0\x80\xFD[\x81QaO!aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aOCW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x80Q\x83R` \x92\x83\x01\x92\x01aOHV[`\0` \x82\x84\x03\x12\x15aOrW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x88W`\0\x80\xFD[\x82\x01``\x81\x85\x03\x12\x15aO\x9AW`\0\x80\xFD[aO\xA2aD%V[aO\xAB\x82aNIV[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xC6W`\0\x80\xFD[aO\xD2\x86\x82\x85\x01aN\xB0V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xF1W`\0\x80\xFD[\x91\x90\x91\x01\x90`@\x82\x86\x03\x12\x15aP\x06W`\0\x80\xFD[aP\x0EaD\x03V[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aP$W`\0\x80\xFD[aP0\x87\x82\x86\x01aO\x02V[\x82RP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aPLW`\0\x80\xFD[aPX\x87\x82\x86\x01aN\x90V[` \x83\x01RP`@\x82\x01R\x94\x93PPPPV[\x80Q\x82R`\0` \x82\x01Q`@` \x85\x01RaL\x1B`@\x85\x01\x82aM\x08V[`\x01`\x01`@\x1B\x03\x84\x16\x81R``` \x82\x01R`\0aP\xAC``\x83\x01\x85aPkV[\x82\x81\x03`@\x84\x01R\x83Q`@\x82RaP\xC7`@\x83\x01\x82aH,V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01RaP\xE0\x82\x82aM\x08V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aQ\x14W`\0\x80\xFD[\x81Qa,\xF5\x81aE\xDAV[`\0` \x82\x84\x03\x12\x15aQ1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a,\xF5W`\0\x80\xFD[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a6\x88W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q``\x87RaQ\x8F``\x88\x01\x82aG\xE7V[\x90P` \x82\x01Q\x87\x82\x03` \x89\x01RaQ\xA8\x82\x82aH,V[`@\x93\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x98\x90\x93\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aQiV[`\0` \x82\x84\x03\x12\x15aQ\xE7W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xFDW`\0\x80\xFD[aL\x1B\x84\x82\x85\x01aO\x02V[`\0` \x82\x84\x03\x12\x15aR\x1BW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR1W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aRBW`\0\x80\xFD[aL\x1B\x84\x82Q` \x84\x01aN`V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x81\x16` \x83\x01R`@\x82\x01\x86\x90R\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xC0`\xA0\x82\x01\x81\x90R`\0\x90aP\xE0\x90\x83\x01\x84aM\x08V[`\x01\x81\x81\x1C\x90\x82\x16\x80aR\xA7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aR\xC7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aR\xF5WaR\xF5aR\xCDV[P`\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01R`\0\x83Q`@``\x84\x01RaS&`\xA0\x84\x01\x82aM\x08V[` \x95\x90\x95\x01Q`\x80\x84\x01RPP`@\x01R\x91\x90PV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90aS`\x81`\x04\x85\x01` \x87\x01aL\xE4V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82QaS\x80\x81\x84` \x87\x01aL\xE4V[\x91\x90\x91\x01\x92\x91PPV[`\0\x83QaS\x9C\x81\x84` \x88\x01aL\xE4V[`\x17`\xF9\x1B\x90\x83\x01\x90\x81R\x83QaS\xBA\x81`\x01\x84\x01` \x88\x01aL\xE4V[\x01`\x01\x01\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x80\x15aS\xD9W`\0\x80\xFD[P`\0aS\xE4aDGV[\x83Q\x81R` \x84\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14aS\xFDW\x82\x83\xFD[` \x82\x01RaT\x0E`@\x85\x01aNIV[`@\x82\x01R``\x84\x01Q\x80`\x07\x0B\x81\x14aT&W\x82\x83\xFD[``\x82\x01RaT7`\x80\x85\x01aNIV[`\x80\x82\x01R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aTVW`\0\x80\xFD[a,\xF5\x82aNIV[`\0`@\x82\x01`@\x83R\x80\x85TaTz\x81\x84\x90\x81R` \x01\x90V[`\0\x88\x81R` \x81 \x94P\x90\x92P[\x81`\x05\x82\x01\x10\x15aT\xF0W\x83Td\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x85R`(\x82\x90\x1C\x81\x16` \x86\x01R`P\x82\x90\x1C\x81\x16`@\x86\x01R`x\x82\x90\x1C\x81\x16``\x86\x01R`\xA0\x82\x81\x1C\x82\x16`\x80\x87\x01R`\xC8\x92\x90\x92\x1C\x16\x90\x84\x01R`\x01\x90\x93\x01\x92`\xC0\x90\x92\x01\x91`\x06\x01aT\x89V[\x92T\x92\x81\x81\x10\x15aU\x0FWd\xFF\xFF\xFF\xFF\xFF\x84\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aU/Wd\xFF\xFF\xFF\xFF\xFF`(\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aUOWd\xFF\xFF\xFF\xFF\xFF`P\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aUoWd\xFF\xFF\xFF\xFF\xFF`x\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aU\x8FWd\xFF\xFF\xFF\xFF\xFF`\xA0\x85\x90\x1C\x16\x83R` \x90\x92\x01\x91`\x01\x01[\x81\x81\x10\x15aU\xACW`\xC8\x84\x90\x1Cd\xFF\xFF\xFF\xFF\xFF\x16\x83R` \x83\x01\x92P[PP`\x01`\x01`@\x1B\x03\x85\x16` \x85\x01R\x91Pa,\xF5\x90PV[`\0` \x82\x84\x03\x12\x15aU\xD8W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xEEW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15aV\0W`\0\x80\xFD[aV\x08aD\x03V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x1EW`\0\x80\xFD[aV*\x86\x82\x85\x01aN\xB0V[\x82RP` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aVFW`\0\x80\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12aV[W`\0\x80\xFD[\x81QaViaE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x87\x83\x11\x15aV\x8BW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aW\x1AW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xAEW`\0\x80\xFD[\x86\x01``\x81\x8B\x03`\x1F\x19\x01\x12\x15aV\xC4W`\0\x80\xFD[aV\xCCaD%V[` \x82\x81\x01Q\x82R`@\x83\x01Q\x90\x82\x01R``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xF6W`\0\x80\xFD[aW\x05\x8C` \x83\x86\x01\x01aN\x90V[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01aV\x90V[P` \x84\x01RP\x90\x95\x94PPPPPV[`@\x81R`\0aW>`@\x83\x01\x85aPkV[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01`\0[\x83\x81\x10\x15aW\xB5W`\x1F\x19\x86\x84\x03\x01\x85R\x81Q\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q\x90P```@\x85\x01RaW\x9E``\x85\x01\x82aM\x08V[` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01aWbV[P\x90\x98\x97PPPPPPPPV[`@\x81R`\x05`@\x82\x01Rd\"\xB997\xB9`\xD9\x1B``\x82\x01R`\x80` \x82\x01R`\0a,\xF5`\x80\x83\x01\x84aM\x08V[``\x81R`\0aX\x05``\x83\x01\x86aH^V[\x82\x81\x03` \x84\x01RaX\x17\x81\x86aG\xE7V[\x91PP\x82\x15\x15`@\x83\x01R\x94\x93PPPPV[`\0\x84QaX<\x81\x84` \x89\x01aL\xE4V[`\x17`\xF9\x1B\x90\x83\x01\x90\x81R\x84QaXZ\x81`\x01\x84\x01` \x89\x01aL\xE4V[`\x1D`\xF9\x1B`\x01\x92\x90\x91\x01\x91\x82\x01R\x83QaX|\x81`\x02\x84\x01` \x88\x01aL\xE4V[\x01`\x02\x01\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aX\x9CW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aX\xB2W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aX\xC3W`\0\x80\xFD[\x80QaX\xD1aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aX\xF3W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15aY\x1EW\x83QaY\r\x81aE\xDAV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aX\xFAV[\x80\x95PPPPP` \x83\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aY>W`\0\x80\xFD[aG\xDD\x85\x82\x86\x01aO\x02V[\x80\x82\x01\x80\x82\x11\x15a\x08PWa\x08PaR\xCDV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x08PWa\x08PaR\xCDV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aY\xA1WaY\xA1aY|V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x08PWa\x08PaR\xCDV[\x81\x81\x03\x81\x81\x11\x15a\x08PWa\x08PaR\xCDV[`\0\x82aY\xDFWaY\xDFaY|V[P\x06\x90V[`\0` \x82\x84\x03\x12\x15aY\xF6W`\0\x80\xFD[\x81Qa,\xF5\x81aC\x95V[`\0\x82`\x1F\x83\x01\x12aZ\x12W`\0\x80\xFD[\x81QaZ aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aZBW`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15aFVW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZeW`\0\x80\xFD[aZt\x88` \x83\x8A\x01\x01aO\x02V[\x84RP` \x92\x83\x01\x92\x01aZGV[`\0` \x82\x84\x03\x12\x15aZ\x95W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xABW`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15aZ\xBDW`\0\x80\xFD[aZ\xC5aDiV[aZ\xCE\x82aNIV[\x81R` \x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15aZ\xE9W`\0\x80\xFD[aZ\xF5\x86\x82\x85\x01aN\xB0V[` \x83\x01RP`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[\x14W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x86\x13a[%W`\0\x80\xFD[\x80Qa[3aE1\x82aE\xB7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a[UW`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a[\x96W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[xW`\0\x80\xFD[a[\x87\x8B` \x83\x89\x01\x01aN\x90V[\x84RP` \x92\x83\x01\x92\x01a[ZV[P`@\x85\x01RPPP``\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a[\xB8W`\0\x80\xFD[a[\xC4\x86\x82\x85\x01aZ\x01V[``\x83\x01RP\x94\x93PPPPV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15a\\\"W`\x1F\x19\x85\x84\x03\x01\x88Ra\\\x0C\x83\x83QaH,V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a[\xF0V[P\x90\x96\x95PPPPPPV[`\x01`\x01`@\x1B\x03\x86\x16\x81R`\xA0` \x82\x01R`\0a\\P`\xA0\x83\x01\x87aPkV[\x82\x81\x03`@\x84\x01Ra\\b\x81\x87aM\x8DV[\x90P\x82\x81\x03``\x84\x01R\x80\x85Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x88\x01`\0[\x83\x81\x10\x15a\\\xBAW`\x1F\x19\x86\x84\x03\x01\x85Ra\\\xA4\x83\x83QaM\x08V[` \x95\x86\x01\x95\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a\\\x88V[PP\x85\x81\x03`\x80\x87\x01Ra\\\xCE\x81\x88a[\xD2V[\x9B\x9APPPPPPPPPPPV[`@\x81R`\0a\\\xF0`@\x83\x01\x85aM\x08V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x84Qa]\x11\x81\x84` \x89\x01aL\xE4V[\x84Q\x90\x83\x01\x90a]%\x81\x83` \x89\x01aL\xE4V[\x84Q\x91\x01\x90a]8\x81\x83` \x88\x01aL\xE4V[\x01\x95\x94PPPPPV\xFEA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP- depositing balance to beacon chain (gwei)- exiting all validators and completing checkpoint- submitting num checkpoint proofs\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8User.queueWithdrawals: length mismatch\xA2dipfsX\"\x12 \xBCO\x9E`A;i\xF3K\x8DV\xF33~h\xC7\xBB\xB91\xFA\x94\xFB\xC1IY\xD7\xC5\xAE\xE9\xE9\x88\xFDdsolcC\0\x08\x1B\x003",
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
    /**Function with signature `depositIntoEigenlayer_M1(address[],uint256[])` and selector `0x654bb5d9`.
```solidity
function depositIntoEigenlayer_M1(address[] memory strategies, uint256[] memory tokenBalances) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositIntoEigenlayer_M1Call {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub tokenBalances: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`depositIntoEigenlayer_M1(address[],uint256[])`](depositIntoEigenlayer_M1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct depositIntoEigenlayer_M1Return {}
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
            impl ::core::convert::From<depositIntoEigenlayer_M1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoEigenlayer_M1Call) -> Self {
                    (value.strategies, value.tokenBalances)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositIntoEigenlayer_M1Call {
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
            impl ::core::convert::From<depositIntoEigenlayer_M1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: depositIntoEigenlayer_M1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for depositIntoEigenlayer_M1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for depositIntoEigenlayer_M1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = depositIntoEigenlayer_M1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "depositIntoEigenlayer_M1(address[],uint256[])";
            const SELECTOR: [u8; 4] = [101u8, 75u8, 181u8, 217u8];
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
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
```solidity
function isValidSignature(bytes32 hash, bytes memory) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub hash: alloy::sol_types::private::FixedBytes<32>,
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<isValidSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value.hash, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { hash: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isValidSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
function queueWithdrawals(address[] memory strategies, uint256[] memory shares) external returns (IDelegationManagerTypes.Withdrawal[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsCall {
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub shares: alloy::sol_types::private::Vec<
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
                    (value.strategies, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategies: tuple.0,
                        shares: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = signedHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    ///Container for all the [`User_M1_AltMethods`](self) function calls.
    pub enum User_M1_AltMethodsCalls {
        IS_TEST(IS_TESTCall),
        NAME(NAMECall),
        completeCheckpoint(completeCheckpointCall),
        completeWithdrawalAsShares(completeWithdrawalAsSharesCall),
        completeWithdrawalAsTokens(completeWithdrawalAsTokensCall),
        completeWithdrawalsAsShares(completeWithdrawalsAsSharesCall),
        completeWithdrawalsAsTokens(completeWithdrawalsAsTokensCall),
        delegateTo(delegateToCall),
        depositIntoEigenlayer(depositIntoEigenlayerCall),
        depositIntoEigenlayer_M1(depositIntoEigenlayer_M1Call),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        exitValidators(exitValidatorsCall),
        failed(failedCall),
        forceUndelegate(forceUndelegateCall),
        getActiveValidators(getActiveValidatorsCall),
        isValidSignature(isValidSignatureCall),
        pod(podCall),
        queueWithdrawals(queueWithdrawalsCall),
        registerAsOperator(registerAsOperatorCall),
        signedHashes(signedHashesCall),
        startCheckpoint(startCheckpointCall),
        startValidators(startValidatorsCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        undelegate(undelegateCall),
        updateBalances(updateBalancesCall),
        verifyStaleBalance(verifyStaleBalanceCall),
        verifyWithdrawalCredentials(verifyWithdrawalCredentialsCall),
    }
    #[automatically_derived]
    impl User_M1_AltMethodsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [7u8, 28u8, 37u8, 183u8],
            [22u8, 38u8, 186u8, 126u8],
            [30u8, 215u8, 131u8, 28u8],
            [32u8, 165u8, 69u8, 217u8],
            [35u8, 228u8, 129u8, 117u8],
            [42u8, 52u8, 173u8, 232u8],
            [52u8, 78u8, 19u8, 131u8],
            [57u8, 28u8, 201u8, 246u8],
            [61u8, 140u8, 8u8, 212u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [64u8, 27u8, 230u8, 94u8],
            [70u8, 165u8, 190u8, 13u8],
            [101u8, 75u8, 181u8, 217u8],
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
    impl alloy_sol_types::SolInterface for User_M1_AltMethodsCalls {
        const NAME: &'static str = "User_M1_AltMethodsCalls";
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
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositIntoEigenlayer(_) => {
                    <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::depositIntoEigenlayer_M1(_) => {
                    <depositIntoEigenlayer_M1Call as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pod(_) => <podCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::queueWithdrawals(_) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::signedHashes(_) => {
                    <signedHashesCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls>] = &[
                {
                    fn verifyStaleBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::verifyStaleBalance)
                    }
                    verifyStaleBalance
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn updateBalances(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <updateBalancesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::updateBalances)
                    }
                    updateBalances
                },
                {
                    fn queueWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::queueWithdrawals)
                    }
                    queueWithdrawals
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::registerAsOperator)
                    }
                    registerAsOperator
                },
                {
                    fn completeWithdrawalsAsTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <completeWithdrawalsAsTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::completeWithdrawalsAsTokens)
                    }
                    completeWithdrawalsAsTokens
                },
                {
                    fn forceUndelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <forceUndelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::forceUndelegate)
                    }
                    forceUndelegate
                },
                {
                    fn exitValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <exitValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::exitValidators)
                    }
                    exitValidators
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn completeWithdrawalAsTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <completeWithdrawalAsTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::completeWithdrawalAsTokens)
                    }
                    completeWithdrawalAsTokens
                },
                {
                    fn completeWithdrawalsAsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <completeWithdrawalsAsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::completeWithdrawalsAsShares)
                    }
                    completeWithdrawalsAsShares
                },
                {
                    fn depositIntoEigenlayer_M1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <depositIntoEigenlayer_M1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::depositIntoEigenlayer_M1)
                    }
                    depositIntoEigenlayer_M1
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn completeWithdrawalAsShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <completeWithdrawalAsSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::completeWithdrawalAsShares)
                    }
                    completeWithdrawalAsShares
                },
                {
                    fn depositIntoEigenlayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <depositIntoEigenlayerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::depositIntoEigenlayer)
                    }
                    depositIntoEigenlayer
                },
                {
                    fn verifyWithdrawalCredentials(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::verifyWithdrawalCredentials)
                    }
                    verifyWithdrawalCredentials
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn startCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <startCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::startCheckpoint)
                    }
                    startCheckpoint
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::undelegate)
                    }
                    undelegate
                },
                {
                    fn getActiveValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <getActiveValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::getActiveValidators)
                    }
                    getActiveValidators
                },
                {
                    fn NAME(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::NAME)
                    }
                    NAME
                },
                {
                    fn pod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <podCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::pod)
                    }
                    pod
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn signedHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <signedHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::signedHashes)
                    }
                    signedHashes
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::failed)
                    }
                    failed
                },
                {
                    fn completeCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <completeCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::completeCheckpoint)
                    }
                    completeCheckpoint
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn startValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <startValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::startValidators)
                    }
                    startValidators
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<User_M1_AltMethodsCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(User_M1_AltMethodsCalls::IS_TEST)
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
                Self::depositIntoEigenlayer_M1(inner) => {
                    <depositIntoEigenlayer_M1Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::signedHashes(inner) => {
                    <signedHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::depositIntoEigenlayer_M1(inner) => {
                    <depositIntoEigenlayer_M1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::signedHashes(inner) => {
                    <signedHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`User_M1_AltMethods`](self) events.
    pub enum User_M1_AltMethodsEvents {
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
    impl User_M1_AltMethodsEvents {
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
    impl alloy_sol_types::SolEventInterface for User_M1_AltMethodsEvents {
        const NAME: &'static str = "User_M1_AltMethodsEvents";
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
    impl alloy_sol_types::private::IntoLogData for User_M1_AltMethodsEvents {
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
    /**Creates a new wrapper around an on-chain [`User_M1_AltMethods`](self) contract instance.

See the [wrapper's documentation](`User_M1_AltMethodsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> User_M1_AltMethodsInstance<T, P, N> {
        User_M1_AltMethodsInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<User_M1_AltMethodsInstance<T, P, N>>,
    > {
        User_M1_AltMethodsInstance::<T, P, N>::deploy(provider, name)
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
        User_M1_AltMethodsInstance::<T, P, N>::deploy_builder(provider, name)
    }
    /**A [`User_M1_AltMethods`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`User_M1_AltMethods`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct User_M1_AltMethodsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for User_M1_AltMethodsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("User_M1_AltMethodsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > User_M1_AltMethodsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`User_M1_AltMethods`](self) contract instance.

See the [wrapper's documentation](`User_M1_AltMethodsInstance`) for more details.*/
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
        ) -> alloy_contract::Result<User_M1_AltMethodsInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> User_M1_AltMethodsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> User_M1_AltMethodsInstance<T, P, N> {
            User_M1_AltMethodsInstance {
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
    > User_M1_AltMethodsInstance<T, P, N> {
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
        ///Creates a new call builder for the [`depositIntoEigenlayer_M1`] function.
        pub fn depositIntoEigenlayer_M1(
            &self,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            tokenBalances: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, depositIntoEigenlayer_M1Call, N> {
            self.call_builder(
                &depositIntoEigenlayer_M1Call {
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
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            hash: alloy::sol_types::private::FixedBytes<32>,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(&isValidSignatureCall { hash, _1 })
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
            shares: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalsCall, N> {
            self.call_builder(
                &queueWithdrawalsCall {
                    strategies,
                    shares,
                },
            )
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
    > User_M1_AltMethodsInstance<T, P, N> {
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
