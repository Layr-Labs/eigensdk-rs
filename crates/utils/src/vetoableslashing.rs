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
///Module containing a contract's types and functions.
/**

```solidity
library ISlasherTypes {
    type SlashingStatus is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ISlasherTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<SlashingStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl SlashingStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SlashingStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SlashingStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISlasherTypes`](self) contract instance.

See the [wrapper's documentation](`ISlasherTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISlasherTypesInstance<T, P, N> {
        ISlasherTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISlasherTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISlasherTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISlasherTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISlasherTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISlasherTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISlasherTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISlasherTypes`](self) contract instance.

See the [wrapper's documentation](`ISlasherTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISlasherTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISlasherTypesInstance<T, P, N> {
            ISlasherTypesInstance {
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
    > ISlasherTypesInstance<T, P, N> {
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
    > ISlasherTypesInstance<T, P, N> {
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

library ISlasherTypes {
    type SlashingStatus is uint8;
}

interface VetoableSlashing {
    event Initialized(uint8 version);
    event OperatorSlashed(uint256 indexed slashingRequestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);
    event SlashingRequestCancelled(uint256 indexed requestId);
    event SlashingRequested(uint256 indexed requestId, address indexed operator, uint32 indexed operatorSetId, uint256[] wadsToSlash, string description);

    function VETO_PERIOD() external view returns (uint256);
    function cancelSlashingRequest(uint256 requestId) external;
    function fulfillSlashingRequest(uint256 requestId) external;
    function initialize(address _serviceManager, address _vetoCommittee, address _slasher) external;
    function nextRequestId() external view returns (uint256);
    function queueSlashingRequest(IAllocationManagerTypes.SlashingParams memory params) external;
    function serviceManager() external view returns (address);
    function slasher() external view returns (address);
    function slashingRequests(uint256) external view returns (IAllocationManagerTypes.SlashingParams memory params, uint256 requestTimestamp, ISlasherTypes.SlashingStatus status);
    function vetoCommittee() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "VETO_PERIOD",
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
    "name": "cancelSlashingRequest",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "fulfillSlashingRequest",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
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
        "name": "_vetoCommittee",
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
    "name": "queueSlashingRequest",
    "inputs": [
      {
        "name": "params",
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
    "type": "function",
    "name": "slashingRequests",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "params",
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
      },
      {
        "name": "requestTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum ISlasherTypes.SlashingStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "vetoCommittee",
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
pub mod VetoableSlashing {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b5061238d8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061009c575f3560e01c80636a8dcf54116100645780636a8dcf5414610130578063b13442711461014e578063c0c53b8b1461016c578063e861dc8514610188578063f48ab27f146101a65761009c565b806304e17704146100a05780630b6d4a20146100bc5780631f05cc8e146100d85780633998fdd3146100f45780636a84a98514610112575b5f5ffd5b6100ba60048036038101906100b59190611504565b6101d8565b005b6100d660048036038101906100d1919061154b565b6101ed565b005b6100f260048036038101906100ed919061154b565b610507565b005b6100fc610607565b6040516101099190611585565b60405180910390f35b61011a61062c565b60405161012791906115ad565b60405180910390f35b610138610632565b6040516101459190611585565b60405180910390f35b610156610657565b6040516101639190611585565b60405180910390f35b610186600480360381019061018191906115c6565b61067c565b005b6101906107fb565b60405161019d91906115ad565b60405180910390f35b6101c060048036038101906101bb919061154b565b610802565b6040516101cf93929190611942565b60405180910390f35b6101e133610a28565b6101ea81610aba565b50565b6101f633610a28565b5f60335f8381526020019081526020015f2090506203f480816004015461021d91906119ab565b42101561025f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161025690611a84565b60405180910390fd5b60016003811115610273576102726118cf565b5b816005015f9054906101000a900460ff166003811115610296576102956118cf565b5b146102d6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016102cd90611b38565b60405180910390fd5b6002816005015f6101000a81548160ff021916908360038111156102fd576102fc6118cf565b5b021790555061050382825f016040518060a00160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020016001820180548060200260200160405190810160405280929190818152602001828054801561040f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116103c6575b505050505081526020016002820180548060200260200160405190810160405280929190818152602001828054801561046557602002820191905f5260205f20905b815481526020019060010190808311610451575b5050505050815260200160038201805461047e90611b83565b80601f01602080910402602001604051908101604052809291908181526020018280546104aa90611b83565b80156104f55780601f106104cc576101008083540402835291602001916104f5565b820191905f5260205f20905b8154815290600101906020018083116104d857829003601f168201915b505050505081525050610c7c565b5050565b61051033610d6e565b6203f48060335f8381526020019081526020015f206004015461053391906119ab565b4210610574576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161056b90611c23565b60405180910390fd5b60016003811115610588576105876118cf565b5b60335f8381526020019081526020015f206005015f9054906101000a900460ff1660038111156105bb576105ba6118cf565b5b146105fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105f290611cd7565b60405180910390fd5b61060481610e00565b50565b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60025481565b60325f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f60019054906101000a900460ff161590508080156106ac575060015f5f9054906101000a900460ff1660ff16105b806106d957506106bb30610e6c565b1580156106d8575060015f5f9054906101000a900460ff1660ff16145b5b610718576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161070f90611d65565b60405180910390fd5b60015f5f6101000a81548160ff021916908360ff16021790555080156107535760015f60016101000a81548160ff0219169083151502179055505b61075d8483610e8e565b8260325f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555080156107f5575f5f60016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860016040516107ec9190611dc8565b60405180910390a15b50505050565b6203f48081565b6033602052805f5260405f205f91509050805f016040518060a00160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020016001820180548060200260200160405190810160405280929190818152602001828054801561091c57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116108d3575b505050505081526020016002820180548060200260200160405190810160405280929190818152602001828054801561097257602002820191905f5260205f20905b81548152602001906001019080831161095e575b5050505050815260200160038201805461098b90611b83565b80601f01602080910402602001604051908101604052809291908181526020018280546109b790611b83565b8015610a025780601f106109d957610100808354040283529160200191610a02565b820191905f5260205f20905b8154815290600101906020018083116109e557829003601f168201915b50505050508152505090806004015490806005015f9054906101000a900460ff16905083565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614610ab7576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610aae90611e51565b60405180910390fd5b50565b5f60025f815480929190610acd90611e6f565b919050559050604051806060016040528083815260200142815260200160016003811115610afe57610afd6118cf565b5b81525060335f8381526020019081526020015f205f820151815f015f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151815f0160146101000a81548163ffffffff021916908363ffffffff1602179055506040820151816001019080519060200190610b9f929190610f60565b506060820151816002019080519060200190610bbc929190610fe7565b506080820151816003019081610bd2919061204d565b505050602082015181600401556040820151816005015f6101000a81548160ff02191690836003811115610c0957610c086118cf565b5b0217905550905050816020015163ffffffff16825f015173ffffffffffffffffffffffffffffffffffffffff16827fadd285945f652e749df3dab9a584be524ec7fbd2a2cad39851278950f9b7322785606001518660800151604051610c709291906121c0565b60405180910390a45050565b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16633d071422826040518263ffffffff1660e01b8152600401610cd691906121f5565b5f604051808303815f87803b158015610ced575f5ffd5b505af1158015610cff573d5f5f3e3d5ffd5b50505050806020015163ffffffff16815f015173ffffffffffffffffffffffffffffffffffffffff16837f8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b84606001518560800151604051610d629291906121c0565b60405180910390a45050565b60325f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614610dfd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610df4906122ab565b60405180910390fd5b50565b600360335f8381526020019081526020015f206005015f6101000a81548160ff02191690836003811115610e3757610e366118cf565b5b0217905550807fb612b6d36795da938211ef5caee2c1d887a24430f76918733089eee28b7f54ac60405160405180910390a250565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f60019054906101000a900460ff16610edc576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ed390612339565b60405180910390fd5b815f60026101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508060015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b828054828255905f5260205f20908101928215610fd6579160200282015b82811115610fd5578251825f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555091602001919060010190610f7e565b5b509050610fe39190611032565b5090565b828054828255905f5260205f20908101928215611021579160200282015b82811115611020578251825591602001919060010190611005565b5b50905061102e9190611032565b5090565b5b80821115611049575f815f905550600101611033565b5090565b5f604051905090565b5f5ffd5b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6110a882611062565b810181811067ffffffffffffffff821117156110c7576110c6611072565b5b80604052505050565b5f6110d961104d565b90506110e5828261109f565b919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611117826110ee565b9050919050565b6111278161110d565b8114611131575f5ffd5b50565b5f813590506111428161111e565b92915050565b5f63ffffffff82169050919050565b61116081611148565b811461116a575f5ffd5b50565b5f8135905061117b81611157565b92915050565b5f5ffd5b5f67ffffffffffffffff82111561119f5761119e611072565b5b602082029050602081019050919050565b5f5ffd5b5f6111be8261110d565b9050919050565b6111ce816111b4565b81146111d8575f5ffd5b50565b5f813590506111e9816111c5565b92915050565b5f6112016111fc84611185565b6110d0565b90508083825260208201905060208402830185811115611224576112236111b0565b5b835b8181101561124d578061123988826111db565b845260208401935050602081019050611226565b5050509392505050565b5f82601f83011261126b5761126a611181565b5b813561127b8482602086016111ef565b91505092915050565b5f67ffffffffffffffff82111561129e5761129d611072565b5b602082029050602081019050919050565b5f819050919050565b6112c1816112af565b81146112cb575f5ffd5b50565b5f813590506112dc816112b8565b92915050565b5f6112f46112ef84611284565b6110d0565b90508083825260208201905060208402830185811115611317576113166111b0565b5b835b81811015611340578061132c88826112ce565b845260208401935050602081019050611319565b5050509392505050565b5f82601f83011261135e5761135d611181565b5b813561136e8482602086016112e2565b91505092915050565b5f5ffd5b5f67ffffffffffffffff82111561139557611394611072565b5b61139e82611062565b9050602081019050919050565b828183375f83830152505050565b5f6113cb6113c68461137b565b6110d0565b9050828152602081018484840111156113e7576113e6611377565b5b6113f28482856113ab565b509392505050565b5f82601f83011261140e5761140d611181565b5b813561141e8482602086016113b9565b91505092915050565b5f60a0828403121561143c5761143b61105e565b5b61144660a06110d0565b90505f61145584828501611134565b5f8301525060206114688482850161116d565b602083015250604082013567ffffffffffffffff81111561148c5761148b6110ea565b5b61149884828501611257565b604083015250606082013567ffffffffffffffff8111156114bc576114bb6110ea565b5b6114c88482850161134a565b606083015250608082013567ffffffffffffffff8111156114ec576114eb6110ea565b5b6114f8848285016113fa565b60808301525092915050565b5f6020828403121561151957611518611056565b5b5f82013567ffffffffffffffff8111156115365761153561105a565b5b61154284828501611427565b91505092915050565b5f602082840312156115605761155f611056565b5b5f61156d848285016112ce565b91505092915050565b61157f8161110d565b82525050565b5f6020820190506115985f830184611576565b92915050565b6115a7816112af565b82525050565b5f6020820190506115c05f83018461159e565b92915050565b5f5f5f606084860312156115dd576115dc611056565b5b5f6115ea86828701611134565b93505060206115fb86828701611134565b925050604061160c86828701611134565b9150509250925092565b61161f8161110d565b82525050565b61162e81611148565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f819050919050565b5f61168061167b611676846110ee565b61165d565b6110ee565b9050919050565b5f61169182611666565b9050919050565b5f6116a282611687565b9050919050565b6116b281611698565b82525050565b5f6116c383836116a9565b60208301905092915050565b5f602082019050919050565b5f6116e582611634565b6116ef818561163e565b93506116fa8361164e565b805f5b8381101561172a57815161171188826116b8565b975061171c836116cf565b9250506001810190506116fd565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b611769816112af565b82525050565b5f61177a8383611760565b60208301905092915050565b5f602082019050919050565b5f61179c82611737565b6117a68185611741565b93506117b183611751565b805f5b838110156117e15781516117c8888261176f565b97506117d383611786565b9250506001810190506117b4565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f611820826117ee565b61182a81856117f8565b935061183a818560208601611808565b61184381611062565b840191505092915050565b5f60a083015f8301516118635f860182611616565b5060208301516118766020860182611625565b506040830151848203604086015261188e82826116db565b915050606083015184820360608601526118a88282611792565b915050608083015184820360808601526118c28282611816565b9150508091505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061190d5761190c6118cf565b5b50565b5f81905061191d826118fc565b919050565b5f61192c82611910565b9050919050565b61193c81611922565b82525050565b5f6060820190508181035f83015261195a818661184e565b9050611969602083018561159e565b6119766040830184611933565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6119b5826112af565b91506119c0836112af565b92508282019050808211156119d8576119d761197e565b5b92915050565b5f82825260208201905092915050565b7f5665746f61626c65536c617368696e672e66756c66696c6c536c617368696e675f8201527f526571756573743a207665746f20706572696f6420686173206e6f742070617360208201527f7365640000000000000000000000000000000000000000000000000000000000604082015250565b5f611a6e6043836119de565b9150611a79826119ee565b606082019050919050565b5f6020820190508181035f830152611a9b81611a62565b9050919050565b7f5665746f61626c65536c617368696e672e66756c66696c6c536c617368696e675f8201527f526571756573743a207265717565737420686173206265656e2063616e63656c60208201527f6c65640000000000000000000000000000000000000000000000000000000000604082015250565b5f611b226043836119de565b9150611b2d82611aa2565b606082019050919050565b5f6020820190508181035f830152611b4f81611b16565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680611b9a57607f821691505b602082108103611bad57611bac611b56565b5b50919050565b7f5665746f61626c65536c617368696e672e63616e63656c536c617368696e67525f8201527f6571756573743a207665746f20706572696f6420686173207061737365640000602082015250565b5f611c0d603e836119de565b9150611c1882611bb3565b604082019050919050565b5f6020820190508181035f830152611c3a81611c01565b9050919050565b7f5665746f61626c65536c617368696e672e63616e63656c536c617368696e67525f8201527f6571756573743a2072657175657374206973206e6f7420696e2052657175657360208201527f7465642073746174757300000000000000000000000000000000000000000000604082015250565b5f611cc1604a836119de565b9150611ccc82611c41565b606082019050919050565b5f6020820190508181035f830152611cee81611cb5565b9050919050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c7265615f8201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b5f611d4f602e836119de565b9150611d5a82611cf5565b604082019050919050565b5f6020820190508181035f830152611d7c81611d43565b9050919050565b5f819050919050565b5f60ff82169050919050565b5f611db2611dad611da884611d83565b61165d565b611d8c565b9050919050565b611dc281611d98565b82525050565b5f602082019050611ddb5f830184611db9565b92915050565b7f536c6173686572426173652e5f636865636b536c61736865723a2063616c6c655f8201527f72206973206e6f742074686520736c6173686572000000000000000000000000602082015250565b5f611e3b6034836119de565b9150611e4682611de1565b604082019050919050565b5f6020820190508181035f830152611e6881611e2f565b9050919050565b5f611e79826112af565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203611eab57611eaa61197e565b5b600182019050919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302611f127fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82611ed7565b611f1c8683611ed7565b95508019841693508086168417925050509392505050565b5f611f4e611f49611f44846112af565b61165d565b6112af565b9050919050565b5f819050919050565b611f6783611f34565b611f7b611f7382611f55565b848454611ee3565b825550505050565b5f5f905090565b611f92611f83565b611f9d818484611f5e565b505050565b5b81811015611fc057611fb55f82611f8a565b600181019050611fa3565b5050565b601f82111561200557611fd681611eb6565b611fdf84611ec8565b81016020851015611fee578190505b612002611ffa85611ec8565b830182611fa2565b50505b505050565b5f82821c905092915050565b5f6120255f198460080261200a565b1980831691505092915050565b5f61203d8383612016565b9150826002028217905092915050565b612056826117ee565b67ffffffffffffffff81111561206f5761206e611072565b5b6120798254611b83565b612084828285611fc4565b5f60209050601f8311600181146120b5575f84156120a3578287015190505b6120ad8582612032565b865550612114565b601f1984166120c386611eb6565b5f5b828110156120ea578489015182556001820191506020850194506020810190506120c5565b868310156121075784890151612103601f891682612016565b8355505b6001600288020188555050505b505050505050565b5f82825260208201905092915050565b5f61213682611737565b612140818561211c565b935061214b83611751565b805f5b8381101561217b578151612162888261176f565b975061216d83611786565b92505060018101905061214e565b5085935050505092915050565b5f612192826117ee565b61219c81856119de565b93506121ac818560208601611808565b6121b581611062565b840191505092915050565b5f6040820190508181035f8301526121d8818561212c565b905081810360208301526121ec8184612188565b90509392505050565b5f6020820190508181035f83015261220d818461184e565b905092915050565b7f5665746f61626c65536c617368696e672e5f636865636b5665746f436f6d6d695f8201527f747465653a2063616c6c6572206973206e6f7420746865207665746f20636f6d60208201527f6d69747465650000000000000000000000000000000000000000000000000000604082015250565b5f6122956046836119de565b91506122a082612215565b606082019050919050565b5f6020820190508181035f8301526122c281612289565b9050919050565b7f496e697469616c697a61626c653a20636f6e7472616374206973206e6f7420695f8201527f6e697469616c697a696e67000000000000000000000000000000000000000000602082015250565b5f612323602b836119de565b915061232e826122c9565b604082019050919050565b5f6020820190508181035f83015261235081612317565b905091905056fea26469706673582212203f712e15f9390dfb264e62fd18f26c4e6a927926e34bddb80c2ef7d65bcc225464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa#\x8D\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9CW_5`\xE0\x1C\x80cj\x8D\xCFT\x11a\0dW\x80cj\x8D\xCFT\x14a\x010W\x80c\xB14Bq\x14a\x01NW\x80c\xC0\xC5;\x8B\x14a\x01lW\x80c\xE8a\xDC\x85\x14a\x01\x88W\x80c\xF4\x8A\xB2\x7F\x14a\x01\xA6Wa\0\x9CV[\x80c\x04\xE1w\x04\x14a\0\xA0W\x80c\x0BmJ \x14a\0\xBCW\x80c\x1F\x05\xCC\x8E\x14a\0\xD8W\x80c9\x98\xFD\xD3\x14a\0\xF4W\x80cj\x84\xA9\x85\x14a\x01\x12W[__\xFD[a\0\xBA`\x04\x806\x03\x81\x01\x90a\0\xB5\x91\x90a\x15\x04V[a\x01\xD8V[\0[a\0\xD6`\x04\x806\x03\x81\x01\x90a\0\xD1\x91\x90a\x15KV[a\x01\xEDV[\0[a\0\xF2`\x04\x806\x03\x81\x01\x90a\0\xED\x91\x90a\x15KV[a\x05\x07V[\0[a\0\xFCa\x06\x07V[`@Qa\x01\t\x91\x90a\x15\x85V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Aa\x06,V[`@Qa\x01'\x91\x90a\x15\xADV[`@Q\x80\x91\x03\x90\xF3[a\x018a\x062V[`@Qa\x01E\x91\x90a\x15\x85V[`@Q\x80\x91\x03\x90\xF3[a\x01Va\x06WV[`@Qa\x01c\x91\x90a\x15\x85V[`@Q\x80\x91\x03\x90\xF3[a\x01\x86`\x04\x806\x03\x81\x01\x90a\x01\x81\x91\x90a\x15\xC6V[a\x06|V[\0[a\x01\x90a\x07\xFBV[`@Qa\x01\x9D\x91\x90a\x15\xADV[`@Q\x80\x91\x03\x90\xF3[a\x01\xC0`\x04\x806\x03\x81\x01\x90a\x01\xBB\x91\x90a\x15KV[a\x08\x02V[`@Qa\x01\xCF\x93\x92\x91\x90a\x19BV[`@Q\x80\x91\x03\x90\xF3[a\x01\xE13a\n(V[a\x01\xEA\x81a\n\xBAV[PV[a\x01\xF63a\n(V[_`3_\x83\x81R` \x01\x90\x81R` \x01_ \x90Pb\x03\xF4\x80\x81`\x04\x01Ta\x02\x1D\x91\x90a\x19\xABV[B\x10\x15a\x02_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02V\x90a\x1A\x84V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x81\x11\x15a\x02sWa\x02ra\x18\xCFV[[\x81`\x05\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x02\x96Wa\x02\x95a\x18\xCFV[[\x14a\x02\xD6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xCD\x90a\x1B8V[`@Q\x80\x91\x03\x90\xFD[`\x02\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x02\xFDWa\x02\xFCa\x18\xCFV[[\x02\x17\x90UPa\x05\x03\x82\x82_\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\x0FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x03\xC6W[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04eW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x04QW[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x04~\x90a\x1B\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xAA\x90a\x1B\x83V[\x80\x15a\x04\xF5W\x80`\x1F\x10a\x04\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPPa\x0C|V[PPV[a\x05\x103a\rnV[b\x03\xF4\x80`3_\x83\x81R` \x01\x90\x81R` \x01_ `\x04\x01Ta\x053\x91\x90a\x19\xABV[B\x10a\x05tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05k\x90a\x1C#V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x81\x11\x15a\x05\x88Wa\x05\x87a\x18\xCFV[[`3_\x83\x81R` \x01\x90\x81R` \x01_ `\x05\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x05\xBBWa\x05\xBAa\x18\xCFV[[\x14a\x05\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xF2\x90a\x1C\xD7V[`@Q\x80\x91\x03\x90\xFD[a\x06\x04\x81a\x0E\0V[PV[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x02T\x81V[`2_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x90P\x80\x80\x15a\x06\xACWP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[\x80a\x06\xD9WPa\x06\xBB0a\x0ElV[\x15\x80\x15a\x06\xD8WP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x14[[a\x07\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x0F\x90a\x1DeV[`@Q\x80\x91\x03\x90\xFD[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x15a\x07SW`\x01_`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[a\x07]\x84\x83a\x0E\x8EV[\x82`2_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x15a\x07\xF5W__`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\x01`@Qa\x07\xEC\x91\x90a\x1D\xC8V[`@Q\x80\x91\x03\x90\xA1[PPPPV[b\x03\xF4\x80\x81V[`3` R\x80_R`@_ _\x91P\x90P\x80_\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x1CW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xD3W[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\trW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t^W[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\t\x8B\x90a\x1B\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xB7\x90a\x1B\x83V[\x80\x15a\n\x02W\x80`\x1F\x10a\t\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x02V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xE5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90\x80`\x04\x01T\x90\x80`\x05\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83V[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xAE\x90a\x1EQV[`@Q\x80\x91\x03\x90\xFD[PV[_`\x02_\x81T\x80\x92\x91\x90a\n\xCD\x90a\x1EoV[\x91\x90PU\x90P`@Q\x80``\x01`@R\x80\x83\x81R` \x01B\x81R` \x01`\x01`\x03\x81\x11\x15a\n\xFEWa\n\xFDa\x18\xCFV[[\x81RP`3_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x0B\x9F\x92\x91\x90a\x0F`V[P``\x82\x01Q\x81`\x02\x01\x90\x80Q\x90` \x01\x90a\x0B\xBC\x92\x91\x90a\x0F\xE7V[P`\x80\x82\x01Q\x81`\x03\x01\x90\x81a\x0B\xD2\x91\x90a MV[PPP` \x82\x01Q\x81`\x04\x01U`@\x82\x01Q\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x0C\tWa\x0C\x08a\x18\xCFV[[\x02\x17\x90UP\x90PP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\xAD\xD2\x85\x94_e.t\x9D\xF3\xDA\xB9\xA5\x84\xBERN\xC7\xFB\xD2\xA2\xCA\xD3\x98Q'\x89P\xF9\xB72'\x85``\x01Q\x86`\x80\x01Q`@Qa\x0Cp\x92\x91\x90a!\xC0V[`@Q\x80\x91\x03\x90\xA4PPV[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\x07\x14\"\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD6\x91\x90a!\xF5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xEDW__\xFD[PZ\xF1\x15\x80\x15a\x0C\xFFW=__>=_\xFD[PPPP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\x8A\x83\xCF\x9A\xFB\t\xA9\x811OO\xB3S\xB9[\x004Q\xDA\x17\n\x99\xF4\x8D\x8D\xB6GK\x06\xD7\x9F;\x84``\x01Q\x85`\x80\x01Q`@Qa\rb\x92\x91\x90a!\xC0V[`@Q\x80\x91\x03\x90\xA4PPV[`2_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r\xFDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xF4\x90a\"\xABV[`@Q\x80\x91\x03\x90\xFD[PV[`\x03`3_\x83\x81R` \x01\x90\x81R` \x01_ `\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x0E7Wa\x0E6a\x18\xCFV[[\x02\x17\x90UP\x80\x7F\xB6\x12\xB6\xD3g\x95\xDA\x93\x82\x11\xEF\\\xAE\xE2\xC1\xD8\x87\xA2D0\xF7i\x18s0\x89\xEE\xE2\x8B\x7FT\xAC`@Q`@Q\x80\x91\x03\x90\xA2PV[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0E\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xD3\x90a#9V[`@Q\x80\x91\x03\x90\xFD[\x81_`\x02a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x0F\xD6W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x0F\xD5W\x82Q\x82_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x91` \x01\x91\x90`\x01\x01\x90a\x0F~V[[P\x90Pa\x0F\xE3\x91\x90a\x102V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x10!W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x10 W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x10\x05V[[P\x90Pa\x10.\x91\x90a\x102V[P\x90V[[\x80\x82\x11\x15a\x10IW_\x81_\x90UP`\x01\x01a\x103V[P\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x10\xA8\x82a\x10bV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\xC7Wa\x10\xC6a\x10rV[[\x80`@RPPPV[_a\x10\xD9a\x10MV[\x90Pa\x10\xE5\x82\x82a\x10\x9FV[\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x11\x17\x82a\x10\xEEV[\x90P\x91\x90PV[a\x11'\x81a\x11\rV[\x81\x14a\x111W__\xFD[PV[_\x815\x90Pa\x11B\x81a\x11\x1EV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11`\x81a\x11HV[\x81\x14a\x11jW__\xFD[PV[_\x815\x90Pa\x11{\x81a\x11WV[\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\x9FWa\x11\x9Ea\x10rV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_a\x11\xBE\x82a\x11\rV[\x90P\x91\x90PV[a\x11\xCE\x81a\x11\xB4V[\x81\x14a\x11\xD8W__\xFD[PV[_\x815\x90Pa\x11\xE9\x81a\x11\xC5V[\x92\x91PPV[_a\x12\x01a\x11\xFC\x84a\x11\x85V[a\x10\xD0V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x12$Wa\x12#a\x11\xB0V[[\x83[\x81\x81\x10\x15a\x12MW\x80a\x129\x88\x82a\x11\xDBV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x12&V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x12kWa\x12ja\x11\x81V[[\x815a\x12{\x84\x82` \x86\x01a\x11\xEFV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12\x9EWa\x12\x9Da\x10rV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x12\xC1\x81a\x12\xAFV[\x81\x14a\x12\xCBW__\xFD[PV[_\x815\x90Pa\x12\xDC\x81a\x12\xB8V[\x92\x91PPV[_a\x12\xF4a\x12\xEF\x84a\x12\x84V[a\x10\xD0V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x13\x17Wa\x13\x16a\x11\xB0V[[\x83[\x81\x81\x10\x15a\x13@W\x80a\x13,\x88\x82a\x12\xCEV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x13\x19V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x13^Wa\x13]a\x11\x81V[[\x815a\x13n\x84\x82` \x86\x01a\x12\xE2V[\x91PP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13\x95Wa\x13\x94a\x10rV[[a\x13\x9E\x82a\x10bV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x13\xCBa\x13\xC6\x84a\x13{V[a\x10\xD0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x13\xE7Wa\x13\xE6a\x13wV[[a\x13\xF2\x84\x82\x85a\x13\xABV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x14\x0EWa\x14\ra\x11\x81V[[\x815a\x14\x1E\x84\x82` \x86\x01a\x13\xB9V[\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a\x14<Wa\x14;a\x10^V[[a\x14F`\xA0a\x10\xD0V[\x90P_a\x14U\x84\x82\x85\x01a\x114V[_\x83\x01RP` a\x14h\x84\x82\x85\x01a\x11mV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x8CWa\x14\x8Ba\x10\xEAV[[a\x14\x98\x84\x82\x85\x01a\x12WV[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xBCWa\x14\xBBa\x10\xEAV[[a\x14\xC8\x84\x82\x85\x01a\x13JV[``\x83\x01RP`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xECWa\x14\xEBa\x10\xEAV[[a\x14\xF8\x84\x82\x85\x01a\x13\xFAV[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x15\x19Wa\x15\x18a\x10VV[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x156Wa\x155a\x10ZV[[a\x15B\x84\x82\x85\x01a\x14'V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x15`Wa\x15_a\x10VV[[_a\x15m\x84\x82\x85\x01a\x12\xCEV[\x91PP\x92\x91PPV[a\x15\x7F\x81a\x11\rV[\x82RPPV[_` \x82\x01\x90Pa\x15\x98_\x83\x01\x84a\x15vV[\x92\x91PPV[a\x15\xA7\x81a\x12\xAFV[\x82RPPV[_` \x82\x01\x90Pa\x15\xC0_\x83\x01\x84a\x15\x9EV[\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x15\xDDWa\x15\xDCa\x10VV[[_a\x15\xEA\x86\x82\x87\x01a\x114V[\x93PP` a\x15\xFB\x86\x82\x87\x01a\x114V[\x92PP`@a\x16\x0C\x86\x82\x87\x01a\x114V[\x91PP\x92P\x92P\x92V[a\x16\x1F\x81a\x11\rV[\x82RPPV[a\x16.\x81a\x11HV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x16\x80a\x16{a\x16v\x84a\x10\xEEV[a\x16]V[a\x10\xEEV[\x90P\x91\x90PV[_a\x16\x91\x82a\x16fV[\x90P\x91\x90PV[_a\x16\xA2\x82a\x16\x87V[\x90P\x91\x90PV[a\x16\xB2\x81a\x16\x98V[\x82RPPV[_a\x16\xC3\x83\x83a\x16\xA9V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x16\xE5\x82a\x164V[a\x16\xEF\x81\x85a\x16>V[\x93Pa\x16\xFA\x83a\x16NV[\x80_[\x83\x81\x10\x15a\x17*W\x81Qa\x17\x11\x88\x82a\x16\xB8V[\x97Pa\x17\x1C\x83a\x16\xCFV[\x92PP`\x01\x81\x01\x90Pa\x16\xFDV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x17i\x81a\x12\xAFV[\x82RPPV[_a\x17z\x83\x83a\x17`V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x17\x9C\x82a\x177V[a\x17\xA6\x81\x85a\x17AV[\x93Pa\x17\xB1\x83a\x17QV[\x80_[\x83\x81\x10\x15a\x17\xE1W\x81Qa\x17\xC8\x88\x82a\x17oV[\x97Pa\x17\xD3\x83a\x17\x86V[\x92PP`\x01\x81\x01\x90Pa\x17\xB4V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x18 \x82a\x17\xEEV[a\x18*\x81\x85a\x17\xF8V[\x93Pa\x18:\x81\x85` \x86\x01a\x18\x08V[a\x18C\x81a\x10bV[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa\x18c_\x86\x01\x82a\x16\x16V[P` \x83\x01Qa\x18v` \x86\x01\x82a\x16%V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x18\x8E\x82\x82a\x16\xDBV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x18\xA8\x82\x82a\x17\x92V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x18\xC2\x82\x82a\x18\x16V[\x91PP\x80\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x19\rWa\x19\x0Ca\x18\xCFV[[PV[_\x81\x90Pa\x19\x1D\x82a\x18\xFCV[\x91\x90PV[_a\x19,\x82a\x19\x10V[\x90P\x91\x90PV[a\x19<\x81a\x19\"V[\x82RPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19Z\x81\x86a\x18NV[\x90Pa\x19i` \x83\x01\x85a\x15\x9EV[a\x19v`@\x83\x01\x84a\x193V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x19\xB5\x82a\x12\xAFV[\x91Pa\x19\xC0\x83a\x12\xAFV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x19\xD8Wa\x19\xD7a\x19~V[[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FVetoableSlashing.fulfillSlashing_\x82\x01R\x7FRequest: veto period has not pas` \x82\x01R\x7Fsed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1An`C\x83a\x19\xDEV[\x91Pa\x1Ay\x82a\x19\xEEV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1A\x9B\x81a\x1AbV[\x90P\x91\x90PV[\x7FVetoableSlashing.fulfillSlashing_\x82\x01R\x7FRequest: request has been cancel` \x82\x01R\x7Fled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1B\"`C\x83a\x19\xDEV[\x91Pa\x1B-\x82a\x1A\xA2V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1BO\x81a\x1B\x16V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x1B\x9AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1B\xADWa\x1B\xACa\x1BVV[[P\x91\x90PV[\x7FVetoableSlashing.cancelSlashingR_\x82\x01R\x7Fequest: veto period has passed\0\0` \x82\x01RPV[_a\x1C\r`>\x83a\x19\xDEV[\x91Pa\x1C\x18\x82a\x1B\xB3V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C:\x81a\x1C\x01V[\x90P\x91\x90PV[\x7FVetoableSlashing.cancelSlashingR_\x82\x01R\x7Fequest: request is not in Reques` \x82\x01R\x7Fted status\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1C\xC1`J\x83a\x19\xDEV[\x91Pa\x1C\xCC\x82a\x1CAV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C\xEE\x81a\x1C\xB5V[\x90P\x91\x90PV[\x7FInitializable: contract is alrea_\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1DO`.\x83a\x19\xDEV[\x91Pa\x1DZ\x82a\x1C\xF5V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1D|\x81a\x1DCV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_a\x1D\xB2a\x1D\xADa\x1D\xA8\x84a\x1D\x83V[a\x16]V[a\x1D\x8CV[\x90P\x91\x90PV[a\x1D\xC2\x81a\x1D\x98V[\x82RPPV[_` \x82\x01\x90Pa\x1D\xDB_\x83\x01\x84a\x1D\xB9V[\x92\x91PPV[\x7FSlasherBase._checkSlasher: calle_\x82\x01R\x7Fr is not the slasher\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1E;`4\x83a\x19\xDEV[\x91Pa\x1EF\x82a\x1D\xE1V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1Eh\x81a\x1E/V[\x90P\x91\x90PV[_a\x1Ey\x82a\x12\xAFV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1E\xABWa\x1E\xAAa\x19~V[[`\x01\x82\x01\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x1F\x12\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x1E\xD7V[a\x1F\x1C\x86\x83a\x1E\xD7V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_a\x1FNa\x1FIa\x1FD\x84a\x12\xAFV[a\x16]V[a\x12\xAFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x1Fg\x83a\x1F4V[a\x1F{a\x1Fs\x82a\x1FUV[\x84\x84Ta\x1E\xE3V[\x82UPPPPV[__\x90P\x90V[a\x1F\x92a\x1F\x83V[a\x1F\x9D\x81\x84\x84a\x1F^V[PPPV[[\x81\x81\x10\x15a\x1F\xC0Wa\x1F\xB5_\x82a\x1F\x8AV[`\x01\x81\x01\x90Pa\x1F\xA3V[PPV[`\x1F\x82\x11\x15a \x05Wa\x1F\xD6\x81a\x1E\xB6V[a\x1F\xDF\x84a\x1E\xC8V[\x81\x01` \x85\x10\x15a\x1F\xEEW\x81\x90P[a \x02a\x1F\xFA\x85a\x1E\xC8V[\x83\x01\x82a\x1F\xA2V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a %_\x19\x84`\x08\x02a \nV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a =\x83\x83a \x16V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a V\x82a\x17\xEEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a oWa na\x10rV[[a y\x82Ta\x1B\x83V[a \x84\x82\x82\x85a\x1F\xC4V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a \xB5W_\x84\x15a \xA3W\x82\x87\x01Q\x90P[a \xAD\x85\x82a 2V[\x86UPa!\x14V[`\x1F\x19\x84\x16a \xC3\x86a\x1E\xB6V[_[\x82\x81\x10\x15a \xEAW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa \xC5V[\x86\x83\x10\x15a!\x07W\x84\x89\x01Qa!\x03`\x1F\x89\x16\x82a \x16V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a!6\x82a\x177V[a!@\x81\x85a!\x1CV[\x93Pa!K\x83a\x17QV[\x80_[\x83\x81\x10\x15a!{W\x81Qa!b\x88\x82a\x17oV[\x97Pa!m\x83a\x17\x86V[\x92PP`\x01\x81\x01\x90Pa!NV[P\x85\x93PPPP\x92\x91PPV[_a!\x92\x82a\x17\xEEV[a!\x9C\x81\x85a\x19\xDEV[\x93Pa!\xAC\x81\x85` \x86\x01a\x18\x08V[a!\xB5\x81a\x10bV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra!\xD8\x81\x85a!,V[\x90P\x81\x81\x03` \x83\x01Ra!\xEC\x81\x84a!\x88V[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\"\r\x81\x84a\x18NV[\x90P\x92\x91PPV[\x7FVetoableSlashing._checkVetoCommi_\x82\x01R\x7Fttee: caller is not the veto com` \x82\x01R\x7Fmittee\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\"\x95`F\x83a\x19\xDEV[\x91Pa\"\xA0\x82a\"\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\"\xC2\x81a\"\x89V[\x90P\x91\x90PV[\x7FInitializable: contract is not i_\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a##`+\x83a\x19\xDEV[\x91Pa#.\x82a\"\xC9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra#P\x81a#\x17V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 ?q.\x15\xF99\r\xFB&Nb\xFD\x18\xF2lNj\x92y&\xE3K\xDD\xB8\x0C.\xF7\xD6[\xCC\"TdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061009c575f3560e01c80636a8dcf54116100645780636a8dcf5414610130578063b13442711461014e578063c0c53b8b1461016c578063e861dc8514610188578063f48ab27f146101a65761009c565b806304e17704146100a05780630b6d4a20146100bc5780631f05cc8e146100d85780633998fdd3146100f45780636a84a98514610112575b5f5ffd5b6100ba60048036038101906100b59190611504565b6101d8565b005b6100d660048036038101906100d1919061154b565b6101ed565b005b6100f260048036038101906100ed919061154b565b610507565b005b6100fc610607565b6040516101099190611585565b60405180910390f35b61011a61062c565b60405161012791906115ad565b60405180910390f35b610138610632565b6040516101459190611585565b60405180910390f35b610156610657565b6040516101639190611585565b60405180910390f35b610186600480360381019061018191906115c6565b61067c565b005b6101906107fb565b60405161019d91906115ad565b60405180910390f35b6101c060048036038101906101bb919061154b565b610802565b6040516101cf93929190611942565b60405180910390f35b6101e133610a28565b6101ea81610aba565b50565b6101f633610a28565b5f60335f8381526020019081526020015f2090506203f480816004015461021d91906119ab565b42101561025f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161025690611a84565b60405180910390fd5b60016003811115610273576102726118cf565b5b816005015f9054906101000a900460ff166003811115610296576102956118cf565b5b146102d6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016102cd90611b38565b60405180910390fd5b6002816005015f6101000a81548160ff021916908360038111156102fd576102fc6118cf565b5b021790555061050382825f016040518060a00160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020016001820180548060200260200160405190810160405280929190818152602001828054801561040f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116103c6575b505050505081526020016002820180548060200260200160405190810160405280929190818152602001828054801561046557602002820191905f5260205f20905b815481526020019060010190808311610451575b5050505050815260200160038201805461047e90611b83565b80601f01602080910402602001604051908101604052809291908181526020018280546104aa90611b83565b80156104f55780601f106104cc576101008083540402835291602001916104f5565b820191905f5260205f20905b8154815290600101906020018083116104d857829003601f168201915b505050505081525050610c7c565b5050565b61051033610d6e565b6203f48060335f8381526020019081526020015f206004015461053391906119ab565b4210610574576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161056b90611c23565b60405180910390fd5b60016003811115610588576105876118cf565b5b60335f8381526020019081526020015f206005015f9054906101000a900460ff1660038111156105bb576105ba6118cf565b5b146105fb576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016105f290611cd7565b60405180910390fd5b61060481610e00565b50565b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60025481565b60325f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b5f5f60019054906101000a900460ff161590508080156106ac575060015f5f9054906101000a900460ff1660ff16105b806106d957506106bb30610e6c565b1580156106d8575060015f5f9054906101000a900460ff1660ff16145b5b610718576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161070f90611d65565b60405180910390fd5b60015f5f6101000a81548160ff021916908360ff16021790555080156107535760015f60016101000a81548160ff0219169083151502179055505b61075d8483610e8e565b8260325f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555080156107f5575f5f60016101000a81548160ff0219169083151502179055507f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249860016040516107ec9190611dc8565b60405180910390a15b50505050565b6203f48081565b6033602052805f5260405f205f91509050805f016040518060a00160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020016001820180548060200260200160405190810160405280929190818152602001828054801561091c57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116108d3575b505050505081526020016002820180548060200260200160405190810160405280929190818152602001828054801561097257602002820191905f5260205f20905b81548152602001906001019080831161095e575b5050505050815260200160038201805461098b90611b83565b80601f01602080910402602001604051908101604052809291908181526020018280546109b790611b83565b8015610a025780601f106109d957610100808354040283529160200191610a02565b820191905f5260205f20905b8154815290600101906020018083116109e557829003601f168201915b50505050508152505090806004015490806005015f9054906101000a900460ff16905083565b60015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614610ab7576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610aae90611e51565b60405180910390fd5b50565b5f60025f815480929190610acd90611e6f565b919050559050604051806060016040528083815260200142815260200160016003811115610afe57610afd6118cf565b5b81525060335f8381526020019081526020015f205f820151815f015f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151815f0160146101000a81548163ffffffff021916908363ffffffff1602179055506040820151816001019080519060200190610b9f929190610f60565b506060820151816002019080519060200190610bbc929190610fe7565b506080820151816003019081610bd2919061204d565b505050602082015181600401556040820151816005015f6101000a81548160ff02191690836003811115610c0957610c086118cf565b5b0217905550905050816020015163ffffffff16825f015173ffffffffffffffffffffffffffffffffffffffff16827fadd285945f652e749df3dab9a584be524ec7fbd2a2cad39851278950f9b7322785606001518660800151604051610c709291906121c0565b60405180910390a45050565b5f60029054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16633d071422826040518263ffffffff1660e01b8152600401610cd691906121f5565b5f604051808303815f87803b158015610ced575f5ffd5b505af1158015610cff573d5f5f3e3d5ffd5b50505050806020015163ffffffff16815f015173ffffffffffffffffffffffffffffffffffffffff16837f8a83cf9afb09a981314f4fb353b95b003451da170a99f48d8db6474b06d79f3b84606001518560800151604051610d629291906121c0565b60405180910390a45050565b60325f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614610dfd576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610df4906122ab565b60405180910390fd5b50565b600360335f8381526020019081526020015f206005015f6101000a81548160ff02191690836003811115610e3757610e366118cf565b5b0217905550807fb612b6d36795da938211ef5caee2c1d887a24430f76918733089eee28b7f54ac60405160405180910390a250565b5f5f8273ffffffffffffffffffffffffffffffffffffffff163b119050919050565b5f60019054906101000a900460ff16610edc576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610ed390612339565b60405180910390fd5b815f60026101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508060015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050565b828054828255905f5260205f20908101928215610fd6579160200282015b82811115610fd5578251825f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555091602001919060010190610f7e565b5b509050610fe39190611032565b5090565b828054828255905f5260205f20908101928215611021579160200282015b82811115611020578251825591602001919060010190611005565b5b50905061102e9190611032565b5090565b5b80821115611049575f815f905550600101611033565b5090565b5f604051905090565b5f5ffd5b5f5ffd5b5f5ffd5b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6110a882611062565b810181811067ffffffffffffffff821117156110c7576110c6611072565b5b80604052505050565b5f6110d961104d565b90506110e5828261109f565b919050565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f611117826110ee565b9050919050565b6111278161110d565b8114611131575f5ffd5b50565b5f813590506111428161111e565b92915050565b5f63ffffffff82169050919050565b61116081611148565b811461116a575f5ffd5b50565b5f8135905061117b81611157565b92915050565b5f5ffd5b5f67ffffffffffffffff82111561119f5761119e611072565b5b602082029050602081019050919050565b5f5ffd5b5f6111be8261110d565b9050919050565b6111ce816111b4565b81146111d8575f5ffd5b50565b5f813590506111e9816111c5565b92915050565b5f6112016111fc84611185565b6110d0565b90508083825260208201905060208402830185811115611224576112236111b0565b5b835b8181101561124d578061123988826111db565b845260208401935050602081019050611226565b5050509392505050565b5f82601f83011261126b5761126a611181565b5b813561127b8482602086016111ef565b91505092915050565b5f67ffffffffffffffff82111561129e5761129d611072565b5b602082029050602081019050919050565b5f819050919050565b6112c1816112af565b81146112cb575f5ffd5b50565b5f813590506112dc816112b8565b92915050565b5f6112f46112ef84611284565b6110d0565b90508083825260208201905060208402830185811115611317576113166111b0565b5b835b81811015611340578061132c88826112ce565b845260208401935050602081019050611319565b5050509392505050565b5f82601f83011261135e5761135d611181565b5b813561136e8482602086016112e2565b91505092915050565b5f5ffd5b5f67ffffffffffffffff82111561139557611394611072565b5b61139e82611062565b9050602081019050919050565b828183375f83830152505050565b5f6113cb6113c68461137b565b6110d0565b9050828152602081018484840111156113e7576113e6611377565b5b6113f28482856113ab565b509392505050565b5f82601f83011261140e5761140d611181565b5b813561141e8482602086016113b9565b91505092915050565b5f60a0828403121561143c5761143b61105e565b5b61144660a06110d0565b90505f61145584828501611134565b5f8301525060206114688482850161116d565b602083015250604082013567ffffffffffffffff81111561148c5761148b6110ea565b5b61149884828501611257565b604083015250606082013567ffffffffffffffff8111156114bc576114bb6110ea565b5b6114c88482850161134a565b606083015250608082013567ffffffffffffffff8111156114ec576114eb6110ea565b5b6114f8848285016113fa565b60808301525092915050565b5f6020828403121561151957611518611056565b5b5f82013567ffffffffffffffff8111156115365761153561105a565b5b61154284828501611427565b91505092915050565b5f602082840312156115605761155f611056565b5b5f61156d848285016112ce565b91505092915050565b61157f8161110d565b82525050565b5f6020820190506115985f830184611576565b92915050565b6115a7816112af565b82525050565b5f6020820190506115c05f83018461159e565b92915050565b5f5f5f606084860312156115dd576115dc611056565b5b5f6115ea86828701611134565b93505060206115fb86828701611134565b925050604061160c86828701611134565b9150509250925092565b61161f8161110d565b82525050565b61162e81611148565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f819050919050565b5f61168061167b611676846110ee565b61165d565b6110ee565b9050919050565b5f61169182611666565b9050919050565b5f6116a282611687565b9050919050565b6116b281611698565b82525050565b5f6116c383836116a9565b60208301905092915050565b5f602082019050919050565b5f6116e582611634565b6116ef818561163e565b93506116fa8361164e565b805f5b8381101561172a57815161171188826116b8565b975061171c836116cf565b9250506001810190506116fd565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b611769816112af565b82525050565b5f61177a8383611760565b60208301905092915050565b5f602082019050919050565b5f61179c82611737565b6117a68185611741565b93506117b183611751565b805f5b838110156117e15781516117c8888261176f565b97506117d383611786565b9250506001810190506117b4565b5085935050505092915050565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f611820826117ee565b61182a81856117f8565b935061183a818560208601611808565b61184381611062565b840191505092915050565b5f60a083015f8301516118635f860182611616565b5060208301516118766020860182611625565b506040830151848203604086015261188e82826116db565b915050606083015184820360608601526118a88282611792565b915050608083015184820360808601526118c28282611816565b9150508091505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6004811061190d5761190c6118cf565b5b50565b5f81905061191d826118fc565b919050565b5f61192c82611910565b9050919050565b61193c81611922565b82525050565b5f6060820190508181035f83015261195a818661184e565b9050611969602083018561159e565b6119766040830184611933565b949350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6119b5826112af565b91506119c0836112af565b92508282019050808211156119d8576119d761197e565b5b92915050565b5f82825260208201905092915050565b7f5665746f61626c65536c617368696e672e66756c66696c6c536c617368696e675f8201527f526571756573743a207665746f20706572696f6420686173206e6f742070617360208201527f7365640000000000000000000000000000000000000000000000000000000000604082015250565b5f611a6e6043836119de565b9150611a79826119ee565b606082019050919050565b5f6020820190508181035f830152611a9b81611a62565b9050919050565b7f5665746f61626c65536c617368696e672e66756c66696c6c536c617368696e675f8201527f526571756573743a207265717565737420686173206265656e2063616e63656c60208201527f6c65640000000000000000000000000000000000000000000000000000000000604082015250565b5f611b226043836119de565b9150611b2d82611aa2565b606082019050919050565b5f6020820190508181035f830152611b4f81611b16565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680611b9a57607f821691505b602082108103611bad57611bac611b56565b5b50919050565b7f5665746f61626c65536c617368696e672e63616e63656c536c617368696e67525f8201527f6571756573743a207665746f20706572696f6420686173207061737365640000602082015250565b5f611c0d603e836119de565b9150611c1882611bb3565b604082019050919050565b5f6020820190508181035f830152611c3a81611c01565b9050919050565b7f5665746f61626c65536c617368696e672e63616e63656c536c617368696e67525f8201527f6571756573743a2072657175657374206973206e6f7420696e2052657175657360208201527f7465642073746174757300000000000000000000000000000000000000000000604082015250565b5f611cc1604a836119de565b9150611ccc82611c41565b606082019050919050565b5f6020820190508181035f830152611cee81611cb5565b9050919050565b7f496e697469616c697a61626c653a20636f6e747261637420697320616c7265615f8201527f647920696e697469616c697a6564000000000000000000000000000000000000602082015250565b5f611d4f602e836119de565b9150611d5a82611cf5565b604082019050919050565b5f6020820190508181035f830152611d7c81611d43565b9050919050565b5f819050919050565b5f60ff82169050919050565b5f611db2611dad611da884611d83565b61165d565b611d8c565b9050919050565b611dc281611d98565b82525050565b5f602082019050611ddb5f830184611db9565b92915050565b7f536c6173686572426173652e5f636865636b536c61736865723a2063616c6c655f8201527f72206973206e6f742074686520736c6173686572000000000000000000000000602082015250565b5f611e3b6034836119de565b9150611e4682611de1565b604082019050919050565b5f6020820190508181035f830152611e6881611e2f565b9050919050565b5f611e79826112af565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203611eab57611eaa61197e565b5b600182019050919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302611f127fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82611ed7565b611f1c8683611ed7565b95508019841693508086168417925050509392505050565b5f611f4e611f49611f44846112af565b61165d565b6112af565b9050919050565b5f819050919050565b611f6783611f34565b611f7b611f7382611f55565b848454611ee3565b825550505050565b5f5f905090565b611f92611f83565b611f9d818484611f5e565b505050565b5b81811015611fc057611fb55f82611f8a565b600181019050611fa3565b5050565b601f82111561200557611fd681611eb6565b611fdf84611ec8565b81016020851015611fee578190505b612002611ffa85611ec8565b830182611fa2565b50505b505050565b5f82821c905092915050565b5f6120255f198460080261200a565b1980831691505092915050565b5f61203d8383612016565b9150826002028217905092915050565b612056826117ee565b67ffffffffffffffff81111561206f5761206e611072565b5b6120798254611b83565b612084828285611fc4565b5f60209050601f8311600181146120b5575f84156120a3578287015190505b6120ad8582612032565b865550612114565b601f1984166120c386611eb6565b5f5b828110156120ea578489015182556001820191506020850194506020810190506120c5565b868310156121075784890151612103601f891682612016565b8355505b6001600288020188555050505b505050505050565b5f82825260208201905092915050565b5f61213682611737565b612140818561211c565b935061214b83611751565b805f5b8381101561217b578151612162888261176f565b975061216d83611786565b92505060018101905061214e565b5085935050505092915050565b5f612192826117ee565b61219c81856119de565b93506121ac818560208601611808565b6121b581611062565b840191505092915050565b5f6040820190508181035f8301526121d8818561212c565b905081810360208301526121ec8184612188565b90509392505050565b5f6020820190508181035f83015261220d818461184e565b905092915050565b7f5665746f61626c65536c617368696e672e5f636865636b5665746f436f6d6d695f8201527f747465653a2063616c6c6572206973206e6f7420746865207665746f20636f6d60208201527f6d69747465650000000000000000000000000000000000000000000000000000604082015250565b5f6122956046836119de565b91506122a082612215565b606082019050919050565b5f6020820190508181035f8301526122c281612289565b9050919050565b7f496e697469616c697a61626c653a20636f6e7472616374206973206e6f7420695f8201527f6e697469616c697a696e67000000000000000000000000000000000000000000602082015250565b5f612323602b836119de565b915061232e826122c9565b604082019050919050565b5f6020820190508181035f83015261235081612317565b905091905056fea26469706673582212203f712e15f9390dfb264e62fd18f26c4e6a927926e34bddb80c2ef7d65bcc225464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9CW_5`\xE0\x1C\x80cj\x8D\xCFT\x11a\0dW\x80cj\x8D\xCFT\x14a\x010W\x80c\xB14Bq\x14a\x01NW\x80c\xC0\xC5;\x8B\x14a\x01lW\x80c\xE8a\xDC\x85\x14a\x01\x88W\x80c\xF4\x8A\xB2\x7F\x14a\x01\xA6Wa\0\x9CV[\x80c\x04\xE1w\x04\x14a\0\xA0W\x80c\x0BmJ \x14a\0\xBCW\x80c\x1F\x05\xCC\x8E\x14a\0\xD8W\x80c9\x98\xFD\xD3\x14a\0\xF4W\x80cj\x84\xA9\x85\x14a\x01\x12W[__\xFD[a\0\xBA`\x04\x806\x03\x81\x01\x90a\0\xB5\x91\x90a\x15\x04V[a\x01\xD8V[\0[a\0\xD6`\x04\x806\x03\x81\x01\x90a\0\xD1\x91\x90a\x15KV[a\x01\xEDV[\0[a\0\xF2`\x04\x806\x03\x81\x01\x90a\0\xED\x91\x90a\x15KV[a\x05\x07V[\0[a\0\xFCa\x06\x07V[`@Qa\x01\t\x91\x90a\x15\x85V[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Aa\x06,V[`@Qa\x01'\x91\x90a\x15\xADV[`@Q\x80\x91\x03\x90\xF3[a\x018a\x062V[`@Qa\x01E\x91\x90a\x15\x85V[`@Q\x80\x91\x03\x90\xF3[a\x01Va\x06WV[`@Qa\x01c\x91\x90a\x15\x85V[`@Q\x80\x91\x03\x90\xF3[a\x01\x86`\x04\x806\x03\x81\x01\x90a\x01\x81\x91\x90a\x15\xC6V[a\x06|V[\0[a\x01\x90a\x07\xFBV[`@Qa\x01\x9D\x91\x90a\x15\xADV[`@Q\x80\x91\x03\x90\xF3[a\x01\xC0`\x04\x806\x03\x81\x01\x90a\x01\xBB\x91\x90a\x15KV[a\x08\x02V[`@Qa\x01\xCF\x93\x92\x91\x90a\x19BV[`@Q\x80\x91\x03\x90\xF3[a\x01\xE13a\n(V[a\x01\xEA\x81a\n\xBAV[PV[a\x01\xF63a\n(V[_`3_\x83\x81R` \x01\x90\x81R` \x01_ \x90Pb\x03\xF4\x80\x81`\x04\x01Ta\x02\x1D\x91\x90a\x19\xABV[B\x10\x15a\x02_W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02V\x90a\x1A\x84V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x81\x11\x15a\x02sWa\x02ra\x18\xCFV[[\x81`\x05\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x02\x96Wa\x02\x95a\x18\xCFV[[\x14a\x02\xD6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x02\xCD\x90a\x1B8V[`@Q\x80\x91\x03\x90\xFD[`\x02\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x02\xFDWa\x02\xFCa\x18\xCFV[[\x02\x17\x90UPa\x05\x03\x82\x82_\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04\x0FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x03\xC6W[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04eW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x04QW[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x04~\x90a\x1B\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xAA\x90a\x1B\x83V[\x80\x15a\x04\xF5W\x80`\x1F\x10a\x04\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPPa\x0C|V[PPV[a\x05\x103a\rnV[b\x03\xF4\x80`3_\x83\x81R` \x01\x90\x81R` \x01_ `\x04\x01Ta\x053\x91\x90a\x19\xABV[B\x10a\x05tW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05k\x90a\x1C#V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x03\x81\x11\x15a\x05\x88Wa\x05\x87a\x18\xCFV[[`3_\x83\x81R` \x01\x90\x81R` \x01_ `\x05\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x03\x81\x11\x15a\x05\xBBWa\x05\xBAa\x18\xCFV[[\x14a\x05\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xF2\x90a\x1C\xD7V[`@Q\x80\x91\x03\x90\xFD[a\x06\x04\x81a\x0E\0V[PV[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x02T\x81V[`2_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[__`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x90P\x80\x80\x15a\x06\xACWP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x10[\x80a\x06\xD9WPa\x06\xBB0a\x0ElV[\x15\x80\x15a\x06\xD8WP`\x01__\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x14[[a\x07\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\x0F\x90a\x1DeV[`@Q\x80\x91\x03\x90\xFD[`\x01__a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x15a\x07SW`\x01_`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[a\x07]\x84\x83a\x0E\x8EV[\x82`2_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x15a\x07\xF5W__`\x01a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98`\x01`@Qa\x07\xEC\x91\x90a\x1D\xC8V[`@Q\x80\x91\x03\x90\xA1[PPPPV[b\x03\xF4\x80\x81V[`3` R\x80_R`@_ _\x91P\x90P\x80_\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\t\x1CW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xD3W[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\trW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t^W[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\t\x8B\x90a\x1B\x83V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xB7\x90a\x1B\x83V[\x80\x15a\n\x02W\x80`\x1F\x10a\t\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x02V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xE5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90\x80`\x04\x01T\x90\x80`\x05\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83V[`\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n\xB7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\xAE\x90a\x1EQV[`@Q\x80\x91\x03\x90\xFD[PV[_`\x02_\x81T\x80\x92\x91\x90a\n\xCD\x90a\x1EoV[\x91\x90PU\x90P`@Q\x80``\x01`@R\x80\x83\x81R` \x01B\x81R` \x01`\x01`\x03\x81\x11\x15a\n\xFEWa\n\xFDa\x18\xCFV[[\x81RP`3_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x0B\x9F\x92\x91\x90a\x0F`V[P``\x82\x01Q\x81`\x02\x01\x90\x80Q\x90` \x01\x90a\x0B\xBC\x92\x91\x90a\x0F\xE7V[P`\x80\x82\x01Q\x81`\x03\x01\x90\x81a\x0B\xD2\x91\x90a MV[PPP` \x82\x01Q\x81`\x04\x01U`@\x82\x01Q\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x0C\tWa\x0C\x08a\x18\xCFV[[\x02\x17\x90UP\x90PP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x82_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\xAD\xD2\x85\x94_e.t\x9D\xF3\xDA\xB9\xA5\x84\xBERN\xC7\xFB\xD2\xA2\xCA\xD3\x98Q'\x89P\xF9\xB72'\x85``\x01Q\x86`\x80\x01Q`@Qa\x0Cp\x92\x91\x90a!\xC0V[`@Q\x80\x91\x03\x90\xA4PPV[_`\x02\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c=\x07\x14\"\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD6\x91\x90a!\xF5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xEDW__\xFD[PZ\xF1\x15\x80\x15a\x0C\xFFW=__>=_\xFD[PPPP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x81_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\x8A\x83\xCF\x9A\xFB\t\xA9\x811OO\xB3S\xB9[\x004Q\xDA\x17\n\x99\xF4\x8D\x8D\xB6GK\x06\xD7\x9F;\x84``\x01Q\x85`\x80\x01Q`@Qa\rb\x92\x91\x90a!\xC0V[`@Q\x80\x91\x03\x90\xA4PPV[`2_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r\xFDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\xF4\x90a\"\xABV[`@Q\x80\x91\x03\x90\xFD[PV[`\x03`3_\x83\x81R` \x01\x90\x81R` \x01_ `\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x03\x81\x11\x15a\x0E7Wa\x0E6a\x18\xCFV[[\x02\x17\x90UP\x80\x7F\xB6\x12\xB6\xD3g\x95\xDA\x93\x82\x11\xEF\\\xAE\xE2\xC1\xD8\x87\xA2D0\xF7i\x18s0\x89\xEE\xE2\x8B\x7FT\xAC`@Q`@Q\x80\x91\x03\x90\xA2PV[__\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x11\x90P\x91\x90PV[_`\x01\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0E\xDCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xD3\x90a#9V[`@Q\x80\x91\x03\x90\xFD[\x81_`\x02a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPV[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x0F\xD6W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x0F\xD5W\x82Q\x82_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x91` \x01\x91\x90`\x01\x01\x90a\x0F~V[[P\x90Pa\x0F\xE3\x91\x90a\x102V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x10!W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x10 W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x10\x05V[[P\x90Pa\x10.\x91\x90a\x102V[P\x90V[[\x80\x82\x11\x15a\x10IW_\x81_\x90UP`\x01\x01a\x103V[P\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[__\xFD[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a\x10\xA8\x82a\x10bV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\xC7Wa\x10\xC6a\x10rV[[\x80`@RPPPV[_a\x10\xD9a\x10MV[\x90Pa\x10\xE5\x82\x82a\x10\x9FV[\x91\x90PV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x11\x17\x82a\x10\xEEV[\x90P\x91\x90PV[a\x11'\x81a\x11\rV[\x81\x14a\x111W__\xFD[PV[_\x815\x90Pa\x11B\x81a\x11\x1EV[\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x11`\x81a\x11HV[\x81\x14a\x11jW__\xFD[PV[_\x815\x90Pa\x11{\x81a\x11WV[\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11\x9FWa\x11\x9Ea\x10rV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_a\x11\xBE\x82a\x11\rV[\x90P\x91\x90PV[a\x11\xCE\x81a\x11\xB4V[\x81\x14a\x11\xD8W__\xFD[PV[_\x815\x90Pa\x11\xE9\x81a\x11\xC5V[\x92\x91PPV[_a\x12\x01a\x11\xFC\x84a\x11\x85V[a\x10\xD0V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x12$Wa\x12#a\x11\xB0V[[\x83[\x81\x81\x10\x15a\x12MW\x80a\x129\x88\x82a\x11\xDBV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x12&V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x12kWa\x12ja\x11\x81V[[\x815a\x12{\x84\x82` \x86\x01a\x11\xEFV[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x12\x9EWa\x12\x9Da\x10rV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x12\xC1\x81a\x12\xAFV[\x81\x14a\x12\xCBW__\xFD[PV[_\x815\x90Pa\x12\xDC\x81a\x12\xB8V[\x92\x91PPV[_a\x12\xF4a\x12\xEF\x84a\x12\x84V[a\x10\xD0V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\x13\x17Wa\x13\x16a\x11\xB0V[[\x83[\x81\x81\x10\x15a\x13@W\x80a\x13,\x88\x82a\x12\xCEV[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa\x13\x19V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x13^Wa\x13]a\x11\x81V[[\x815a\x13n\x84\x82` \x86\x01a\x12\xE2V[\x91PP\x92\x91PPV[__\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x13\x95Wa\x13\x94a\x10rV[[a\x13\x9E\x82a\x10bV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a\x13\xCBa\x13\xC6\x84a\x13{V[a\x10\xD0V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x13\xE7Wa\x13\xE6a\x13wV[[a\x13\xF2\x84\x82\x85a\x13\xABV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x14\x0EWa\x14\ra\x11\x81V[[\x815a\x14\x1E\x84\x82` \x86\x01a\x13\xB9V[\x91PP\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15a\x14<Wa\x14;a\x10^V[[a\x14F`\xA0a\x10\xD0V[\x90P_a\x14U\x84\x82\x85\x01a\x114V[_\x83\x01RP` a\x14h\x84\x82\x85\x01a\x11mV[` \x83\x01RP`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x8CWa\x14\x8Ba\x10\xEAV[[a\x14\x98\x84\x82\x85\x01a\x12WV[`@\x83\x01RP``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xBCWa\x14\xBBa\x10\xEAV[[a\x14\xC8\x84\x82\x85\x01a\x13JV[``\x83\x01RP`\x80\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xECWa\x14\xEBa\x10\xEAV[[a\x14\xF8\x84\x82\x85\x01a\x13\xFAV[`\x80\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x15\x19Wa\x15\x18a\x10VV[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x156Wa\x155a\x10ZV[[a\x15B\x84\x82\x85\x01a\x14'V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x15`Wa\x15_a\x10VV[[_a\x15m\x84\x82\x85\x01a\x12\xCEV[\x91PP\x92\x91PPV[a\x15\x7F\x81a\x11\rV[\x82RPPV[_` \x82\x01\x90Pa\x15\x98_\x83\x01\x84a\x15vV[\x92\x91PPV[a\x15\xA7\x81a\x12\xAFV[\x82RPPV[_` \x82\x01\x90Pa\x15\xC0_\x83\x01\x84a\x15\x9EV[\x92\x91PPV[___``\x84\x86\x03\x12\x15a\x15\xDDWa\x15\xDCa\x10VV[[_a\x15\xEA\x86\x82\x87\x01a\x114V[\x93PP` a\x15\xFB\x86\x82\x87\x01a\x114V[\x92PP`@a\x16\x0C\x86\x82\x87\x01a\x114V[\x91PP\x92P\x92P\x92V[a\x16\x1F\x81a\x11\rV[\x82RPPV[a\x16.\x81a\x11HV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a\x16\x80a\x16{a\x16v\x84a\x10\xEEV[a\x16]V[a\x10\xEEV[\x90P\x91\x90PV[_a\x16\x91\x82a\x16fV[\x90P\x91\x90PV[_a\x16\xA2\x82a\x16\x87V[\x90P\x91\x90PV[a\x16\xB2\x81a\x16\x98V[\x82RPPV[_a\x16\xC3\x83\x83a\x16\xA9V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x16\xE5\x82a\x164V[a\x16\xEF\x81\x85a\x16>V[\x93Pa\x16\xFA\x83a\x16NV[\x80_[\x83\x81\x10\x15a\x17*W\x81Qa\x17\x11\x88\x82a\x16\xB8V[\x97Pa\x17\x1C\x83a\x16\xCFV[\x92PP`\x01\x81\x01\x90Pa\x16\xFDV[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x17i\x81a\x12\xAFV[\x82RPPV[_a\x17z\x83\x83a\x17`V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\x17\x9C\x82a\x177V[a\x17\xA6\x81\x85a\x17AV[\x93Pa\x17\xB1\x83a\x17QV[\x80_[\x83\x81\x10\x15a\x17\xE1W\x81Qa\x17\xC8\x88\x82a\x17oV[\x97Pa\x17\xD3\x83a\x17\x86V[\x92PP`\x01\x81\x01\x90Pa\x17\xB4V[P\x85\x93PPPP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_a\x18 \x82a\x17\xEEV[a\x18*\x81\x85a\x17\xF8V[\x93Pa\x18:\x81\x85` \x86\x01a\x18\x08V[a\x18C\x81a\x10bV[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa\x18c_\x86\x01\x82a\x16\x16V[P` \x83\x01Qa\x18v` \x86\x01\x82a\x16%V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x18\x8E\x82\x82a\x16\xDBV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x18\xA8\x82\x82a\x17\x92V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra\x18\xC2\x82\x82a\x18\x16V[\x91PP\x80\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x04\x81\x10a\x19\rWa\x19\x0Ca\x18\xCFV[[PV[_\x81\x90Pa\x19\x1D\x82a\x18\xFCV[\x91\x90PV[_a\x19,\x82a\x19\x10V[\x90P\x91\x90PV[a\x19<\x81a\x19\"V[\x82RPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x19Z\x81\x86a\x18NV[\x90Pa\x19i` \x83\x01\x85a\x15\x9EV[a\x19v`@\x83\x01\x84a\x193V[\x94\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a\x19\xB5\x82a\x12\xAFV[\x91Pa\x19\xC0\x83a\x12\xAFV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a\x19\xD8Wa\x19\xD7a\x19~V[[\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FVetoableSlashing.fulfillSlashing_\x82\x01R\x7FRequest: veto period has not pas` \x82\x01R\x7Fsed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1An`C\x83a\x19\xDEV[\x91Pa\x1Ay\x82a\x19\xEEV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1A\x9B\x81a\x1AbV[\x90P\x91\x90PV[\x7FVetoableSlashing.fulfillSlashing_\x82\x01R\x7FRequest: request has been cancel` \x82\x01R\x7Fled\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1B\"`C\x83a\x19\xDEV[\x91Pa\x1B-\x82a\x1A\xA2V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1BO\x81a\x1B\x16V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x1B\x9AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1B\xADWa\x1B\xACa\x1BVV[[P\x91\x90PV[\x7FVetoableSlashing.cancelSlashingR_\x82\x01R\x7Fequest: veto period has passed\0\0` \x82\x01RPV[_a\x1C\r`>\x83a\x19\xDEV[\x91Pa\x1C\x18\x82a\x1B\xB3V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C:\x81a\x1C\x01V[\x90P\x91\x90PV[\x7FVetoableSlashing.cancelSlashingR_\x82\x01R\x7Fequest: request is not in Reques` \x82\x01R\x7Fted status\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\x1C\xC1`J\x83a\x19\xDEV[\x91Pa\x1C\xCC\x82a\x1CAV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1C\xEE\x81a\x1C\xB5V[\x90P\x91\x90PV[\x7FInitializable: contract is alrea_\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1DO`.\x83a\x19\xDEV[\x91Pa\x1DZ\x82a\x1C\xF5V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1D|\x81a\x1DCV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_`\xFF\x82\x16\x90P\x91\x90PV[_a\x1D\xB2a\x1D\xADa\x1D\xA8\x84a\x1D\x83V[a\x16]V[a\x1D\x8CV[\x90P\x91\x90PV[a\x1D\xC2\x81a\x1D\x98V[\x82RPPV[_` \x82\x01\x90Pa\x1D\xDB_\x83\x01\x84a\x1D\xB9V[\x92\x91PPV[\x7FSlasherBase._checkSlasher: calle_\x82\x01R\x7Fr is not the slasher\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a\x1E;`4\x83a\x19\xDEV[\x91Pa\x1EF\x82a\x1D\xE1V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\x1Eh\x81a\x1E/V[\x90P\x91\x90PV[_a\x1Ey\x82a\x12\xAFV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x1E\xABWa\x1E\xAAa\x19~V[[`\x01\x82\x01\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a\x1F\x12\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x1E\xD7V[a\x1F\x1C\x86\x83a\x1E\xD7V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_a\x1FNa\x1FIa\x1FD\x84a\x12\xAFV[a\x16]V[a\x12\xAFV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a\x1Fg\x83a\x1F4V[a\x1F{a\x1Fs\x82a\x1FUV[\x84\x84Ta\x1E\xE3V[\x82UPPPPV[__\x90P\x90V[a\x1F\x92a\x1F\x83V[a\x1F\x9D\x81\x84\x84a\x1F^V[PPPV[[\x81\x81\x10\x15a\x1F\xC0Wa\x1F\xB5_\x82a\x1F\x8AV[`\x01\x81\x01\x90Pa\x1F\xA3V[PPV[`\x1F\x82\x11\x15a \x05Wa\x1F\xD6\x81a\x1E\xB6V[a\x1F\xDF\x84a\x1E\xC8V[\x81\x01` \x85\x10\x15a\x1F\xEEW\x81\x90P[a \x02a\x1F\xFA\x85a\x1E\xC8V[\x83\x01\x82a\x1F\xA2V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a %_\x19\x84`\x08\x02a \nV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a =\x83\x83a \x16V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a V\x82a\x17\xEEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a oWa na\x10rV[[a y\x82Ta\x1B\x83V[a \x84\x82\x82\x85a\x1F\xC4V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a \xB5W_\x84\x15a \xA3W\x82\x87\x01Q\x90P[a \xAD\x85\x82a 2V[\x86UPa!\x14V[`\x1F\x19\x84\x16a \xC3\x86a\x1E\xB6V[_[\x82\x81\x10\x15a \xEAW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa \xC5V[\x86\x83\x10\x15a!\x07W\x84\x89\x01Qa!\x03`\x1F\x89\x16\x82a \x16V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a!6\x82a\x177V[a!@\x81\x85a!\x1CV[\x93Pa!K\x83a\x17QV[\x80_[\x83\x81\x10\x15a!{W\x81Qa!b\x88\x82a\x17oV[\x97Pa!m\x83a\x17\x86V[\x92PP`\x01\x81\x01\x90Pa!NV[P\x85\x93PPPP\x92\x91PPV[_a!\x92\x82a\x17\xEEV[a!\x9C\x81\x85a\x19\xDEV[\x93Pa!\xAC\x81\x85` \x86\x01a\x18\x08V[a!\xB5\x81a\x10bV[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra!\xD8\x81\x85a!,V[\x90P\x81\x81\x03` \x83\x01Ra!\xEC\x81\x84a!\x88V[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\"\r\x81\x84a\x18NV[\x90P\x92\x91PPV[\x7FVetoableSlashing._checkVetoCommi_\x82\x01R\x7Fttee: caller is not the veto com` \x82\x01R\x7Fmittee\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a\"\x95`F\x83a\x19\xDEV[\x91Pa\"\xA0\x82a\"\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\"\xC2\x81a\"\x89V[\x90P\x91\x90PV[\x7FInitializable: contract is not i_\x82\x01R\x7Fnitializing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_a##`+\x83a\x19\xDEV[\x91Pa#.\x82a\"\xC9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra#P\x81a#\x17V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 ?q.\x15\xF99\r\xFB&Nb\xFD\x18\xF2lNj\x92y&\xE3K\xDD\xB8\x0C.\xF7\xD6[\xCC\"TdsolcC\0\x08\x1B\x003",
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
    /**Function with signature `VETO_PERIOD()` and selector `0xe861dc85`.
```solidity
function VETO_PERIOD() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VETO_PERIODCall {}
    ///Container type for the return parameters of the [`VETO_PERIOD()`](VETO_PERIODCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VETO_PERIODReturn {
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
            impl ::core::convert::From<VETO_PERIODCall> for UnderlyingRustTuple<'_> {
                fn from(value: VETO_PERIODCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VETO_PERIODCall {
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
            impl ::core::convert::From<VETO_PERIODReturn> for UnderlyingRustTuple<'_> {
                fn from(value: VETO_PERIODReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VETO_PERIODReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for VETO_PERIODCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = VETO_PERIODReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VETO_PERIOD()";
            const SELECTOR: [u8; 4] = [232u8, 97u8, 220u8, 133u8];
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
    /**Function with signature `cancelSlashingRequest(uint256)` and selector `0x1f05cc8e`.
```solidity
function cancelSlashingRequest(uint256 requestId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSlashingRequestCall {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`cancelSlashingRequest(uint256)`](cancelSlashingRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSlashingRequestReturn {}
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
            impl ::core::convert::From<cancelSlashingRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelSlashingRequestCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelSlashingRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            impl ::core::convert::From<cancelSlashingRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cancelSlashingRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cancelSlashingRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelSlashingRequestCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelSlashingRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelSlashingRequest(uint256)";
            const SELECTOR: [u8; 4] = [31u8, 5u8, 204u8, 142u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
    /**Function with signature `fulfillSlashingRequest(uint256)` and selector `0x0b6d4a20`.
```solidity
function fulfillSlashingRequest(uint256 requestId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct fulfillSlashingRequestCall {
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`fulfillSlashingRequest(uint256)`](fulfillSlashingRequestCall) function.
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
            impl ::core::convert::From<fulfillSlashingRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: fulfillSlashingRequestCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for fulfillSlashingRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = fulfillSlashingRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "fulfillSlashingRequest(uint256)";
            const SELECTOR: [u8; 4] = [11u8, 109u8, 74u8, 32u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
    /**Function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`.
```solidity
function initialize(address _serviceManager, address _vetoCommittee, address _slasher) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _vetoCommittee: alloy::sol_types::private::Address,
        pub _slasher: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value._serviceManager, value._vetoCommittee, value._slasher)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        _vetoCommittee: tuple.1,
                        _slasher: tuple.2,
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
            const SIGNATURE: &'static str = "initialize(address,address,address)";
            const SELECTOR: [u8; 4] = [192u8, 197u8, 59u8, 139u8];
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
                        &self._vetoCommittee,
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
    /**Function with signature `queueSlashingRequest((address,uint32,address[],uint256[],string))` and selector `0x04e17704`.
```solidity
function queueSlashingRequest(IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueSlashingRequestCall {
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`queueSlashingRequest((address,uint32,address[],uint256[],string))`](queueSlashingRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueSlashingRequestReturn {}
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
            impl ::core::convert::From<queueSlashingRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueSlashingRequestCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueSlashingRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<queueSlashingRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: queueSlashingRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for queueSlashingRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueSlashingRequestCall {
            type Parameters<'a> = (IAllocationManagerTypes::SlashingParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueSlashingRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueSlashingRequest((address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [4u8, 225u8, 119u8, 4u8];
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
                        &self.params,
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
    /**Function with signature `slashingRequests(uint256)` and selector `0xf48ab27f`.
```solidity
function slashingRequests(uint256) external view returns (IAllocationManagerTypes.SlashingParams memory params, uint256 requestTimestamp, ISlasherTypes.SlashingStatus status);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRequestsCall {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`slashingRequests(uint256)`](slashingRequestsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashingRequestsReturn {
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        pub requestTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub status: <ISlasherTypes::SlashingStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<slashingRequestsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRequestsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRequestsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IAllocationManagerTypes::SlashingParams,
                alloy::sol_types::sol_data::Uint<256>,
                ISlasherTypes::SlashingStatus,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                <ISlasherTypes::SlashingStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<slashingRequestsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashingRequestsReturn) -> Self {
                    (value.params, value.requestTimestamp, value.status)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashingRequestsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        params: tuple.0,
                        requestTimestamp: tuple.1,
                        status: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashingRequestsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashingRequestsReturn;
            type ReturnTuple<'a> = (
                IAllocationManagerTypes::SlashingParams,
                alloy::sol_types::sol_data::Uint<256>,
                ISlasherTypes::SlashingStatus,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashingRequests(uint256)";
            const SELECTOR: [u8; 4] = [244u8, 138u8, 178u8, 127u8];
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
                        256,
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
    /**Function with signature `vetoCommittee()` and selector `0x6a8dcf54`.
```solidity
function vetoCommittee() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct vetoCommitteeCall {}
    ///Container type for the return parameters of the [`vetoCommittee()`](vetoCommitteeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct vetoCommitteeReturn {
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
            impl ::core::convert::From<vetoCommitteeCall> for UnderlyingRustTuple<'_> {
                fn from(value: vetoCommitteeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for vetoCommitteeCall {
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
            impl ::core::convert::From<vetoCommitteeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: vetoCommitteeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for vetoCommitteeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for vetoCommitteeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = vetoCommitteeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "vetoCommittee()";
            const SELECTOR: [u8; 4] = [106u8, 141u8, 207u8, 84u8];
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
    ///Container for all the [`VetoableSlashing`](self) function calls.
    pub enum VetoableSlashingCalls {
        VETO_PERIOD(VETO_PERIODCall),
        cancelSlashingRequest(cancelSlashingRequestCall),
        fulfillSlashingRequest(fulfillSlashingRequestCall),
        initialize(initializeCall),
        nextRequestId(nextRequestIdCall),
        queueSlashingRequest(queueSlashingRequestCall),
        serviceManager(serviceManagerCall),
        slasher(slasherCall),
        slashingRequests(slashingRequestsCall),
        vetoCommittee(vetoCommitteeCall),
    }
    #[automatically_derived]
    impl VetoableSlashingCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 225u8, 119u8, 4u8],
            [11u8, 109u8, 74u8, 32u8],
            [31u8, 5u8, 204u8, 142u8],
            [57u8, 152u8, 253u8, 211u8],
            [106u8, 132u8, 169u8, 133u8],
            [106u8, 141u8, 207u8, 84u8],
            [177u8, 52u8, 66u8, 113u8],
            [192u8, 197u8, 59u8, 139u8],
            [232u8, 97u8, 220u8, 133u8],
            [244u8, 138u8, 178u8, 127u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for VetoableSlashingCalls {
        const NAME: &'static str = "VetoableSlashingCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 10usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::VETO_PERIOD(_) => {
                    <VETO_PERIODCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelSlashingRequest(_) => {
                    <cancelSlashingRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::fulfillSlashingRequest(_) => {
                    <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextRequestId(_) => {
                    <nextRequestIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queueSlashingRequest(_) => {
                    <queueSlashingRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::slashingRequests(_) => {
                    <slashingRequestsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::vetoCommittee(_) => {
                    <vetoCommitteeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<VetoableSlashingCalls>] = &[
                {
                    fn queueSlashingRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <queueSlashingRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::queueSlashingRequest)
                    }
                    queueSlashingRequest
                },
                {
                    fn fulfillSlashingRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <fulfillSlashingRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::fulfillSlashingRequest)
                    }
                    fulfillSlashingRequest
                },
                {
                    fn cancelSlashingRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <cancelSlashingRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::cancelSlashingRequest)
                    }
                    cancelSlashingRequest
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn nextRequestId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <nextRequestIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::nextRequestId)
                    }
                    nextRequestId
                },
                {
                    fn vetoCommittee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <vetoCommitteeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::vetoCommittee)
                    }
                    vetoCommittee
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::slasher)
                    }
                    slasher
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::initialize)
                    }
                    initialize
                },
                {
                    fn VETO_PERIOD(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <VETO_PERIODCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::VETO_PERIOD)
                    }
                    VETO_PERIOD
                },
                {
                    fn slashingRequests(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<VetoableSlashingCalls> {
                        <slashingRequestsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(VetoableSlashingCalls::slashingRequests)
                    }
                    slashingRequests
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
                Self::VETO_PERIOD(inner) => {
                    <VETO_PERIODCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelSlashingRequest(inner) => {
                    <cancelSlashingRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::queueSlashingRequest(inner) => {
                    <queueSlashingRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::slashingRequests(inner) => {
                    <slashingRequestsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::vetoCommittee(inner) => {
                    <vetoCommitteeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::VETO_PERIOD(inner) => {
                    <VETO_PERIODCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelSlashingRequest(inner) => {
                    <cancelSlashingRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::queueSlashingRequest(inner) => {
                    <queueSlashingRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::slashingRequests(inner) => {
                    <slashingRequestsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::vetoCommittee(inner) => {
                    <vetoCommitteeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`VetoableSlashing`](self) events.
    pub enum VetoableSlashingEvents {
        Initialized(Initialized),
        OperatorSlashed(OperatorSlashed),
        SlashingRequestCancelled(SlashingRequestCancelled),
        SlashingRequested(SlashingRequested),
    }
    #[automatically_derived]
    impl VetoableSlashingEvents {
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
    impl alloy_sol_types::SolEventInterface for VetoableSlashingEvents {
        const NAME: &'static str = "VetoableSlashingEvents";
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
    impl alloy_sol_types::private::IntoLogData for VetoableSlashingEvents {
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
    /**Creates a new wrapper around an on-chain [`VetoableSlashing`](self) contract instance.

See the [wrapper's documentation](`VetoableSlashingInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> VetoableSlashingInstance<T, P, N> {
        VetoableSlashingInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<VetoableSlashingInstance<T, P, N>>,
    > {
        VetoableSlashingInstance::<T, P, N>::deploy(provider)
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
        VetoableSlashingInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`VetoableSlashing`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`VetoableSlashing`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct VetoableSlashingInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for VetoableSlashingInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("VetoableSlashingInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > VetoableSlashingInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`VetoableSlashing`](self) contract instance.

See the [wrapper's documentation](`VetoableSlashingInstance`) for more details.*/
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
        ) -> alloy_contract::Result<VetoableSlashingInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> VetoableSlashingInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> VetoableSlashingInstance<T, P, N> {
            VetoableSlashingInstance {
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
    > VetoableSlashingInstance<T, P, N> {
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
        ///Creates a new call builder for the [`VETO_PERIOD`] function.
        pub fn VETO_PERIOD(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, VETO_PERIODCall, N> {
            self.call_builder(&VETO_PERIODCall {})
        }
        ///Creates a new call builder for the [`cancelSlashingRequest`] function.
        pub fn cancelSlashingRequest(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelSlashingRequestCall, N> {
            self.call_builder(
                &cancelSlashingRequestCall {
                    requestId,
                },
            )
        }
        ///Creates a new call builder for the [`fulfillSlashingRequest`] function.
        pub fn fulfillSlashingRequest(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, fulfillSlashingRequestCall, N> {
            self.call_builder(
                &fulfillSlashingRequestCall {
                    requestId,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            _vetoCommittee: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _serviceManager,
                    _vetoCommittee,
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
        ///Creates a new call builder for the [`queueSlashingRequest`] function.
        pub fn queueSlashingRequest(
            &self,
            params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueSlashingRequestCall, N> {
            self.call_builder(&queueSlashingRequestCall { params })
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
        ///Creates a new call builder for the [`slashingRequests`] function.
        pub fn slashingRequests(
            &self,
            _0: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashingRequestsCall, N> {
            self.call_builder(&slashingRequestsCall { _0 })
        }
        ///Creates a new call builder for the [`vetoCommittee`] function.
        pub fn vetoCommittee(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, vetoCommitteeCall, N> {
            self.call_builder(&vetoCommitteeCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > VetoableSlashingInstance<T, P, N> {
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
