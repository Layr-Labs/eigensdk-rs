///Module containing a contract's types and functions.
/**

```solidity
library IStakeRegistryTypes {
    type StakeType is uint8;
    struct StakeUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint96 stake; }
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IStakeRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<StakeType> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl StakeType {
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
        impl alloy_sol_types::SolType for StakeType {
            type RustType = u8;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeType {
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct StakeUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint96 stake; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeUpdate {
        pub updateBlockNumber: u32,
        pub nextUpdateBlockNumber: u32,
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
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
        impl ::core::convert::From<StakeUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: StakeUpdate) -> Self {
                (
                    value.updateBlockNumber,
                    value.nextUpdateBlockNumber,
                    value.stake,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StakeUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    updateBlockNumber: tuple.0,
                    nextUpdateBlockNumber: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StakeUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StakeUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.updateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.stake,
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
        impl alloy_sol_types::SolType for StakeUpdate {
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
        impl alloy_sol_types::SolStruct for StakeUpdate {
            const NAME: &'static str = "StakeUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StakeUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint96 stake)",
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
                            &self.updateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nextUpdateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StakeUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.updateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nextUpdateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
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
                    &rust.updateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nextUpdateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

    See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeRegistryTypesInstance<T, P, N> {
        IStakeRegistryTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IStakeRegistryTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IStakeRegistryTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeRegistryTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IStakeRegistryTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeRegistryTypesInstance")
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
        > IStakeRegistryTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IStakeRegistryTypes`](self) contract instance.

        See the [wrapper's documentation](`IStakeRegistryTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IStakeRegistryTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeRegistryTypesInstance<T, P, N> {
            IStakeRegistryTypesInstance {
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
        > IStakeRegistryTypesInstance<T, P, N>
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
        > IStakeRegistryTypesInstance<T, P, N>
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
library IStakeRegistryTypes {
    type StakeType is uint8;
    struct StakeUpdate {
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
        uint96 stake;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface StakeRegistry {
    error BelowMinimumStakeRequirement();
    error EmptyStakeHistory();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InputDuplicateStrategy();
    error InputMultiplierZero();
    error InvalidBlockNumber();
    error OnlySlashingRegistryCoordinator();
    error OnlySlashingRegistryCoordinatorOwner();
    error QuorumAlreadyExists();
    error QuorumDoesNotExist();

    event LookAheadPeriodChanged(uint32 oldLookAheadBlocks, uint32 newLookAheadBlocks);
    event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
    event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
    event QuorumCreated(uint8 indexed quorumNumber);
    event StakeTypeSet(IStakeRegistryTypes.StakeType newStakeType);
    event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
    event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
    event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);

    constructor(address _slashingRegistryCoordinator, address _delegationManager, address _avsDirectory, address _allocationManager);

    function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
    function WEIGHTING_DIVISOR() external view returns (uint256);
    function addStrategies(uint8 quorumNumber, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    function allocationManager() external view returns (address);
    function avsDirectory() external view returns (address);
    function delegation() external view returns (address);
    function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
    function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
    function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);
    function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint96);
    function getStakeAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, bytes32 operatorId, uint256 index) external view returns (uint96);
    function getStakeHistory(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate[] memory);
    function getStakeHistoryLength(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
    function getStakeUpdateAtIndex(uint8 quorumNumber, bytes32 operatorId, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    function getStakeUpdateIndexAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
    function getTotalStakeAtBlockNumberFromIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (uint96);
    function getTotalStakeHistoryLength(uint8 quorumNumber) external view returns (uint256);
    function getTotalStakeIndicesAtBlockNumber(uint32 blockNumber, bytes memory quorumNumbers) external view returns (uint32[] memory);
    function getTotalStakeUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    function initializeDelegatedStakeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    function initializeSlashableStakeQuorum(uint8 quorumNumber, uint96 minimumStake, uint32 lookAheadPeriod, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    function isOperatorSetQuorum(uint8 quorumNumber) external view returns (bool);
    function minimumStakeForQuorum(uint8) external view returns (uint96);
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
    function registryCoordinator() external view returns (address);
    function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
    function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
    function setSlashableStakeLookahead(uint8 quorumNumber, uint32 _lookAheadBlocks) external;
    function slashableStakeLookAheadPerQuorum(uint8 quorumNumber) external view returns (uint32);
    function stakeTypePerQuorum(uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeType);
    function strategiesPerQuorum(uint8 quorumNumber, uint256) external view returns (address);
    function strategyParams(uint8 quorumNumber, uint256) external view returns (address strategy, uint96 multiplier);
    function strategyParamsByIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StrategyParams memory);
    function strategyParamsLength(uint8 quorumNumber) external view returns (uint256);
    function updateOperatorStake(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint192);
    function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_slashingRegistryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      },
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "MAX_WEIGHING_FUNCTION_LENGTH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "WEIGHTING_DIVISOR",
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
    "name": "addStrategies",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
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
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegation",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelegationManager"
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
    "name": "getCurrentStake",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentTotalStake",
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
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestStakeUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
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
        "internalType": "struct IStakeRegistryTypes.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeAtBlockNumber",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
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
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeAtBlockNumberAndIndex",
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
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeHistory",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate[]",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeHistoryLength",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
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
    "name": "getStakeUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeUpdateIndexAtBlockNumber",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
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
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeAtBlockNumberFromIndex",
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
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeHistoryLength",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializeDelegatedStakeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
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
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initializeSlashableStakeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistryTypes.StrategyParams[]",
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
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isOperatorSetQuorum",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumStakeForQuorum",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "modifyStrategyParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "strategyIndices",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "newMultipliers",
        "type": "uint96[]",
        "internalType": "uint96[]"
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
        "type": "uint96[]",
        "internalType": "uint96[]"
      },
      {
        "name": "",
        "type": "uint96[]",
        "internalType": "uint96[]"
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
    "name": "removeStrategies",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "indicesToRemove",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMinimumStakeForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setSlashableStakeLookahead",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_lookAheadBlocks",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashableStakeLookAheadPerQuorum",
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
    "type": "function",
    "name": "stakeTypePerQuorum",
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
        "type": "uint8",
        "internalType": "enum IStakeRegistryTypes.StakeType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategiesPerQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
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
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParamsByIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistryTypes.StrategyParams",
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
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParamsLength",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateOperatorStake",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "weightOfOperatorForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "LookAheadPeriodChanged",
    "inputs": [
      {
        "name": "oldLookAheadBlocks",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "newLookAheadBlocks",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinimumStakeForQuorumUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorStakeUpdate",
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
        "name": "stake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumCreated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakeTypeSet",
    "inputs": [
      {
        "name": "newStakeType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IStakeRegistryTypes.StakeType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyAddedToQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyMultiplierUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "multiplier",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyRemovedFromQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "BelowMinimumStakeRequirement",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyStakeHistory",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputDuplicateStrategy",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputMultiplierZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlySlashingRegistryCoordinator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlySlashingRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "QuorumDoesNotExist",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b50604051613a22380380613a2283398101604081905261002f91610068565b6001600160a01b0393841660e052918316608052821660a0521660c0526100c4565b6001600160a01b0381168114610065575f5ffd5b50565b5f5f5f5f6080858703121561007b575f5ffd5b845161008681610051565b602086015190945061009781610051565b60408601519093506100a881610051565b60608601519092506100b981610051565b939692955090935050565b60805160a05160c05160e0516138de6101445f395f818161042801528181610ece01528181611325015281816113930152818161174001528181611f0601528181611fc20152612ab101525f81816105f301528181610e9f015281816117110152612a7701525f6103e901525f81816106530152611d7701526138de5ff3fe608060405234801561000f575f5ffd5b5060043610610229575f3560e01c80639f3ccf651161012a578063c8294c56116100b4578063df5cf72311610079578063df5cf7231461064e578063e086adb314610675578063f2be94ae14610688578063f851e1981461069b578063fa28c627146106ae575f5ffd5b8063c8294c56146105db578063ca8aa7c7146105ee578063cc5a7c2014610615578063d5eccc0514610628578063dd9846b91461063b575f5ffd5b8063b6904b78116100fa578063b6904b7814610567578063bc9a40c31461057a578063bd29b8cd1461058d578063c46778a5146105a0578063c601527d146105c8575f5ffd5b80639f3ccf65146104d15780639f8aff26146104e4578063ac6bfb0314610507578063adc804da14610527575f5ffd5b80635e5a6775116101b65780636d14a9871161017b5780636d14a9871461042357806375d4173a1461044a5780637c1723471461045d57806381c07502146104775780639ab4d6ff14610497575f5ffd5b80635e5a6775146103685780635f1f2d771461037757806366acfefe1461038a578063697fbd93146103b55780636b3aa72e146103e4575f5ffd5b806325504777116101fc57806325504777146102c35780632cd95940146102e45780633ca5a5f5146103045780634bd26e09146103265780635401ed2714610355575f5ffd5b80630491b41c1461022d57806308732461146102625780631f9b74e01461028357806320b66298146102ae575b5f5ffd5b61024f61023b366004612c3a565b60ff165f9081526001602052604090205490565b6040519081526020015b60405180910390f35b610275610270366004612c53565b6106c1565b604051610259929190612c7b565b610296610291366004612cb1565b610706565b6040516001600160601b039091168152602001610259565b6102c16102bc366004612d26565b610728565b005b6102d66102d1366004612de1565b6108c6565b604051610259929190612e7b565b6102f76102f2366004612e9f565b610a4e565b6040516102599190612ec9565b61024f610312366004612c3a565b60ff165f9081526003602052604090205490565b61024f610334366004612e9f565b5f91825260026020908152604080842060ff93909316845291905290205490565b610296610363366004612e9f565b610aeb565b61024f670de0b6b3a764000081565b6102c1610385366004612fd0565b610b03565b61039d610398366004612de1565b610fa3565b6040516001600160c01b039091168152602001610259565b6103d76103c3366004612c3a565b60056020525f908152604090205460ff1681565b604051610259919061308c565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610259565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6102c161045836600461315d565b61103f565b610465602081565b60405160ff9091168152602001610259565b61048a6104853660046131c9565b611125565b6040516102599190613217565b6104bc6104a5366004612c3a565b60066020525f908152604090205463ffffffff1681565b60405163ffffffff9091168152602001610259565b61040b6104df366004612c53565b6112cd565b6104f76104f2366004612c3a565b611301565b6040519015158152602001610259565b61051a610515366004613254565b611426565b6040516102599190613284565b61053a610535366004612c53565b6114bc565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610259565b61051a610575366004612c53565b611533565b6102c16105883660046132be565b6115c0565b6102c161059b3660046132e6565b6115e1565b6102966105ae366004612c3a565b5f602081905290815260409020546001600160601b031681565b6102c16105d6366004613314565b611644565b6102966105e936600461335e565b611807565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6102c1610623366004613398565b611883565b610296610636366004612c3a565b611975565b6104bc610649366004613402565b6119c6565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6102c161068336600461343b565b6119d2565b610296610696366004613463565b6119ee565b61051a6106a9366004612e9f565b611a81565b6102966106bc366004613402565b611b67565b6003602052815f5260405f2081815481106106da575f80fd5b5f918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b5f8261071181611bc6565b5f61071c8585611bf8565b509250505b5092915050565b610730611f04565b8461073a81611bc6565b83806107595760405163796cc52560e01b815260040160405180910390fd5b828114610779576040516343714afd60e01b815260040160405180910390fd5b60ff87165f908152600360205260408120905b828110156108bb578585828181106107a6576107a66134a2565b90506020020160208101906107bb91906134b6565b828989848181106107ce576107ce6134a2565b90506020020135815481106107e5576107e56134a2565b905f5260205f20015f0160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a8581811061084b5761084b6134a2565b9050602002013581548110610862576108626134a2565b5f918252602090912001546001600160a01b0316888885818110610888576108886134a2565b905060200201602081019061089d91906134b6565b6040516108ab929190612c7b565b60405180910390a260010161078c565b505050505050505050565b6060806108d1611fb7565b5f836001600160401b038111156108ea576108ea612f42565b604051908082528060200260200182016040528015610913578160200160208202803683370190505b5090505f846001600160401b0381111561092f5761092f612f42565b604051908082528060200260200182016040528015610958578160200160208202803683370190505b5090505f5b85811015610a40575f878783818110610978576109786134a2565b919091013560f81c915061098d905081611bc6565b5f5f610999838d611bf8565b91509150806109bb5760405163207f13e360e11b815260040160405180910390fd5b5f6109c78c8585612000565b9050828786815181106109dc576109dc6134a2565b60200260200101906001600160601b031690816001600160601b031681525050610a068482612279565b868681518110610a1857610a186134a2565b6001600160601b039092166020928302919091019091015250506001909201915061095d9050565b509097909650945050505050565b5f82815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610ade575f848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610a86565b5050505090505b92915050565b5f5f610af78484611a81565b60400151949350505050565b610b0b611f04565b81610b1581611bc6565b815180610b355760405163796cc52560e01b815260040160405180910390fd5b60ff84165f908152600360209081526040808320600490925282209091836001600160401b03811115610b6a57610b6a612f42565b604051908082528060200260200182016040528015610b93578160200160208202803683370190505b5090505f5b84811015610e8e5783878281518110610bb357610bb36134a2565b602002602001015181548110610bcb57610bcb6134a2565b5f9182526020909120015482516001600160a01b0390911690839083908110610bf657610bf66134a2565b60200260200101906001600160a01b031690816001600160a01b0316815250508760ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f785898481518110610c4e57610c4e6134a2565b602002602001015181548110610c6657610c666134a2565b5f91825260209182902001546040516001600160a01b0390911681520160405180910390a28760ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7585898481518110610cc357610cc36134a2565b602002602001015181548110610cdb57610cdb6134a2565b5f91825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a283548490610d1a906001906134e3565b81548110610d2a57610d2a6134a2565b905f5260205f200184888381518110610d4557610d456134a2565b602002602001015181548110610d5d57610d5d6134a2565b5f91825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558354849080610daf57610daf6134f6565b5f8281526020812082015f199081019190915501905582548390610dd5906001906134e3565b81548110610de557610de56134a2565b905f5260205f20015f9054906101000a90046001600160a01b031683888381518110610e1357610e136134a2565b602002602001015181548110610e2b57610e2b6134a2565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555082805480610e6657610e666134f6565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610b98565b50610e9887611301565b15610f9a577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b66bd9897f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316630764cb936040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f4c919061350a565b89846040518463ffffffff1660e01b8152600401610f6c93929190613525565b5f604051808303815f87803b158015610f83575f5ffd5b505af1158015610f95573d5f5f3e3d5ffd5b505050505b50505050505050565b5f610fac611fb7565b5f805b83811015611035575f858583818110610fca57610fca6134a2565b919091013560f81c9150610fdf905081611bc6565b5f5f610feb838b611bf8565b915091508061100c575f9150600160ff84161b6001600160c01b0386161794505b5f6110188a8585612000565b90506110248482612279565b505060019093019250610faf915050565b5095945050505050565b611047611fb7565b60ff83165f9081526001602052604090205415611077576040516310cda51760e21b815260040160405180910390fd5b61108183826123ea565b61108b83836126e2565b611095835f61274a565b505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60605f826001600160401b0381111561114057611140612f42565b604051908082528060200260200182016040528015611169578160200160208202803683370190505b5090505f5b838110156112c2575f858583818110611189576111896134a2565b919091013560f81c915061119e905081611bc6565b60ff81165f908152600160205260408120805463ffffffff8a1692906111c6576111c66134a2565b5f9182526020909120015463ffffffff1611156111f65760405163cc64657360e01b815260040160405180910390fd5b60ff81165f90815260016020526040812054905b818110156112b75760ff83165f90815260016020819052604090912063ffffffff8b169161123884866134e3565b61124291906134e3565b81548110611252576112526134a2565b5f9182526020909120015463ffffffff16116112af57600161127482846134e3565b61127e91906134e3565b858581518110611290576112906134a2565b602002602001019063ffffffff16908163ffffffff16815250506112b7565b60010161120a565b50505060010161116e565b5090505b9392505050565b6004602052815f5260405f2081815481106112e6575f80fd5b5f918252602090912001546001600160a01b03169150829050565b60405163a4d7871f60e01b815260ff821660048201525f9081906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a4d7871f90602401602060405180830381865afa15801561136a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061138e919061358f565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166381f936d26040518163ffffffff1660e01b8152600401602060405180830381865afa1580156113ed573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611411919061358f565b905080801561141e575081155b949350505050565b604080516060810182525f80825260208083018290528284018290528582526002815283822060ff8816835290529190912080548390811061146a5761146a6134a2565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091525f808252602082015260ff83165f9081526003602052604090208054839081106114f2576114f26134a2565b5f918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182525f808252602080830182905282840182905260ff86168252600190529190912080548390811061156f5761156f6134a2565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b6115c8611f04565b816115d281611bc6565b6115dc83836126e2565b505050565b6115e9611fb7565b5f5b8181101561163e575f838383818110611606576116066134a2565b919091013560f81c915061161b905081611bc6565b5f61162786835f612000565b90506116338282612279565b5050506001016115eb565b50505050565b61164c611f04565b8161165681611bc6565b61166083836123ea565b815161166b84611301565b1561163e575f816001600160401b0381111561168957611689612f42565b6040519080825280602002602001820160405280156116b2578160200160208202803683370190505b5090505f5b8281101561170e578481815181106116d1576116d16134a2565b60200260200101515f01518282815181106116ee576116ee6134a2565b6001600160a01b03909216602092830291909101909101526001016116b7565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166350feea207f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316630764cb936040518163ffffffff1660e01b8152600401602060405180830381865afa15801561179a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117be919061350a565b87846040518463ffffffff1660e01b81526004016117de93929190613525565b5f604051808303815f87803b1580156117f5575f5ffd5b505af11580156108bb573d5f5f3e3d5ffd5b60ff83165f90815260016020526040812080548291908490811061182d5761182d6134a2565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610af781856127b7565b61188b611fb7565b60ff84165f90815260016020526040902054156118bb576040516310cda51760e21b815260040160405180910390fd5b6118c584826123ea565b6118cf84846126e2565b6118da84600161274a565b6118e4848361282e565b50505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff81165f908152600160208190526040822080549091611995916134e3565b815481106119a5576119a56134a2565b5f91825260209091200154600160401b90046001600160601b031692915050565b5f61141e84848461289a565b6119da611f04565b816119e481611bc6565b6115dc838361282e565b5f82815260026020908152604080832060ff881684529091528120805482919084908110611a1e57611a1e6134a2565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611a7481866127b7565b6040015195945050505050565b60408051606080820183525f80835260208084018290528385018290528682526002815284822060ff8716835281528482205485519384018652828452908301829052938201819052919291829003611add579150610ae59050565b5f85815260026020908152604080832060ff881684529091529020611b036001846134e3565b81548110611b1357611b136134a2565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610ae5915050565b5f83815260026020908152604080832060ff861684529091528120611b8d85858561289a565b63ffffffff1681548110611ba357611ba36134a2565b5f91825260209091200154600160401b90046001600160601b0316949350505050565b60ff81165f90815260016020526040902054611bf557604051637310cff560e11b815260040160405180910390fd5b50565b5f5f5f5f611c148660ff165f9081526003602052604090205490565b604080518082019091525f80825260208201529091506060600160ff808a165f90815260056020526040902054166001811115611c5357611c53613078565b03611d4c57611c628888612a01565b90505f5b83811015611d465760ff89165f908152600360205260409020805482908110611c9157611c916134a2565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611cde57611cde6134a2565b60200260200101511115611d3e57670de0b6b3a764000083602001516001600160601b0316838381518110611d1557611d156134a2565b6020026020010151611d2791906135ae565b611d3191906135c5565b611d3b90866135e4565b94505b600101611c66565b50611ed2565b60ff88165f908152600460208190526040918290209151639004134760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001692639004134792611dab928c92910161363f565b5f60405180830381865afa158015611dc5573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611dec91908101906136bd565b90505f5b83811015611ed05760ff89165f908152600360205260409020805482908110611e1b57611e1b6134a2565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611e6857611e686134a2565b60200260200101511115611ec857670de0b6b3a764000083602001516001600160601b0316838381518110611e9f57611e9f6134a2565b6020026020010151611eb191906135ae565b611ebb91906135c5565b611ec590866135e4565b94505b600101611df0565b505b50505060ff85165f908152602081905260409020549092506001600160601b03908116908316101590505b9250929050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f60573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f84919061350a565b6001600160a01b0316336001600160a01b031614611fb55760405163ce98c24b60e01b815260040160405180910390fd5b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611fb557604051632c01b20560e21b815260040160405180910390fd5b5f83815260026020908152604080832060ff8616845290915281205481908082036120c4575f86815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561221f565b5f86815260026020908152604080832060ff8916845290915281206120ea6001846134e3565b815481106120fa576120fa6134a2565b5f91825260209091200180546001600160601b03600160401b9091048116945090915085168303612130575f93505050506112c6565b805463ffffffff438116911603612168578054600160401b600160a01b031916600160401b6001600160601b0387160217815561221d565b805467ffffffff000000001916600160201b4363ffffffff9081168281029390931784555f8a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a261226f8285612be1565b9695505050505050565b60ff82165f908152600160208190526040822080549183919061229c90846134e3565b815481106122ac576122ac6134a2565b905f5260205f20019050835f036122d75754600160401b90046001600160601b03169150610ae59050565b80545f906122f590600160401b90046001600160601b031686612bf8565b825490915063ffffffff438116911603612330578154600160401b600160a01b031916600160401b6001600160601b038316021782556123e1565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff89165f90815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b5f81511161240b5760405163796cc52560e01b815260040160405180910390fd5b805160ff83165f908152600360209081526040909120549061242d83836136ee565b111561244c576040516343714afd60e01b815260040160405180910390fd5b5f5b828110156126db575f5b61246282846136ee565b8110156124f15784828151811061247b5761247b6134a2565b60200260200101515f01516001600160a01b031660035f8860ff1660ff1681526020019081526020015f2082815481106124b7576124b76134a2565b5f918252602090912001546001600160a01b0316036124e957604051637b74340b60e01b815260040160405180910390fd5b600101612458565b505f848281518110612505576125056134a2565b6020026020010151602001516001600160601b03161161253857604051637257125160e01b815260040160405180910390fd5b60ff85165f908152600360205260409020845185908390811061255d5761255d6134a2565b60209081029190910181015182546001810184555f9384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106125c1576125c16134a2565b6020908102919091018101515182546001810184555f938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540490869084908110612637576126376134a2565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75858381518110612694576126946134a2565b60200260200101515f01518684815181106126b1576126b16134a2565b6020026020010151602001516040516126cb929190612c7b565b60405180910390a260010161244e565b5050505050565b60ff82165f818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60ff82165f908152600560205260409020805482919060ff19166001838181111561277757612777613078565b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516127ab919061308c565b60405180910390a15050565b815f015163ffffffff168163ffffffff1610156127e757604051631391e11b60e21b815260040160405180910390fd5b602082015163ffffffff16158061280d5750816020015163ffffffff168163ffffffff16105b61282a57604051631391e11b60e21b815260040160405180910390fd5b5050565b60ff82165f90815260066020908152604091829020805463ffffffff19811663ffffffff8681169182179093558451929091168083529282015290917f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7910160405180910390a1505050565b5f83815260026020908152604080832060ff86168452909152812054805b8015612938575f86815260026020908152604080832060ff89168452909152902063ffffffff8516906128ec6001846134e3565b815481106128fc576128fc6134a2565b5f9182526020909120015463ffffffff16116129265761291d6001826134e3565b925050506112c6565b8061293081613701565b9150506128b8565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e40160405180910390fd5b6040805160018082528183019092526060915f91906020808301908036833701905050905082815f81518110612a3957612a396134a2565b6001600160a01b0390921660209283029190910182015260ff85165f90815260069091526040812054612a729063ffffffff16436136ee565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632bab2c4a60405180604001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316630764cb936040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b2f919061350a565b6001600160a01b0316815260ff8a1660209182018190525f90815260049182905260409081902090516001600160e01b031960e086901b168152612b799392899291899101613716565b5f60405180830381865afa158015612b93573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612bba91908101906137a1565b9050805f81518110612bce57612bce6134a2565b6020026020010151935050505092915050565b5f6112c66001600160601b03808516908416613850565b5f5f821215612c1b57612c0a8261386f565b612c149084613889565b9050610ae5565b612c1482846135e4565b803560ff81168114612c35575f5ffd5b919050565b5f60208284031215612c4a575f5ffd5b6112c682612c25565b5f5f60408385031215612c64575f5ffd5b612c6d83612c25565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114611bf5575f5ffd5b5f5f60408385031215612cc2575f5ffd5b612ccb83612c25565b91506020830135612cdb81612c9d565b809150509250929050565b5f5f83601f840112612cf6575f5ffd5b5081356001600160401b03811115612d0c575f5ffd5b6020830191508360208260051b8501011115611efd575f5ffd5b5f5f5f5f5f60608688031215612d3a575f5ffd5b612d4386612c25565b945060208601356001600160401b03811115612d5d575f5ffd5b612d6988828901612ce6565b90955093505060408601356001600160401b03811115612d87575f5ffd5b612d9388828901612ce6565b969995985093965092949392505050565b5f5f83601f840112612db4575f5ffd5b5081356001600160401b03811115612dca575f5ffd5b602083019150836020828501011115611efd575f5ffd5b5f5f5f5f60608587031215612df4575f5ffd5b8435612dff81612c9d565b93506020850135925060408501356001600160401b03811115612e20575f5ffd5b612e2c87828801612da4565b95989497509550505050565b5f8151808452602084019350602083015f5b82811015612e715781516001600160601b0316865260209586019590910190600101612e4a565b5093949350505050565b604081525f612e8d6040830185612e38565b82810360208401526123e18185612e38565b5f5f60408385031215612eb0575f5ffd5b82359150612ec060208401612c25565b90509250929050565b602080825282518282018190525f918401906040840190835b81811015612f3757612f2183855163ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b6020939093019260609290920191600101612ee2565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715612f7857612f78612f42565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612fa657612fa6612f42565b604052919050565b5f6001600160401b03821115612fc657612fc6612f42565b5060051b60200190565b5f5f60408385031215612fe1575f5ffd5b612fea83612c25565b915060208301356001600160401b03811115613004575f5ffd5b8301601f81018513613014575f5ffd5b803561302761302282612fae565b612f7e565b8082825260208201915060208360051b850101925087831115613048575f5ffd5b6020840193505b8284101561306a57833582526020938401939091019061304f565b809450505050509250929050565b634e487b7160e01b5f52602160045260245ffd5b60208101600283106130ac57634e487b7160e01b5f52602160045260245ffd5b91905290565b80356001600160601b0381168114612c35575f5ffd5b5f82601f8301126130d7575f5ffd5b81356130e561302282612fae565b8082825260208201915060208360061b860101925085831115613106575f5ffd5b602085015b838110156110355760408188031215613122575f5ffd5b61312a612f56565b813561313581612c9d565b8152613143602083016130b2565b60208201528084525060208301925060408101905061310b565b5f5f5f6060848603121561316f575f5ffd5b61317884612c25565b9250613186602085016130b2565b915060408401356001600160401b038111156131a0575f5ffd5b6131ac868287016130c8565b9150509250925092565b803563ffffffff81168114612c35575f5ffd5b5f5f5f604084860312156131db575f5ffd5b6131e4846131b6565b925060208401356001600160401b038111156131fe575f5ffd5b61320a86828701612da4565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b81811015612f3757835163ffffffff16835260209384019390920191600101613230565b5f5f5f60608486031215613266575f5ffd5b61326f84612c25565b95602085013595506040909401359392505050565b60608101610ae5828463ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b5f5f604083850312156132cf575f5ffd5b6132d883612c25565b9150612ec0602084016130b2565b5f5f5f604084860312156132f8575f5ffd5b8335925060208401356001600160401b038111156131fe575f5ffd5b5f5f60408385031215613325575f5ffd5b61332e83612c25565b915060208301356001600160401b03811115613348575f5ffd5b613354858286016130c8565b9150509250929050565b5f5f5f60608486031215613370575f5ffd5b61337984612c25565b9250613387602085016131b6565b929592945050506040919091013590565b5f5f5f5f608085870312156133ab575f5ffd5b6133b485612c25565b93506133c2602086016130b2565b92506133d0604086016131b6565b915060608501356001600160401b038111156133ea575f5ffd5b6133f6878288016130c8565b91505092959194509250565b5f5f5f60608486031215613414575f5ffd5b8335925061342460208501612c25565b9150613432604085016131b6565b90509250925092565b5f5f6040838503121561344c575f5ffd5b61345583612c25565b9150612ec0602084016131b6565b5f5f5f5f60808587031215613476575f5ffd5b61347f85612c25565b935061348d602086016131b6565b93969395505050506040820135916060013590565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156134c6575f5ffd5b6112c6826130b2565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610ae557610ae56134cf565b634e487b7160e01b5f52603160045260245ffd5b5f6020828403121561351a575f5ffd5b81516112c681612c9d565b6001600160a01b038416815260ff831660208083019190915260606040830181905283519083018190525f918401906080840190835b818110156135825783516001600160a01b031683526020938401939092019160010161355b565b5090979650505050505050565b5f6020828403121561359f575f5ffd5b815180151581146112c6575f5ffd5b8082028115828204841417610ae557610ae56134cf565b5f826135df57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160601b038181168382160190811115610ae557610ae56134cf565b5f8154808452602084019350825f5260205f205f5b82811015612e715781546001600160a01b0316865260209095019460019182019101613618565b6001600160a01b03831681526040602082018190525f9061141e90830184613603565b5f82601f830112613671575f5ffd5b815161367f61302282612fae565b8082825260208201915060208360051b8601019250858311156136a0575f5ffd5b602085015b838110156110355780518352602092830192016136a5565b5f602082840312156136cd575f5ffd5b81516001600160401b038111156136e2575f5ffd5b61141e84828501613662565b80820180821115610ae557610ae56134cf565b5f8161370f5761370f6134cf565b505f190190565b84516001600160a01b0316815260208086015163ffffffff168183015260a06040830181905285519083018190525f9186019060c0840190835b818110156137775783516001600160a01b0316835260209384019390920191600101613750565b5050838103606085015261378b8187613603565b925050506123e1608083018463ffffffff169052565b5f602082840312156137b1575f5ffd5b81516001600160401b038111156137c6575f5ffd5b8201601f810184136137d6575f5ffd5b80516137e461302282612fae565b8082825260208201915060208360051b850101925086831115613805575f5ffd5b602084015b838110156138455780516001600160401b03811115613827575f5ffd5b61383689602083890101613662565b8452506020928301920161380a565b509695505050505050565b8181035f831280158383131683831282161715610721576107216134cf565b5f600160ff1b8201613883576138836134cf565b505f0390565b6001600160601b038281168282160390811115610ae557610ae56134cf56fea2646970667358221220c2c11ea3fbabf1ea5764f279bec484b353a24b049000160105e414393a50f62264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa:\"8\x03\x80a:\"\x839\x81\x01`@\x81\x90Ra\0/\x91a\0hV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xE0R\x91\x83\x16`\x80R\x82\x16`\xA0R\x16`\xC0Ra\0\xC4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0eW__\xFD[PV[____`\x80\x85\x87\x03\x12\x15a\0{W__\xFD[\x84Qa\0\x86\x81a\0QV[` \x86\x01Q\x90\x94Pa\0\x97\x81a\0QV[`@\x86\x01Q\x90\x93Pa\0\xA8\x81a\0QV[``\x86\x01Q\x90\x92Pa\0\xB9\x81a\0QV[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa8\xDEa\x01D_9_\x81\x81a\x04(\x01R\x81\x81a\x0E\xCE\x01R\x81\x81a\x13%\x01R\x81\x81a\x13\x93\x01R\x81\x81a\x17@\x01R\x81\x81a\x1F\x06\x01R\x81\x81a\x1F\xC2\x01Ra*\xB1\x01R_\x81\x81a\x05\xF3\x01R\x81\x81a\x0E\x9F\x01R\x81\x81a\x17\x11\x01Ra*w\x01R_a\x03\xE9\x01R_\x81\x81a\x06S\x01Ra\x1Dw\x01Ra8\xDE_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02)W_5`\xE0\x1C\x80c\x9F<\xCFe\x11a\x01*W\x80c\xC8)LV\x11a\0\xB4W\x80c\xDF\\\xF7#\x11a\0yW\x80c\xDF\\\xF7#\x14a\x06NW\x80c\xE0\x86\xAD\xB3\x14a\x06uW\x80c\xF2\xBE\x94\xAE\x14a\x06\x88W\x80c\xF8Q\xE1\x98\x14a\x06\x9BW\x80c\xFA(\xC6'\x14a\x06\xAEW__\xFD[\x80c\xC8)LV\x14a\x05\xDBW\x80c\xCA\x8A\xA7\xC7\x14a\x05\xEEW\x80c\xCCZ| \x14a\x06\x15W\x80c\xD5\xEC\xCC\x05\x14a\x06(W\x80c\xDD\x98F\xB9\x14a\x06;W__\xFD[\x80c\xB6\x90Kx\x11a\0\xFAW\x80c\xB6\x90Kx\x14a\x05gW\x80c\xBC\x9A@\xC3\x14a\x05zW\x80c\xBD)\xB8\xCD\x14a\x05\x8DW\x80c\xC4gx\xA5\x14a\x05\xA0W\x80c\xC6\x01R}\x14a\x05\xC8W__\xFD[\x80c\x9F<\xCFe\x14a\x04\xD1W\x80c\x9F\x8A\xFF&\x14a\x04\xE4W\x80c\xACk\xFB\x03\x14a\x05\x07W\x80c\xAD\xC8\x04\xDA\x14a\x05'W__\xFD[\x80c^Zgu\x11a\x01\xB6W\x80cm\x14\xA9\x87\x11a\x01{W\x80cm\x14\xA9\x87\x14a\x04#W\x80cu\xD4\x17:\x14a\x04JW\x80c|\x17#G\x14a\x04]W\x80c\x81\xC0u\x02\x14a\x04wW\x80c\x9A\xB4\xD6\xFF\x14a\x04\x97W__\xFD[\x80c^Zgu\x14a\x03hW\x80c_\x1F-w\x14a\x03wW\x80cf\xAC\xFE\xFE\x14a\x03\x8AW\x80ci\x7F\xBD\x93\x14a\x03\xB5W\x80ck:\xA7.\x14a\x03\xE4W__\xFD[\x80c%PGw\x11a\x01\xFCW\x80c%PGw\x14a\x02\xC3W\x80c,\xD9Y@\x14a\x02\xE4W\x80c<\xA5\xA5\xF5\x14a\x03\x04W\x80cK\xD2n\t\x14a\x03&W\x80cT\x01\xED'\x14a\x03UW__\xFD[\x80c\x04\x91\xB4\x1C\x14a\x02-W\x80c\x08s$a\x14a\x02bW\x80c\x1F\x9Bt\xE0\x14a\x02\x83W\x80c \xB6b\x98\x14a\x02\xAEW[__\xFD[a\x02Oa\x02;6`\x04a,:V[`\xFF\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ua\x02p6`\x04a,SV[a\x06\xC1V[`@Qa\x02Y\x92\x91\x90a,{V[a\x02\x96a\x02\x916`\x04a,\xB1V[a\x07\x06V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02YV[a\x02\xC1a\x02\xBC6`\x04a-&V[a\x07(V[\0[a\x02\xD6a\x02\xD16`\x04a-\xE1V[a\x08\xC6V[`@Qa\x02Y\x92\x91\x90a.{V[a\x02\xF7a\x02\xF26`\x04a.\x9FV[a\nNV[`@Qa\x02Y\x91\x90a.\xC9V[a\x02Oa\x03\x126`\x04a,:V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Oa\x0346`\x04a.\x9FV[_\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x96a\x03c6`\x04a.\x9FV[a\n\xEBV[a\x02Og\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xC1a\x03\x856`\x04a/\xD0V[a\x0B\x03V[a\x03\x9Da\x03\x986`\x04a-\xE1V[a\x0F\xA3V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02YV[a\x03\xD7a\x03\xC36`\x04a,:V[`\x05` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02Y\x91\x90a0\x8CV[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02YV[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x04X6`\x04a1]V[a\x10?V[a\x04e` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02YV[a\x04\x8Aa\x04\x856`\x04a1\xC9V[a\x11%V[`@Qa\x02Y\x91\x90a2\x17V[a\x04\xBCa\x04\xA56`\x04a,:V[`\x06` R_\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02YV[a\x04\x0Ba\x04\xDF6`\x04a,SV[a\x12\xCDV[a\x04\xF7a\x04\xF26`\x04a,:V[a\x13\x01V[`@Q\x90\x15\x15\x81R` \x01a\x02YV[a\x05\x1Aa\x05\x156`\x04a2TV[a\x14&V[`@Qa\x02Y\x91\x90a2\x84V[a\x05:a\x0556`\x04a,SV[a\x14\xBCV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02YV[a\x05\x1Aa\x05u6`\x04a,SV[a\x153V[a\x02\xC1a\x05\x886`\x04a2\xBEV[a\x15\xC0V[a\x02\xC1a\x05\x9B6`\x04a2\xE6V[a\x15\xE1V[a\x02\x96a\x05\xAE6`\x04a,:V[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\xC1a\x05\xD66`\x04a3\x14V[a\x16DV[a\x02\x96a\x05\xE96`\x04a3^V[a\x18\x07V[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x06#6`\x04a3\x98V[a\x18\x83V[a\x02\x96a\x0666`\x04a,:V[a\x19uV[a\x04\xBCa\x06I6`\x04a4\x02V[a\x19\xC6V[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x06\x836`\x04a4;V[a\x19\xD2V[a\x02\x96a\x06\x966`\x04a4cV[a\x19\xEEV[a\x05\x1Aa\x06\xA96`\x04a.\x9FV[a\x1A\x81V[a\x02\x96a\x06\xBC6`\x04a4\x02V[a\x1BgV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x06\xDAW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[_\x82a\x07\x11\x81a\x1B\xC6V[_a\x07\x1C\x85\x85a\x1B\xF8V[P\x92PP[P\x92\x91PPV[a\x070a\x1F\x04V[\x84a\x07:\x81a\x1B\xC6V[\x83\x80a\x07YW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x07yW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x87\x16_\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\x08\xBBW\x85\x85\x82\x81\x81\x10a\x07\xA6Wa\x07\xA6a4\xA2V[\x90P` \x02\x01` \x81\x01\x90a\x07\xBB\x91\x90a4\xB6V[\x82\x89\x89\x84\x81\x81\x10a\x07\xCEWa\x07\xCEa4\xA2V[\x90P` \x02\x015\x81T\x81\x10a\x07\xE5Wa\x07\xE5a4\xA2V[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08KWa\x08Ka4\xA2V[\x90P` \x02\x015\x81T\x81\x10a\x08bWa\x08ba4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\x08\x88Wa\x08\x88a4\xA2V[\x90P` \x02\x01` \x81\x01\x90a\x08\x9D\x91\x90a4\xB6V[`@Qa\x08\xAB\x92\x91\x90a,{V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x07\x8CV[PPPPPPPPPV[``\x80a\x08\xD1a\x1F\xB7V[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xEAWa\x08\xEAa/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x13W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\t/Wa\t/a/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tXW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a\n@W_\x87\x87\x83\x81\x81\x10a\txWa\txa4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\t\x8D\x90P\x81a\x1B\xC6V[__a\t\x99\x83\x8Da\x1B\xF8V[\x91P\x91P\x80a\t\xBBW`@Qc \x7F\x13\xE3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\xC7\x8C\x85\x85a \0V[\x90P\x82\x87\x86\x81Q\x81\x10a\t\xDCWa\t\xDCa4\xA2V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\n\x06\x84\x82a\"yV[\x86\x86\x81Q\x81\x10a\n\x18Wa\n\x18a4\xA2V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x90\x92\x01\x91Pa\t]\x90PV[P\x90\x97\x90\x96P\x94PPPPPV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\n\xDEW_\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\n\x86V[PPPP\x90P[\x92\x91PPV[__a\n\xF7\x84\x84a\x1A\x81V[`@\x01Q\x94\x93PPPPV[a\x0B\x0Ba\x1F\x04V[\x81a\x0B\x15\x81a\x1B\xC6V[\x81Q\x80a\x0B5W`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x84\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BjWa\x0Bja/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x93W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x0E\x8EW\x83\x87\x82\x81Q\x81\x10a\x0B\xB3Wa\x0B\xB3a4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0B\xCBWa\x0B\xCBa4\xA2V[_\x91\x82R` \x90\x91 \x01T\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a\x0B\xF6Wa\x0B\xF6a4\xA2V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x87`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x85\x89\x84\x81Q\x81\x10a\x0CNWa\x0CNa4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0CfWa\x0Cfa4\xA2V[_\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x87`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x89\x84\x81Q\x81\x10a\x0C\xC3Wa\x0C\xC3a4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0C\xDBWa\x0C\xDBa4\xA2V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x83T\x84\x90a\r\x1A\x90`\x01\x90a4\xE3V[\x81T\x81\x10a\r*Wa\r*a4\xA2V[\x90_R` _ \x01\x84\x88\x83\x81Q\x81\x10a\rEWa\rEa4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\r]Wa\r]a4\xA2V[_\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x83T\x84\x90\x80a\r\xAFWa\r\xAFa4\xF6V[_\x82\x81R` \x81 \x82\x01_\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x82T\x83\x90a\r\xD5\x90`\x01\x90a4\xE3V[\x81T\x81\x10a\r\xE5Wa\r\xE5a4\xA2V[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x83\x81Q\x81\x10a\x0E\x13Wa\x0E\x13a4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0E+Wa\x0E+a4\xA2V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x82\x80T\x80a\x0EfWa\x0Efa4\xF6V[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\x0B\x98V[Pa\x0E\x98\x87a\x13\x01V[\x15a\x0F\x9AW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB6k\xD9\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x07d\xCB\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FL\x91\x90a5\nV[\x89\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Fl\x93\x92\x91\x90a5%V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\x83W__\xFD[PZ\xF1\x15\x80\x15a\x0F\x95W=__>=_\xFD[PPPP[PPPPPPPV[_a\x0F\xACa\x1F\xB7V[_\x80[\x83\x81\x10\x15a\x105W_\x85\x85\x83\x81\x81\x10a\x0F\xCAWa\x0F\xCAa4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x0F\xDF\x90P\x81a\x1B\xC6V[__a\x0F\xEB\x83\x8Ba\x1B\xF8V[\x91P\x91P\x80a\x10\x0CW_\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[_a\x10\x18\x8A\x85\x85a \0V[\x90Pa\x10$\x84\x82a\"yV[PP`\x01\x90\x93\x01\x92Pa\x0F\xAF\x91PPV[P\x95\x94PPPPPV[a\x10Ga\x1F\xB7V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x10wW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\x81\x83\x82a#\xEAV[a\x10\x8B\x83\x83a&\xE2V[a\x10\x95\x83_a'JV[PP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11@Wa\x11@a/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11iW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x12\xC2W_\x85\x85\x83\x81\x81\x10a\x11\x89Wa\x11\x89a4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x11\x9E\x90P\x81a\x1B\xC6V[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x11\xC6Wa\x11\xC6a4\xA2V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x11\xF6W`@Qc\xCCdes`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x12\xB7W`\xFF\x83\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x128\x84\x86a4\xE3V[a\x12B\x91\x90a4\xE3V[\x81T\x81\x10a\x12RWa\x12Ra4\xA2V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x12\xAFW`\x01a\x12t\x82\x84a4\xE3V[a\x12~\x91\x90a4\xE3V[\x85\x85\x81Q\x81\x10a\x12\x90Wa\x12\x90a4\xA2V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x12\xB7V[`\x01\x01a\x12\nV[PPP`\x01\x01a\x11nV[P\x90P[\x93\x92PPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x12\xE6W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@Qc\xA4\xD7\x87\x1F`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA4\xD7\x87\x1F\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x8E\x91\x90a5\x8FV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xF96\xD2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x11\x91\x90a5\x8FV[\x90P\x80\x80\x15a\x14\x1EWP\x81\x15[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14jWa\x14ja4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x14\xF2Wa\x14\xF2a4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15oWa\x15oa4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x15\xC8a\x1F\x04V[\x81a\x15\xD2\x81a\x1B\xC6V[a\x15\xDC\x83\x83a&\xE2V[PPPV[a\x15\xE9a\x1F\xB7V[_[\x81\x81\x10\x15a\x16>W_\x83\x83\x83\x81\x81\x10a\x16\x06Wa\x16\x06a4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x16\x1B\x90P\x81a\x1B\xC6V[_a\x16'\x86\x83_a \0V[\x90Pa\x163\x82\x82a\"yV[PPP`\x01\x01a\x15\xEBV[PPPPV[a\x16La\x1F\x04V[\x81a\x16V\x81a\x1B\xC6V[a\x16`\x83\x83a#\xEAV[\x81Qa\x16k\x84a\x13\x01V[\x15a\x16>W_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x89Wa\x16\x89a/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xB2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x17\x0EW\x84\x81\x81Q\x81\x10a\x16\xD1Wa\x16\xD1a4\xA2V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x16\xEEWa\x16\xEEa4\xA2V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x16\xB7V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cP\xFE\xEA \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x07d\xCB\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xBE\x91\x90a5\nV[\x87\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xDE\x93\x92\x91\x90a5%V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xF5W__\xFD[PZ\xF1\x15\x80\x15a\x08\xBBW=__>=_\xFD[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x18-Wa\x18-a4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\n\xF7\x81\x85a'\xB7V[a\x18\x8Ba\x1F\xB7V[`\xFF\x84\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x18\xBBW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xC5\x84\x82a#\xEAV[a\x18\xCF\x84\x84a&\xE2V[a\x18\xDA\x84`\x01a'JV[a\x18\xE4\x84\x83a(.V[PPP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x81\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x19\x95\x91a4\xE3V[\x81T\x81\x10a\x19\xA5Wa\x19\xA5a4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[_a\x14\x1E\x84\x84\x84a(\x9AV[a\x19\xDAa\x1F\x04V[\x81a\x19\xE4\x81a\x1B\xC6V[a\x15\xDC\x83\x83a(.V[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\x1EWa\x1A\x1Ea4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1At\x81\x86a'\xB7V[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x91\x92\x91\x82\x90\x03a\x1A\xDDW\x91Pa\n\xE5\x90PV[_\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1B\x03`\x01\x84a4\xE3V[\x81T\x81\x10a\x1B\x13Wa\x1B\x13a4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\n\xE5\x91PPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1B\x8D\x85\x85\x85a(\x9AV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1B\xA3Wa\x1B\xA3a4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x90 Ta\x1B\xF5W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[____a\x1C\x14\x86`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90\x91P```\x01`\xFF\x80\x8A\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a\x1CSWa\x1CSa0xV[\x03a\x1DLWa\x1Cb\x88\x88a*\x01V[\x90P_[\x83\x81\x10\x15a\x1DFW`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1C\x91Wa\x1C\x91a4\xA2V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1C\xDEWa\x1C\xDEa4\xA2V[` \x02` \x01\x01Q\x11\x15a\x1D>Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1D\x15Wa\x1D\x15a4\xA2V[` \x02` \x01\x01Qa\x1D'\x91\x90a5\xAEV[a\x1D1\x91\x90a5\xC5V[a\x1D;\x90\x86a5\xE4V[\x94P[`\x01\x01a\x1CfV[Pa\x1E\xD2V[`\xFF\x88\x16_\x90\x81R`\x04` \x81\x90R`@\x91\x82\x90 \x91Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1D\xAB\x92\x8C\x92\x91\x01a6?V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xC5W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\xEC\x91\x90\x81\x01\x90a6\xBDV[\x90P_[\x83\x81\x10\x15a\x1E\xD0W`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1E\x1BWa\x1E\x1Ba4\xA2V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1EhWa\x1Eha4\xA2V[` \x02` \x01\x01Q\x11\x15a\x1E\xC8Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1E\x9FWa\x1E\x9Fa4\xA2V[` \x02` \x01\x01Qa\x1E\xB1\x91\x90a5\xAEV[a\x1E\xBB\x91\x90a5\xC5V[a\x1E\xC5\x90\x86a5\xE4V[\x94P[`\x01\x01a\x1D\xF0V[P[PPP`\xFF\x85\x16_\x90\x81R` \x81\x90R`@\x90 T\x90\x92P`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x84\x91\x90a5\nV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F\xB5W`@Qc\xCE\x98\xC2K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1F\xB5W`@Qc,\x01\xB2\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x82\x03a \xC4W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"\x1FV[_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a \xEA`\x01\x84a4\xE3V[\x81T\x81\x10a \xFAWa \xFAa4\xA2V[_\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x03a!0W_\x93PPPPa\x12\xC6V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a!hW\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"\x1DV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U_\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"o\x82\x85a+\xE1V[\x96\x95PPPPPPV[`\xFF\x82\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\x9C\x90\x84a4\xE3V[\x81T\x81\x10a\"\xACWa\"\xACa4\xA2V[\x90_R` _ \x01\x90P\x83_\x03a\"\xD7WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\n\xE5\x90PV[\x80T_\x90a\"\xF5\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a+\xF8V[\x82T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a#0W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua#\xE1V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[_\x81Q\x11a$\x0BW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a$-\x83\x83a6\xEEV[\x11\x15a$LW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a&\xDBW_[a$b\x82\x84a6\xEEV[\x81\x10\x15a$\xF1W\x84\x82\x81Q\x81\x10a${Wa${a4\xA2V[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a$\xB7Wa$\xB7a4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a$\xE9W`@Qc{t4\x0B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a$XV[P_\x84\x82\x81Q\x81\x10a%\x05Wa%\x05a4\xA2V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a%8W`@QcrW\x12Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x85\x16_\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a%]Wa%]a4\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a%\xC1Wa%\xC1a4\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U_\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a&7Wa&7a4\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a&\x94Wa&\x94a4\xA2V[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a&\xB1Wa&\xB1a4\xA2V[` \x02` \x01\x01Q` \x01Q`@Qa&\xCB\x92\x91\x90a,{V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a$NV[PPPPPV[`\xFF\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\xFF\x82\x16_\x90\x81R`\x05` R`@\x90 \x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a'wWa'wa0xV[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa'\xAB\x91\x90a0\x8CV[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a'\xE7W`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a(\rWP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a(*W`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\xFF\x82\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x81\x16c\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x17\x90\x93U\x84Q\x92\x90\x91\x16\x80\x83R\x92\x82\x01R\x90\x91\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a)8W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a(\xEC`\x01\x84a4\xE3V[\x81T\x81\x10a(\xFCWa(\xFCa4\xA2V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a)&Wa)\x1D`\x01\x82a4\xE3V[\x92PPPa\x12\xC6V[\x80a)0\x81a7\x01V[\x91PPa(\xB8V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a*9Wa*9a4\xA2V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`\xFF\x85\x16_\x90\x81R`\x06\x90\x91R`@\x81 Ta*r\x90c\xFF\xFF\xFF\xFF\x16Ca6\xEEV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x07d\xCB\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+/\x91\x90a5\nV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xFF\x8A\x16` \x91\x82\x01\x81\x90R_\x90\x81R`\x04\x91\x82\x90R`@\x90\x81\x90 \x90Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra+y\x93\x92\x89\x92\x91\x89\x91\x01a7\x16V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x93W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\xBA\x91\x90\x81\x01\x90a7\xA1V[\x90P\x80_\x81Q\x81\x10a+\xCEWa+\xCEa4\xA2V[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[_a\x12\xC6`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a8PV[__\x82\x12\x15a,\x1BWa,\n\x82a8oV[a,\x14\x90\x84a8\x89V[\x90Pa\n\xE5V[a,\x14\x82\x84a5\xE4V[\x805`\xFF\x81\x16\x81\x14a,5W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a,JW__\xFD[a\x12\xC6\x82a,%V[__`@\x83\x85\x03\x12\x15a,dW__\xFD[a,m\x83a,%V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xF5W__\xFD[__`@\x83\x85\x03\x12\x15a,\xC2W__\xFD[a,\xCB\x83a,%V[\x91P` \x83\x015a,\xDB\x81a,\x9DV[\x80\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a,\xF6W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x0CW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\xFDW__\xFD[_____``\x86\x88\x03\x12\x15a-:W__\xFD[a-C\x86a,%V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-]W__\xFD[a-i\x88\x82\x89\x01a,\xE6V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x87W__\xFD[a-\x93\x88\x82\x89\x01a,\xE6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a-\xB4W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCAW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E\xFDW__\xFD[____``\x85\x87\x03\x12\x15a-\xF4W__\xFD[\x845a-\xFF\x81a,\x9DV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a. W__\xFD[a.,\x87\x82\x88\x01a-\xA4V[\x95\x98\x94\x97P\x95PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a.qW\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a.JV[P\x93\x94\x93PPPPV[`@\x81R_a.\x8D`@\x83\x01\x85a.8V[\x82\x81\x03` \x84\x01Ra#\xE1\x81\x85a.8V[__`@\x83\x85\x03\x12\x15a.\xB0W__\xFD[\x825\x91Pa.\xC0` \x84\x01a,%V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a/7Wa/!\x83\x85Qc\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[` \x93\x90\x93\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.\xE2V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/xWa/xa/BV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\xA6Wa/\xA6a/BV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xC6Wa/\xC6a/BV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a/\xE1W__\xFD[a/\xEA\x83a,%V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x04W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a0\x14W__\xFD[\x805a0'a0\"\x82a/\xAEV[a/~V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a0HW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a0jW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a0OV[\x80\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x02\x83\x10a0\xACWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,5W__\xFD[_\x82`\x1F\x83\x01\x12a0\xD7W__\xFD[\x815a0\xE5a0\"\x82a/\xAEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a1\x06W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x105W`@\x81\x88\x03\x12\x15a1\"W__\xFD[a1*a/VV[\x815a15\x81a,\x9DV[\x81Ra1C` \x83\x01a0\xB2V[` \x82\x01R\x80\x84RP` \x83\x01\x92P`@\x81\x01\x90Pa1\x0BV[___``\x84\x86\x03\x12\x15a1oW__\xFD[a1x\x84a,%V[\x92Pa1\x86` \x85\x01a0\xB2V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xA0W__\xFD[a1\xAC\x86\x82\x87\x01a0\xC8V[\x91PP\x92P\x92P\x92V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,5W__\xFD[___`@\x84\x86\x03\x12\x15a1\xDBW__\xFD[a1\xE4\x84a1\xB6V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xFEW__\xFD[a2\n\x86\x82\x87\x01a-\xA4V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a/7W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a20V[___``\x84\x86\x03\x12\x15a2fW__\xFD[a2o\x84a,%V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[``\x81\x01a\n\xE5\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[__`@\x83\x85\x03\x12\x15a2\xCFW__\xFD[a2\xD8\x83a,%V[\x91Pa.\xC0` \x84\x01a0\xB2V[___`@\x84\x86\x03\x12\x15a2\xF8W__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xFEW__\xFD[__`@\x83\x85\x03\x12\x15a3%W__\xFD[a3.\x83a,%V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3HW__\xFD[a3T\x85\x82\x86\x01a0\xC8V[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a3pW__\xFD[a3y\x84a,%V[\x92Pa3\x87` \x85\x01a1\xB6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[____`\x80\x85\x87\x03\x12\x15a3\xABW__\xFD[a3\xB4\x85a,%V[\x93Pa3\xC2` \x86\x01a0\xB2V[\x92Pa3\xD0`@\x86\x01a1\xB6V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xEAW__\xFD[a3\xF6\x87\x82\x88\x01a0\xC8V[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a4\x14W__\xFD[\x835\x92Pa4$` \x85\x01a,%V[\x91Pa42`@\x85\x01a1\xB6V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a4LW__\xFD[a4U\x83a,%V[\x91Pa.\xC0` \x84\x01a1\xB6V[____`\x80\x85\x87\x03\x12\x15a4vW__\xFD[a4\x7F\x85a,%V[\x93Pa4\x8D` \x86\x01a1\xB6V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a4\xC6W__\xFD[a\x12\xC6\x82a0\xB2V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\xE5Wa\n\xE5a4\xCFV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a5\x1AW__\xFD[\x81Qa\x12\xC6\x81a,\x9DV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\xFF\x83\x16` \x80\x83\x01\x91\x90\x91R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R_\x91\x84\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15a5\x82W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a5[V[P\x90\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a5\x9FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12\xC6W__\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xE5Wa\n\xE5a4\xCFV[_\x82a5\xDFWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\n\xE5Wa\n\xE5a4\xCFV[_\x81T\x80\x84R` \x84\x01\x93P\x82_R` _ _[\x82\x81\x10\x15a.qW\x81T`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x95\x01\x94`\x01\x91\x82\x01\x91\x01a6\x18V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x14\x1E\x90\x83\x01\x84a6\x03V[_\x82`\x1F\x83\x01\x12a6qW__\xFD[\x81Qa6\x7Fa0\"\x82a/\xAEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a6\xA0W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x105W\x80Q\x83R` \x92\x83\x01\x92\x01a6\xA5V[_` \x82\x84\x03\x12\x15a6\xCDW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xE2W__\xFD[a\x14\x1E\x84\x82\x85\x01a6bV[\x80\x82\x01\x80\x82\x11\x15a\n\xE5Wa\n\xE5a4\xCFV[_\x81a7\x0FWa7\x0Fa4\xCFV[P_\x19\x01\x90V[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x83\x01R`\xA0`@\x83\x01\x81\x90R\x85Q\x90\x83\x01\x81\x90R_\x91\x86\x01\x90`\xC0\x84\x01\x90\x83[\x81\x81\x10\x15a7wW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7PV[PP\x83\x81\x03``\x85\x01Ra7\x8B\x81\x87a6\x03V[\x92PPPa#\xE1`\x80\x83\x01\x84c\xFF\xFF\xFF\xFF\x16\x90RV[_` \x82\x84\x03\x12\x15a7\xB1W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xC6W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a7\xD6W__\xFD[\x80Qa7\xE4a0\"\x82a/\xAEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a8\x05W__\xFD[` \x84\x01[\x83\x81\x10\x15a8EW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a8'W__\xFD[a86\x89` \x83\x89\x01\x01a6bV[\x84RP` \x92\x83\x01\x92\x01a8\nV[P\x96\x95PPPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x07!Wa\x07!a4\xCFV[_`\x01`\xFF\x1B\x82\x01a8\x83Wa8\x83a4\xCFV[P_\x03\x90V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\n\xE5Wa\n\xE5a4\xCFV\xFE\xA2dipfsX\"\x12 \xC2\xC1\x1E\xA3\xFB\xAB\xF1\xEAWd\xF2y\xBE\xC4\x84\xB3S\xA2K\x04\x90\0\x16\x01\x05\xE4\x149:P\xF6\"dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610229575f3560e01c80639f3ccf651161012a578063c8294c56116100b4578063df5cf72311610079578063df5cf7231461064e578063e086adb314610675578063f2be94ae14610688578063f851e1981461069b578063fa28c627146106ae575f5ffd5b8063c8294c56146105db578063ca8aa7c7146105ee578063cc5a7c2014610615578063d5eccc0514610628578063dd9846b91461063b575f5ffd5b8063b6904b78116100fa578063b6904b7814610567578063bc9a40c31461057a578063bd29b8cd1461058d578063c46778a5146105a0578063c601527d146105c8575f5ffd5b80639f3ccf65146104d15780639f8aff26146104e4578063ac6bfb0314610507578063adc804da14610527575f5ffd5b80635e5a6775116101b65780636d14a9871161017b5780636d14a9871461042357806375d4173a1461044a5780637c1723471461045d57806381c07502146104775780639ab4d6ff14610497575f5ffd5b80635e5a6775146103685780635f1f2d771461037757806366acfefe1461038a578063697fbd93146103b55780636b3aa72e146103e4575f5ffd5b806325504777116101fc57806325504777146102c35780632cd95940146102e45780633ca5a5f5146103045780634bd26e09146103265780635401ed2714610355575f5ffd5b80630491b41c1461022d57806308732461146102625780631f9b74e01461028357806320b66298146102ae575b5f5ffd5b61024f61023b366004612c3a565b60ff165f9081526001602052604090205490565b6040519081526020015b60405180910390f35b610275610270366004612c53565b6106c1565b604051610259929190612c7b565b610296610291366004612cb1565b610706565b6040516001600160601b039091168152602001610259565b6102c16102bc366004612d26565b610728565b005b6102d66102d1366004612de1565b6108c6565b604051610259929190612e7b565b6102f76102f2366004612e9f565b610a4e565b6040516102599190612ec9565b61024f610312366004612c3a565b60ff165f9081526003602052604090205490565b61024f610334366004612e9f565b5f91825260026020908152604080842060ff93909316845291905290205490565b610296610363366004612e9f565b610aeb565b61024f670de0b6b3a764000081565b6102c1610385366004612fd0565b610b03565b61039d610398366004612de1565b610fa3565b6040516001600160c01b039091168152602001610259565b6103d76103c3366004612c3a565b60056020525f908152604090205460ff1681565b604051610259919061308c565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610259565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6102c161045836600461315d565b61103f565b610465602081565b60405160ff9091168152602001610259565b61048a6104853660046131c9565b611125565b6040516102599190613217565b6104bc6104a5366004612c3a565b60066020525f908152604090205463ffffffff1681565b60405163ffffffff9091168152602001610259565b61040b6104df366004612c53565b6112cd565b6104f76104f2366004612c3a565b611301565b6040519015158152602001610259565b61051a610515366004613254565b611426565b6040516102599190613284565b61053a610535366004612c53565b6114bc565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610259565b61051a610575366004612c53565b611533565b6102c16105883660046132be565b6115c0565b6102c161059b3660046132e6565b6115e1565b6102966105ae366004612c3a565b5f602081905290815260409020546001600160601b031681565b6102c16105d6366004613314565b611644565b6102966105e936600461335e565b611807565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6102c1610623366004613398565b611883565b610296610636366004612c3a565b611975565b6104bc610649366004613402565b6119c6565b61040b7f000000000000000000000000000000000000000000000000000000000000000081565b6102c161068336600461343b565b6119d2565b610296610696366004613463565b6119ee565b61051a6106a9366004612e9f565b611a81565b6102966106bc366004613402565b611b67565b6003602052815f5260405f2081815481106106da575f80fd5b5f918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b5f8261071181611bc6565b5f61071c8585611bf8565b509250505b5092915050565b610730611f04565b8461073a81611bc6565b83806107595760405163796cc52560e01b815260040160405180910390fd5b828114610779576040516343714afd60e01b815260040160405180910390fd5b60ff87165f908152600360205260408120905b828110156108bb578585828181106107a6576107a66134a2565b90506020020160208101906107bb91906134b6565b828989848181106107ce576107ce6134a2565b90506020020135815481106107e5576107e56134a2565b905f5260205f20015f0160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a8581811061084b5761084b6134a2565b9050602002013581548110610862576108626134a2565b5f918252602090912001546001600160a01b0316888885818110610888576108886134a2565b905060200201602081019061089d91906134b6565b6040516108ab929190612c7b565b60405180910390a260010161078c565b505050505050505050565b6060806108d1611fb7565b5f836001600160401b038111156108ea576108ea612f42565b604051908082528060200260200182016040528015610913578160200160208202803683370190505b5090505f846001600160401b0381111561092f5761092f612f42565b604051908082528060200260200182016040528015610958578160200160208202803683370190505b5090505f5b85811015610a40575f878783818110610978576109786134a2565b919091013560f81c915061098d905081611bc6565b5f5f610999838d611bf8565b91509150806109bb5760405163207f13e360e11b815260040160405180910390fd5b5f6109c78c8585612000565b9050828786815181106109dc576109dc6134a2565b60200260200101906001600160601b031690816001600160601b031681525050610a068482612279565b868681518110610a1857610a186134a2565b6001600160601b039092166020928302919091019091015250506001909201915061095d9050565b509097909650945050505050565b5f82815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610ade575f848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610a86565b5050505090505b92915050565b5f5f610af78484611a81565b60400151949350505050565b610b0b611f04565b81610b1581611bc6565b815180610b355760405163796cc52560e01b815260040160405180910390fd5b60ff84165f908152600360209081526040808320600490925282209091836001600160401b03811115610b6a57610b6a612f42565b604051908082528060200260200182016040528015610b93578160200160208202803683370190505b5090505f5b84811015610e8e5783878281518110610bb357610bb36134a2565b602002602001015181548110610bcb57610bcb6134a2565b5f9182526020909120015482516001600160a01b0390911690839083908110610bf657610bf66134a2565b60200260200101906001600160a01b031690816001600160a01b0316815250508760ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f785898481518110610c4e57610c4e6134a2565b602002602001015181548110610c6657610c666134a2565b5f91825260209182902001546040516001600160a01b0390911681520160405180910390a28760ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7585898481518110610cc357610cc36134a2565b602002602001015181548110610cdb57610cdb6134a2565b5f91825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a283548490610d1a906001906134e3565b81548110610d2a57610d2a6134a2565b905f5260205f200184888381518110610d4557610d456134a2565b602002602001015181548110610d5d57610d5d6134a2565b5f91825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558354849080610daf57610daf6134f6565b5f8281526020812082015f199081019190915501905582548390610dd5906001906134e3565b81548110610de557610de56134a2565b905f5260205f20015f9054906101000a90046001600160a01b031683888381518110610e1357610e136134a2565b602002602001015181548110610e2b57610e2b6134a2565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555082805480610e6657610e666134f6565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610b98565b50610e9887611301565b15610f9a577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b66bd9897f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316630764cb936040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f28573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f4c919061350a565b89846040518463ffffffff1660e01b8152600401610f6c93929190613525565b5f604051808303815f87803b158015610f83575f5ffd5b505af1158015610f95573d5f5f3e3d5ffd5b505050505b50505050505050565b5f610fac611fb7565b5f805b83811015611035575f858583818110610fca57610fca6134a2565b919091013560f81c9150610fdf905081611bc6565b5f5f610feb838b611bf8565b915091508061100c575f9150600160ff84161b6001600160c01b0386161794505b5f6110188a8585612000565b90506110248482612279565b505060019093019250610faf915050565b5095945050505050565b611047611fb7565b60ff83165f9081526001602052604090205415611077576040516310cda51760e21b815260040160405180910390fd5b61108183826123ea565b61108b83836126e2565b611095835f61274a565b505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60605f826001600160401b0381111561114057611140612f42565b604051908082528060200260200182016040528015611169578160200160208202803683370190505b5090505f5b838110156112c2575f858583818110611189576111896134a2565b919091013560f81c915061119e905081611bc6565b60ff81165f908152600160205260408120805463ffffffff8a1692906111c6576111c66134a2565b5f9182526020909120015463ffffffff1611156111f65760405163cc64657360e01b815260040160405180910390fd5b60ff81165f90815260016020526040812054905b818110156112b75760ff83165f90815260016020819052604090912063ffffffff8b169161123884866134e3565b61124291906134e3565b81548110611252576112526134a2565b5f9182526020909120015463ffffffff16116112af57600161127482846134e3565b61127e91906134e3565b858581518110611290576112906134a2565b602002602001019063ffffffff16908163ffffffff16815250506112b7565b60010161120a565b50505060010161116e565b5090505b9392505050565b6004602052815f5260405f2081815481106112e6575f80fd5b5f918252602090912001546001600160a01b03169150829050565b60405163a4d7871f60e01b815260ff821660048201525f9081906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a4d7871f90602401602060405180830381865afa15801561136a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061138e919061358f565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166381f936d26040518163ffffffff1660e01b8152600401602060405180830381865afa1580156113ed573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611411919061358f565b905080801561141e575081155b949350505050565b604080516060810182525f80825260208083018290528284018290528582526002815283822060ff8816835290529190912080548390811061146a5761146a6134a2565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091525f808252602082015260ff83165f9081526003602052604090208054839081106114f2576114f26134a2565b5f918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182525f808252602080830182905282840182905260ff86168252600190529190912080548390811061156f5761156f6134a2565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b6115c8611f04565b816115d281611bc6565b6115dc83836126e2565b505050565b6115e9611fb7565b5f5b8181101561163e575f838383818110611606576116066134a2565b919091013560f81c915061161b905081611bc6565b5f61162786835f612000565b90506116338282612279565b5050506001016115eb565b50505050565b61164c611f04565b8161165681611bc6565b61166083836123ea565b815161166b84611301565b1561163e575f816001600160401b0381111561168957611689612f42565b6040519080825280602002602001820160405280156116b2578160200160208202803683370190505b5090505f5b8281101561170e578481815181106116d1576116d16134a2565b60200260200101515f01518282815181106116ee576116ee6134a2565b6001600160a01b03909216602092830291909101909101526001016116b7565b507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166350feea207f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316630764cb936040518163ffffffff1660e01b8152600401602060405180830381865afa15801561179a573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117be919061350a565b87846040518463ffffffff1660e01b81526004016117de93929190613525565b5f604051808303815f87803b1580156117f5575f5ffd5b505af11580156108bb573d5f5f3e3d5ffd5b60ff83165f90815260016020526040812080548291908490811061182d5761182d6134a2565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610af781856127b7565b61188b611fb7565b60ff84165f90815260016020526040902054156118bb576040516310cda51760e21b815260040160405180910390fd5b6118c584826123ea565b6118cf84846126e2565b6118da84600161274a565b6118e4848361282e565b50505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff81165f908152600160208190526040822080549091611995916134e3565b815481106119a5576119a56134a2565b5f91825260209091200154600160401b90046001600160601b031692915050565b5f61141e84848461289a565b6119da611f04565b816119e481611bc6565b6115dc838361282e565b5f82815260026020908152604080832060ff881684529091528120805482919084908110611a1e57611a1e6134a2565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611a7481866127b7565b6040015195945050505050565b60408051606080820183525f80835260208084018290528385018290528682526002815284822060ff8716835281528482205485519384018652828452908301829052938201819052919291829003611add579150610ae59050565b5f85815260026020908152604080832060ff881684529091529020611b036001846134e3565b81548110611b1357611b136134a2565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610ae5915050565b5f83815260026020908152604080832060ff861684529091528120611b8d85858561289a565b63ffffffff1681548110611ba357611ba36134a2565b5f91825260209091200154600160401b90046001600160601b0316949350505050565b60ff81165f90815260016020526040902054611bf557604051637310cff560e11b815260040160405180910390fd5b50565b5f5f5f5f611c148660ff165f9081526003602052604090205490565b604080518082019091525f80825260208201529091506060600160ff808a165f90815260056020526040902054166001811115611c5357611c53613078565b03611d4c57611c628888612a01565b90505f5b83811015611d465760ff89165f908152600360205260409020805482908110611c9157611c916134a2565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611cde57611cde6134a2565b60200260200101511115611d3e57670de0b6b3a764000083602001516001600160601b0316838381518110611d1557611d156134a2565b6020026020010151611d2791906135ae565b611d3191906135c5565b611d3b90866135e4565b94505b600101611c66565b50611ed2565b60ff88165f908152600460208190526040918290209151639004134760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001692639004134792611dab928c92910161363f565b5f60405180830381865afa158015611dc5573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611dec91908101906136bd565b90505f5b83811015611ed05760ff89165f908152600360205260409020805482908110611e1b57611e1b6134a2565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611e6857611e686134a2565b60200260200101511115611ec857670de0b6b3a764000083602001516001600160601b0316838381518110611e9f57611e9f6134a2565b6020026020010151611eb191906135ae565b611ebb91906135c5565b611ec590866135e4565b94505b600101611df0565b505b50505060ff85165f908152602081905260409020549092506001600160601b03908116908316101590505b9250929050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f60573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f84919061350a565b6001600160a01b0316336001600160a01b031614611fb55760405163ce98c24b60e01b815260040160405180910390fd5b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611fb557604051632c01b20560e21b815260040160405180910390fd5b5f83815260026020908152604080832060ff8616845290915281205481908082036120c4575f86815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561221f565b5f86815260026020908152604080832060ff8916845290915281206120ea6001846134e3565b815481106120fa576120fa6134a2565b5f91825260209091200180546001600160601b03600160401b9091048116945090915085168303612130575f93505050506112c6565b805463ffffffff438116911603612168578054600160401b600160a01b031916600160401b6001600160601b0387160217815561221d565b805467ffffffff000000001916600160201b4363ffffffff9081168281029390931784555f8a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a261226f8285612be1565b9695505050505050565b60ff82165f908152600160208190526040822080549183919061229c90846134e3565b815481106122ac576122ac6134a2565b905f5260205f20019050835f036122d75754600160401b90046001600160601b03169150610ae59050565b80545f906122f590600160401b90046001600160601b031686612bf8565b825490915063ffffffff438116911603612330578154600160401b600160a01b031916600160401b6001600160601b038316021782556123e1565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff89165f90815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b5f81511161240b5760405163796cc52560e01b815260040160405180910390fd5b805160ff83165f908152600360209081526040909120549061242d83836136ee565b111561244c576040516343714afd60e01b815260040160405180910390fd5b5f5b828110156126db575f5b61246282846136ee565b8110156124f15784828151811061247b5761247b6134a2565b60200260200101515f01516001600160a01b031660035f8860ff1660ff1681526020019081526020015f2082815481106124b7576124b76134a2565b5f918252602090912001546001600160a01b0316036124e957604051637b74340b60e01b815260040160405180910390fd5b600101612458565b505f848281518110612505576125056134a2565b6020026020010151602001516001600160601b03161161253857604051637257125160e01b815260040160405180910390fd5b60ff85165f908152600360205260409020845185908390811061255d5761255d6134a2565b60209081029190910181015182546001810184555f9384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106125c1576125c16134a2565b6020908102919091018101515182546001810184555f938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540490869084908110612637576126376134a2565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75858381518110612694576126946134a2565b60200260200101515f01518684815181106126b1576126b16134a2565b6020026020010151602001516040516126cb929190612c7b565b60405180910390a260010161244e565b5050505050565b60ff82165f818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60ff82165f908152600560205260409020805482919060ff19166001838181111561277757612777613078565b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516127ab919061308c565b60405180910390a15050565b815f015163ffffffff168163ffffffff1610156127e757604051631391e11b60e21b815260040160405180910390fd5b602082015163ffffffff16158061280d5750816020015163ffffffff168163ffffffff16105b61282a57604051631391e11b60e21b815260040160405180910390fd5b5050565b60ff82165f90815260066020908152604091829020805463ffffffff19811663ffffffff8681169182179093558451929091168083529282015290917f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7910160405180910390a1505050565b5f83815260026020908152604080832060ff86168452909152812054805b8015612938575f86815260026020908152604080832060ff89168452909152902063ffffffff8516906128ec6001846134e3565b815481106128fc576128fc6134a2565b5f9182526020909120015463ffffffff16116129265761291d6001826134e3565b925050506112c6565b8061293081613701565b9150506128b8565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e40160405180910390fd5b6040805160018082528183019092526060915f91906020808301908036833701905050905082815f81518110612a3957612a396134a2565b6001600160a01b0390921660209283029190910182015260ff85165f90815260069091526040812054612a729063ffffffff16436136ee565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632bab2c4a60405180604001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316630764cb936040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b0b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b2f919061350a565b6001600160a01b0316815260ff8a1660209182018190525f90815260049182905260409081902090516001600160e01b031960e086901b168152612b799392899291899101613716565b5f60405180830381865afa158015612b93573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612bba91908101906137a1565b9050805f81518110612bce57612bce6134a2565b6020026020010151935050505092915050565b5f6112c66001600160601b03808516908416613850565b5f5f821215612c1b57612c0a8261386f565b612c149084613889565b9050610ae5565b612c1482846135e4565b803560ff81168114612c35575f5ffd5b919050565b5f60208284031215612c4a575f5ffd5b6112c682612c25565b5f5f60408385031215612c64575f5ffd5b612c6d83612c25565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114611bf5575f5ffd5b5f5f60408385031215612cc2575f5ffd5b612ccb83612c25565b91506020830135612cdb81612c9d565b809150509250929050565b5f5f83601f840112612cf6575f5ffd5b5081356001600160401b03811115612d0c575f5ffd5b6020830191508360208260051b8501011115611efd575f5ffd5b5f5f5f5f5f60608688031215612d3a575f5ffd5b612d4386612c25565b945060208601356001600160401b03811115612d5d575f5ffd5b612d6988828901612ce6565b90955093505060408601356001600160401b03811115612d87575f5ffd5b612d9388828901612ce6565b969995985093965092949392505050565b5f5f83601f840112612db4575f5ffd5b5081356001600160401b03811115612dca575f5ffd5b602083019150836020828501011115611efd575f5ffd5b5f5f5f5f60608587031215612df4575f5ffd5b8435612dff81612c9d565b93506020850135925060408501356001600160401b03811115612e20575f5ffd5b612e2c87828801612da4565b95989497509550505050565b5f8151808452602084019350602083015f5b82811015612e715781516001600160601b0316865260209586019590910190600101612e4a565b5093949350505050565b604081525f612e8d6040830185612e38565b82810360208401526123e18185612e38565b5f5f60408385031215612eb0575f5ffd5b82359150612ec060208401612c25565b90509250929050565b602080825282518282018190525f918401906040840190835b81811015612f3757612f2183855163ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b6020939093019260609290920191600101612ee2565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715612f7857612f78612f42565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612fa657612fa6612f42565b604052919050565b5f6001600160401b03821115612fc657612fc6612f42565b5060051b60200190565b5f5f60408385031215612fe1575f5ffd5b612fea83612c25565b915060208301356001600160401b03811115613004575f5ffd5b8301601f81018513613014575f5ffd5b803561302761302282612fae565b612f7e565b8082825260208201915060208360051b850101925087831115613048575f5ffd5b6020840193505b8284101561306a57833582526020938401939091019061304f565b809450505050509250929050565b634e487b7160e01b5f52602160045260245ffd5b60208101600283106130ac57634e487b7160e01b5f52602160045260245ffd5b91905290565b80356001600160601b0381168114612c35575f5ffd5b5f82601f8301126130d7575f5ffd5b81356130e561302282612fae565b8082825260208201915060208360061b860101925085831115613106575f5ffd5b602085015b838110156110355760408188031215613122575f5ffd5b61312a612f56565b813561313581612c9d565b8152613143602083016130b2565b60208201528084525060208301925060408101905061310b565b5f5f5f6060848603121561316f575f5ffd5b61317884612c25565b9250613186602085016130b2565b915060408401356001600160401b038111156131a0575f5ffd5b6131ac868287016130c8565b9150509250925092565b803563ffffffff81168114612c35575f5ffd5b5f5f5f604084860312156131db575f5ffd5b6131e4846131b6565b925060208401356001600160401b038111156131fe575f5ffd5b61320a86828701612da4565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b81811015612f3757835163ffffffff16835260209384019390920191600101613230565b5f5f5f60608486031215613266575f5ffd5b61326f84612c25565b95602085013595506040909401359392505050565b60608101610ae5828463ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b5f5f604083850312156132cf575f5ffd5b6132d883612c25565b9150612ec0602084016130b2565b5f5f5f604084860312156132f8575f5ffd5b8335925060208401356001600160401b038111156131fe575f5ffd5b5f5f60408385031215613325575f5ffd5b61332e83612c25565b915060208301356001600160401b03811115613348575f5ffd5b613354858286016130c8565b9150509250929050565b5f5f5f60608486031215613370575f5ffd5b61337984612c25565b9250613387602085016131b6565b929592945050506040919091013590565b5f5f5f5f608085870312156133ab575f5ffd5b6133b485612c25565b93506133c2602086016130b2565b92506133d0604086016131b6565b915060608501356001600160401b038111156133ea575f5ffd5b6133f6878288016130c8565b91505092959194509250565b5f5f5f60608486031215613414575f5ffd5b8335925061342460208501612c25565b9150613432604085016131b6565b90509250925092565b5f5f6040838503121561344c575f5ffd5b61345583612c25565b9150612ec0602084016131b6565b5f5f5f5f60808587031215613476575f5ffd5b61347f85612c25565b935061348d602086016131b6565b93969395505050506040820135916060013590565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156134c6575f5ffd5b6112c6826130b2565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610ae557610ae56134cf565b634e487b7160e01b5f52603160045260245ffd5b5f6020828403121561351a575f5ffd5b81516112c681612c9d565b6001600160a01b038416815260ff831660208083019190915260606040830181905283519083018190525f918401906080840190835b818110156135825783516001600160a01b031683526020938401939092019160010161355b565b5090979650505050505050565b5f6020828403121561359f575f5ffd5b815180151581146112c6575f5ffd5b8082028115828204841417610ae557610ae56134cf565b5f826135df57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160601b038181168382160190811115610ae557610ae56134cf565b5f8154808452602084019350825f5260205f205f5b82811015612e715781546001600160a01b0316865260209095019460019182019101613618565b6001600160a01b03831681526040602082018190525f9061141e90830184613603565b5f82601f830112613671575f5ffd5b815161367f61302282612fae565b8082825260208201915060208360051b8601019250858311156136a0575f5ffd5b602085015b838110156110355780518352602092830192016136a5565b5f602082840312156136cd575f5ffd5b81516001600160401b038111156136e2575f5ffd5b61141e84828501613662565b80820180821115610ae557610ae56134cf565b5f8161370f5761370f6134cf565b505f190190565b84516001600160a01b0316815260208086015163ffffffff168183015260a06040830181905285519083018190525f9186019060c0840190835b818110156137775783516001600160a01b0316835260209384019390920191600101613750565b5050838103606085015261378b8187613603565b925050506123e1608083018463ffffffff169052565b5f602082840312156137b1575f5ffd5b81516001600160401b038111156137c6575f5ffd5b8201601f810184136137d6575f5ffd5b80516137e461302282612fae565b8082825260208201915060208360051b850101925086831115613805575f5ffd5b602084015b838110156138455780516001600160401b03811115613827575f5ffd5b61383689602083890101613662565b8452506020928301920161380a565b509695505050505050565b8181035f831280158383131683831282161715610721576107216134cf565b5f600160ff1b8201613883576138836134cf565b505f0390565b6001600160601b038281168282160390811115610ae557610ae56134cf56fea2646970667358221220c2c11ea3fbabf1ea5764f279bec484b353a24b049000160105e414393a50f62264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02)W_5`\xE0\x1C\x80c\x9F<\xCFe\x11a\x01*W\x80c\xC8)LV\x11a\0\xB4W\x80c\xDF\\\xF7#\x11a\0yW\x80c\xDF\\\xF7#\x14a\x06NW\x80c\xE0\x86\xAD\xB3\x14a\x06uW\x80c\xF2\xBE\x94\xAE\x14a\x06\x88W\x80c\xF8Q\xE1\x98\x14a\x06\x9BW\x80c\xFA(\xC6'\x14a\x06\xAEW__\xFD[\x80c\xC8)LV\x14a\x05\xDBW\x80c\xCA\x8A\xA7\xC7\x14a\x05\xEEW\x80c\xCCZ| \x14a\x06\x15W\x80c\xD5\xEC\xCC\x05\x14a\x06(W\x80c\xDD\x98F\xB9\x14a\x06;W__\xFD[\x80c\xB6\x90Kx\x11a\0\xFAW\x80c\xB6\x90Kx\x14a\x05gW\x80c\xBC\x9A@\xC3\x14a\x05zW\x80c\xBD)\xB8\xCD\x14a\x05\x8DW\x80c\xC4gx\xA5\x14a\x05\xA0W\x80c\xC6\x01R}\x14a\x05\xC8W__\xFD[\x80c\x9F<\xCFe\x14a\x04\xD1W\x80c\x9F\x8A\xFF&\x14a\x04\xE4W\x80c\xACk\xFB\x03\x14a\x05\x07W\x80c\xAD\xC8\x04\xDA\x14a\x05'W__\xFD[\x80c^Zgu\x11a\x01\xB6W\x80cm\x14\xA9\x87\x11a\x01{W\x80cm\x14\xA9\x87\x14a\x04#W\x80cu\xD4\x17:\x14a\x04JW\x80c|\x17#G\x14a\x04]W\x80c\x81\xC0u\x02\x14a\x04wW\x80c\x9A\xB4\xD6\xFF\x14a\x04\x97W__\xFD[\x80c^Zgu\x14a\x03hW\x80c_\x1F-w\x14a\x03wW\x80cf\xAC\xFE\xFE\x14a\x03\x8AW\x80ci\x7F\xBD\x93\x14a\x03\xB5W\x80ck:\xA7.\x14a\x03\xE4W__\xFD[\x80c%PGw\x11a\x01\xFCW\x80c%PGw\x14a\x02\xC3W\x80c,\xD9Y@\x14a\x02\xE4W\x80c<\xA5\xA5\xF5\x14a\x03\x04W\x80cK\xD2n\t\x14a\x03&W\x80cT\x01\xED'\x14a\x03UW__\xFD[\x80c\x04\x91\xB4\x1C\x14a\x02-W\x80c\x08s$a\x14a\x02bW\x80c\x1F\x9Bt\xE0\x14a\x02\x83W\x80c \xB6b\x98\x14a\x02\xAEW[__\xFD[a\x02Oa\x02;6`\x04a,:V[`\xFF\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ua\x02p6`\x04a,SV[a\x06\xC1V[`@Qa\x02Y\x92\x91\x90a,{V[a\x02\x96a\x02\x916`\x04a,\xB1V[a\x07\x06V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02YV[a\x02\xC1a\x02\xBC6`\x04a-&V[a\x07(V[\0[a\x02\xD6a\x02\xD16`\x04a-\xE1V[a\x08\xC6V[`@Qa\x02Y\x92\x91\x90a.{V[a\x02\xF7a\x02\xF26`\x04a.\x9FV[a\nNV[`@Qa\x02Y\x91\x90a.\xC9V[a\x02Oa\x03\x126`\x04a,:V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Oa\x0346`\x04a.\x9FV[_\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x96a\x03c6`\x04a.\x9FV[a\n\xEBV[a\x02Og\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xC1a\x03\x856`\x04a/\xD0V[a\x0B\x03V[a\x03\x9Da\x03\x986`\x04a-\xE1V[a\x0F\xA3V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02YV[a\x03\xD7a\x03\xC36`\x04a,:V[`\x05` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02Y\x91\x90a0\x8CV[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02YV[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x04X6`\x04a1]V[a\x10?V[a\x04e` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02YV[a\x04\x8Aa\x04\x856`\x04a1\xC9V[a\x11%V[`@Qa\x02Y\x91\x90a2\x17V[a\x04\xBCa\x04\xA56`\x04a,:V[`\x06` R_\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02YV[a\x04\x0Ba\x04\xDF6`\x04a,SV[a\x12\xCDV[a\x04\xF7a\x04\xF26`\x04a,:V[a\x13\x01V[`@Q\x90\x15\x15\x81R` \x01a\x02YV[a\x05\x1Aa\x05\x156`\x04a2TV[a\x14&V[`@Qa\x02Y\x91\x90a2\x84V[a\x05:a\x0556`\x04a,SV[a\x14\xBCV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02YV[a\x05\x1Aa\x05u6`\x04a,SV[a\x153V[a\x02\xC1a\x05\x886`\x04a2\xBEV[a\x15\xC0V[a\x02\xC1a\x05\x9B6`\x04a2\xE6V[a\x15\xE1V[a\x02\x96a\x05\xAE6`\x04a,:V[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\xC1a\x05\xD66`\x04a3\x14V[a\x16DV[a\x02\x96a\x05\xE96`\x04a3^V[a\x18\x07V[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x06#6`\x04a3\x98V[a\x18\x83V[a\x02\x96a\x0666`\x04a,:V[a\x19uV[a\x04\xBCa\x06I6`\x04a4\x02V[a\x19\xC6V[a\x04\x0B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x06\x836`\x04a4;V[a\x19\xD2V[a\x02\x96a\x06\x966`\x04a4cV[a\x19\xEEV[a\x05\x1Aa\x06\xA96`\x04a.\x9FV[a\x1A\x81V[a\x02\x96a\x06\xBC6`\x04a4\x02V[a\x1BgV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x06\xDAW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[_\x82a\x07\x11\x81a\x1B\xC6V[_a\x07\x1C\x85\x85a\x1B\xF8V[P\x92PP[P\x92\x91PPV[a\x070a\x1F\x04V[\x84a\x07:\x81a\x1B\xC6V[\x83\x80a\x07YW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x07yW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x87\x16_\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\x08\xBBW\x85\x85\x82\x81\x81\x10a\x07\xA6Wa\x07\xA6a4\xA2V[\x90P` \x02\x01` \x81\x01\x90a\x07\xBB\x91\x90a4\xB6V[\x82\x89\x89\x84\x81\x81\x10a\x07\xCEWa\x07\xCEa4\xA2V[\x90P` \x02\x015\x81T\x81\x10a\x07\xE5Wa\x07\xE5a4\xA2V[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08KWa\x08Ka4\xA2V[\x90P` \x02\x015\x81T\x81\x10a\x08bWa\x08ba4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\x08\x88Wa\x08\x88a4\xA2V[\x90P` \x02\x01` \x81\x01\x90a\x08\x9D\x91\x90a4\xB6V[`@Qa\x08\xAB\x92\x91\x90a,{V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x07\x8CV[PPPPPPPPPV[``\x80a\x08\xD1a\x1F\xB7V[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xEAWa\x08\xEAa/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x13W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\t/Wa\t/a/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tXW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a\n@W_\x87\x87\x83\x81\x81\x10a\txWa\txa4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\t\x8D\x90P\x81a\x1B\xC6V[__a\t\x99\x83\x8Da\x1B\xF8V[\x91P\x91P\x80a\t\xBBW`@Qc \x7F\x13\xE3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\xC7\x8C\x85\x85a \0V[\x90P\x82\x87\x86\x81Q\x81\x10a\t\xDCWa\t\xDCa4\xA2V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\n\x06\x84\x82a\"yV[\x86\x86\x81Q\x81\x10a\n\x18Wa\n\x18a4\xA2V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x90\x92\x01\x91Pa\t]\x90PV[P\x90\x97\x90\x96P\x94PPPPPV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\n\xDEW_\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\n\x86V[PPPP\x90P[\x92\x91PPV[__a\n\xF7\x84\x84a\x1A\x81V[`@\x01Q\x94\x93PPPPV[a\x0B\x0Ba\x1F\x04V[\x81a\x0B\x15\x81a\x1B\xC6V[\x81Q\x80a\x0B5W`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x84\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BjWa\x0Bja/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x93W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x0E\x8EW\x83\x87\x82\x81Q\x81\x10a\x0B\xB3Wa\x0B\xB3a4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0B\xCBWa\x0B\xCBa4\xA2V[_\x91\x82R` \x90\x91 \x01T\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a\x0B\xF6Wa\x0B\xF6a4\xA2V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x87`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x85\x89\x84\x81Q\x81\x10a\x0CNWa\x0CNa4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0CfWa\x0Cfa4\xA2V[_\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x87`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x89\x84\x81Q\x81\x10a\x0C\xC3Wa\x0C\xC3a4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0C\xDBWa\x0C\xDBa4\xA2V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x83T\x84\x90a\r\x1A\x90`\x01\x90a4\xE3V[\x81T\x81\x10a\r*Wa\r*a4\xA2V[\x90_R` _ \x01\x84\x88\x83\x81Q\x81\x10a\rEWa\rEa4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\r]Wa\r]a4\xA2V[_\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x83T\x84\x90\x80a\r\xAFWa\r\xAFa4\xF6V[_\x82\x81R` \x81 \x82\x01_\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x82T\x83\x90a\r\xD5\x90`\x01\x90a4\xE3V[\x81T\x81\x10a\r\xE5Wa\r\xE5a4\xA2V[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x83\x81Q\x81\x10a\x0E\x13Wa\x0E\x13a4\xA2V[` \x02` \x01\x01Q\x81T\x81\x10a\x0E+Wa\x0E+a4\xA2V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x82\x80T\x80a\x0EfWa\x0Efa4\xF6V[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\x0B\x98V[Pa\x0E\x98\x87a\x13\x01V[\x15a\x0F\x9AW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB6k\xD9\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x07d\xCB\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F(W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FL\x91\x90a5\nV[\x89\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Fl\x93\x92\x91\x90a5%V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\x83W__\xFD[PZ\xF1\x15\x80\x15a\x0F\x95W=__>=_\xFD[PPPP[PPPPPPPV[_a\x0F\xACa\x1F\xB7V[_\x80[\x83\x81\x10\x15a\x105W_\x85\x85\x83\x81\x81\x10a\x0F\xCAWa\x0F\xCAa4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x0F\xDF\x90P\x81a\x1B\xC6V[__a\x0F\xEB\x83\x8Ba\x1B\xF8V[\x91P\x91P\x80a\x10\x0CW_\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[_a\x10\x18\x8A\x85\x85a \0V[\x90Pa\x10$\x84\x82a\"yV[PP`\x01\x90\x93\x01\x92Pa\x0F\xAF\x91PPV[P\x95\x94PPPPPV[a\x10Ga\x1F\xB7V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x10wW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\x81\x83\x82a#\xEAV[a\x10\x8B\x83\x83a&\xE2V[a\x10\x95\x83_a'JV[PP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11@Wa\x11@a/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11iW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x12\xC2W_\x85\x85\x83\x81\x81\x10a\x11\x89Wa\x11\x89a4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x11\x9E\x90P\x81a\x1B\xC6V[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x11\xC6Wa\x11\xC6a4\xA2V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x11\xF6W`@Qc\xCCdes`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x12\xB7W`\xFF\x83\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x128\x84\x86a4\xE3V[a\x12B\x91\x90a4\xE3V[\x81T\x81\x10a\x12RWa\x12Ra4\xA2V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x12\xAFW`\x01a\x12t\x82\x84a4\xE3V[a\x12~\x91\x90a4\xE3V[\x85\x85\x81Q\x81\x10a\x12\x90Wa\x12\x90a4\xA2V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x12\xB7V[`\x01\x01a\x12\nV[PPP`\x01\x01a\x11nV[P\x90P[\x93\x92PPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x12\xE6W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@Qc\xA4\xD7\x87\x1F`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA4\xD7\x87\x1F\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13jW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x8E\x91\x90a5\x8FV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x81\xF96\xD2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x11\x91\x90a5\x8FV[\x90P\x80\x80\x15a\x14\x1EWP\x81\x15[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14jWa\x14ja4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x14\xF2Wa\x14\xF2a4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15oWa\x15oa4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x15\xC8a\x1F\x04V[\x81a\x15\xD2\x81a\x1B\xC6V[a\x15\xDC\x83\x83a&\xE2V[PPPV[a\x15\xE9a\x1F\xB7V[_[\x81\x81\x10\x15a\x16>W_\x83\x83\x83\x81\x81\x10a\x16\x06Wa\x16\x06a4\xA2V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x16\x1B\x90P\x81a\x1B\xC6V[_a\x16'\x86\x83_a \0V[\x90Pa\x163\x82\x82a\"yV[PPP`\x01\x01a\x15\xEBV[PPPPV[a\x16La\x1F\x04V[\x81a\x16V\x81a\x1B\xC6V[a\x16`\x83\x83a#\xEAV[\x81Qa\x16k\x84a\x13\x01V[\x15a\x16>W_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\x89Wa\x16\x89a/BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xB2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x17\x0EW\x84\x81\x81Q\x81\x10a\x16\xD1Wa\x16\xD1a4\xA2V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x16\xEEWa\x16\xEEa4\xA2V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x16\xB7V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cP\xFE\xEA \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x07d\xCB\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x9AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xBE\x91\x90a5\nV[\x87\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xDE\x93\x92\x91\x90a5%V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xF5W__\xFD[PZ\xF1\x15\x80\x15a\x08\xBBW=__>=_\xFD[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x18-Wa\x18-a4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\n\xF7\x81\x85a'\xB7V[a\x18\x8Ba\x1F\xB7V[`\xFF\x84\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x18\xBBW`@Qc\x10\xCD\xA5\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xC5\x84\x82a#\xEAV[a\x18\xCF\x84\x84a&\xE2V[a\x18\xDA\x84`\x01a'JV[a\x18\xE4\x84\x83a(.V[PPP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x81\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x19\x95\x91a4\xE3V[\x81T\x81\x10a\x19\xA5Wa\x19\xA5a4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[_a\x14\x1E\x84\x84\x84a(\x9AV[a\x19\xDAa\x1F\x04V[\x81a\x19\xE4\x81a\x1B\xC6V[a\x15\xDC\x83\x83a(.V[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\x1EWa\x1A\x1Ea4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1At\x81\x86a'\xB7V[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x91\x92\x91\x82\x90\x03a\x1A\xDDW\x91Pa\n\xE5\x90PV[_\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1B\x03`\x01\x84a4\xE3V[\x81T\x81\x10a\x1B\x13Wa\x1B\x13a4\xA2V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\n\xE5\x91PPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1B\x8D\x85\x85\x85a(\x9AV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1B\xA3Wa\x1B\xA3a4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x90 Ta\x1B\xF5W`@Qcs\x10\xCF\xF5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[____a\x1C\x14\x86`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90\x91P```\x01`\xFF\x80\x8A\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a\x1CSWa\x1CSa0xV[\x03a\x1DLWa\x1Cb\x88\x88a*\x01V[\x90P_[\x83\x81\x10\x15a\x1DFW`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1C\x91Wa\x1C\x91a4\xA2V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1C\xDEWa\x1C\xDEa4\xA2V[` \x02` \x01\x01Q\x11\x15a\x1D>Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1D\x15Wa\x1D\x15a4\xA2V[` \x02` \x01\x01Qa\x1D'\x91\x90a5\xAEV[a\x1D1\x91\x90a5\xC5V[a\x1D;\x90\x86a5\xE4V[\x94P[`\x01\x01a\x1CfV[Pa\x1E\xD2V[`\xFF\x88\x16_\x90\x81R`\x04` \x81\x90R`@\x91\x82\x90 \x91Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1D\xAB\x92\x8C\x92\x91\x01a6?V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xC5W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\xEC\x91\x90\x81\x01\x90a6\xBDV[\x90P_[\x83\x81\x10\x15a\x1E\xD0W`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1E\x1BWa\x1E\x1Ba4\xA2V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1EhWa\x1Eha4\xA2V[` \x02` \x01\x01Q\x11\x15a\x1E\xC8Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1E\x9FWa\x1E\x9Fa4\xA2V[` \x02` \x01\x01Qa\x1E\xB1\x91\x90a5\xAEV[a\x1E\xBB\x91\x90a5\xC5V[a\x1E\xC5\x90\x86a5\xE4V[\x94P[`\x01\x01a\x1D\xF0V[P[PPP`\xFF\x85\x16_\x90\x81R` \x81\x90R`@\x90 T\x90\x92P`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x84\x91\x90a5\nV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F\xB5W`@Qc\xCE\x98\xC2K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1F\xB5W`@Qc,\x01\xB2\x05`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x82\x03a \xC4W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"\x1FV[_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a \xEA`\x01\x84a4\xE3V[\x81T\x81\x10a \xFAWa \xFAa4\xA2V[_\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x03a!0W_\x93PPPPa\x12\xC6V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a!hW\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"\x1DV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U_\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"o\x82\x85a+\xE1V[\x96\x95PPPPPPV[`\xFF\x82\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\x9C\x90\x84a4\xE3V[\x81T\x81\x10a\"\xACWa\"\xACa4\xA2V[\x90_R` _ \x01\x90P\x83_\x03a\"\xD7WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\n\xE5\x90PV[\x80T_\x90a\"\xF5\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a+\xF8V[\x82T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a#0W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua#\xE1V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[_\x81Q\x11a$\x0BW`@Qcyl\xC5%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a$-\x83\x83a6\xEEV[\x11\x15a$LW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a&\xDBW_[a$b\x82\x84a6\xEEV[\x81\x10\x15a$\xF1W\x84\x82\x81Q\x81\x10a${Wa${a4\xA2V[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a$\xB7Wa$\xB7a4\xA2V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a$\xE9W`@Qc{t4\x0B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a$XV[P_\x84\x82\x81Q\x81\x10a%\x05Wa%\x05a4\xA2V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a%8W`@QcrW\x12Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x85\x16_\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a%]Wa%]a4\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a%\xC1Wa%\xC1a4\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U_\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a&7Wa&7a4\xA2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a&\x94Wa&\x94a4\xA2V[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a&\xB1Wa&\xB1a4\xA2V[` \x02` \x01\x01Q` \x01Q`@Qa&\xCB\x92\x91\x90a,{V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a$NV[PPPPPV[`\xFF\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\xFF\x82\x16_\x90\x81R`\x05` R`@\x90 \x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a'wWa'wa0xV[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa'\xAB\x91\x90a0\x8CV[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a'\xE7W`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a(\rWP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a(*W`@Qc\x13\x91\xE1\x1B`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\xFF\x82\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x81\x16c\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x17\x90\x93U\x84Q\x92\x90\x91\x16\x80\x83R\x92\x82\x01R\x90\x91\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a)8W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a(\xEC`\x01\x84a4\xE3V[\x81T\x81\x10a(\xFCWa(\xFCa4\xA2V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a)&Wa)\x1D`\x01\x82a4\xE3V[\x92PPPa\x12\xC6V[\x80a)0\x81a7\x01V[\x91PPa(\xB8V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a*9Wa*9a4\xA2V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`\xFF\x85\x16_\x90\x81R`\x06\x90\x91R`@\x81 Ta*r\x90c\xFF\xFF\xFF\xFF\x16Ca6\xEEV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x07d\xCB\x93`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x0BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+/\x91\x90a5\nV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xFF\x8A\x16` \x91\x82\x01\x81\x90R_\x90\x81R`\x04\x91\x82\x90R`@\x90\x81\x90 \x90Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Ra+y\x93\x92\x89\x92\x91\x89\x91\x01a7\x16V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x93W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\xBA\x91\x90\x81\x01\x90a7\xA1V[\x90P\x80_\x81Q\x81\x10a+\xCEWa+\xCEa4\xA2V[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[_a\x12\xC6`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a8PV[__\x82\x12\x15a,\x1BWa,\n\x82a8oV[a,\x14\x90\x84a8\x89V[\x90Pa\n\xE5V[a,\x14\x82\x84a5\xE4V[\x805`\xFF\x81\x16\x81\x14a,5W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a,JW__\xFD[a\x12\xC6\x82a,%V[__`@\x83\x85\x03\x12\x15a,dW__\xFD[a,m\x83a,%V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xF5W__\xFD[__`@\x83\x85\x03\x12\x15a,\xC2W__\xFD[a,\xCB\x83a,%V[\x91P` \x83\x015a,\xDB\x81a,\x9DV[\x80\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a,\xF6W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x0CW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\xFDW__\xFD[_____``\x86\x88\x03\x12\x15a-:W__\xFD[a-C\x86a,%V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-]W__\xFD[a-i\x88\x82\x89\x01a,\xE6V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x87W__\xFD[a-\x93\x88\x82\x89\x01a,\xE6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a-\xB4W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCAW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E\xFDW__\xFD[____``\x85\x87\x03\x12\x15a-\xF4W__\xFD[\x845a-\xFF\x81a,\x9DV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a. W__\xFD[a.,\x87\x82\x88\x01a-\xA4V[\x95\x98\x94\x97P\x95PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a.qW\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a.JV[P\x93\x94\x93PPPPV[`@\x81R_a.\x8D`@\x83\x01\x85a.8V[\x82\x81\x03` \x84\x01Ra#\xE1\x81\x85a.8V[__`@\x83\x85\x03\x12\x15a.\xB0W__\xFD[\x825\x91Pa.\xC0` \x84\x01a,%V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a/7Wa/!\x83\x85Qc\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[` \x93\x90\x93\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.\xE2V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/xWa/xa/BV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\xA6Wa/\xA6a/BV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xC6Wa/\xC6a/BV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a/\xE1W__\xFD[a/\xEA\x83a,%V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x04W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a0\x14W__\xFD[\x805a0'a0\"\x82a/\xAEV[a/~V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a0HW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a0jW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a0OV[\x80\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x02\x83\x10a0\xACWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,5W__\xFD[_\x82`\x1F\x83\x01\x12a0\xD7W__\xFD[\x815a0\xE5a0\"\x82a/\xAEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a1\x06W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x105W`@\x81\x88\x03\x12\x15a1\"W__\xFD[a1*a/VV[\x815a15\x81a,\x9DV[\x81Ra1C` \x83\x01a0\xB2V[` \x82\x01R\x80\x84RP` \x83\x01\x92P`@\x81\x01\x90Pa1\x0BV[___``\x84\x86\x03\x12\x15a1oW__\xFD[a1x\x84a,%V[\x92Pa1\x86` \x85\x01a0\xB2V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xA0W__\xFD[a1\xAC\x86\x82\x87\x01a0\xC8V[\x91PP\x92P\x92P\x92V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,5W__\xFD[___`@\x84\x86\x03\x12\x15a1\xDBW__\xFD[a1\xE4\x84a1\xB6V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xFEW__\xFD[a2\n\x86\x82\x87\x01a-\xA4V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a/7W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a20V[___``\x84\x86\x03\x12\x15a2fW__\xFD[a2o\x84a,%V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[``\x81\x01a\n\xE5\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[__`@\x83\x85\x03\x12\x15a2\xCFW__\xFD[a2\xD8\x83a,%V[\x91Pa.\xC0` \x84\x01a0\xB2V[___`@\x84\x86\x03\x12\x15a2\xF8W__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xFEW__\xFD[__`@\x83\x85\x03\x12\x15a3%W__\xFD[a3.\x83a,%V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3HW__\xFD[a3T\x85\x82\x86\x01a0\xC8V[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a3pW__\xFD[a3y\x84a,%V[\x92Pa3\x87` \x85\x01a1\xB6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[____`\x80\x85\x87\x03\x12\x15a3\xABW__\xFD[a3\xB4\x85a,%V[\x93Pa3\xC2` \x86\x01a0\xB2V[\x92Pa3\xD0`@\x86\x01a1\xB6V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xEAW__\xFD[a3\xF6\x87\x82\x88\x01a0\xC8V[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a4\x14W__\xFD[\x835\x92Pa4$` \x85\x01a,%V[\x91Pa42`@\x85\x01a1\xB6V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a4LW__\xFD[a4U\x83a,%V[\x91Pa.\xC0` \x84\x01a1\xB6V[____`\x80\x85\x87\x03\x12\x15a4vW__\xFD[a4\x7F\x85a,%V[\x93Pa4\x8D` \x86\x01a1\xB6V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a4\xC6W__\xFD[a\x12\xC6\x82a0\xB2V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\xE5Wa\n\xE5a4\xCFV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a5\x1AW__\xFD[\x81Qa\x12\xC6\x81a,\x9DV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\xFF\x83\x16` \x80\x83\x01\x91\x90\x91R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R_\x91\x84\x01\x90`\x80\x84\x01\x90\x83[\x81\x81\x10\x15a5\x82W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a5[V[P\x90\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a5\x9FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12\xC6W__\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\xE5Wa\n\xE5a4\xCFV[_\x82a5\xDFWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\n\xE5Wa\n\xE5a4\xCFV[_\x81T\x80\x84R` \x84\x01\x93P\x82_R` _ _[\x82\x81\x10\x15a.qW\x81T`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x95\x01\x94`\x01\x91\x82\x01\x91\x01a6\x18V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x14\x1E\x90\x83\x01\x84a6\x03V[_\x82`\x1F\x83\x01\x12a6qW__\xFD[\x81Qa6\x7Fa0\"\x82a/\xAEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a6\xA0W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x105W\x80Q\x83R` \x92\x83\x01\x92\x01a6\xA5V[_` \x82\x84\x03\x12\x15a6\xCDW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xE2W__\xFD[a\x14\x1E\x84\x82\x85\x01a6bV[\x80\x82\x01\x80\x82\x11\x15a\n\xE5Wa\n\xE5a4\xCFV[_\x81a7\x0FWa7\x0Fa4\xCFV[P_\x19\x01\x90V[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x83\x01R`\xA0`@\x83\x01\x81\x90R\x85Q\x90\x83\x01\x81\x90R_\x91\x86\x01\x90`\xC0\x84\x01\x90\x83[\x81\x81\x10\x15a7wW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7PV[PP\x83\x81\x03``\x85\x01Ra7\x8B\x81\x87a6\x03V[\x92PPPa#\xE1`\x80\x83\x01\x84c\xFF\xFF\xFF\xFF\x16\x90RV[_` \x82\x84\x03\x12\x15a7\xB1W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xC6W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a7\xD6W__\xFD[\x80Qa7\xE4a0\"\x82a/\xAEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a8\x05W__\xFD[` \x84\x01[\x83\x81\x10\x15a8EW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a8'W__\xFD[a86\x89` \x83\x89\x01\x01a6bV[\x84RP` \x92\x83\x01\x92\x01a8\nV[P\x96\x95PPPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x07!Wa\x07!a4\xCFV[_`\x01`\xFF\x1B\x82\x01a8\x83Wa8\x83a4\xCFV[P_\x03\x90V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\n\xE5Wa\n\xE5a4\xCFV\xFE\xA2dipfsX\"\x12 \xC2\xC1\x1E\xA3\xFB\xAB\xF1\xEAWd\xF2y\xBE\xC4\x84\xB3S\xA2K\x04\x90\0\x16\x01\x05\xE4\x149:P\xF6\"dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `BelowMinimumStakeRequirement()` and selector `0x40fe27c6`.
    ```solidity
    error BelowMinimumStakeRequirement();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BelowMinimumStakeRequirement {}
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
        impl ::core::convert::From<BelowMinimumStakeRequirement> for UnderlyingRustTuple<'_> {
            fn from(value: BelowMinimumStakeRequirement) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BelowMinimumStakeRequirement {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BelowMinimumStakeRequirement {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BelowMinimumStakeRequirement()";
            const SELECTOR: [u8; 4] = [64u8, 254u8, 39u8, 198u8];
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
    /**Custom error with signature `EmptyStakeHistory()` and selector `0xcc646573`.
    ```solidity
    error EmptyStakeHistory();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyStakeHistory {}
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
        impl ::core::convert::From<EmptyStakeHistory> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyStakeHistory) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyStakeHistory {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyStakeHistory {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyStakeHistory()";
            const SELECTOR: [u8; 4] = [204u8, 100u8, 101u8, 115u8];
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
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
    ```solidity
    error InputArrayLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch {}
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
        impl ::core::convert::From<InputArrayLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
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
    /**Custom error with signature `InputArrayLengthZero()` and selector `0x796cc525`.
    ```solidity
    error InputArrayLengthZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthZero {}
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
        impl ::core::convert::From<InputArrayLengthZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthZero()";
            const SELECTOR: [u8; 4] = [121u8, 108u8, 197u8, 37u8];
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
    /**Custom error with signature `InputDuplicateStrategy()` and selector `0x7b74340b`.
    ```solidity
    error InputDuplicateStrategy();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputDuplicateStrategy {}
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
        impl ::core::convert::From<InputDuplicateStrategy> for UnderlyingRustTuple<'_> {
            fn from(value: InputDuplicateStrategy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputDuplicateStrategy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputDuplicateStrategy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputDuplicateStrategy()";
            const SELECTOR: [u8; 4] = [123u8, 116u8, 52u8, 11u8];
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
    /**Custom error with signature `InputMultiplierZero()` and selector `0x72571251`.
    ```solidity
    error InputMultiplierZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputMultiplierZero {}
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
        impl ::core::convert::From<InputMultiplierZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputMultiplierZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputMultiplierZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputMultiplierZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputMultiplierZero()";
            const SELECTOR: [u8; 4] = [114u8, 87u8, 18u8, 81u8];
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
    /**Custom error with signature `InvalidBlockNumber()` and selector `0x4e47846c`.
    ```solidity
    error InvalidBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBlockNumber {}
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
        impl ::core::convert::From<InvalidBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBlockNumber()";
            const SELECTOR: [u8; 4] = [78u8, 71u8, 132u8, 108u8];
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
    /**Custom error with signature `OnlySlashingRegistryCoordinator()` and selector `0xb006c814`.
    ```solidity
    error OnlySlashingRegistryCoordinator();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySlashingRegistryCoordinator {}
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
        impl ::core::convert::From<OnlySlashingRegistryCoordinator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlySlashingRegistryCoordinator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlySlashingRegistryCoordinator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySlashingRegistryCoordinator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySlashingRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [176u8, 6u8, 200u8, 20u8];
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
    /**Custom error with signature `OnlySlashingRegistryCoordinatorOwner()` and selector `0xce98c24b`.
    ```solidity
    error OnlySlashingRegistryCoordinatorOwner();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlySlashingRegistryCoordinatorOwner {}
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
        impl ::core::convert::From<OnlySlashingRegistryCoordinatorOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OnlySlashingRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlySlashingRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlySlashingRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlySlashingRegistryCoordinatorOwner()";
            const SELECTOR: [u8; 4] = [206u8, 152u8, 194u8, 75u8];
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
    /**Custom error with signature `QuorumAlreadyExists()` and selector `0x4336945c`.
    ```solidity
    error QuorumAlreadyExists();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumAlreadyExists {}
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
        impl ::core::convert::From<QuorumAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumAlreadyExists()";
            const SELECTOR: [u8; 4] = [67u8, 54u8, 148u8, 92u8];
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
    /**Custom error with signature `QuorumDoesNotExist()` and selector `0xe6219fea`.
    ```solidity
    error QuorumDoesNotExist();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumDoesNotExist {}
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
        impl ::core::convert::From<QuorumDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for QuorumDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QuorumDoesNotExist()";
            const SELECTOR: [u8; 4] = [230u8, 33u8, 159u8, 234u8];
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
    /**Event with signature `LookAheadPeriodChanged(uint32,uint32)` and selector `0x28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7`.
    ```solidity
    event LookAheadPeriodChanged(uint32 oldLookAheadBlocks, uint32 newLookAheadBlocks);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LookAheadPeriodChanged {
        #[allow(missing_docs)]
        pub oldLookAheadBlocks: u32,
        #[allow(missing_docs)]
        pub newLookAheadBlocks: u32,
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
        impl alloy_sol_types::SolEvent for LookAheadPeriodChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "LookAheadPeriodChanged(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    40u8, 215u8, 53u8, 139u8, 121u8, 240u8, 45u8, 33u8, 184u8, 183u8, 225u8, 122u8,
                    239u8, 196u8, 24u8, 90u8, 100u8, 48u8, 138u8, 163u8, 116u8, 6u8, 250u8, 91u8,
                    239u8, 192u8, 91u8, 145u8, 147u8, 44u8, 57u8, 199u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldLookAheadBlocks: data.0,
                    newLookAheadBlocks: data.1,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.oldLookAheadBlocks,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.newLookAheadBlocks,
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
        impl alloy_sol_types::private::IntoLogData for LookAheadPeriodChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LookAheadPeriodChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LookAheadPeriodChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MinimumStakeForQuorumUpdated(uint8,uint96)` and selector `0x26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf`.
    ```solidity
    event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumStakeForQuorumUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
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
        impl alloy_sol_types::SolEvent for MinimumStakeForQuorumUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "MinimumStakeForQuorumUpdated(uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    38u8, 238u8, 207u8, 242u8, 183u8, 11u8, 10u8, 113u8, 16u8, 79u8, 244u8, 217u8,
                    64u8, 186u8, 113u8, 98u8, 210u8, 58u8, 149u8, 194u8, 72u8, 119u8, 31u8, 196u8,
                    135u8, 167u8, 190u8, 23u8, 165u8, 150u8, 179u8, 207u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    minimumStake: data.0,
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.minimumStake,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumStakeForQuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumStakeForQuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumStakeForQuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorStakeUpdate(bytes32,uint8,uint96)` and selector `0x2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d`.
    ```solidity
    event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorStakeUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
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
        impl alloy_sol_types::SolEvent for OperatorStakeUpdate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorStakeUpdate(bytes32,uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 82u8, 125u8, 82u8, 126u8, 149u8, 216u8, 254u8, 64u8, 174u8, 197u8, 83u8,
                    119u8, 116u8, 59u8, 183u8, 121u8, 8u8, 125u8, 163u8, 246u8, 208u8, 208u8,
                    143u8, 18u8, 227u8, 100u8, 68u8, 218u8, 98u8, 50u8, 125u8,
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
                    stake: data.1,
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.stake,
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
        impl alloy_sol_types::private::IntoLogData for OperatorStakeUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorStakeUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorStakeUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumCreated(uint8)` and selector `0x831a9c86c45bb303caf3f064be2bc2b9fd4ecf19e47c4ac02a61e75dabfe55b4`.
    ```solidity
    event QuorumCreated(uint8 indexed quorumNumber);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumCreated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
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
        impl alloy_sol_types::SolEvent for QuorumCreated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumCreated(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    131u8, 26u8, 156u8, 134u8, 196u8, 91u8, 179u8, 3u8, 202u8, 243u8, 240u8, 100u8,
                    190u8, 43u8, 194u8, 185u8, 253u8, 78u8, 207u8, 25u8, 228u8, 124u8, 74u8, 192u8,
                    42u8, 97u8, 231u8, 93u8, 171u8, 254u8, 85u8, 180u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakeTypeSet(uint8)` and selector `0x7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d`.
    ```solidity
    event StakeTypeSet(IStakeRegistryTypes.StakeType newStakeType);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakeTypeSet {
        #[allow(missing_docs)]
        pub newStakeType: <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for StakeTypeSet {
            type DataTuple<'a> = (IStakeRegistryTypes::StakeType,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StakeTypeSet(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    124u8, 17u8, 46u8, 134u8, 60u8, 207u8, 0u8, 120u8, 98u8, 226u8, 201u8, 226u8,
                    88u8, 25u8, 201u8, 51u8, 254u8, 219u8, 201u8, 53u8, 10u8, 100u8, 67u8, 66u8,
                    59u8, 74u8, 133u8, 153u8, 194u8, 232u8, 165u8, 45u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newStakeType: data.0,
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
                    <IStakeRegistryTypes::StakeType as alloy_sol_types::SolType>::tokenize(
                        &self.newStakeType,
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
        impl alloy_sol_types::private::IntoLogData for StakeTypeSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakeTypeSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakeTypeSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyAddedToQuorum(uint8,address)` and selector `0x10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f5404`.
    ```solidity
    event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyAddedToQuorum {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StrategyAddedToQuorum {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyAddedToQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    16u8, 86u8, 94u8, 86u8, 202u8, 203u8, 243u8, 46u8, 202u8, 38u8, 121u8, 69u8,
                    240u8, 84u8, 254u8, 192u8, 46u8, 89u8, 117u8, 0u8, 50u8, 209u8, 19u8, 211u8,
                    48u8, 33u8, 130u8, 173u8, 150u8, 127u8, 84u8, 4u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyAddedToQuorum {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyAddedToQuorum> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyAddedToQuorum) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyMultiplierUpdated(uint8,address,uint256)` and selector `0x11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75`.
    ```solidity
    event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyMultiplierUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for StrategyMultiplierUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyMultiplierUpdated(uint8,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    17u8, 165u8, 100u8, 19u8, 34u8, 218u8, 29u8, 255u8, 86u8, 164u8, 182u8, 110u8,
                    170u8, 195u8, 31u8, 250u8, 70u8, 82u8, 149u8, 236u8, 233u8, 7u8, 205u8, 22u8,
                    52u8, 55u8, 121u8, 59u8, 77u8, 0u8, 154u8, 117u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
                    multiplier: data.1,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyMultiplierUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyMultiplierUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyMultiplierUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyRemovedFromQuorum(uint8,address)` and selector `0x31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f7`.
    ```solidity
    event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyRemovedFromQuorum {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StrategyRemovedFromQuorum {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyRemovedFromQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 250u8, 46u8, 44u8, 210u8, 128u8, 201u8, 55u8, 94u8, 19u8, 255u8, 207u8,
                    61u8, 129u8, 226u8, 55u8, 129u8, 0u8, 24u8, 110u8, 64u8, 88u8, 248u8, 211u8,
                    221u8, 182u8, 144u8, 184u8, 45u8, 205u8, 49u8, 247u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyRemovedFromQuorum {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyRemovedFromQuorum> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyRemovedFromQuorum) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _slashingRegistryCoordinator, address _delegationManager, address _avsDirectory, address _allocationManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        pub _delegationManager: alloy::sol_types::private::Address,
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _allocationManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._slashingRegistryCoordinator,
                        value._delegationManager,
                        value._avsDirectory,
                        value._allocationManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _slashingRegistryCoordinator: tuple.0,
                        _delegationManager: tuple.1,
                        _avsDirectory: tuple.2,
                        _allocationManager: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
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
                        &self._slashingRegistryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`.
    ```solidity
    function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WEIGHING_FUNCTION_LENGTHCall {}
    ///Container type for the return parameters of the [`MAX_WEIGHING_FUNCTION_LENGTH()`](MAX_WEIGHING_FUNCTION_LENGTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WEIGHING_FUNCTION_LENGTHReturn {
        pub _0: u8,
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
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_WEIGHING_FUNCTION_LENGTHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_WEIGHING_FUNCTION_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_WEIGHING_FUNCTION_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_WEIGHING_FUNCTION_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_WEIGHING_FUNCTION_LENGTH()";
            const SELECTOR: [u8; 4] = [124u8, 23u8, 35u8, 71u8];
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
    /**Function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`.
    ```solidity
    function WEIGHTING_DIVISOR() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WEIGHTING_DIVISORCall {}
    ///Container type for the return parameters of the [`WEIGHTING_DIVISOR()`](WEIGHTING_DIVISORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WEIGHTING_DIVISORReturn {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<WEIGHTING_DIVISORCall> for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WEIGHTING_DIVISORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
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
            impl ::core::convert::From<WEIGHTING_DIVISORReturn> for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WEIGHTING_DIVISORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WEIGHTING_DIVISORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = WEIGHTING_DIVISORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WEIGHTING_DIVISOR()";
            const SELECTOR: [u8; 4] = [94u8, 90u8, 103u8, 117u8];
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
    /**Function with signature `addStrategies(uint8,(address,uint96)[])` and selector `0xc601527d`.
    ```solidity
    function addStrategies(uint8 quorumNumber, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesCall {
        pub quorumNumber: u8,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`addStrategies(uint8,(address,uint96)[])`](addStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesReturn {}
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
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesCall) -> Self {
                    (value.quorumNumber, value._strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _strategyParams: tuple.1,
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
            impl ::core::convert::From<addStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategies(uint8,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [198u8, 1u8, 82u8, 125u8];
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
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
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
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
    ```solidity
    function allocationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
    ```solidity
    function avsDirectory() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
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
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
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
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
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
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
    ```solidity
    function delegation() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
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
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
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
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
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
    /**Function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`.
    ```solidity
    function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentStakeCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getCurrentStake(bytes32,uint8)`](getCurrentStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentStakeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            impl ::core::convert::From<getCurrentStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getCurrentStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentStake(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [84u8, 1u8, 237u8, 39u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`.
    ```solidity
    function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalStakeCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getCurrentTotalStake(uint8)`](getCurrentTotalStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalStakeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getCurrentTotalStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentTotalStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getCurrentTotalStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentTotalStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalStakeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentTotalStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentTotalStake(uint8)";
            const SELECTOR: [u8; 4] = [213u8, 236u8, 204u8, 5u8];
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
    /**Function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`.
    ```solidity
    function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestStakeUpdateCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getLatestStakeUpdate(bytes32,uint8)`](getLatestStakeUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestStakeUpdateReturn {
        pub _0: <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            impl ::core::convert::From<getLatestStakeUpdateCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestStakeUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getLatestStakeUpdateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestStakeUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestStakeUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestStakeUpdateReturn;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestStakeUpdate(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [248u8, 81u8, 225u8, 152u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`.
    ```solidity
    function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getStakeAtBlockNumber(bytes32,uint8,uint32)`](getStakeAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8, u32);
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
            impl ::core::convert::From<getStakeAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getStakeAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeAtBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeAtBlockNumber(bytes32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [250u8, 40u8, 198u8, 39u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`.
    ```solidity
    function getStakeAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, bytes32 operatorId, uint256 index) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberAndIndexCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)`](getStakeAtBlockNumberAndIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberAndIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexCall) -> Self {
                    (
                        value.quorumNumber,
                        value.blockNumber,
                        value.operatorId,
                        value.index,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberAndIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                        operatorId: tuple.2,
                        index: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberAndIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeAtBlockNumberAndIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeAtBlockNumberAndIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [242u8, 190u8, 148u8, 174u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
    /**Function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`.
    ```solidity
    function getStakeHistory(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeUpdate[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getStakeHistory(bytes32,uint8)`](getStakeHistoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            impl ::core::convert::From<getStakeHistoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StakeUpdate>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStakeHistoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeHistoryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeHistoryReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StakeUpdate>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeHistory(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [44u8, 217u8, 89u8, 64u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`.
    ```solidity
    function getStakeHistoryLength(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryLengthCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getStakeHistoryLength(bytes32,uint8)`](getStakeHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryLengthReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            impl ::core::convert::From<getStakeHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
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
            impl ::core::convert::From<getStakeHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeHistoryLengthCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeHistoryLength(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [75u8, 210u8, 110u8, 9u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`.
    ```solidity
    function getStakeUpdateAtIndex(uint8 quorumNumber, bytes32 operatorId, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getStakeUpdateAtIndex(uint8,bytes32,uint256)`](getStakeUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateAtIndexReturn {
        pub _0: <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getStakeUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorId: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getStakeUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeUpdateAtIndex(uint8,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [172u8, 107u8, 251u8, 3u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
    /**Function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`.
    ```solidity
    function getStakeUpdateIndexAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateIndexAtBlockNumberCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)`](getStakeUpdateIndexAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateIndexAtBlockNumberReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8, u32);
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
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateIndexAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        blockNumber: tuple.2,
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
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateIndexAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeUpdateIndexAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeUpdateIndexAtBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [221u8, 152u8, 70u8, 185u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`.
    ```solidity
    function getTotalStakeAtBlockNumberFromIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeAtBlockNumberFromIndexCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)`](getTotalStakeAtBlockNumberFromIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeAtBlockNumberFromIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                u32,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexCall) -> Self {
                    (value.quorumNumber, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeAtBlockNumberFromIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeAtBlockNumberFromIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeAtBlockNumberFromIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeAtBlockNumberFromIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)";
            const SELECTOR: [u8; 4] = [200u8, 41u8, 76u8, 86u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
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
    /**Function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`.
    ```solidity
    function getTotalStakeHistoryLength(uint8 quorumNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeHistoryLengthCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getTotalStakeHistoryLength(uint8)`](getTotalStakeHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeHistoryLengthReturn {
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
            impl ::core::convert::From<getTotalStakeHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
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
            impl ::core::convert::From<getTotalStakeHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeHistoryLength(uint8)";
            const SELECTOR: [u8; 4] = [4u8, 145u8, 180u8, 28u8];
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
    /**Function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`.
    ```solidity
    function getTotalStakeIndicesAtBlockNumber(uint32 blockNumber, bytes memory quorumNumbers) external view returns (uint32[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeIndicesAtBlockNumberCall {
        pub blockNumber: u32,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`getTotalStakeIndicesAtBlockNumber(uint32,bytes)`](getTotalStakeIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeIndicesAtBlockNumberReturn {
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::Bytes);
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
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
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
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeIndicesAtBlockNumber(uint32,bytes)";
            const SELECTOR: [u8; 4] = [129u8, 192u8, 117u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
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
    /**Function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`.
    ```solidity
    function getTotalStakeUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StakeUpdate memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getTotalStakeUpdateAtIndex(uint8,uint256)`](getTotalStakeUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeUpdateAtIndexReturn {
        pub _0: <IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<getTotalStakeUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistryTypes::StakeUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getTotalStakeUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeUpdateAtIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [182u8, 144u8, 75u8, 120u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
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
    /**Function with signature `initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])` and selector `0x75d4173a`.
    ```solidity
    function initializeDelegatedStakeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeDelegatedStakeQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])`](initializeDelegatedStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeDelegatedStakeQuorumReturn {}
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
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeDelegatedStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumCall) -> Self {
                    (
                        value.quorumNumber,
                        value.minimumStake,
                        value._strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeDelegatedStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                        _strategyParams: tuple.2,
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
            impl ::core::convert::From<initializeDelegatedStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeDelegatedStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeDelegatedStakeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [117u8, 212u8, 23u8, 58u8];
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
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
    /**Function with signature `initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])` and selector `0xcc5a7c20`.
    ```solidity
    function initializeSlashableStakeQuorum(uint8 quorumNumber, uint96 minimumStake, uint32 lookAheadPeriod, IStakeRegistryTypes.StrategyParams[] memory _strategyParams) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeSlashableStakeQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub lookAheadPeriod: u32,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])`](initializeSlashableStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeSlashableStakeQuorumReturn {}
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
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
                u32,
                alloy::sol_types::private::Vec<
                    <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeSlashableStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumCall) -> Self {
                    (
                        value.quorumNumber,
                        value.minimumStake,
                        value.lookAheadPeriod,
                        value._strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeSlashableStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                        lookAheadPeriod: tuple.2,
                        _strategyParams: tuple.3,
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
            impl ::core::convert::From<initializeSlashableStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeSlashableStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeSlashableStakeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IStakeRegistryTypes::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [204u8, 90u8, 124u8, 32u8];
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistryTypes::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
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
    /**Function with signature `isOperatorSetQuorum(uint8)` and selector `0x9f8aff26`.
    ```solidity
    function isOperatorSetQuorum(uint8 quorumNumber) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`isOperatorSetQuorum(uint8)`](isOperatorSetQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetQuorumReturn {
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
            impl ::core::convert::From<isOperatorSetQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
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
            impl ::core::convert::From<isOperatorSetQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSetQuorum(uint8)";
            const SELECTOR: [u8; 4] = [159u8, 138u8, 255u8, 38u8];
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
    /**Function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`.
    ```solidity
    function minimumStakeForQuorum(uint8) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumStakeForQuorumCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`minimumStakeForQuorum(uint8)`](minimumStakeForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumStakeForQuorumReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<minimumStakeForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<minimumStakeForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumStakeForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumStakeForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumStakeForQuorum(uint8)";
            const SELECTOR: [u8; 4] = [196u8, 103u8, 120u8, 165u8];
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
                        &self._0,
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
    /**Function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`.
    ```solidity
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsCall {
        pub quorumNumber: u8,
        pub strategyIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        pub newMultipliers:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
    }
    ///Container type for the return parameters of the [`modifyStrategyParams(uint8,uint256[],uint96[])`](modifyStrategyParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            impl ::core::convert::From<modifyStrategyParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsCall) -> Self {
                    (
                        value.quorumNumber,
                        value.strategyIndices,
                        value.newMultipliers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyStrategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        strategyIndices: tuple.1,
                        newMultipliers: tuple.2,
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
            impl ::core::convert::From<modifyStrategyParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyStrategyParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyStrategyParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyStrategyParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyStrategyParams(uint8,uint256[],uint96[])";
            const SELECTOR: [u8; 4] = [32u8, 182u8, 98u8, 152u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMultipliers),
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
    /**Function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`.
    ```solidity
    function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,bytes32,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        pub _1: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorId: tuple.1,
                        quorumNumbers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [37u8, 80u8, 71u8, 119u8];
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
    /**Function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`.
    ```solidity
    function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesCall {
        pub quorumNumber: u8,
        pub indicesToRemove:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`removeStrategies(uint8,uint256[])`](removeStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<removeStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesCall) -> Self {
                    (value.quorumNumber, value.indicesToRemove)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        indicesToRemove: tuple.1,
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
            impl ::core::convert::From<removeStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategies(uint8,uint256[])";
            const SELECTOR: [u8; 4] = [95u8, 31u8, 45u8, 119u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.indicesToRemove),
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
    /**Function with signature `setMinimumStakeForQuorum(uint8,uint96)` and selector `0xbc9a40c3`.
    ```solidity
    function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakeForQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
    }
    ///Container type for the return parameters of the [`setMinimumStakeForQuorum(uint8,uint96)`](setMinimumStakeForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakeForQuorumReturn {}
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
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U96);
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
            impl ::core::convert::From<setMinimumStakeForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
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
            impl ::core::convert::From<setMinimumStakeForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMinimumStakeForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinimumStakeForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMinimumStakeForQuorum(uint8,uint96)";
            const SELECTOR: [u8; 4] = [188u8, 154u8, 64u8, 195u8];
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.minimumStake,
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
    /**Function with signature `setSlashableStakeLookahead(uint8,uint32)` and selector `0xe086adb3`.
    ```solidity
    function setSlashableStakeLookahead(uint8 quorumNumber, uint32 _lookAheadBlocks) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashableStakeLookaheadCall {
        pub quorumNumber: u8,
        pub _lookAheadBlocks: u32,
    }
    ///Container type for the return parameters of the [`setSlashableStakeLookahead(uint8,uint32)`](setSlashableStakeLookaheadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashableStakeLookaheadReturn {}
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
            impl ::core::convert::From<setSlashableStakeLookaheadCall> for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadCall) -> Self {
                    (value.quorumNumber, value._lookAheadBlocks)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setSlashableStakeLookaheadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _lookAheadBlocks: tuple.1,
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
            impl ::core::convert::From<setSlashableStakeLookaheadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setSlashableStakeLookaheadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSlashableStakeLookaheadCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSlashableStakeLookaheadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSlashableStakeLookahead(uint8,uint32)";
            const SELECTOR: [u8; 4] = [224u8, 134u8, 173u8, 179u8];
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
                        &self._lookAheadBlocks,
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
    /**Function with signature `slashableStakeLookAheadPerQuorum(uint8)` and selector `0x9ab4d6ff`.
    ```solidity
    function slashableStakeLookAheadPerQuorum(uint8 quorumNumber) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashableStakeLookAheadPerQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`slashableStakeLookAheadPerQuorum(uint8)`](slashableStakeLookAheadPerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashableStakeLookAheadPerQuorumReturn {
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
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashableStakeLookAheadPerQuorumCall {
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
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashableStakeLookAheadPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashableStakeLookAheadPerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashableStakeLookAheadPerQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashableStakeLookAheadPerQuorum(uint8)";
            const SELECTOR: [u8; 4] = [154u8, 180u8, 214u8, 255u8];
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
    /**Function with signature `stakeTypePerQuorum(uint8)` and selector `0x697fbd93`.
    ```solidity
    function stakeTypePerQuorum(uint8 quorumNumber) external view returns (IStakeRegistryTypes.StakeType);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTypePerQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`stakeTypePerQuorum(uint8)`](stakeTypePerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTypePerQuorumReturn {
        pub _0: <IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<stakeTypePerQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeTypePerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StakeType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistryTypes::StakeType as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<stakeTypePerQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeTypePerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeTypePerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeTypePerQuorumReturn;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StakeType,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeTypePerQuorum(uint8)";
            const SELECTOR: [u8; 4] = [105u8, 127u8, 189u8, 147u8];
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
    /**Function with signature `strategiesPerQuorum(uint8,uint256)` and selector `0x9f3ccf65`.
    ```solidity
    function strategiesPerQuorum(uint8 quorumNumber, uint256) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesPerQuorumCall {
        pub quorumNumber: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategiesPerQuorum(uint8,uint256)`](strategiesPerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesPerQuorumReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<strategiesPerQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumCall) -> Self {
                    (value.quorumNumber, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategiesPerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _1: tuple.1,
                    }
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
            impl ::core::convert::From<strategiesPerQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategiesPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategiesPerQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategiesPerQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategiesPerQuorum(uint8,uint256)";
            const SELECTOR: [u8; 4] = [159u8, 60u8, 207u8, 101u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`.
    ```solidity
    function strategyParams(uint8 quorumNumber, uint256) external view returns (address strategy, uint96 multiplier);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsCall {
        pub quorumNumber: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategyParams(uint8,uint256)`](strategyParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<strategyParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsCall) -> Self {
                    (value.quorumNumber, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
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
            impl ::core::convert::From<strategyParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsReturn) -> Self {
                    (value.strategy, value.multiplier)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategy: tuple.0,
                        multiplier: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParams(uint8,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 115u8, 36u8, 97u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`.
    ```solidity
    function strategyParamsByIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistryTypes.StrategyParams memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsByIndexCall {
        pub quorumNumber: u8,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategyParamsByIndex(uint8,uint256)`](strategyParamsByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsByIndexReturn {
        pub _0: <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<strategyParamsByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistryTypes::StrategyParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<strategyParamsByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsByIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistryTypes::StrategyParams,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParamsByIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [173u8, 200u8, 4u8, 218u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
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
    /**Function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`.
    ```solidity
    function strategyParamsLength(uint8 quorumNumber) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsLengthCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`strategyParamsLength(uint8)`](strategyParamsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsLengthReturn {
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
            impl ::core::convert::From<strategyParamsLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
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
            impl ::core::convert::From<strategyParamsLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParamsLength(uint8)";
            const SELECTOR: [u8; 4] = [60u8, 165u8, 165u8, 245u8];
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
    /**Function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`.
    ```solidity
    function updateOperatorStake(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint192);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorStakeCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorStake(address,bytes32,bytes)`](updateOperatorStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorStakeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<updateOperatorStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorStakeCall) -> Self {
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorId: tuple.1,
                        quorumNumbers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U192,);
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
            impl ::core::convert::From<updateOperatorStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorStake(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [102u8, 172u8, 254u8, 254u8];
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
    /**Function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`.
    ```solidity
    function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct weightOfOperatorForQuorumCall {
        pub quorumNumber: u8,
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`weightOfOperatorForQuorum(uint8,address)`](weightOfOperatorForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct weightOfOperatorForQuorumReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::Address);
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
            impl ::core::convert::From<weightOfOperatorForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumCall) -> Self {
                    (value.quorumNumber, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for weightOfOperatorForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<weightOfOperatorForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for weightOfOperatorForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for weightOfOperatorForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = weightOfOperatorForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weightOfOperatorForQuorum(uint8,address)";
            const SELECTOR: [u8; 4] = [31u8, 155u8, 116u8, 224u8];
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
    ///Container for all the [`StakeRegistry`](self) function calls.
    pub enum StakeRegistryCalls {
        MAX_WEIGHING_FUNCTION_LENGTH(MAX_WEIGHING_FUNCTION_LENGTHCall),
        WEIGHTING_DIVISOR(WEIGHTING_DIVISORCall),
        addStrategies(addStrategiesCall),
        allocationManager(allocationManagerCall),
        avsDirectory(avsDirectoryCall),
        delegation(delegationCall),
        deregisterOperator(deregisterOperatorCall),
        getCurrentStake(getCurrentStakeCall),
        getCurrentTotalStake(getCurrentTotalStakeCall),
        getLatestStakeUpdate(getLatestStakeUpdateCall),
        getStakeAtBlockNumber(getStakeAtBlockNumberCall),
        getStakeAtBlockNumberAndIndex(getStakeAtBlockNumberAndIndexCall),
        getStakeHistory(getStakeHistoryCall),
        getStakeHistoryLength(getStakeHistoryLengthCall),
        getStakeUpdateAtIndex(getStakeUpdateAtIndexCall),
        getStakeUpdateIndexAtBlockNumber(getStakeUpdateIndexAtBlockNumberCall),
        getTotalStakeAtBlockNumberFromIndex(getTotalStakeAtBlockNumberFromIndexCall),
        getTotalStakeHistoryLength(getTotalStakeHistoryLengthCall),
        getTotalStakeIndicesAtBlockNumber(getTotalStakeIndicesAtBlockNumberCall),
        getTotalStakeUpdateAtIndex(getTotalStakeUpdateAtIndexCall),
        initializeDelegatedStakeQuorum(initializeDelegatedStakeQuorumCall),
        initializeSlashableStakeQuorum(initializeSlashableStakeQuorumCall),
        isOperatorSetQuorum(isOperatorSetQuorumCall),
        minimumStakeForQuorum(minimumStakeForQuorumCall),
        modifyStrategyParams(modifyStrategyParamsCall),
        registerOperator(registerOperatorCall),
        registryCoordinator(registryCoordinatorCall),
        removeStrategies(removeStrategiesCall),
        setMinimumStakeForQuorum(setMinimumStakeForQuorumCall),
        setSlashableStakeLookahead(setSlashableStakeLookaheadCall),
        slashableStakeLookAheadPerQuorum(slashableStakeLookAheadPerQuorumCall),
        stakeTypePerQuorum(stakeTypePerQuorumCall),
        strategiesPerQuorum(strategiesPerQuorumCall),
        strategyParams(strategyParamsCall),
        strategyParamsByIndex(strategyParamsByIndexCall),
        strategyParamsLength(strategyParamsLengthCall),
        updateOperatorStake(updateOperatorStakeCall),
        weightOfOperatorForQuorum(weightOfOperatorForQuorumCall),
    }
    #[automatically_derived]
    impl StakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 145u8, 180u8, 28u8],
            [8u8, 115u8, 36u8, 97u8],
            [31u8, 155u8, 116u8, 224u8],
            [32u8, 182u8, 98u8, 152u8],
            [37u8, 80u8, 71u8, 119u8],
            [44u8, 217u8, 89u8, 64u8],
            [60u8, 165u8, 165u8, 245u8],
            [75u8, 210u8, 110u8, 9u8],
            [84u8, 1u8, 237u8, 39u8],
            [94u8, 90u8, 103u8, 117u8],
            [95u8, 31u8, 45u8, 119u8],
            [102u8, 172u8, 254u8, 254u8],
            [105u8, 127u8, 189u8, 147u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 20u8, 169u8, 135u8],
            [117u8, 212u8, 23u8, 58u8],
            [124u8, 23u8, 35u8, 71u8],
            [129u8, 192u8, 117u8, 2u8],
            [154u8, 180u8, 214u8, 255u8],
            [159u8, 60u8, 207u8, 101u8],
            [159u8, 138u8, 255u8, 38u8],
            [172u8, 107u8, 251u8, 3u8],
            [173u8, 200u8, 4u8, 218u8],
            [182u8, 144u8, 75u8, 120u8],
            [188u8, 154u8, 64u8, 195u8],
            [189u8, 41u8, 184u8, 205u8],
            [196u8, 103u8, 120u8, 165u8],
            [198u8, 1u8, 82u8, 125u8],
            [200u8, 41u8, 76u8, 86u8],
            [202u8, 138u8, 167u8, 199u8],
            [204u8, 90u8, 124u8, 32u8],
            [213u8, 236u8, 204u8, 5u8],
            [221u8, 152u8, 70u8, 185u8],
            [223u8, 92u8, 247u8, 35u8],
            [224u8, 134u8, 173u8, 179u8],
            [242u8, 190u8, 148u8, 174u8],
            [248u8, 81u8, 225u8, 152u8],
            [250u8, 40u8, 198u8, 39u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryCalls {
        const NAME: &'static str = "StakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 38usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(_) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WEIGHTING_DIVISOR(_) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategies(_) => <addStrategiesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentStake(_) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentTotalStake(_) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestStakeUpdate(_) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeAtBlockNumber(_) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeAtBlockNumberAndIndex(_) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeHistory(_) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeHistoryLength(_) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeUpdateAtIndex(_) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeUpdateIndexAtBlockNumber(_) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeAtBlockNumberFromIndex(_) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeHistoryLength(_) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeIndicesAtBlockNumber(_) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeUpdateAtIndex(_) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeDelegatedStakeQuorum(_) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeSlashableStakeQuorum(_) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSetQuorum(_) => {
                    <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumStakeForQuorum(_) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyStrategyParams(_) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategies(_) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinimumStakeForQuorum(_) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSlashableStakeLookahead(_) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashableStakeLookAheadPerQuorum(_) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeTypePerQuorum(_) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategiesPerQuorum(_) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParams(_) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParamsByIndex(_) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParamsLength(_) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorStake(_) => {
                    <updateOperatorStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::weightOfOperatorForQuorum(_) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<StakeRegistryCalls>] = &[
                {
                    fn getTotalStakeHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeHistoryLength)
                    }
                    getTotalStakeHistoryLength
                },
                {
                    fn strategyParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::strategyParams)
                    }
                    strategyParams
                },
                {
                    fn weightOfOperatorForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::weightOfOperatorForQuorum)
                    }
                    weightOfOperatorForQuorum
                },
                {
                    fn modifyStrategyParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::modifyStrategyParams)
                    }
                    modifyStrategyParams
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getStakeHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getStakeHistory)
                    }
                    getStakeHistory
                },
                {
                    fn strategyParamsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::strategyParamsLength)
                    }
                    strategyParamsLength
                },
                {
                    fn getStakeHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getStakeHistoryLength)
                    }
                    getStakeHistoryLength
                },
                {
                    fn getCurrentStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getCurrentStake)
                    }
                    getCurrentStake
                },
                {
                    fn WEIGHTING_DIVISOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::WEIGHTING_DIVISOR)
                    }
                    WEIGHTING_DIVISOR
                },
                {
                    fn removeStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <removeStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::removeStrategies)
                    }
                    removeStrategies
                },
                {
                    fn updateOperatorStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::updateOperatorStake)
                    }
                    updateOperatorStake
                },
                {
                    fn stakeTypePerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::stakeTypePerQuorum)
                    }
                    stakeTypePerQuorum
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn initializeDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::initializeDelegatedStakeQuorum)
                    }
                    initializeDelegatedStakeQuorum
                },
                {
                    fn MAX_WEIGHING_FUNCTION_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::MAX_WEIGHING_FUNCTION_LENGTH)
                    }
                    MAX_WEIGHING_FUNCTION_LENGTH
                },
                {
                    fn getTotalStakeIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeIndicesAtBlockNumber)
                    }
                    getTotalStakeIndicesAtBlockNumber
                },
                {
                    fn slashableStakeLookAheadPerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::slashableStakeLookAheadPerQuorum)
                    }
                    slashableStakeLookAheadPerQuorum
                },
                {
                    fn strategiesPerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::strategiesPerQuorum)
                    }
                    strategiesPerQuorum
                },
                {
                    fn isOperatorSetQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::isOperatorSetQuorum)
                    }
                    isOperatorSetQuorum
                },
                {
                    fn getStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getStakeUpdateAtIndex)
                    }
                    getStakeUpdateAtIndex
                },
                {
                    fn strategyParamsByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::strategyParamsByIndex)
                    }
                    strategyParamsByIndex
                },
                {
                    fn getTotalStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeUpdateAtIndex)
                    }
                    getTotalStakeUpdateAtIndex
                },
                {
                    fn setMinimumStakeForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::setMinimumStakeForQuorum)
                    }
                    setMinimumStakeForQuorum
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn minimumStakeForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::minimumStakeForQuorum)
                    }
                    minimumStakeForQuorum
                },
                {
                    fn addStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <addStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::addStrategies)
                    }
                    addStrategies
                },
                {
                    fn getTotalStakeAtBlockNumberFromIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeAtBlockNumberFromIndex)
                    }
                    getTotalStakeAtBlockNumberFromIndex
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initializeSlashableStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::initializeSlashableStakeQuorum)
                    }
                    initializeSlashableStakeQuorum
                },
                {
                    fn getCurrentTotalStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getCurrentTotalStake)
                    }
                    getCurrentTotalStake
                },
                {
                    fn getStakeUpdateIndexAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateIndexAtBlockNumber)
                    }
                    getStakeUpdateIndexAtBlockNumber
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(StakeRegistryCalls::delegation)
                    }
                    delegation
                },
                {
                    fn setSlashableStakeLookahead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::setSlashableStakeLookahead)
                    }
                    setSlashableStakeLookahead
                },
                {
                    fn getStakeAtBlockNumberAndIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumberAndIndex)
                    }
                    getStakeAtBlockNumberAndIndex
                },
                {
                    fn getLatestStakeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getLatestStakeUpdate)
                    }
                    getLatestStakeUpdate
                },
                {
                    fn getStakeAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getStakeAtBlockNumber)
                    }
                    getStakeAtBlockNumber
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(inner) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WEIGHTING_DIVISOR(inner) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStrategies(inner) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentStake(inner) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentTotalStake(inner) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestStakeUpdate(inner) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeAtBlockNumber(inner) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeAtBlockNumberAndIndex(inner) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeHistory(inner) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeHistoryLength(inner) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeUpdateAtIndex(inner) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeUpdateIndexAtBlockNumber(inner) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeAtBlockNumberFromIndex(inner) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeHistoryLength(inner) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeIndicesAtBlockNumber(inner) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeUpdateAtIndex(inner) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeDelegatedStakeQuorum(inner) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeSlashableStakeQuorum(inner) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorSetQuorum(inner) => {
                    <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumStakeForQuorum(inner) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyStrategyParams(inner) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::removeStrategies(inner) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setSlashableStakeLookahead(inner) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashableStakeLookAheadPerQuorum(inner) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeTypePerQuorum(inner) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategiesPerQuorum(inner) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParams(inner) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParamsByIndex(inner) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParamsLength(inner) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorStake(inner) => {
                    <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::weightOfOperatorForQuorum(inner) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(inner) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WEIGHTING_DIVISOR(inner) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addStrategies(inner) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCurrentStake(inner) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentTotalStake(inner) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestStakeUpdate(inner) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeAtBlockNumber(inner) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeAtBlockNumberAndIndex(inner) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeHistory(inner) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeHistoryLength(inner) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeUpdateAtIndex(inner) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeUpdateIndexAtBlockNumber(inner) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeAtBlockNumberFromIndex(inner) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeHistoryLength(inner) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeIndicesAtBlockNumber(inner) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeUpdateAtIndex(inner) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeDelegatedStakeQuorum(inner) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeSlashableStakeQuorum(inner) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorSetQuorum(inner) => {
                    <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumStakeForQuorum(inner) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyStrategyParams(inner) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::removeStrategies(inner) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setSlashableStakeLookahead(inner) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashableStakeLookAheadPerQuorum(inner) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeTypePerQuorum(inner) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategiesPerQuorum(inner) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParams(inner) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParamsByIndex(inner) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParamsLength(inner) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorStake(inner) => {
                    <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::weightOfOperatorForQuorum(inner) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StakeRegistry`](self) custom errors.
    pub enum StakeRegistryErrors {
        BelowMinimumStakeRequirement(BelowMinimumStakeRequirement),
        EmptyStakeHistory(EmptyStakeHistory),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InputArrayLengthZero(InputArrayLengthZero),
        InputDuplicateStrategy(InputDuplicateStrategy),
        InputMultiplierZero(InputMultiplierZero),
        InvalidBlockNumber(InvalidBlockNumber),
        OnlySlashingRegistryCoordinator(OnlySlashingRegistryCoordinator),
        OnlySlashingRegistryCoordinatorOwner(OnlySlashingRegistryCoordinatorOwner),
        QuorumAlreadyExists(QuorumAlreadyExists),
        QuorumDoesNotExist(QuorumDoesNotExist),
    }
    #[automatically_derived]
    impl StakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [64u8, 254u8, 39u8, 198u8],
            [67u8, 54u8, 148u8, 92u8],
            [67u8, 113u8, 74u8, 253u8],
            [78u8, 71u8, 132u8, 108u8],
            [114u8, 87u8, 18u8, 81u8],
            [121u8, 108u8, 197u8, 37u8],
            [123u8, 116u8, 52u8, 11u8],
            [176u8, 6u8, 200u8, 20u8],
            [204u8, 100u8, 101u8, 115u8],
            [206u8, 152u8, 194u8, 75u8],
            [230u8, 33u8, 159u8, 234u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryErrors {
        const NAME: &'static str = "StakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BelowMinimumStakeRequirement(_) => {
                    <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyStakeHistory(_) => {
                    <EmptyStakeHistory as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthZero(_) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputDuplicateStrategy(_) => {
                    <InputDuplicateStrategy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputMultiplierZero(_) => {
                    <InputMultiplierZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBlockNumber(_) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlySlashingRegistryCoordinator(_) => {
                    <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlySlashingRegistryCoordinatorOwner(_) => {
                    <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumAlreadyExists(_) => {
                    <QuorumAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::QuorumDoesNotExist(_) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::SELECTOR
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<StakeRegistryErrors>] = &[
                {
                    fn BelowMinimumStakeRequirement(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::BelowMinimumStakeRequirement)
                    }
                    BelowMinimumStakeRequirement
                },
                {
                    fn QuorumAlreadyExists(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::QuorumAlreadyExists)
                    }
                    QuorumAlreadyExists
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn InvalidBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::InvalidBlockNumber)
                    }
                    InvalidBlockNumber
                },
                {
                    fn InputMultiplierZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputMultiplierZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::InputMultiplierZero)
                    }
                    InputMultiplierZero
                },
                {
                    fn InputArrayLengthZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputArrayLengthZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::InputArrayLengthZero)
                    }
                    InputArrayLengthZero
                },
                {
                    fn InputDuplicateStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::InputDuplicateStrategy)
                    }
                    InputDuplicateStrategy
                },
                {
                    fn OnlySlashingRegistryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryErrors::OnlySlashingRegistryCoordinator)
                    }
                    OnlySlashingRegistryCoordinator
                },
                {
                    fn EmptyStakeHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <EmptyStakeHistory as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::EmptyStakeHistory)
                    }
                    EmptyStakeHistory
                },
                {
                    fn OnlySlashingRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryErrors::OnlySlashingRegistryCoordinatorOwner,
                            )
                    }
                    OnlySlashingRegistryCoordinatorOwner
                },
                {
                    fn QuorumDoesNotExist(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryErrors> {
                        <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryErrors::QuorumDoesNotExist)
                    }
                    QuorumDoesNotExist
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BelowMinimumStakeRequirement(inner) => {
                    <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyStakeHistory(inner) => {
                    <EmptyStakeHistory as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputDuplicateStrategy(inner) => {
                    <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputMultiplierZero(inner) => {
                    <InputMultiplierZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBlockNumber(inner) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlySlashingRegistryCoordinator(inner) => {
                    <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlySlashingRegistryCoordinatorOwner(inner) => {
                    <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumAlreadyExists(inner) => {
                    <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BelowMinimumStakeRequirement(inner) => {
                    <BelowMinimumStakeRequirement as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyStakeHistory(inner) => {
                    <EmptyStakeHistory as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthZero(inner) => {
                    <InputArrayLengthZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputDuplicateStrategy(inner) => {
                    <InputDuplicateStrategy as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputMultiplierZero(inner) => {
                    <InputMultiplierZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBlockNumber(inner) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlySlashingRegistryCoordinator(inner) => {
                    <OnlySlashingRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlySlashingRegistryCoordinatorOwner(inner) => {
                    <OnlySlashingRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumAlreadyExists(inner) => {
                    <QuorumAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QuorumDoesNotExist(inner) => {
                    <QuorumDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StakeRegistry`](self) events.
    pub enum StakeRegistryEvents {
        LookAheadPeriodChanged(LookAheadPeriodChanged),
        MinimumStakeForQuorumUpdated(MinimumStakeForQuorumUpdated),
        OperatorStakeUpdate(OperatorStakeUpdate),
        QuorumCreated(QuorumCreated),
        StakeTypeSet(StakeTypeSet),
        StrategyAddedToQuorum(StrategyAddedToQuorum),
        StrategyMultiplierUpdated(StrategyMultiplierUpdated),
        StrategyRemovedFromQuorum(StrategyRemovedFromQuorum),
    }
    #[automatically_derived]
    impl StakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                16u8, 86u8, 94u8, 86u8, 202u8, 203u8, 243u8, 46u8, 202u8, 38u8, 121u8, 69u8, 240u8,
                84u8, 254u8, 192u8, 46u8, 89u8, 117u8, 0u8, 50u8, 209u8, 19u8, 211u8, 48u8, 33u8,
                130u8, 173u8, 150u8, 127u8, 84u8, 4u8,
            ],
            [
                17u8, 165u8, 100u8, 19u8, 34u8, 218u8, 29u8, 255u8, 86u8, 164u8, 182u8, 110u8,
                170u8, 195u8, 31u8, 250u8, 70u8, 82u8, 149u8, 236u8, 233u8, 7u8, 205u8, 22u8, 52u8,
                55u8, 121u8, 59u8, 77u8, 0u8, 154u8, 117u8,
            ],
            [
                38u8, 238u8, 207u8, 242u8, 183u8, 11u8, 10u8, 113u8, 16u8, 79u8, 244u8, 217u8,
                64u8, 186u8, 113u8, 98u8, 210u8, 58u8, 149u8, 194u8, 72u8, 119u8, 31u8, 196u8,
                135u8, 167u8, 190u8, 23u8, 165u8, 150u8, 179u8, 207u8,
            ],
            [
                40u8, 215u8, 53u8, 139u8, 121u8, 240u8, 45u8, 33u8, 184u8, 183u8, 225u8, 122u8,
                239u8, 196u8, 24u8, 90u8, 100u8, 48u8, 138u8, 163u8, 116u8, 6u8, 250u8, 91u8,
                239u8, 192u8, 91u8, 145u8, 147u8, 44u8, 57u8, 199u8,
            ],
            [
                47u8, 82u8, 125u8, 82u8, 126u8, 149u8, 216u8, 254u8, 64u8, 174u8, 197u8, 83u8,
                119u8, 116u8, 59u8, 183u8, 121u8, 8u8, 125u8, 163u8, 246u8, 208u8, 208u8, 143u8,
                18u8, 227u8, 100u8, 68u8, 218u8, 98u8, 50u8, 125u8,
            ],
            [
                49u8, 250u8, 46u8, 44u8, 210u8, 128u8, 201u8, 55u8, 94u8, 19u8, 255u8, 207u8, 61u8,
                129u8, 226u8, 55u8, 129u8, 0u8, 24u8, 110u8, 64u8, 88u8, 248u8, 211u8, 221u8,
                182u8, 144u8, 184u8, 45u8, 205u8, 49u8, 247u8,
            ],
            [
                124u8, 17u8, 46u8, 134u8, 60u8, 207u8, 0u8, 120u8, 98u8, 226u8, 201u8, 226u8, 88u8,
                25u8, 201u8, 51u8, 254u8, 219u8, 201u8, 53u8, 10u8, 100u8, 67u8, 66u8, 59u8, 74u8,
                133u8, 153u8, 194u8, 232u8, 165u8, 45u8,
            ],
            [
                131u8, 26u8, 156u8, 134u8, 196u8, 91u8, 179u8, 3u8, 202u8, 243u8, 240u8, 100u8,
                190u8, 43u8, 194u8, 185u8, 253u8, 78u8, 207u8, 25u8, 228u8, 124u8, 74u8, 192u8,
                42u8, 97u8, 231u8, 93u8, 171u8, 254u8, 85u8, 180u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for StakeRegistryEvents {
        const NAME: &'static str = "StakeRegistryEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<LookAheadPeriodChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LookAheadPeriodChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::LookAheadPeriodChanged)
                }
                Some(
                    <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::MinimumStakeForQuorumUpdated),
                Some(<OperatorStakeUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorStakeUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorStakeUpdate)
                }
                Some(<QuorumCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumCreated)
                }
                Some(<StakeTypeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeTypeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakeTypeSet)
                }
                Some(<StrategyAddedToQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyAddedToQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyAddedToQuorum)
                }
                Some(<StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyMultiplierUpdated)
                }
                Some(<StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyRemovedFromQuorum)
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
    impl alloy_sol_types::private::IntoLogData for StakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::LookAheadPeriodChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakeTypeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyAddedToQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyMultiplierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyRemovedFromQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::LookAheadPeriodChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakeTypeSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyAddedToQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyMultiplierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyRemovedFromQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

    See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StakeRegistryInstance<T, P, N> {
        StakeRegistryInstance::<T, P, N>::new(address, provider)
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
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<StakeRegistryInstance<T, P, N>>>
    {
        StakeRegistryInstance::<T, P, N>::deploy(
            provider,
            _slashingRegistryCoordinator,
            _delegationManager,
            _avsDirectory,
            _allocationManager,
        )
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
        _slashingRegistryCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        StakeRegistryInstance::<T, P, N>::deploy_builder(
            provider,
            _slashingRegistryCoordinator,
            _delegationManager,
            _avsDirectory,
            _allocationManager,
        )
    }
    /**A [`StakeRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`StakeRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StakeRegistryInstance")
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
        > StakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
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
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<StakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _slashingRegistryCoordinator,
                _delegationManager,
                _avsDirectory,
                _allocationManager,
            );
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
            _slashingRegistryCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _slashingRegistryCoordinator,
                        _delegationManager,
                        _avsDirectory,
                        _allocationManager,
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
    impl<T, P: ::core::clone::Clone, N> StakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StakeRegistryInstance<T, P, N> {
            StakeRegistryInstance {
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
        > StakeRegistryInstance<T, P, N>
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
        ///Creates a new call builder for the [`MAX_WEIGHING_FUNCTION_LENGTH`] function.
        pub fn MAX_WEIGHING_FUNCTION_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_WEIGHING_FUNCTION_LENGTHCall, N> {
            self.call_builder(&MAX_WEIGHING_FUNCTION_LENGTHCall {})
        }
        ///Creates a new call builder for the [`WEIGHTING_DIVISOR`] function.
        pub fn WEIGHTING_DIVISOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, WEIGHTING_DIVISORCall, N> {
            self.call_builder(&WEIGHTING_DIVISORCall {})
        }
        ///Creates a new call builder for the [`addStrategies`] function.
        pub fn addStrategies(
            &self,
            quorumNumber: u8,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesCall, N> {
            self.call_builder(&addStrategiesCall {
                quorumNumber,
                _strategyParams,
            })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
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
        ///Creates a new call builder for the [`getCurrentStake`] function.
        pub fn getCurrentStake(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentStakeCall, N> {
            self.call_builder(&getCurrentStakeCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getCurrentTotalStake`] function.
        pub fn getCurrentTotalStake(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalStakeCall, N> {
            self.call_builder(&getCurrentTotalStakeCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getLatestStakeUpdate`] function.
        pub fn getLatestStakeUpdate(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestStakeUpdateCall, N> {
            self.call_builder(&getLatestStakeUpdateCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumber`] function.
        pub fn getStakeAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeAtBlockNumberCall, N> {
            self.call_builder(&getStakeAtBlockNumberCall {
                operatorId,
                quorumNumber,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumberAndIndex`] function.
        pub fn getStakeAtBlockNumberAndIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeAtBlockNumberAndIndexCall, N> {
            self.call_builder(&getStakeAtBlockNumberAndIndexCall {
                quorumNumber,
                blockNumber,
                operatorId,
                index,
            })
        }
        ///Creates a new call builder for the [`getStakeHistory`] function.
        pub fn getStakeHistory(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeHistoryCall, N> {
            self.call_builder(&getStakeHistoryCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeHistoryLength`] function.
        pub fn getStakeHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeHistoryLengthCall, N> {
            self.call_builder(&getStakeHistoryLengthCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeUpdateAtIndex`] function.
        pub fn getStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeUpdateAtIndexCall, N> {
            self.call_builder(&getStakeUpdateAtIndexCall {
                quorumNumber,
                operatorId,
                index,
            })
        }
        ///Creates a new call builder for the [`getStakeUpdateIndexAtBlockNumber`] function.
        pub fn getStakeUpdateIndexAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeUpdateIndexAtBlockNumberCall, N>
        {
            self.call_builder(&getStakeUpdateIndexAtBlockNumberCall {
                operatorId,
                quorumNumber,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getTotalStakeAtBlockNumberFromIndex`] function.
        pub fn getTotalStakeAtBlockNumberFromIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeAtBlockNumberFromIndexCall, N>
        {
            self.call_builder(&getTotalStakeAtBlockNumberFromIndexCall {
                quorumNumber,
                blockNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getTotalStakeHistoryLength`] function.
        pub fn getTotalStakeHistoryLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeHistoryLengthCall, N> {
            self.call_builder(&getTotalStakeHistoryLengthCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getTotalStakeIndicesAtBlockNumber`] function.
        pub fn getTotalStakeIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeIndicesAtBlockNumberCall, N>
        {
            self.call_builder(&getTotalStakeIndicesAtBlockNumberCall {
                blockNumber,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`getTotalStakeUpdateAtIndex`] function.
        pub fn getTotalStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeUpdateAtIndexCall, N> {
            self.call_builder(&getTotalStakeUpdateAtIndexCall {
                quorumNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`initializeDelegatedStakeQuorum`] function.
        pub fn initializeDelegatedStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeDelegatedStakeQuorumCall, N> {
            self.call_builder(&initializeDelegatedStakeQuorumCall {
                quorumNumber,
                minimumStake,
                _strategyParams,
            })
        }
        ///Creates a new call builder for the [`initializeSlashableStakeQuorum`] function.
        pub fn initializeSlashableStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            lookAheadPeriod: u32,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistryTypes::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeSlashableStakeQuorumCall, N> {
            self.call_builder(&initializeSlashableStakeQuorumCall {
                quorumNumber,
                minimumStake,
                lookAheadPeriod,
                _strategyParams,
            })
        }
        ///Creates a new call builder for the [`isOperatorSetQuorum`] function.
        pub fn isOperatorSetQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetQuorumCall, N> {
            self.call_builder(&isOperatorSetQuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`minimumStakeForQuorum`] function.
        pub fn minimumStakeForQuorum(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, minimumStakeForQuorumCall, N> {
            self.call_builder(&minimumStakeForQuorumCall { _0 })
        }
        ///Creates a new call builder for the [`modifyStrategyParams`] function.
        pub fn modifyStrategyParams(
            &self,
            quorumNumber: u8,
            strategyIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            newMultipliers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyStrategyParamsCall, N> {
            self.call_builder(&modifyStrategyParamsCall {
                quorumNumber,
                strategyIndices,
                newMultipliers,
            })
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall {
                operator,
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
        ///Creates a new call builder for the [`removeStrategies`] function.
        pub fn removeStrategies(
            &self,
            quorumNumber: u8,
            indicesToRemove: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeStrategiesCall, N> {
            self.call_builder(&removeStrategiesCall {
                quorumNumber,
                indicesToRemove,
            })
        }
        ///Creates a new call builder for the [`setMinimumStakeForQuorum`] function.
        pub fn setMinimumStakeForQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<T, &P, setMinimumStakeForQuorumCall, N> {
            self.call_builder(&setMinimumStakeForQuorumCall {
                quorumNumber,
                minimumStake,
            })
        }
        ///Creates a new call builder for the [`setSlashableStakeLookahead`] function.
        pub fn setSlashableStakeLookahead(
            &self,
            quorumNumber: u8,
            _lookAheadBlocks: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setSlashableStakeLookaheadCall, N> {
            self.call_builder(&setSlashableStakeLookaheadCall {
                quorumNumber,
                _lookAheadBlocks,
            })
        }
        ///Creates a new call builder for the [`slashableStakeLookAheadPerQuorum`] function.
        pub fn slashableStakeLookAheadPerQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashableStakeLookAheadPerQuorumCall, N>
        {
            self.call_builder(&slashableStakeLookAheadPerQuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`stakeTypePerQuorum`] function.
        pub fn stakeTypePerQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeTypePerQuorumCall, N> {
            self.call_builder(&stakeTypePerQuorumCall { quorumNumber })
        }
        ///Creates a new call builder for the [`strategiesPerQuorum`] function.
        pub fn strategiesPerQuorum(
            &self,
            quorumNumber: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategiesPerQuorumCall, N> {
            self.call_builder(&strategiesPerQuorumCall { quorumNumber, _1 })
        }
        ///Creates a new call builder for the [`strategyParams`] function.
        pub fn strategyParams(
            &self,
            quorumNumber: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsCall, N> {
            self.call_builder(&strategyParamsCall { quorumNumber, _1 })
        }
        ///Creates a new call builder for the [`strategyParamsByIndex`] function.
        pub fn strategyParamsByIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsByIndexCall, N> {
            self.call_builder(&strategyParamsByIndexCall {
                quorumNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`strategyParamsLength`] function.
        pub fn strategyParamsLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsLengthCall, N> {
            self.call_builder(&strategyParamsLengthCall { quorumNumber })
        }
        ///Creates a new call builder for the [`updateOperatorStake`] function.
        pub fn updateOperatorStake(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorStakeCall, N> {
            self.call_builder(&updateOperatorStakeCall {
                operator,
                operatorId,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`weightOfOperatorForQuorum`] function.
        pub fn weightOfOperatorForQuorum(
            &self,
            quorumNumber: u8,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, weightOfOperatorForQuorumCall, N> {
            self.call_builder(&weightOfOperatorForQuorumCall {
                quorumNumber,
                operator,
            })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > StakeRegistryInstance<T, P, N>
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
        ///Creates a new event filter for the [`LookAheadPeriodChanged`] event.
        pub fn LookAheadPeriodChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, LookAheadPeriodChanged, N> {
            self.event_filter::<LookAheadPeriodChanged>()
        }
        ///Creates a new event filter for the [`MinimumStakeForQuorumUpdated`] event.
        pub fn MinimumStakeForQuorumUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumStakeForQuorumUpdated, N> {
            self.event_filter::<MinimumStakeForQuorumUpdated>()
        }
        ///Creates a new event filter for the [`OperatorStakeUpdate`] event.
        pub fn OperatorStakeUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorStakeUpdate, N> {
            self.event_filter::<OperatorStakeUpdate>()
        }
        ///Creates a new event filter for the [`QuorumCreated`] event.
        pub fn QuorumCreated_filter(&self) -> alloy_contract::Event<T, &P, QuorumCreated, N> {
            self.event_filter::<QuorumCreated>()
        }
        ///Creates a new event filter for the [`StakeTypeSet`] event.
        pub fn StakeTypeSet_filter(&self) -> alloy_contract::Event<T, &P, StakeTypeSet, N> {
            self.event_filter::<StakeTypeSet>()
        }
        ///Creates a new event filter for the [`StrategyAddedToQuorum`] event.
        pub fn StrategyAddedToQuorum_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyAddedToQuorum, N> {
            self.event_filter::<StrategyAddedToQuorum>()
        }
        ///Creates a new event filter for the [`StrategyMultiplierUpdated`] event.
        pub fn StrategyMultiplierUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyMultiplierUpdated, N> {
            self.event_filter::<StrategyMultiplierUpdated>()
        }
        ///Creates a new event filter for the [`StrategyRemovedFromQuorum`] event.
        pub fn StrategyRemovedFromQuorum_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyRemovedFromQuorum, N> {
            self.event_filter::<StrategyRemovedFromQuorum>()
        }
    }
}
